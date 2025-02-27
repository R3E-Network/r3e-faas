# Function Development Guide for Neo N3 FaaS Platform

This guide provides detailed information about developing functions for the Neo N3 FaaS platform.

## Table of Contents

1. [Introduction](#introduction)
2. [Function Basics](#function-basics)
3. [Function Types](#function-types)
4. [Neo N3 Integration](#neo-n3-integration)
5. [Oracle Services](#oracle-services)
6. [TEE Services](#tee-services)
7. [Function Configuration](#function-configuration)
8. [Testing and Debugging](#testing-and-debugging)
9. [Best Practices](#best-practices)

## Introduction

Functions are the core building blocks of the Neo N3 FaaS platform. They are small, single-purpose pieces of code that are executed in response to events. Functions can be triggered by HTTP requests, blockchain events, schedules, or custom events.

## Function Basics

### Function Structure

A function is a JavaScript module that exports a default function. The function takes two parameters: `event` and `context`.

```javascript
export default async function(event, context) {
  // Function code
  return {
    message: "Hello, Neo N3 FaaS!"
  };
}
```

### Event Object

The `event` object contains information about the event that triggered the function. The structure depends on the trigger type (HTTP, blockchain, schedule, etc.).

#### HTTP Event

```javascript
{
  "method": "GET",
  "path": "/hello",
  "query": {
    "name": "Neo"
  },
  "headers": {
    "content-type": "application/json"
  },
  "body": {
    "message": "Hello"
  }
}
```

#### Blockchain Event

```javascript
{
  "type": "block",
  "data": {
    "index": 12345,
    "hash": "0x1234567890abcdef",
    "timestamp": 1609459200000
  }
}
```

#### Schedule Event

```javascript
{
  "type": "schedule",
  "data": {
    "timestamp": 1609459200000,
    "name": "hourly-job"
  }
}
```

### Context Object

The `context` object provides access to platform services and utilities:

- **neo**: Access to Neo N3 blockchain
- **oracle**: Access to oracle services
- **tee**: Access to TEE services
- **storage**: Access to persistent storage
- **log**: Logging utilities
- **env**: Environment variables

```javascript
// Using the context object
export default async function(event, context) {
  // Log information
  context.log.info("Function invoked");
  
  // Access environment variables
  const environment = context.env.NODE_ENV;
  
  // Access Neo N3 blockchain
  const blockHeight = await context.neo.getCurrentBlockHeight();
  
  // Access oracle services
  const neoPrice = await context.oracle.getPrice("NEO", "USD");
  
  // Access TEE services
  const secureResult = await context.tee.execute(async (secureContext) => {
    return { secure: true };
  });
  
  // Access persistent storage
  const value = await context.storage.get("key");
  await context.storage.set("key", "value");
  
  return {
    environment,
    blockHeight,
    neoPrice,
    secureResult,
    value
  };
}
```

## Function Types

### HTTP Functions

HTTP functions are triggered by HTTP requests. They can be used to build RESTful APIs or web applications.

```javascript
export default async function(event, context) {
  // Get name from query parameters or use default
  const name = event.query.name || "World";
  
  // Log request
  context.log.info(`HTTP request received: ${event.method} ${event.path}`);
  
  // Return response
  return {
    statusCode: 200,
    headers: {
      "content-type": "application/json"
    },
    body: {
      message: `Hello, ${name}!`
    }
  };
}
```

### Blockchain Event Functions

Blockchain event functions are triggered by blockchain events such as new blocks, transactions, or smart contract notifications.

```javascript
export default async function(event, context) {
  // Handle different event types
  if (event.type === "block") {
    // New block event
    const blockHeight = event.data.index;
    const blockHash = event.data.hash;
    
    // Log block information
    context.log.info(`New block: ${blockHeight} (${blockHash})`);
    
    // Get block details
    const block = await context.neo.getBlock(blockHeight);
    
    return {
      blockHeight,
      blockHash,
      transactions: block.tx.length
    };
  } else if (event.type === "transaction") {
    // New transaction event
    const txHash = event.data.hash;
    
    // Log transaction information
    context.log.info(`New transaction: ${txHash}`);
    
    // Get transaction details
    const tx = await context.neo.getTransaction(txHash);
    
    return {
      txHash,
      sender: tx.sender,
      size: tx.size
    };
  } else if (event.type === "notification") {
    // Smart contract notification event
    const contractHash = event.data.contract;
    const eventName = event.data.eventName;
    const eventArgs = event.data.eventArgs;
    
    // Log notification information
    context.log.info(`Contract notification: ${contractHash} - ${eventName}`);
    
    return {
      contractHash,
      eventName,
      eventArgs
    };
  }
  
  // Unsupported event type
  return {
    error: "Unsupported event type"
  };
}
```

### Schedule Functions

Schedule functions are triggered by schedules. They can be used for periodic tasks such as data aggregation, cleanup, or reporting.

```javascript
export default async function(event, context) {
  // Get schedule information
  const timestamp = event.data.timestamp;
  const scheduleName = event.data.name;
  
  // Log schedule information
  context.log.info(`Schedule triggered: ${scheduleName} at ${new Date(timestamp).toISOString()}`);
  
  // Get NEO price
  const neoPrice = await context.oracle.getPrice("NEO", "USD");
  
  // Store price in persistent storage
  await context.storage.set(`neo-price-${timestamp}`, neoPrice);
  
  // Get historical prices
  const prices = [];
  const now = Date.now();
  for (let i = 0; i < 24; i++) {
    const time = now - i * 60 * 60 * 1000;
    const price = await context.storage.get(`neo-price-${time}`);
    if (price) {
      prices.push({ time, price });
    }
  }
  
  return {
    timestamp,
    scheduleName,
    neoPrice,
    historicalPrices: prices
  };
}
```

## Neo N3 Integration

The Neo N3 FaaS platform provides seamless integration with the Neo N3 blockchain through the `context.neo` object.

### Getting Blockchain Information

```javascript
export default async function(event, context) {
  // Get current block height
  const blockHeight = await context.neo.getCurrentBlockHeight();
  
  // Get block by height
  const block = await context.neo.getBlock(blockHeight);
  
  // Get block by hash
  const blockByHash = await context.neo.getBlockByHash(block.hash);
  
  // Get transaction by hash
  const tx = await context.neo.getTransaction(block.tx[0]);
  
  // Get transaction receipt
  const receipt = await context.neo.getTransactionReceipt(block.tx[0]);
  
  return {
    blockHeight,
    blockHash: block.hash,
    blockTime: block.time,
    transactions: block.tx.length,
    firstTransaction: tx.hash,
    receipt
  };
}
```

### Interacting with Smart Contracts

```javascript
export default async function(event, context) {
  // Get contract instance
  const contract = await context.neo.getContract("0x1234567890abcdef");
  
  // Call contract method (read-only)
  const totalSupply = await contract.call("totalSupply", []);
  
  // Call contract method with parameters
  const balance = await contract.call("balanceOf", ["NXV7ZhHiyM1aHXwvUBCta7dGNP1tHwfoQG"]);
  
  // Invoke contract method (write)
  const txHash = await contract.invoke("transfer", [
    "NXV7ZhHiyM1aHXwvUBCta7dGNP1tHwfoQG",
    "NLnyLtep7jwyq1qhNPkwXbJpurC4jUT8ke",
    100
  ]);
  
  // Wait for transaction to be confirmed
  const receipt = await context.neo.waitForTransaction(txHash);
  
  return {
    contractHash: "0x1234567890abcdef",
    totalSupply,
    balance,
    txHash,
    receipt
  };
}
```

### Working with NEP-17 Tokens

```javascript
export default async function(event, context) {
  // Get NEP-17 token instance
  const token = await context.neo.getNep17Token("0x1234567890abcdef");
  
  // Get token information
  const symbol = await token.symbol();
  const decimals = await token.decimals();
  const totalSupply = await token.totalSupply();
  
  // Get token balance
  const balance = await token.balanceOf("NXV7ZhHiyM1aHXwvUBCta7dGNP1tHwfoQG");
  
  // Transfer tokens
  const txHash = await token.transfer(
    "NXV7ZhHiyM1aHXwvUBCta7dGNP1tHwfoQG",
    "NLnyLtep7jwyq1qhNPkwXbJpurC4jUT8ke",
    100
  );
  
  // Wait for transaction to be confirmed
  const receipt = await context.neo.waitForTransaction(txHash);
  
  return {
    tokenHash: "0x1234567890abcdef",
    symbol,
    decimals,
    totalSupply,
    balance,
    txHash,
    receipt
  };
}
```

## Oracle Services

The Neo N3 FaaS platform provides oracle services for accessing external data through the `context.oracle` object.

### Price Feed Oracle

```javascript
export default async function(event, context) {
  // Get NEO price in USD
  const neoPrice = await context.oracle.getPrice("NEO", "USD");
  
  // Get GAS price in USD
  const gasPrice = await context.oracle.getPrice("GAS", "USD");
  
  // Get BTC price in USD
  const btcPrice = await context.oracle.getPrice("BTC", "USD");
  
  // Get ETH price in USD
  const ethPrice = await context.oracle.getPrice("ETH", "USD");
  
  // Get multiple prices at once
  const prices = await context.oracle.getPrices(["NEO", "GAS", "BTC", "ETH"], "USD");
  
  // Get historical prices
  const historicalPrices = await context.oracle.getHistoricalPrices("NEO", "USD", {
    from: Date.now() - 7 * 24 * 60 * 60 * 1000, // 7 days ago
    to: Date.now(),
    interval: "1d" // 1 day interval
  });
  
  return {
    neoPrice,
    gasPrice,
    btcPrice,
    ethPrice,
    prices,
    historicalPrices
  };
}
```

### Random Number Oracle

```javascript
export default async function(event, context) {
  // Generate random number between 1 and 100
  const randomNumber = await context.oracle.getRandomNumber(1, 100);
  
  // Generate multiple random numbers
  const randomNumbers = await context.oracle.getRandomNumbers(1, 100, 5);
  
  // Generate random bytes
  const randomBytes = await context.oracle.getRandomBytes(32);
  
  // Generate random string
  const randomString = await context.oracle.getRandomString(16);
  
  return {
    randomNumber,
    randomNumbers,
    randomBytes,
    randomString
  };
}
```

### Weather Oracle

```javascript
export default async function(event, context) {
  // Get current weather by city
  const weatherByCity = await context.oracle.getWeather("New York");
  
  // Get current weather by coordinates
  const weatherByCoordinates = await context.oracle.getWeather({
    latitude: 40.7128,
    longitude: -74.0060
  });
  
  // Get weather forecast
  const forecast = await context.oracle.getWeatherForecast("New York", {
    days: 5
  });
  
  return {
    weatherByCity,
    weatherByCoordinates,
    forecast
  };
}
```

## TEE Services

The Neo N3 FaaS platform provides Trusted Execution Environment (TEE) services for secure execution through the `context.tee` object.

### Secure Execution

```javascript
export default async function(event, context) {
  // Execute code in TEE
  const result = await context.tee.execute(async (secureContext) => {
    // Generate key pair
    const keyPair = await secureContext.crypto.generateKeyPair();
    
    // Sign data
    const signature = await secureContext.crypto.sign(
      keyPair.privateKey,
      "Hello, Neo N3 FaaS!"
    );
    
    // Return public key and signature
    return {
      publicKey: keyPair.publicKey,
      signature
    };
  });
  
  // Verify signature outside TEE
  const verified = await context.crypto.verify(
    result.publicKey,
    "Hello, Neo N3 FaaS!",
    result.signature
  );
  
  return {
    result,
    verified
  };
}
```

### Secure Storage

```javascript
export default async function(event, context) {
  // Execute code in TEE
  const result = await context.tee.execute(async (secureContext) => {
    // Store sensitive data
    await secureContext.storage.set("api-key", "secret-api-key");
    
    // Retrieve sensitive data
    const apiKey = await secureContext.storage.get("api-key");
    
    // Use sensitive data
    const response = await secureContext.http.get("https://api.example.com", {
      headers: {
        "Authorization": `Bearer ${apiKey}`
      }
    });
    
    // Return result without exposing sensitive data
    return {
      success: response.status === 200,
      data: response.data
    };
  });
  
  return {
    result
  };
}
```

### Secure Multi-Party Computation

```javascript
export default async function(event, context) {
  // Execute code in TEE
  const result = await context.tee.execute(async (secureContext) => {
    // Get input from multiple parties
    const inputA = await secureContext.getInput("party-a");
    const inputB = await secureContext.getInput("party-b");
    
    // Perform computation on inputs
    const result = inputA + inputB;
    
    // Return result to all parties
    await secureContext.setOutput("party-a", result);
    await secureContext.setOutput("party-b", result);
    
    // Return public result
    return {
      success: true
    };
  });
  
  return {
    result
  };
}
```

## Function Configuration

Functions are configured using the `r3e.yaml` file in the project root directory.

```yaml
functions:
  hello-world:
    handler: functions/hello-world.js
    runtime: javascript
    trigger:
      type: http
      path: /hello-world
    environment:
      NODE_ENV: production
    resources:
      memory: 128
      cpu: 0.1
      timeout: 30
    permissions:
      - neo:read
      - oracle:price
      - storage:read
      - storage:write
  
  neo-info:
    handler: functions/neo-info.js
    runtime: javascript
    trigger:
      type: blockchain
      events:
        - type: block
    environment:
      NODE_ENV: production
    resources:
      memory: 256
      cpu: 0.2
      timeout: 60
    permissions:
      - neo:read
      - oracle:price
  
  price-tracker:
    handler: functions/price-tracker.js
    runtime: javascript
    trigger:
      type: schedule
      cron: "0 * * * *" # Every hour
    environment:
      NODE_ENV: production
    resources:
      memory: 128
      cpu: 0.1
      timeout: 30
    permissions:
      - neo:read
      - oracle:price
      - storage:read
      - storage:write
```

### Trigger Types

The Neo N3 FaaS platform supports several trigger types:

- **HTTP**: Triggered by HTTP requests
  ```yaml
  trigger:
    type: http
    path: /hello-world
    methods: [GET, POST] # Optional, defaults to all methods
    cors: true # Optional, defaults to false
  ```

- **Blockchain**: Triggered by blockchain events
  ```yaml
  trigger:
    type: blockchain
    events:
      - type: block
      - type: transaction
      - type: notification
        contract: "0x1234567890abcdef"
        event: "Transfer"
  ```

- **Schedule**: Triggered by schedules
  ```yaml
  trigger:
    type: schedule
    cron: "0 * * * *" # Every hour
    timezone: "UTC" # Optional, defaults to UTC
  ```

- **Custom**: Triggered by custom events
  ```yaml
  trigger:
    type: custom
    event: "custom-event"
  ```

### Environment Variables

Environment variables can be defined in the function configuration:

```yaml
environment:
  NODE_ENV: production
  DEBUG: false
  API_KEY: ${SECRET_API_KEY} # Reference to secret
```

### Resources

Resource limits can be defined in the function configuration:

```yaml
resources:
  memory: 128 # Memory limit in MB
  cpu: 0.1 # CPU limit in cores
  timeout: 30 # Timeout in seconds
```

### Permissions

Permissions can be defined in the function configuration:

```yaml
permissions:
  - neo:read # Read access to Neo N3 blockchain
  - neo:write # Write access to Neo N3 blockchain
  - oracle:price # Access to price oracle
  - oracle:random # Access to random number oracle
  - oracle:weather # Access to weather oracle
  - tee:execute # Access to TEE execution
  - storage:read # Read access to persistent storage
  - storage:write # Write access to persistent storage
```

## Testing and Debugging

### Local Testing

The Neo N3 FaaS platform provides tools for testing functions locally.

```bash
# Test function locally
r3e-faas-cli invoke-local --function hello-world

# Test function with parameters
r3e-faas-cli invoke-local --function hello-world --params '{"name": "Neo"}'

# Test function with event
r3e-faas-cli invoke-local --function hello-world --event '{"method": "GET", "path": "/hello-world", "query": {"name": "Neo"}}'
```

### Debugging

The Neo N3 FaaS platform provides tools for debugging functions.

```bash
# Debug function locally
r3e-faas-cli invoke-local --function hello-world --debug

# View function logs
r3e-faas-cli logs --function hello-world

# Follow function logs
r3e-faas-cli logs --function hello-world --follow

# View function metrics
r3e-faas-cli metrics --function hello-world
```

### Unit Testing

Functions can be unit tested using standard JavaScript testing frameworks such as Jest or Mocha.

```javascript
// hello-world.test.js
const helloWorld = require('./hello-world');

describe('hello-world', () => {
  it('should return hello message', async () => {
    const event = {
      method: 'GET',
      path: '/hello-world',
      query: {
        name: 'Neo'
      }
    };
    
    const context = {
      log: {
        info: jest.fn()
      }
    };
    
    const result = await helloWorld(event, context);
    
    expect(result).toEqual({
      statusCode: 200,
      headers: {
        'content-type': 'application/json'
      },
      body: {
        message: 'Hello, Neo!'
      }
    });
    
    expect(context.log.info).toHaveBeenCalled();
  });
});
```

## Best Practices

### Function Design

- **Keep functions small and focused**: Each function should do one thing and do it well.
- **Use async/await**: Use async/await for asynchronous operations to make your code more readable.
- **Handle errors gracefully**: Use try/catch blocks to handle errors and provide meaningful error messages.
- **Validate input**: Validate all input parameters to prevent security vulnerabilities.
- **Use environment variables**: Use environment variables for configuration to make your functions more portable.

### Performance

- **Minimize cold starts**: Keep your functions small and avoid unnecessary dependencies to minimize cold start times.
- **Use caching**: Cache expensive operations to improve performance.
- **Optimize database queries**: Minimize the number of database queries and optimize them for performance.
- **Use connection pooling**: Reuse connections to external services to improve performance.
- **Implement timeouts**: Implement timeouts for external service calls to prevent hanging functions.

### Security

- **Follow the principle of least privilege**: Only request the permissions your function needs.
- **Use TEE for sensitive operations**: Use TEE for operations that involve sensitive data.
- **Encrypt sensitive data**: Encrypt sensitive data at rest and in transit.
- **Implement proper authentication and authorization**: Verify the identity of users and ensure they have the necessary permissions.
- **Validate and sanitize input**: Validate and sanitize all input to prevent injection attacks.

### Monitoring and Logging

- **Log important events**: Log important events to help with debugging and monitoring.
- **Use structured logging**: Use structured logging to make logs easier to search and analyze.
- **Monitor function performance**: Monitor function performance to identify bottlenecks and issues.
- **Set up alerts**: Set up alerts for abnormal behavior or errors.
- **Implement distributed tracing**: Implement distributed tracing to track requests across multiple functions and services.

### Testing

- **Write unit tests**: Write unit tests for your functions to ensure they work as expected.
- **Write integration tests**: Write integration tests to ensure your functions work with other components.
- **Use mocks for external services**: Use mocks for external services to make your tests more reliable.
- **Test error handling**: Test error handling to ensure your functions handle errors gracefully.
- **Test performance**: Test performance to ensure your functions meet performance requirements.

For more information, see the [API Reference](../api-reference.md) and [Architecture](../architecture.md) documents.
