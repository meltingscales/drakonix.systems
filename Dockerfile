# Build stage
FROM rust:1.83-slim as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY src ./src
COPY templates ./templates
COPY static ./static

# Build the actual application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/rust-blog /app/rust-blog

# Copy static assets and templates
COPY templates /app/templates
COPY static /app/static

# Create content directory (will be populated at runtime)
RUN mkdir -p /app/content/posts /app/content/pages

# Copy example content if exists
COPY content /app/content 2>/dev/null || true

# Set environment variables
ENV RUST_LOG=info
ENV PORT=8080

# Expose port
EXPOSE 8080

# Run the application
CMD ["/app/rust-blog"]
