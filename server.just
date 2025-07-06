help:
  @echo "[server]"
  @just --list server --list-heading ""

# Build the server crate
[group('cargo')]
build:
  cargo build -p syndicode-server

# Run the server crate
[group('cargo')]
run:
  cargo run -p syndicode-server

# Run the server crate and restore the database
[group('cargo')]
restore:
  cargo run -p syndicode-server -- --restore $SERVER_DB_DUMP_URL

# Run clippy
[group('cargo')]
clippy:
  cargo clippy -p syndicode-server --all-targets --all-features -- -D warnings

# Apply clippy fixes
[group('cargo')]
clippy-fix:
  cargo clippy --fix --lib -p syndicode-server

# Test the server crate
[group('cargo')]
test:
  cargo test -p syndicode-server

# Check the formatting
[group('cargo')]
fmt:
  cargo fmt --manifest-path syndicode-server/Cargo.toml


# Build the image
[group('docker')]
image:
  docker build -f syndicode-server/Dockerfile -t syndicode-server .
