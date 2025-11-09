# Build stage
FROM rust:1.75 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs

# Copy source code
COPY src ./src
COPY migrations ./migrations
COPY migrations-sqlite ./migrations-sqlite

# Build application
RUN touch src/main.rs
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    sqlite3 \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1000 app

WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/sultan /app/sultan
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/migrations-sqlite /app/migrations-sqlite

# Change ownership
RUN chown -R app:app /app
USER app

EXPOSE 3001

CMD ["./sultan"]
