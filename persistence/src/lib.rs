use anyhow::Context;
use sea_orm::{ConnectOptions, DatabaseConnection};

use crate::config::DatabaseSettings;

pub mod config;
pub mod mapper;
pub mod migrations;
mod query;
pub mod table;

pub struct Database {
    options: ConnectOptions,
    connection: DatabaseConnection,
}

impl Database {
    pub async fn new(config: &DatabaseSettings) -> Result<Self, anyhow::Error> {
        sqlx::any::install_default_drivers();
        let options = config.pg_connect_options();
        Ok(Self {
            options: options.clone(),
            connection: sea_orm::Database::connect(options)
                .await
                .context("Failed to connect to the database")?,
        })
    }

    pub fn options(&self) -> &ConnectOptions {
        &self.options
    }
}
