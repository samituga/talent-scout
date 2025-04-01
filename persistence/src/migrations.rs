use std::{collections::HashMap, path::Path, time::Duration};

use anyhow::{Result, anyhow};
use sea_orm::ConnectOptions;
use sqlx::{
    Connection,
    migrate::{Migrate, Migration, MigrationSource},
};

pub async fn run_database_migrations(
    database_options: &ConnectOptions,
    migrations_path: impl AsRef<Path>,
) -> Result<Vec<(Migration, Duration)>> {
    tracing::debug!("Resolving migrations from path: {:?}", migrations_path.as_ref());
    let migrations = MigrationSource::resolve(migrations_path.as_ref())
        .await
        .map_err(|err| {
            let msg = format!("failed to load migrations: {err:?}");
            anyhow!(msg)
        })?;

    tracing::debug!("Creating sqlx any connection...");
    let mut connection = sqlx::AnyConnection::connect(database_options.get_url()).await?;
    tracing::debug!("sqlx any connection created.");

    connection.ensure_migrations_table().await?;

    let applied_migrations: HashMap<_, _> = connection
        .list_applied_migrations()
        .await?
        .into_iter()
        .map(|migration| (migration.version, migration))
        .collect();

    let mut new_migrations = Vec::new();
    for migration in migrations {
        match applied_migrations.get(&migration.version) {
            Some(applied_migration) => {
                if migration.checksum != applied_migration.checksum {
                    let err_msg = format!(
                        "checksum mismatch for applied migration {} (version {})",
                        migration.description, migration.version
                    );
                    return Err(anyhow!(err_msg));
                } else {
                    tracing::debug!(
                        "Skipping migration {} (version {}): already applied and checksum matches.",
                        migration.description,
                        migration.version
                    );
                }
            }
            None => {
                tracing::debug!(
                    "Applying migration: {} (version {})",
                    migration.description,
                    migration.version
                );
                let elapsed = connection.apply(&migration).await?;
                tracing::debug!("Applied migration {} in {:?}", migration.description, elapsed);
                new_migrations.push((migration, elapsed));
            }
        }
    }

    tracing::info!(
        "Migrations complete. {} new migration(s) applied.",
        new_migrations.len()
    );
    Ok(new_migrations)
}
