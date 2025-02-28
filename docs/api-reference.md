# R3E FaaS API Reference

This document provides a comprehensive reference for the R3E FaaS platform API, including JavaScript functions, built-in services, and configuration options.

## JavaScript API

The R3E FaaS platform provides a rich JavaScript API for interacting with the platform's services. This API is available in all JavaScript functions deployed to the platform.

### Global Objects

The following global objects are available in all JavaScript functions:

- `r3e`: The main object for accessing platform services
- `console`: Standard console object for logging
- `fetch`: Standard fetch API for making HTTP requests
- `crypto`: Standard Web Crypto API for cryptographic operations

### r3e Object

The `r3e` object provides access to all platform services:

```javascript
// Access Oracle service
const price = await r3e.oracle.getPrice("NEO/USD");

// Access Neo services
const balance = await r3e.neoServices.getBalance();

// Access Secret management
const apiKey = await r3e.secrets.get("API_KEY");

// Access Trigger management
const trigger = await r3e.trigger.create({ type: "blockchain", event: "NeoNewBlock" });

// Access Auto Contract service
const contract = await r3e.autoContract.create({ /* ... */ });

// Access Zero-Knowledge Computing
const circuitId = await r3e.zk.compileCircuit(/* ... */);

// Access Fully Homomorphic Encryption
const keyPairId = await r3e.fhe.generateKeys(/* ... */);

// Access TEE service
const attestation = await r3e.tee.getAttestation();
```

## Oracle Service API

The Oracle Service provides access to external data sources.

### Price Data

```javascript
// Get cryptocurrency price
const neoPrice = await r3e.oracle.getPrice("NEO/USD");
const gasPrice = await r3e.oracle.getPrice("GAS/USD");
const btcPrice = await r3e.oracle.getPrice("BTC/USD");
const ethPrice = await r3e.oracle.getPrice("ETH/USD");

// Get price with options
const neoPriceWithOptions = await r3e.oracle.getPrice("NEO/USD", {
  source: "coinmarketcap",  // Specific data source
  timeout: 5000,            // Timeout in milliseconds
  cache: true,              // Use cached data if available
  cacheTtl: 60              // Cache TTL in seconds
});
```

### Random Numbers

```javascript
// Get random number between min and max (inclusive)
const randomNumber = await r3e.oracle.getRandom(1, 100);

// Get random number with options
const randomNumberWithOptions = await r3e.oracle.getRandom(1, 100, {
  seed: "custom-seed",      // Custom seed for deterministic randomness
  secure: true,             // Use cryptographically secure randomness
  verifiable: true          // Generate verifiable random number
});

// Get random bytes
const randomBytes = await r3e.oracle.getRandomBytes(32);
```

### Custom Data

```javascript
// Get custom data from HTTP endpoint
const weatherData = await r3e.oracle.getCustomData("https://api.weather.com/data", {
  method: "GET",
  headers: {
    "Authorization": "Bearer token"
  },
  timeout: 5000
});
```

## Neo Services API

The Neo Services provide integration with the Neo N3 blockchain.

### Gas Bank

```javascript
// Create a gas bank account
const account = await r3e.neoServices.gasBank.createAccount({
  address: "neo1abc123def456",
  feeModel: "fixed",
  feeValue: 10,
  creditLimit: 1000
});

// Get a gas bank account
const account = await r3e.neoServices.gasBank.getAccount("neo1abc123def456");

// Deposit gas to an account
const deposit = await r3e.neoServices.gasBank.deposit({
  txHash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
  address: "neo1abc123def456",
  amount: 1000
});

// Withdraw gas from an account
const withdrawal = await r3e.neoServices.gasBank.withdraw({
  address: "neo1abc123def456",
  amount: 500
});

// Pay gas for a transaction
const transaction = await r3e.neoServices.gasBank.payGas({
  txHash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
  address: "neo1abc123def456",
  amount: 10
});

// Get current gas price
const gasPrice = await r3e.neoServices.gasBank.getGasPrice();
```

### Meta Transactions

```javascript
// Submit a meta transaction
const metaTx = await r3e.neoServices.metaTx.submit({
  txData: "0x1234567890abcdef",
  sender: "neo1abc123def456",
  signature: "0xsignature",
  nonce: 1,
  deadline: Math.floor(Date.now() / 1000) + 3600,
  feeModel: "fixed",
  feeAmount: 10
});

// Get meta transaction status
const status = await r3e.neoServices.metaTx.getStatus("request-id");

// Get meta transaction details
const transaction = await r3e.neoServices.metaTx.getTransaction("request-id");

// Get next nonce for a sender
const nonce = await r3e.neoServices.metaTx.getNextNonce("neo1abc123def456");
```

### Ethereum Meta Transactions (EIP-712)

```javascript
// Create an EIP-712 typed data
const typedData = await r3e.neoServices.metaTx.createEIP712TypedData({
  domain: {
    name: "My App",
    version: "1",
    chainId: 1,
    verifyingContract: "0x1234567890abcdef1234567890abcdef12345678"
  },
  types: {
    Person: [
      { name: "name", type: "string" },
      { name: "wallet", type: "address" }
    ],
    Mail: [
      { name: "from", type: "Person" },
      { name: "to", type: "Person" },
      { name: "contents", type: "string" }
    ]
  },
  primaryType: "Mail",
  message: {
    from: {
      name: "Alice",
      wallet: "0x1234567890abcdef1234567890abcdef12345678"
    },
    to: {
      name: "Bob",
      wallet: "0xabcdef1234567890abcdef1234567890abcdef12"
    },
    contents: "Hello, Bob!"
  }
});

// Submit an EIP-712 meta transaction
const metaTx = await r3e.neoServices.metaTx.submitEIP712({
  typedData: typedData,
  signature: "0xsignature",
  sender: "0x1234567890abcdef1234567890abcdef12345678",
  nonce: 1,
  deadline: Math.floor(Date.now() / 1000) + 3600,
  feeModel: "fixed",
  feeAmount: 10
});
```

## Secret Management API

The Secret Management Service provides secure storage and access to sensitive data.

```javascript
// Store a secret
await r3e.secrets.set("API_KEY", "your-api-key");

// Get a secret
const apiKey = await r3e.secrets.get("API_KEY");

// Delete a secret
await r3e.secrets.delete("API_KEY");

// List all secrets
const secrets = await r3e.secrets.list();

// Store a secret with options
await r3e.secrets.set("API_KEY", "your-api-key", {
  ttl: 3600,                // Time-to-live in seconds
  functionScoped: true,     // Only accessible by this function
  encrypted: true           // Encrypt the secret (default)
});
```

## Trigger API

The Trigger Service enables setting up triggers for automatic function execution.

```javascript
// Create a blockchain event trigger
const trigger = await r3e.trigger.create({
  type: "blockchain",
  name: "New Block Trigger",
  description: "Trigger on new Neo blocks",
  blockchain: "neo",
  event: "NewBlock"
});

// Create a time-based trigger
const timeTrigger = await r3e.trigger.create({
  type: "time",
  name: "Hourly Trigger",
  description: "Trigger every hour",
  schedule: "0 * * * *",    // Cron expression
  timezone: "UTC"
});

// Create a market price trigger
const priceTrigger = await r3e.trigger.create({
  type: "market",
  name: "NEO Price Trigger",
  description: "Trigger when NEO price changes significantly",
  assetPair: "NEO/USD",
  condition: "change",
  threshold: 5,             // 5% change
  direction: "any"          // "up", "down", or "any"
});

// Get a trigger
const trigger = await r3e.trigger.get("trigger-id");

// Update a trigger
await r3e.trigger.update("trigger-id", {
  name: "Updated Trigger Name",
  description: "Updated description",
  enabled: false
});

// Delete a trigger
await r3e.trigger.delete("trigger-id");

// List all triggers
const triggers = await r3e.trigger.list();

// List triggers by type
const blockchainTriggers = await r3e.trigger.list({ type: "blockchain" });
```

## Auto Contract API

The Auto Contract Service enables automatic smart contract execution based on triggers.

```javascript
// Create an automatic contract
const contract = await r3e.autoContract.create({
  trigger: {
    type: "price",
    assetPair: "NEO/USD",
    condition: "above",
    threshold: 50
  },
  action: {
    blockchain: "neo",
    contract: "0x1234567890abcdef1234567890abcdef12345678",
    method: "transfer",
    params: [
      { type: "Hash160", value: "NbnjKGMBJzJ6j5PHeYhjJDaQ5Vy5UYu4Fv" },
      { type: "Integer", value: 5 }
    ]
  },
  maxGas: 10,
  priority: 1
});

// Get an automatic contract
const contract = await r3e.autoContract.get("contract-id");

// Update an automatic contract
await r3e.autoContract.update("contract-id", {
  maxGas: 20,
  priority: 2,
  enabled: false
});

// Delete an automatic contract
await r3e.autoContract.delete("contract-id");

// List all automatic contracts
const contracts = await r3e.autoContract.list();

// Get execution history
const history = await r3e.autoContract.getHistory("contract-id");
```

## Zero-Knowledge Computing API

The Zero-Knowledge Computing Service provides cryptographic proof generation and verification.

```javascript
// Compile a ZoKrates circuit
const circuitSource = `
  def main(private field a, private field b, field c) -> bool:
    return a * b == c
`;
const circuitId = await r3e.zk.compileCircuit(circuitSource, r3e.zk.CircuitType.ZOKRATES, "multiply");

// Generate keys
const { provingKeyId, verificationKeyId } = await r3e.zk.generateKeys(circuitId);

// Generate a proof
const publicInputs = ["6"]; // The public value c = 6
const privateInputs = ["2", "3"]; // The private values a = 2, b = 3
const proofId = await r3e.zk.generateProof(circuitId, provingKeyId, publicInputs, privateInputs);

// Verify a proof
const isValid = await r3e.zk.verifyProof(proofId, verificationKeyId, publicInputs);

// List available circuits
const circuits = await r3e.zk.listCircuits();

// Get circuit details
const circuit = await r3e.zk.getCircuit(circuitId);

// Delete a circuit
await r3e.zk.deleteCircuit(circuitId);
```

### Circom/SnarkJS Integration

```javascript
// Compile a Circom circuit
const circuitSource = `
  pragma circom 2.0.0;
  
  template Multiplier() {
    signal input a;
    signal input b;
    signal output c;
    
    c <== a * b;
  }
  
  component main = Multiplier();
`;
const circuitId = await r3e.zk.compileCircuit(circuitSource, r3e.zk.CircuitType.CIRCOM, "multiplier");

// Generate a proof with Circom/SnarkJS
const publicInputs = { c: "6" };
const privateInputs = { a: "2", b: "3" };
const proofId = await r3e.zk.generateProof(circuitId, provingKeyId, publicInputs, privateInputs);
```

### Bulletproofs Integration

```javascript
// Create a Bulletproofs range proof
const commitment = await r3e.zk.createRangeProof({
  value: 42,
  min: 0,
  max: 100,
  blindingFactor: "random" // or provide a specific blinding factor
});

// Verify a Bulletproofs range proof
const isValid = await r3e.zk.verifyRangeProof(commitment.proof, commitment.commitment, 0, 100);
```

## Fully Homomorphic Encryption API

The Fully Homomorphic Encryption Service enables computation on encrypted data.

```javascript
// Generate FHE keys
const keyPairId = await r3e.fhe.generateKeys(r3e.fhe.SchemeType.TFHE);
const publicKeyId = `${keyPairId}_public`;
const privateKeyId = `${keyPairId}_private`;

// Encrypt data
const ciphertext1Id = await r3e.fhe.encrypt(publicKeyId, "42");
const ciphertext2Id = await r3e.fhe.encrypt(publicKeyId, "8");

// Perform homomorphic addition
const addResultId = await r3e.fhe.add(ciphertext1Id, ciphertext2Id);

// Perform homomorphic multiplication
const multiplyResultId = await r3e.fhe.multiply(ciphertext1Id, ciphertext2Id);

// Decrypt results
const addResult = await r3e.fhe.decrypt(privateKeyId, addResultId);
const multiplyResult = await r3e.fhe.decrypt(privateKeyId, multiplyResultId);

// List available keys
const keys = await r3e.fhe.listKeys();

// Delete keys
await r3e.fhe.deleteKey(publicKeyId);
await r3e.fhe.deleteKey(privateKeyId);
```

## TEE Service API

The Trusted Execution Environment Service provides secure execution for sensitive operations.

```javascript
// Get attestation report
const attestation = await r3e.tee.getAttestation();

// Verify attestation
const isValid = await r3e.tee.verifyAttestation(attestation);

// Encrypt data for TEE
const encryptedData = await r3e.tee.encrypt("sensitive data");

// Decrypt data in TEE
const decryptedData = await r3e.tee.decrypt(encryptedData);

// Generate a key in TEE
const keyId = await r3e.tee.generateKey("aes-256-gcm");

// Encrypt data with TEE key
const encryptedData = await r3e.tee.encryptWithKey(keyId, "sensitive data");

// Decrypt data with TEE key
const decryptedData = await r3e.tee.decryptWithKey(keyId, encryptedData);

// Sign data in TEE
const signature = await r3e.tee.sign("data to sign");

// Verify signature in TEE
const isValid = await r3e.tee.verify("data to sign", signature);
```

## Balance Management API

The Balance Management Service provides functions for managing user balances.

```javascript
// Get account balance
const balance = await r3e.balance.getBalance();

// Get transaction history
const history = await r3e.balance.getTransactionHistory();

// Get transaction details
const transaction = await r3e.balance.getTransaction("transaction-id");

// Withdraw funds
const withdrawal = await r3e.balance.withdraw({
  amount: 100,
  destination: "neo1abc123def456"
});
```

## Configuration

The R3E FaaS platform can be configured using environment variables or configuration files.

### Environment Variables

- `R3E_FAAS__GENERAL__ENVIRONMENT`: Environment (development, production)
- `R3E_FAAS__GENERAL__LOG_LEVEL`: Log level (debug, info, warn, error)
- `R3E_FAAS__API__PORT`: API server port
- `R3E_FAAS__API__HOST`: API server host
- `R3E_FAAS__STORAGE__TYPE`: Storage type (memory, rocksdb)
- `R3E_FAAS__STORAGE__PATH`: Storage path for RocksDB
- `R3E_FAAS__NEO__RPC_URL`: Neo N3 RPC URL
- `R3E_FAAS__NEO__NETWORK`: Neo N3 network (mainnet, testnet)
- `R3E_FAAS__ETHEREUM__RPC_URL`: Ethereum RPC URL
- `R3E_FAAS__ETHEREUM__NETWORK`: Ethereum network (mainnet, goerli, sepolia)

### Configuration File

Configuration can also be provided in a YAML file:

```yaml
general:
  environment: production
  log_level: info

api:
  port: 8080
  host: 0.0.0.0

storage:
  type: rocksdb
  path: /data/rocksdb

neo:
  rpc_url: https://rpc.neo.org
  network: mainnet

ethereum:
  rpc_url: https://mainnet.infura.io/v3/your-api-key
  network: mainnet
```

## Error Handling

All API functions return promises that may be rejected with errors. It's recommended to use try/catch blocks to handle errors:

```javascript
try {
  const price = await r3e.oracle.getPrice("NEO/USD");
  console.log(`NEO price: ${price}`);
} catch (error) {
  console.error(`Error getting NEO price: ${error.message}`);
}
```

Common error types:

- `ValidationError`: Invalid input parameters
- `AuthenticationError`: Authentication failed
- `AuthorizationError`: Insufficient permissions
- `ResourceNotFoundError`: Requested resource not found
- `RateLimitError`: Rate limit exceeded
- `BlockchainError`: Blockchain-related error
- `ServiceError`: Internal service error
- `TimeoutError`: Operation timed out
- `NetworkError`: Network-related error
