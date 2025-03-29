mod config;
mod db;
mod environment;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // dotenv().ok();
    // let configuration = get_configuration().expect("Failed to read
    // configuration."); let connection_pool =
    // get_connection_pool(&configuration.database);
    //
    // let api_key = std::env::var("RGAPI_KEY").expect("RGAPI_KEY env missing");
    // println!("RGAPI_KEY: {}", api_key);
    // let riot_api = RiotApi::new(api_key);
    //
    // let platform = PlatformRoute::EUW1;
    //
    // let account = riot_api
    //     .account_v1()
    //     .get_by_riot_id(platform.to_regional(), "avada", "AVD")
    //     .await
    //     .expect("Get summoner failed.")
    //     .expect("There is no summoner with that name.");

    Ok(())
}
