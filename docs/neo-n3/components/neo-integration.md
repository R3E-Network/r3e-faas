# Neo N3 Blockchain Integration

This document provides detailed information about the Neo N3 blockchain integration in the Neo N3 FaaS platform.

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [NeoRust SDK Integration](#neorust-sdk-integration)
4. [Event Sources](#event-sources)
5. [Event Triggers](#event-triggers)
6. [Smart Contract Interaction](#smart-contract-interaction)
7. [Transaction Building](#transaction-building)
8. [Wallet Management](#wallet-management)
9. [RPC Client](#rpc-client)
10. [Configuration](#configuration)
11. [Security Considerations](#security-considerations)
12. [Best Practices](#best-practices)

## Overview

The Neo N3 blockchain integration is a core component of the Neo N3 FaaS platform. It provides connectivity to the Neo N3 blockchain network, allowing functions to interact with the blockchain, monitor blockchain events, and execute transactions.

## Architecture

The Neo N3 blockchain integration follows a modular architecture with several key components:

```
                                  +-------------------+
                                  |                   |
                                  |  Neo N3 Blockchain|
                                  |                   |
                                  +--------+----------+
                                           |
                                           | Events
                                           v
+----------------+  Triggers  +------------+-----------+  Tasks  +----------------+
|                |<-----------|                        |-------->|                |
| API Service    |            |     Event System       |         | Scheduler      |
|                |----------->|                        |<--------|                |
+----------------+  Requests  +------------+-----------+  Results+----------------+
                                   |
                                   | Events
                                   v
                      +------------+-----------+
                      |                        |
                      |   Neo Event Source     |
                      |                        |
                      +------------+-----------+
                                   |
                                   | RPC
                                   v
                      +------------+-----------+
                      |                        |
                      |   NeoRust SDK Client   |
                      |                        |
                      +------------------------+
```

### Components

- **Neo N3 Blockchain**: The Neo N3 blockchain network that the platform interacts with.
- **Event System**: Monitors and processes events from the Neo N3 blockchain.
- **Neo Event Source**: Connects to the Neo N3 blockchain and detects events.
- **NeoRust SDK Client**: Provides the API for interacting with the Neo N3 blockchain.
- **API Service**: Provides REST and GraphQL APIs for platform management.
- **Scheduler**: Distributes function execution tasks to worker nodes.

## NeoRust SDK Integration

The Neo N3 FaaS platform integrates with the NeoRust SDK to interact with the Neo N3 blockchain. The NeoRust SDK provides a comprehensive set of tools for working with the Neo N3 blockchain, including wallet management, transaction building, smart contract interaction, and RPC client capabilities.

### Integration Details

The NeoRust SDK is integrated into the platform through the `r3e-event` crate, which provides the event system for the platform. The `neo.rs` module in the `r3e-event/src/source` directory implements the Neo event source, which uses the NeoRust SDK to connect to the Neo N3 blockchain and detect events.

```rust
// r3e-event/src/source/neo.rs
use neo_rust::prelude::*;

pub struct NeoTaskSource {
    client: NeoClient,
    // ...
}

impl NeoTaskSource {
    pub fn new(config: NeoConfig) -> Self {
        let client = NeoClient::new(&config.rpc_url);
        // ...
    }
    
    pub async fn get_latest_block(&self) -> Result<Block, NeoError> {
        self.client.get_block_count().await
    }
    
    // ...
}
```

## Event Sources

The Neo N3 blockchain integration provides event sources for monitoring blockchain events. These event sources are implemented in the `r3e-event/src/source/neo.rs` file and are registered with the event system in the `r3e-event/src/source/mod.rs` file.

### Block Events

Block events are triggered when a new block is added to the blockchain. The Neo event source monitors the blockchain for new blocks and triggers functions based on these events.

```rust
// r3e-event/src/source/neo.rs
impl NeoTaskSource {
    // ...
    
    pub async fn monitor_blocks(&self) -> Result<Vec<Task>, NeoError> {
        let latest_block = self.get_latest_block().await?;
        let tasks = self.create_block_tasks(latest_block);
        Ok(tasks)
    }
    
    fn create_block_tasks(&self, block: Block) -> Vec<Task> {
        // Create tasks for functions triggered by block events
        // ...
    }
    
    // ...
}
```

### Transaction Events

Transaction events are triggered when a new transaction is added to the blockchain. The Neo event source monitors the blockchain for new transactions and triggers functions based on these events.

```rust
// r3e-event/src/source/neo.rs
impl NeoTaskSource {
    // ...
    
    pub async fn monitor_transactions(&self) -> Result<Vec<Task>, NeoError> {
        let latest_block = self.get_latest_block().await?;
        let transactions = self.client.get_block_transactions(latest_block.hash).await?;
        let tasks = self.create_transaction_tasks(transactions);
        Ok(tasks)
    }
    
    fn create_transaction_tasks(&self, transactions: Vec<Transaction>) -> Vec<Task> {
        // Create tasks for functions triggered by transaction events
        // ...
    }
    
    // ...
}
```

### Smart Contract Events

Smart contract events are triggered when a smart contract emits an event. The Neo event source monitors the blockchain for smart contract events and triggers functions based on these events.

```rust
// r3e-event/src/source/neo.rs
impl NeoTaskSource {
    // ...
    
    pub async fn monitor_contract_events(&self) -> Result<Vec<Task>, NeoError> {
        let latest_block = self.get_latest_block().await?;
        let events = self.client.get_block_events(latest_block.hash).await?;
        let tasks = self.create_contract_event_tasks(events);
        Ok(tasks)
    }
    
    fn create_contract_event_tasks(&self, events: Vec<ContractEvent>) -> Vec<Task> {
        // Create tasks for functions triggered by contract events
        // ...
    }
    
    // ...
}
```

## Event Triggers

The Neo N3 blockchain integration provides event triggers for functions. These triggers allow functions to be executed in response to blockchain events.

### Block Triggers

Block triggers allow functions to be executed when a new block is added to the blockchain. Functions can be triggered on every block or based on specific block criteria.

```javascript
// Example function with a block trigger
export default async function(event, context) {
  const blockHeight = event.block.height;
  const blockHash = event.block.hash;
  
  console.log(`New block: ${blockHeight} (${blockHash})`);
  
  // Process the block
  // ...
  
  return {
    blockHeight,
    blockHash
  };
}
```

### Transaction Triggers

Transaction triggers allow functions to be executed when a new transaction is added to the blockchain. Functions can be triggered on every transaction or based on specific transaction criteria.

```javascript
// Example function with a transaction trigger
export default async function(event, context) {
  const txHash = event.transaction.hash;
  const sender = event.transaction.sender;
  const recipient = event.transaction.recipient;
  const amount = event.transaction.amount;
  
  console.log(`New transaction: ${txHash} (${sender} -> ${recipient}: ${amount})`);
  
  // Process the transaction
  // ...
  
  return {
    txHash,
    sender,
    recipient,
    amount
  };
}
```

### Smart Contract Event Triggers

Smart contract event triggers allow functions to be executed when a smart contract emits an event. Functions can be triggered on every contract event or based on specific contract and event criteria.

```javascript
// Example function with a smart contract event trigger
export default async function(event, context) {
  const contractHash = event.contract.hash;
  const eventName = event.name;
  const eventArgs = event.args;
  
  console.log(`Contract event: ${contractHash} - ${eventName}`);
  
  // Process the contract event
  // ...
  
  return {
    contractHash,
    eventName,
    eventArgs
  };
}
```

## Smart Contract Interaction

The Neo N3 blockchain integration provides APIs for interacting with smart contracts on the Neo N3 blockchain.

### Contract Invocation

Functions can invoke smart contract methods using the Neo N3 API provided by the platform.

```javascript
// Example function that invokes a smart contract method
export default async function(event, context) {
  // Get contract instance
  const contract = await context.neo.getContract("0x1234567890abcdef");
  
  // Call contract method
  const result = await contract.call("transfer", [
    "NXV7ZhHiyM1aHXwvUBCta7dGNP1tHwfoQG",
    "NZHf1NJvz1tvELGLWZjhpb3NqZJFFUYpxT",
    100
  ]);
  
  console.log(`Transfer result: ${result}`);
  
  return {
    result
  };
}
```

### Contract Deployment

Functions can deploy smart contracts to the Neo N3 blockchain using the Neo N3 API provided by the platform.

```javascript
// Example function that deploys a smart contract
export default async function(event, context) {
  // Get wallet
  const wallet = await context.neo.getWallet("my-wallet");
  
  // Deploy contract
  const contractHash = await wallet.deployContract(
    "0x1234567890abcdef", // Contract script
    {
      name: "MyContract",
      version: "1.0.0",
      author: "Neo N3 FaaS",
      email: "faas@neo.org",
      description: "My Neo N3 smart contract"
    }
  );
  
  console.log(`Contract deployed: ${contractHash}`);
  
  return {
    contractHash
  };
}
```

## Transaction Building

The Neo N3 blockchain integration provides APIs for building and sending transactions on the Neo N3 blockchain.

### Transaction Creation

Functions can create transactions using the Neo N3 API provided by the platform.

```javascript
// Example function that creates a transaction
export default async function(event, context) {
  // Get wallet
  const wallet = await context.neo.getWallet("my-wallet");
  
  // Create transaction
  const tx = await wallet.createTransaction({
    type: "transfer",
    asset: "NEO",
    from: wallet.address,
    to: "NZHf1NJvz1tvELGLWZjhpb3NqZJFFUYpxT",
    amount: 10
  });
  
  // Sign transaction
  const signedTx = await wallet.signTransaction(tx);
  
  // Send transaction
  const txHash = await context.neo.sendTransaction(signedTx);
  
  console.log(`Transaction sent: ${txHash}`);
  
  return {
    txHash
  };
}
```

### Transaction Signing

Functions can sign transactions using the Neo N3 API provided by the platform.

```javascript
// Example function that signs a transaction
export default async function(event, context) {
  // Get wallet
  const wallet = await context.neo.getWallet("my-wallet");
  
  // Create transaction
  const tx = await wallet.createTransaction({
    type: "transfer",
    asset: "NEO",
    from: wallet.address,
    to: "NZHf1NJvz1tvELGLWZjhpb3NqZJFFUYpxT",
    amount: 10
  });
  
  // Sign transaction
  const signedTx = await wallet.signTransaction(tx);
  
  console.log(`Transaction signed: ${signedTx.hash}`);
  
  return {
    signedTx
  };
}
```

## Wallet Management

The Neo N3 blockchain integration provides APIs for managing wallets on the Neo N3 blockchain.

### Wallet Creation

Functions can create wallets using the Neo N3 API provided by the platform.

```javascript
// Example function that creates a wallet
export default async function(event, context) {
  // Create wallet
  const wallet = await context.neo.createWallet({
    name: "my-wallet",
    password: "my-password"
  });
  
  console.log(`Wallet created: ${wallet.address}`);
  
  return {
    address: wallet.address,
    publicKey: wallet.publicKey
  };
}
```

### Wallet Import

Functions can import wallets using the Neo N3 API provided by the platform.

```javascript
// Example function that imports a wallet
export default async function(event, context) {
  // Import wallet from private key
  const wallet = await context.neo.importWallet({
    name: "my-wallet",
    privateKey: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
    password: "my-password"
  });
  
  console.log(`Wallet imported: ${wallet.address}`);
  
  return {
    address: wallet.address,
    publicKey: wallet.publicKey
  };
}
```

## RPC Client

The Neo N3 blockchain integration provides an RPC client for interacting with the Neo N3 blockchain.

### RPC Methods

Functions can call RPC methods using the Neo N3 API provided by the platform.

```javascript
// Example function that calls RPC methods
export default async function(event, context) {
  // Get block count
  const blockCount = await context.neo.rpc("getblockcount", []);
  
  // Get block by height
  const block = await context.neo.rpc("getblock", [blockCount - 1, 1]);
  
  // Get transaction by hash
  const tx = await context.neo.rpc("getrawtransaction", [block.tx[0], 1]);
  
  console.log(`Block count: ${blockCount}`);
  console.log(`Block: ${block.hash}`);
  console.log(`Transaction: ${tx.txid}`);
  
  return {
    blockCount,
    block,
    tx
  };
}
```

## Configuration

The Neo N3 blockchain integration can be configured using the platform's configuration system.

### RPC Endpoint

The RPC endpoint for the Neo N3 blockchain can be configured using the `neo.rpc.endpoint` configuration key.

```bash
r3e-faas-cli config set --key neo.rpc.endpoint --value https://neo-rpc.example.com
```

### Network

The Neo N3 network (mainnet, testnet, or private net) can be configured using the `neo.network` configuration key.

```bash
r3e-faas-cli config set --key neo.network --value mainnet
```

### Wallet

The default wallet for the Neo N3 blockchain can be configured using the `neo.wallet` configuration key.

```bash
r3e-faas-cli config set --key neo.wallet --value my-wallet
```

## Security Considerations

### Private Key Management

Private keys should be stored securely and never exposed in function code or logs. The platform provides secure key management through the wallet API.

```javascript
// Example function that uses a wallet securely
export default async function(event, context) {
  // Get wallet
  const wallet = await context.neo.getWallet("my-wallet");
  
  // Use wallet to sign transaction
  const signedTx = await wallet.signTransaction(tx);
  
  // Never log or return private keys
  // console.log(`Private key: ${wallet.privateKey}`); // DON'T DO THIS
  
  return {
    signedTx
  };
}
```

### RPC Authentication

RPC endpoints may require authentication. The platform provides secure authentication through the configuration system.

```bash
r3e-faas-cli config set --key neo.rpc.auth.username --value my-username
r3e-faas-cli config set --key neo.rpc.auth.password --value my-password
```

### Rate Limiting

RPC endpoints may impose rate limits. The platform provides rate limiting to prevent exceeding these limits.

```javascript
// Example function that uses rate limiting
export default async function(event, context) {
  // Use rate limiter
  const rateLimiter = new context.neo.RateLimiter({
    maxRequests: 10,
    perMinute: 1
  });
  
  if (await rateLimiter.canMakeRequest()) {
    const blockCount = await context.neo.rpc("getblockcount", []);
    return { blockCount };
  } else {
    return { error: "Rate limit exceeded" };
  }
}
```

## Best Practices

### Error Handling

Functions should handle errors gracefully and provide meaningful error messages.

```javascript
// Example function with error handling
export default async function(event, context) {
  try {
    const blockCount = await context.neo.rpc("getblockcount", []);
    return { blockCount };
  } catch (error) {
    console.error(`Error: ${error.message}`);
    return { error: error.message };
  }
}
```

### Caching

Functions should use caching to reduce the number of RPC calls and improve performance.

```javascript
// Example function with caching
export default async function(event, context) {
  const cacheKey = "blockcount";
  let blockCount = await context.cache.get(cacheKey);
  
  if (!blockCount) {
    blockCount = await context.neo.rpc("getblockcount", []);
    await context.cache.set(cacheKey, blockCount, { ttl: 60 }); // Cache for 60 seconds
  }
  
  return { blockCount };
}
```

### Retry Logic

Functions should implement retry logic for RPC calls to handle temporary failures.

```javascript
// Example function with retry logic
export default async function(event, context) {
  const maxRetries = 3;
  let retries = 0;
  
  while (retries < maxRetries) {
    try {
      const blockCount = await context.neo.rpc("getblockcount", []);
      return { blockCount };
    } catch (error) {
      retries++;
      if (retries >= maxRetries) throw error;
      await new Promise(resolve => setTimeout(resolve, 1000 * Math.pow(2, retries)));
    }
  }
}
```

### Transaction Verification

Functions should verify transactions before sending them to the blockchain.

```javascript
// Example function with transaction verification
export default async function(event, context) {
  // Create transaction
  const tx = await wallet.createTransaction({
    type: "transfer",
    asset: "NEO",
    from: wallet.address,
    to: "NZHf1NJvz1tvELGLWZjhpb3NqZJFFUYpxT",
    amount: 10
  });
  
  // Verify transaction
  const isValid = await context.neo.verifyTransaction(tx);
  
  if (!isValid) {
    return { error: "Invalid transaction" };
  }
  
  // Sign and send transaction
  const signedTx = await wallet.signTransaction(tx);
  const txHash = await context.neo.sendTransaction(signedTx);
  
  return { txHash };
}
```

### Resource Cleanup

Functions should clean up resources after use to prevent memory leaks.

```javascript
// Example function with resource cleanup
export default async function(event, context) {
  let client = null;
  
  try {
    client = await context.neo.createClient();
    const blockCount = await client.getBlockCount();
    return { blockCount };
  } catch (error) {
    console.error(`Error: ${error.message}`);
    return { error: error.message };
  } finally {
    if (client) {
      await client.close();
    }
  }
}
```
