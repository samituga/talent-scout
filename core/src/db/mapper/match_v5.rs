use std::sync::Arc;

use riven::{consts, models::match_v5};

use crate::db::{
    mapper::util::{f32_to_decimal, f64_to_decimal, i32_to_bool},
    table,
};

pub struct Models {
    pub r#match: table::matches::Model,
    pub teams: Vec<table::teams::Model>,
    pub bans: Vec<table::bans::Model>,
    pub objectives: Vec<table::objectives::Model>,
    pub feats: Vec<table::feats::Model>,
    pub challenges: Vec<table::challenges::Model>,
    pub missions: Vec<table::missions::Model>,
    pub participants: Vec<table::participants::Model>,
    pub perks: Vec<(
        table::participant_perks::Model,
        Vec<(table::perk_styles::Model, Vec<table::perk_style_selections::Model>)>,
    )>,
}

pub fn all(m: match_v5::Match) -> Result<Models, anyhow::Error> {
    let match_model = match_to_model(&m);

    let teams = m.info.teams;

    let feats_vec: Vec<(consts::Team, Option<match_v5::Feats>)> =
        teams.iter().map(|t| (t.team_id, t.feats.clone())).collect();

    let objectives: Vec<(consts::Team, match_v5::Objectives)> =
        teams.iter().map(|t| (t.team_id, t.objectives.clone())).collect();

    let bans_vec: Vec<(consts::Team, Vec<match_v5::Ban>)> =
        teams.iter().map(|t| (t.team_id.clone(), t.bans.clone())).collect();

    let participants = m.info.participants;

    let match_id = Arc::new(Box::new(m.metadata.match_id.clone()));

    let challenges: Vec<(String, Option<match_v5::Challenges>)> = participants
        .iter()
        .map(|p| (p.puuid.clone(), p.challenges.clone()))
        .collect();

    let missions: Vec<(String, Option<match_v5::Missions>)> = participants
        .iter()
        .map(|p| (p.puuid.clone(), p.missions.clone()))
        .collect();

    let perks: Vec<(String, match_v5::Perks)> = participants
        .iter()
        .map(|p| (p.puuid.clone(), p.perks.clone()))
        .collect();

    let feats_model: Vec<table::feats::Model> = feats_vec
        .into_iter()
        .filter(|(_, feats)| feats.is_some())
        .map(|(team_id, feats)| (team_id, feats.unwrap()))
        .map(|(team_id, feats)| feats_to_model(*match_id.as_ref().clone(), team_id as i32, feats))
        .collect();

    let objectives_model: Vec<table::objectives::Model> = objectives
        .into_iter()
        .map(|(team_id, objectives)| objectives_to_model(*match_id.as_ref().clone(), team_id as i32, objectives))
        .collect();

    let bans_model: Vec<table::bans::Model> = bans_vec
        .into_iter()
        .flat_map(|(team_id, bans)| {
            let team_id_other = team_id.clone();
            let match_id_other = *match_id.as_ref().clone();
            bans.into_iter()
                .map(move |ban| ban_to_model(match_id_other.clone(), team_id_other as i32, ban))
        })
        .collect();

    let teams_model: Vec<table::teams::Model> = teams
        .into_iter()
        .map(|team| team_to_model(*match_id.as_ref().clone(), team))
        .collect();

    let perks_model: Vec<(
        table::participant_perks::Model,
        Vec<(table::perk_styles::Model, Vec<table::perk_style_selections::Model>)>,
    )> = perks
        .into_iter()
        .map(|(puuid, perks)| perks_to_model(*match_id.as_ref().clone(), puuid, perks))
        .collect();

    let missions_model: Vec<table::missions::Model> = missions
        .into_iter()
        .filter(|(_, missions)| missions.is_some())
        .map(|(puuid, missions)| (puuid, missions.unwrap()))
        .map(|(puuid, missions)| missions_to_model(*match_id.as_ref().clone(), puuid, missions))
        .collect();

    let challenges_model: Vec<table::challenges::Model> = challenges
        .into_iter()
        .filter(|(_, challenges)| challenges.is_some())
        .map(|(puuid, challenges)| (puuid, challenges.unwrap()))
        .map(|(puuid, challenges)| challenges_to_model(*match_id.as_ref().clone(), puuid, challenges))
        .collect();

    let participants_model: Vec<table::participants::Model> = participants
        .into_iter()
        .map(|participant| participants_to_model(participant, *match_id.as_ref().clone()))
        .collect();

    let models = Models {
        r#match: match_model,
        teams: teams_model,
        bans: bans_model,
        objectives: objectives_model,
        feats: feats_model,
        challenges: challenges_model,
        missions: missions_model,
        participants: participants_model,
        perks: perks_model,
    };

    Ok(models)
}

// TODO check if this is ok, many clones
fn match_to_model(m: &match_v5::Match) -> table::matches::Model {
    let metadata = &m.metadata;
    let info = &m.info;

    let game_type = info.game_type.as_ref().map(|e| e.to_string()).unwrap_or_default();
    let game_mode = info.game_mode.to_string();

    table::matches::Model {
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
    }
}

fn participants_to_model(participant: match_v5::Participant, match_id: String) -> table::participants::Model {
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

    table::participants::Model {
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
        champion_name: participant.champion_name,
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
        individual_position: participant.individual_position,
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
        lane: participant.lane,
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
        puuid: participant.puuid,
        quadra_kills: participant.quadra_kills,
        riot_id_game_name: participant.riot_id_game_name,
        riot_id_tagline: participant.riot_id_tagline,
        role: participant.role,
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
        summoner_id: participant.summoner_id,
        summoner_level: participant.summoner_level,
        summoner_name: participant.summoner_name,
        team_early_surrendered: participant.team_early_surrendered,
        team_id: participant.team_id as i32,
        team_position: participant.team_position,
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
    }
}

pub fn challenges_to_model(
    match_id: String,
    puuid: String,
    challenges: match_v5::Challenges,
) -> table::challenges::Model {
    let control_ward_time_coverage_in_river_or_enemy_half = challenges
        .control_ward_time_coverage_in_river_or_enemy_half
        .map(f32_to_decimal);
    let earliest_baron = challenges.earliest_baron.map(f64_to_decimal);
    let earliest_dragon_takedown = challenges.earliest_dragon_takedown.map(f64_to_decimal);
    let earliest_elder_dragon = challenges.earliest_elder_dragon.map(f64_to_decimal);
    let early_laning_phase_gold_exp_advantage = challenges.early_laning_phase_gold_exp_advantage.map(f64_to_decimal);
    let faster_support_quest_completion = challenges.faster_support_quest_completion.map(i32_to_bool);
    let fastest_legendary = challenges.fastest_legendary.map(f64_to_decimal);
    let had_afk_teammate = challenges.had_afk_teammate.map(i32_to_bool);
    let highest_ward_kills = challenges.highest_ward_kills.map(i32_to_bool);
    let laning_phase_gold_exp_advantage = challenges.laning_phase_gold_exp_advantage.map(i32_to_bool);
    let highest_crowd_control_score = challenges.highest_crowd_control_score.map(i32_to_bool);
    let max_cs_advantage_on_lane_opponent = challenges.max_cs_advantage_on_lane_opponent.map(f32_to_decimal);
    let played_champ_select_position = challenges.played_champ_select_position.map(i32_to_bool);
    let third_inhibitor_destroyed_time = challenges.third_inhibitor_destroyed_time.map(f64_to_decimal);
    let gold_per_minute = challenges.gold_per_minute.map(f32_to_decimal);
    let vision_score_advantage_lane_opponent = challenges.vision_score_advantage_lane_opponent.map(f32_to_decimal);
    let game_length = challenges.game_length.map(f32_to_decimal);
    let allied_jungle_monster_kills = challenges.allied_jungle_monster_kills.map(f32_to_decimal);
    let bounty_gold = challenges.bounty_gold.map(f64_to_decimal);
    let damage_per_minute = challenges.damage_per_minute.map(f32_to_decimal);
    let damage_taken_on_team_percentage = challenges.damage_taken_on_team_percentage.map(f32_to_decimal);
    let effective_heal_and_shielding = challenges.effective_heal_and_shielding.map(f32_to_decimal);
    let enemy_jungle_monster_kills = challenges.enemy_jungle_monster_kills.map(f32_to_decimal);
    let first_turret_killed = challenges.first_turret_killed.map(f64_to_decimal);
    let first_turret_killed_time = challenges.first_turret_killed_time.map(f32_to_decimal);
    let jungle_cs_before10_minutes = challenges.jungle_cs_before10_minutes.map(f32_to_decimal);
    let kda = challenges.kda.map(f32_to_decimal);
    let kill_participation = challenges.kill_participation.map(f32_to_decimal);
    let more_enemy_jungle_than_opponent = challenges.more_enemy_jungle_than_opponent.map(f32_to_decimal);
    let shortest_time_to_ace_from_first_takedown =
        challenges.shortest_time_to_ace_from_first_takedown.map(f32_to_decimal);
    let swarm_pickup_gold = challenges.swarm_pickup_gold.map(f32_to_decimal);
    let team_damage_percentage = challenges.team_damage_percentage.map(f32_to_decimal);
    let vision_score_per_minute = challenges.vision_score_per_minute.map(f32_to_decimal);
    let heal_from_map_sources = challenges.heal_from_map_sources.map(f64_to_decimal);

    table::challenges::Model {
        match_id,
        puuid,
        x_12_assist_streak_count: challenges.x12_assist_streak_count,
        baron_buff_gold_advantage_over_threshold: challenges.baron_buff_gold_advantage_over_threshold,
        control_ward_time_coverage_in_river_or_enemy_half,
        earliest_baron,
        earliest_dragon_takedown,
        earliest_elder_dragon,
        early_laning_phase_gold_exp_advantage,
        faster_support_quest_completion,
        fastest_legendary,
        had_afk_teammate,
        highest_champion_damage: challenges.highest_champion_damage,
        highest_crowd_control_score,
        highest_ward_kills,
        jungler_kills_early_jungle: challenges.jungler_kills_early_jungle,
        kills_on_laners_early_jungle_as_jungler: challenges.kills_on_laners_early_jungle_as_jungler,
        laning_phase_gold_exp_advantage,
        legendary_count: challenges.legendary_count,
        max_cs_advantage_on_lane_opponent,
        max_level_lead_lane_opponent: challenges.max_level_lead_lane_opponent,
        most_wards_destroyed_one_sweeper: challenges.most_wards_destroyed_one_sweeper,
        mythic_item_used: challenges.mythic_item_used,
        played_champ_select_position,
        solo_turrets_lategame: challenges.solo_turrets_lategame,
        takedowns_first25_minutes: challenges.takedowns_first25_minutes,
        teleport_takedowns: challenges.teleport_takedowns,
        third_inhibitor_destroyed_time,
        three_wards_one_sweeper_count: challenges.three_wards_one_sweeper_count,
        vision_score_advantage_lane_opponent,
        infernal_scale_pickup: challenges.infernal_scale_pickup,
        fist_bump_participation: challenges.fist_bump_participation,
        void_monster_kill: challenges.void_monster_kill,
        ability_uses: challenges.ability_uses,
        aces_before15_minutes: challenges.aces_before15_minutes,
        allied_jungle_monster_kills,
        baron_takedowns: challenges.baron_takedowns,
        blast_cone_opposite_opponent_count: challenges.blast_cone_opposite_opponent_count,
        bounty_gold,
        buffs_stolen: challenges.buffs_stolen,
        complete_support_quest_in_time: challenges.complete_support_quest_in_time,
        control_wards_placed: challenges.control_wards_placed,
        damage_per_minute,
        damage_taken_on_team_percentage,
        danced_with_rift_herald: challenges.danced_with_rift_herald,
        deaths_by_enemy_champs: challenges.deaths_by_enemy_champs,
        dodge_skill_shots_small_window: challenges.dodge_skill_shots_small_window,
        double_aces: challenges.double_aces,
        dragon_takedowns: challenges.dragon_takedowns,
        legendary_item_used: challenges.legendary_item_used,
        effective_heal_and_shielding,
        elder_dragon_kills_with_opposing_soul: challenges.elder_dragon_kills_with_opposing_soul,
        elder_dragon_multikills: challenges.elder_dragon_multikills,
        enemy_champion_immobilizations: challenges.enemy_champion_immobilizations,
        enemy_jungle_monster_kills,
        epic_monster_kills_near_enemy_jungler: challenges.epic_monster_kills_near_enemy_jungler,
        epic_monster_kills_within30_seconds_of_spawn: challenges.epic_monster_kills_within30_seconds_of_spawn,
        epic_monster_steals: challenges.epic_monster_steals,
        epic_monster_stolen_without_smite: challenges.epic_monster_stolen_without_smite,
        first_turret_killed,
        first_turret_killed_time,
        flawless_aces: challenges.flawless_aces,
        full_team_takedown: challenges.full_team_takedown,
        game_length,
        get_takedowns_in_all_lanes_early_jungle_as_laner: challenges.get_takedowns_in_all_lanes_early_jungle_as_laner,
        gold_per_minute,
        had_open_nexus: challenges.had_open_nexus,
        immobilize_and_kill_with_ally: challenges.immobilize_and_kill_with_ally,
        initial_buff_count: challenges.initial_buff_count,
        initial_crab_count: challenges.initial_crab_count,
        jungle_cs_before10_minutes,
        jungler_takedowns_near_damaged_epic_monster: challenges.jungler_takedowns_near_damaged_epic_monster,
        kda,
        kill_after_hidden_with_ally: challenges.kill_after_hidden_with_ally,
        killed_champ_took_full_team_damage_survived: challenges.killed_champ_took_full_team_damage_survived,
        killing_sprees: challenges.killing_sprees,
        kill_participation,
        kills_near_enemy_turret: challenges.kills_near_enemy_turret,
        kills_on_other_lanes_early_jungle_as_laner: challenges.kills_on_other_lanes_early_jungle_as_laner,
        kills_on_recently_healed_by_aram_pack: challenges.kills_on_recently_healed_by_aram_pack,
        kills_under_own_turret: challenges.kills_under_own_turret,
        kills_with_help_from_epic_monster: challenges.kills_with_help_from_epic_monster,
        knock_enemy_into_team_and_kill: challenges.knock_enemy_into_team_and_kill,
        k_turrets_destroyed_before_plates_fall: challenges.k_turrets_destroyed_before_plates_fall,
        land_skill_shots_early_game: challenges.land_skill_shots_early_game,
        lane_minions_first10_minutes: challenges.lane_minions_first10_minutes,
        lost_an_inhibitor: challenges.lost_an_inhibitor,
        max_kill_deficit: challenges.max_kill_deficit,
        mejais_full_stack_in_time: challenges.mejais_full_stack_in_time,
        more_enemy_jungle_than_opponent,
        multi_kill_one_spell: challenges.multi_kill_one_spell,
        multikills: challenges.multikills,
        multikills_after_aggressive_flash: challenges.multikills_after_aggressive_flash,
        multi_turret_rift_herald_count: challenges.multi_turret_rift_herald_count,
        outer_turret_executes_before10_minutes: challenges.outer_turret_executes_before10_minutes,
        outnumbered_kills: challenges.outnumbered_kills,
        outnumbered_nexus_kill: challenges.outnumbered_nexus_kill,
        perfect_dragon_souls_taken: challenges.perfect_dragon_souls_taken,
        perfect_game: challenges.perfect_game,
        pick_kill_with_ally: challenges.pick_kill_with_ally,
        poro_explosions: challenges.poro_explosions,
        quick_cleanse: challenges.quick_cleanse,
        quick_first_turret: challenges.quick_first_turret,
        quick_solo_kills: challenges.quick_solo_kills,
        rift_herald_takedowns: challenges.rift_herald_takedowns,
        save_ally_from_death: challenges.save_ally_from_death,
        scuttle_crab_kills: challenges.scuttle_crab_kills,
        shortest_time_to_ace_from_first_takedown,
        skillshots_dodged: challenges.skillshots_dodged,
        skillshots_hit: challenges.skillshots_hit,
        snowballs_hit: challenges.snowballs_hit,
        solo_baron_kills: challenges.solo_baron_kills,
        swarm_defeat_aatrox: challenges.swarm_defeat_aatrox,
        swarm_defeat_briar: challenges.swarm_defeat_briar,
        swarm_defeat_mini_bosses: challenges.swarm_defeat_mini_bosses,
        swarm_evolve_weapon: challenges.swarm_evolve_weapon,
        swarm_have3_passives: challenges.swarm_have3_passives,
        swarm_kill_enemy: challenges.swarm_kill_enemy,
        swarm_pickup_gold,
        swarm_reach_level50: challenges.swarm_reach_level50,
        swarm_survive15_min: challenges.swarm_survive15_min,
        swarm_win_with5_evolved_weapons: challenges.swarm_win_with5_evolved_weapons,
        solo_kills: challenges.solo_kills,
        stealth_wards_placed: challenges.stealth_wards_placed,
        survived_single_digit_hp_count: challenges.survived_single_digit_hp_count,
        survived_three_immobilizes_in_fight: challenges.survived_three_immobilizes_in_fight,
        takedown_on_first_turret: challenges.takedown_on_first_turret,
        takedowns: challenges.takedowns,
        takedowns_after_gaining_level_advantage: challenges.takedowns_after_gaining_level_advantage,
        takedowns_before_jungle_minion_spawn: challenges.takedowns_before_jungle_minion_spawn,
        takedowns_first_x_minutes: challenges.takedowns_first_x_minutes,
        takedowns_in_alcove: challenges.takedowns_in_alcove,
        takedowns_in_enemy_fountain: challenges.takedowns_in_enemy_fountain,
        team_baron_kills: challenges.team_baron_kills,
        team_damage_percentage,
        team_elder_dragon_kills: challenges.team_elder_dragon_kills,
        team_rift_herald_kills: challenges.team_rift_herald_kills,
        took_large_damage_survived: challenges.took_large_damage_survived,
        turret_plates_taken: challenges.turret_plates_taken,
        turrets_taken_with_rift_herald: challenges.turrets_taken_with_rift_herald,
        turret_takedowns: challenges.turret_takedowns,
        twenty_minions_in3_seconds_count: challenges.twenty_minions_in3_seconds_count,
        two_wards_one_sweeper_count: challenges.two_wards_one_sweeper_count,
        unseen_recalls: challenges.unseen_recalls,
        vision_score_per_minute,
        wards_guarded: challenges.wards_guarded,
        ward_takedowns: challenges.ward_takedowns,
        ward_takedowns_before20_m: challenges.ward_takedowns_before20_m,
        heal_from_map_sources,
    }
}

fn missions_to_model(match_id: String, puuid: String, missions: match_v5::Missions) -> table::missions::Model {
    let player_score0 = missions.player_score0.map(f32_to_decimal);
    let player_score1 = missions.player_score1.map(f32_to_decimal);
    let player_score2 = missions.player_score2.map(f32_to_decimal);
    let player_score3 = missions.player_score3.map(f32_to_decimal);
    let player_score4 = missions.player_score4.map(f32_to_decimal);
    let player_score5 = missions.player_score5.map(f32_to_decimal);
    let player_score6 = missions.player_score6.map(f32_to_decimal);
    let player_score7 = missions.player_score7.map(f32_to_decimal);
    let player_score8 = missions.player_score8.map(f32_to_decimal);
    let player_score9 = missions.player_score9.map(f32_to_decimal);
    let player_score10 = missions.player_score10.map(f32_to_decimal);
    let player_score11 = missions.player_score11.map(f32_to_decimal);

    table::missions::Model {
        match_id,
        puuid,
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
    }
}

// TODO this doesn't look good, the ids
// TODO maybe use UUID instead of relying on the BD to generate the ids
fn perks_to_model(
    match_id: String,
    puuid: String,
    perks: match_v5::Perks,
) -> (
    table::participant_perks::Model,
    Vec<(table::perk_styles::Model, Vec<table::perk_style_selections::Model>)>,
) {
    let participant_perks = table::participant_perks::Model {
        match_id: match_id.clone(),
        puuid: puuid.clone(),
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
                puuid: puuid.clone(),
                description: style.description.clone(),
                style: style.style,
            };

            let selections: Vec<table::perk_style_selections::Model> = style
                .selections
                .into_iter()
                .map(|sel| table::perk_style_selections::Model {
                    perk_style_selection_id: 0, // SERIAL
                    perk_style_id: 0,           // Needs to be updated with correct id after perk style insert
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

fn team_to_model(match_id: String, team: match_v5::Team) -> table::teams::Model {
    table::teams::Model {
        match_id,
        team_id: team.team_id as i32,
        win: team.win,
    }
}

fn ban_to_model(match_id: String, team_id: i32, ban: match_v5::Ban) -> table::bans::Model {
    table::bans::Model {
        ban_id: 0, // SERIAL
        match_id,
        team_id,
        champion_id: ban.champion_id.0 as i32,
        pick_turn: ban.pick_turn,
    }
}

fn objectives_to_model(match_id: String, team_id: i32, objectives: match_v5::Objectives) -> table::objectives::Model {
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

    table::objectives::Model {
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
    }
}

fn feats_to_model(match_id: String, team_id: i32, feats: match_v5::Feats) -> table::feats::Model {
    let epic_monster_kill_state = feats.epic_monster_kill.and_then(|feat| feat.feat_state);
    let first_blood_state = feats.first_blood.and_then(|feat| feat.feat_state);
    let first_turret_state = feats.first_turret.and_then(|feat| feat.feat_state);

    table::feats::Model {
        match_id,
        team_id,
        epic_monster_kill_state,
        first_blood_state,
        first_turret_state,
    }
}
