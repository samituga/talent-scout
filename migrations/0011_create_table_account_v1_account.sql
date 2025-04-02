CREATE TABLE IF NOT EXISTS account_v1.accounts
(
    puuid     TEXT PRIMARY KEY,
    game_name TEXT,
    tag_line  TEXT,
    region    TEXT NOT NULL
);

COMMENT ON TABLE account_v1.accounts IS 'Original model name - account-v1.AccountDto.';
COMMENT ON COLUMN account_v1.accounts.game_name IS 'This field may be excluded from the response if the account doesn&#39;t have a gameName.. Original param name - gameName.';
COMMENT ON COLUMN account_v1.accounts.tag_line IS 'This field may be excluded from the response if the account doesn&#39;t have a tagLine.. Original param name - tagLine.';