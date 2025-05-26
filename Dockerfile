FROM rustlang/rust:nightly as builder

WORKDIR /app
COPY . .

RUN cargo build --release --bin axum-server
RUN cargo build --release --bin setup_db

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/axum-server /app/axum-server
COPY --from=builder /app/target/release/setup_db /app/setup_db
COPY --from=builder /app/migrations /app/migrations

# Install required dependencies for SSL and other runtime needs
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

CMD ["/app/axum-server", "--port", "8080"] 