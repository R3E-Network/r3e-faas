# R3E FaaS Platform

The R3E Function-as-a-Service (FaaS) platform is a comprehensive serverless computing solution designed specifically for blockchain applications. It enables developers to deploy and execute JavaScript functions in a secure, scalable, and cost-effective environment, with built-in support for Neo N3 and Ethereum blockchains.

## Features

- **Serverless JavaScript Execution**: Deploy and run JavaScript functions without managing infrastructure
- **Blockchain Integration**: Seamless integration with Neo N3 and Ethereum blockchains
- **Secure Sandboxing**: Isolated execution environments for user functions
- **Event-Driven Architecture**: Trigger functions based on blockchain events, time schedules, or market conditions
- **Built-in Services**: Comprehensive suite of services for common blockchain operations
- **Trusted Execution Environment (TEE)**: Support for secure computing using Intel SGX and AWS Nitro
- **Zero-Knowledge Computing**: ZK-SNARK implementations for privacy-preserving computations
- **Fully Homomorphic Encryption**: Compute on encrypted data without decryption
- **Meta Transactions**: Support for gasless transactions on Neo N3 and Ethereum (EIP-712)
- **Secret Management**: Secure storage and management of sensitive data
- **RocksDB Storage**: High-performance persistent storage for function data
- **Custom Triggers**: Define complex event triggers from multiple sources
- **Auto Contract Execution**: Automatically invoke smart contracts based on triggers

## Built-in Services

### Core Services

- **Balance Management**: Track and manage user balances for service usage
- **Identity Verification**: Secure user authentication and authorization
- **Pricing Service**: Dynamic pricing for platform resources
- **Indexing Service**: Efficient data indexing for quick retrieval
- **Bridge Operations**: Cross-chain asset and data transfers

### Blockchain Services

- **Oracle Service**: Provide off-chain data to smart contracts
  - Price feeds (NEO/USD, GAS/USD, BTC/USD, ETH/USD, etc.)
  - Random number generation
  - Custom data sources
- **Gas Bank**: Pay for user transactions on Neo N3
- **Meta Transaction Service**: Support for gasless transactions
  - Neo N3 meta transactions
  - Ethereum meta transactions (EIP-712)
- **Auto Contract Service**: Automatic smart contract execution based on triggers

### Cryptographic Services

- **Trusted Execution Environment (TEE)**: Secure computing environments
  - Intel SGX support
  - AWS Nitro support
  - Attestation verification
  - Secure key management
- **Zero-Knowledge Computing**: Privacy-preserving computations
  - ZoKrates implementation
  - Bulletproofs implementation
  - Circom/SnarkJS integration
  - Bellman integration
  - Arkworks integration
- **Fully Homomorphic Encryption**: Compute on encrypted data
  - TFHE implementation
  - OpenFHE implementation

### Event Processing

- **Event Sources**: Capture events from various sources
  - Neo N3 blockchain events
  - Ethereum blockchain events
  - Time-based events
  - Market price events
  - Custom events
- **Trigger Service**: Define and evaluate complex triggers
  - Blockchain event triggers
  - Time schedule triggers
  - Market condition triggers
  - Custom triggers
- **Function Service**: Execute user functions based on triggers

## Getting Started

### Prerequisites

- Node.js 16+
- Rust 1.60+
- Docker (optional, for containerized deployment)

### Installation

```bash
# Clone the repository
git clone https://github.com/R3E-Network/r3e-faas.git
cd r3e-faas

# Install dependencies
cargo build
```

### Development Environment

For an easy development setup, use the provided DevContainer configuration:

```bash
# Open in VS Code with DevContainer extension
code .
# Then click "Reopen in Container" when prompted
```

Alternatively, use Docker Compose for development:

```bash
# Start development environment
docker-compose -f docker-compose.dev.yml up
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific tests
cargo test --package r3e-neo-services
cargo test --package r3e-oracle
```

### Deployment

For production deployment, use Docker Compose:

```bash
# Start production environment
docker-compose -f docker-compose.prod.yml up -d
```

## Documentation

- [Installation Guide](docs/installation.md)
- [Quick Start Guide](docs/quickstart.md)
- [Development Guide](docs/development.md)
- [API Reference](docs/api-reference.md)
- [Docker Development](docs/docker-development.md)
- [Docker Production](docs/docker-production.md)
- [ZK Service](docs/zk-service.md)
- [FHE Service](docs/fhe-service.md)

## Examples

The `examples` directory contains sample applications demonstrating various features of the platform:

- [Auto Contract Execution](examples/auto_contract.js)
- [Secret Management](examples/secret_management.js)
- [ZK Computing](examples/zk_computing.js)
- [FHE Computing](examples/fhe_computing.js)
- [Neo N3 Examples](examples/neo-n3/)
  - [Meta Transaction Entry Contract](examples/neo-n3/meta-tx-entry-contract/)
  - [Blockchain Gateway Contract](examples/neo-n3/blockchain-gateway-contract/)
  - [Oracle Services](examples/neo-n3/oracle-services/)
  - [TEE Services](examples/neo-n3/tee-services/)
  - [Blockchain Events](examples/neo-n3/blockchain-events/)
  - [Service API](examples/neo-n3/service-api/)

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
