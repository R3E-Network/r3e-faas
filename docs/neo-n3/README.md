# Neo N3 FaaS Platform Documentation

This documentation provides comprehensive information about the Neo N3 Function-as-a-Service (FaaS) platform, including its architecture, components, and usage instructions.

## Table of Contents

1. [Introduction](#introduction)
2. [Architecture Overview](#architecture-overview)
3. [Components](#components)
   - [Neo N3 Blockchain Integration](#neo-n3-blockchain-integration)
   - [JavaScript Runtime](#javascript-runtime)
   - [Oracle Services](#oracle-services)
   - [TEE Computing Services](#tee-computing-services)
   - [Service API](#service-api)
4. [Getting Started](#getting-started)
5. [API Reference](#api-reference)
6. [Examples](#examples)
7. [Troubleshooting](#troubleshooting)

## Introduction

The Neo N3 FaaS platform is a serverless computing platform designed specifically for the Neo N3 blockchain ecosystem. It allows developers to write and deploy JavaScript functions that can be triggered by blockchain events, API calls, or scheduled tasks. The platform provides built-in services such as oracle data feeds, trusted execution environments, and blockchain event monitoring.

## Architecture Overview

The Neo N3 FaaS platform follows a modular architecture with several key components:

![Architecture Diagram](./architecture.png)

The platform consists of the following main components:

- **Event System**: Monitors blockchain events and triggers functions
- **Scheduler**: Distributes function execution tasks to worker nodes
- **Worker Nodes**: Execute JavaScript functions in a secure environment
- **Oracle Services**: Provide external data to functions
- **TEE Services**: Enable secure execution of sensitive code
- **API Service**: Provides REST and GraphQL APIs for platform management

## Components

### Neo N3 Blockchain Integration

The Neo N3 blockchain integration component provides connectivity to the Neo N3 blockchain network. It monitors blockchain events such as new blocks, transactions, and smart contract notifications, and triggers functions based on these events.

Key features:
- Neo N3 RPC client integration
- Block and transaction event monitoring
- Smart contract notification handling
- NEP-17 token support
- Neo Name Service integration

### JavaScript Runtime

The JavaScript runtime component provides a secure execution environment for user-defined JavaScript functions. It is based on the Deno Core (V8) engine and provides a sandboxed environment with controlled access to resources.

Key features:
- Deno Core (V8) JavaScript engine
- Secure sandboxing
- Resource limiting and monitoring
- Neo-specific JavaScript APIs
- Oracle and TEE service access

### Oracle Services

The Oracle services component provides access to external data sources such as price feeds, random number generation, and other real-world data. It ensures that the data is reliable and tamper-proof.

Key features:
- Price feed oracles
- Random number generation
- Weather data
- Sports results
- Financial data

### TEE Computing Services

The Trusted Execution Environment (TEE) services component provides a secure execution environment for sensitive code and data. It ensures that the code and data are protected from unauthorized access and tampering.

Key features:
- Intel SGX, AMD SEV, and ARM TrustZone support
- Secure key management
- Attestation verification
- Secure data transfer
- Confidential computing

### Service API

The Service API component provides REST and GraphQL APIs for platform management. It allows developers to register, deploy, and manage their functions and services.

Key features:
- REST API
- GraphQL API
- Authentication and authorization
- Service registration and discovery
- Function deployment and management

## Getting Started

To get started with the Neo N3 FaaS platform, follow these steps:

1. **Install the CLI tool**

   ```bash
   npm install -g r3e-faas-cli
   ```

2. **Initialize a new project**

   ```bash
   r3e-faas-cli init my-neo-faas-project
   cd my-neo-faas-project
   ```

3. **Create a function**

   Create a new file `functions/hello-neo.js`:

   ```javascript
   export default async function(event, context) {
     console.log("Hello, Neo N3!");
     return {
       message: "Hello, Neo N3!",
       event: event,
       blockHeight: context.neo.getCurrentBlockHeight()
     };
   }
   ```

4. **Configure the function**

   Edit the `r3e.yaml` file:

   ```yaml
   functions:
     hello-neo:
       handler: functions/hello-neo.js
       trigger:
         type: http
         path: /hello-neo
       runtime: javascript
   ```

5. **Deploy the function**

   ```bash
   r3e-faas-cli deploy
   ```

6. **Test the function**

   ```bash
   curl https://your-faas-endpoint.example.com/hello-neo
   ```

## API Reference

For detailed API reference, see the [API Reference](./api-reference.md) document.

## Examples

For examples of how to use the Neo N3 FaaS platform, see the [Examples](../examples/neo-n3) directory.

## Troubleshooting

For troubleshooting information, see the [Troubleshooting](./troubleshooting.md) document.
