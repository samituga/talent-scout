use std::sync::Arc;

use anyhow::Context;
use futures::{StreamExt, stream};
use persistence::{Database, mapper};
use riven::{consts::PlatformRoute, models::account_v1};
use tokio::task::JoinHandle;

pub async fn ingest_accounts(
    api: &riven::RiotApi,
    route: PlatformRoute,
    database: Arc<Box<Database>>,
) -> anyhow::Result<()> {
    let page_size: u64 = 128;
    let mut current_page: u64 = 0;

    loop {
        match database
            .fetch_puuids_from_match_v5_participants_paginated(current_page, page_size)
            .await
        {
            Ok(page_data) => {
                ingest_chunk(api, route, page_data.items, database.clone()).await?;

                if current_page >= page_data.total_pages - 1 {
                    break;
                }
                current_page += 1;
            }
            Err(err) => {
                tracing::error!("Error fetching page {}: {:?}", current_page, err);
                break;
            }
        }
    }

    Ok(())
}

async fn ingest_chunk(
    api: &riven::RiotApi,
    route: PlatformRoute,
    puuids: Vec<String>,
    database: Arc<Box<Database>>,
) -> anyhow::Result<()> {
    let join_handles: Vec<JoinHandle<()>> = stream::iter(puuids)
        .map(|puuid| {
            let database = database.clone();
            async move {
                tracing::info!("Fetching account for puuid: {}", puuid);
                match fetch_account(api, route.to_regional(), &puuid).await {
                    Ok(m) => tokio::spawn(async move {
                        let account = mapper::account_v1::account::map(m, route.to_string());
                        if let Err(e) = database.insert_account(account).await {
                            tracing::error!("Failed to insert account {}: {:?}", puuid, e);
                        } else {
                            tracing::info!("Account successfully inserted with ID: {}", puuid);
                        }
                    }),
                    Err(e) => {
                        tracing::error!("Failed to fetch account {}: {:?}", puuid, e);
                        tokio::spawn(async {})
                    }
                }
            }
        })
        .buffer_unordered(10)
        .collect()
        .await;

    for handle in join_handles {
        handle.await?;
    }

    tracing::info!("Account ingestion complete.");

    Ok(())
}

async fn fetch_account(
    api: &riven::RiotApi,
    route: riven::consts::RegionalRoute,
    puuid: &str,
) -> anyhow::Result<account_v1::Account> {
    api.account_v1()
        .get_by_puuid(route, puuid)
        .await
        .context(format!("Failed to fetch account for puuid {}", puuid))
}
