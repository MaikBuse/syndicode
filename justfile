# Load environment variables
set dotenv-required := true
set dotenv-load := true

# Group: Server
mod server

# Group: Database
mod db

# Default: Show all available recipes from grouped justfiles
help:
  @echo "[server]"
  @just --list server --list-heading ""
  @echo "[db]"
  @just --list db --list-heading ""
