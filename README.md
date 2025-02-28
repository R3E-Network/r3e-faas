# R3E FaaS: A Function as a Service Platform for Web3

R3E FaaS is a serverless computing platform built specifically for Web3 applications, enabling developers to run JavaScript functions in response to blockchain events without managing infrastructure. The platform integrates with the Neo N3 blockchain, providing built-in services for oracle data, gas management, and secure execution environments.

## Key Features

- **Blockchain Integration**: Seamlessly interact with Neo N3 blockchain
- **Secure Sandboxed Execution**: Run user JavaScript functions in isolated environments
- **Built-in Web3 Services**: Access oracle data, gas management, and TEE capabilities
- **Token-based Billing**: Pay for execution using NEO or GAS tokens
- **Event-driven Architecture**: Execute functions in response to blockchain events
- **Secret Management**: Store and access encrypted secrets in your functions
- **Custom Triggers**: Set up triggers based on blockchain events, time, or market prices
- **Automatic Smart Contracts**: Automatically invoke smart contracts upon specified triggers

## Getting Started

- [Installation Guide](./docs/installation.md)
- [Quick Start Guide](./docs/quickstart.md)
- [Development Guide](./docs/development.md)
- [API Reference](./docs/api-reference.md)
- [Docker Development](./docs/docker-development.md)
- [Docker Production](./docs/docker-production.md)

## Project Structure

The R3E FaaS platform is organized as a Rust workspace with multiple crates:

- **Core Crates**:
  - `r3e-core`: Common types, traits, and utilities
  - `r3e-config`: Configuration management
  - `r3e-store`: Storage abstractions and implementations
  - `r3e-event`: Event handling and processing

- **Service Crates**:
  - `r3e-built-in-services`: Consolidated services (oracle, gas bank, TEE, etc.)
  - `r3e-neo-services`: Neo N3 blockchain integration
  - `r3e-oracle`: Oracle services for external data
  - `r3e-tee`: Trusted Execution Environment implementation
  - `r3e-secrets`: Secret management with encryption

- **Runtime Crates**:
  - `r3e-deno`: JavaScript runtime based on deno-core with V8 engine
  - `r3e-runtime`: Function execution runtime
  - `r3e-worker`: Worker node implementation
  - `r3e-scheduler`: Function scheduling and distribution

- **API and Utilities**:
  - `r3e-api`: HTTP API server
  - `r3e-proc-macros`: Procedural macros for code generation
  - `r3e-runlog`: Function execution logging
  - `r3e-stock`: Blockchain history and data querying

- **Main Application**:
  - `r3e`: Command-line interface and entry point

## Built-in Services

### Oracle Service

The Oracle Service provides access to external data sources from within your JavaScript functions:

- **Price Data**: Get cryptocurrency and asset prices
- **Random Numbers**: Generate verifiable random numbers
- **Weather Data**: Access global weather information
- **Custom Data**: Request data from any HTTP endpoint

### Gas Bank Service

The Gas Bank Service manages GAS tokens for function execution:

- **Account Management**: Create and manage gas bank accounts
- **Deposits**: Deposit GAS tokens to your account
- **Withdrawals**: Withdraw unused GAS tokens
- **Gas Payment**: Automatically pay for transaction gas costs

### TEE (Trusted Execution Environment)

The TEE Service provides secure execution for sensitive operations:

- **Secure Execution**: Run code in an isolated, secure environment
- **Attestation**: Verify the integrity of the execution environment
- **Confidential Computing**: Process sensitive data with privacy guarantees
- **Cloud Provider Support**: AWS Nitro, Google Confidential Computing, Azure Confidential Computing

### Data Indexing Service

The Data Indexing Service provides efficient data querying and storage capabilities:

- **Flexible Querying**: Query indexed data with complex filters and sorting
- **Collection Management**: Create and manage data collections
- **Document Indexing**: Index JSON documents with customizable indexes
- **Real-time Updates**: Get real-time updates when indexed data changes

### Identity Service

The Identity Service provides decentralized identity management:

- **DID Support**: Create and manage Decentralized Identifiers (DIDs)
- **Credential Management**: Issue and verify verifiable credentials
- **Authentication**: Multiple authentication methods (keys, passwords, biometrics)
- **Recovery Options**: Social recovery and backup mechanisms

### Cross-Chain Bridge Service

The Cross-Chain Bridge Service enables interoperability between blockchains:

- **Token Transfers**: Transfer tokens between Neo N3 and other blockchains
- **Asset Wrapping**: Wrap assets from one chain to another
- **Message Passing**: Pass messages between smart contracts on different chains
- **Transaction Monitoring**: Track cross-chain transactions

### Secret Management Service

The Secret Management Service provides secure storage and access to sensitive data:

- **Encryption**: AES-256-GCM encryption for all secrets
- **Access Control**: Function-specific access control
- **User-owned Secrets**: Secrets are owned by the user who created them
- **Secure Storage**: Secrets are stored securely using RocksDB

### Automatic Smart Contract Service

The Automatic Smart Contract Service enables automatic invocation of smart contracts:

- **Trigger-based Execution**: Execute smart contracts based on triggers
- **Multiple Trigger Types**: Blockchain events, time-based, market price, custom events
- **Execution Tracking**: Track execution history and status
- **Contract Management**: Create, update, and delete automatic contracts

### Zero-Knowledge Computing Service

The Zero-Knowledge Computing Service provides cryptographic proof generation and verification:

- **Multiple ZK Solutions**: Support for ZoKrates, Bulletproofs, and Circom
- **Circuit Compilation**: Compile ZK circuits from source code
- **Key Generation**: Generate proving and verification keys
- **Proof Generation**: Create ZK proofs with public and private inputs
- **Proof Verification**: Verify ZK proofs without revealing private inputs

### Fully Homomorphic Encryption Service

The Fully Homomorphic Encryption Service enables computation on encrypted data:

- **Multiple FHE Schemes**: Support for TFHE, OpenFHE, SEAL, HElib, and Lattigo
- **Key Management**: Generate and manage FHE key pairs
- **Homomorphic Operations**: Perform addition, subtraction, multiplication on encrypted data
- **Noise Budget Management**: Track and manage noise budget for operations
- **Secure Computation**: Compute on encrypted data without decryption

## Token Deposit and Balance Management

### How It Works

1. **Deposit Tokens**: Send NEO or GAS tokens to the platform's address to fund your account
2. **Check Balance**: View your current balance through the API or dashboard
3. **Execute Functions**: Run JavaScript functions, which consume GAS based on execution time and resources
4. **Withdraw Remaining Balance**: Withdraw unused tokens back to your wallet at any time

### Fee Structure

Function execution costs are calculated based on:

- **Execution Time**: Longer-running functions cost more GAS
- **Memory Usage**: Functions using more memory incur higher costs
- **Network Operations**: External API calls and blockchain interactions have additional costs
- **Storage Usage**: Storing data incurs additional costs
- **TEE Usage**: Using Trusted Execution Environments incurs additional costs

## Sandboxed JavaScript Execution

User JavaScript functions run in separate sandboxes with the following security features:

- **Memory Limits**: Prevent excessive memory consumption
- **Execution Time Limits**: Automatically terminate long-running functions
- **Resource Isolation**: Restrict access to system resources
- **Permission System**: Explicitly request access to network, file system, etc.
- **JIT Disabled**: Enhanced security through JIT-less execution

## Usage Guidelines

### Deploying a Function

```javascript
// Example function that responds to Neo N3 blockchain events
export default function(event) {
  // Access blockchain event data
  const { txHash, blockNumber, timestamp } = event.data;
  
  // Use built-in services
  const price = r3e.oracle.getPrice("NEO/USD");
  
  // Perform computation
  const result = processData(event.data);
  
  // Return result
  return { 
    success: true, 
    txHash, 
    price, 
    result 
  };
}
```

### Managing Your Balance

```javascript
// Check your current balance
const balance = await r3e.neoServices.getBalance();
console.log(`NEO Balance: ${balance.neo_balance}`);
console.log(`GAS Balance: ${balance.gas_balance}`);

// Withdraw tokens
const withdrawal = await r3e.neoServices.withdraw("GAS", 100);
console.log(`Withdrawal transaction: ${withdrawal.tx_hash}`);
```

### Using Oracle Services

```javascript
// Get cryptocurrency price
const neoPrice = await r3e.oracle.getPrice("NEO/USD");

// Get random number
const randomNumber = await r3e.oracle.getRandom(1, 100);

// Get weather data
const weather = await r3e.oracle.getWeather("New York");
```

### Managing Secrets

```javascript
// Store a secret
await r3e.secrets.set("API_KEY", "your-api-key");

// Get a secret
const apiKey = await r3e.secrets.get("API_KEY");

// Use the secret
const response = await fetch("https://api.example.com/data", {
  headers: {
    "Authorization": `Bearer ${apiKey}`
  }
});
```

### Setting Up Triggers

```javascript
// Create a blockchain event trigger
const trigger = await r3e.trigger.create({
  type: "blockchain",
  event: "NeoNewBlock"
});

// Create a time-based trigger (every hour)
const timeTrigger = await r3e.trigger.create({
  type: "time",
  schedule: "0 * * * *"
});

// Create a price trigger (when NEO price changes by 5%)
const priceTrigger = await r3e.trigger.create({
  type: "price",
  asset: "NEO/USD",
  change: 5
});
```

### Creating Automatic Smart Contracts

```javascript
// Create an automatic contract that executes when NEO price is above $50
const contract = await r3e.autoContract.create({
  trigger: {
    type: "price",
    asset: "NEO/USD",
    condition: "above",
    threshold: 50
  },
  action: {
    contract: "0x1234567890abcdef",
    method: "transfer",
    params: ["NbnjKGMBJzJ6j5PHeYhjJDaQ5Vy5UYu4Fv", 5]
  }
});
```

### Using Zero-Knowledge Computing

```javascript
// Compile a ZoKrates circuit for proving knowledge of factors
const circuitSource = `
  def main(private field a, private field b, field c) -> bool:
    return a * b == c
`;
const circuitId = zk.compileCircuit(circuitSource, zk.CircuitType.ZOKRATES, "multiply");

// Generate keys
const { provingKeyId, verificationKeyId } = zk.generateKeys(circuitId);

// Generate a proof (proving we know factors of 6 without revealing them)
const publicInputs = ["6"]; // The public value c = 6
const privateInputs = ["2", "3"]; // The private values a = 2, b = 3
const proofId = zk.generateProof(circuitId, provingKeyId, publicInputs, privateInputs);

// Verify the proof
const isValid = zk.verifyProof(proofId, verificationKeyId, publicInputs);
console.log(`Proof is valid: ${isValid}`);
```

### Using Fully Homomorphic Encryption

```javascript
// Generate FHE keys
const keyPairId = fhe.generateKeys(fhe.SchemeType.TFHE);
const publicKeyId = `${keyPairId}_public`;
const privateKeyId = `${keyPairId}_private`;

// Encrypt data
const ciphertext1Id = fhe.encrypt(publicKeyId, "42");
const ciphertext2Id = fhe.encrypt(publicKeyId, "8");

// Perform homomorphic operations
const addResultId = fhe.add(ciphertext1Id, ciphertext2Id);
const multiplyResultId = fhe.multiply(ciphertext1Id, ciphertext2Id);

// Decrypt results
const addResult = fhe.decrypt(privateKeyId, addResultId, true);
const multiplyResult = fhe.decrypt(privateKeyId, multiplyResultId, true);
console.log(`Addition result: ${addResult}`); // 50
console.log(`Multiplication result: ${multiplyResult}`); // 336
```

## Business Model

The platform offers multiple pricing tiers and subscription models to suit different needs:

### Pricing Tiers

- **Basic**: For development and testing with limited resources
- **Standard**: For production workloads with moderate resources
- **Premium**: For demanding applications with high resources
- **Enterprise**: For custom solutions with dedicated support

### Resource-Based Pricing

Execution costs are calculated based on resource usage:

- **Execution Time**: Cost per millisecond of function execution
- **Memory Usage**: Cost per MB of memory used
- **Storage Usage**: Cost per MB of data stored
- **Network Usage**: Cost per MB of data transferred
- **TEE Usage**: Additional cost for secure execution environments

### Subscription Models

Choose from different subscription models:

- **Pay-as-you-go**: Pay only for what you use
- **Monthly**: Fixed monthly fee for a certain amount of resources
- **Annual**: Discounted annual fee for committed usage
- **Reserved Capacity**: Discounted rates for reserved capacity

## Docker Support

The R3E FaaS platform provides Docker support for both development and production environments:

### Development Environment

Use the development Docker image for a consistent development environment:

```bash
# Build the development Docker image
docker build -f Dockerfile.dev -t r3e-faas-dev .

# Run the container with mounted source code
docker run -v $(pwd):/app -p 8080:8080 r3e-faas-dev
```

### Production Deployment

Deploy the platform in production using Docker:

```bash
# Build the production Docker image
docker build -t r3e-faas:latest .

# Run the container
docker run -d \
  --name r3e-faas \
  -p 8080:8080 \
  -v /var/lib/r3e-faas:/data \
  -e R3E_FAAS__GENERAL__ENVIRONMENT=production \
  r3e-faas:latest
```

For more details, see the [Docker Development](./docs/docker-development.md) and [Docker Production](./docs/docker-production.md) guides.

## Architecture

The R3E FaaS platform follows a modular microservice architecture:

```
┌─────────────────────────────────────────────────────────────────┐
│                           API Layer                             │
│                         (r3e-api)                               │
└───────────────────────────┬─────────────────────────────────────┘
                            │
┌───────────────────────────┼─────────────────────────────────────┐
│                           │                                     │
│  ┌─────────────────┐    ┌─┴──────────────┐    ┌──────────────┐  │
│  │  Event Sources  │    │   Scheduler    │    │    Worker    │  │
│  │  (r3e-event)    │───►│ (r3e-scheduler)│───►│ (r3e-worker) │  │
│  └─────────────────┘    └────────────────┘    └──────┬───────┘  │
│                                                      │          │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────▼──────┐   │
│  │  Storage Layer  │◄───┤  Runtime Layer  │◄───┤ JavaScript │   │
│  │  (r3e-store)    │    │  (r3e-runtime)  │    │ (r3e-deno) │   │
│  └─────────────────┘    └─────────────────┘    └────────────┘   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
                            │
┌───────────────────────────┼─────────────────────────────────────┐
│                           │                                     │
│  ┌─────────────────┐    ┌─┴──────────────┐    ┌──────────────┐  │
│  │  Oracle Service │    │   Gas Bank     │    │     TEE      │  │
│  │  (r3e-oracle)   │    │(r3e-neo-service)│    │  (r3e-tee)   │  │
│  └─────────────────┘    └────────────────┘    └──────────────┘  │
│                                                                 │
│  ┌─────────────────┐    ┌─────────────────┐    ┌──────────────┐  │
│  │ Secret Service  │    │ Indexing Service│    │Identity Service│ │
│  │  (r3e-secrets)  │    │(r3e-built-in-svc)│    │(r3e-built-in-svc)│ │
│  └─────────────────┘    └─────────────────┘    └──────────────┘  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Contributing

Contributions are welcome! Please see the [Development Guide](./docs/development.md) for more information on how to contribute to the project.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
