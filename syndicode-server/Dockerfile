# -------- Builder Stage --------
FROM rust:1.85.1 as builder

# Install protoc and pkg-config dependencies
RUN apt-get update && apt-get install -y protobuf-compiler pkg-config libssl-dev

WORKDIR /workspace

# Copy the full workspace
COPY . .

# Build only the server crate
RUN cargo build --release -p syndicode-server

# -------- Runtime Stage --------
FROM debian:bookworm-slim

# Install OpenSSL 3 runtime library
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Optional: Create non-root user
RUN adduser --disabled-password --gecos "" appuser

# Copy the built binary
COPY --from=builder /workspace/target/release/syndicode-server /usr/local/bin/server

RUN chown appuser:appuser /usr/local/bin/syndicode-server

USER appuser
EXPOSE 50051
CMD ["syndicode-server"]

