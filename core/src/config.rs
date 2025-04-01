use dotenvy::dotenv;
use persistence::config::DatabaseSettings;

use crate::environment::ENVIRONMENT;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    if !ENVIRONMENT.is_production() {
        allow_dot_env_vars();
    }

    let environment_filename = format!("{}.toml", ENVIRONMENT.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("base.toml")))
        .add_source(config::File::from(configuration_directory.join(environment_filename)))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

fn allow_dot_env_vars() {
    dotenv().ok();
}
