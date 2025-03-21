CREATE TABLE match_v5.missions
(
    match_id       TEXT    NOT NULL,
    participant_id INTEGER NOT NULL,
    player_score0  REAL,
    player_score1  REAL,
    player_score2  REAL,
    player_score3  REAL,
    player_score4  REAL,
    player_score5  REAL,
    player_score6  REAL,
    player_score7  REAL,
    player_score8  REAL,
    player_score9  REAL,
    player_score10 REAL,
    player_score11 REAL,
    PRIMARY KEY (match_id, participant_id),
    FOREIGN KEY (match_id, participant_id)
        REFERENCES match_v5.participant (match_id, participant_id)
);
