# ---- Build Stage ----
FROM --platform=$BUILDPLATFORM rust:1.88.0 as builder
WORKDIR /app

# Install cross-compilation tools for multi-platform builds
RUN apt-get update && apt-get install -y \
    gcc-aarch64-linux-gnu \
    g++-aarch64-linux-gnu \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release --bin sequential-thinking-server && \
    rm -rf src

# Copy the actual source code
COPY src ./src

# Build for the target platform
ARG TARGETPLATFORM
ARG BUILDPLATFORM
ARG TARGETOS
ARG TARGETARCH

# Set up cross-compilation targets
RUN case "$TARGETARCH" in \
        amd64) \
            echo "Building for x86_64-unknown-linux-gnu" \
            ;; \
        arm64) \
            rustup target add aarch64-unknown-linux-gnu \
            echo "Building for aarch64-unknown-linux-gnu" \
            ;; \
        arm) \
            rustup target add armv7-unknown-linux-gnueabihf \
            echo "Building for armv7-unknown-linux-gnueabihf" \
            ;; \
        *) \
            echo "Unsupported architecture: $TARGETARCH" \
            exit 1 \
            ;; \
    esac

# Build the application
RUN case "$TARGETARCH" in \
        amd64) \
            cargo build --release --bin sequential-thinking-server \
            ;; \
        arm64) \
            CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
            CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++ \
            cargo build --release --bin sequential-thinking-server --target aarch64-unknown-linux-gnu \
            ;; \
        arm) \
            cargo build --release --bin sequential-thinking-server --target armv7-unknown-linux-gnueabihf \
            ;; \
    esac

# Determine the correct binary path based on target architecture
RUN case "$TARGETARCH" in \
        amd64) \
            cp /app/target/release/sequential-thinking-server /app/sequential-thinking-server \
            ;; \
        arm64) \
            cp /app/target/aarch64-unknown-linux-gnu/release/sequential-thinking-server /app/sequential-thinking-server \
            ;; \
        arm) \
            cp /app/target/armv7-unknown-linux-gnueabihf/release/sequential-thinking-server /app/sequential-thinking-server \
            ;; \
    esac

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