# ---- Build Stage ----
FROM rust:1.88.0 as builder
WORKDIR /app
# Copy only the necessary files for building the crate
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY README.md ./
# If you have a build.rs or other files, add them here
# COPY build.rs ./
RUN cargo build --release --bin sequential-thinking-server
RUN ls -lh /app/target/release

# ---- Runtime Stage ----
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/sequential-thinking-server /usr/local/bin/sequential-thinking-server
EXPOSE 8080
CMD ["/usr/local/bin/sequential-thinking-server", "--transport", "http", "--port", "8080"] 