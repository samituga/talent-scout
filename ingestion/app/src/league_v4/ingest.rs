use std::sync::Arc;

use persistence::{Database, mapper, table};
use riven::{consts::QueueType, models::league_v4};

pub async fn ingest_big_leagues(
    api: &riven::RiotApi,
    route: riven::consts::PlatformRoute,
    database: Arc<Box<Database>>,
) -> anyhow::Result<()> {
    tracing::info!("Fetching challenger league");
    let challenger_league_items = api
        .league_v4()
        .get_challenger_league(route, QueueType::RANKED_SOLO_5x5)
        .await?
        .entries;

    tracing::info!("Fetching grandmaster league");
    let grandmaster_league_items = api
        .league_v4()
        .get_grandmaster_league(route, QueueType::RANKED_SOLO_5x5)
        .await?
        .entries;

    tracing::info!("Fetching master league");
    let master_league_items = api
        .league_v4()
        .get_master_league(route, QueueType::RANKED_SOLO_5x5)
        .await?
        .entries;

    let big_league_entries: Vec<league_v4::LeagueItem> = challenger_league_items
        .into_iter()
        .chain(grandmaster_league_items.into_iter())
        .chain(master_league_items.into_iter())
        .collect();

    let len = big_league_entries.len();

    let (puuids, league_items) = big_league_entries
        .into_iter()
        .map(|league_item| {
            (
                league_item.puuid.clone(),
                mapper::league_v4::league_entry::league_item_to_model(league_item),
            )
        })
        .map(|(puuid, (league_item, _))| (puuid, league_item))
        .fold(
            (Vec::with_capacity(len), Vec::with_capacity(len)),
            |(mut puuids, mut league_items), (puuid, league_item)| {
                puuids.push(puuid);
                league_items.push(league_item);
                (puuids, league_items)
            },
        );

    tracing::info!(
        "Inserting accounts (do nothing if it already exists) len: {}",
        puuids.len()
    );
    database
        .insert_many_account_if_not_exists(puuids, route.to_string())
        .await?;

    tracing::info!("Upserting league items, len: {}", league_items.len());
    database.upsert_league_items(league_items).await?;

    Ok(())
}
