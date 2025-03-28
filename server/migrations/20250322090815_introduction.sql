-- Users table
CREATE TABLE IF NOT EXISTS users (
  uuid UUID PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  password_hash TEXT NOT NULL,
  role SMALLINT NOT NULL DEFAULT 2
);

-- Sessions table
CREATE TABLE IF NOT EXISTS sessions (
  uuid UUID PRIMARY KEY,
  interval BIGINT NOT NULL DEFAULT 0,
  state SMALLINT NOT NULL DEFAULT 1
);

-- Session ↔ Users mapping table
CREATE TABLE IF NOT EXISTS session_users (
  uuid UUID PRIMARY KEY,
  session_uuid UUID NOT NULL,
  user_uuid UUID NOT NULL,
  UNIQUE (session_uuid, user_uuid),
  FOREIGN KEY (session_uuid) REFERENCES sessions(uuid) ON DELETE CASCADE,
  FOREIGN KEY (user_uuid) REFERENCES users(uuid) ON DELETE CASCADE
);

-- Corporations table
CREATE TABLE IF NOT EXISTS corporations (
  uuid UUID PRIMARY KEY,
  session_uuid UUID NOT NULL,
  user_uuid UUID NOT NULL,
  name TEXT NOT NULL,
  balance BIGINT NOT NULL,
  UNIQUE (session_uuid, user_uuid),
  FOREIGN KEY (session_uuid) REFERENCES sessions(uuid) ON DELETE CASCADE,
  FOREIGN KEY (user_uuid) REFERENCES users(uuid) ON DELETE CASCADE
);

-- Units table
CREATE TABLE IF NOT EXISTS units (
  uuid UUID PRIMARY KEY,
  session_uuid UUID NOT NULL,
  user_uuid UUID NOT NULL,
  FOREIGN KEY (session_uuid) REFERENCES sessions(uuid) ON DELETE CASCADE,
  FOREIGN KEY (user_uuid) REFERENCES users(uuid) ON DELETE CASCADE
);

