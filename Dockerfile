# Build stage
FROM rust:1.92-slim-trixie AS builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Caching dependencies
#RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

# Copy source code
COPY src ./src

# Copy migrations
COPY migrations ./migrations

RUN cargo build --release

# Runtime stage
FROM debian:trixie-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/rust-backend-template /app/rust-backend-template

# Expose the port
EXPOSE 3000

# Run the binary
CMD ["/app/rust-backend-template"]
