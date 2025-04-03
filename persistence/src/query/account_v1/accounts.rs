use sea_orm::{DbErr, EntityTrait, IntoActiveModel, PaginatorTrait, QuerySelect};

use crate::{Database, Page, table, table::match_v5::participants};

impl Database {
    pub async fn insert_account(&self, account: table::account_v1::accounts::ActiveModel) -> Result<(), DbErr> {
        table::account_v1::accounts::Entity::insert(account)
            .exec(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn insert_account_if_not_exists(&self, puuid: String, region: String) -> Result<(), DbErr> {
        let account = table::account_v1::accounts::Model {
            puuid,
            game_name: None,
            tag_line: None,
            region,
        }
        .into_active_model();

        table::account_v1::accounts::Entity::insert(account)
            .on_conflict_do_nothing()
            .exec(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn insert_many_account_if_not_exists(&self, puuids: Vec<String>, region: String) -> Result<(), DbErr> {
        let accounts = puuids
            .into_iter()
            .map(|puuid| {
                table::account_v1::accounts::Model {
                    puuid,
                    game_name: None,
                    tag_line: None,
                    region: region.clone(),
                }
                .into_active_model()
            })
            .collect();

        self.insert_many_chunks_on_conflict_do_nothing(accounts, &self.pool, 512)
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
