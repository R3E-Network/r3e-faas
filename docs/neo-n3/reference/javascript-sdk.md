# JavaScript SDK Reference for Neo N3 FaaS Platform

This reference provides detailed information about the JavaScript SDK for the Neo N3 FaaS platform.

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Configuration](#configuration)
4. [Neo N3 API](#neo-n3-api)
5. [Oracle API](#oracle-api)
6. [TEE API](#tee-api)
7. [Event API](#event-api)
8. [Storage API](#storage-api)
9. [Utility API](#utility-api)
10. [Error Handling](#error-handling)
11. [Examples](#examples)

## Introduction

The Neo N3 FaaS JavaScript SDK provides a set of APIs for interacting with the Neo N3 blockchain, Oracle services, TEE services, and other platform features from JavaScript functions. The SDK is automatically available in the function execution environment and does not need to be installed separately.

## Installation

The SDK is automatically available in the function execution environment via the `context` object. However, for local development and testing, you can install the SDK using npm:

```bash
npm install r3e-faas-sdk
```

## Configuration

The SDK is automatically configured in the function execution environment. However, for local development and testing, you can configure the SDK as follows:

```javascript
const { R3EClient } = require('r3e-faas-sdk');

// Create a client with default configuration
const client = new R3EClient();

// Create a client with custom configuration
const client = new R3EClient({
  apiUrl: 'https://faas.example.com',
  apiKey: 'your-api-key',
  network: 'mainnet',
  timeout: 30000
});
```

### Configuration Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `apiUrl` | String | No | The API URL for the platform |
| `apiKey` | String | No | The API key for authentication |
| `network` | String | No | The network (e.g., `mainnet`, `testnet`) |
| `timeout` | Number | No | The timeout in milliseconds |

## Neo N3 API

The Neo N3 API provides methods for interacting with the Neo N3 blockchain.

### Get Current Block Height

```javascript
// Using the context object in a function
const blockHeight = await context.neo.getCurrentBlockHeight();

// Using the client in local development
const blockHeight = await client.neo.getCurrentBlockHeight();
```

### Get Block by Height

```javascript
// Using the context object in a function
const block = await context.neo.getBlock(blockHeight);

// Using the client in local development
const block = await client.neo.getBlock(blockHeight);
```

### Get Transaction by Hash

```javascript
// Using the context object in a function
const tx = await context.neo.getTransaction('0x1234567890abcdef');

// Using the client in local development
const tx = await client.neo.getTransaction('0x1234567890abcdef');
```

### Get Contract by Hash

```javascript
// Using the context object in a function
const contract = await context.neo.getContract('0x1234567890abcdef');

// Using the client in local development
const contract = await client.neo.getContract('0x1234567890abcdef');
```

### Call Contract Method

```javascript
// Using the context object in a function
const result = await context.neo.callContract('0x1234567890abcdef', 'balanceOf', ['NXV7ZhHiyM1aHXwvUBCta7dGNP1tHwfoQG']);

// Using the client in local development
const result = await client.neo.callContract('0x1234567890abcdef', 'balanceOf', ['NXV7ZhHiyM1aHXwvUBCta7dGNP1tHwfoQG']);
```

### Create and Send Transaction

```javascript
// Using the context object in a function
const tx = await context.neo.createTransaction({
  from: 'NXV7ZhHiyM1aHXwvUBCta7dGNP1tHwfoQG',
  to: 'NXV7ZhHiyM1aHXwvUBCta7dGNP1tHwfoQG',
  asset: 'NEO',
  amount: 1
});
const txHash = await context.neo.sendTransaction(tx);

// Using the client in local development
const tx = await client.neo.createTransaction({
  from: 'NXV7ZhHiyM1aHXwvUBCta7dGNP1tHwfoQG',
  to: 'NXV7ZhHiyM1aHXwvUBCta7dGNP1tHwfoQG',
  asset: 'NEO',
  amount: 1
});
const txHash = await client.neo.sendTransaction(tx);
```

### Get Account Balance

```javascript
// Using the context object in a function
const balance = await context.neo.getBalance('NXV7ZhHiyM1aHXwvUBCta7dGNP1tHwfoQG', 'NEO');

// Using the client in local development
const balance = await client.neo.getBalance('NXV7ZhHiyM1aHXwvUBCta7dGNP1tHwfoQG', 'NEO');
```

### Get Account Balances

```javascript
// Using the context object in a function
const balances = await context.neo.getBalances('NXV7ZhHiyM1aHXwvUBCta7dGNP1tHwfoQG');

// Using the client in local development
const balances = await client.neo.getBalances('NXV7ZhHiyM1aHXwvUBCta7dGNP1tHwfoQG');
```

### Create Wallet

```javascript
// Using the context object in a function
const wallet = await context.neo.createWallet();

// Using the client in local development
const wallet = await client.neo.createWallet();
```

### Open Wallet

```javascript
// Using the context object in a function
const wallet = await context.neo.openWallet('/path/to/wallet.json', 'password');

// Using the client in local development
const wallet = await client.neo.openWallet('/path/to/wallet.json', 'password');
```

### Sign Message

```javascript
// Using the context object in a function
const signature = await context.neo.signMessage('Hello, Neo N3!', wallet);

// Using the client in local development
const signature = await client.neo.signMessage('Hello, Neo N3!', wallet);
```

### Verify Signature

```javascript
// Using the context object in a function
const isValid = await context.neo.verifySignature('Hello, Neo N3!', signature, wallet.publicKey);

// Using the client in local development
const isValid = await client.neo.verifySignature('Hello, Neo N3!', signature, wallet.publicKey);
```

## Oracle API

The Oracle API provides methods for accessing Oracle services.

### Get Price

```javascript
// Using the context object in a function
const price = await context.oracle.getPrice('NEO', 'USD');

// Using the client in local development
const price = await client.oracle.getPrice('NEO', 'USD');
```

### Get Random Number

```javascript
// Using the context object in a function
const randomNumber = await context.oracle.getRandomNumber(1, 100);

// Using the client in local development
const randomNumber = await client.oracle.getRandomNumber(1, 100);
```

### Get Weather

```javascript
// Using the context object in a function
const weather = await context.oracle.getWeather('New York');

// Using the client in local development
const weather = await client.oracle.getWeather('New York');
```

### Get Sports Results

```javascript
// Using the context object in a function
const results = await context.oracle.getSportsResults('NBA');

// Using the client in local development
const results = await client.oracle.getSportsResults('NBA');
```

### Get Financial Data

```javascript
// Using the context object in a function
const data = await context.oracle.getFinancialData('AAPL');

// Using the client in local development
const data = await client.oracle.getFinancialData('AAPL');
```

## TEE API

The TEE API provides methods for accessing TEE services.

### Execute in TEE

```javascript
// Using the context object in a function
const result = await context.tee.execute(async (secureContext) => {
  // Generate key pair
  const keyPair = await secureContext.crypto.generateKeyPair();
  
  // Return public key
  return { publicKey: keyPair.publicKey };
});

// Using the client in local development
const result = await client.tee.execute(async (secureContext) => {
  // Generate key pair
  const keyPair = await secureContext.crypto.generateKeyPair();
  
  // Return public key
  return { publicKey: keyPair.publicKey };
});
```

### Generate Key Pair

```javascript
// Using the context object in a function
const keyPair = await context.tee.generateKeyPair();

// Using the client in local development
const keyPair = await client.tee.generateKeyPair();
```

### Encrypt Data

```javascript
// Using the context object in a function
const encryptedData = await context.tee.encrypt('Hello, Neo N3!', keyPair.publicKey);

// Using the client in local development
const encryptedData = await client.tee.encrypt('Hello, Neo N3!', keyPair.publicKey);
```

### Decrypt Data

```javascript
// Using the context object in a function
const decryptedData = await context.tee.decrypt(encryptedData, keyPair.privateKey);

// Using the client in local development
const decryptedData = await client.tee.decrypt(encryptedData, keyPair.privateKey);
```

### Sign Data

```javascript
// Using the context object in a function
const signature = await context.tee.sign('Hello, Neo N3!', keyPair.privateKey);

// Using the client in local development
const signature = await client.tee.sign('Hello, Neo N3!', keyPair.privateKey);
```

### Verify Signature

```javascript
// Using the context object in a function
const isValid = await context.tee.verify('Hello, Neo N3!', signature, keyPair.publicKey);

// Using the client in local development
const isValid = await client.tee.verify('Hello, Neo N3!', signature, keyPair.publicKey);
```

## Event API

The Event API provides methods for working with events.

### Subscribe to Events

```javascript
// Using the context object in a function
const subscription = await context.event.subscribe('neo:NewBlock', (event) => {
  console.log('New block:', event.data.blockHeight);
});

// Using the client in local development
const subscription = await client.event.subscribe('neo:NewBlock', (event) => {
  console.log('New block:', event.data.blockHeight);
});
```

### Unsubscribe from Events

```javascript
// Using the context object in a function
await context.event.unsubscribe(subscription);

// Using the client in local development
await client.event.unsubscribe(subscription);
```

### Publish Event

```javascript
// Using the context object in a function
await context.event.publish('app:UserCreated', { userId: '123', name: 'John Doe' });

// Using the client in local development
await client.event.publish('app:UserCreated', { userId: '123', name: 'John Doe' });
```

### Get Event History

```javascript
// Using the context object in a function
const events = await context.event.getHistory('neo:NewBlock', { limit: 10 });

// Using the client in local development
const events = await client.event.getHistory('neo:NewBlock', { limit: 10 });
```

## Storage API

The Storage API provides methods for working with storage.

### Set Item

```javascript
// Using the context object in a function
await context.storage.set('user:123', { name: 'John Doe', email: 'john.doe@example.com' });

// Using the client in local development
await client.storage.set('user:123', { name: 'John Doe', email: 'john.doe@example.com' });
```

### Get Item

```javascript
// Using the context object in a function
const user = await context.storage.get('user:123');

// Using the client in local development
const user = await client.storage.get('user:123');
```

### Delete Item

```javascript
// Using the context object in a function
await context.storage.delete('user:123');

// Using the client in local development
await client.storage.delete('user:123');
```

### List Items

```javascript
// Using the context object in a function
const users = await context.storage.list('user:');

// Using the client in local development
const users = await client.storage.list('user:');
```

## Utility API

The Utility API provides utility methods.

### Generate UUID

```javascript
// Using the context object in a function
const uuid = context.util.uuid();

// Using the client in local development
const uuid = client.util.uuid();
```

### Hash Data

```javascript
// Using the context object in a function
const hash = context.util.hash('Hello, Neo N3!', 'sha256');

// Using the client in local development
const hash = client.util.hash('Hello, Neo N3!', 'sha256');
```

### Encode Base64

```javascript
// Using the context object in a function
const encoded = context.util.base64Encode('Hello, Neo N3!');

// Using the client in local development
const encoded = client.util.base64Encode('Hello, Neo N3!');
```

### Decode Base64

```javascript
// Using the context object in a function
const decoded = context.util.base64Decode(encoded);

// Using the client in local development
const decoded = client.util.base64Decode(encoded);
```

### Parse JSON

```javascript
// Using the context object in a function
const obj = context.util.parseJson('{"name": "John Doe"}');

// Using the client in local development
const obj = client.util.parseJson('{"name": "John Doe"}');
```

### Stringify JSON

```javascript
// Using the context object in a function
const json = context.util.stringifyJson({ name: 'John Doe' });

// Using the client in local development
const json = client.util.stringifyJson({ name: 'John Doe' });
```

## Error Handling

The SDK provides a set of error classes for handling errors.

```javascript
const { R3EError, NeoError, OracleError, TEEError } = require('r3e-faas-sdk');

try {
  // Code that might throw an error
  const balance = await client.neo.getBalance('invalid-address', 'NEO');
} catch (error) {
  if (error instanceof NeoError) {
    console.error('Neo error:', error.message);
  } else if (error instanceof OracleError) {
    console.error('Oracle error:', error.message);
  } else if (error instanceof TEEError) {
    console.error('TEE error:', error.message);
  } else if (error instanceof R3EError) {
    console.error('R3E error:', error.message);
  } else {
    console.error('Unknown error:', error);
  }
}
```

### Error Classes

| Class | Description |
|-------|-------------|
| `R3EError` | Base error class for all SDK errors |
| `NeoError` | Error class for Neo N3 API errors |
| `OracleError` | Error class for Oracle API errors |
| `TEEError` | Error class for TEE API errors |
| `EventError` | Error class for Event API errors |
| `StorageError` | Error class for Storage API errors |
| `UtilError` | Error class for Utility API errors |

## Examples

### Get Neo N3 Blockchain Information

```javascript
export default async function(event, context) {
  try {
    // Get current block height
    const blockHeight = await context.neo.getCurrentBlockHeight();
    
    // Get latest block
    const block = await context.neo.getBlock(blockHeight);
    
    // Get NEO price in USD
    const neoPrice = await context.oracle.getPrice('NEO', 'USD');
    
    // Get GAS price in USD
    const gasPrice = await context.oracle.getPrice('GAS', 'USD');
    
    return {
      blockHeight,
      blockTime: block.time,
      blockHash: block.hash,
      neoPrice,
      gasPrice,
      timestamp: new Date().toISOString()
    };
  } catch (error) {
    console.error('Error:', error);
    throw error;
  }
}
```

### Transfer NEO

```javascript
export default async function(event, context) {
  try {
    const { from, to, amount } = event.data;
    
    // Create transaction
    const tx = await context.neo.createTransaction({
      from,
      to,
      asset: 'NEO',
      amount: parseFloat(amount)
    });
    
    // Send transaction
    const txHash = await context.neo.sendTransaction(tx);
    
    return {
      txHash,
      from,
      to,
      asset: 'NEO',
      amount: parseFloat(amount),
      timestamp: new Date().toISOString()
    };
  } catch (error) {
    console.error('Error:', error);
    throw error;
  }
}
```

### Secure Key Generation

```javascript
export default async function(event, context) {
  try {
    // Execute in TEE
    const result = await context.tee.execute(async (secureContext) => {
      // Generate key pair
      const keyPair = await secureContext.crypto.generateKeyPair();
      
      // Store private key securely
      await secureContext.storage.set('privateKey', keyPair.privateKey);
      
      // Return public key
      return { publicKey: keyPair.publicKey };
    });
    
    return {
      publicKey: result.publicKey,
      timestamp: new Date().toISOString()
    };
  } catch (error) {
    console.error('Error:', error);
    throw error;
  }
}
```

### Event Subscription

```javascript
export default async function(event, context) {
  try {
    // Subscribe to Neo N3 new block events
    const subscription = await context.event.subscribe('neo:NewBlock', async (event) => {
      // Get block
      const block = await context.neo.getBlock(event.data.blockHeight);
      
      // Store block information
      await context.storage.set(`block:${event.data.blockHeight}`, {
        height: event.data.blockHeight,
        hash: block.hash,
        time: block.time,
        transactions: block.transactions.length
      });
      
      console.log(`Processed block ${event.data.blockHeight}`);
    });
    
    return {
      subscription: subscription.id,
      message: 'Subscribed to Neo N3 new block events',
      timestamp: new Date().toISOString()
    };
  } catch (error) {
    console.error('Error:', error);
    throw error;
  }
}
```

For more information, see the [API Reference](../api-reference.md) and [Architecture](../architecture.md) documents.
