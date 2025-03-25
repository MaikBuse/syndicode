server-build:
  cargo build -p server

server-run:
  cargo run -p server

db-setup:
  sqlx db create
  sqlx migrate run --source ./server/migrations

db-drop:
  sqlx db drop
