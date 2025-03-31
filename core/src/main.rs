mod application;
mod config;
pub mod db;
mod environment;
mod telemetry;

use anyhow::Result;
use dotenvy::dotenv;
use riven::{RiotApi, consts::PlatformRoute};

use crate::{application::Application, config::get_configuration};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok(); // TODO check env

    let telemetry_subscriber = telemetry::get_subscriber("talent-scout".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(telemetry_subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let _ = Application::build(configuration).await?;

    let api_key = std::env::var("RGAPI_KEY").expect("RGAPI_KEY env missing");
    println!("RGAPI_KEY: {}", api_key);
    let riot_api = RiotApi::new(api_key);

    let platform = PlatformRoute::EUW1;

    let account = riot_api
        .account_v1()
        .get_by_riot_id(platform.to_regional(), "avada", "AVD")
        .await
        .expect("Get summoner failed.")
        .expect("There is no summoner with that name.");

    Ok(())
}
