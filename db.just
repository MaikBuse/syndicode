# Add a new database migration
[group('migration')]
add name:
  sqlx migrate add -r --source ./syndicode-server/migrations {{name}}

# Run database migrations
[group('migration')]
migrate:
  sqlx migrate run --source ./syndicode-server/migrations --database-url "postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}"

# Create the database and run migrations
[group('sqlx')]
setup:
  sqlx db create --database-url "postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}"
  sqlx migrate run --source ./syndicode-server/migrations --database-url "postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}"
  cargo sqlx prepare --workspace

# Revert database migrations
[group('migration')]
revert:
  sqlx migrate revert --source ./syndicode-server/migrations --database-url "postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}"

# Create query data in order to use SQLx offline
[group('sqlx')]
prepare:
  cargo sqlx prepare --workspace --database-url "postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}"

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
  psql -h localhost -U $POSTGRES_USER -d $POSTGRES_DB

# Print the postgres sql connection string. Can be used with `eval "$(just db conn)"` to set the 'DATABASE_URL' environment variable
[group('utils')]
conn:
  echo export DATABASE_URL=postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@$POSTGRES_HOST:$POSTGRES_PORT/$POSTGRES_DB
