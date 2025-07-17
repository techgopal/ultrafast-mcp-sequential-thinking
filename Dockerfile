# ---- Build Stage ----
FROM --platform=$BUILDPLATFORM rust:1.88.0 AS builder
WORKDIR /app

# Install cross-compilation tools for multi-platform builds
RUN apt-get update && apt-get install -y \
    gcc-aarch64-linux-gnu \
    g++-aarch64-linux-gnu \
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

# Build the application - use a script to handle the build logic
RUN echo '#!/bin/bash' > /tmp/build.sh && \
    echo 'set -e' >> /tmp/build.sh && \
    echo 'if [ "$TARGETARCH" = "amd64" ]; then' >> /tmp/build.sh && \
    echo '    echo "Building for amd64..."' >> /tmp/build.sh && \
    echo '    cargo build --release --bin sequential-thinking-server' >> /tmp/build.sh && \
    echo 'elif [ "$TARGETARCH" = "arm64" ]; then' >> /tmp/build.sh && \
    echo '    echo "Building for arm64..."' >> /tmp/build.sh && \
    echo '    CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++ cargo build --release --bin sequential-thinking-server --target aarch64-unknown-linux-gnu' >> /tmp/build.sh && \
    echo 'elif [ "$TARGETARCH" = "arm" ]; then' >> /tmp/build.sh && \
    echo '    echo "Building for arm..."' >> /tmp/build.sh && \
    echo '    cargo build --release --bin sequential-thinking-server --target armv7-unknown-linux-gnueabihf' >> /tmp/build.sh && \
    echo 'else' >> /tmp/build.sh && \
    echo '    echo "Unsupported architecture: $TARGETARCH"' >> /tmp/build.sh && \
    echo '    exit 1' >> /tmp/build.sh && \
    echo 'fi' >> /tmp/build.sh && \
    chmod +x /tmp/build.sh && \
    /tmp/build.sh

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