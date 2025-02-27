# Neo N3 FaaS Platform API Reference

This document provides a comprehensive reference for the Neo N3 FaaS platform APIs, including REST and GraphQL endpoints, authentication, and request/response formats.

## Table of Contents

1. [Authentication](#authentication)
2. [REST API](#rest-api)
   - [Service Management](#service-management)
   - [Function Management](#function-management)
   - [Oracle Services](#oracle-services)
   - [TEE Services](#tee-services)
3. [GraphQL API](#graphql-api)
   - [Queries](#queries)
   - [Mutations](#mutations)
   - [Types](#types)
4. [JavaScript SDK](#javascript-sdk)
5. [CLI Reference](#cli-reference)

## Authentication

The Neo N3 FaaS platform uses JWT (JSON Web Token) for authentication. To authenticate, you need to obtain a JWT token by calling the `/auth/login` endpoint with your credentials.

### Obtaining a JWT Token

**Request:**

```http
POST /auth/login
Content-Type: application/json

{
  "username": "your-username",
  "password": "your-password"
}
```

**Response:**

```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_in": 3600
}
```

### Using the JWT Token

Include the JWT token in the `Authorization` header of your requests:

```http
GET /services
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

## REST API

### Service Management

#### List Services

**Request:**

```http
GET /services
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Response:**

```json
{
  "services": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "price-oracle",
      "description": "Price oracle service for Neo N3",
      "service_type": "oracle",
      "status": "active",
      "visibility": "public",
      "created_at": "2025-01-01T00:00:00Z",
      "updated_at": "2025-01-01T00:00:00Z"
    }
  ],
  "total": 1,
  "page": 1,
  "page_size": 10
}
```

#### Get Service

**Request:**

```http
GET /services/{service_id}
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Response:**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "price-oracle",
  "description": "Price oracle service for Neo N3",
  "service_type": "oracle",
  "config": {
    "data_sources": ["binance", "coinbase"],
    "update_interval": 60
  },
  "status": "active",
  "visibility": "public",
  "version": "1.0.0",
  "created_at": "2025-01-01T00:00:00Z",
  "updated_at": "2025-01-01T00:00:00Z"
}
```

#### Create Service

**Request:**

```http
POST /services
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
Content-Type: application/json

{
  "name": "price-oracle",
  "description": "Price oracle service for Neo N3",
  "service_type": "oracle",
  "config": {
    "data_sources": ["binance", "coinbase"],
    "update_interval": 60
  },
  "visibility": "public"
}
```

**Response:**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "price-oracle",
  "description": "Price oracle service for Neo N3",
  "service_type": "oracle",
  "config": {
    "data_sources": ["binance", "coinbase"],
    "update_interval": 60
  },
  "status": "active",
  "visibility": "public",
  "version": "1.0.0",
  "created_at": "2025-01-01T00:00:00Z",
  "updated_at": "2025-01-01T00:00:00Z"
}
```

#### Update Service

**Request:**

```http
PUT /services/{service_id}
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
Content-Type: application/json

{
  "description": "Updated price oracle service for Neo N3",
  "config": {
    "data_sources": ["binance", "coinbase", "kraken"],
    "update_interval": 30
  },
  "status": "active"
}
```

**Response:**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "price-oracle",
  "description": "Updated price oracle service for Neo N3",
  "service_type": "oracle",
  "config": {
    "data_sources": ["binance", "coinbase", "kraken"],
    "update_interval": 30
  },
  "status": "active",
  "visibility": "public",
  "version": "1.0.1",
  "created_at": "2025-01-01T00:00:00Z",
  "updated_at": "2025-01-02T00:00:00Z"
}
```

#### Delete Service

**Request:**

```http
DELETE /services/{service_id}
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Response:**

```json
{
  "success": true,
  "message": "Service deleted successfully"
}
```

### Function Management

#### List Functions

**Request:**

```http
GET /functions
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Response:**

```json
{
  "functions": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440001",
      "service_id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "get-neo-price",
      "description": "Get the current price of NEO",
      "trigger_type": "http",
      "status": "active",
      "created_at": "2025-01-01T00:00:00Z",
      "updated_at": "2025-01-01T00:00:00Z"
    }
  ],
  "total": 1,
  "page": 1,
  "page_size": 10
}
```

#### Get Function

**Request:**

```http
GET /functions/{function_id}
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Response:**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "service_id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "get-neo-price",
  "description": "Get the current price of NEO",
  "code": "export default async function(event, context) {\n  const price = await context.oracle.getPrice('NEO', 'USD');\n  return { price };\n}",
  "runtime": "javascript",
  "trigger_type": "http",
  "trigger_config": {
    "path": "/get-neo-price",
    "method": "GET"
  },
  "security_level": "standard",
  "status": "active",
  "version": "1.0.0",
  "hash": "sha256:1234567890abcdef",
  "created_at": "2025-01-01T00:00:00Z",
  "updated_at": "2025-01-01T00:00:00Z"
}
```

#### Create Function

**Request:**

```http
POST /functions
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
Content-Type: application/json

{
  "service_id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "get-neo-price",
  "description": "Get the current price of NEO",
  "code": "export default async function(event, context) {\n  const price = await context.oracle.getPrice('NEO', 'USD');\n  return { price };\n}",
  "runtime": "javascript",
  "trigger_type": "http",
  "trigger_config": {
    "path": "/get-neo-price",
    "method": "GET"
  },
  "security_level": "standard"
}
```

**Response:**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "service_id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "get-neo-price",
  "description": "Get the current price of NEO",
  "code": "export default async function(event, context) {\n  const price = await context.oracle.getPrice('NEO', 'USD');\n  return { price };\n}",
  "runtime": "javascript",
  "trigger_type": "http",
  "trigger_config": {
    "path": "/get-neo-price",
    "method": "GET"
  },
  "security_level": "standard",
  "status": "active",
  "version": "1.0.0",
  "hash": "sha256:1234567890abcdef",
  "created_at": "2025-01-01T00:00:00Z",
  "updated_at": "2025-01-01T00:00:00Z"
}
```

#### Update Function

**Request:**

```http
PUT /functions/{function_id}
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
Content-Type: application/json

{
  "description": "Get the current price of NEO in USD",
  "code": "export default async function(event, context) {\n  const price = await context.oracle.getPrice('NEO', 'USD');\n  return { price, timestamp: new Date().toISOString() };\n}",
  "status": "active"
}
```

**Response:**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "service_id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "get-neo-price",
  "description": "Get the current price of NEO in USD",
  "code": "export default async function(event, context) {\n  const price = await context.oracle.getPrice('NEO', 'USD');\n  return { price, timestamp: new Date().toISOString() };\n}",
  "runtime": "javascript",
  "trigger_type": "http",
  "trigger_config": {
    "path": "/get-neo-price",
    "method": "GET"
  },
  "security_level": "standard",
  "status": "active",
  "version": "1.0.1",
  "hash": "sha256:0987654321fedcba",
  "created_at": "2025-01-01T00:00:00Z",
  "updated_at": "2025-01-02T00:00:00Z"
}
```

#### Delete Function

**Request:**

```http
DELETE /functions/{function_id}
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Response:**

```json
{
  "success": true,
  "message": "Function deleted successfully"
}
```

#### Invoke Function

**Request:**

```http
POST /functions/{function_id}/invoke
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
Content-Type: application/json

{
  "params": {
    "currency": "USD"
  }
}
```

**Response:**

```json
{
  "result": {
    "price": 42.5,
    "timestamp": "2025-01-02T00:00:00Z"
  },
  "execution_time_ms": 123
}
```

### Oracle Services

#### Get Price

**Request:**

```http
GET /oracle/price?base=NEO&quote=USD
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Response:**

```json
{
  "base": "NEO",
  "quote": "USD",
  "price": 42.5,
  "timestamp": "2025-01-02T00:00:00Z",
  "sources": ["binance", "coinbase"]
}
```

#### Get Random Number

**Request:**

```http
GET /oracle/random?min=1&max=100
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Response:**

```json
{
  "value": 42,
  "min": 1,
  "max": 100,
  "timestamp": "2025-01-02T00:00:00Z",
  "proof": "sha256:1234567890abcdef"
}
```

### TEE Services

#### Create TEE Environment

**Request:**

```http
POST /tee/environments
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
Content-Type: application/json

{
  "name": "secure-wallet",
  "description": "Secure wallet environment for Neo N3",
  "provider": "sgx",
  "config": {
    "memory_size": 128,
    "cpu_count": 2
  }
}
```

**Response:**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440002",
  "name": "secure-wallet",
  "description": "Secure wallet environment for Neo N3",
  "provider": "sgx",
  "config": {
    "memory_size": 128,
    "cpu_count": 2
  },
  "status": "initializing",
  "attestation": null,
  "created_at": "2025-01-02T00:00:00Z",
  "updated_at": "2025-01-02T00:00:00Z"
}
```

#### Get TEE Environment

**Request:**

```http
GET /tee/environments/{environment_id}
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Response:**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440002",
  "name": "secure-wallet",
  "description": "Secure wallet environment for Neo N3",
  "provider": "sgx",
  "config": {
    "memory_size": 128,
    "cpu_count": 2
  },
  "status": "active",
  "attestation": {
    "quote": "base64-encoded-quote",
    "signature": "base64-encoded-signature",
    "timestamp": "2025-01-02T00:00:00Z"
  },
  "created_at": "2025-01-02T00:00:00Z",
  "updated_at": "2025-01-02T00:00:00Z"
}
```

#### Execute in TEE Environment

**Request:**

```http
POST /tee/environments/{environment_id}/execute
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
Content-Type: application/json

{
  "code": "export default async function(event, context) {\n  const wallet = await context.neo.createWallet();\n  return { address: wallet.address };\n}",
  "params": {}
}
```

**Response:**

```json
{
  "result": {
    "address": "NXV7ZhHiyM1aHXwvUBCta7dGNP1tHwfoQG"
  },
  "execution_time_ms": 234,
  "attestation": {
    "quote": "base64-encoded-quote",
    "signature": "base64-encoded-signature",
    "timestamp": "2025-01-02T00:00:00Z"
  }
}
```

## GraphQL API

The Neo N3 FaaS platform provides a GraphQL API that allows you to query and mutate data in a more flexible way than the REST API. The GraphQL API is available at the `/graphql` endpoint.

### Queries

#### Get Services

```graphql
query GetServices {
  services(page: 1, pageSize: 10) {
    id
    name
    description
    serviceType
    status
    visibility
    createdAt
    updatedAt
  }
}
```

#### Get Service

```graphql
query GetService($id: ID!) {
  service(id: $id) {
    id
    name
    description
    serviceType
    config
    status
    visibility
    version
    createdAt
    updatedAt
    functions {
      id
      name
      description
      triggerType
      status
    }
  }
}
```

#### Get Functions

```graphql
query GetFunctions {
  functions(page: 1, pageSize: 10) {
    id
    serviceId
    name
    description
    triggerType
    status
    createdAt
    updatedAt
  }
}
```

#### Get Function

```graphql
query GetFunction($id: ID!) {
  function(id: $id) {
    id
    serviceId
    name
    description
    code
    runtime
    triggerType
    triggerConfig
    securityLevel
    status
    version
    hash
    createdAt
    updatedAt
  }
}
```

### Mutations

#### Create Service

```graphql
mutation CreateService($input: ServiceInput!) {
  createService(input: $input) {
    success
    message
    service {
      id
      name
      description
      serviceType
      config
      status
      visibility
      version
      createdAt
      updatedAt
    }
  }
}
```

#### Update Service

```graphql
mutation UpdateService($id: ID!, $input: ServiceInput!) {
  updateService(id: $id, input: $input) {
    success
    message
    service {
      id
      name
      description
      serviceType
      config
      status
      visibility
      version
      createdAt
      updatedAt
    }
  }
}
```

#### Delete Service

```graphql
mutation DeleteService($id: ID!) {
  deleteService(id: $id) {
    success
    message
  }
}
```

#### Create Function

```graphql
mutation CreateFunction($input: FunctionInput!) {
  createFunction(input: $input) {
    success
    message
    function {
      id
      serviceId
      name
      description
      code
      runtime
      triggerType
      triggerConfig
      securityLevel
      status
      version
      hash
      createdAt
      updatedAt
    }
  }
}
```

#### Update Function

```graphql
mutation UpdateFunction($id: ID!, $input: FunctionInput!) {
  updateFunction(id: $id, input: $input) {
    success
    message
    function {
      id
      serviceId
      name
      description
      code
      runtime
      triggerType
      triggerConfig
      securityLevel
      status
      version
      hash
      createdAt
      updatedAt
    }
  }
}
```

#### Delete Function

```graphql
mutation DeleteFunction($id: ID!) {
  deleteFunction(id: $id) {
    success
    message
  }
}
```

#### Invoke Function

```graphql
mutation InvokeFunction($id: ID!, $params: JSON) {
  invokeFunction(id: $id, params: $params) {
    success
    message
    function {
      id
      name
    }
    invocationResult
    executionTimeMs
  }
}
```

### Types

For a complete list of GraphQL types, see the GraphQL schema documentation available at the `/playground` endpoint.

## JavaScript SDK

The Neo N3 FaaS platform provides a JavaScript SDK that makes it easy to interact with the platform from your JavaScript applications.

### Installation

```bash
npm install r3e-faas-sdk
```

### Usage

```javascript
import { R3EFaaSClient } from 'r3e-faas-sdk';

// Create a client
const client = new R3EFaaSClient({
  endpoint: 'https://faas.example.com',
  token: 'your-jwt-token'
});

// List services
const services = await client.services.list();

// Create a service
const service = await client.services.create({
  name: 'price-oracle',
  description: 'Price oracle service for Neo N3',
  serviceType: 'oracle',
  config: {
    dataSources: ['binance', 'coinbase'],
    updateInterval: 60
  },
  visibility: 'public'
});

// Create a function
const func = await client.functions.create({
  serviceId: service.id,
  name: 'get-neo-price',
  description: 'Get the current price of NEO',
  code: `export default async function(event, context) {
  const price = await context.oracle.getPrice('NEO', 'USD');
  return { price };
}`,
  runtime: 'javascript',
  triggerType: 'http',
  triggerConfig: {
    path: '/get-neo-price',
    method: 'GET'
  },
  securityLevel: 'standard'
});

// Invoke a function
const result = await client.functions.invoke(func.id, {
  currency: 'USD'
});
```

## CLI Reference

The Neo N3 FaaS platform provides a command-line interface (CLI) that makes it easy to interact with the platform from your terminal.

### Installation

```bash
npm install -g r3e-faas-cli
```

### Authentication

```bash
r3e-faas-cli login
```

### Service Management

```bash
# List services
r3e-faas-cli services list

# Get service
r3e-faas-cli services get <service-id>

# Create service
r3e-faas-cli services create --name price-oracle --description "Price oracle service for Neo N3" --type oracle --config '{"dataSources": ["binance", "coinbase"], "updateInterval": 60}' --visibility public

# Update service
r3e-faas-cli services update <service-id> --description "Updated price oracle service for Neo N3" --config '{"dataSources": ["binance", "coinbase", "kraken"], "updateInterval": 30}'

# Delete service
r3e-faas-cli services delete <service-id>
```

### Function Management

```bash
# List functions
r3e-faas-cli functions list

# Get function
r3e-faas-cli functions get <function-id>

# Create function
r3e-faas-cli functions create --service-id <service-id> --name get-neo-price --description "Get the current price of NEO" --file ./get-neo-price.js --trigger-type http --trigger-config '{"path": "/get-neo-price", "method": "GET"}' --security-level standard

# Update function
r3e-faas-cli functions update <function-id> --description "Get the current price of NEO in USD" --file ./updated-get-neo-price.js

# Delete function
r3e-faas-cli functions delete <function-id>

# Invoke function
r3e-faas-cli functions invoke <function-id> --params '{"currency": "USD"}'
```

### Project Management

```bash
# Initialize a new project
r3e-faas-cli init my-neo-faas-project

# Deploy a project
r3e-faas-cli deploy

# Get project status
r3e-faas-cli status
```

For more information about the CLI, run `r3e-faas-cli help` or `r3e-faas-cli <command> help`.
