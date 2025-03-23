build-server:
  cargo build -p server

run-server:
  cargo run -p server

setup-db:
  sqlx db create
  sqlx migrate run --source ./server/migrations

drop-db:
  sqlx db drop
