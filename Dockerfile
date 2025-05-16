FROM rustlang/rust:nightly as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/axum-server /app/axum-server

# Install required dependencies for SSL and other runtime needs
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

CMD ["/app/axum-server", "--port", "8080"] 