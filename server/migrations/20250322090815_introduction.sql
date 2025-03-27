-- Define ENUM types
CREATE TYPE user_role AS ENUM ('Admin', 'User');
CREATE TYPE session_state AS ENUM ('Initializing', 'Running');

-- Users table
CREATE TABLE IF NOT EXISTS users (
  uuid UUID PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  password_hash TEXT NOT NULL,
  role user_role NOT NULL DEFAULT 'User'
);

-- Sessions table
CREATE TABLE IF NOT EXISTS sessions (
  uuid UUID PRIMARY KEY,
  interval BIGINT NOT NULL DEFAULT 0,
  state session_state NOT NULL DEFAULT 'Initializing'
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

