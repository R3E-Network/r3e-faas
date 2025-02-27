# Service API

This document provides detailed information about the Service API in the Neo N3 FaaS platform.

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [REST API](#rest-api)
4. [GraphQL API](#graphql-api)
5. [Authentication](#authentication)
6. [Service Management](#service-management)
7. [Function Management](#function-management)
8. [Monitoring and Logging](#monitoring-and-logging)
9. [API Reference](#api-reference)
10. [Best Practices](#best-practices)

## Overview

The Service API is a core component of the Neo N3 FaaS platform. It provides a comprehensive set of APIs for managing services, functions, and other platform resources. The Service API is designed to be flexible, secure, and easy to use, allowing developers to integrate the Neo N3 FaaS platform with their applications and workflows.

## Architecture

The Service API follows a modular architecture with several key components:

```
                      +------------------------+
                      |                        |
                      |   Service API          |
                      |                        |
                      +------------+-----------+
                                   |
                                   v
+----------------+    +------------+-----------+    +----------------+
|                |    |                        |    |                |
| REST API       |<-->|    API Service         |<-->| GraphQL API    |
|                |    |                        |    |                |
+----------------+    +------------+-----------+    +----------------+
                                   |
                                   v
+----------------+    +------------+-----------+    +----------------+
|                |    |                        |    |                |
| Authentication |<-->|    Service Registry    |<-->| Authorization  |
|                |    |                        |    |                |
+----------------+    +------------+-----------+    +----------------+
                                   |
                                   v
+----------------+    +------------+-----------+    +----------------+
|                |    |                        |    |                |
| Function Mgmt  |<-->|    Event System        |<-->| Service Mgmt   |
|                |    |                        |    |                |
+----------------+    +------------------------+    +----------------+
```

- **API Service**: The main component that provides API services to the platform.
- **REST API**: Provides RESTful endpoints for platform management.
- **GraphQL API**: Provides a GraphQL endpoint for flexible queries.
- **Authentication**: Authenticates users and services.
- **Authorization**: Controls access to platform resources.
- **Service Registry**: Manages service registration and discovery.
- **Event System**: Manages event triggers and handlers.
- **Function Management**: Manages function lifecycle.
- **Service Management**: Manages service lifecycle.

## REST API

The REST API provides a set of RESTful endpoints for managing platform resources. It follows REST principles and uses standard HTTP methods for operations.

### Endpoints

The REST API provides the following endpoints:

- `/api/v1/health`: Health check endpoint
- `/api/v1/auth`: Authentication endpoints
- `/api/v1/functions`: Function management endpoints
- `/api/v1/services`: Service management endpoints
- `/api/v1/events`: Event management endpoints
- `/api/v1/users`: User management endpoints

### Example Requests

#### Health Check

```http
GET /api/v1/health HTTP/1.1
Host: faas.example.com
```

Response:

```json
{
  "status": "ok",
  "version": "1.0.0",
  "uptime": 3600
}
```

#### Authentication

```http
POST /api/v1/auth/login HTTP/1.1
Host: faas.example.com
Content-Type: application/json

{
  "username": "user",
  "password": "password"
}
```

Response:

```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_at": "2023-01-01T00:00:00Z"
}
```

#### Function Management

```http
POST /api/v1/functions HTTP/1.1
Host: faas.example.com
Content-Type: application/json
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...

{
  "name": "hello-neo",
  "code": "export default async function(event, context) { return { message: 'Hello, Neo!' }; }",
  "trigger": {
    "type": "http",
    "path": "/hello-neo"
  }
}
```

Response:

```json
{
  "id": "function-123",
  "name": "hello-neo",
  "status": "created",
  "created_at": "2023-01-01T00:00:00Z"
}
```

#### Service Management

```http
POST /api/v1/services HTTP/1.1
Host: faas.example.com
Content-Type: application/json
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...

{
  "name": "price-oracle",
  "type": "oracle",
  "config": {
    "sources": ["coinGecko", "binance"],
    "assets": ["NEO", "GAS"]
  }
}
```

Response:

```json
{
  "id": "service-123",
  "name": "price-oracle",
  "status": "created",
  "created_at": "2023-01-01T00:00:00Z"
}
```

## GraphQL API

The GraphQL API provides a flexible and powerful way to query and mutate platform resources. It allows clients to request exactly the data they need, reducing over-fetching and under-fetching of data.

### Schema

The GraphQL API provides a comprehensive schema that includes types for all platform resources, including functions, services, events, and users.

```graphql
type Query {
  function(id: ID!): Function
  functions(limit: Int, offset: Int): [Function!]!
  service(id: ID!): Service
  services(limit: Int, offset: Int): [Service!]!
  event(id: ID!): Event
  events(limit: Int, offset: Int): [Event!]!
  user(id: ID!): User
  users(limit: Int, offset: Int): [User!]!
}

type Mutation {
  createFunction(input: CreateFunctionInput!): Function!
  updateFunction(id: ID!, input: UpdateFunctionInput!): Function!
  deleteFunction(id: ID!): Boolean!
  createService(input: CreateServiceInput!): Service!
  updateService(id: ID!, input: UpdateServiceInput!): Service!
  deleteService(id: ID!): Boolean!
  createEvent(input: CreateEventInput!): Event!
  updateEvent(id: ID!, input: UpdateEventInput!): Event!
  deleteEvent(id: ID!): Boolean!
  createUser(input: CreateUserInput!): User!
  updateUser(id: ID!, input: UpdateUserInput!): User!
  deleteUser(id: ID!): Boolean!
}

type Function {
  id: ID!
  name: String!
  code: String!
  trigger: Trigger!
  status: FunctionStatus!
  createdAt: DateTime!
  updatedAt: DateTime!
}

type Service {
  id: ID!
  name: String!
  type: ServiceType!
  config: JSONObject!
  status: ServiceStatus!
  createdAt: DateTime!
  updatedAt: DateTime!
}

type Event {
  id: ID!
  type: EventType!
  data: JSONObject!
  createdAt: DateTime!
}

type User {
  id: ID!
  username: String!
  email: String!
  roles: [Role!]!
  createdAt: DateTime!
  updatedAt: DateTime!
}

enum FunctionStatus {
  CREATED
  DEPLOYED
  RUNNING
  STOPPED
  FAILED
}

enum ServiceType {
  ORACLE
  TEE
  BLOCKCHAIN
  STANDARD
}

enum ServiceStatus {
  CREATED
  DEPLOYED
  RUNNING
  STOPPED
  FAILED
}

enum EventType {
  BLOCK
  TRANSACTION
  CONTRACT
  SCHEDULE
  CUSTOM
}

enum Role {
  ADMIN
  USER
  SERVICE
}

scalar DateTime
scalar JSONObject
```

### Example Queries

#### Get Function

```graphql
query GetFunction($id: ID!) {
  function(id: $id) {
    id
    name
    code
    trigger {
      type
      path
    }
    status
    createdAt
    updatedAt
  }
}
```

Variables:

```json
{
  "id": "function-123"
}
```

Response:

```json
{
  "data": {
    "function": {
      "id": "function-123",
      "name": "hello-neo",
      "code": "export default async function(event, context) { return { message: 'Hello, Neo!' }; }",
      "trigger": {
        "type": "http",
        "path": "/hello-neo"
      },
      "status": "RUNNING",
      "createdAt": "2023-01-01T00:00:00Z",
      "updatedAt": "2023-01-01T00:00:00Z"
    }
  }
}
```

#### Create Service

```graphql
mutation CreateService($input: CreateServiceInput!) {
  createService(input: $input) {
    id
    name
    type
    config
    status
    createdAt
    updatedAt
  }
}
```

Variables:

```json
{
  "input": {
    "name": "price-oracle",
    "type": "ORACLE",
    "config": {
      "sources": ["coinGecko", "binance"],
      "assets": ["NEO", "GAS"]
    }
  }
}
```

Response:

```json
{
  "data": {
    "createService": {
      "id": "service-123",
      "name": "price-oracle",
      "type": "ORACLE",
      "config": {
        "sources": ["coinGecko", "binance"],
        "assets": ["NEO", "GAS"]
      },
      "status": "CREATED",
      "createdAt": "2023-01-01T00:00:00Z",
      "updatedAt": "2023-01-01T00:00:00Z"
    }
  }
}
```

## Authentication

The Service API uses JSON Web Tokens (JWT) for authentication. Clients must include a valid JWT token in the `Authorization` header of their requests.

### Token-based Authentication

```http
POST /api/v1/auth/login HTTP/1.1
Host: faas.example.com
Content-Type: application/json

{
  "username": "user",
  "password": "password"
}
```

Response:

```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_at": "2023-01-01T00:00:00Z"
}
```

### API Key Authentication

```http
POST /api/v1/auth/api-key HTTP/1.1
Host: faas.example.com
Content-Type: application/json
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...

{
  "name": "my-api-key",
  "expires_at": "2023-12-31T23:59:59Z"
}
```

Response:

```json
{
  "api_key": "api-key-123",
  "name": "my-api-key",
  "expires_at": "2023-12-31T23:59:59Z"
}
```

### OAuth2 Authentication

The Service API also supports OAuth2 authentication for third-party applications.

```http
GET /api/v1/auth/oauth2/authorize?client_id=client-123&redirect_uri=https://example.com/callback&response_type=code&scope=read,write HTTP/1.1
Host: faas.example.com
```

After user authorization, the user is redirected to the specified `redirect_uri` with an authorization code:

```
https://example.com/callback?code=authorization-code-123
```

The client can then exchange the authorization code for an access token:

```http
POST /api/v1/auth/oauth2/token HTTP/1.1
Host: faas.example.com
Content-Type: application/x-www-form-urlencoded

grant_type=authorization_code&code=authorization-code-123&redirect_uri=https://example.com/callback&client_id=client-123&client_secret=client-secret-123
```

Response:

```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "bearer",
  "expires_in": 3600,
  "refresh_token": "refresh-token-123",
  "scope": "read,write"
}
```

## Service Management

The Service API provides comprehensive service management capabilities, allowing developers to create, update, delete, and query services.

### Service Types

The platform supports several types of services:

- **Oracle Services**: Provide external data to functions
- **TEE Services**: Provide secure execution environments
- **Blockchain Services**: Provide blockchain interaction
- **Standard Services**: General-purpose services

### Service Lifecycle

Services go through several stages in their lifecycle:

1. **Created**: Service is created but not deployed
2. **Deployed**: Service is deployed but not running
3. **Running**: Service is running and available
4. **Stopped**: Service is stopped but can be restarted
5. **Failed**: Service has failed and needs attention

### Service Configuration

Services are configured using a JSON configuration object that specifies the service's behavior and resources.

```json
{
  "name": "price-oracle",
  "type": "ORACLE",
  "config": {
    "sources": ["coinGecko", "binance"],
    "assets": ["NEO", "GAS"],
    "update_interval": 60,
    "cache_ttl": 300,
    "rate_limit": 100
  }
}
```

### Service Discovery

The platform provides service discovery capabilities, allowing services to discover and communicate with each other.

```http
GET /api/v1/services/discovery?type=ORACLE HTTP/1.1
Host: faas.example.com
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

Response:

```json
{
  "services": [
    {
      "id": "service-123",
      "name": "price-oracle",
      "type": "ORACLE",
      "endpoints": [
        {
          "type": "http",
          "url": "https://faas.example.com/services/price-oracle"
        },
        {
          "type": "grpc",
          "url": "grpc://faas.example.com:50051"
        }
      ]
    },
    {
      "id": "service-456",
      "name": "weather-oracle",
      "type": "ORACLE",
      "endpoints": [
        {
          "type": "http",
          "url": "https://faas.example.com/services/weather-oracle"
        }
      ]
    }
  ]
}
```

## Function Management

The Service API provides comprehensive function management capabilities, allowing developers to create, update, delete, and query functions.

### Function Types

The platform supports several types of functions:

- **HTTP Functions**: Triggered by HTTP requests
- **Event Functions**: Triggered by blockchain events
- **Schedule Functions**: Triggered by schedules
- **Custom Functions**: Triggered by custom events

### Function Lifecycle

Functions go through several stages in their lifecycle:

1. **Created**: Function is created but not deployed
2. **Deployed**: Function is deployed but not running
3. **Running**: Function is running and available
4. **Stopped**: Function is stopped but can be restarted
5. **Failed**: Function has failed and needs attention

### Function Configuration

Functions are configured using a JSON configuration object that specifies the function's behavior and resources.

```json
{
  "name": "hello-neo",
  "code": "export default async function(event, context) { return { message: 'Hello, Neo!' }; }",
  "trigger": {
    "type": "http",
    "path": "/hello-neo"
  },
  "resources": {
    "memory": 128,
    "cpu": 0.1,
    "timeout": 30
  },
  "environment": {
    "NODE_ENV": "production",
    "DEBUG": "false"
  }
}
```

### Function Invocation

Functions can be invoked directly through the API.

```http
POST /api/v1/functions/hello-neo/invoke HTTP/1.1
Host: faas.example.com
Content-Type: application/json
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...

{
  "name": "Neo"
}
```

Response:

```json
{
  "message": "Hello, Neo!"
}
```

## Monitoring and Logging

The Service API provides monitoring and logging capabilities, allowing developers to monitor the health and performance of their services and functions.

### Metrics

The platform collects metrics for services and functions, including CPU usage, memory usage, request count, error count, and latency.

```http
GET /api/v1/metrics/functions/hello-neo?start=2023-01-01T00:00:00Z&end=2023-01-02T00:00:00Z HTTP/1.1
Host: faas.example.com
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

Response:

```json
{
  "metrics": {
    "cpu_usage": [
      {
        "timestamp": "2023-01-01T00:00:00Z",
        "value": 0.1
      },
      {
        "timestamp": "2023-01-01T01:00:00Z",
        "value": 0.2
      }
    ],
    "memory_usage": [
      {
        "timestamp": "2023-01-01T00:00:00Z",
        "value": 64
      },
      {
        "timestamp": "2023-01-01T01:00:00Z",
        "value": 128
      }
    ],
    "request_count": [
      {
        "timestamp": "2023-01-01T00:00:00Z",
        "value": 100
      },
      {
        "timestamp": "2023-01-01T01:00:00Z",
        "value": 200
      }
    ],
    "error_count": [
      {
        "timestamp": "2023-01-01T00:00:00Z",
        "value": 0
      },
      {
        "timestamp": "2023-01-01T01:00:00Z",
        "value": 1
      }
    ],
    "latency": [
      {
        "timestamp": "2023-01-01T00:00:00Z",
        "value": 10
      },
      {
        "timestamp": "2023-01-01T01:00:00Z",
        "value": 20
      }
    ]
  }
}
```

### Logs

The platform collects logs for services and functions, allowing developers to debug issues and monitor activity.

```http
GET /api/v1/logs/functions/hello-neo?start=2023-01-01T00:00:00Z&end=2023-01-02T00:00:00Z HTTP/1.1
Host: faas.example.com
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

Response:

```json
{
  "logs": [
    {
      "timestamp": "2023-01-01T00:00:00Z",
      "level": "info",
      "message": "Function invoked",
      "function_id": "function-123",
      "request_id": "request-123"
    },
    {
      "timestamp": "2023-01-01T00:00:01Z",
      "level": "info",
      "message": "Function completed",
      "function_id": "function-123",
      "request_id": "request-123",
      "duration": 10
    }
  ]
}
```

### Alerts

The platform provides alerting capabilities, allowing developers to be notified of issues with their services and functions.

```http
POST /api/v1/alerts HTTP/1.1
Host: faas.example.com
Content-Type: application/json
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...

{
  "name": "high-error-rate",
  "target": {
    "type": "function",
    "id": "function-123"
  },
  "condition": {
    "metric": "error_count",
    "operator": ">",
    "threshold": 10,
    "window": 60
  },
  "actions": [
    {
      "type": "email",
      "recipient": "user@example.com"
    },
    {
      "type": "webhook",
      "url": "https://example.com/webhook"
    }
  ]
}
```

Response:

```json
{
  "id": "alert-123",
  "name": "high-error-rate",
  "status": "active",
  "created_at": "2023-01-01T00:00:00Z"
}
```

## API Reference

For detailed API reference, see the [API Reference](../api-reference.md) document.

## Best Practices

When using the Service API, follow these best practices:

### Authentication

- Use API keys for service-to-service communication
- Use OAuth2 for third-party applications
- Rotate API keys and tokens regularly
- Use scopes to limit access to resources

### Error Handling

- Handle errors gracefully
- Provide meaningful error messages
- Use appropriate HTTP status codes
- Log errors for debugging

### Rate Limiting

- Implement rate limiting to prevent abuse
- Use exponential backoff for retries
- Cache responses to reduce API calls
- Use bulk operations when possible

### Security

- Use HTTPS for all API calls
- Validate all input parameters
- Implement proper authorization
- Follow the principle of least privilege

### Monitoring

- Monitor API usage and performance
- Set up alerts for abnormal behavior
- Collect and analyze logs
- Track error rates and latency

### Documentation

- Keep API documentation up to date
- Provide examples for common use cases
- Document error codes and messages
- Use OpenAPI/Swagger for API documentation

### Versioning

- Use semantic versioning for API versions
- Support multiple API versions
- Deprecate old versions gracefully
- Communicate changes to users
