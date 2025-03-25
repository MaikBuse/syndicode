PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS users (
  uuid BLOB PRIMARY KEY NOT NULL,
  name TEXT NOT NULL UNIQUE,
  password_hash TEXT NOT NULL,
  role TEXT NOT NULL CHECK (role IN ('Admin', 'Player'))
);

CREATE TABLE IF NOT EXISTS sessions (
  uuid BLOB PRIMARY KEY NOT NULL,
  interval INTEGER NOT NULL DEFAULT 0,
  state TEXT NOT NULL DEFAULT 'Idle' CHECK (state IN ('Idle', 'Initializing', 'Running'))
);

CREATE TABLE IF NOT EXISTS session_users (
  uuid BLOB PRIMARY KEY NOT NULL,
  session_uuid BLOB NOT NULL,
  user_uuid BLOB NOT NULL,
  FOREIGN KEY (session_uuid) REFERENCES sessions(uuid) ON DELETE CASCADE,
  FOREIGN KEY (user_uuid) REFERENCES users(uuid) ON DELETE CASCADE
  UNIQUE (session_uuid, user_uuid)
);

CREATE TABLE IF NOT EXISTS corporations (
  uuid BLOB PRIMARY KEY NOT NULL,
  session_uuid BLOB NOT NULL,
  user_uuid BLOB NOT NULL,
  name TEXT NOT NULL,
  balance INTEGER NOT NULL,
  FOREIGN KEY (session_uuid) REFERENCES sessions(uuid) ON DELETE CASCADE,
  FOREIGN KEY (user_uuid) REFERENCES users(uuid) ON DELETE CASCADE
  UNIQUE (session_uuid, user_uuid)
);

CREATE TABLE IF NOT EXISTS units (
  uuid BLOB PRIMARY KEY NOT NULL,
  session_uuid BLOB NOT NULL,
  user_uuid BLOB NOT NULL,
  FOREIGN KEY (session_uuid) REFERENCES sessions(uuid) ON DELETE CASCADE,
  FOREIGN KEY (user_uuid) REFERENCES users(uuid) ON DELETE CASCADE
);
