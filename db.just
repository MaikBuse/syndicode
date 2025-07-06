help:
  @echo "[db]"
  @just --list db --list-heading ""

# Add a new database migration
[group('migration')]
add name:
  sqlx migrate add -r --source ./syndicode-server/migrations {{name}}

# Run database migrations
[group('migration')]
migrate:
  sqlx migrate run --source ./syndicode-server/migrations --database-url "postgres://${SERVER_POSTGRES_USER}:${SERVER_POSTGRES_PASSWORD}@${SERVER_POSTGRES_HOST}:${SERVER_POSTGRES_PORT}/${SERVER_POSTGRES_DB}"

# Create the database and run migrations
[group('sqlx')]
setup:
  sqlx db create --database-url "postgres://${SERVER_POSTGRES_USER}:${SERVER_POSTGRES_PASSWORD}@${SERVER_POSTGRES_HOST}:${SERVER_POSTGRES_PORT}/${SERVER_POSTGRES_DB}"
  sqlx migrate run --source ./syndicode-server/migrations --database-url "postgres://${SERVER_POSTGRES_USER}:${SERVER_POSTGRES_PASSWORD}@${SERVER_POSTGRES_HOST}:${SERVER_POSTGRES_PORT}/${SERVER_POSTGRES_DB}"
  cargo sqlx prepare --workspace

# Revert database migrations
[group('migration')]
revert:
  sqlx migrate revert --source ./syndicode-server/migrations --database-url "postgres://${SERVER_POSTGRES_USER}:${SERVER_POSTGRES_PASSWORD}@${SERVER_POSTGRES_HOST}:${SERVER_POSTGRES_PORT}/${SERVER_POSTGRES_DB}"

# Create query data in order to use SQLx offline
[group('sqlx')]
prepare:
  cargo sqlx prepare --workspace --database-url "postgres://${SERVER_POSTGRES_USER}:${SERVER_POSTGRES_PASSWORD}@${SERVER_POSTGRES_HOST}:${SERVER_POSTGRES_PORT}/${SERVER_POSTGRES_DB}"

# Show logs
[group('utils')]
logs:
  if command -v docker-compose > /dev/null; then \
    docker-compose logs -f; \
  else \
    docker compose logs -f; \
  fi

# PSQL Shell
[group('utils')]
psql:
  psql -h localhost -U $SERVER_POSTGRES_USER -d $SERVER_POSTGRES_DB

# Print the postgres sql connection string. Can be used with `eval "$(just db conn)"` to set the 'DATABASE_URL' environment variable
[group('utils')]
conn:
  echo export DATABASE_URL=postgres://$SERVER_POSTGRES_USER:$SERVER_POSTGRES_PASSWORD@$SERVER_POSTGRES_HOST:$SERVER_POSTGRES_PORT/$SERVER_POSTGRES_DB

# Dump the database using pg_dump
[group('utils')]
dump:
  PGPASSWORD=$SERVER_POSTGRES_PASSWORD pg_dump \
    --format=custom \
    --file=syndicode_server_backup.dump \
    --host=${SERVER_POSTGRES_HOST} \
    --port=${SERVER_POSTGRES_PORT} \
    --username=${SERVER_POSTGRES_USER} \
    --clean \
    --if-exists \
    --create \
    --no-owner \
    --no-acl \
    --verbose \
    syndicode
