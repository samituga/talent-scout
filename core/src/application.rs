use crate::{
    config::Settings,
    db::{Database, migrations::run_database_migrations},
};

pub struct Application;

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        tracing::debug!("Starting application");
        tracing::info!("Starting database connection");
        let db = Database::new(&configuration.database).await?;

        run_database_migrations(db.options(), "./migrations").await?;

        Ok(Self)
    }
}
