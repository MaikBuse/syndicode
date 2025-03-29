# Build the client crate
[group('cargo')]
build:
  cargo build -p syndicode-client

# Run the client crate
[group('cargo')]
run:
  cargo run -p syndicode-client
