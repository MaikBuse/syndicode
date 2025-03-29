# -------- Builder Stage --------
FROM rust:1.85.1 as builder

WORKDIR /workspace

# Copy the full workspace
COPY . .

# Build only the server crate
RUN cargo build --release -p server

# -------- Runtime Stage --------
FROM debian:bullseye-slim

# Optional: Create non-root user
RUN adduser --disabled-password --gecos "" appuser

# Copy the built binary
COPY --from=builder /workspace/target/release/server /usr/local/bin/server

RUN chown appuser:appuser /usr/local/bin/server

USER appuser
EXPOSE 50051
CMD ["server"]
