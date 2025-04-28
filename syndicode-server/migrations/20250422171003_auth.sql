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
