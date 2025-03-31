use sea_orm::{DbErr, EntityTrait, TransactionTrait};

use crate::db::{
    Database,
    mapper::match_v5::Models,
    table::{
        bans, challenges, feats, matches, missions, objectives, participant_perks, participants, perk_style_selections,
        perk_styles, teams,
    },
};
impl Database {
    pub async fn insert_match_v5_match(&self, models: Models) -> Result<(), DbErr> {
        let txn = self.pool.begin().await?;

        matches::Entity::insert(models.r#match).exec(&txn).await?;
        teams::Entity::insert_many(models.teams).exec(&txn).await?;
        bans::Entity::insert_many(models.bans).exec(&txn).await?;
        objectives::Entity::insert_many(models.objectives).exec(&txn).await?;
        feats::Entity::insert_many(models.feats).exec(&txn).await?;
        challenges::Entity::insert_many(models.challenges).exec(&txn).await?;
        missions::Entity::insert_many(models.missions).exec(&txn).await?;
        participants::Entity::insert_many(models.participants)
            .exec(&txn)
            .await?;
        participant_perks::Entity::insert_many(models.perks).exec(&txn).await?;
        perk_styles::Entity::insert_many(models.perk_styles).exec(&txn).await?;
        perk_style_selections::Entity::insert_many(models.perks_style_selections)
            .exec(&txn)
            .await?;

        txn.commit().await?;

        Ok(())
    }
}
