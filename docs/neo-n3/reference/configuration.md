# Configuration Reference for Neo N3 FaaS Platform

This reference provides detailed information about configuration options for the Neo N3 FaaS platform.

## Table of Contents

1. [Introduction](#introduction)
2. [Project Configuration](#project-configuration)
3. [Function Configuration](#function-configuration)
4. [Service Configuration](#service-configuration)
5. [Environment Configuration](#environment-configuration)
6. [Secrets Configuration](#secrets-configuration)
7. [Network Configuration](#network-configuration)
8. [Storage Configuration](#storage-configuration)
9. [Logging Configuration](#logging-configuration)
10. [Security Configuration](#security-configuration)

## Introduction

The Neo N3 FaaS platform uses YAML configuration files to define project settings, functions, services, and environments. The primary configuration file is `r3e.yaml`, which is located in the project root directory.

## Project Configuration

The project configuration section defines global settings for the project.

```yaml
# Project configuration
project:
  name: my-neo-faas-project
  version: 0.1.0
  description: My Neo N3 FaaS project
  author: John Doe
  email: john.doe@example.com
  license: MIT
  repository: https://github.com/johndoe/my-neo-faas-project
  keywords:
    - neo
    - faas
    - serverless
  dependencies:
    - name: neo-sdk
      version: 1.0.0
    - name: oracle-sdk
      version: 1.0.0
```

### Project Configuration Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `name` | String | Yes | The name of the project |
| `version` | String | Yes | The version of the project |
| `description` | String | No | A description of the project |
| `author` | String | No | The author of the project |
| `email` | String | No | The email of the author |
| `license` | String | No | The license of the project |
| `repository` | String | No | The repository URL of the project |
| `keywords` | Array | No | Keywords for the project |
| `dependencies` | Array | No | Dependencies for the project |

## Function Configuration

The function configuration section defines settings for functions.

```yaml
# Function configuration
functions:
  hello-world:
    handler: functions/hello-world.js
    runtime: javascript
    trigger:
      type: http
      path: /hello-world
      method: GET
      cors: true
    resources:
      memory: 128
      cpu: 0.1
      timeout: 30
    environment:
      NODE_ENV: production
      DEBUG: false
    deployment:
      strategy: rolling
      replicas: 3
      auto_scaling:
        min_replicas: 1
        max_replicas: 10
        target_cpu_utilization: 70
      regions:
        - us-east-1
        - eu-west-1
```

### Function Configuration Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `handler` | String | Yes | The path to the function handler |
| `runtime` | String | Yes | The runtime for the function (e.g., `javascript`) |
| `trigger` | Object | Yes | The trigger configuration for the function |
| `resources` | Object | No | The resource configuration for the function |
| `environment` | Object | No | Environment variables for the function |
| `deployment` | Object | No | Deployment configuration for the function |

### Trigger Configuration Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `type` | String | Yes | The type of trigger (e.g., `http`, `neo`, `schedule`) |
| `path` | String | Conditional | The path for HTTP triggers |
| `method` | String | Conditional | The HTTP method for HTTP triggers |
| `cors` | Boolean | Conditional | Whether to enable CORS for HTTP triggers |
| `event` | String | Conditional | The event type for Neo triggers |
| `contract` | String | Conditional | The contract address for Neo triggers |
| `schedule` | String | Conditional | The schedule expression for schedule triggers |

### Resource Configuration Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `memory` | Number | No | The memory allocation for the function in MB |
| `cpu` | Number | No | The CPU allocation for the function in cores |
| `timeout` | Number | No | The timeout for the function in seconds |

### Deployment Configuration Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `strategy` | String | No | The deployment strategy (e.g., `rolling`, `blue-green`) |
| `replicas` | Number | No | The number of replicas for the function |
| `auto_scaling` | Object | No | Auto-scaling configuration for the function |
| `regions` | Array | No | The regions to deploy the function to |

## Service Configuration

The service configuration section defines settings for services.

```yaml
# Service configuration
services:
  oracle-service:
    type: oracle
    config:
      sources: ["coinGecko", "binance"]
      assets: ["NEO", "GAS"]
      update_interval: 60
    resources:
      memory: 256
      cpu: 0.2
    environment:
      NODE_ENV: production
      DEBUG: false
    deployment:
      strategy: rolling
      replicas: 3
      auto_scaling:
        min_replicas: 1
        max_replicas: 10
        target_cpu_utilization: 70
      regions:
        - us-east-1
        - eu-west-1
```

### Service Configuration Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `type` | String | Yes | The type of service (e.g., `oracle`, `tee`, `blockchain`) |
| `config` | Object | Yes | The configuration for the service |
| `resources` | Object | No | The resource configuration for the service |
| `environment` | Object | No | Environment variables for the service |
| `deployment` | Object | No | Deployment configuration for the service |

### Oracle Service Configuration Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `sources` | Array | Yes | The data sources for the oracle service |
| `assets` | Array | Yes | The assets to track for the oracle service |
| `update_interval` | Number | No | The update interval in seconds |
| `cache_ttl` | Number | No | The cache time-to-live in seconds |
| `rate_limit` | Number | No | The rate limit in requests per minute |

### TEE Service Configuration Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `provider` | String | Yes | The TEE provider (e.g., `sgx`, `sev`, `trustzone`) |
| `enclave_path` | String | Conditional | The path to the enclave binary |
| `attestation` | Boolean | No | Whether to enable attestation |
| `key_management` | Boolean | No | Whether to enable key management |

### Blockchain Service Configuration Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `network` | String | Yes | The network (e.g., `mainnet`, `testnet`) |
| `rpc_url` | String | Yes | The RPC URL for the blockchain |
| `wallet_path` | String | Conditional | The path to the wallet file |
| `wallet_password` | String | Conditional | The wallet password (reference to secret) |

## Environment Configuration

The environment configuration section defines settings for environments.

```yaml
# Environment configuration
environments:
  development:
    network: testnet
    rpc_url: https://n3seed1.ngd.network:10332
    region: us-east-1
    log_level: debug
    debug: true
  
  staging:
    network: testnet
    rpc_url: https://n3seed1.ngd.network:10332
    region: us-east-1
    log_level: info
    debug: false
  
  production:
    network: mainnet
    rpc_url: https://n3seed1.ngd.network:10332
    region: us-east-1
    log_level: warn
    debug: false
```

### Environment Configuration Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `network` | String | Yes | The network for the environment |
| `rpc_url` | String | Yes | The RPC URL for the environment |
| `region` | String | No | The region for the environment |
| `log_level` | String | No | The log level for the environment |
| `debug` | Boolean | No | Whether to enable debug mode |

## Secrets Configuration

The secrets configuration section defines settings for secrets.

```yaml
# Secrets configuration
secrets:
  api_key: ${API_KEY}
  db_password: ${DB_PASSWORD}
  jwt_secret: ${JWT_SECRET}
```

### Secrets Configuration Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `api_key` | String | No | The API key |
| `db_password` | String | No | The database password |
| `jwt_secret` | String | No | The JWT secret |

## Network Configuration

The network configuration section defines settings for network.

```yaml
# Network configuration
network:
  domain: faas.example.com
  ssl:
    enabled: true
    certificate: /path/to/certificate.pem
    key: /path/to/key.pem
  firewall:
    enabled: true
    rules:
      - name: allow-http
        protocol: tcp
        port: 80
        source: 0.0.0.0/0
        action: allow
      - name: allow-https
        protocol: tcp
        port: 443
        source: 0.0.0.0/0
        action: allow
      - name: deny-all
        protocol: all
        port: all
        source: 0.0.0.0/0
        action: deny
```

### Network Configuration Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `domain` | String | No | The domain for the network |
| `ssl` | Object | No | The SSL configuration |
| `firewall` | Object | No | The firewall configuration |

## Storage Configuration

The storage configuration section defines settings for storage.

```yaml
# Storage configuration
storage:
  type: s3
  config:
    bucket: my-neo-faas-bucket
    region: us-east-1
    access_key: ${AWS_ACCESS_KEY}
    secret_key: ${AWS_SECRET_KEY}
```

### Storage Configuration Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `type` | String | Yes | The type of storage (e.g., `s3`, `local`) |
| `config` | Object | Yes | The configuration for the storage |

## Logging Configuration

The logging configuration section defines settings for logging.

```yaml
# Logging configuration
logging:
  level: info
  format: json
  output: stdout
  file:
    enabled: true
    path: /var/log/r3e-faas.log
    max_size: 10
    max_files: 5
  syslog:
    enabled: false
    facility: local0
    tag: r3e-faas
```

### Logging Configuration Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `level` | String | No | The log level (e.g., `debug`, `info`, `warn`, `error`) |
| `format` | String | No | The log format (e.g., `json`, `text`) |
| `output` | String | No | The log output (e.g., `stdout`, `file`) |
| `file` | Object | No | The file logging configuration |
| `syslog` | Object | No | The syslog logging configuration |

## Security Configuration

The security configuration section defines settings for security.

```yaml
# Security configuration
security:
  authentication:
    enabled: true
    providers:
      - type: jwt
        config:
          secret: ${JWT_SECRET}
          expiration: 3600
      - type: api_key
        config:
          header: X-API-Key
  authorization:
    enabled: true
    model: rbac
    roles:
      - name: admin
        permissions:
          - function:create
          - function:read
          - function:update
          - function:delete
      - name: developer
        permissions:
          - function:create
          - function:read
          - function:update
      - name: user
        permissions:
          - function:read
  encryption:
    enabled: true
    algorithm: aes-256-gcm
    key: ${ENCRYPTION_KEY}
```

### Security Configuration Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `authentication` | Object | No | The authentication configuration |
| `authorization` | Object | No | The authorization configuration |
| `encryption` | Object | No | The encryption configuration |

For more information, see the [API Reference](../api-reference.md) and [Architecture](../architecture.md) documents.
