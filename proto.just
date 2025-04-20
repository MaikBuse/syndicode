help:
  @echo "[proto]"
  @just --list proto --list-heading ""

# Build the proto library
[group('cargo')]
build:
  cargo build -p syndicode-proto
