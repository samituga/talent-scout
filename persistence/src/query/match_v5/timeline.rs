use sea_orm::{DatabaseTransaction, DbErr, EntityTrait, TransactionTrait};

use crate::{Database, mapper::match_v5::timeline::TimelineModels, table};

impl Database {
    pub async fn insert_match_v5_timeline(&self, models: TimelineModels) -> Result<(), DbErr> {
        let txn: DatabaseTransaction = self.pool.begin().await?;

        table::timelines::Entity::insert(models.timeline).exec(&txn).await?;
        self.insert_many_chunks_512(models.timeline_participants, &txn).await?;
        self.insert_many_chunks_512(models.frames, &txn).await?;
        self.insert_many_chunks_512(models.timeline_participant_frames, &txn)
            .await?;
        self.insert_many_chunks_512(models.champion_stats, &txn).await?;
        self.insert_many_chunks_512(models.damage_stats, &txn).await?;
        self.insert_many_chunks_512(models.events_timeline, &txn).await?;
        self.insert_many_chunks_512(models.match_timeline_victim_damage_dealt, &txn)
            .await?;
        txn.commit().await?;

        Ok(())
    }
}
