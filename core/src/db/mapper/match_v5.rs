use riven::models::match_v5;
use rust_decimal::{Decimal, prelude::FromPrimitive};

use crate::db::table;

pub fn match_to_entity(m: match_v5::Match) -> Result<table::matches::Model, anyhow::Error> {
    let metadata = &m.metadata;
    let info = &m.info;

    let game_type = info.game_type.as_ref().map(|e| e.to_string()).unwrap_or_default();
    let game_mode = info.game_mode.to_string();

    let entity = table::matches::Model {
        match_id: metadata.match_id.clone(),
        data_version: metadata.data_version.clone(),
        end_of_game_result: info.end_of_game_result.clone(),
        game_creation: info.game_creation,
        game_duration: info.game_duration,
        game_end_timestamp: info.game_end_timestamp,
        game_id: info.game_id,
        game_mode,
        game_name: info.game_name.clone(),
        game_start_timestamp: info.game_start_timestamp,
        game_type,
        game_version: info.game_version.clone(),
        map_id: info.map_id.0 as i32,
        platform_id: info.platform_id.clone(),
        queue_id: info.queue_id.0 as i32,
        tournament_code: info.tournament_code.clone(),
    };

    Ok(entity)
}

pub fn participant_with_match_id_to_entity(
    participant: match_v5::Participant,
    match_id: String,
) -> Result<table::participants::Model, anyhow::Error> {
    let champion_id = participant.champion().expect("FromStr imp for String is infallible").0 as i32;

    let player_score0 = participant.player_score0.map(f32_to_decimal);
    let player_score1 = participant.player_score1.map(f32_to_decimal);
    let player_score2 = participant.player_score2.map(f32_to_decimal);
    let player_score3 = participant.player_score3.map(f32_to_decimal);
    let player_score4 = participant.player_score4.map(f32_to_decimal);
    let player_score5 = participant.player_score5.map(f32_to_decimal);
    let player_score6 = participant.player_score6.map(f32_to_decimal);
    let player_score7 = participant.player_score7.map(f32_to_decimal);
    let player_score8 = participant.player_score8.map(f32_to_decimal);
    let player_score9 = participant.player_score9.map(f32_to_decimal);
    let player_score10 = participant.player_score10.map(f32_to_decimal);
    let player_score11 = participant.player_score11.map(f32_to_decimal);

    let entity = table::participants::Model {
        match_id,
        participant_id: participant.participant_id,
        all_in_pings: participant.all_in_pings,
        assist_me_pings: participant.assist_me_pings,
        assists: participant.assists,
        baron_kills: participant.baron_kills,
        bounty_level: participant.bounty_level,
        champ_experience: participant.champ_experience,
        champ_level: participant.champ_level,
        champion_id,
        champion_name: participant.champion_name.clone(),
        champion_transform: participant.champion_transform,
        consumables_purchased: participant.consumables_purchased,
        command_pings: participant.command_pings,
        damage_dealt_to_buildings: participant.damage_dealt_to_buildings,
        damage_dealt_to_objectives: participant.damage_dealt_to_objectives,
        damage_dealt_to_turrets: participant.damage_dealt_to_turrets,
        damage_self_mitigated: participant.damage_self_mitigated,
        deaths: participant.deaths,
        detector_wards_placed: participant.detector_wards_placed,
        double_kills: participant.double_kills,
        dragon_kills: participant.dragon_kills,
        eligible_for_progression: participant.eligible_for_progression,
        enemy_missing_pings: participant.enemy_missing_pings,
        enemy_vision_pings: participant.enemy_vision_pings,
        first_blood_assist: participant.first_blood_assist,
        first_blood_kill: participant.first_blood_kill,
        first_tower_assist: participant.first_tower_assist,
        first_tower_kill: participant.first_tower_kill,
        game_ended_in_early_surrender: participant.game_ended_in_early_surrender,
        game_ended_in_surrender: participant.game_ended_in_surrender,
        hold_pings: participant.hold_pings,
        get_back_pings: participant.get_back_pings,
        gold_earned: participant.gold_earned,
        gold_spent: participant.gold_spent,
        individual_position: participant.individual_position.clone(),
        inhibitor_kills: participant.inhibitor_kills,
        inhibitor_takedowns: participant.inhibitor_takedowns,
        inhibitors_lost: participant.inhibitors_lost,
        item0: participant.item0,
        item1: participant.item1,
        item2: participant.item2,
        item3: participant.item3,
        item4: participant.item4,
        item5: participant.item5,
        item6: participant.item6,
        items_purchased: participant.items_purchased,
        killing_sprees: participant.killing_sprees,
        kills: participant.kills,
        lane: participant.lane.clone(),
        largest_critical_strike: participant.largest_critical_strike,
        largest_killing_spree: participant.largest_killing_spree,
        largest_multi_kill: participant.largest_multi_kill,
        longest_time_spent_living: participant.longest_time_spent_living,
        magic_damage_dealt: participant.magic_damage_dealt,
        magic_damage_dealt_to_champions: participant.magic_damage_dealt_to_champions,
        magic_damage_taken: participant.magic_damage_taken,
        neutral_minions_killed: participant.neutral_minions_killed,
        need_vision_pings: participant.need_vision_pings,
        nexus_kills: participant.nexus_kills,
        nexus_takedowns: participant.nexus_takedowns,
        nexus_lost: participant.nexus_lost,
        objectives_stolen: participant.objectives_stolen,
        objectives_stolen_assists: participant.objectives_stolen_assists,
        on_my_way_pings: participant.on_my_way_pings,
        player_score0,
        player_score1,
        player_score2,
        player_score3,
        player_score4,
        player_score5,
        player_score6,
        player_score7,
        player_score8,
        player_score9,
        player_score10,
        player_score11,
        penta_kills: participant.penta_kills,
        physical_damage_dealt: participant.physical_damage_dealt,
        physical_damage_dealt_to_champions: participant.physical_damage_dealt_to_champions,
        physical_damage_taken: participant.physical_damage_taken,
        placement: participant.placement,
        player_augment1: participant.player_augment1,
        player_augment2: participant.player_augment2,
        player_augment3: participant.player_augment3,
        player_augment4: participant.player_augment4,
        player_subteam_id: participant.player_subteam_id,
        push_pings: participant.push_pings,
        profile_icon: participant.profile_icon,
        puuid: participant.puuid.clone(),
        quadra_kills: participant.quadra_kills,
        riot_id_game_name: participant.riot_id_game_name,
        riot_id_tagline: participant.riot_id_tagline,
        role: participant.role.clone(),
        sight_wards_bought_in_game: participant.sight_wards_bought_in_game,
        spell1_casts: participant.spell1_casts,
        spell2_casts: participant.spell2_casts,
        spell3_casts: participant.spell3_casts,
        spell4_casts: participant.spell4_casts,
        subteam_placement: participant.subteam_placement,
        summoner1_casts: participant.summoner1_casts,
        summoner1_id: participant.summoner1_id,
        summoner2_casts: participant.summoner2_casts,
        summoner2_id: participant.summoner2_id,
        summoner_id: participant.summoner_id.clone(),
        summoner_level: participant.summoner_level,
        summoner_name: participant.summoner_name.clone(),
        team_early_surrendered: participant.team_early_surrendered,
        team_id: participant.team_id as i32,
        team_position: participant.team_position.clone(),
        time_c_cing_others: participant.time_c_cing_others,
        time_played: participant.time_played,
        total_ally_jungle_minions_killed: participant.total_ally_jungle_minions_killed,
        total_damage_dealt: participant.total_damage_dealt,
        total_damage_dealt_to_champions: participant.total_damage_dealt_to_champions,
        total_damage_shielded_on_teammates: participant.total_damage_shielded_on_teammates,
        total_damage_taken: participant.total_damage_taken,
        total_enemy_jungle_minions_killed: participant.total_enemy_jungle_minions_killed,
        total_heal: participant.total_heal,
        total_heals_on_teammates: participant.total_heals_on_teammates,
        total_minions_killed: participant.total_minions_killed,
        total_time_cc_dealt: participant.total_time_cc_dealt,
        total_time_spent_dead: participant.total_time_spent_dead,
        total_units_healed: participant.total_units_healed,
        triple_kills: participant.triple_kills,
        true_damage_dealt: participant.true_damage_dealt,
        true_damage_dealt_to_champions: participant.true_damage_dealt_to_champions,
        true_damage_taken: participant.true_damage_taken,
        turret_kills: participant.turret_kills,
        turret_takedowns: participant.turret_takedowns,
        turrets_lost: participant.turrets_lost,
        unreal_kills: participant.unreal_kills,
        vision_score: participant.vision_score,
        vision_cleared_pings: participant.vision_cleared_pings,
        vision_wards_bought_in_game: participant.vision_wards_bought_in_game,
        wards_killed: participant.wards_killed,
        wards_placed: participant.wards_placed,
        win: participant.win,
        bait_pings: participant.bait_pings,
        danger_pings: participant.danger_pings,
        basic_pings: participant.basic_pings,
        player_augment5: participant.player_augment5,
        player_augment6: participant.player_augment6,
        riot_id_name: participant.riot_id_name,
        retreat_pings: participant.retreat_pings,
    };

    Ok(entity)
}

// TODO check if this works
fn f32_to_decimal(n: f32) -> Decimal {
    Decimal::from_f32(n).expect("Failed to convert to decimal") // Need to confirm if this ever fails
}

// TODO this doesn't look good, the ids
// TODO maybe use UUID instead of relying on the BD to generate the ids
pub fn perks_with_match_id_and_participant_id_to_entity(
    match_id: String,
    participant_id: i32,
    perks: match_v5::Perks,
) -> (
    table::participant_perks::Model,
    Vec<(table::perk_styles::Model, Vec<table::perk_style_selections::Model>)>,
) {
    let participant_perks = table::participant_perks::Model {
        match_id: match_id.clone(),
        participant_id,
        defense: perks.stat_perks.defense,
        flex: perks.stat_perks.flex,
        offense: perks.stat_perks.offense,
    };

    let perk_styles_selection = perks
        .styles
        .into_iter()
        .map(|style| {
            let perk_style = table::perk_styles::Model {
                perk_style_id: 0, // SERIAL
                match_id: match_id.clone(),
                participant_id,
                description: style.description.clone(),
                style: style.style,
            };

            let selections: Vec<table::perk_style_selections::Model> = style
                .selections
                .into_iter()
                .map(|sel| table::perk_style_selections::Model {
                    perk_style_selection_id: 0, // SERIAL
                    perk_style_id: 0,           // Needs to be updated with correct id after insert
                    perk: sel.perk,
                    var1: sel.var1,
                    var2: sel.var2,
                    var3: sel.var3,
                })
                .collect();

            (perk_style, selections)
        })
        .collect();

    (participant_perks, perk_styles_selection)
}

pub fn team_to_entity(team: match_v5::Team, match_id: String) -> Result<table::teams::Model, anyhow::Error> {
    let entity = table::teams::Model {
        match_id,
        team_id: team.team_id as i32,
        win: team.win,
    };

    Ok(entity)
}

pub fn ban_to_entity(ban: match_v5::Ban, match_id: String, team_id: i32) -> Result<table::bans::Model, anyhow::Error> {
    let entity = table::bans::Model {
        ban_id: 0, // SERIAL
        match_id,
        team_id,
        champion_id: ban.champion_id.0 as i32,
        pick_turn: ban.pick_turn,
    };

    Ok(entity)
}

pub fn objectives_to_entity(
    objectives: match_v5::Objectives,
    match_id: String,
    team_id: i32,
) -> Result<table::objectives::Model, anyhow::Error> {
    fn map_objective(obj: &match_v5::Objective) -> (bool, i32) {
        (obj.first, obj.kills)
    }

    let (baron_first, baron_kills) = map_objective(&objectives.baron);
    let (champion_first, champion_kills) = map_objective(&objectives.champion);
    let (dragon_first, dragon_kills) = map_objective(&objectives.dragon);
    let (inhibitor_first, inhibitor_kills) = map_objective(&objectives.inhibitor);
    let (rift_herald_first, rift_herald_kills) = map_objective(&objectives.rift_herald);
    let (tower_first, tower_kills) = map_objective(&objectives.tower);

    let (horde_first, horde_kills) = match objectives.horde {
        Some(ref obj) => {
            let (first, kills) = map_objective(obj);
            (Some(first), Some(kills))
        }
        None => (None, None),
    };

    let (atakhan_first, atakhan_kills) = match objectives.atakhan {
        Some(ref obj) => {
            let (first, kills) = map_objective(obj);
            (Some(first), Some(kills))
        }
        None => (None, None),
    };

    let entity = table::objectives::Model {
        match_id,
        team_id,
        baron_first,
        baron_kills,
        champion_first,
        champion_kills,
        dragon_first,
        dragon_kills,
        horde_first,
        horde_kills,
        inhibitor_first,
        inhibitor_kills,
        rift_herald_first,
        rift_herald_kills,
        tower_first,
        tower_kills,
        atakhan_first,
        atakhan_kills,
    };

    Ok(entity)
}

pub fn feats_to_entity(
    match_id: String,
    team_id: i32,
    feats: match_v5::Feats,
) -> Result<table::feats::Model, anyhow::Error> {
    let epic_monster_kill_state = feats.epic_monster_kill.and_then(|feat| feat.feat_state);
    let first_blood_state = feats.first_blood.and_then(|feat| feat.feat_state);
    let first_turret_state = feats.first_turret.and_then(|feat| feat.feat_state);

    let entity = table::feats::Model {
        match_id,
        team_id,
        epic_monster_kill_state,
        first_blood_state,
        first_turret_state,
    };

    Ok(entity)
}
