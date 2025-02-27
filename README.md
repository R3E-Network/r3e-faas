# R3E FaaS: A Function as a Service Platform for Web3

R3E FaaS is a serverless computing platform built specifically for Web3 applications, enabling developers to run JavaScript functions in response to blockchain events without managing infrastructure. The platform integrates with the Neo N3 blockchain, providing built-in services for oracle data, gas management, and secure execution environments.

## Key Features

- **Blockchain Integration**: Seamlessly interact with Neo N3 blockchain
- **Secure Sandboxed Execution**: Run user JavaScript functions in isolated environments
- **Built-in Web3 Services**: Access oracle data, gas management, and TEE capabilities
- **Token-based Billing**: Pay for execution using NEO or GAS tokens
- **Event-driven Architecture**: Execute functions in response to blockchain events

## Components

* **r3e-built-in-services**: Consolidated services including oracle, gas bank, and TEE
* **r3e-core**: Core and common functions
* **r3e-deno**: JavaScript runtime based on deno-core with V8 engine
* **r3e-event**: Event definitions and sources from various blockchain events
* **r3e-neo-services**: Neo N3 blockchain services integration
* **r3e-oracle**: Oracle services for external data access
* **r3e-proc-macros**: Procedural macros for code generation
* **r3e-runlog**: Function execution logging and monitoring
* **r3e-scheduler**: Scheduling logic for worker nodes and function execution
* **r3e-stock**: Blockchain history and data querying
* **r3e-store**: Event and data storage
* **r3e-tee**: Trusted Execution Environment implementation
* **r3e-worker**: Worker node implementation for function execution

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

### Requesting Permissions

```javascript
// Request network access permission
await r3e.sandbox.requestPermission("net");

// Request file system access
await r3e.sandbox.requestPermission("fs");
```

## Architecture
