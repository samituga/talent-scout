use std::sync::Arc;

use anyhow::Context;
use futures::{StreamExt, stream};
use persistence::{Database, mapper};
use riven::models::match_v5;
use tokio::task::JoinHandle;

pub async fn ingest(
    api: &riven::RiotApi,
    puuid: &str,
    route: riven::consts::RegionalRoute,
    database: Arc<Box<Database>>,
) -> anyhow::Result<()> {
    let match_ids = fetch_all_match_ids(api, puuid, route).await?;
    tracing::info!("Fetched {} match IDs", match_ids.len());

    let join_handles: Vec<JoinHandle<()>> = stream::iter(match_ids)
        .map(|id| {
            let database = database.clone();
            async move {
                tracing::info!("Fetching match info for id: {}", id);
                match fetch_match_info(api, route, &id).await {
                    Ok(m) => tokio::spawn(async move {
                        let models = mapper::match_v5::all(m);
                        if let Err(e) = database.insert_match_v5_match(models).await {
                            tracing::error!("Failed to insert match {}: {:?}", id, e);
                        } else {
                            tracing::info!("Match successfully inserted with ID: {}", id);
                        }
                    }),
                    Err(e) => {
                        tracing::error!("Failed to fetch match {}: {:?}", id, e);
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

    tracing::info!("Ingestion complete.");
    Ok(())
}
pub async fn ingest_match(
    api: &riven::RiotApi,
    match_id: &str,
    route: riven::consts::RegionalRoute,
    database: Arc<Box<Database>>,
) -> anyhow::Result<()> {
    let m = fetch_match_info(api, route, match_id).await?;

    let models = mapper::match_v5::all(m);
    database
        .insert_match_v5_match(models)
        .await
        .context("Failed to insert match in db")
}

pub async fn fetch_all_match_ids(
    api: &riven::RiotApi,
    puuid: &str,
    route: riven::consts::RegionalRoute,
) -> anyhow::Result<Vec<String>> {
    let match_v5 = api.match_v5();
    let mut all_match_ids = Vec::new();
    let mut start = 0;
    let count = 100;

    loop {
        let match_ids = match_v5
            .get_match_ids_by_puuid(route, puuid, Some(count), None, None, None, Some(start), None)
            .await?;

        if match_ids.is_empty() {
            break;
        }

        all_match_ids.extend(match_ids.clone());
        start += match_ids.len() as i32;
    }

    Ok(all_match_ids)
}

pub async fn fetch_match_info(
    api: &riven::RiotApi,
    route: riven::consts::RegionalRoute,
    match_id: &str,
) -> anyhow::Result<match_v5::Match> {
    match api.match_v5().get_match(route, match_id).await? {
        None => Err(anyhow::anyhow!("No match found with id: {}", match_id)),
        Some(v5_match) => Ok(v5_match),
    }
}
