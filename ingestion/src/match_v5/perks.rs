// use anyhow::Result;
// use riven::models::match_v5::{PerkStats, PerkStyle, PerkStyleSelection,
// Perks}; use sqlx::PgPool;
//
// pub async fn insert_participant_perks(
//     pool: &PgPool,
//     match_id: &str,
//     participant_id: i32,
//     perks: &Perks,
// ) -> Result<(), sqlx::Error> {
//     // Begin a transaction to ensure atomicity.
//     let mut tx = pool.begin().await?;
//
//     // Insert into participant_perks
//     sqlx::query!(
//         r#"
//         INSERT INTO match_v5.participant_perks (
//             match_id, participant_id, defense, flex, offense
//         )
//         VALUES (
//             $1, $2, $3, $4, $5
//         )
//         "#,
//         match_id,
//         participant_id,
//         perks.stat_perks.defense,
//         perks.stat_perks.flex,
//         perks.stat_perks.offense
//     )
//     .execute(&mut tx)
//     .await?;
//
//     for style in &perks.styles {
//         let record = sqlx::query_scalar!(
//             r#"
//             INSERT INTO match_v5.perk_style (
//                 match_id, participant_id, description, style
//             )
//             VALUES (
//                 $1, $2, $3, $4
//             )
//             RETURNING id as "id!"
//             "#,
//             match_id,
//             participant_id,
//             style.description,
//             style.style,
//         )
//         .fetch_one(&mut tx)
//         .await?;
//         let perk_style_id = record;
//
//         for selection in &style.selections {
//             sqlx::query!(
//                 r#"
//                 INSERT INTO match_v5.perk_style_selection (
//                     perk_style_id, perk, var1, var2, var3
//                 )
//                 VALUES (
//                     $1, $2, $3, $4, $5
//                 )
//                 "#,
//                 perk_style_id,
//                 selection.perk,
//                 selection.var1,
//                 selection.var2,
//                 selection.var3,
//             )
//             .execute(&mut tx)
//             .await?;
//         }
//     }
//
//     tx.commit().await?;
//     Ok(())
// }
