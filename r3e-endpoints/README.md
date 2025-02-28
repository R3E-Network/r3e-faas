# R3E Endpoints

This crate provides a RESTful API for accessing R3E FaaS services. It allows users to connect their Ethereum and Neo N3 wallets, sign messages, and interact with the R3E FaaS services.

## Features

- Multi-chain wallet connectivity (Ethereum and Neo N3)
- Message signing and verification
- Meta transaction submission and status tracking
- Service discovery and invocation
- JWT-based authentication

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Cargo
- R3E FaaS services

### Installation

Add the crate to your workspace in `Cargo.toml`:

```toml
[workspace]
members = [
    # ...
    "r3e-endpoints",
    # ...
]
```

### Configuration

The crate can be configured using environment variables:

- `R3E_ENDPOINTS_PORT`: The port to listen on (default: 3000)
- `R3E_ENDPOINTS_HOST`: The host to bind to (default: 0.0.0.0)
- `R3E_ENDPOINTS_JWT_SECRET`: The secret to use for JWT tokens
- `R3E_ENDPOINTS_JWT_EXPIRATION`: The expiration time for JWT tokens in seconds (default: 86400)

### Usage

Start the server:

```bash
cargo run -p r3e-endpoints
```

## API Endpoints

### Health

- `GET /health`: Check the health of the service

### Authentication

- `POST /auth/login`: Login with username and password
- `POST /auth/register`: Register a new user
- `POST /auth/refresh`: Refresh a JWT token

### Wallet

- `POST /wallet/connect`: Connect a wallet
- `POST /wallet/sign`: Sign a message
- `POST /wallet/verify`: Verify a signature

### Meta Transactions

- `POST /meta-tx/submit`: Submit a meta transaction
- `GET /meta-tx/status/:id`: Get the status of a meta transaction
- `GET /meta-tx/transaction/:id`: Get a meta transaction
- `GET /meta-tx/nonce/:address`: Get the next nonce for an address

### Services

- `GET /services`: List available services
- `GET /services/:id`: Get a service
- `POST /services/:id/invoke`: Invoke a service

## Frontend Integration

The R3E FaaS frontend application integrates with this API to provide a user-friendly interface for accessing R3E FaaS services. It allows users to connect their Ethereum and Neo N3 wallets, sign messages, and interact with the R3E FaaS services.

## License

Copyright @ 2023 - 2024, R3E Network. All Rights Reserved.
