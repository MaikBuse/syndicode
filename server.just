# Build the server crate
[group('cargo')]
build:
  cargo build -p syndicode-server

# Run the server crate
[group('cargo')]
run:
  cargo run -p syndicode-server

# Build the image
[group('docker')]
image:
  docker build -f syndicode-server/Dockerfile -t syndicode-server .
