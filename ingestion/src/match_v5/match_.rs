// use chrono::{DateTime, Utc};
// use riven::models::match_v5;
// use sqlx::PgPool;
//
// use crate::util::unix_timestamp_to_date_time;
//
// pub async fn insert_match(pool: &PgPool, m: &match_v5::Match) -> Result<(),
// anyhow::Error> {     let metadata = &m.metadata;
//     let info = &m.info;
//     let game_creation = unix_timestamp_to_date_time(info.game_creation)?;
//     let game_start_timestamp =
// unix_timestamp_to_date_time(info.game_start_timestamp)?;
//     let game_end_timestamp: Option<DateTime<Utc>> = match
// info.game_end_timestamp {         Some(ts) =>
// Some(unix_timestamp_to_date_time(ts)?),         None => None,
//     };
//     let game_type = info.game_type.map(|e| e.to_string());
//     let game_mode = info.game_mode.to_string();
//     let queue_id = info.queue_id.to_string();
//     let map_id = info.map_id.to_string();
//     sqlx::query!(
//         r#"
//             INSERT INTO match_v5.match (
//                 match_id,
//                 data_version,
//                 end_of_game_result,
//                 game_creation,
//                 game_duration,
//                 game_end_timestamp,
//                 game_id,
//                 game_mode,
//                 game_name,
//                 game_start_timestamp,
//                 game_type,
//                 game_version,
//                 map_id,
//                 platform_id,
//                 queue_id,
//                 tournament_code
//             )
//             VALUES (
//                 $1, $2, $3, $4, $5, $6,
//                 $7, $8, $9, $10, $11, $12,
//                 $13, $14, $15, $16
//             )
//         "#,
//         metadata.match_id,
//         metadata.data_version,
//         info.end_of_game_result,
//         game_creation,
//         info.game_duration,
//         game_end_timestamp,
//         info.game_id,
//         game_mode,
//         info.game_name,
//         game_start_timestamp,
//         game_type,
//         info.game_version,
//         map_id,
//         info.platform_id,
//         queue_id,
//         info.tournament_code
//     )
//     .execute(pool)
//     .await
//     .map_err(|e| anyhow::Error::from(e))?;
//
//     Ok(())
// }
