-- Corporations table (Represents player-controlled entities in the game)
-- Stores the state of each corporation at each tick.
CREATE TABLE IF NOT EXISTS corporations (
    game_tick BIGINT NOT NULL,
    uuid UUID NOT NULL,
    user_uuid UUID NOT NULL,
    name TEXT NOT NULL,
    cash_balance BIGINT NOT NULL CHECK (cash_balance >= 0),

    PRIMARY KEY (game_tick, uuid),

    UNIQUE (game_tick, name)
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
    center POINT NOT NULL,

    PRIMARY KEY (game_tick, uuid)
);

-- Index for finding business state quickly by its persistent ID and tick
CREATE INDEX IF NOT EXISTS idx_businesses_uuid_game_tick ON businesses (uuid, game_tick);
-- Index for finding all businesses within a specific market at a specific tick
CREATE INDEX IF NOT EXISTS idx_businesses_market_uuid_game_tick ON businesses (market_uuid, game_tick);
-- Index for finding all businesses owned by a specific corporation at a specific tick
CREATE INDEX IF NOT EXISTS idx_businesses_owner_uuid_game_tick ON businesses (owning_corporation_uuid, game_tick);

-- Business Listing Table
CREATE TABLE IF NOT EXISTS business_listings (
    game_tick BIGINT NOT NULL,
    uuid UUID NOT NULL,
    business_uuid UUID NOT NULL,
    seller_corporation_uuid UUID,
    asking_price BIGINT NOT NULL CHECK (asking_price > 0),

    PRIMARY KEY (game_tick, uuid)
);

CREATE INDEX IF NOT EXISTS idx_business_uuid_game_tick ON business_listings (business_uuid, game_tick);
CREATE INDEX IF NOT EXISTS idx_seller_corporation_uuid_game_tick ON business_listings (seller_corporation_uuid, game_tick);

-- Business Offers Table
CREATE TABLE IF NOT EXISTS business_offers (
    game_tick BIGINT NOT NULL,
    uuid UUID NOT NULL,
    business_uuid UUID NOT NULL,
    offering_corporation_uuid UUID NOT NULL,
    target_corporation_uuid UUID,
    offer_price BIGINT NOT NULL CHECK (offer_price > 0),

    PRIMARY KEY (game_tick, uuid)
);

CREATE INDEX IF NOT EXISTS idx_business_uuid_game_tick ON business_offers (business_uuid, game_tick);
CREATE INDEX IF NOT EXISTS idx_offering_corporation_uuid ON business_offers (offering_corporation_uuid, game_tick);
CREATE INDEX IF NOT EXISTS idx_target_corporation_uuid ON business_offers (target_corporation_uuid, game_tick);


CREATE TABLE buildings (
    game_tick BIGINT NOT NULL,
    uuid UUID NOT NULL,
    gml_id TEXT NOT NULL,
    name TEXT NOT NULL,
    owning_business_uuid UUID,
    address TEXT NOT NULL,
    usage TEXT NOT NULL,
    usage_code SMALLINT NOT NULL,
    class TEXT NOT NULL,
    class_code SMALLINT NOT NULL,
    city TEXT NOT NULL,
    city_code TEXT NOT NULL,
    center POINT NOT NULL,
    footprint POLYGON NOT NULL,
    height DOUBLE PRECISION NOT NULL,
    prefecture TEXT NOT NULL,

    PRIMARY KEY (game_tick, uuid)
);

-- Create a standard index on gml_id.
-- This allows for very fast lookups if you need to find a specific building by its source ID.
CREATE INDEX idx_buildings_gml_id_idx ON buildings (gml_id);

CREATE INDEX IF NOT EXISTS idx_building_uuid_game_tick ON buildings (uuid, game_tick);
CREATE INDEX IF NOT EXISTS idx_owning_business_uuid_game_tick ON buildings (owning_business_uuid, game_tick);
