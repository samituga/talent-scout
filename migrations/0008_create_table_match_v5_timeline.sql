CREATE TABLE match_v5.timelines
(
    match_id           TEXT PRIMARY KEY,

    data_version       TEXT   NOT NULL,
    end_of_game_result TEXT,
    frame_interval     BIGINT NOT NULL,
    game_id            BIGINT
);

COMMENT ON COLUMN match_v5.timelines.data_version IS 'Match data version.. Original param name - dataVersion.';
COMMENT ON COLUMN match_v5.timelines.match_id IS 'Match id.. Original param name - matchId.';
COMMENT ON COLUMN match_v5.timelines.end_of_game_result IS 'Refer to indicate if the game ended in termination.. Original param name - endOfGameResult.';
COMMENT ON COLUMN match_v5.timelines.frame_interval IS 'Original param name - frameInterval.';
COMMENT ON COLUMN match_v5.timelines.game_id IS 'Original param name - gameId.';

CREATE TABLE match_v5.timeline_participants
(
    puuid          TEXT    NOT NULL,
    match_id       TEXT    NOT NULL,

    participant_id INTEGER NOT NULL,
    PRIMARY KEY (match_id, puuid),
    FOREIGN KEY (match_id) REFERENCES match_v5.timelines (match_id)
);

COMMENT ON TABLE match_v5.timeline_participants IS 'Original model name - match-v5.ParticipantTimeLineDto.';
COMMENT ON COLUMN match_v5.timeline_participants.participant_id IS 'Original param name - participantId.';
