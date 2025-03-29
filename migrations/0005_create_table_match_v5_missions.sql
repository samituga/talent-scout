CREATE TABLE match_v5.missions
(
    match_id       TEXT    NOT NULL,
    participant_id INTEGER NOT NULL,

    player_score0  DECIMAL(20, 9),
    player_score1  DECIMAL(20, 9),
    player_score2  DECIMAL(20, 9),
    player_score3  DECIMAL(20, 9),
    player_score4  DECIMAL(20, 9),
    player_score5  DECIMAL(20, 9),
    player_score6  DECIMAL(20, 9),
    player_score7  DECIMAL(20, 9),
    player_score8  DECIMAL(20, 9),
    player_score9  DECIMAL(20, 9),
    player_score10 DECIMAL(20, 9),
    player_score11 DECIMAL(20, 9),
    PRIMARY KEY (match_id, participant_id),
    FOREIGN KEY (match_id, participant_id)
        REFERENCES match_v5.participants (match_id, participant_id)
);


COMMENT ON TABLE match_v5.missions IS 'Missions DTO. Original model name - match-v5.MissionsDto.';

COMMENT ON COLUMN match_v5.missions.player_score0 IS 'Original param name - playerScore0.';
COMMENT ON COLUMN match_v5.missions.player_score1 IS 'Original param name - playerScore1.';
COMMENT ON COLUMN match_v5.missions.player_score2 IS 'Original param name - playerScore2.';
COMMENT ON COLUMN match_v5.missions.player_score3 IS 'Original param name - playerScore3.';
COMMENT ON COLUMN match_v5.missions.player_score4 IS 'Original param name - playerScore4.';
COMMENT ON COLUMN match_v5.missions.player_score5 IS 'Original param name - playerScore5.';
COMMENT ON COLUMN match_v5.missions.player_score6 IS 'Original param name - playerScore6.';
COMMENT ON COLUMN match_v5.missions.player_score7 IS 'Original param name - playerScore7.';
COMMENT ON COLUMN match_v5.missions.player_score8 IS 'Original param name - playerScore8.';
COMMENT ON COLUMN match_v5.missions.player_score9 IS 'Original param name - playerScore9.';
COMMENT ON COLUMN match_v5.missions.player_score10 IS 'Original param name - playerScore10.';
COMMENT ON COLUMN match_v5.missions.player_score11 IS 'Original param name - playerScore11.';