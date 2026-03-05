# Stage 1: Build
FROM rust:1.87-slim AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Cache dependencies
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# Build actual source
COPY src ./src
COPY benches ./benches
RUN cargo build --release

# Stage 2: Runtime (minimal image)
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/compression-platform ./server
EXPOSE 3001
CMD ["./server"]
