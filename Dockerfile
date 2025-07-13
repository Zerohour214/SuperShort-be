# Use the official Rust image to build the app
FROM rust:1.88 AS builder

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy source and build
COPY . .
RUN cargo build --release

# Use a minimal base image for running
FROM debian:bookworm-slim

WORKDIR /app

# Install needed system dependencies (e.g., for SQLite)
RUN apt-get update && apt-get install -y libsqlite3-0 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/supershort /app/app

EXPOSE 3000

CMD ["./app"]