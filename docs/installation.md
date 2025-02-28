# R3E FaaS Installation Guide

This guide provides detailed instructions for installing and setting up the R3E FaaS platform in various environments.

## Prerequisites

Before installing the R3E FaaS platform, ensure you have the following prerequisites:

- **Operating System**: Linux (Ubuntu 20.04+ recommended), macOS, or Windows with WSL2
- **Rust**: Version 1.60+ with Cargo
- **Node.js**: Version 16+
- **Docker**: Latest version (optional, for containerized deployment)
- **Git**: Latest version
- **Database**: RocksDB (installed automatically as a dependency)

## Installation Methods

There are several ways to install and run the R3E FaaS platform:

1. **Local Installation**: Build and run from source
2. **Docker Installation**: Run using Docker containers
3. **DevContainer**: Use VS Code DevContainer for development

## Local Installation

### 1. Clone the Repository

```bash
git clone https://github.com/R3E-Network/r3e-faas.git
cd r3e-faas
```

### 2. Build the Project

```bash
# Build all crates
cargo build

# Build with release optimizations
cargo build --release
```

### 3. Configure the Platform

Create a configuration file or use environment variables:

```bash
# Create a configuration directory
mkdir -p ~/.r3e-faas

# Create a configuration file
cat > ~/.r3e-faas/config.yaml << EOL
general:
  environment: development
  log_level: info

api:
  port: 8080
  host: 127.0.0.1

storage:
  type: rocksdb
  path: ~/.r3e-faas/data

neo:
  rpc_url: https://testnet.rpc.neo.org
  network: testnet

ethereum:
  rpc_url: https://goerli.infura.io/v3/your-api-key
  network: goerli
EOL
```

Alternatively, you can use environment variables:

```bash
export R3E_FAAS__GENERAL__ENVIRONMENT=development
export R3E_FAAS__GENERAL__LOG_LEVEL=info
export R3E_FAAS__API__PORT=8080
export R3E_FAAS__API__HOST=127.0.0.1
export R3E_FAAS__STORAGE__TYPE=rocksdb
export R3E_FAAS__STORAGE__PATH=~/.r3e-faas/data
export R3E_FAAS__NEO__RPC_URL=https://testnet.rpc.neo.org
export R3E_FAAS__NEO__NETWORK=testnet
export R3E_FAAS__ETHEREUM__RPC_URL=https://goerli.infura.io/v3/your-api-key
export R3E_FAAS__ETHEREUM__NETWORK=goerli
```

### 4. Start the Platform

```bash
# Start the API server
cargo run --bin r3e-api

# Start the worker
cargo run --bin r3e-worker
```

### 5. Verify the Installation

```bash
# Check if the API server is running
curl http://localhost:8080/api/v1/health

# Expected output: {"status":"ok","version":"0.1.0"}
```

## Docker Installation

### 1. Clone the Repository

```bash
git clone https://github.com/R3E-Network/r3e-faas.git
cd r3e-faas
```

### 2. Configure the Platform

Create a `.env` file for Docker Compose:

```bash
cat > .env << EOL
R3E_FAAS__GENERAL__ENVIRONMENT=development
R3E_FAAS__GENERAL__LOG_LEVEL=info
R3E_FAAS__API__PORT=8080
R3E_FAAS__API__HOST=0.0.0.0
R3E_FAAS__STORAGE__TYPE=rocksdb
R3E_FAAS__STORAGE__PATH=/data/rocksdb
R3E_FAAS__NEO__RPC_URL=https://testnet.rpc.neo.org
R3E_FAAS__NEO__NETWORK=testnet
R3E_FAAS__ETHEREUM__RPC_URL=https://goerli.infura.io/v3/your-api-key
R3E_FAAS__ETHEREUM__NETWORK=goerli
EOL
```

### 3. Start the Platform with Docker Compose

```bash
# Start the development environment
docker-compose -f docker-compose.dev.yml up

# Start the production environment
docker-compose -f docker-compose.prod.yml up -d
```

### 4. Verify the Installation

```bash
# Check if the API server is running
curl http://localhost:8080/api/v1/health

# Expected output: {"status":"ok","version":"0.1.0"}
```

## DevContainer Installation

### 1. Prerequisites

- Visual Studio Code
- Docker
- [Remote - Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension

### 2. Clone the Repository

```bash
git clone https://github.com/R3E-Network/r3e-faas.git
```

### 3. Open in DevContainer

1. Open the cloned repository in VS Code
2. Click "Reopen in Container" when prompted, or use the command palette (F1) and select "Remote-Containers: Reopen in Container"

### 4. Build and Run

Once the DevContainer is running, you can build and run the platform:

```bash
# Build the project
cargo build

# Start the API server
cargo run --bin r3e-api

# Start the worker
cargo run --bin r3e-worker
```

## Configuration Options

The R3E FaaS platform can be configured using a YAML configuration file or environment variables.

### Configuration File Structure

```yaml
general:
  environment: development # or production
  log_level: info # debug, info, warn, error

api:
  port: 8080
  host: 127.0.0.1
  cors_allowed_origins: "*"
  request_timeout: 30 # seconds

storage:
  type: rocksdb # or memory
  path: ~/.r3e-faas/data

neo:
  rpc_url: https://testnet.rpc.neo.org
  network: testnet # or mainnet
  gas_bank_contract: 0x1234567890abcdef1234567890abcdef12345678
  meta_tx_contract: 0x1234567890abcdef1234567890abcdef12345678

ethereum:
  rpc_url: https://goerli.infura.io/v3/your-api-key
  network: goerli # or mainnet, sepolia
  gas_bank_contract: 0x1234567890abcdef1234567890abcdef12345678
  meta_tx_contract: 0x1234567890abcdef1234567890abcdef12345678

worker:
  max_concurrent_functions: 10
  function_timeout: 60 # seconds
  memory_limit: 512 # MB

zk:
  provider: zokrates # or bulletproofs, circom, bellman, arkworks
  storage_type: rocksdb
  storage_path: ~/.r3e-faas/data/zk
  max_circuit_size: 10485760 # 10 MB
  timeout: 300 # 5 minutes

fhe:
  scheme: tfhe # or openfhe
  storage_type: rocksdb
  storage_path: ~/.r3e-faas/data/fhe
  max_ciphertext_size: 10485760 # 10 MB
  timeout: 300 # 5 minutes

tee:
  provider: nitro # or sgx
  attestation_url: https://attestation.example.com
  attestation_timeout: 30 # seconds
```

### Environment Variables

Environment variables follow the pattern `R3E_FAAS__SECTION__KEY=value`:

```bash
# General
R3E_FAAS__GENERAL__ENVIRONMENT=development
R3E_FAAS__GENERAL__LOG_LEVEL=info

# API
R3E_FAAS__API__PORT=8080
R3E_FAAS__API__HOST=127.0.0.1
R3E_FAAS__API__CORS_ALLOWED_ORIGINS="*"
R3E_FAAS__API__REQUEST_TIMEOUT=30

# Storage
R3E_FAAS__STORAGE__TYPE=rocksdb
R3E_FAAS__STORAGE__PATH=~/.r3e-faas/data

# Neo
R3E_FAAS__NEO__RPC_URL=https://testnet.rpc.neo.org
R3E_FAAS__NEO__NETWORK=testnet
R3E_FAAS__NEO__GAS_BANK_CONTRACT=0x1234567890abcdef1234567890abcdef12345678
R3E_FAAS__NEO__META_TX_CONTRACT=0x1234567890abcdef1234567890abcdef12345678

# Ethereum
R3E_FAAS__ETHEREUM__RPC_URL=https://goerli.infura.io/v3/your-api-key
R3E_FAAS__ETHEREUM__NETWORK=goerli
R3E_FAAS__ETHEREUM__GAS_BANK_CONTRACT=0x1234567890abcdef1234567890abcdef12345678
R3E_FAAS__ETHEREUM__META_TX_CONTRACT=0x1234567890abcdef1234567890abcdef12345678

# Worker
R3E_FAAS__WORKER__MAX_CONCURRENT_FUNCTIONS=10
R3E_FAAS__WORKER__FUNCTION_TIMEOUT=60
R3E_FAAS__WORKER__MEMORY_LIMIT=512

# ZK
R3E_FAAS__ZK__PROVIDER=zokrates
R3E_FAAS__ZK__STORAGE_TYPE=rocksdb
R3E_FAAS__ZK__STORAGE_PATH=~/.r3e-faas/data/zk
R3E_FAAS__ZK__MAX_CIRCUIT_SIZE=10485760
R3E_FAAS__ZK__TIMEOUT=300

# FHE
R3E_FAAS__FHE__SCHEME=tfhe
R3E_FAAS__FHE__STORAGE_TYPE=rocksdb
R3E_FAAS__FHE__STORAGE_PATH=~/.r3e-faas/data/fhe
R3E_FAAS__FHE__MAX_CIPHERTEXT_SIZE=10485760
R3E_FAAS__FHE__TIMEOUT=300

# TEE
R3E_FAAS__TEE__PROVIDER=nitro
R3E_FAAS__TEE__ATTESTATION_URL=https://attestation.example.com
R3E_FAAS__TEE__ATTESTATION_TIMEOUT=30
```

## Troubleshooting

### Common Issues

#### RocksDB Installation Issues

If you encounter issues with RocksDB installation:

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y librocksdb-dev

# macOS
brew install rocksdb

# Windows
# Install using vcpkg or build from source
```

#### Port Already in Use

If port 8080 is already in use:

```bash
# Change the port in the configuration
export R3E_FAAS__API__PORT=8081

# Or check what's using the port
sudo lsof -i :8080
```

#### Docker Permission Issues

If you encounter permission issues with Docker:

```bash
# Add your user to the docker group
sudo usermod -aG docker $USER

# Log out and log back in for the changes to take effect
```

#### Memory Limits

If you encounter memory issues:

```bash
# Increase the memory limit in the configuration
export R3E_FAAS__WORKER__MEMORY_LIMIT=1024

# Or increase Docker memory limit
docker-compose -f docker-compose.dev.yml up -d --scale worker=1 --memory=2g
```

### Getting Help

If you encounter issues not covered in this guide:

- Check the logs for error messages
- Open an issue on GitHub
- Join the community chat

## Next Steps

After installation, you can:

- Follow the [Quick Start Guide](./quickstart.md) to deploy your first function
- Explore the [API Reference](./api-reference.md) for detailed information on available services
- Check out the [Examples](../examples/) directory for sample applications
