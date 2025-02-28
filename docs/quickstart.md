# R3E FaaS Quick Start Guide

This guide will help you get started with the R3E FaaS platform quickly. Follow these steps to deploy your first JavaScript function and interact with the platform's built-in services.

## Prerequisites

Before you begin, make sure you have the following:

- Node.js 16+ installed
- Rust 1.60+ installed
- Docker (optional, for containerized deployment)
- A Neo N3 wallet with some GAS tokens (for blockchain interactions)

## Installation

1. Clone the repository:

```bash
git clone https://github.com/R3E-Network/r3e-faas.git
cd r3e-faas
```

2. Build the project:

```bash
cargo build
```

3. Start the development environment:

```bash
# Using Docker Compose
docker-compose -f docker-compose.dev.yml up

# Or run locally
cargo run --bin r3e-api
```

## Creating Your First Function

1. Create a new JavaScript file in the `functions` directory:

```bash
mkdir -p functions
touch functions/hello-world.js
```

2. Open the file and add the following code:

```javascript
// functions/hello-world.js
export default function(event) {
  console.log("Hello, R3E FaaS!");
  
  return {
    message: "Hello, R3E FaaS!",
    timestamp: new Date().toISOString(),
    event: event
  };
}
```

3. Deploy the function using the CLI:

```bash
cargo run --bin r3e -- deploy functions/hello-world.js --name hello-world
```

## Invoking Your Function

You can invoke your function using the CLI or the HTTP API:

### Using the CLI

```bash
cargo run --bin r3e -- invoke hello-world --data '{"test": "data"}'
```

### Using the HTTP API

```bash
curl -X POST http://localhost:8080/api/v1/functions/hello-world/invoke \
  -H "Content-Type: application/json" \
  -d '{"test": "data"}'
```

## Using Built-in Services

Let's create a more advanced function that uses some of the platform's built-in services:

```javascript
// functions/crypto-price-alert.js
export default async function(event) {
  // Get the current NEO price
  const neoPrice = await r3e.oracle.getPrice("NEO/USD");
  console.log(`Current NEO price: $${neoPrice}`);
  
  // Store the price in our function's storage
  await r3e.storage.set("last_neo_price", neoPrice);
  
  // Get the previous price (if any)
  const lastPrice = await r3e.storage.get("last_neo_price") || neoPrice;
  
  // Calculate the price change percentage
  const changePercent = ((neoPrice - lastPrice) / lastPrice) * 100;
  
  // Create a trigger for future price changes
  if (!await r3e.trigger.exists("neo-price-alert")) {
    await r3e.trigger.create({
      type: "market",
      name: "NEO Price Alert",
      description: "Alert when NEO price changes by 5%",
      assetPair: "NEO/USD",
      condition: "change",
      threshold: 5,
      direction: "any"
    });
  }
  
  return {
    asset: "NEO/USD",
    current_price: neoPrice,
    previous_price: lastPrice,
    change_percent: changePercent.toFixed(2) + "%",
    timestamp: new Date().toISOString()
  };
}
```

Deploy and invoke this function:

```bash
cargo run --bin r3e -- deploy functions/crypto-price-alert.js --name crypto-price-alert
cargo run --bin r3e -- invoke crypto-price-alert
```

## Setting Up Automatic Smart Contracts

Create a function that sets up an automatic smart contract execution:

```javascript
// functions/auto-contract.js
export default async function(event) {
  // Create an automatic contract that executes when NEO price is above $50
  const contract = await r3e.autoContract.create({
    trigger: {
      type: "price",
      assetPair: "NEO/USD",
      condition: "above",
      threshold: 50
    },
    action: {
      blockchain: "neo",
      contract: "0x1234567890abcdef1234567890abcdef12345678", // Replace with your contract hash
      method: "transfer",
      params: [
        { type: "Hash160", value: "NbnjKGMBJzJ6j5PHeYhjJDaQ5Vy5UYu4Fv" }, // Replace with your address
        { type: "Integer", value: 5 }
      ]
    },
    maxGas: 10,
    priority: 1
  });
  
  return {
    message: "Auto contract created successfully",
    contract_id: contract.id,
    trigger: contract.trigger,
    action: contract.action
  };
}
```

## Using Zero-Knowledge Computing

Create a function that uses the Zero-Knowledge Computing service:

```javascript
// functions/zk-proof.js
export default async function(event) {
  // Compile a ZoKrates circuit for proving knowledge of factors
  const circuitSource = `
    def main(private field a, private field b, field c) -> bool:
      return a * b == c
  `;
  const circuitId = await r3e.zk.compileCircuit(circuitSource, r3e.zk.CircuitType.ZOKRATES, "multiply");

  // Generate keys
  const { provingKeyId, verificationKeyId } = await r3e.zk.generateKeys(circuitId);

  // Generate a proof (proving we know factors of 6 without revealing them)
  const publicInputs = ["6"]; // The public value c = 6
  const privateInputs = ["2", "3"]; // The private values a = 2, b = 3
  const proofId = await r3e.zk.generateProof(circuitId, provingKeyId, publicInputs, privateInputs);

  // Verify the proof
  const isValid = await r3e.zk.verifyProof(proofId, verificationKeyId, publicInputs);
  
  return {
    message: "Zero-knowledge proof demonstration",
    circuit_id: circuitId,
    proof_id: proofId,
    is_valid: isValid
  };
}
```

## Using Fully Homomorphic Encryption

Create a function that uses the Fully Homomorphic Encryption service:

```javascript
// functions/fhe-compute.js
export default async function(event) {
  // Generate FHE keys
  const keyPairId = await r3e.fhe.generateKeys(r3e.fhe.SchemeType.TFHE);
  const publicKeyId = `${keyPairId}_public`;
  const privateKeyId = `${keyPairId}_private`;

  // Encrypt data
  const ciphertext1Id = await r3e.fhe.encrypt(publicKeyId, "42");
  const ciphertext2Id = await r3e.fhe.encrypt(publicKeyId, "8");

  // Perform homomorphic operations
  const addResultId = await r3e.fhe.add(ciphertext1Id, ciphertext2Id);
  const multiplyResultId = await r3e.fhe.multiply(ciphertext1Id, ciphertext2Id);

  // Decrypt results
  const addResult = await r3e.fhe.decrypt(privateKeyId, addResultId);
  const multiplyResult = await r3e.fhe.decrypt(privateKeyId, multiplyResultId);
  
  return {
    message: "Fully homomorphic encryption demonstration",
    addition_result: addResult,
    multiplication_result: multiplyResult
  };
}
```

## Setting Up Event Triggers

Create a function that sets up event triggers:

```javascript
// functions/event-triggers.js
export default async function(event) {
  // Create a blockchain event trigger
  const blockchainTrigger = await r3e.trigger.create({
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
    schedule: "0 * * * *", // Cron expression
    timezone: "UTC"
  });
  
  // Create a market price trigger
  const priceTrigger = await r3e.trigger.create({
    type: "market",
    name: "NEO Price Trigger",
    description: "Trigger when NEO price changes significantly",
    assetPair: "NEO/USD",
    condition: "change",
    threshold: 5, // 5% change
    direction: "any" // "up", "down", or "any"
  });
  
  return {
    message: "Event triggers created successfully",
    triggers: {
      blockchain: blockchainTrigger.id,
      time: timeTrigger.id,
      price: priceTrigger.id
    }
  };
}
```

## Managing Your Balance

Create a function to manage your balance:

```javascript
// functions/balance-management.js
export default async function(event) {
  // Get your current balance
  const balance = await r3e.balance.getBalance();
  
  // Get transaction history
  const history = await r3e.balance.getTransactionHistory();
  
  // Get the latest transaction
  const latestTransaction = history.length > 0 ? history[0] : null;
  
  return {
    message: "Balance information",
    current_balance: balance,
    transaction_count: history.length,
    latest_transaction: latestTransaction
  };
}
```

## Next Steps

Now that you've created your first functions and explored some of the platform's capabilities, you can:

1. Explore the [API Reference](./api-reference.md) for detailed information on all available services
2. Check out the [Examples](../examples/) directory for more advanced use cases
3. Learn about [Docker Deployment](./docker-production.md) for production environments
4. Explore [Zero-Knowledge Computing](./zk-service.md) and [Fully Homomorphic Encryption](./fhe-service.md) services

For more detailed information, refer to the [Development Guide](./development.md).
