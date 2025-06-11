# Load environment variables
set dotenv-required := true
set dotenv-load := true
set export := true

mod proto
mod grpcurl
mod server
mod client
mod db
mod docker

# Default: Show all available recipes from grouped justfiles
help:
  @echo "[proto]"
  @just --list proto --list-heading ""
  @echo "[grpcurl]"
  @just --list grpcurl --list-heading ""
  @echo "[client]"
  @just --list client --list-heading ""
  @echo "[server]"
  @just --list server --list-heading ""
  @echo "[docker]"
  @just --list docker --list-heading ""
  @echo "[db]"
  @just --list db --list-heading ""

# Format relevant workspace members
[group('cargo')]
fmt:
  cargo fmt --package syndicode-server --package syndicode-client

# Run clippy linting on the workspace
[group('cargo')]
clippy:
  cargo clippy --all-targets --all-features -- -D warnings

# Apply the clippy suggested fixes
[group('cargo')]
clippy-fix:
  cargo clippy --fix
