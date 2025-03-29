CREATE TABLE match_v5.teams
(
    match_id TEXT    NOT NULL,

--     bans       JSON    NOT NULL,
--     objectives TEXT    NOT NULL,
    team_id  INTEGER NOT NULL, -- TODO team_id might already be unique and I wouldn't need a composite primary key
    win      BOOLEAN NOT NULL,
--     feats    TEXT,
    PRIMARY KEY (match_id, team_id),
    FOREIGN KEY (match_id) REFERENCES match_v5.matches (match_id)
);
COMMENT ON TABLE match_v5.teams IS 'Original model name - match-v5.TeamDto.';
COMMENT ON COLUMN match_v5.teams.team_id IS 'Original param name - teamId.';


CREATE TABLE match_v5.bans
(
    ban_id      SERIAL  NOT NULL,
    match_id    TEXT    NOT NULL,
    team_id     INTEGER NOT NULL,

    champion_id INTEGER NOT NULL,
    pick_turn   INTEGER NOT NULL,
    PRIMARY KEY (ban_id, match_id, team_id),
    FOREIGN KEY (match_id, team_id) REFERENCES match_v5.teams (match_id, team_id)
);
COMMENT ON TABLE match_v5.bans IS 'Original model name - match-v5.BanDto.';
COMMENT ON COLUMN match_v5.bans.champion_id IS 'Original param name - championId.';
COMMENT ON COLUMN match_v5.bans.pick_turn IS 'Original param name - pickTurn.';

CREATE TABLE match_v5.objectives
(
    match_id          TEXT    NOT NULL,
    team_id           INTEGER NOT NULL,

    baron_first       BOOLEAN NOT NULL,
    baron_kills       INTEGER NOT NULL,
    champion_first    BOOLEAN NOT NULL,
    champion_kills    INTEGER NOT NULL,
    dragon_first      BOOLEAN NOT NULL,
    dragon_kills      INTEGER NOT NULL,
    horde_first       BOOLEAN,
    horde_kills       INTEGER,
    inhibitor_first   BOOLEAN NOT NULL,
    inhibitor_kills   INTEGER NOT NULL,
    rift_herald_first BOOLEAN NOT NULL,
    rift_herald_kills INTEGER NOT NULL,
    tower_first       BOOLEAN NOT NULL,
    tower_kills       INTEGER NOT NULL,
    atakhan_first     BOOLEAN,
    atakhan_kills     INTEGER,

    PRIMARY KEY (match_id, team_id),
    FOREIGN KEY (match_id, team_id) REFERENCES match_v5.teams (match_id, team_id)
);

COMMENT ON TABLE match_v5.objectives IS 'Original model name - match-v5.ObjectivesDto.';


CREATE TABLE match_v5.feats
(
    match_id                TEXT    NOT NULL,
    team_id                 INTEGER NOT NULL,

    epic_monster_kill_state INTEGER,
    first_blood_state       INTEGER,
    first_turret_state      INTEGER,
    PRIMARY KEY (match_id, team_id),
    FOREIGN KEY (match_id, team_id) REFERENCES match_v5.teams (match_id, team_id)
);

COMMENT ON TABLE match_v5.feats IS 'Original model name - match-v5.FeatsDto.';
COMMENT ON COLUMN match_v5.feats.epic_monster_kill_state IS 'Original param name - EPIC_MONSTER_KILL.';
COMMENT ON COLUMN match_v5.feats.first_blood_state IS 'Original param name - FIRST_BLOOD.';
COMMENT ON COLUMN match_v5.feats.first_turret_state IS 'Original param name - FIRST_TURRET.';