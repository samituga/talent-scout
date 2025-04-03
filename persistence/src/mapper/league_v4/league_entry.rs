use riven::models::league_v4;
use sea_orm::IntoActiveModel;

use crate::table;

pub fn league_item_to_model(
    league_item: league_v4::LeagueItem,
) -> (
    table::league_v4::league_item::ActiveModel,
    Option<table::league_v4::mini_series::ActiveModel>,
) {
    let mini_series = league_item
        .mini_series
        .map(|mini_series| mini_series_to_model(league_item.puuid.clone(), mini_series));

    let league_item = table::league_v4::league_item::Model {
        puuid: league_item.puuid,
        fresh_blood: league_item.fresh_blood,
        wins: league_item.wins,
        inactive: league_item.inactive,
        veteran: league_item.veteran,
        hot_streak: league_item.hot_streak,
        rank: league_item.rank.to_string(),
        league_points: league_item.league_points,
        losses: league_item.losses,
        summoner_id: league_item.summoner_id,
    }
    .into_active_model();

    (league_item, mini_series)
}

fn mini_series_to_model(
    puuid: String,
    mini_series: league_v4::MiniSeries,
) -> table::league_v4::mini_series::ActiveModel {
    table::league_v4::mini_series::Model {
        puuid,
        losses: mini_series.losses,
        progress: mini_series.progress,
        target: mini_series.target,
        wins: mini_series.wins,
    }
    .into_active_model()
}
