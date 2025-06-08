-- Units table
CREATE TABLE IF NOT EXISTS units (
    game_tick BIGINT NOT NULL,
    uuid UUID NOT NULL,
    corporation_uuid UUID NOT NULL,

    PRIMARY KEY (game_tick, uuid)
);

-- Index to potentially quickly find all units owned by a user at a specific tick
CREATE INDEX IF NOT EXISTS idx_units_game_tick_corporation ON units (game_tick, corporation_uuid);
-- Index to potentially find the history of a specific unit
CREATE INDEX IF NOT EXISTS idx_units_uuid ON units (uuid);
