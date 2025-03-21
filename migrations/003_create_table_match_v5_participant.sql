CREATE TABLE match_v5.participant
(
    match_id                           TEXT    NOT NULL, -- Identifier of the match (parent key)
    participant_id                     INTEGER NOT NULL, -- The participant's index/order in the match
    all_in_pings                       INTEGER,
    assist_me_pings                    INTEGER,
    assists                            INTEGER NOT NULL,
    baron_kills                        INTEGER NOT NULL,
    bounty_level                       INTEGER NOT NULL,
    champ_experience                   INTEGER NOT NULL,
    champ_level                        INTEGER NOT NULL,
    champion_id                        INTEGER NOT NULL,
    champion_name                      TEXT    NOT NULL,
    champion_transform                 INTEGER NOT NULL,
    consumables_purchased              INTEGER NOT NULL,
    command_pings                      INTEGER,

    -- Nested object: challenges (to be normalized in a separate table)
    -- challenges JSONB, -- (Normalization for challenges will be handled in a separate table)

    damage_dealt_to_buildings          INTEGER,
    damage_dealt_to_objectives         INTEGER NOT NULL,
    damage_dealt_to_turrets            INTEGER NOT NULL,
    damage_self_mitigated              INTEGER NOT NULL,
    deaths                             INTEGER NOT NULL,
    detector_wards_placed              INTEGER NOT NULL,
    double_kills                       INTEGER NOT NULL,
    dragon_kills                       INTEGER NOT NULL,
    eligible_for_progression           BOOLEAN,
    enemy_missing_pings                INTEGER,
    enemy_vision_pings                 INTEGER,
    first_blood_assist                 BOOLEAN NOT NULL,
    first_blood_kill                   BOOLEAN NOT NULL,
    first_tower_assist                 BOOLEAN NOT NULL,
    first_tower_kill                   BOOLEAN NOT NULL,
    game_ended_in_early_surrender      BOOLEAN NOT NULL,
    game_ended_in_surrender            BOOLEAN NOT NULL,
    hold_pings                         INTEGER,
    get_back_pings                     INTEGER,
    gold_earned                        INTEGER NOT NULL,
    gold_spent                         INTEGER NOT NULL,
    individual_position                TEXT    NOT NULL,
    inhibitor_kills                    INTEGER NOT NULL,
    inhibitor_takedowns                INTEGER,
    inhibitors_lost                    INTEGER,
    item0                              INTEGER NOT NULL,
    item1                              INTEGER NOT NULL,
    item2                              INTEGER NOT NULL,
    item3                              INTEGER NOT NULL,
    item4                              INTEGER NOT NULL,
    item5                              INTEGER NOT NULL,
    item6                              INTEGER NOT NULL,
    items_purchased                    INTEGER NOT NULL,
    killing_sprees                     INTEGER NOT NULL,
    kills                              INTEGER NOT NULL,
    lane                               TEXT    NOT NULL,
    largest_critical_strike            INTEGER NOT NULL,
    largest_killing_spree              INTEGER NOT NULL,
    largest_multi_kill                 INTEGER NOT NULL,
    longest_time_spent_living          INTEGER NOT NULL,
    magic_damage_dealt                 INTEGER NOT NULL,
    magic_damage_dealt_to_champions    INTEGER NOT NULL,
    magic_damage_taken                 INTEGER NOT NULL,

    -- Nested object: missions (to be normalized in a separate table)
    -- missions JSONB, -- (Normalization for missions will be handled separately)

    neutral_minions_killed             INTEGER NOT NULL,
    need_vision_pings                  INTEGER,
    nexus_kills                        INTEGER NOT NULL,
    nexus_takedowns                    INTEGER NOT NULL,
    nexus_lost                         INTEGER NOT NULL,
    objectives_stolen                  INTEGER NOT NULL,
    objectives_stolen_assists          INTEGER NOT NULL,
    on_my_way_pings                    INTEGER,
    player_score0                      REAL,
    player_score1                      REAL,
    player_score2                      REAL,
    player_score3                      REAL,
    player_score4                      REAL,
    player_score5                      REAL,
    player_score6                      REAL,
    player_score7                      REAL,
    player_score8                      REAL,
    player_score9                      REAL,
    player_score10                     REAL,
    player_score11                     REAL,
    penta_kills                        INTEGER NOT NULL,

    -- Nested object: perks (to be normalized in a separate table)
    -- perks JSONB, -- (Normalization for perks will be handled in a separate table)

    physical_damage_dealt              INTEGER NOT NULL,
    physical_damage_dealt_to_champions INTEGER NOT NULL,
    physical_damage_taken              INTEGER NOT NULL,
    placement                          INTEGER NOT NULL,
    player_augment1                    INTEGER,
    player_augment2                    INTEGER,
    player_augment3                    INTEGER,
    player_augment4                    INTEGER,
    player_subteam_id                  INTEGER,
    push_pings                         INTEGER,
    profile_icon                       INTEGER NOT NULL,
    puuid                              TEXT    NOT NULL, -- TODO Should be foreign key
    quadra_kills                       INTEGER NOT NULL,
    riot_id_game_name                  TEXT,
    riot_id_tagline                    TEXT,
    role                               TEXT    NOT NULL,
    sight_wards_bought_in_game         INTEGER NOT NULL,
    spell1_casts                       INTEGER NOT NULL,
    spell2_casts                       INTEGER NOT NULL,
    spell3_casts                       INTEGER NOT NULL,
    spell4_casts                       INTEGER NOT NULL,
    subteam_placement                  INTEGER,
    summoner1_casts                    INTEGER NOT NULL,
    summoner1_id                       INTEGER NOT NULL,
    summoner2_casts                    INTEGER NOT NULL,
    summoner2_id                       INTEGER NOT NULL,
    summoner_id                        TEXT    NOT NULL,
    summoner_level                     INTEGER NOT NULL,
    summoner_name                      TEXT    NOT NULL,
    team_early_surrendered             BOOLEAN NOT NULL,
    team_id                            INTEGER NOT NULL, -- TODO define if we use the integer or the string representation of the enum
    team_position                      TEXT    NOT NULL,
    time_ccing_others                  INTEGER NOT NULL,
    time_played                        INTEGER NOT NULL,
    total_ally_jungle_minions_killed   INTEGER,
    total_damage_dealt                 INTEGER NOT NULL,
    total_damage_dealt_to_champions    INTEGER NOT NULL,
    total_damage_shielded_on_teammates INTEGER NOT NULL,
    total_damage_taken                 INTEGER NOT NULL,
    total_enemy_jungle_minions_killed  INTEGER NOT NULL,
    total_heal                         INTEGER NOT NULL,
    total_heals_on_teammates           INTEGER NOT NULL,
    total_minions_killed               INTEGER NOT NULL,
    total_time_cc_dealt                INTEGER NOT NULL,
    total_time_spent_dead              INTEGER NOT NULL,
    total_units_healed                 INTEGER NOT NULL,
    triple_kills                       INTEGER NOT NULL,
    true_damage_dealt                  INTEGER NOT NULL,
    true_damage_dealt_to_champions     INTEGER NOT NULL,
    true_damage_taken                  INTEGER NOT NULL,
    turret_kills                       INTEGER NOT NULL,
    turret_takedowns                   INTEGER NOT NULL,
    turrets_lost                       INTEGER NOT NULL,
    unreal_kills                       INTEGER NOT NULL,
    vision_score                       INTEGER NOT NULL,
    vision_cleared_pings               INTEGER,
    vision_wards_bought_in_game        INTEGER NOT NULL,
    wards_killed                       INTEGER NOT NULL,
    wards_placed                       INTEGER NOT NULL,
    win                                BOOLEAN NOT NULL,

    bait_pings                         INTEGER,
    danger_pings                       INTEGER,
    basic_pings                        INTEGER,
    player_augment5                    INTEGER,
    player_augment6                    INTEGER,
    riot_id_name                       TEXT,
    retreat_pings                      INTEGER,

    PRIMARY KEY (match_id, participant_id),
    FOREIGN KEY (match_id) REFERENCES match_v5.match (match_id)
);

-- Add column comments for some key fields (only a sample; you can extend this for all columns)
COMMENT ON COLUMN match_v5.participant.all_in_pings IS 'Yellow crossed swords.';
COMMENT ON COLUMN match_v5.participant.assist_me_pings IS 'Green flag.';
COMMENT ON COLUMN match_v5.participant.command_pings IS 'Blue generic ping (ALT+click).';
COMMENT ON COLUMN match_v5.participant.champion_id IS 'Prior to patch 11.4 this field might be unreliable; use champion_name for accurate identification.';
COMMENT ON COLUMN match_v5.participant.champion_transform IS 'This field is currently only utilized for Kayn''s transformations. (Legal values: 0 - None, 1 - Slayer, 2 - Assassin).';
COMMENT ON COLUMN match_v5.participant.enemy_missing_pings IS 'Yellow questionmark.';
COMMENT ON COLUMN match_v5.participant.enemy_vision_pings IS 'Red eyeball.';
COMMENT ON COLUMN match_v5.participant.get_back_pings IS 'Yellow circle with horizontal line.';
COMMENT ON COLUMN match_v5.participant.individual_position IS 'Both individualPosition and teamPosition are computed by the game server and are different versions of the most likely position played by a player. The individualPosition is the best guess for which position the player actually played in isolation of anything else. The teamPosition is the best guess for which position the player actually played if we add the constraint that each team must have one top player, one jungle, one middle, etc. Generally the recommendation is to use the teamPosition field over the individualPosition field.';
COMMENT ON COLUMN match_v5.participant.neutral_minions_killed IS 'neutralMinionsKilled = mNeutralMinionsKilled, which is incremented on kills of kPet and kJungleMonster.';
COMMENT ON COLUMN match_v5.participant.need_vision_pings IS 'Green ward.';
COMMENT ON COLUMN match_v5.participant.on_my_way_pings IS 'Blue arrow pointing at ground.';
COMMENT ON COLUMN match_v5.participant.push_pings IS 'Green minion.';
COMMENT ON COLUMN match_v5.participant.team_position IS 'Both individualPosition and teamPosition are computed by the game server and are different versions of the most likely position played by a player. The individualPosition is the best guess for which position the player actually played in isolation of anything else. The teamPosition is the best guess for which position the player actually played if we add the constraint that each team must have one top player, one jungle, one middle, etc. Generally the recommendation is to use the teamPosition field over the individualPosition field.';
COMMENT ON COLUMN match_v5.participant.total_heal IS 'Whenever positive health is applied (which translates to all heals in the game but not things like regeneration), totalHeal is incremented by the amount of health received. This includes healing enemies, jungle monsters, yourself, etc.';
COMMENT ON COLUMN match_v5.participant.total_heals_on_teammates IS 'Whenever positive health is applied (which translates to all heals in the game but not things like regeneration), totalHealsOnTeammates is incremented by the amount of health received.  This is post modified, so if you heal someone missing 5 health for 100 you will get +5 totalHealsOnTeammates.';
COMMENT ON COLUMN match_v5.participant.total_minions_killed IS 'totalMillionsKilled = mMinionsKilled, which is only incremented on kills of kTeamMinion, kMeleeLaneMinion, kSuperLaneMinion, kRangedLaneMinion and kSiegeLaneMinion.';
COMMENT ON COLUMN match_v5.participant.puuid IS 'Encrypted PUUID of the participant.'; -- TODO is it really encrypted? docs say nothing of that
COMMENT ON COLUMN match_v5.participant.danger_pings IS 'https://github.com/RiotGames/developer-relations/issues/870.';
COMMENT ON COLUMN match_v5.participant.basic_pings IS 'https://github.com/RiotGames/developer-relations/issues/814.';
COMMENT ON COLUMN match_v5.participant.riot_id_name IS 'Deprecated, use `riotIdGameName`. This field name was briefly used instead of `riotIdGameName`, prior to patch 14.5.';
COMMENT ON COLUMN match_v5.participant.retreat_pings IS 'https://github.com/RiotGames/developer-relations/issues/814';

-- Reminder: The nested objects “challenges”, “missions”, and “perks” are not included here.
-- Please provide their definitions (or let me know which fields you would like to include) so that we can
-- create normalized tables for them.
