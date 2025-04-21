CREATE EXTENSION IF NOT EXISTS citext;

-- Create the system_flags table
CREATE TABLE IF NOT EXISTS system_flags (
    flag_key VARCHAR(100) PRIMARY KEY,
    is_set BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Insert the specific flag row we'll check, ensuring it exists
INSERT INTO system_flags (flag_key, is_set)
VALUES ('database_initialized', FALSE)
ON CONFLICT (flag_key) DO NOTHING;

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


-- Corporations table (Represents player-controlled entities in the game)
-- Stores the state of each corporation at each tick.
CREATE TABLE IF NOT EXISTS corporations (
    game_tick BIGINT NOT NULL,
    uuid UUID NOT NULL,
    user_uuid UUID NOT NULL,
    name TEXT NOT NULL,
    cash_balance BIGINT NOT NULL CHECK (cash_balance >= 0),

    PRIMARY KEY (game_tick, uuid),

    -- Ensures a user controls only one corporation state per tick (logically should be only one corp per user ever)
    UNIQUE (game_tick, user_uuid),

    CONSTRAINT fk_user FOREIGN KEY (user_uuid) REFERENCES users(uuid) ON DELETE CASCADE
);

-- Index for finding corporation state quickly by its persistent ID and tick
CREATE INDEX IF NOT EXISTS idx_corporations_uuid_game_tick ON corporations (uuid, game_tick);
-- Index for finding the corporation controlled by a user at a specific tick
CREATE INDEX IF NOT EXISTS idx_corporations_user_uuid_game_tick ON corporations (user_uuid, game_tick);


-- Markets table (Represents economic sectors, e.g., 'Gambling', 'Retail')
CREATE TABLE IF NOT EXISTS markets (
    game_tick BIGINT NOT NULL,
    uuid UUID NOT NULL,
    name SMALLINT NOT NULL,
    volume BIGINT NOT NULL CHECK (volume >= 0),

    PRIMARY KEY (game_tick, uuid)
);

-- Index for finding market state quickly by its persistent ID and tick
CREATE INDEX IF NOT EXISTS idx_markets_uuid_game_tick ON markets (uuid, game_tick);

-- Businesses table (Represents individual assets within markets)
CREATE TABLE IF NOT EXISTS businesses (
    game_tick BIGINT NOT NULL,
    uuid UUID NOT NULL,
    market_uuid UUID NOT NULL,
    owning_corporation_uuid UUID,
    name TEXT NOT NULL,
    operational_expenses BIGINT NOT NULL CHECK (operational_expenses >= 0),

    PRIMARY KEY (game_tick, uuid)
);

-- Index for finding business state quickly by its persistent ID and tick
CREATE INDEX IF NOT EXISTS idx_businesses_uuid_game_tick ON businesses (uuid, game_tick);
-- Index for finding all businesses within a specific market at a specific tick
CREATE INDEX IF NOT EXISTS idx_businesses_market_uuid_game_tick ON businesses (market_uuid, game_tick);
-- Index for finding all businesses owned by a specific corporation at a specific tick
CREATE INDEX IF NOT EXISTS idx_businesses_owner_uuid_game_tick ON businesses (owning_corporation_uuid, game_tick);

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
