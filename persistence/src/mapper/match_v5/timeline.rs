use riven::models::match_v5;
use sea_orm::IntoActiveModel;
use uuid::Uuid;

use crate::table;

pub struct TimelineModels {
    pub timeline: table::match_v5::timelines::ActiveModel,
    pub timeline_participants: Vec<table::match_v5::timeline_participants::ActiveModel>,
    pub frames: Vec<table::match_v5::frames::ActiveModel>,
    pub timeline_participant_frames: Vec<table::match_v5::timeline_participant_frames::ActiveModel>,
    pub champion_stats: Vec<table::match_v5::champion_stats::ActiveModel>,
    pub damage_stats: Vec<table::match_v5::damage_stats::ActiveModel>,
    pub events_timeline: Vec<table::match_v5::events_timeline::ActiveModel>,
    pub match_timeline_victim_damage_dealt: Vec<table::match_v5::match_timeline_victim_damage::ActiveModel>,
}

pub fn all(timeline: match_v5::Timeline) -> TimelineModels {
    let match_id = timeline.metadata.match_id.clone();

    let timeline_model = timeline_to_model(timeline.clone());

    let timeline_participants: Vec<table::match_v5::timeline_participants::ActiveModel> = timeline
        .info
        .participants
        .unwrap_or_default()
        .into_iter()
        .map(|pt| participant_timeline_to_model(match_id.clone(), pt))
        .collect();

    let (
        frames,
        timeline_participant_frames,
        champion_stats,
        damage_stats,
        events_timeline,
        match_timeline_victim_damage_dealt,
    ) = timeline.info.frames.into_iter().fold(
        (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()),
        |(mut frames_acc, mut tpfs_acc, mut cs_acc, mut ds_acc, mut et_acc, mut vd_acc), ft| {
            for event in ft.events.clone() {
                let (event_model, victim_models) = events_timeline_to_model(match_id.clone(), event);
                et_acc.push(event_model);
                vd_acc.extend(victim_models);
            }

            let (frame_model, tpf_models, cs_models, ds_models) = frames_to_model(match_id.clone(), ft);
            frames_acc.push(frame_model);
            tpfs_acc.extend(tpf_models);
            cs_acc.extend(cs_models);
            ds_acc.extend(ds_models);

            (frames_acc, tpfs_acc, cs_acc, ds_acc, et_acc, vd_acc)
        },
    );

    TimelineModels {
        timeline: timeline_model,
        timeline_participants,
        frames,
        timeline_participant_frames,
        champion_stats,
        damage_stats,
        events_timeline,
        match_timeline_victim_damage_dealt,
    }
}

fn timeline_to_model(timeline: match_v5::Timeline) -> table::match_v5::timelines::ActiveModel {
    table::match_v5::timelines::Model {
        match_id: timeline.metadata.match_id,
        data_version: timeline.metadata.data_version,
        end_of_game_result: timeline.info.end_of_game_result,
        frame_interval: timeline.info.frame_interval,
        game_id: timeline.info.game_id,
    }
    .into_active_model()
}

fn participant_timeline_to_model(
    match_id: String,
    participant_time_line: match_v5::ParticipantTimeLine,
) -> table::match_v5::timeline_participants::ActiveModel {
    table::match_v5::timeline_participants::Model {
        puuid: participant_time_line.puuid,
        match_id,
        participant_id: participant_time_line.participant_id,
    }
    .into_active_model()
}

fn frames_to_model(
    match_id: String,
    ft: match_v5::FramesTimeLine,
) -> (
    table::match_v5::frames::ActiveModel,
    Vec<table::match_v5::timeline_participant_frames::ActiveModel>,
    Vec<table::match_v5::champion_stats::ActiveModel>,
    Vec<table::match_v5::damage_stats::ActiveModel>,
) {
    let frame_id = Uuid::new_v4();

    let frame_model = table::match_v5::frames::Model {
        frame_id,
        match_id,
        timestamp: ft.timestamp,
    }
    .into_active_model();

    let (timeline_pf_vec, champ_stats_vec, dmg_stats_vec) = ft.participant_frames.unwrap_or_default().into_iter().fold(
        (Vec::new(), Vec::new(), Vec::new()),
        |(mut timelines, mut champs, mut dmg), (_pid, pf)| {
            let (timeline_pf, champ_stats, dmg_stats) = map_full_participant_frame(frame_id, pf);
            timelines.push(timeline_pf);
            champs.push(champ_stats);
            dmg.push(dmg_stats);
            (timelines, champs, dmg)
        },
    );

    (frame_model, timeline_pf_vec, champ_stats_vec, dmg_stats_vec)
}

fn map_full_participant_frame(
    frame_id: Uuid,
    pf: match_v5::ParticipantFrame,
) -> (
    table::match_v5::timeline_participant_frames::ActiveModel,
    table::match_v5::champion_stats::ActiveModel,
    table::match_v5::damage_stats::ActiveModel,
) {
    let timeline_pf = table::match_v5::timeline_participant_frames::Model {
        frame_id,
        participant_id: pf.participant_id,
        current_gold: pf.current_gold,
        gold_per_second: pf.gold_per_second,
        jungle_minions_killed: pf.jungle_minions_killed,
        level: pf.level,
        minions_killed: pf.minions_killed,
        position_x: pf.position.x,
        position_y: pf.position.y,
        time_enemy_spent_controlled: pf.time_enemy_spent_controlled,
        total_gold: pf.total_gold,
        xp: pf.xp,
    }
    .into_active_model();

    let champ_stats = champion_stats_to_model(frame_id, pf.participant_id, pf.champion_stats);

    let dmg_stats = damage_stats_to_model(frame_id, pf.participant_id, pf.damage_stats);

    (timeline_pf, champ_stats, dmg_stats)
}

fn champion_stats_to_model(
    frame_id: Uuid,
    participant_id: i32,
    stats: match_v5::ChampionStats,
) -> table::match_v5::champion_stats::ActiveModel {
    table::match_v5::champion_stats::Model {
        frame_id,
        participant_id,
        ability_haste: stats.ability_haste,
        ability_power: stats.ability_power,
        armor: stats.armor,
        armor_pen: stats.armor_pen,
        armor_pen_percent: stats.armor_pen_percent,
        attack_damage: stats.attack_damage,
        attack_speed: stats.attack_speed,
        bonus_armor_pen_percent: stats.bonus_armor_pen_percent,
        bonus_magic_pen_percent: stats.bonus_magic_pen_percent,
        cc_reduction: stats.cc_reduction,
        cooldown_reduction: stats.cooldown_reduction,
        health: stats.health,
        health_max: stats.health_max,
        health_regen: stats.health_regen,
        lifesteal: stats.lifesteal,
        magic_pen: stats.magic_pen,
        magic_pen_percent: stats.magic_pen_percent,
        magic_resist: stats.magic_resist,
        movement_speed: stats.movement_speed,
        omnivamp: stats.omnivamp,
        physical_vamp: stats.physical_vamp,
        power: stats.power,
        power_max: stats.power_max,
        power_regen: stats.power_regen,
        spell_vamp: stats.spell_vamp,
    }
    .into_active_model()
}

fn damage_stats_to_model(
    frame_id: Uuid,
    participant_id: i32,
    stats: match_v5::DamageStats,
) -> table::match_v5::damage_stats::ActiveModel {
    table::match_v5::damage_stats::Model {
        frame_id,
        participant_id,
        magic_damage_done: stats.magic_damage_done,
        magic_damage_done_to_champions: stats.magic_damage_done_to_champions,
        magic_damage_taken: stats.magic_damage_taken,
        physical_damage_done: stats.physical_damage_done,
        physical_damage_done_to_champions: stats.physical_damage_done_to_champions,
        physical_damage_taken: stats.physical_damage_taken,
        total_damage_done: stats.total_damage_done,
        total_damage_done_to_champions: stats.total_damage_done_to_champions,
        total_damage_taken: stats.total_damage_taken,
        true_damage_done: stats.true_damage_done,
        true_damage_done_to_champions: stats.true_damage_done_to_champions,
        true_damage_taken: stats.true_damage_taken,
    }
    .into_active_model()
}

fn events_timeline_to_model(
    match_id: String,
    events_time_line: match_v5::EventsTimeLine,
) -> (
    table::match_v5::events_timeline::ActiveModel,
    Vec<table::match_v5::match_timeline_victim_damage::ActiveModel>,
) {
    let event_timeline_id = Uuid::new_v4();

    let (position_x, position_y) = if let Some(pos) = events_time_line.position {
        (Some(pos.x), Some(pos.y))
    } else {
        (None, None)
    };

    let team_id = events_time_line.team_id.map(|t| t as i32);
    let killer_team_id = events_time_line.killer_team_id.map(|t| t as i32);

    let event_timeline_model = table::match_v5::events_timeline::Model {
        event_timeline_id,
        match_id,
        timestamp: events_time_line.timestamp,
        real_timestamp: events_time_line.real_timestamp,
        r#type: events_time_line.r#type,
        item_id: events_time_line.item_id,
        participant_id: events_time_line.participant_id,
        level_up_type: events_time_line.level_up_type,
        skill_slot: events_time_line.skill_slot,
        creator_id: events_time_line.creator_id,
        ward_type: events_time_line.ward_type,
        level: events_time_line.level,
        assisting_participant_ids: events_time_line.assisting_participant_ids,
        bounty: events_time_line.bounty,
        kill_streak_length: events_time_line.kill_streak_length,
        killer_id: events_time_line.killer_id,
        position_x,
        position_y,
        victim_id: events_time_line.victim_id,
        kill_type: events_time_line.kill_type,
        lane_type: events_time_line.lane_type,
        team_id,
        multi_kill_length: events_time_line.multi_kill_length,
        killer_team_id,
        monster_type: events_time_line.monster_type,
        monster_sub_type: events_time_line.monster_sub_type,
        building_type: events_time_line.building_type,
        tower_type: events_time_line.tower_type,
        after_id: events_time_line.after_id,
        before_id: events_time_line.before_id,
        gold_gain: events_time_line.gold_gain,
        game_id: events_time_line.game_id,
        winning_team: events_time_line.winning_team,
        transform_type: events_time_line.transform_type,
        name: events_time_line.name,
        shutdown_bounty: events_time_line.shutdown_bounty,
        actual_start_time: events_time_line.actual_start_time,
        feat_type: events_time_line.feat_type,
        feat_value: events_time_line.feat_value,
    }
    .into_active_model();

    let victim_damage_models = {
        let dealt = events_time_line
            .victim_damage_dealt
            .unwrap_or_default()
            .into_iter()
            .map(|d| {
                create_victim_damage_model(
                    d,
                    event_timeline_id,
                    table::match_v5::sea_orm_active_enums::DamageDirection::Dealt,
                )
            });
        let received = events_time_line
            .victim_damage_received
            .unwrap_or_default()
            .into_iter()
            .map(|d| {
                create_victim_damage_model(
                    d,
                    event_timeline_id,
                    table::match_v5::sea_orm_active_enums::DamageDirection::Received,
                )
            });
        dealt.chain(received).collect()
    };

    (event_timeline_model, victim_damage_models)
}

fn create_victim_damage_model(
    d: match_v5::MatchTimelineVictimDamage,
    event_timeline_id: Uuid,
    direction: table::match_v5::sea_orm_active_enums::DamageDirection,
) -> table::match_v5::match_timeline_victim_damage::ActiveModel {
    table::match_v5::match_timeline_victim_damage::Model {
        id: Uuid::new_v4(),
        event_timeline_id,
        basic: d.basic,
        magic_damage: d.magic_damage,
        name: d.name,
        participant_id: d.participant_id,
        physical_damage: d.physical_damage,
        spell_name: d.spell_name,
        spell_slot: d.spell_slot,
        true_damage: d.true_damage,
        r#type: d.r#type,
        damage_direction: direction,
    }
    .into_active_model()
}
