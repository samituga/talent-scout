use sea_orm::DatabaseConnection;

pub mod mapper;
pub mod query;
pub mod table;

pub struct Database {
    pool: DatabaseConnection,
}

impl Database {
    pub(crate) fn pool(&self) -> &DatabaseConnection {
        &self.pool
    }
}
