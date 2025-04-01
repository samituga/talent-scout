use secrecy::{ExposeSecret, SecretString};
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn pg_connect_options(&self) -> sea_orm::ConnectOptions {
        let ssl_mode = if self.require_ssl { "require" } else { "prefer" };

        let mut opt = sea_orm::ConnectOptions::new(format!(
            "postgres://{}:{}@{}/{}?sslmode={}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.database_name,
            ssl_mode
        ));
        opt.sqlx_logging(false).sqlx_logging_level(log::LevelFilter::Error);
        opt
    }
}
