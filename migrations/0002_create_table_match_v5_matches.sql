CREATE TABLE match_v5.matches
(
    -- match_v5.metadata_dto
    match_id             TEXT PRIMARY KEY,
    data_version         TEXT    NOT NULL,
    -- match_v5.info_dto
    end_of_game_result   TEXT,
    game_creation        BIGINT  NOT NULL,
    game_duration        BIGINT  NOT NULL,
    game_end_timestamp   BIGINT,
    game_id              BIGINT  NOT NULL,
    game_mode            TEXT    NOT NULL,
    game_name            TEXT    NOT NULL,
    game_start_timestamp BIGINT  NOT NULL,
    game_type            TEXT    NOT NULL,
    game_version         TEXT    NOT NULL,
    map_id               INTEGER NOT NULL,
--     participants         JSON    NOT NULL,
    platform_id          TEXT    NOT NULL,
    queue_id             INTEGER NOT NULL,
--     teams                JSON    NOT NULL,
    tournament_code      TEXT
);

COMMENT ON TABLE match_v5.matches IS 'Original model name - match-v5.MatchDto.';

COMMENT ON COLUMN match_v5.matches.match_id IS 'Match id.. Original param name - matchId.';
COMMENT ON COLUMN match_v5.matches.data_version IS 'Match data version.. Original param name - dataVersion.';

COMMENT ON COLUMN match_v5.matches.end_of_game_result IS 'Refer to indicate if the game ended in termination.. Original param name - endOfGameResult.';
COMMENT ON COLUMN match_v5.matches.game_creation IS 'Unix timestamp for when the game is created on the game server (i.e., the loading screen).. Original param name - gameCreation.';
COMMENT ON COLUMN match_v5.matches.game_duration IS 'Prior to patch 11.20, this field returns the game length in milliseconds calculated from gameEndTimestamp - gameStartTimestamp. Post patch 11.20, this field returns the max timePlayed of any participant in the game in seconds, which makes the behavior of this field consistent with that of match-v4. The best way to handling the change in this field is to treat the value as milliseconds if the gameEndTimestamp field isn&#39;t in the response and to treat the value as seconds if gameEndTimestamp is in the response.. Original param name - gameDuration.';
COMMENT ON COLUMN match_v5.matches.game_end_timestamp IS 'Unix timestamp for when match ends on the game server. This timestamp can occasionally be significantly longer than when the match \&quot;ends\&quot;. The most reliable way of determining the timestamp for the end of the match would be to add the max time played of any participant to the gameStartTimestamp. This field was added to match-v5 in patch 11.20 on Oct 5th, 2021.. Original param name - gameEndTimestamp.';
COMMENT ON COLUMN match_v5.matches.game_id IS 'Original param name - gameId.';
COMMENT ON COLUMN match_v5.matches.game_mode IS 'Refer to the Game Constants documentation.. Original param name - gameMode.';
COMMENT ON COLUMN match_v5.matches.game_name IS 'Original param name - gameName.';
COMMENT ON COLUMN match_v5.matches.game_start_timestamp IS 'Unix timestamp for when match starts on the game server.. Original param name - gameStartTimestamp.';
COMMENT ON COLUMN match_v5.matches.game_type IS 'Original param name - gameType.';
COMMENT ON COLUMN match_v5.matches.game_version IS 'The first two parts can be used to determine the patch a game was played on.. Original param name - gameVersion.';
COMMENT ON COLUMN match_v5.matches.map_id IS 'Refer to the Game Constants documentation.. Original param name - mapId.';
COMMENT ON COLUMN match_v5.matches.platform_id IS 'Platform where the match was played.. Original param name - platformId.';
COMMENT ON COLUMN match_v5.matches.queue_id IS 'Refer to the Game Constants documentation.. Original param name - queueId.';
COMMENT ON COLUMN match_v5.matches.tournament_code IS 'Tournament code used to generate the match. This field was added to match-v5 in patch 11.13 on June 23rd, 2021.. Original param name - tournamentCode.';
