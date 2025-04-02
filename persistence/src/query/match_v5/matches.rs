use sea_orm::{DbErr, EntityTrait, PaginatorTrait, QuerySelect, TransactionTrait};

use crate::{
    Database, Page,
    mapper::match_v5::r#match,
    table::match_v5::{
        bans, challenges, feats, matches, missions, objectives, participant_perks, participants, perk_style_selections,
        perk_styles, teams,
    },
};

impl Database {
    pub async fn insert_match_v5_match(&self, models: r#match::MatchModels) -> Result<(), DbErr> {
        let txn = self.pool.begin().await?;

        matches::Entity::insert(models.r#match).exec(&txn).await?;
        participants::Entity::insert_many(models.participants)
            .exec(&txn)
            .await?;
        teams::Entity::insert_many(models.teams).exec(&txn).await?;
        if !models.feats.is_empty() {
            feats::Entity::insert_many(models.feats).exec(&txn).await?;
        }
        if !models.bans.is_empty() {
            bans::Entity::insert_many(models.bans).exec(&txn).await?;
        }
        objectives::Entity::insert_many(models.objectives).exec(&txn).await?;
        if !models.challenges.is_empty() {
            challenges::Entity::insert_many(models.challenges).exec(&txn).await?;
        }
        if !models.missions.is_empty() {
            missions::Entity::insert_many(models.missions).exec(&txn).await?;
        }
        participant_perks::Entity::insert_many(models.perks).exec(&txn).await?;
        if !models.perk_styles.is_empty() {
            perk_styles::Entity::insert_many(models.perk_styles).exec(&txn).await?;
        }
        if !models.perk_style_selections.is_empty() {
            perk_style_selections::Entity::insert_many(models.perk_style_selections)
                .exec(&txn)
                .await?;
        }

        txn.commit().await?;

        Ok(())
    }

    pub async fn fetch_all_match_ids(&self) -> Result<Vec<String>, DbErr> {
        matches::Entity::find()
            .select_only()
            .column(matches::Column::MatchId)
            .into_tuple()
            .all(&self.pool)
            .await
    }

    pub async fn fetch_match_ids_paginated(&self, page: u64, page_size: u64) -> Result<Vec<String>, DbErr> {
        let match_ids = matches::Entity::find()
            .select_only()
            .column(matches::Column::MatchId)
            .into_tuple()
            .paginate(&self.pool, page_size)
            .fetch_page(page)
            .await?;
        Ok(match_ids)
    }

    pub async fn fetch_puuids_from_match_v5_participants_paginated(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<Page<String>, DbErr> {
        let paginator = participants::Entity::find()
            .select_only()
            .distinct()
            .column(participants::Column::Puuid)
            .into_tuple()
            .paginate(&self.pool, page_size);

        let items = paginator.fetch_page(page).await?;
        let total_pages = paginator.num_pages().await?;
        let total_items = paginator.num_items().await?;

        Ok(Page {
            items,
            current_page: page,
            page_size,
            total_pages,
            total_items,
        })
    }
}
