# API Reference

This document provides a reference for the R3E FaaS platform's HTTP API and JavaScript API.

## HTTP API

The R3E FaaS platform provides a RESTful HTTP API for managing functions, triggers, and other resources.

### Base URL

```
http://localhost:8080/api/v1
```

### Authentication

Most API endpoints require authentication. You can authenticate using an API key in the `Authorization` header:

```
Authorization: Bearer your-api-key
```

### Functions

#### List Functions

```
GET /functions
```

Response:

```json
{
  "functions": [
    {
      "id": "function-id",
      "name": "function-name",
      "created_at": "2023-01-01T00:00:00Z",
      "updated_at": "2023-01-01T00:00:00Z"
    }
  ]
}
```

#### Get Function

```
GET /functions/{function_id}
```

Response:

```json
{
  "id": "function-id",
  "name": "function-name",
  "code": "export default function(event) { return { hello: 'world' }; }",
  "created_at": "2023-01-01T00:00:00Z",
  "updated_at": "2023-01-01T00:00:00Z"
}
```

#### Create Function

```
POST /functions
```

Request:

```json
{
  "name": "function-name",
  "code": "export default function(event) { return { hello: 'world' }; }"
}
```

Response:

```json
{
  "id": "function-id",
  "name": "function-name",
  "created_at": "2023-01-01T00:00:00Z",
  "updated_at": "2023-01-01T00:00:00Z"
}
```

#### Update Function

```
PUT /functions/{function_id}
```

Request:

```json
{
  "code": "export default function(event) { return { hello: 'updated' }; }"
}
```

Response:

```json
{
  "id": "function-id",
  "name": "function-name",
  "updated_at": "2023-01-01T00:00:00Z"
}
```

#### Delete Function

```
DELETE /functions/{function_id}
```

Response:

```json
{
  "success": true
}
```

#### Invoke Function

```
POST /functions/{function_id}/invoke
```

Request:

```json
{
  "data": {
    "key": "value"
  }
}
```

Response:

```json
{
  "result": {
    "hello": "world"
  },
  "execution_time_ms": 5
}
```

### Triggers

#### List Triggers

```
GET /triggers
```

Response:

```json
{
  "triggers": [
    {
      "id": "trigger-id",
      "function_id": "function-id",
      "type": "blockchain",
      "event": "NeoNewBlock",
      "created_at": "2023-01-01T00:00:00Z"
    }
  ]
}
```

#### Create Trigger

```
POST /triggers
```

Request:

```json
{
  "function_id": "function-id",
  "type": "blockchain",
  "event": "NeoNewBlock"
}
```

Response:

```json
{
  "id": "trigger-id",
  "function_id": "function-id",
  "type": "blockchain",
  "event": "NeoNewBlock",
  "created_at": "2023-01-01T00:00:00Z"
}
```

#### Delete Trigger

```
DELETE /triggers/{trigger_id}
```

Response:

```json
{
  "success": true
}
```

### Account

#### Get Account Info

```
GET /account
```

Response:

```json
{
  "address": "NbnjKGMBJzJ6j5PHeYhjJDaQ5Vy5UYu4Fv",
  "neo_balance": "10.0",
  "gas_balance": "5.0"
}
```

#### Withdraw Tokens

```
POST /account/withdraw
```

Request:

```json
{
  "token": "GAS",
  "amount": "1.0",
  "address": "NbnjKGMBJzJ6j5PHeYhjJDaQ5Vy5UYu4Fv"
}
```

Response:

```json
{
  "tx_hash": "0x1234567890abcdef",
  "amount": "1.0",
  "token": "GAS"
}
```

### Secrets

#### List Secrets

```
GET /secrets
```

Response:

```json
{
  "secrets": [
    {
      "name": "API_KEY",
      "created_at": "2023-01-01T00:00:00Z"
    }
  ]
}
```

#### Set Secret

```
POST /secrets
```

Request:

```json
{
  "name": "API_KEY",
  "value": "your-api-key"
}
```

Response:

```json
{
  "name": "API_KEY",
  "created_at": "2023-01-01T00:00:00Z"
}
```

#### Delete Secret

```
DELETE /secrets/{secret_name}
```

Response:

```json
{
  "success": true
}
```

## JavaScript API

The R3E FaaS platform provides a JavaScript API for use within functions.

### Event Object

Functions receive an event object as their first argument:

```javascript
export default function(event) {
  console.log(event);
  return event;
}
```

The event object has the following structure:

```javascript
{
  // Event context
  context: {
    trigger: "NeoNewBlock", // Trigger type
    triggered_time: 1672531200, // Unix timestamp
    source: "Neo" // Event source
  },
  
  // Event data
  data: {
    id: "event-id",
    payload: {
      // Event-specific payload
      block_number: 12345,
      block_hash: "0x1234567890abcdef"
    }
  }
}
```

### r3e.oracle

The `r3e.oracle` object provides access to oracle services:

```javascript
// Get cryptocurrency price
const neoPrice = await r3e.oracle.getPrice("NEO/USD");

// Get random number
const randomNumber = await r3e.oracle.getRandom(1, 100);

// Get weather data
const weather = await r3e.oracle.getWeather("New York");

// Get data from HTTP endpoint
const data = await r3e.oracle.getHttp("https://api.example.com/data");
```

### r3e.neoServices

The `r3e.neoServices` object provides access to Neo N3 blockchain services:

```javascript
// Get balance
const balance = await r3e.neoServices.getBalance();

// Transfer GAS
const result = await r3e.neoServices.transferGas(
  "NbnjKGMBJzJ6j5PHeYhjJDaQ5Vy5UYu4Fv", // recipient address
  5 // amount
);

// Invoke smart contract
const invokeResult = await r3e.neoServices.invokeContract(
  "0x1234567890abcdef", // script hash
  "transfer", // method
  ["NbnjKGMBJzJ6j5PHeYhjJDaQ5Vy5UYu4Fv", 5] // parameters
);
```

### r3e.tee

The `r3e.tee` object provides access to Trusted Execution Environment services:

```javascript
// Execute code in TEE
const result = await r3e.tee.execute(
  "function secureOperation(data) { return data * 2; }", // code
  [42] // arguments
);

// Verify attestation
const isValid = await r3e.tee.verifyAttestation(attestationData);
```

### r3e.secrets

The `r3e.secrets` object provides access to secret management:

```javascript
// Get secret
const apiKey = await r3e.secrets.get("API_KEY");

// Set secret
await r3e.secrets.set("NEW_SECRET", "secret-value");

// Delete secret
await r3e.secrets.delete("OLD_SECRET");
```

### r3e.sandbox

The `r3e.sandbox` object provides access to sandbox management:

```javascript
// Request permission
await r3e.sandbox.requestPermission("net");

// Check permission
const hasPermission = await r3e.sandbox.hasPermission("fs");
```

### r3e.indexing

The `r3e.indexing` object provides access to data indexing services:

```javascript
// Create collection
await r3e.indexing.createCollection("users");

// Insert document
await r3e.indexing.insertDocument("users", {
  id: "user-1",
  name: "John Doe",
  email: "john@example.com"
});

// Query documents
const users = await r3e.indexing.queryDocuments("users", {
  name: "John Doe"
});
```

### r3e.identity

The `r3e.identity` object provides access to identity services:

```javascript
// Create DID
const did = await r3e.identity.createDid();

// Issue credential
const credential = await r3e.identity.issueCredential(
  did,
  {
    name: "John Doe",
    email: "john@example.com"
  }
);

// Verify credential
const isValid = await r3e.identity.verifyCredential(credential);
```

### r3e.bridge

The `r3e.bridge` object provides access to cross-chain bridge services:

```javascript
// Transfer tokens
const result = await r3e.bridge.transferTokens(
  "Neo", // source chain
  "Ethereum", // target chain
  "GAS", // token
  "0x1234567890abcdef", // recipient address
  5 // amount
);

// Get transaction status
const status = await r3e.bridge.getTransactionStatus(result.tx_hash);
```

### r3e.autoContract

The `r3e.autoContract` object provides access to automatic smart contract services:

```javascript
// Create automatic contract
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

// Get contract status
const status = await r3e.autoContract.getStatus(contract.id);

// Delete contract
await r3e.autoContract.delete(contract.id);
```
