# ---- Build Stage ----
FROM rust:1.76 as builder
WORKDIR /app
COPY . .
WORKDIR /app/ultrafast-mcp-sequential-thinking
RUN cargo build --release --bin sequential-thinking-server

# ---- Runtime Stage ----
FROM debian:bullseye-slim
WORKDIR /app
# Install minimal dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/ultrafast-mcp-sequential-thinking/target/release/sequential-thinking-server /usr/local/bin/sequential-thinking-server
EXPOSE 8080
CMD ["/usr/local/bin/sequential-thinking-server", "--transport", "http", "--port", "8080"] 