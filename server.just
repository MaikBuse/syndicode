# Build the server crate
[group('cargo')]
build:
  cargo build -p server

# Run the server crate
[group('cargo')]
run:
  cargo run -p server

# Run integration tests
[group('cargo')]
e2e:
  cargo test -p server --test e2e


# Build the image
[group('docker')]
image:
  docker build -f server/Dockerfile -t syndicode-server .
