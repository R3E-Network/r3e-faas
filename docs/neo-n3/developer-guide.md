# Neo N3 FaaS Platform Developer Guide

This guide provides detailed information for developers who want to build applications and services on the Neo N3 FaaS platform.

## Table of Contents

1. [Introduction](#introduction)
2. [Development Environment Setup](#development-environment-setup)
3. [Function Development](#function-development)
4. [Neo N3 Integration](#neo-n3-integration)
5. [Oracle Services](#oracle-services)
6. [TEE Services](#tee-services)
7. [Service Development](#service-development)
8. [API Integration](#api-integration)
9. [Testing and Debugging](#testing-and-debugging)
10. [Best Practices](#best-practices)

## Introduction

The Neo N3 FaaS platform is a serverless computing platform designed specifically for the Neo N3 blockchain ecosystem. It allows developers to write and deploy JavaScript functions that can be triggered by blockchain events, API calls, or scheduled tasks.

## Development Environment Setup

### Prerequisites

- Node.js 16 or later
- npm 7 or later
- Git

### Installing the CLI

```bash
npm install -g r3e-faas-cli
```

### Creating a Project

```bash
r3e-faas-cli init my-neo-faas-project
cd my-neo-faas-project
```

## Function Development

### JavaScript Functions

```javascript
export default async function(event, context) {
  console.log("Hello, Neo N3!");
  return {
    message: "Hello, Neo N3!",
    blockHeight: context.neo.getCurrentBlockHeight()
  };
}
```

### Function Triggers

- HTTP Triggers: Invoke functions via HTTP requests
- Neo N3 Blockchain Triggers: React to blockchain events
- Schedule Triggers: Execute functions on a schedule

## Neo N3 Integration

### Neo N3 API

```javascript
// Get current block height
const blockHeight = await context.neo.getCurrentBlockHeight();

// Get contract instance
const contract = await context.neo.getContract("0x1234567890abcdef");

// Call contract method
const balance = await contract.call("balanceOf", ["NXV7ZhHiyM1aHXwvUBCta7dGNP1tHwfoQG"]);
```

## Oracle Services

### Price Feeds

```javascript
// Get NEO price in USD
const neoPrice = await context.oracle.getPrice("NEO", "USD");
```

### Random Number Generation

```javascript
// Generate random number
const randomNumber = await context.oracle.getRandomNumber(1, 100);
```

## TEE Services

### Secure Execution

```javascript
// Execute code in TEE
const result = await context.tee.execute(async (secureContext) => {
  // Generate key pair
  const keyPair = await secureContext.crypto.generateKeyPair();
  
  // Return public key
  return { publicKey: keyPair.publicKey };
});
```

## Service Development

### Service Types

- Standard Services: General-purpose services
- Oracle Services: External data providers
- TEE Services: Secure execution environments
- Blockchain Services: Blockchain interaction

## API Integration

### REST API

```javascript
// Invoke function via REST API
const response = await fetch('https://faas.example.com/functions/hello-neo/invoke', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'Authorization': 'Bearer YOUR_JWT_TOKEN'
  },
  body: JSON.stringify({ params: { name: 'Neo' } })
});
```

## Testing and Debugging

### Local Testing

```bash
r3e-faas-cli invoke-local --function hello-neo --params '{"name": "Neo"}'
```

### Logging

```bash
r3e-faas-cli logs --function hello-neo
```

## Best Practices

1. Use TEE for sensitive operations
2. Validate all input parameters
3. Implement proper error handling
4. Use caching for expensive operations
5. Keep functions small and focused
6. Follow the principle of least privilege
7. Use environment variables for configuration
8. Write comprehensive tests
9. Monitor function performance
10. Keep dependencies up to date

For more detailed information, see the [API Reference](./api-reference.md) and [Architecture](./architecture.md) documents.
