# Build stage
FROM rust:1.75 as builder
WORKDIR /app
COPY . .

# Install trunk and wasm target
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

# Build the app
RUN trunk build --release

# Runtime stage
FROM pierrezemb/gostatic
COPY --from=builder /app/dist /srv/http/
CMD ["-port","8080","-https-promote", "-enable-logging"]
