use sea_orm::{DbErr, EntityTrait, PaginatorTrait, QuerySelect};

use crate::{Database, Page, table, table::match_v5::participants};

impl Database {
    pub async fn insert_account(&self, account: table::account_v1::accounts::ActiveModel) -> Result<(), DbErr> {
        table::account_v1::accounts::Entity::insert(account)
            .exec(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn fetch_all_puuid(&self, page: u64, page_size: u64) -> Result<Page<String>, DbErr> {
        let paginator = table::account_v1::accounts::Entity::find()
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
