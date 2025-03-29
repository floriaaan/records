# -------------------------------
# 1. Build Stage (Rust Compilation)
# -------------------------------
FROM rust:1-bookworm AS builder

WORKDIR /app

# Install dependencies for compilation
RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential

# Copy dependencies first to leverage Docker cache
COPY Cargo.toml Cargo.lock ./
COPY src src
COPY sqlx-data.json sqlx-data.json
COPY templates templates

# Fetch dependencies
RUN cargo fetch

ENV SQLX_OFFLINE=true
RUN cargo build --release

RUN cargo install sqlx-cli --no-default-features --features postgres

# -------------------------------
# 2. Runtime Stage (Minimal Final Image using debian:bookworm-slim)
# -------------------------------
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Install necessary runtime dependencies (static netcat)
# RUN apk add --no-cache netcat-openbsd libssl
RUN apt-get update && apt-get install -y libssl-dev ca-certificates netcat-openbsd curl && rm -rf /var/lib/apt/lists/*

# Install rust, cargo, and sqlx-cli
COPY --from=builder /usr/local/cargo/bin/sqlx /app/sqlx


# Copy compiled Rust binary from builder stage
COPY --from=builder /app/target/release/records /app/records


# Copy entrypoint script & set permissions
COPY docker-entrypoint.sh /app/docker-entrypoint.sh
RUN chmod +x /app/docker-entrypoint.sh

# Copy migrations
COPY migrations /app/migrations
COPY Rocket.toml /app/Rocket.toml

# Expose API port
EXPOSE 8000

# Use entrypoint script
ENTRYPOINT ["/app/docker-entrypoint.sh"]
# -------------------------------
