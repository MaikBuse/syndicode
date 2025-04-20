CREATE EXTENSION IF NOT EXISTS citext;

-- Users table
CREATE TABLE IF NOT EXISTS users (
    uuid UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    email CITEXT NOT NULL UNIQUE,
    role SMALLINT NOT NULL DEFAULT 2,
    status TEXT NOT NULL DEFAULT 'pending'
);

-- User table for verification codes
CREATE TABLE user_verifications (
    user_uuid UUID PRIMARY KEY, -- Ensures one verification attempt per user at a time
    code TEXT NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    -- Link back to the users table and ensure cleanup if a user is deleted
    CONSTRAINT fk_user
        FOREIGN KEY(user_uuid)
        REFERENCES users(uuid)
        ON DELETE CASCADE
);

-- Current Game Tick table (Singleton - Tracks the latest published state)
-- Stores the single source of truth for the current game tick available for reads.
CREATE TABLE IF NOT EXISTS current_game_tick (
    -- Using a fixed key ensures only one row can exist, enforcing the singleton nature.
    singleton_key BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK (singleton_key = TRUE),
    current_game_tick BIGINT NOT NULL DEFAULT 0
);

-- Initialize the current_game_tick table with a starting value (if it's empty)
INSERT INTO current_game_tick (singleton_key, current_game_tick)
VALUES (TRUE, 0)
ON CONFLICT (singleton_key) DO NOTHING;


-- Corporations table
CREATE TABLE IF NOT EXISTS corporations (
    game_tick BIGINT NOT NULL,
    uuid UUID NOT NULL,
    user_uuid UUID NOT NULL,
    name TEXT NOT NULL,
    balance BIGINT NOT NULL,

    UNIQUE (game_tick, user_uuid),
    PRIMARY KEY (game_tick, uuid),
    FOREIGN KEY (user_uuid) REFERENCES users(uuid) ON DELETE CASCADE
);

-- Index to potentially quickly find all corporations owned by a user at a specific tick
CREATE INDEX IF NOT EXISTS idx_corporations_game_tick_user ON corporations (game_tick, user_uuid);
-- Index to potentially find the history of a specific corporation (PK might cover this)
CREATE INDEX IF NOT EXISTS idx_corporations_uuid ON corporations (uuid);

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
