use std::sync::Arc;

use persistence::{Database, migrations::run_database_migrations};

use crate::config::Settings;

pub struct Application {
    pub database: Arc<Box<Database>>,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        tracing::info!("Starting database connection");
        let db = Database::new(&configuration.database).await?;

        run_database_migrations(db.options(), "./migrations").await?;

        Ok(Self {
            database: Arc::new(Box::new(db)),
        })
    }
}
