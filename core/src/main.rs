use std::{fs::File, io::Write};

use anyhow::Result;
use dotenvy::dotenv;
use ingestion::fetch_all_match_ids;
use riven::{RiotApi, consts::PlatformRoute};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

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

    let matches = fetch_all_match_ids(&riot_api, account.puuid.as_str(), platform.to_regional()).await?;

    println!(
        "{}#{} Champion Masteries:",
        account.game_name.unwrap_or_default(),
        account.tag_line.unwrap_or_default(),
    );

    // let matches_ids = riot_api
    //     .match_v5()
    //     .get_match_ids_by_puuid(
    //         platform.to_regional(),
    //         account.puuid.as_str(),
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //     )
    //     .await?;
    //
    // let last_match_id = matches_ids.last().unwrap().to_string();
    //
    // let timeline = riot_api
    //     .match_v5()
    //     .get_timeline(platform.to_regional(), last_match_id.as_str())
    //     .await
    //     .expect("Error getting timeline.")
    //     .expect("No timeline found.");
    // let frames_json = serde_json::to_vec_pretty(&timeline).expect(
    //     "Failed to
    // serialize timeline.",
    // );

    write_strings_to_file("matches.json", matches).expect("TODO: panic message");

    Ok(())
}

fn write_strings_to_file(path: &str, data: Vec<String>) -> std::io::Result<()> {
    write_bytes_to_file(path, data.into_iter().flat_map(|e| e.into_bytes()).collect())
}

fn write_bytes_to_file(path: &str, data: Vec<u8>) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(&data)?;
    Ok(())
}
