mod application;
mod config;
mod environment;
mod telemetry;

use anyhow::Result;
use dotenvy::dotenv;
use ingestion_app::league_v4::ingest::ingest_big_leagues;
use riven::{RiotApi, consts::PlatformRoute};

use crate::{application::Application, config::get_configuration};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok(); // TODO check if env is dev

    let telemetry_subscriber = telemetry::get_subscriber("talent-scout".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(telemetry_subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let app = Application::build(configuration).await?;

    let api_key = std::env::var("RGAPI_KEY").expect("RGAPI_KEY env missing");
    let riot_api = RiotApi::new(api_key);

    let platform = PlatformRoute::EUW1;

    ingest_big_leagues(&riot_api, platform, app.database.clone()).await?;

    // let account = riot_api
    //     .account_v1()
    //     .get_by_riot_id(platform.to_regional(), "avada", "AVD")
    //     .await
    //     .expect("Get summoner failed.")
    //     .expect("There is no summoner with that name.");
    //
    // ingest(&riot_api, &account.puuid, platform.to_regional(),
    // app.database.clone()).await?; ingest_match(
    //     &riot_api,
    //     "EUW1_7291285667",
    //     platform.to_regional(),
    //     app.database.clone(),
    // )
    // .await?;

    Ok(())
}
