CREATE TABLE IF NOT EXISTS league_v4.league_item
(
    puuid         TEXT PRIMARY KEY,

    fresh_blood   BOOLEAN NOT NULL,
    wins          INTEGER NOT NULL,
    inactive      BOOLEAN NOT NULL,
    veteran       BOOLEAN NOT NULL,
    hot_streak    BOOLEAN NOT NULL,
    "rank"        TEXT    NOT NULL,
    league_points INTEGER NOT NULL,
    losses        INTEGER NOT NULL,
    summoner_id   TEXT    NOT NULL,
    FOREIGN KEY (puuid) REFERENCES account_v1.accounts (puuid)
);
COMMENT ON TABLE league_v4.league_item IS 'Original model name - league-v4.LeagueItemDTO.';
COMMENT ON COLUMN league_v4.league_item.fresh_blood IS 'Original param name - freshBlood.';
COMMENT ON COLUMN league_v4.league_item.wins IS 'Winning team on Summoners Rift.';
COMMENT ON COLUMN league_v4.league_item.hot_streak IS 'Original param name - hotStreak.';
COMMENT ON COLUMN league_v4.league_item.league_points IS 'Original param name - leaguePoints.';
COMMENT ON COLUMN league_v4.league_item.losses IS 'Losing team on Summoners Rift.';
COMMENT ON COLUMN league_v4.league_item.summoner_id IS 'Player&#39;s encrypted summonerId.. Original param name - summonerId.';
COMMENT ON COLUMN league_v4.league_item.puuid IS 'Player&#39;s encrypted puuid.';

CREATE TABLE IF NOT EXISTS league_v4.mini_series
(
    puuid    TEXT PRIMARY KEY,

    losses   INTEGER NOT NULL,
    progress TEXT    NOT NULL,
    "target" INTEGER NOT NULL,
    wins     INTEGER NOT NULL,
    FOREIGN KEY (puuid) REFERENCES league_v4.league_item (puuid)
);
COMMENT ON TABLE league_v4.mini_series IS 'Original model name - league-v4.MiniSeriesDTO.';


