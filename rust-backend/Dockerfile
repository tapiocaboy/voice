# Build stage
FROM rust:1.70 as builder

WORKDIR /usr/src/app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release

# Remove the dummy source
RUN rm -rf src

# Copy actual source
COPY src ./src

# Build actual application
RUN touch src/main.rs && cargo build --release

# Production stage
FROM debian:bullseye-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /usr/src/app/target/release/voice-analytics-backend ./

# Create directory for audio files
RUN mkdir -p audio_data

# Set environment variables
ENV RUST_LOG=info

# Expose port
EXPOSE 8000

# Start application
CMD ["./voice-analytics-backend"] 