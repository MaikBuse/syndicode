-- Units table
CREATE TABLE IF NOT EXISTS units (
    game_tick BIGINT NOT NULL,
    uuid UUID NOT NULL,
    user_uuid UUID NOT NULL,

    PRIMARY KEY (game_tick, uuid),
    FOREIGN KEY (user_uuid) REFERENCES users(uuid) ON DELETE CASCADE
);

-- Index to potentially quickly find all units owned by a user at a specific tick
CREATE INDEX IF NOT EXISTS idx_units_game_tick_user ON units (game_tick, user_uuid);
-- Index to potentially find the history of a specific unit (PK might cover this)
CREATE INDEX IF NOT EXISTS idx_units_uuid ON units (uuid);
