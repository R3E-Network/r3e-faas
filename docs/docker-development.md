# Docker Development Guide for R3E FaaS

This guide provides detailed instructions for setting up and using Docker for development with the R3E FaaS platform.

## Prerequisites

Before using Docker for development, ensure you have the following prerequisites:

- **Docker**: Latest version
- **Docker Compose**: Latest version
- **Git**: Latest version

## Development Environment Setup

### 1. Clone the Repository

```bash
git clone https://github.com/R3E-Network/r3e-faas.git
cd r3e-faas
```

### 2. Development Docker Compose Configuration

The repository includes a `docker-compose.dev.yml` file specifically configured for development:

```yaml
version: '3.8'

services:
  api:
    build:
      context: .
      dockerfile: docker/api/Dockerfile
      target: development
    ports:
      - "8080:8080"
    volumes:
      - .:/app
      - r3e-data:/data
    environment:
      - R3E_FAAS__GENERAL__ENVIRONMENT=development
      - R3E_FAAS__GENERAL__LOG_LEVEL=debug
      - R3E_FAAS__API__PORT=8080
      - R3E_FAAS__API__HOST=0.0.0.0
      - R3E_FAAS__STORAGE__TYPE=rocksdb
      - R3E_FAAS__STORAGE__PATH=/data/rocksdb
    depends_on:
      - worker

  worker:
    build:
      context: .
      dockerfile: docker/worker/Dockerfile
      target: development
    volumes:
      - .:/app
      - r3e-data:/data
    environment:
      - R3E_FAAS__GENERAL__ENVIRONMENT=development
      - R3E_FAAS__GENERAL__LOG_LEVEL=debug
      - R3E_FAAS__STORAGE__TYPE=rocksdb
      - R3E_FAAS__STORAGE__PATH=/data/rocksdb
    deploy:
      replicas: 2

volumes:
  r3e-data:
```

### 3. Start the Development Environment

```bash
# Start the development environment
docker-compose -f docker-compose.dev.yml up
```

This command starts the API server and worker services in development mode with hot reloading.

### 4. Accessing the Development Environment

- **API Server**: http://localhost:8080
- **API Documentation**: http://localhost:8080/api/v1/docs
- **Health Check**: http://localhost:8080/api/v1/health

## Development Workflow

### Code Changes

When you make changes to the code, the development environment automatically reloads:

1. Edit the source code
2. Save the changes
3. The services automatically rebuild and restart

### Running Tests in Docker

```bash
# Run all tests
docker-compose -f docker-compose.dev.yml run --rm api cargo test

# Run tests for a specific crate
docker-compose -f docker-compose.dev.yml run --rm api cargo test -p r3e-neo-services

# Run a specific test
docker-compose -f docker-compose.dev.yml run --rm api cargo test -p r3e-neo-services gas_bank_test
```

### Building Documentation in Docker

```bash
# Generate documentation
docker-compose -f docker-compose.dev.yml run --rm api cargo doc --no-deps

# The documentation is available in the target/doc directory
```

## Docker Development Features

### Hot Reloading

The development environment is configured with hot reloading, which automatically rebuilds and restarts the services when you make changes to the code.

### Volume Mounting

The development environment mounts the current directory as a volume, allowing you to edit the code on your host machine and see the changes immediately in the Docker containers.

### Debugging

You can attach a debugger to the Docker containers for debugging:

```bash
# Attach to the API server container
docker attach r3e-faas_api_1

# Attach to a worker container
docker attach r3e-faas_worker_1
```

### Environment Variables

You can customize the environment variables in the `docker-compose.dev.yml` file or create a `.env` file:

```bash
# Create a .env file
cat > .env << EOL
R3E_FAAS__GENERAL__ENVIRONMENT=development
R3E_FAAS__GENERAL__LOG_LEVEL=debug
R3E_FAAS__API__PORT=8080
R3E_FAAS__API__HOST=0.0.0.0
R3E_FAAS__STORAGE__TYPE=rocksdb
R3E_FAAS__STORAGE__PATH=/data/rocksdb
EOL
```

## Docker Development Best Practices

### Resource Management

You can limit the resources used by the Docker containers:

```bash
# Limit CPU and memory
docker-compose -f docker-compose.dev.yml up -d --scale worker=2 --memory=2g --cpus=2
```

### Container Management

```bash
# List running containers
docker-compose -f docker-compose.dev.yml ps

# Stop the development environment
docker-compose -f docker-compose.dev.yml down

# Stop and remove volumes
docker-compose -f docker-compose.dev.yml down -v
```

### Rebuilding Images

If you make changes to the Dockerfiles or dependencies, you need to rebuild the images:

```bash
# Rebuild images
docker-compose -f docker-compose.dev.yml build

# Rebuild and start
docker-compose -f docker-compose.dev.yml up --build
```

## Development Dockerfile

The development Dockerfile (`Dockerfile.dev`) is configured for development:

```dockerfile
FROM rust:1.60 as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && \
    apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    clang \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./
COPY r3e-core/Cargo.toml r3e-core/
COPY r3e-config/Cargo.toml r3e-config/
COPY r3e-store/Cargo.toml r3e-store/
COPY r3e-event/Cargo.toml r3e-event/
COPY r3e-deno/Cargo.toml r3e-deno/
COPY r3e-worker/Cargo.toml r3e-worker/
COPY r3e-api/Cargo.toml r3e-api/
COPY r3e-neo-services/Cargo.toml r3e-neo-services/
COPY r3e-oracle/Cargo.toml r3e-oracle/
COPY r3e-tee/Cargo.toml r3e-tee/
COPY r3e-secrets/Cargo.toml r3e-secrets/
COPY r3e-zk/Cargo.toml r3e-zk/
COPY r3e-fhe/Cargo.toml r3e-fhe/
COPY r3e-built-in-services/Cargo.toml r3e-built-in-services/
COPY r3e/Cargo.toml r3e/

# Create dummy source files
RUN mkdir -p r3e-core/src && echo "fn main() {}" > r3e-core/src/lib.rs
RUN mkdir -p r3e-config/src && echo "fn main() {}" > r3e-config/src/lib.rs
RUN mkdir -p r3e-store/src && echo "fn main() {}" > r3e-store/src/lib.rs
RUN mkdir -p r3e-event/src && echo "fn main() {}" > r3e-event/src/lib.rs
RUN mkdir -p r3e-deno/src && echo "fn main() {}" > r3e-deno/src/lib.rs
RUN mkdir -p r3e-worker/src && echo "fn main() {}" > r3e-worker/src/lib.rs
RUN mkdir -p r3e-api/src && echo "fn main() {}" > r3e-api/src/lib.rs
RUN mkdir -p r3e-neo-services/src && echo "fn main() {}" > r3e-neo-services/src/lib.rs
RUN mkdir -p r3e-oracle/src && echo "fn main() {}" > r3e-oracle/src/lib.rs
RUN mkdir -p r3e-tee/src && echo "fn main() {}" > r3e-tee/src/lib.rs
RUN mkdir -p r3e-secrets/src && echo "fn main() {}" > r3e-secrets/src/lib.rs
RUN mkdir -p r3e-zk/src && echo "fn main() {}" > r3e-zk/src/lib.rs
RUN mkdir -p r3e-fhe/src && echo "fn main() {}" > r3e-fhe/src/lib.rs
RUN mkdir -p r3e-built-in-services/src && echo "fn main() {}" > r3e-built-in-services/src/lib.rs
RUN mkdir -p r3e/src && echo "fn main() {}" > r3e/src/main.rs

# Build dependencies
RUN cargo build

# Development stage
FROM builder as development

# Install development tools
RUN rustup component add rustfmt clippy

# Install cargo-watch for hot reloading
RUN cargo install cargo-watch

# Set working directory
WORKDIR /app

# Copy source code
COPY . .

# Start with hot reloading
CMD ["cargo", "watch", "-x", "run --bin r3e-api"]
```

## Docker Compose Services

### API Server

The API server service is configured in `docker/api/Dockerfile`:

```dockerfile
FROM rust:1.60 as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && \
    apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    clang \
    && rm -rf /var/lib/apt/lists/*

# Copy source code
COPY . .

# Build the API server
RUN cargo build --release --bin r3e-api

# Development stage
FROM builder as development

# Install development tools
RUN rustup component add rustfmt clippy
RUN cargo install cargo-watch

# Start with hot reloading
CMD ["cargo", "watch", "-x", "run --bin r3e-api"]

# Production stage
FROM debian:bullseye-slim as production

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y \
    libssl1.1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary
COPY --from=builder /app/target/release/r3e-api /app/r3e-api

# Expose the API port
EXPOSE 8080

# Start the API server
CMD ["/app/r3e-api"]
```

### Worker

The worker service is configured in `docker/worker/Dockerfile`:

```dockerfile
FROM rust:1.60 as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && \
    apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    clang \
    && rm -rf /var/lib/apt/lists/*

# Copy source code
COPY . .

# Build the worker
RUN cargo build --release --bin r3e-worker

# Development stage
FROM builder as development

# Install development tools
RUN rustup component add rustfmt clippy
RUN cargo install cargo-watch

# Start with hot reloading
CMD ["cargo", "watch", "-x", "run --bin r3e-worker"]

# Production stage
FROM debian:bullseye-slim as production

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y \
    libssl1.1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary
COPY --from=builder /app/target/release/r3e-worker /app/r3e-worker

# Start the worker
CMD ["/app/r3e-worker"]
```

## Troubleshooting

### Common Issues

#### Permission Issues

If you encounter permission issues with Docker volumes:

```bash
# Change the ownership of the volume
docker-compose -f docker-compose.dev.yml run --rm --user root api chown -R 1000:1000 /data
```

#### Port Conflicts

If port 8080 is already in use:

```bash
# Change the port mapping in docker-compose.dev.yml
ports:
  - "8081:8080"
```

#### Memory Issues

If you encounter memory issues:

```bash
# Increase the memory limit
docker-compose -f docker-compose.dev.yml up -d --memory=4g
```

### Getting Help

If you encounter issues not covered in this guide:

- Check the Docker logs: `docker-compose -f docker-compose.dev.yml logs`
- Open an issue on GitHub
- Join the community chat

## Next Steps

After setting up the Docker development environment, you can:

- Follow the [Quick Start Guide](./quickstart.md) to deploy your first function
- Explore the [API Reference](./api-reference.md) for detailed information on available services
- Check out the [Examples](../examples/) directory for sample applications
