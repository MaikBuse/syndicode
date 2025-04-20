help:
  @echo "[docker]"
  @just --list docker --list-heading ""

# Start the database
[group('docker')]
start:
  if command -v docker-compose > /dev/null; then \
    docker-compose up -d; \
  else \
    docker compose up -d; \
  fi

# Stop the database
[group('docker')]
stop:
  if command -v docker-compose > /dev/null; then \
    docker-compose down; \
  else \
    docker compose down; \
  fi

# Restart the database
[group('docker')]
restart:
  just stop
  just start

