# Quick Start Guide

This guide will help you get started with the R3E FaaS platform.

## Prerequisites

- Completed [installation](./installation.md)
- Basic knowledge of JavaScript
- Neo N3 wallet (for token deposits)

## Running Your First Function

### 1. Create a JavaScript Function

Create a file named `hello.js` with the following content:

```javascript
// hello.js
export default function(event) {
  console.log("Hello, R3E FaaS!");
  return {
    message: "Hello, R3E FaaS!",
    event: event
  };
}
```

### 2. Deploy the Function

```bash
# Deploy the function
r3e function deploy --name hello --file hello.js
```

### 3. Invoke the Function

```bash
# Invoke the function
r3e function invoke --name hello --data '{"key": "value"}'
```

## Using Built-in Services

### Oracle Service

```javascript
// price-alert.js
export default async function(event) {
  // Get the current price of NEO
  const neoPrice = await r3e.oracle.getPrice("NEO/USD");
  
  // Check if the price is above a threshold
  if (neoPrice > 50) {
    console.log("NEO price is above $50!");
  }
  
  return {
    price: neoPrice,
    timestamp: Date.now()
  };
}
```

### Gas Bank Service

```javascript
// gas-transfer.js
export default async function(event) {
  // Get the current balance
  const balance = await r3e.neoServices.getBalance();
  
  // Transfer GAS to another address
  if (balance.gas_balance > 10) {
    const result = await r3e.neoServices.transferGas(
      "NbnjKGMBJzJ6j5PHeYhjJDaQ5Vy5UYu4Fv", // recipient address
      5 // amount
    );
    
    return {
      success: true,
      txHash: result.tx_hash
    };
  }
  
  return {
    success: false,
    reason: "Insufficient balance"
  };
}
```

## Depositing Tokens

To deposit tokens to your account:

1. Get your deposit address:

```bash
r3e account info
```

2. Send NEO or GAS tokens to the displayed address from your Neo N3 wallet.

3. Check your balance:

```bash
r3e account balance
```

## Setting Up Triggers

You can set up triggers to automatically invoke functions in response to events:

```bash
# Set up a trigger for Neo N3 block events
r3e trigger create --function hello --event NeoNewBlock

# Set up a time-based trigger (every hour)
r3e trigger create --function hello --schedule "0 * * * *"

# Set up a price trigger (when NEO price changes by 5%)
r3e trigger create --function hello --price-change "NEO/USD" --threshold 5
```

## Managing Secrets

You can store and use secrets in your functions:

```bash
# Store a secret
r3e secret set --name API_KEY --value "your-api-key"
```

```javascript
// use-secret.js
export default async function(event) {
  // Get the secret
  const apiKey = await r3e.secrets.get("API_KEY");
  
  // Use the secret
  const response = await fetch("https://api.example.com/data", {
    headers: {
      "Authorization": `Bearer ${apiKey}`
    }
  });
  
  return await response.json();
}
```

## Next Steps

- [Development Guide](./development.md)
- [API Reference](./api-reference.md)
- [Docker Deployment](./docker-production.md)
