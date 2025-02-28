FROM rust:1.75-bullseye as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    libclang-dev \
    libssl-dev \
    pkg-config \
    librocksdb-dev \
    && rm -rf /var/lib/apt/lists/*

# Set up working directory
WORKDIR /app

# Copy the entire project
COPY . .

# Build the API server
RUN cargo build --release --bin r3e-api

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    librocksdb-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set up working directory
WORKDIR /app

# Copy the built binary
COPY --from=builder /app/target/release/r3e-api /app/r3e-api

# Copy configuration files
COPY ./config /app/config

# Expose API port
EXPOSE 8080

# Set up environment
ENV RUST_BACKTRACE=1
ENV R3E_FAAS__GENERAL__ENVIRONMENT=production

# Run the API server
CMD ["/app/r3e-api", "--config", "/app/config/api.yaml"]
