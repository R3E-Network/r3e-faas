# Installation Guide

This guide will help you set up the R3E FaaS platform on your system.

## Prerequisites

- Rust 1.75 or later
- RocksDB 6.20.3 or later
- Node.js 18 or later (for testing JavaScript functions)
- Docker (optional, for containerized deployment)

## System Dependencies

### Ubuntu/Debian

```bash
# Install system dependencies
sudo apt-get update && sudo apt-get install -y \
    build-essential \
    cmake \
    libclang-dev \
    libssl-dev \
    pkg-config \
    librocksdb-dev
```

### macOS

```bash
# Install system dependencies
brew install cmake pkg-config openssl rocksdb
```

## Installing from Source

1. Clone the repository:

```bash
git clone https://github.com/R3E-Network/r3e-faas.git
cd r3e-faas
```

2. Build the project:

```bash
cargo build --release
```

3. Install the binary:

```bash
cargo install --path r3e
```

## Docker Installation

For a containerized installation, you can use Docker:

```bash
# Build the Docker image
docker build -t r3e-faas .

# Run the container
docker run -p 8080:8080 r3e-faas
```

Alternatively, you can use Docker Compose:

```bash
# Start all services
docker-compose up -d
```

## Configuration

After installation, you need to configure the platform:

1. Create a configuration file:

```bash
cp config/r3e-faas.example.yaml config/r3e-faas.yaml
```

2. Edit the configuration file to match your environment:

```bash
# Edit with your favorite editor
nano config/r3e-faas.yaml
```

3. Set up environment variables (optional):

```bash
export R3E_FAAS__GENERAL__ENVIRONMENT=production
export R3E_FAAS__STORAGE__STORAGE_TYPE=rocksdb
export R3E_FAAS__RUNTIME__JS__MAX_MEMORY_MB=256
```

## Verifying Installation

To verify that the installation was successful:

```bash
# Check the version
r3e --version

# Start the worker
r3e worker --config config/r3e-faas.yaml
```

## Next Steps

- [Quick Start Guide](./quickstart.md)
- [Development Guide](./development.md)
- [API Reference](./api-reference.md)
