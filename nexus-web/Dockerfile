# Build stage
FROM rust:1.81 as builder

# Install trunk (do this early for caching)
RUN cargo install trunk --version 0.21.14 --locked

WORKDIR /app

# Copy all source files
COPY . .

# Install wasm target and build the app
RUN rustup target add wasm32-unknown-unknown && \
    trunk build --release

# Runtime stage
FROM pierrezemb/gostatic
COPY --from=builder /app/dist /srv/http/
CMD ["-port","8080","-https-promote", "-enable-logging"] 