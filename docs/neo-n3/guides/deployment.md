# Deployment Guide for Neo N3 FaaS Platform

This guide provides detailed information about deploying functions and services on the Neo N3 FaaS platform.

## Table of Contents

1. [Introduction](#introduction)
2. [Deployment Prerequisites](#deployment-prerequisites)
3. [Deployment Options](#deployment-options)
4. [Deploying Functions](#deploying-functions)
5. [Deploying Services](#deploying-services)
6. [Configuration Management](#configuration-management)
7. [Environment Management](#environment-management)
8. [Continuous Integration and Deployment](#continuous-integration-and-deployment)
9. [Monitoring Deployments](#monitoring-deployments)
10. [Troubleshooting](#troubleshooting)

## Introduction

Deploying functions and services on the Neo N3 FaaS platform is a straightforward process that can be accomplished using the platform's CLI tool or API. This guide will walk you through the deployment process, from preparing your project for deployment to monitoring your deployed functions and services.

## Deployment Prerequisites

Before deploying functions and services on the Neo N3 FaaS platform, ensure you have the following:

- **Neo N3 FaaS CLI**: Install the CLI tool using `npm install -g r3e-faas-cli`
- **Project Configuration**: A valid `r3e.yaml` file in your project root
- **Neo N3 Wallet**: A Neo N3 wallet with GAS for transaction fees
- **Authentication**: Valid credentials for the Neo N3 FaaS platform

### Authentication Setup

```bash
# Log in to the Neo N3 FaaS platform
r3e-faas-cli login

# Or provide credentials directly
r3e-faas-cli login --username your-username --password your-password

# Or use API key
r3e-faas-cli login --api-key your-api-key
```

### Project Structure

Ensure your project follows the recommended structure:

```
my-neo-faas-project/
├── functions/
│   ├── function1.js
│   └── function2.js
├── services/
│   ├── service1/
│   │   ├── src/
│   │   └── Cargo.toml
│   └── service2/
│       ├── src/
│       └── Cargo.toml
├── r3e.yaml
├── .env
└── package.json
```

## Deployment Options

The Neo N3 FaaS platform offers several deployment options:

### Local Deployment

Local deployment is useful for testing and development. It deploys functions and services to your local machine.

```bash
# Deploy locally
r3e-faas-cli deploy --local
```

### Development Environment

Development environment deployment is useful for testing and integration. It deploys functions and services to a shared development environment.

```bash
# Deploy to development environment
r3e-faas-cli deploy --env development
```

### Staging Environment

Staging environment deployment is useful for pre-production testing. It deploys functions and services to a staging environment that mirrors production.

```bash
# Deploy to staging environment
r3e-faas-cli deploy --env staging
```

### Production Environment

Production environment deployment deploys functions and services to the production environment.

```bash
# Deploy to production environment
r3e-faas-cli deploy --env production
```

### Custom Environment

Custom environment deployment deploys functions and services to a custom environment.

```bash
# Deploy to custom environment
r3e-faas-cli deploy --env custom --url https://custom.example.com
```

## Deploying Functions

### Deploying a Single Function

```bash
# Deploy a single function
r3e-faas-cli deploy --function hello-world
```

### Deploying Multiple Functions

```bash
# Deploy multiple functions
r3e-faas-cli deploy --function hello-world --function neo-info
```

### Deploying All Functions

```bash
# Deploy all functions
r3e-faas-cli deploy --all-functions
```

### Function Deployment Options

```bash
# Deploy with specific options
r3e-faas-cli deploy --function hello-world \
  --memory 256 \
  --cpu 0.2 \
  --timeout 60 \
  --env-var NODE_ENV=production \
  --env-var DEBUG=false
```

### Function Deployment Configuration

Function deployment can be configured in the `r3e.yaml` file:

```yaml
functions:
  hello-world:
    handler: functions/hello-world.js
    runtime: javascript
    trigger:
      type: http
      path: /hello-world
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

## Deploying Services

### Deploying a Single Service

```bash
# Deploy a single service
r3e-faas-cli deploy --service oracle-service
```

### Deploying Multiple Services

```bash
# Deploy multiple services
r3e-faas-cli deploy --service oracle-service --service tee-service
```

### Deploying All Services

```bash
# Deploy all services
r3e-faas-cli deploy --all-services
```

### Service Deployment Options

```bash
# Deploy with specific options
r3e-faas-cli deploy --service oracle-service \
  --replicas 3 \
  --env-var NODE_ENV=production \
  --env-var DEBUG=false
```

### Service Deployment Configuration

Service deployment can be configured in the `r3e.yaml` file:

```yaml
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

## Configuration Management

### Environment Variables

Environment variables can be defined in several ways:

1. **In the `r3e.yaml` file**:

```yaml
functions:
  hello-world:
    environment:
      NODE_ENV: production
      DEBUG: false
```

2. **In a `.env` file**:

```
NODE_ENV=production
DEBUG=false
```

3. **On the command line**:

```bash
r3e-faas-cli deploy --env-var NODE_ENV=production --env-var DEBUG=false
```

4. **Using the platform's secrets management**:

```bash
# Create a secret
r3e-faas-cli secret create API_KEY "your-api-key"

# Use the secret in your function
r3e-faas-cli deploy --env-var API_KEY=${API_KEY}
```

### Configuration Files

Configuration files can be used to manage complex configurations:

1. **Create a configuration file**:

```json
{
  "api": {
    "url": "https://api.example.com",
    "timeout": 30,
    "retry": 3
  },
  "database": {
    "host": "db.example.com",
    "port": 5432,
    "username": "user",
    "password": "${DB_PASSWORD}"
  }
}
```

2. **Deploy with the configuration file**:

```bash
r3e-faas-cli deploy --config-file config.json
```

3. **Access the configuration in your function**:

```javascript
export default async function(event, context) {
  const config = context.config;
  const apiUrl = config.api.url;
  const dbHost = config.database.host;
  
  // Use the configuration
  
  return { success: true };
}
```

## Environment Management

### Creating Environments

```bash
# Create a new environment
r3e-faas-cli env create development

# Create an environment with specific settings
r3e-faas-cli env create staging \
  --region us-east-1 \
  --network testnet \
  --rpc-url https://n3seed1.ngd.network:10332
```

### Listing Environments

```bash
# List all environments
r3e-faas-cli env list
```

### Switching Environments

```bash
# Switch to a different environment
r3e-faas-cli env use production
```

### Environment Configuration

Environment configuration can be defined in the `r3e.yaml` file:

```yaml
environments:
  development:
    network: testnet
    rpc_url: https://n3seed1.ngd.network:10332
    region: us-east-1
  
  staging:
    network: testnet
    rpc_url: https://n3seed1.ngd.network:10332
    region: us-east-1
  
  production:
    network: mainnet
    rpc_url: https://n3seed1.ngd.network:10332
    region: us-east-1
```

## Continuous Integration and Deployment

### GitHub Actions

```yaml
# .github/workflows/deploy.yml
name: Deploy

on:
  push:
    branches:
      - main

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Setup Node.js
        uses: actions/setup-node@v2
        with:
          node-version: '16'
      
      - name: Install dependencies
        run: npm install
      
      - name: Install R3E FaaS CLI
        run: npm install -g r3e-faas-cli
      
      - name: Login to R3E FaaS
        run: r3e-faas-cli login --api-key ${{ secrets.R3E_FAAS_API_KEY }}
      
      - name: Deploy
        run: r3e-faas-cli deploy --env production
```

### GitLab CI/CD

```yaml
# .gitlab-ci.yml
stages:
  - deploy

deploy:
  stage: deploy
  image: node:16
  script:
    - npm install
    - npm install -g r3e-faas-cli
    - r3e-faas-cli login --api-key $R3E_FAAS_API_KEY
    - r3e-faas-cli deploy --env production
  only:
    - main
```

### Jenkins Pipeline

```groovy
// Jenkinsfile
pipeline {
    agent {
        docker {
            image 'node:16'
        }
    }
    
    stages {
        stage('Install') {
            steps {
                sh 'npm install'
                sh 'npm install -g r3e-faas-cli'
            }
        }
        
        stage('Login') {
            steps {
                sh 'r3e-faas-cli login --api-key ${R3E_FAAS_API_KEY}'
            }
        }
        
        stage('Deploy') {
            steps {
                sh 'r3e-faas-cli deploy --env production'
            }
        }
    }
}
```

## Monitoring Deployments

### Deployment Status

```bash
# Check deployment status
r3e-faas-cli status
```

### Deployment Logs

```bash
# View deployment logs
r3e-faas-cli logs --deployment deployment-123
```

### Deployment History

```bash
# View deployment history
r3e-faas-cli history
```

### Deployment Metrics

```bash
# View deployment metrics
r3e-faas-cli metrics --deployment deployment-123
```

## Troubleshooting

### Common Deployment Issues

#### Authentication Issues

```
Error: Authentication failed
```

**Solution**: Ensure you are logged in with valid credentials.

```bash
r3e-faas-cli login
```

#### Configuration Issues

```
Error: Invalid configuration in r3e.yaml
```

**Solution**: Validate your `r3e.yaml` file.

```bash
r3e-faas-cli validate
```

#### Resource Issues

```
Error: Insufficient resources
```

**Solution**: Reduce resource requirements or request more resources.

```bash
r3e-faas-cli deploy --function hello-world --memory 128 --cpu 0.1
```

#### Network Issues

```
Error: Unable to connect to the Neo N3 FaaS platform
```

**Solution**: Check your network connection and the platform's status.

```bash
r3e-faas-cli status --platform
```

#### Deployment Timeout

```
Error: Deployment timed out
```

**Solution**: Increase the deployment timeout.

```bash
r3e-faas-cli deploy --timeout 300
```

### Rollback Deployments

If a deployment fails or causes issues, you can roll back to a previous version:

```bash
# List deployment versions
r3e-faas-cli versions --function hello-world

# Roll back to a specific version
r3e-faas-cli rollback --function hello-world --version 1.0.0
```

### Debugging Deployments

```bash
# Deploy with debug mode
r3e-faas-cli deploy --debug

# View debug logs
r3e-faas-cli logs --function hello-world --level debug
```

For more information, see the [API Reference](../api-reference.md) and [Architecture](../architecture.md) documents.
