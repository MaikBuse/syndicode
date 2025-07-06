help:
  @echo "[client]"
  @just --list client --list-heading ""

# Build the client crate
[group('cargo')]
build:
  cargo build -p syndicode-client

# Run clippy
[group('cargo')]
clippy:
  cargo clippy -p syndicode-client --all-targets --all-features -- -D warnings

# Apply clippy fixes
[group('cargo')]
clippy-fix:
  cargo clippy --fix --lib -p syndicode-client

# Test the client crate
[group('cargo')]
test:
  cargo test -p syndicode-client -- --test-threads=1

# Run the client crate
[group('cargo')]
run:
  cargo run -p syndicode-client

# Check the formatting
[group('cargo')]
fmt:
  cargo fmt --manifest-path syndicode-client/Cargo.toml
