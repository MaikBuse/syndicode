-- Users table
CREATE TABLE IF NOT EXISTS users (
  uuid UUID PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  password_hash TEXT NOT NULL,
  role SMALLINT NOT NULL DEFAULT 2
);

-- Corporations table
CREATE TABLE IF NOT EXISTS corporations (
  uuid UUID PRIMARY KEY,
  user_uuid UUID NOT NULL,
  name TEXT NOT NULL,
  balance BIGINT NOT NULL,
  FOREIGN KEY (user_uuid) REFERENCES users(uuid) ON DELETE CASCADE
);

-- Units table
CREATE TABLE IF NOT EXISTS units (
  uuid UUID PRIMARY KEY,
  user_uuid UUID NOT NULL,
  FOREIGN KEY (user_uuid) REFERENCES users(uuid) ON DELETE CASCADE
);

