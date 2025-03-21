CREATE TABLE match_v5.match
(
    match_id             TEXT PRIMARY KEY,
    data_version         TEXT        NOT NULL,
    end_of_game_result   TEXT,
    game_creation        TIMESTAMPTZ NOT NULL,
    game_duration        BIGINT      NOT NULL,
    game_end_timestamp   TIMESTAMPTZ,
    game_id              BIGINT      NOT NULL,
    game_mode            TEXT        NOT NULL,
    game_name            TEXT        NOT NULL,
    game_start_timestamp TIMESTAMPTZ NOT NULL,
    game_type            TEXT        NOT NULL,
    game_version         TEXT        NOT NULL,
    map_id               INTEGER     NOT NULL,
    platform_id          TEXT        NOT NULL,
    queue_id             INTEGER     NOT NULL,
    tournament_code      TEXT
);

COMMENT ON COLUMN match_v5.match.end_of_game_result IS 'Refer to indicate if the game ended in termination.';
COMMENT ON COLUMN match_v5.match.game_creation IS 'Timestamp for when the game is created on the game server (i.e., the loading screen).';
COMMENT ON COLUMN match_v5.match.game_duration IS 'Game duration (milliseconds or seconds, depending on patch).';
COMMENT ON COLUMN match_v5.match.game_end_timestamp IS 'Timestamp for when the match ends on the game server. This can be significantly later than when the match “ends” (added in patch 11.20 on Oct 5th, 2021).';
COMMENT ON COLUMN match_v5.match.game_mode IS 'Refer to the Game Constants documentation.';
COMMENT ON COLUMN match_v5.match.game_start_timestamp IS 'Timestamp for when the match starts on the game server.';
COMMENT ON COLUMN match_v5.match.game_type IS 'Game type (see Game Constants for legal values).';
COMMENT ON COLUMN match_v5.match.game_version IS 'The first two parts can be used to determine the patch a game was played on.';
COMMENT ON COLUMN match_v5.match.map_id IS 'Refer to the Game Constants documentation for map id.';
COMMENT ON COLUMN match_v5.match.platform_id IS 'Platform where the match was played.';
COMMENT ON COLUMN match_v5.match.queue_id IS 'Refer to the Game Constants documentation for queue id.';
COMMENT ON COLUMN match_v5.match.tournament_code IS 'Tournament code used to generate the match; added in patch 11.13 on June 23rd, 2021.';