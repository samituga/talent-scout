// use anyhow::Context;
// use chrono::{DateTime, Utc};
// use futures::{StreamExt, stream};
// use riven::models::match_v5;
// use sqlx::PgPool;
// use tokio::task::JoinHandle;
//
// use crate::match_v5::match_::insert_match;
//
// /// Ingests match data by fetching match info and inserting it immediately
// into /// the database. The DB insertions are spawned as separate tasks so
// they do not /// block subsequent fetches.
// pub async fn ingest(
//     pool: &PgPool,
//     api: &riven::RiotApi,
//     puuid: &str,
//     route: riven::consts::RegionalRoute,
// ) -> anyhow::Result<()> {
//     // Fetch all match IDs (handled elsewhere)
//     let match_ids = fetch_all_match_ids_from_db(pool).await?;
//     println!("Fetched {} match IDs", match_ids.len());
//
//     // Create a stream over match IDs.
//     // For each match ID, fetch match info and then spawn a DB insertion
// task.     let join_handles: Vec<JoinHandle<()>> = stream::iter(match_ids)
//         .map(|id| async move {
//             let pool = pool.clone();
//             println!("Fetching match info for id: {}", id);
//             match fetch_match_info(api, route, &id).await {
//                 Ok(m) => {
//                     // Spawn a task to insert the match.
//                     // This task runs concurrently and its execution does not
// block further fetches.                     tokio::spawn(async move {
//                         if let Err(e) = insert_match(&pool, &m).await {
//                             eprintln!("Failed to insert match {}: {:?}", id,
// e);                         } else {
//                             println!("Inserted match {}", id);
//                         }
//                     })
//                 }
//                 Err(e) => {
//                     eprintln!("Failed to fetch match {}: {:?}", id, e);
//                     // Return a dummy task that completes immediately.
//                     tokio::spawn(async {})
//                 }
//             }
//         })
//         .buffer_unordered(10) // Allow up to 10 concurrent fetch tasks
// (adjust as needed)         .collect()
//         .await;
//
//     // Wait for all the spawned insertion tasks to complete.
//     for handle in join_handles {
//         // Optionally handle errors from the spawned tasks.
//         handle.await?;
//     }
//
//     println!("Ingestion complete.");
//     Ok(())
// }
//
// pub async fn fetch_all_match_ids(
//     api: &riven::RiotApi,
//     puuid: &str,
//     route: riven::consts::RegionalRoute,
// ) -> anyhow::Result<Vec<String>> {
//     let match_v5 = api.match_v5();
//     let mut all_match_ids = Vec::new();
//     let mut start = 0;
//     let count = 100;
//
//     loop {
//         let match_ids = match_v5
//             .get_match_ids_by_puuid(route, puuid, Some(count), None, None,
// None, Some(start), None)             .await?;
//
//         if match_ids.is_empty() {
//             break;
//         }
//
//         all_match_ids.extend(match_ids.clone());
//         start += match_ids.len() as i32;
//     }
//
//     Ok(all_match_ids)
// }
//
// pub async fn fetch_all_match_ids_from_db(pool: &PgPool) ->
// anyhow::Result<Vec<String>> {     let match_ids = sqlx::query!(
//         r#"
//         SELECT match_id FROM match_v5.match;
//         "#
//     )
//     .fetch_all(pool)
//     .await
//     .context("Failed to perform a query to retrieve user credentials.")?
//     .into_iter()
//     .map(|record| record.match_id)
//     .collect();
//
//     Ok(match_ids)
// }
//
// pub async fn fetch_match_info(
//     api: &riven::RiotApi,
//     route: riven::consts::RegionalRoute,
//     match_id: &str,
// ) -> anyhow::Result<match_v5::Match> {
//     match api.match_v5().get_match(route, match_id).await? {
//         None => Err(anyhow::anyhow!("No match found with id: {}", match_id)),
//         Some(v5_match) => Ok(v5_match),
//     }
// }
