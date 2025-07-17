# ---- Build Stage ----
FROM --platform=$BUILDPLATFORM rust:1.88.0 AS builder
WORKDIR /app

# Install cross-compilation tools and OpenSSL development libraries
RUN apt-get update && apt-get install -y \
    gcc-aarch64-linux-gnu \
    g++-aarch64-linux-gnu \
    gcc-arm-linux-gnueabihf \
    g++-arm-linux-gnueabihf \
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

# Build for the target platform
ARG TARGETPLATFORM
ARG BUILDPLATFORM
ARG TARGETOS
ARG TARGETARCH

# Set up cross-compilation targets based on architecture
RUN if [ "$TARGETARCH" = "arm64" ]; then \
        rustup target add aarch64-unknown-linux-gnu; \
    elif [ "$TARGETARCH" = "arm" ]; then \
        rustup target add armv7-unknown-linux-gnueabihf; \
    fi

# Build the application with proper OpenSSL configuration
RUN if [ "$TARGETARCH" = "amd64" ]; then \
        echo "Building for amd64..."; \
        cargo build --release --bin sequential-thinking-server; \
    elif [ "$TARGETARCH" = "arm64" ]; then \
        echo "Building for arm64..."; \
        export CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc; \
        export CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++; \
        export PKG_CONFIG_ALLOW_CROSS=1; \
        export OPENSSL_STATIC=1; \
        cargo build --release --bin sequential-thinking-server --target aarch64-unknown-linux-gnu; \
    elif [ "$TARGETARCH" = "arm" ]; then \
        echo "Building for arm..."; \
        export CC_armv7_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc; \
        export CXX_armv7_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++; \
        export PKG_CONFIG_ALLOW_CROSS=1; \
        export OPENSSL_STATIC=1; \
        cargo build --release --bin sequential-thinking-server --target armv7-unknown-linux-gnueabihf; \
    fi

# Copy the binary to a standard location
RUN if [ "$TARGETARCH" = "amd64" ]; then \
        cp /app/target/release/sequential-thinking-server /app/sequential-thinking-server; \
    elif [ "$TARGETARCH" = "arm64" ]; then \
        cp /app/target/aarch64-unknown-linux-gnu/release/sequential-thinking-server /app/sequential-thinking-server; \
    elif [ "$TARGETARCH" = "arm" ]; then \
        cp /app/target/armv7-unknown-linux-gnueabihf/release/sequential-thinking-server /app/sequential-thinking-server; \
    fi

# ---- Runtime Stage ----
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/sequential-thinking-server /usr/local/bin/sequential-thinking-server

# Make the binary executable
RUN chmod +x /usr/local/bin/sequential-thinking-server

# Expose the port
EXPOSE 8080

# Set the default command
CMD ["/usr/local/bin/sequential-thinking-server", "--transport", "http", "--port", "8080"] 