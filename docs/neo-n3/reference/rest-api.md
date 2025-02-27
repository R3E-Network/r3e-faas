# REST API Reference for Neo N3 FaaS Platform

This reference provides detailed information about the REST API for the Neo N3 FaaS platform.

## Table of Contents

1. [Introduction](#introduction)
2. [Authentication](#authentication)
3. [Error Handling](#error-handling)
4. [Endpoints](#endpoints)
5. [Examples](#examples)

## Introduction

The Neo N3 FaaS platform provides a RESTful API for managing functions, services, and other platform resources.

### Base URL

```
https://faas.example.com/api/v1
```

## Authentication

### API Key Authentication

```
X-API-Key: your-api-key
```

### JWT Authentication

```
Authorization: Bearer your-jwt-token
```

## Error Handling

Standard HTTP status codes:
- `200 OK`: Success
- `201 Created`: Resource created
- `400 Bad Request`: Invalid request
- `401 Unauthorized`: Authentication required
- `403 Forbidden`: Permission denied
- `404 Not Found`: Resource not found
- `500 Internal Server Error`: Server error

## Endpoints

### User Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/users/me` | GET | Get current user |
| `/users/{id}` | GET | Get user by ID |
| `/users` | GET | List users |
| `/users` | POST | Create user |
| `/users/{id}` | PUT | Update user |
| `/users/{id}` | DELETE | Delete user |

### Function Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/functions/{id}` | GET | Get function by ID |
| `/functions` | GET | List functions |
| `/functions` | POST | Create function |
| `/functions/{id}` | PUT | Update function |
| `/functions/{id}` | DELETE | Delete function |
| `/functions/{id}/deploy` | POST | Deploy function |
| `/functions/{id}/invoke` | POST | Invoke function |

### Service Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/services/{id}` | GET | Get service by ID |
| `/services` | GET | List services |
| `/services` | POST | Create service |
| `/services/{id}` | PUT | Update service |
| `/services/{id}` | DELETE | Delete service |
| `/services/{id}/deploy` | POST | Deploy service |

### Neo N3 Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/neo/blocks/{height}` | GET | Get block by height |
| `/neo/transactions/{hash}` | GET | Get transaction by hash |
| `/neo/contracts/{hash}` | GET | Get contract by hash |

### Oracle Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/oracle/prices/{asset}/{currency}` | GET | Get asset price |
| `/oracle/random` | GET | Get random number |

### TEE Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/tee/execute` | POST | Execute code in TEE |
| `/tee/attestation/{id}` | GET | Get attestation |

## Examples

### Authentication

```bash
# Get JWT token
curl -X POST https://faas.example.com/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "your-username", "password": "your-password"}'
```

### Create Function

```bash
# Create function
curl -X POST https://faas.example.com/api/v1/functions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your-jwt-token" \
  -d '{
    "name": "neo-block-monitor",
    "description": "Monitor Neo N3 blocks",
    "runtime": "JAVASCRIPT",
    "trigger": {
      "type": "NEO",
      "neo": {
        "event": "NEW_BLOCK"
      }
    },
    "code": "export default async function(event, context) { console.log(\"New block:\", event.data.blockHeight); }"
  }'
```

### Invoke Function

```bash
# Invoke function
curl -X POST https://faas.example.com/api/v1/functions/function-id/invoke \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your-jwt-token" \
  -d '{
    "params": {
      "name": "Neo"
    }
  }'
```

For more detailed information, see the [API Reference](../api-reference.md) and [Architecture](../architecture.md) documents.
