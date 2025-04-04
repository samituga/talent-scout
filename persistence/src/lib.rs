use anyhow::Context;
use sea_orm::{
    ActiveModelTrait, ConnectOptions, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, sea_query::OnConflict,
};

use crate::config::DatabaseSettings;

pub mod config;
pub mod mapper;
pub mod migrations;
mod query;
pub mod table;

const DEFAULT_BATCH_SIZE: usize = 512;

pub struct Database {
    options: ConnectOptions,
    pool: DatabaseConnection,
}

// TODO pagination, handle better chunks of data, caller should be able to
// customize request
impl Database {
    pub async fn new(config: &DatabaseSettings) -> Result<Self, anyhow::Error> {
        sqlx::any::install_default_drivers();
        let options = config.pg_connect_options();
        Ok(Self {
            options: options.clone(),
            pool: sea_orm::Database::connect(options)
                .await
                .context("Failed to connect to the database")?,
        })
    }

    pub async fn insert_many_chunks_512<T, C: ConnectionTrait>(&self, models: Vec<T>, db: &C) -> Result<(), DbErr>
    where
        T: ActiveModelTrait + Clone + Send + Sync + 'static,
        T::Entity: EntityTrait,
    {
        self.insert_many_chunks(models, db, DEFAULT_BATCH_SIZE).await
    }

    pub async fn insert_many_chunks<T, C: ConnectionTrait>(
        &self,
        models: Vec<T>,
        db: &C,
        batch_size: usize,
    ) -> Result<(), DbErr>
    where
        T: ActiveModelTrait + Clone + Send + Sync + 'static,
        T::Entity: EntityTrait,
    {
        if models.is_empty() {
            return Ok(());
        }
        for batch in models.chunks(batch_size) {
            let to_insert = batch.to_vec();
            T::Entity::insert_many(to_insert).exec(db).await?;
        }
        Ok(())
    }

    async fn insert_many_chunks_on_conflict_do_nothing<T, C: ConnectionTrait>(
        &self,
        models: Vec<T>,
        db: &C,
        batch_size: usize,
    ) -> Result<(), DbErr>
    where
        T: ActiveModelTrait + Clone + Send + Sync + 'static,
        T::Entity: EntityTrait,
    {
        if models.is_empty() {
            return Ok(());
        }
        for batch in models.chunks(batch_size) {
            let to_insert = batch.to_vec();
            T::Entity::insert_many(to_insert)
                .on_conflict_do_nothing()
                .exec(db)
                .await?;
        }
        Ok(())
    }

    pub async fn upsert_many_chunks<T, C: ConnectionTrait>(
        &self,
        models: Vec<T>,
        conflict_columns: Vec<<T::Entity as EntityTrait>::Column>,
        columns_to_update: Vec<<T::Entity as EntityTrait>::Column>,
        db: &C,
        batch_size: usize,
    ) -> Result<(), DbErr>
    where
        T: ActiveModelTrait + Clone + Send + Sync + 'static,
        T::Entity: EntityTrait,
    {
        if models.is_empty() {
            return Ok(());
        }
        for batch in models.chunks(batch_size) {
            let to_insert = batch.to_vec();
            T::Entity::insert_many(to_insert)
                .on_conflict(
                    OnConflict::columns(conflict_columns.clone())
                        .update_columns(columns_to_update.clone())
                        .to_owned(),
                )
                .exec(db)
                .await?;
        }
        Ok(())
    }

    pub fn options(&self) -> &ConnectOptions {
        &self.options
    }
}

pub struct Page<T> {
    pub items: Vec<T>,
    pub current_page: u64,
    pub page_size: u64,
    pub total_pages: u64,
    pub total_items: u64,
}
