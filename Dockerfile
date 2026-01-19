# Build stage - use Debian 12 to match distroless runtime GLIBC version
FROM rust:1-bookworm AS builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy build to cache dependencies
# This creates stub files for all modules so cargo can compile dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn placeholder() {}" > src/handlers.rs && \
    echo "pub fn placeholder() {}" > src/markdown.rs && \
    echo "pub fn placeholder() {}" > src/models.rs && \
    echo "pub fn placeholder() {}" > src/rss.rs && \
    cargo build --release && \
    rm -rf src target/release/rust-blog* target/release/deps/rust_blog*

# Copy source code
COPY src ./src

# Copy static-macro files needed for compile-time inclusion (include_str! macros)
# Changes to these files will invalidate the build cache
COPY static-macro ./static-macro

# Build the actual application
# Touch source files to ensure they're rebuilt even if deps are cached
RUN touch src/main.rs && cargo build --release

# Runtime stage - using Google's distroless image for minimal attack surface
FROM gcr.io/distroless/cc-debian12

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/rust-blog /app/rust-blog

# Copy static assets and templates (changed frequently, so copy last)
COPY templates /app/templates
COPY static /app/static
COPY content /app/content

# Set environment variables
ENV RUST_LOG=info
ENV PORT=8080
ENV BASE_URL=https://drakonix.systems

# Expose port
EXPOSE 8080

# Run the application
CMD ["/app/rust-blog"]
