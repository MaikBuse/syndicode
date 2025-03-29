# Load environment variables
set dotenv-required := true
set dotenv-load := true
set export := true

mod proto
mod server
mod client
mod db

# Default: Show all available recipes from grouped justfiles
help:
  @echo "[proto]"
  @just --list proto --list-heading ""
  @echo "[client]"
  @just --list client --list-heading ""
  @echo "[server]"
  @just --list server --list-heading ""
  @echo "[db]"
  @just --list db --list-heading ""
