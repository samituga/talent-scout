use riven::models::account_v1;
use sea_orm::IntoActiveModel;

use crate::table;

pub fn map(account: account_v1::Account, region: String) -> table::account_v1::accounts::ActiveModel {
    table::account_v1::accounts::Model {
        puuid: account.puuid,
        game_name: account.game_name,
        tag_line: account.tag_line,
        region,
    }
    .into_active_model()
}
