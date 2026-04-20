FROM rust:1.95-slim-bookworm
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
WORKDIR /app
COPY . .
RUN trunk build --release
