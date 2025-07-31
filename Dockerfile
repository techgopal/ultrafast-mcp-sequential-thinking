# ---- Build Stage ----
FROM rust:1.88.0 AS builder
WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy binary file to build dependencies
RUN mkdir -p src/bin && \
    echo "fn main() {}" > src/bin/server.rs && \
    cargo build --release --bin sequential-thinking-server && \
    rm -rf src

# Copy the actual source code
COPY src ./src

# Build the application for the native platform
RUN cargo build --release --bin sequential-thinking-server

# ---- Runtime Stage ----
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/sequential-thinking-server /usr/local/bin/sequential-thinking-server

# Make the binary executable
RUN chmod +x /usr/local/bin/sequential-thinking-server

# Expose the port
EXPOSE 8080

# Set the default command
CMD ["/usr/local/bin/sequential-thinking-server", "--transport", "http", "--port", "8080"] 