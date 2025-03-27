setup:
  sqlx db create
  sqlx migrate run --source ./server/migrations

drop:
  sqlx db drop
