# CLI Reference for Neo N3 FaaS Platform

This reference provides detailed information about CLI commands for the Neo N3 FaaS platform.

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Configuration](#configuration)
4. [Project Management](#project-management)
5. [Function Management](#function-management)
6. [Service Management](#service-management)
7. [Deployment](#deployment)
8. [Environment Management](#environment-management)
9. [Secrets Management](#secrets-management)
10. [Monitoring](#monitoring)

## Introduction

The Neo N3 FaaS platform provides a command-line interface (CLI) tool for managing your functions and services. The CLI tool is called `r3e-faas-cli` and is available for Windows, macOS, and Linux.

## Installation

### npm

```bash
npm install -g r3e-faas-cli
```

### yarn

```bash
yarn global add r3e-faas-cli
```

### pnpm

```bash
pnpm add -g r3e-faas-cli
```

### Verify Installation

```bash
r3e-faas-cli --version
```

## Configuration

### Login

```bash
# Interactive login
r3e-faas-cli login

# Login with username and password
r3e-faas-cli login --username your-username --password your-password

# Login with API key
r3e-faas-cli login --api-key your-api-key
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--username` | String | Conditional | The username for authentication |
| `--password` | String | Conditional | The password for authentication |
| `--api-key` | String | Conditional | The API key for authentication |

### Logout

```bash
r3e-faas-cli logout
```

### Configure

```bash
# Configure CLI
r3e-faas-cli config set --key api.url --value https://faas.example.com

# Get configuration
r3e-faas-cli config get --key api.url

# List configuration
r3e-faas-cli config list
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--key` | String | Yes | The configuration key |
| `--value` | String | Conditional | The configuration value |

## Project Management

### Initialize Project

```bash
# Initialize a new project
r3e-faas-cli init my-neo-faas-project

# Initialize a new project with specific options
r3e-faas-cli init my-neo-faas-project \
  --template javascript \
  --description "My Neo N3 FaaS project" \
  --author "John Doe" \
  --email "john.doe@example.com"
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--template` | String | No | The template to use (e.g., `javascript`, `typescript`) |
| `--description` | String | No | The description of the project |
| `--author` | String | No | The author of the project |
| `--email` | String | No | The email of the author |

### Validate Project

```bash
# Validate project
r3e-faas-cli validate
```

### List Projects

```bash
# List projects
r3e-faas-cli project list
```

### Get Project

```bash
# Get project
r3e-faas-cli project get --name my-neo-faas-project
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--name` | String | Yes | The name of the project |

### Delete Project

```bash
# Delete project
r3e-faas-cli project delete --name my-neo-faas-project
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--name` | String | Yes | The name of the project |

## Function Management

### Create Function

```bash
# Create a new function
r3e-faas-cli function create --name hello-world --template http

# Create a new function with specific options
r3e-faas-cli function create --name neo-info \
  --template neo \
  --trigger-type neo \
  --trigger-event NewBlock \
  --description "Get Neo blockchain information"
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--name` | String | Yes | The name of the function |
| `--template` | String | No | The template to use (e.g., `http`, `neo`, `schedule`) |
| `--trigger-type` | String | No | The trigger type (e.g., `http`, `neo`, `schedule`) |
| `--trigger-event` | String | Conditional | The trigger event (e.g., `NewBlock`, `NewTx`) |
| `--description` | String | No | The description of the function |

### List Functions

```bash
# List functions
r3e-faas-cli function list

# List functions with specific options
r3e-faas-cli function list --filter trigger-type=http
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--filter` | String | No | The filter to apply (e.g., `trigger-type=http`) |

### Get Function

```bash
# Get function
r3e-faas-cli function get --name hello-world
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--name` | String | Yes | The name of the function |

### Update Function

```bash
# Update function
r3e-faas-cli function update --name hello-world --memory 256 --cpu 0.2
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--name` | String | Yes | The name of the function |
| `--memory` | Number | No | The memory allocation for the function in MB |
| `--cpu` | Number | No | The CPU allocation for the function in cores |
| `--timeout` | Number | No | The timeout for the function in seconds |
| `--env-var` | String | No | Environment variables for the function (e.g., `NODE_ENV=production`) |

### Delete Function

```bash
# Delete function
r3e-faas-cli function delete --name hello-world
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--name` | String | Yes | The name of the function |

### Invoke Function

```bash
# Invoke function
r3e-faas-cli function invoke --name hello-world

# Invoke function with parameters
r3e-faas-cli function invoke --name hello-world --params '{"name": "Neo"}'

# Invoke function locally
r3e-faas-cli function invoke-local --name hello-world
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--name` | String | Yes | The name of the function |
| `--params` | String | No | The parameters for the function in JSON format |
| `--local` | Boolean | No | Whether to invoke the function locally |

### Logs

```bash
# Get function logs
r3e-faas-cli function logs --name hello-world

# Get function logs with specific options
r3e-faas-cli function logs --name hello-world --tail 100 --follow
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--name` | String | Yes | The name of the function |
| `--tail` | Number | No | The number of lines to show |
| `--follow` | Boolean | No | Whether to follow the logs |

### Metrics

```bash
# Get function metrics
r3e-faas-cli function metrics --name hello-world

# Get function metrics with specific options
r3e-faas-cli function metrics --name hello-world --period 1h
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--name` | String | Yes | The name of the function |
| `--period` | String | No | The period for the metrics (e.g., `1h`, `1d`, `1w`) |

## Service Management

### Create Service

```bash
# Create a new service
r3e-faas-cli service create --name oracle-service --type oracle

# Create a new service with specific options
r3e-faas-cli service create --name oracle-service \
  --type oracle \
  --config-file oracle-config.json \
  --description "Oracle service for price feeds"
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--name` | String | Yes | The name of the service |
| `--type` | String | Yes | The type of service (e.g., `oracle`, `tee`, `blockchain`) |
| `--config-file` | String | No | The path to the configuration file |
| `--description` | String | No | The description of the service |

### List Services

```bash
# List services
r3e-faas-cli service list

# List services with specific options
r3e-faas-cli service list --filter type=oracle
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--filter` | String | No | The filter to apply (e.g., `type=oracle`) |

### Get Service

```bash
# Get service
r3e-faas-cli service get --name oracle-service
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--name` | String | Yes | The name of the service |

### Update Service

```bash
# Update service
r3e-faas-cli service update --name oracle-service --replicas 3
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--name` | String | Yes | The name of the service |
| `--replicas` | Number | No | The number of replicas for the service |
| `--env-var` | String | No | Environment variables for the service (e.g., `NODE_ENV=production`) |

### Delete Service

```bash
# Delete service
r3e-faas-cli service delete --name oracle-service
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--name` | String | Yes | The name of the service |

### Logs

```bash
# Get service logs
r3e-faas-cli service logs --name oracle-service

# Get service logs with specific options
r3e-faas-cli service logs --name oracle-service --tail 100 --follow
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--name` | String | Yes | The name of the service |
| `--tail` | Number | No | The number of lines to show |
| `--follow` | Boolean | No | Whether to follow the logs |

### Metrics

```bash
# Get service metrics
r3e-faas-cli service metrics --name oracle-service

# Get service metrics with specific options
r3e-faas-cli service metrics --name oracle-service --period 1h
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--name` | String | Yes | The name of the service |
| `--period` | String | No | The period for the metrics (e.g., `1h`, `1d`, `1w`) |

## Deployment

### Build

```bash
# Build project
r3e-faas-cli build

# Build specific functions
r3e-faas-cli build --function hello-world --function neo-info

# Build specific services
r3e-faas-cli build --service oracle-service --service tee-service
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--function` | String | No | The function to build |
| `--service` | String | No | The service to build |

### Deploy

```bash
# Deploy project
r3e-faas-cli deploy

# Deploy specific functions
r3e-faas-cli deploy --function hello-world --function neo-info

# Deploy specific services
r3e-faas-cli deploy --service oracle-service --service tee-service

# Deploy to specific environment
r3e-faas-cli deploy --env production
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--function` | String | No | The function to deploy |
| `--service` | String | No | The service to deploy |
| `--env` | String | No | The environment to deploy to |
| `--memory` | Number | No | The memory allocation for the function in MB |
| `--cpu` | Number | No | The CPU allocation for the function in cores |
| `--timeout` | Number | No | The timeout for the function in seconds |
| `--env-var` | String | No | Environment variables for the function (e.g., `NODE_ENV=production`) |
| `--replicas` | Number | No | The number of replicas for the service |

### Status

```bash
# Get deployment status
r3e-faas-cli status

# Get deployment status for specific functions
r3e-faas-cli status --function hello-world

# Get deployment status for specific services
r3e-faas-cli status --service oracle-service
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--function` | String | No | The function to get status for |
| `--service` | String | No | The service to get status for |

### History

```bash
# Get deployment history
r3e-faas-cli history

# Get deployment history for specific functions
r3e-faas-cli history --function hello-world

# Get deployment history for specific services
r3e-faas-cli history --service oracle-service
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--function` | String | No | The function to get history for |
| `--service` | String | No | The service to get history for |

### Rollback

```bash
# Rollback to previous version
r3e-faas-cli rollback

# Rollback to specific version
r3e-faas-cli rollback --version 1.0.0

# Rollback specific functions
r3e-faas-cli rollback --function hello-world --version 1.0.0

# Rollback specific services
r3e-faas-cli rollback --service oracle-service --version 1.0.0
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--function` | String | No | The function to rollback |
| `--service` | String | No | The service to rollback |
| `--version` | String | No | The version to rollback to |

## Environment Management

### Create Environment

```bash
# Create a new environment
r3e-faas-cli env create development

# Create a new environment with specific options
r3e-faas-cli env create staging \
  --region us-east-1 \
  --network testnet \
  --rpc-url https://n3seed1.ngd.network:10332
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--region` | String | No | The region for the environment |
| `--network` | String | No | The network for the environment |
| `--rpc-url` | String | No | The RPC URL for the environment |
| `--log-level` | String | No | The log level for the environment |
| `--debug` | Boolean | No | Whether to enable debug mode |

### List Environments

```bash
# List environments
r3e-faas-cli env list
```

### Get Environment

```bash
# Get environment
r3e-faas-cli env get --name development
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--name` | String | Yes | The name of the environment |

### Update Environment

```bash
# Update environment
r3e-faas-cli env update --name development --log-level debug
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--name` | String | Yes | The name of the environment |
| `--region` | String | No | The region for the environment |
| `--network` | String | No | The network for the environment |
| `--rpc-url` | String | No | The RPC URL for the environment |
| `--log-level` | String | No | The log level for the environment |
| `--debug` | Boolean | No | Whether to enable debug mode |

### Delete Environment

```bash
# Delete environment
r3e-faas-cli env delete --name development
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--name` | String | Yes | The name of the environment |

### Use Environment

```bash
# Use environment
r3e-faas-cli env use --name production
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--name` | String | Yes | The name of the environment |

## Secrets Management

### Create Secret

```bash
# Create a new secret
r3e-faas-cli secret create API_KEY "your-api-key"

# Create a new secret from file
r3e-faas-cli secret create JWT_PRIVATE_KEY --file jwt-private.key
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--file` | String | Conditional | The path to the file containing the secret value |

### List Secrets

```bash
# List secrets
r3e-faas-cli secret list
```

### Get Secret

```bash
# Get secret
r3e-faas-cli secret get --name API_KEY
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--name` | String | Yes | The name of the secret |

### Update Secret

```bash
# Update secret
r3e-faas-cli secret update API_KEY "your-new-api-key"

# Update secret from file
r3e-faas-cli secret update JWT_PRIVATE_KEY --file jwt-private-new.key
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--file` | String | Conditional | The path to the file containing the secret value |

### Delete Secret

```bash
# Delete secret
r3e-faas-cli secret delete --name API_KEY
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--name` | String | Yes | The name of the secret |

## Monitoring

### Logs

```bash
# Get logs
r3e-faas-cli logs

# Get logs for specific functions
r3e-faas-cli logs --function hello-world

# Get logs for specific services
r3e-faas-cli logs --service oracle-service

# Get logs with specific options
r3e-faas-cli logs --tail 100 --follow --level error
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--function` | String | No | The function to get logs for |
| `--service` | String | No | The service to get logs for |
| `--tail` | Number | No | The number of lines to show |
| `--follow` | Boolean | No | Whether to follow the logs |
| `--level` | String | No | The log level to filter by |

### Metrics

```bash
# Get metrics
r3e-faas-cli metrics

# Get metrics for specific functions
r3e-faas-cli metrics --function hello-world

# Get metrics for specific services
r3e-faas-cli metrics --service oracle-service

# Get metrics with specific options
r3e-faas-cli metrics --period 1h --format json
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--function` | String | No | The function to get metrics for |
| `--service` | String | No | The service to get metrics for |
| `--period` | String | No | The period for the metrics (e.g., `1h`, `1d`, `1w`) |
| `--format` | String | No | The format for the metrics (e.g., `json`, `text`) |

### Health

```bash
# Get health
r3e-faas-cli health

# Get health with specific options
r3e-faas-cli health --format json
```

#### Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--format` | String | No | The format for the health (e.g., `json`, `text`) |

For more information, see the [API Reference](../api-reference.md) and [Architecture](../architecture.md) documents.
