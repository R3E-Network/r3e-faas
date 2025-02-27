# Service API Example for Neo N3 FaaS Platform

This example demonstrates how to use the Service API on the Neo N3 FaaS platform.

## Prerequisites

- Neo N3 FaaS CLI installed (`npm install -g r3e-faas-cli`)
- A Neo N3 FaaS account
- Basic knowledge of JavaScript, REST APIs, and GraphQL

## Introduction to Service API

The Neo N3 FaaS platform provides a comprehensive Service API in two formats:

1. **REST API**: A traditional RESTful API with HTTP endpoints
2. **GraphQL API**: A flexible query language for your API

## Authentication

The platform supports two authentication methods:

1. **API Key Authentication**: Using an API key in the `X-API-Key` header
2. **JWT Authentication**: Using a JWT token in the `Authorization` header

### Getting an API Key

```bash
r3e-faas-cli auth api-key
```

### Getting a JWT Token

```bash
r3e-faas-cli auth login
```

## REST API Examples

### Base URL

```
https://faas.example.com/api/v1
```

### Creating a Function

```bash
curl -X POST https://faas.example.com/api/v1/functions \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-api-key" \
  -d '{
    "name": "rest-api-function",
    "description": "A function created using the REST API",
    "runtime": "JAVASCRIPT",
    "trigger": {
      "type": "HTTP",
      "http": {
        "path": "/rest-api-function",
        "method": "GET"
      }
    },
    "code": "export default async function(event, context) { return { message: \"Hello, World!\" }; }"
  }'
```

### Listing Functions

```bash
curl -X GET https://faas.example.com/api/v1/functions \
  -H "X-API-Key: your-api-key"
```

### Invoking a Function

```bash
curl -X POST https://faas.example.com/api/v1/functions/function-id/invoke \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-api-key" \
  -d '{"params": {"name": "Neo"}}'
```

## GraphQL API Examples

### GraphQL Endpoint

```
https://faas.example.com/graphql
```

### Creating a Function

```graphql
mutation CreateFunction($input: CreateFunctionInput!) {
  createFunction(input: $input) {
    id
    name
    description
  }
}

# Variables
{
  "input": {
    "name": "graphql-api-function",
    "description": "A function created using the GraphQL API",
    "runtime": "JAVASCRIPT",
    "trigger": {
      "type": "HTTP",
      "http": {
        "path": "/graphql-api-function",
        "method": "GET"
      }
    },
    "code": "export default async function(event, context) { return { message: \"Hello, World!\" }; }"
  }
}
```

### Listing Functions

```graphql
query GetFunctions {
  functions {
    id
    name
    description
  }
}
```

### Invoking a Function

```graphql
mutation InvokeFunction($id: ID!, $params: JSON) {
  invokeFunction(id: $id, params: $params) {
    result
    executionTime
  }
}

# Variables
{
  "id": "function-id",
  "params": {
    "name": "Neo"
  }
}
```

## JavaScript Client Example

Here's a simple JavaScript client for the REST API:

```javascript
// neo-faas-client.js
const fetch = require('node-fetch');

class NeoFaasClient {
  constructor(options) {
    this.apiUrl = options.apiUrl || 'https://faas.example.com/api/v1';
    this.apiKey = options.apiKey;
  }
  
  async listFunctions() {
    const response = await fetch(`${this.apiUrl}/functions`, {
      headers: { 'X-API-Key': this.apiKey }
    });
    return response.json();
  }
  
  async createFunction(functionDef) {
    const response = await fetch(`${this.apiUrl}/functions`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'X-API-Key': this.apiKey
      },
      body: JSON.stringify(functionDef)
    });
    return response.json();
  }
  
  async getFunction(functionId) {
    const response = await fetch(`${this.apiUrl}/functions/${functionId}`, {
      headers: { 'X-API-Key': this.apiKey }
    });
    return response.json();
  }
  
  async updateFunction(functionId, functionDef) {
    const response = await fetch(`${this.apiUrl}/functions/${functionId}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        'X-API-Key': this.apiKey
      },
      body: JSON.stringify(functionDef)
    });
    return response.json();
  }
  
  async deleteFunction(functionId) {
    const response = await fetch(`${this.apiUrl}/functions/${functionId}`, {
      method: 'DELETE',
      headers: { 'X-API-Key': this.apiKey }
    });
    return response.json();
  }
  
  async invokeFunction(functionId, params) {
    const response = await fetch(`${this.apiUrl}/functions/${functionId}/invoke`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'X-API-Key': this.apiKey
      },
      body: JSON.stringify({ params })
    });
    return response.json();
  }
}

module.exports = NeoFaasClient;
```

## TypeScript Client Example

Here's a TypeScript client for the GraphQL API:

```typescript
// neo-faas-graphql-client.ts
import { GraphQLClient } from 'graphql-request';
import { gql } from 'graphql-tag';

interface FunctionInput {
  name: string;
  description?: string;
  runtime: string;
  trigger: {
    type: string;
    [key: string]: any;
  };
  code: string;
  environment?: Record<string, string>;
}

interface Function {
  id: string;
  name: string;
  description?: string;
  runtime: string;
  trigger: {
    type: string;
    [key: string]: any;
  };
  createdAt: string;
  updatedAt: string;
}

interface InvocationResult {
  result: any;
  executionTime: number;
}

class NeoFaasGraphQLClient {
  private client: GraphQLClient;
  
  constructor(options: { endpoint?: string; token?: string }) {
    const endpoint = options.endpoint || 'https://faas.example.com/graphql';
    this.client = new GraphQLClient(endpoint, {
      headers: options.token ? {
        Authorization: `Bearer ${options.token}`
      } : {}
    });
  }
  
  async listFunctions(): Promise<Function[]> {
    const query = gql`
      query GetFunctions {
        functions {
          id
          name
          description
          runtime
          trigger
          createdAt
          updatedAt
        }
      }
    `;
    
    const data = await this.client.request(query);
    return data.functions;
  }
  
  async createFunction(input: FunctionInput): Promise<Function> {
    const mutation = gql`
      mutation CreateFunction($input: CreateFunctionInput!) {
        createFunction(input: $input) {
          id
          name
          description
          runtime
          trigger
          createdAt
          updatedAt
        }
      }
    `;
    
    const data = await this.client.request(mutation, { input });
    return data.createFunction;
  }
  
  async getFunction(id: string): Promise<Function> {
    const query = gql`
      query GetFunction($id: ID!) {
        function(id: $id) {
          id
          name
          description
          runtime
          trigger
          createdAt
          updatedAt
        }
      }
    `;
    
    const data = await this.client.request(query, { id });
    return data.function;
  }
  
  async updateFunction(id: string, input: Partial<FunctionInput>): Promise<Function> {
    const mutation = gql`
      mutation UpdateFunction($id: ID!, $input: UpdateFunctionInput!) {
        updateFunction(id: $id, input: $input) {
          id
          name
          description
          runtime
          trigger
          createdAt
          updatedAt
        }
      }
    `;
    
    const data = await this.client.request(mutation, { id, input });
    return data.updateFunction;
  }
  
  async deleteFunction(id: string): Promise<boolean> {
    const mutation = gql`
      mutation DeleteFunction($id: ID!) {
        deleteFunction(id: $id)
      }
    `;
    
    const data = await this.client.request(mutation, { id });
    return data.deleteFunction;
  }
  
  async invokeFunction(id: string, params?: any): Promise<InvocationResult> {
    const mutation = gql`
      mutation InvokeFunction($id: ID!, $params: JSON) {
        invokeFunction(id: $id, params: $params) {
          result
          executionTime
        }
      }
    `;
    
    const data = await this.client.request(mutation, { id, params });
    return data.invokeFunction;
  }
}

export default NeoFaasGraphQLClient;
```

## Complete Application Example

Here's a complete example of a Node.js application that uses the Service API to manage and invoke functions:

```javascript
// app.js
const NeoFaasClient = require('./neo-faas-client');

async function main() {
  // Create a client instance
  const client = new NeoFaasClient({
    apiUrl: 'https://faas.example.com/api/v1',
    apiKey: 'your-api-key'
  });
  
  try {
    // Create a new function
    console.log('Creating a new function...');
    const newFunction = await client.createFunction({
      name: 'neo-price-function',
      description: 'A function that returns the current price of NEO',
      runtime: 'JAVASCRIPT',
      trigger: {
        type: 'HTTP',
        http: {
          path: '/neo-price',
          method: 'GET'
        }
      },
      code: `
        export default async function(event, context) {
          // Get the NEO price from the oracle service
          const neoPrice = await context.oracle.getPrice('NEO', 'USD');
          
          return {
            asset: 'NEO',
            price: neoPrice,
            currency: 'USD',
            timestamp: new Date().toISOString()
          };
        }
      `
    });
    
    console.log('Function created:', newFunction);
    
    // List all functions
    console.log('Listing all functions...');
    const functions = await client.listFunctions();
    console.log('Functions:', functions);
    
    // Invoke the function
    console.log('Invoking the function...');
    const result = await client.invokeFunction(newFunction.id);
    console.log('Function result:', result);
    
    // Update the function
    console.log('Updating the function...');
    const updatedFunction = await client.updateFunction(newFunction.id, {
      description: 'An updated function that returns the current price of NEO and GAS'
    });
    console.log('Function updated:', updatedFunction);
    
    // Delete the function
    console.log('Deleting the function...');
    const deleteResult = await client.deleteFunction(newFunction.id);
    console.log('Function deleted:', deleteResult);
  } catch (error) {
    console.error('Error:', error);
  }
}

main();
```

## Service API with Neo N3 Blockchain Integration

Here's an example of using the Service API to create a function that interacts with the Neo N3 blockchain:

```javascript
// neo-blockchain-integration.js
const NeoFaasClient = require('./neo-faas-client');

async function main() {
  // Create a client instance
  const client = new NeoFaasClient({
    apiUrl: 'https://faas.example.com/api/v1',
    apiKey: 'your-api-key'
  });
  
  try {
    // Create a function that monitors Neo N3 blocks
    console.log('Creating a block monitor function...');
    const blockMonitorFunction = await client.createFunction({
      name: 'neo-block-monitor',
      description: 'A function that monitors Neo N3 blocks',
      runtime: 'JAVASCRIPT',
      trigger: {
        type: 'NEO',
        neo: {
          event: 'NEW_BLOCK',
          network: 'mainnet'
        }
      },
      code: `
        export default async function(event, context) {
          // Extract block data from the event
          const { blockHeight, blockHash, blockTime, transactions } = event.data;
          
          // Log block information
          console.log(\`New block detected at height \${blockHeight}\`);
          console.log(\`Block hash: \${blockHash}\`);
          console.log(\`Block time: \${new Date(blockTime).toISOString()}\`);
          console.log(\`Transaction count: \${transactions.length}\`);
          
          // Store the block information in the platform storage
          await context.storage.set(\`block:\${blockHeight}\`, {
            height: blockHeight,
            hash: blockHash,
            time: blockTime,
            transactionCount: transactions.length
          });
          
          return {
            blockHeight,
            blockHash,
            blockTime: new Date(blockTime).toISOString(),
            transactionCount: transactions.length,
            processed: true,
            timestamp: new Date().toISOString()
          };
        }
      `
    });
    
    console.log('Block monitor function created:', blockMonitorFunction);
    
    // Create a function that uses the Oracle service
    console.log('Creating an oracle function...');
    const oracleFunction = await client.createFunction({
      name: 'neo-price-oracle',
      description: 'A function that uses the Oracle service to get NEO price',
      runtime: 'JAVASCRIPT',
      trigger: {
        type: 'HTTP',
        http: {
          path: '/neo-price-oracle',
          method: 'GET'
        }
      },
      code: `
        export default async function(event, context) {
          // Get the NEO price from the oracle service
          const neoPrice = await context.oracle.getPrice('NEO', 'USD');
          
          // Get the GAS price from the oracle service
          const gasPrice = await context.oracle.getPrice('GAS', 'USD');
          
          return {
            neo: {
              price: neoPrice,
              currency: 'USD'
            },
            gas: {
              price: gasPrice,
              currency: 'USD'
            },
            timestamp: new Date().toISOString()
          };
        }
      `
    });
    
    console.log('Oracle function created:', oracleFunction);
    
    // Create a function that uses the TEE service
    console.log('Creating a TEE function...');
    const teeFunction = await client.createFunction({
      name: 'neo-secure-signing',
      description: 'A function that uses the TEE service for secure signing',
      runtime: 'JAVASCRIPT',
      trigger: {
        type: 'HTTP',
        http: {
          path: '/neo-secure-signing',
          method: 'POST'
        }
      },
      tee: {
        provider: 'sgx',
        attestation: 'required'
      },
      code: `
        export default async function(event, context) {
          // Get the data to sign from the event parameters
          const data = event.params?.data || 'Hello, Neo N3 FaaS!';
          
          // Execute the operation in the TEE
          const result = await context.tee.execute(async (secureContext) => {
            // Get the key management API
            const keyManager = secureContext.keyManager;
            
            // Generate a new key pair
            const newKeyPair = await keyManager.generateKeyPair('EC_SECP256R1');
            
            // Sign the data with the new key
            const signature = await keyManager.sign(newKeyPair.keyId, data);
            
            return {
              keyId: newKeyPair.keyId,
              publicKey: newKeyPair.publicKey,
              data,
              signature,
              message: \`Signed data with key ID \${newKeyPair.keyId}\`
            };
          });
          
          return {
            ...result,
            timestamp: new Date().toISOString()
          };
        }
      `
    });
    
    console.log('TEE function created:', teeFunction);
    
    // Invoke the oracle function
    console.log('Invoking the oracle function...');
    const oracleResult = await client.invokeFunction(oracleFunction.id);
    console.log('Oracle function result:', oracleResult);
    
    // Invoke the TEE function
    console.log('Invoking the TEE function...');
    const teeResult = await client.invokeFunction(teeFunction.id, {
      data: 'Sign this secure data'
    });
    console.log('TEE function result:', teeResult);
  } catch (error) {
    console.error('Error:', error);
  }
}

main();
```

## Service API with Custom Services

Here's an example of using the Service API to create and use a custom service:

```javascript
// custom-service.js
const NeoFaasClient = require('./neo-faas-client');

async function main() {
  // Create a client instance
  const client = new NeoFaasClient({
    apiUrl: 'https://faas.example.com/api/v1',
    apiKey: 'your-api-key'
  });
  
  try {
    // Create a custom service
    console.log('Creating a custom service...');
    const customService = await client.createService({
      name: 'stock-oracle',
      description: 'A custom oracle service for stock market data',
      type: 'ORACLE',
      handler: `
        module.exports = function(context) {
          return {
            getStockPrice: async (symbol) => {
              // In a real implementation, you would fetch the data from an external API
              // For this example, we'll simulate the API call
              
              // Validate the symbol
              if (!symbol || typeof symbol !== 'string') {
                throw new Error('Invalid symbol');
              }
              
              // Generate a random price based on the symbol
              const basePrice = symbol.split('').reduce((sum, char) => sum + char.charCodeAt(0), 0) % 1000;
              const randomFactor = 0.9 + (Math.random() * 0.2); // Random factor between 0.9 and 1.1
              const price = basePrice * randomFactor;
              
              return parseFloat(price.toFixed(2));
            },
            
            getCompanyInfo: async (symbol) => {
              // Validate the symbol
              if (!symbol || typeof symbol !== 'string') {
                throw new Error('Invalid symbol');
              }
              
              // Define some example company information
              const companies = {
                'AAPL': {
                  name: 'Apple Inc.',
                  sector: 'Technology',
                  industry: 'Consumer Electronics'
                },
                'MSFT': {
                  name: 'Microsoft Corporation',
                  sector: 'Technology',
                  industry: 'Software'
                }
              };
              
              // Return company information or a generic one if not found
              return companies[symbol] || {
                name: \`\${symbol} Corporation\`,
                sector: 'Unknown',
                industry: 'Unknown'
              };
            }
          };
        }
      `,
      config: {
        updateInterval: 60
      }
    });
    
    console.log('Custom service created:', customService);
    
    // Create a function that uses the custom service
    console.log('Creating a function that uses the custom service...');
    const stockFunction = await client.createFunction({
      name: 'stock-info',
      description: 'A function that uses the custom stock oracle service',
      runtime: 'JAVASCRIPT',
      trigger: {
        type: 'HTTP',
        http: {
          path: '/stock-info',
          method: 'GET'
        }
      },
      services: ['stock-oracle'],
      code: `
        export default async function(event, context) {
          // Get the stock symbol from the event parameters or use a default
          const symbol = event.params?.symbol || 'AAPL';
          
          // Get the custom oracle service
          const stockOracle = context.services.stockOracle;
          
          if (!stockOracle) {
            return {
              error: 'Custom oracle service not available'
            };
          }
          
          // Get the stock price
          const stockPrice = await stockOracle.getStockPrice(symbol);
          
          // Get the company information
          const companyInfo = await stockOracle.getCompanyInfo(symbol);
          
          // Return the stock information
          return {
            symbol,
            price: stockPrice,
            company: companyInfo,
            timestamp: new Date().toISOString()
          };
        }
      `
    });
    
    console.log('Stock function created:', stockFunction);
    
    // Invoke the stock function
    console.log('Invoking the stock function...');
    const stockResult = await client.invokeFunction(stockFunction.id, {
      symbol: 'MSFT'
    });
    console.log('Stock function result:', stockResult);
  } catch (error) {
    console.error('Error:', error);
  }
}

main();
```

## Monitoring and Logging

Here's an example of using the Service API to monitor and log function executions:

```javascript
// monitoring.js
const NeoFaasClient = require('./neo-faas-client');

async function main() {
  // Create a client instance
  const client = new NeoFaasClient({
    apiUrl: 'https://faas.example.com/api/v1',
    apiKey: 'your-api-key'
  });
  
  try {
    // Get function logs
    console.log('Getting function logs...');
    const logs = await client.getFunctionLogs('function-id', {
      limit: 10,
      startTime: new Date(Date.now() - 24 * 60 * 60 * 1000).toISOString(), // Last 24 hours
      endTime: new Date().toISOString()
    });
    console.log('Function logs:', logs);
    
    // Get function metrics
    console.log('Getting function metrics...');
    const metrics = await client.getFunctionMetrics('function-id', {
      period: '1h', // 1 hour
      startTime: new Date(Date.now() - 24 * 60 * 60 * 1000).toISOString(), // Last 24 hours
      endTime: new Date().toISOString()
    });
    console.log('Function metrics:', metrics);
    
    // Get function invocations
    console.log('Getting function invocations...');
    const invocations = await client.getFunctionInvocations('function-id', {
      limit: 10,
      startTime: new Date(Date.now() - 24 * 60 * 60 * 1000).toISOString(), // Last 24 hours
      endTime: new Date().toISOString()
    });
    console.log('Function invocations:', invocations);
  } catch (error) {
    console.error('Error:', error);
  }
}

main();
```

## Next Steps

Now that you've learned how to use the Service API on the Neo N3 FaaS platform, you can:

1. Create more complex functions and services
2. Integrate with Neo N3 blockchain events
3. Use Oracle services for external data
4. Implement secure computing with TEE services
5. Build a custom dashboard using the Service API

For more information, see the [Documentation](../README.md).
