CREATE TABLE match_v5.frames
(
    frame_id    UUID PRIMARY KEY,
    match_id    TEXT    NOT NULL,

    "timestamp" INTEGER NOT NULL,
    FOREIGN KEY (match_id) REFERENCES match_v5.timelines (match_id)
);

COMMENT ON TABLE match_v5.frames IS 'Original model name - match-v5.FramesTimeLineDto.';

CREATE TABLE match_v5.timeline_participant_frames
(
    frame_id                    UUID    NOT NULL,
    participant_id              INTEGER NOT NULL, -- TODO maybe puuid could be better here

    current_gold                INTEGER NOT NULL,
    gold_per_second             INTEGER NOT NULL,
    jungle_minions_killed       INTEGER NOT NULL,
    "level"                     INTEGER NOT NULL,
    minions_killed              INTEGER NOT NULL,
    position_x                  INTEGER NOT NULL,
    position_y                  INTEGER NOT NULL,
    time_enemy_spent_controlled INTEGER NOT NULL,
    total_gold                  INTEGER NOT NULL,
    xp                          INTEGER NOT NULL,
    PRIMARY KEY (frame_id, participant_id),
    FOREIGN KEY (frame_id) REFERENCES match_v5.frames (frame_id)
);

COMMENT ON TABLE match_v5.timeline_participant_frames IS 'Original model name - match-v5.ParticipantFrameDto.';
COMMENT ON COLUMN match_v5.timeline_participant_frames.current_gold IS 'Original param name - currentGold.';
COMMENT ON COLUMN match_v5.timeline_participant_frames.gold_per_second IS 'Original param name - goldPerSecond.';
COMMENT ON COLUMN match_v5.timeline_participant_frames.jungle_minions_killed IS 'Original param name - jungleMinionsKilled.';
COMMENT ON COLUMN match_v5.timeline_participant_frames.minions_killed IS 'Original param name - minionsKilled.';
COMMENT ON COLUMN match_v5.timeline_participant_frames.participant_id IS 'Original param name - participantId.';
COMMENT ON COLUMN match_v5.timeline_participant_frames.time_enemy_spent_controlled IS 'Original param name - timeEnemySpentControlled.';
COMMENT ON COLUMN match_v5.timeline_participant_frames.total_gold IS 'Original param name - totalGold.';


CREATE TABLE match_v5.champion_stats
(
    frame_id                UUID    NOT NULL,
    participant_id          INTEGER NOT NULL,

    ability_haste           INTEGER,
    ability_power           INTEGER NOT NULL,
    armor                   INTEGER NOT NULL,
    armor_pen               INTEGER NOT NULL,
    armor_pen_percent       INTEGER NOT NULL,
    attack_damage           INTEGER NOT NULL,
    attack_speed            INTEGER NOT NULL,
    bonus_armor_pen_percent INTEGER NOT NULL,
    bonus_magic_pen_percent INTEGER NOT NULL,
    cc_reduction            INTEGER NOT NULL,
    cooldown_reduction      INTEGER NOT NULL,
    health                  INTEGER NOT NULL,
    health_max              INTEGER NOT NULL,
    health_regen            INTEGER NOT NULL,
    lifesteal               INTEGER NOT NULL,
    magic_pen               INTEGER NOT NULL,
    magic_pen_percent       INTEGER NOT NULL,
    magic_resist            INTEGER NOT NULL,
    movement_speed          INTEGER NOT NULL,
    omnivamp                INTEGER,
    physical_vamp           INTEGER,
    "power"                 INTEGER NOT NULL,
    power_max               INTEGER NOT NULL,
    power_regen             INTEGER NOT NULL,
    spell_vamp              INTEGER NOT NULL,
    PRIMARY KEY (frame_id, participant_id),
    FOREIGN KEY (frame_id, participant_id) REFERENCES match_v5.timeline_participant_frames (frame_id, participant_id)
);

COMMENT ON TABLE match_v5.champion_stats IS 'Original model name - match-v5.ChampionStatsDto.';
COMMENT ON COLUMN match_v5.champion_stats.ability_haste IS 'Original param name - abilityHaste.';
COMMENT ON COLUMN match_v5.champion_stats.ability_power IS 'Original param name - abilityPower.';
COMMENT ON COLUMN match_v5.champion_stats.armor_pen IS 'Original param name - armorPen.';
COMMENT ON COLUMN match_v5.champion_stats.armor_pen_percent IS 'Original param name - armorPenPercent.';
COMMENT ON COLUMN match_v5.champion_stats.attack_damage IS 'Original param name - attackDamage.';
COMMENT ON COLUMN match_v5.champion_stats.attack_speed IS 'Original param name - attackSpeed.';
COMMENT ON COLUMN match_v5.champion_stats.bonus_armor_pen_percent IS 'Original param name - bonusArmorPenPercent.';
COMMENT ON COLUMN match_v5.champion_stats.bonus_magic_pen_percent IS 'Original param name - bonusMagicPenPercent.';
COMMENT ON COLUMN match_v5.champion_stats.cc_reduction IS 'Original param name - ccReduction.';
COMMENT ON COLUMN match_v5.champion_stats.cooldown_reduction IS 'Original param name - cooldownReduction.';
COMMENT ON COLUMN match_v5.champion_stats.health_max IS 'Original param name - healthMax.';
COMMENT ON COLUMN match_v5.champion_stats.health_regen IS 'Original param name - healthRegen.';
COMMENT ON COLUMN match_v5.champion_stats.magic_pen IS 'Original param name - magicPen.';
COMMENT ON COLUMN match_v5.champion_stats.magic_pen_percent IS 'Original param name - magicPenPercent.';
COMMENT ON COLUMN match_v5.champion_stats.magic_resist IS 'Original param name - magicResist.';
COMMENT ON COLUMN match_v5.champion_stats.movement_speed IS 'Original param name - movementSpeed.';
COMMENT ON COLUMN match_v5.champion_stats.physical_vamp IS 'Original param name - physicalVamp.';
COMMENT ON COLUMN match_v5.champion_stats.power_max IS 'Original param name - powerMax.';
COMMENT ON COLUMN match_v5.champion_stats.power_regen IS 'Original param name - powerRegen.';
COMMENT ON COLUMN match_v5.champion_stats.spell_vamp IS 'Original param name - spellVamp.';

CREATE TABLE match_v5.damage_stats
(
    frame_id                          UUID    NOT NULL,
    participant_id                    INTEGER NOT NULL,

    magic_damage_done                 INTEGER NOT NULL,
    magic_damage_done_to_champions    INTEGER NOT NULL,
    magic_damage_taken                INTEGER NOT NULL,
    physical_damage_done              INTEGER NOT NULL,
    physical_damage_done_to_champions INTEGER NOT NULL,
    physical_damage_taken             INTEGER NOT NULL,
    total_damage_done                 INTEGER NOT NULL,
    total_damage_done_to_champions    INTEGER NOT NULL,
    total_damage_taken                INTEGER NOT NULL,
    true_damage_done                  INTEGER NOT NULL,
    true_damage_done_to_champions     INTEGER NOT NULL,
    true_damage_taken                 INTEGER NOT NULL,
    PRIMARY KEY (frame_id, participant_id),
    FOREIGN KEY (frame_id, participant_id) REFERENCES match_v5.timeline_participant_frames (frame_id, participant_id)
);
COMMENT ON TABLE match_v5.damage_stats IS 'Original model name - match-v5.DamageStatsDto.';
COMMENT ON COLUMN match_v5.damage_stats.magic_damage_done IS 'Original param name - magicDamageDone.';
COMMENT ON COLUMN match_v5.damage_stats.magic_damage_done_to_champions IS 'Original param name - magicDamageDoneToChampions.';
COMMENT ON COLUMN match_v5.damage_stats.magic_damage_taken IS 'Original param name - magicDamageTaken.';
COMMENT ON COLUMN match_v5.damage_stats.physical_damage_done IS 'Original param name - physicalDamageDone.';
COMMENT ON COLUMN match_v5.damage_stats.physical_damage_done_to_champions IS 'Original param name - physicalDamageDoneToChampions.';
COMMENT ON COLUMN match_v5.damage_stats.physical_damage_taken IS 'Original param name - physicalDamageTaken.';
COMMENT ON COLUMN match_v5.damage_stats.total_damage_done IS 'Original param name - totalDamageDone.';
COMMENT ON COLUMN match_v5.damage_stats.total_damage_done_to_champions IS 'Original param name - totalDamageDoneToChampions.';
COMMENT ON COLUMN match_v5.damage_stats.total_damage_taken IS 'Original param name - totalDamageTaken.';
COMMENT ON COLUMN match_v5.damage_stats.true_damage_done IS 'Original param name - trueDamageDone.';
COMMENT ON COLUMN match_v5.damage_stats.true_damage_done_to_champions IS 'Original param name - trueDamageDoneToChampions.';
COMMENT ON COLUMN match_v5.damage_stats.true_damage_taken IS 'Original param name - trueDamageTaken.';
