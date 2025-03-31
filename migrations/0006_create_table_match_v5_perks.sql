CREATE TABLE match_v5.participant_perks
(
    match_id TEXT    NOT NULL,
    puuid    TEXT    NOT NULL,

    -- perk_stats_dto
    defense  INTEGER NOT NULL,
    flex     INTEGER NOT NULL,
    offense  INTEGER NOT NULL,
    PRIMARY KEY (match_id, puuid),
    FOREIGN KEY (match_id, puuid)
        REFERENCES match_v5.participants (match_id, puuid)
);

COMMENT ON TABLE match_v5.participant_perks IS 'Original model name - match-v5.PerksDto.';

CREATE TABLE match_v5.perk_styles
(
    perk_style_id SERIAL PRIMARY KEY, -- TODO decide if serial or uuid and generated in the application
    match_id      TEXT    NOT NULL,
    puuid         TEXT    NOT NULL,

    description   TEXT    NOT NULL,
    "style"       INTEGER NOT NULL,
    FOREIGN KEY (match_id, puuid)
        REFERENCES match_v5.participant_perks (match_id, puuid)
);

COMMENT ON TABLE match_v5.perk_styles IS 'Original model name - match-v5.PerkStyleDto.';

CREATE TABLE match_v5.perk_style_selections
(
    perk_style_selection_id SERIAL PRIMARY KEY,
    perk_style_id           INTEGER NOT NULL,

    perk                    INTEGER NOT NULL,
    var1                    INTEGER NOT NULL,
    var2                    INTEGER NOT NULL,
    var3                    INTEGER NOT NULL,
    FOREIGN KEY (perk_style_id)
        REFERENCES match_v5.perk_styles (perk_style_id)
);

COMMENT ON TABLE match_v5.perk_style_selections IS 'Original model name - match-v5.PerkStyleSelectionDto.';
