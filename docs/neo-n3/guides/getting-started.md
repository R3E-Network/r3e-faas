# Getting Started with Neo N3 FaaS Platform

This guide provides step-by-step instructions for getting started with the Neo N3 FaaS platform.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Installation](#installation)
3. [Configuration](#configuration)
4. [Creating Your First Function](#creating-your-first-function)
5. [Deploying Your Function](#deploying-your-function)
6. [Testing Your Function](#testing-your-function)
7. [Monitoring Your Function](#monitoring-your-function)
8. [Next Steps](#next-steps)

## Prerequisites

Before you begin, ensure you have the following:

- **Node.js**: Version 16 or later
- **npm**: Version 7 or later
- **Git**: Latest version
- **Neo N3 Wallet**: A Neo N3 wallet with some GAS for transaction fees

You can check your Node.js and npm versions with the following commands:

```bash
node --version
npm --version
```

If you need to install or update Node.js and npm, visit the [official Node.js website](https://nodejs.org/).

## Installation

### Install the CLI

The Neo N3 FaaS platform provides a command-line interface (CLI) tool for managing your functions and services. Install it globally using npm:

```bash
npm install -g r3e-faas-cli
```

Verify the installation:

```bash
r3e-faas-cli --version
```

### Create a New Project

Create a new project using the CLI:

```bash
r3e-faas-cli init my-neo-faas-project
cd my-neo-faas-project
```

This command creates a new directory with the following structure:

```
my-neo-faas-project/
├── functions/
│   └── hello-world.js
├── services/
├── r3e.yaml
├── package.json
└── README.md
```

## Configuration

### Configure Your Project

The `r3e.yaml` file is the main configuration file for your project. It defines your functions, services, and their configurations.

Here's an example configuration:

```yaml
# r3e.yaml
project:
  name: my-neo-faas-project
  version: 0.1.0

functions:
  hello-world:
    handler: functions/hello-world.js
    runtime: javascript
    trigger:
      type: http
      path: /hello-world
    environment:
      NODE_ENV: production

services:
  neo-oracle:
    type: oracle
    config:
      assets: ["NEO", "GAS"]
      update_interval: 60
```

### Configure Your Neo N3 Connection

Create a `.env` file in your project root to store your Neo N3 connection details:

```
# .env
NEO_RPC_URL=https://n3seed1.ngd.network:10332
NEO_NETWORK=MainNet
NEO_WALLET_PATH=./wallet.json
NEO_WALLET_PASSWORD=your-wallet-password
```

**Note**: Never commit your `.env` file to version control. Add it to your `.gitignore` file.

## Creating Your First Function

### Hello World Function

The project template includes a simple "Hello World" function. Open the `functions/hello-world.js` file:

```javascript
// functions/hello-world.js
export default async function(event, context) {
  return {
    message: "Hello, Neo N3 FaaS!",
    event: event,
    timestamp: new Date().toISOString()
  };
}
```

### Neo N3 Blockchain Function

Let's create a function that interacts with the Neo N3 blockchain. Create a new file `functions/neo-info.js`:

```javascript
// functions/neo-info.js
export default async function(event, context) {
  // Get current block height
  const blockHeight = await context.neo.getCurrentBlockHeight();
  
  // Get NEO price in USD
  const neoPrice = await context.oracle.getPrice("NEO", "USD");
  
  // Get GAS price in USD
  const gasPrice = await context.oracle.getPrice("GAS", "USD");
  
  return {
    blockHeight,
    neoPrice,
    gasPrice,
    timestamp: new Date().toISOString()
  };
}
```

Update your `r3e.yaml` file to include the new function:

```yaml
# r3e.yaml
# ... existing configuration ...

functions:
  hello-world:
    # ... existing configuration ...
  
  neo-info:
    handler: functions/neo-info.js
    runtime: javascript
    trigger:
      type: http
      path: /neo-info
    environment:
      NODE_ENV: production
```

## Deploying Your Function

### Build Your Project

Before deploying, build your project:

```bash
r3e-faas-cli build
```

This command compiles your functions and services, and prepares them for deployment.

### Deploy Your Project

Deploy your project to the Neo N3 FaaS platform:

```bash
r3e-faas-cli deploy
```

This command deploys your functions and services to the platform. You'll see output similar to this:

```
Deploying project: my-neo-faas-project
Deploying function: hello-world... Done!
Deploying function: neo-info... Done!
Deploying service: neo-oracle... Done!
Deployment complete!
Functions:
  - hello-world: https://faas.example.com/functions/hello-world
  - neo-info: https://faas.example.com/functions/neo-info
Services:
  - neo-oracle: https://faas.example.com/services/neo-oracle
```

## Testing Your Function

### Test Locally

Before deploying, you can test your functions locally:

```bash
r3e-faas-cli invoke-local --function hello-world
```

For functions with parameters:

```bash
r3e-faas-cli invoke-local --function hello-world --params '{"name": "Neo"}'
```

### Test Deployed Function

Test your deployed function using curl:

```bash
curl https://faas.example.com/functions/hello-world
```

Or with parameters:

```bash
curl -X POST -H "Content-Type: application/json" -d '{"name": "Neo"}' https://faas.example.com/functions/hello-world
```

## Monitoring Your Function

### View Logs

View the logs for your function:

```bash
r3e-faas-cli logs --function hello-world
```

To follow the logs in real-time:

```bash
r3e-faas-cli logs --function hello-world --follow
```

### View Metrics

View metrics for your function:

```bash
r3e-faas-cli metrics --function hello-world
```

This command shows metrics such as invocation count, error count, and average duration.

### View Function Status

Check the status of your functions:

```bash
r3e-faas-cli list
```

This command shows the status of all your functions and services.

## Next Steps

Now that you've created, deployed, and tested your first function, you can:

- **Create more complex functions**: Explore the [Function Development Guide](./function-development.md) for more advanced function development.
- **Create services**: Learn how to create and deploy services in the [Service Development Guide](./service-development.md).
- **Use Oracle services**: Learn how to use Oracle services in your functions in the [Oracle Service Example](../examples/oracle-service.md).
- **Use TEE services**: Learn how to use TEE services for secure computing in the [TEE Service Example](../examples/tee-service.md).
- **Explore the API**: Learn about the platform's API in the [API Reference](../api-reference.md).

For more information, see the [Documentation](../README.md).
