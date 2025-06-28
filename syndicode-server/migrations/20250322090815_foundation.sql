CREATE EXTENSION IF NOT EXISTS citext;
CREATE EXTENSION IF NOT EXISTS postgis;

-- Create the system_flags table
CREATE TABLE IF NOT EXISTS system_flags (
    flag_key VARCHAR(100) PRIMARY KEY,
    is_set BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Insert the specific flag rows we'll check, ensuring it exists
INSERT INTO system_flags (flag_key, is_set)
VALUES ('database_initialized', FALSE)
ON CONFLICT (flag_key) DO NOTHING;

INSERT INTO system_flags (flag_key, is_set)
VALUES ('admin_domain_initialized', FALSE)
ON CONFLICT (flag_key) DO NOTHING;

INSERT INTO system_flags (flag_key, is_set)
VALUES ('economy_domain_initialized', FALSE)
ON CONFLICT (flag_key) DO NOTHING;


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

