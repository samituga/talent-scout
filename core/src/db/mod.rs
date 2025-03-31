use anyhow::Context;
use sea_orm::{ConnectOptions, DatabaseConnection};

use crate::config::DatabaseSettings;

pub mod mapper;
pub mod migrations;
pub mod query;
pub mod table;

pub struct Database {
    options: ConnectOptions,
    pool: DatabaseConnection,
}

impl Database {
    pub async fn new(config: &DatabaseSettings) -> Result<Self, anyhow::Error> {
        sqlx::any::install_default_drivers();
        let options = config.pg_connect_options();
        Ok(Self {
            options: options.clone(),
            pool: sea_orm::Database::connect(options)
                .await
                .context("Failed to create a database connection")?,
        })
    }

    pub(crate) fn pool(&self) -> &DatabaseConnection {
        &self.pool
    }

    pub fn options(&self) -> &ConnectOptions {
        &self.options
    }
}
