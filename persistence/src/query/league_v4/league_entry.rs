use sea_orm::{DbErr, Iterable};

use crate::{Database, table};

impl Database {
    pub async fn upsert_league_items(
        &self,
        league_items: Vec<table::league_v4::league_item::ActiveModel>,
    ) -> Result<(), DbErr> {
        let league_columns_to_update = table::league_v4::league_item::Column::iter().collect::<Vec<_>>();

        self.upsert_many_chunks(
            league_items,
            vec![table::league_v4::league_item::Column::Puuid],
            league_columns_to_update,
            &self.pool,
            512,
        )
        .await?;

        Ok(())
    }
}
