# -------- Builder Stage --------
FROM rust:1.85.1 AS builder

# Install protoc and pkg-config dependencies
RUN apt-get update && apt-get install -y protobuf-compiler pkg-config libssl-dev

WORKDIR /workspace

# Copy the full workspace
COPY . .

# Build only the server crate
RUN cargo build --release -p syndicode-server

# -------- Runtime Stage --------
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
  # Prerequisites for adding the new repo
  curl \
  gnupg \
  # Original dependencies
  libssl3 \
  ca-certificates \
  && \
  # Add the PostgreSQL GPG key
  curl -fsSL https://www.postgresql.org/media/keys/ACCC4CF8.asc | gpg --dearmor -o /etc/apt/trusted.gpg.d/postgresql.gpg && \
  # Add the PostgreSQL repository
  echo "deb http://apt.postgresql.org/pub/repos/apt/ bookworm-pgdg main" > /etc/apt/sources.list.d/pgdg.list && \
  # Update apt lists again to pull from the new repo
  apt-get update && \
  # Install the specific PostgreSQL client version
  apt-get install -y --no-install-recommends postgresql-client-17 && \
  # Clean up temporary packages and apt cache to keep the image small
  apt-get purge -y --auto-remove curl gnupg && \
  rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN adduser --disabled-password --gecos "" appuser

# Copy the built binary
COPY --from=builder /workspace/target/release/syndicode-server /usr/local/bin/server

RUN chown appuser:appuser /usr/local/bin/server

USER appuser
EXPOSE 50051
CMD ["server"]
