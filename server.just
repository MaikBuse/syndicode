# Build the server crate
[group('cargo')]
build:
  cargo build -p server

# run the server crate
[group('cargo')]
run:
  cargo run -p server
