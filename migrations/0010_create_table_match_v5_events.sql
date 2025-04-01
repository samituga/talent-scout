CREATE TABLE IF NOT EXISTS match_v5.events_timeline
(
    event_timeline_id         UUID PRIMARY KEY,
    match_id                  TEXT      NOT NULL,
    "timestamp"               BIGINT    NOT NULL,
    real_timestamp            BIGINT,
    "type"                    TEXT      NOT NULL,
    item_id                   INTEGER,
    participant_id            INTEGER,
    level_up_type             TEXT,
    skill_slot                INTEGER,
    creator_id                INTEGER,
    ward_type                 TEXT,
    "level"                   INTEGER,
    assisting_participant_ids integer[] NULL,
    bounty                    INTEGER,
    kill_streak_length        INTEGER,
    killer_id                 INTEGER,
    position_x                INTEGER,
    position_y                INTEGER,
    victim_id                 INTEGER,
    kill_type                 TEXT,
    lane_type                 TEXT,
    team_id                   INTEGER,
    multi_kill_length         INTEGER,
    killer_team_id            INTEGER,
    monster_type              TEXT,
    monster_sub_type          TEXT,
    building_type             TEXT,
    tower_type                TEXT,
    after_id                  INTEGER,
    before_id                 INTEGER,
    gold_gain                 INTEGER,
    game_id                   BIGINT,
    winning_team              INTEGER,
    transform_type            TEXT,
    "name"                    TEXT,
    shutdown_bounty           INTEGER,
    actual_start_time         BIGINT,
    feat_type                 INTEGER,
    feat_value                INTEGER,
    FOREIGN KEY (match_id) REFERENCES match_v5.timelines (match_id)
);
COMMENT ON TABLE match_v5.events_timeline IS 'Original model name - match-v5.EventsTimeLineDto.';
COMMENT ON COLUMN match_v5.events_timeline.real_timestamp IS 'Original param name - realTimestamp.';
COMMENT ON COLUMN match_v5.events_timeline.item_id IS 'Original param name - itemId.';
COMMENT ON COLUMN match_v5.events_timeline.participant_id IS 'Original param name - participantId.';
COMMENT ON COLUMN match_v5.events_timeline.level_up_type IS 'Original param name - levelUpType.';
COMMENT ON COLUMN match_v5.events_timeline.skill_slot IS 'Original param name - skillSlot.';
COMMENT ON COLUMN match_v5.events_timeline.creator_id IS 'Original param name - creatorId.';
COMMENT ON COLUMN match_v5.events_timeline.ward_type IS 'Original param name - wardType.';
COMMENT ON COLUMN match_v5.events_timeline.assisting_participant_ids IS 'Original param name - assistingParticipantIds.';
COMMENT ON COLUMN match_v5.events_timeline.kill_streak_length IS 'Original param name - killStreakLength.';
COMMENT ON COLUMN match_v5.events_timeline.killer_id IS 'Original param name - killerId.';
COMMENT ON COLUMN match_v5.events_timeline.victim_id IS 'Original param name - victimId.';
COMMENT ON COLUMN match_v5.events_timeline.kill_type IS 'Original param name - killType.';
COMMENT ON COLUMN match_v5.events_timeline.lane_type IS 'Original param name - laneType.';
COMMENT ON COLUMN match_v5.events_timeline.team_id IS 'Original param name - teamId.';
COMMENT ON COLUMN match_v5.events_timeline.multi_kill_length IS 'Original param name - multiKillLength.';
COMMENT ON COLUMN match_v5.events_timeline.killer_team_id IS 'Original param name - killerTeamId.';
COMMENT ON COLUMN match_v5.events_timeline.monster_type IS 'Original param name - monsterType.';
COMMENT ON COLUMN match_v5.events_timeline.monster_sub_type IS 'Original param name - monsterSubType.';
COMMENT ON COLUMN match_v5.events_timeline.building_type IS 'Original param name - buildingType.';
COMMENT ON COLUMN match_v5.events_timeline.tower_type IS 'Original param name - towerType.';
COMMENT ON COLUMN match_v5.events_timeline.after_id IS 'Original param name - afterId.';
COMMENT ON COLUMN match_v5.events_timeline.before_id IS 'Original param name - beforeId.';
COMMENT ON COLUMN match_v5.events_timeline.gold_gain IS 'Original param name - goldGain.';
COMMENT ON COLUMN match_v5.events_timeline.game_id IS 'Original param name - gameId.';
COMMENT ON COLUMN match_v5.events_timeline.winning_team IS 'Original param name - winningTeam.';
COMMENT ON COLUMN match_v5.events_timeline.transform_type IS 'Original param name - transformType.';
COMMENT ON COLUMN match_v5.events_timeline.shutdown_bounty IS 'Original param name - shutdownBounty.';
COMMENT ON COLUMN match_v5.events_timeline.actual_start_time IS 'Original param name - actualStartTime.';
COMMENT ON COLUMN match_v5.events_timeline.feat_type IS 'Original param name - featType.';
COMMENT ON COLUMN match_v5.events_timeline.feat_value IS 'Original param name - featValue.';


CREATE TYPE match_v5.damage_direction AS ENUM ('dealt', 'received');

CREATE TABLE match_v5.match_timeline_victim_damage
(
    id                UUID PRIMARY KEY,
    event_timeline_id UUID                      NOT NULL,
    basic             BOOLEAN                   NOT NULL,
    magic_damage      INTEGER                   NOT NULL,
    "name"            TEXT                      NOT NULL,
    participant_id    INTEGER                   NOT NULL,
    physical_damage   INTEGER                   NOT NULL,
    spell_name        TEXT                      NOT NULL,
    spell_slot        INTEGER                   NOT NULL,
    true_damage       INTEGER                   NOT NULL,
    "type"            TEXT                      NOT NULL,
    damage_direction  match_v5.damage_direction NOT NULL,
    FOREIGN KEY (event_timeline_id) REFERENCES match_v5.events_timeline (event_timeline_id)
);

COMMENT ON TABLE match_v5.match_timeline_victim_damage IS 'Original model name - match-v5.MatchTimelineVictimDamage.';
COMMENT ON COLUMN match_v5.match_timeline_victim_damage.magic_damage IS 'Original param name - magicDamage.';
COMMENT ON COLUMN match_v5.match_timeline_victim_damage.participant_id IS 'Original param name - participantId.';
COMMENT ON COLUMN match_v5.match_timeline_victim_damage.physical_damage IS 'Original param name - physicalDamage.';
COMMENT ON COLUMN match_v5.match_timeline_victim_damage.spell_name IS 'Original param name - spellName.';
COMMENT ON COLUMN match_v5.match_timeline_victim_damage.spell_slot IS 'Original param name - spellSlot.';
COMMENT ON COLUMN match_v5.match_timeline_victim_damage.true_damage IS 'Original param name - trueDamage.';
