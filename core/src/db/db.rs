use sea_orm::DatabaseConnection;

pub struct Database {
    pool: DatabaseConnection,
}

impl Database {
    pub(crate) fn pool(&self) -> &DatabaseConnection {
        &self.pool
    }
}
