# Build stage
FROM rust:1.95-slim-bookworm AS builder

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

RUN rm -rf src

# Copy actual source
COPY . .

# Touch the binary so Cargo rebuilds only the app, not all dependencies
RUN touch src/main.rs
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime deps (adjust if needed)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/backend /backend

RUN chmod +x /backend

EXPOSE 8080

CMD ["/backend"]