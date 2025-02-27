# Basic Function Example for Neo N3 FaaS Platform

This example demonstrates how to create, deploy, and invoke a simple function on the Neo N3 FaaS platform.

## Prerequisites

Before you begin, ensure you have the following:

- Neo N3 FaaS CLI installed (`npm install -g r3e-faas-cli`)
- A Neo N3 FaaS account
- Basic knowledge of JavaScript

## Project Setup

1. Create a new project directory:

```bash
mkdir hello-neo-faas
cd hello-neo-faas
```

2. Initialize a new Neo N3 FaaS project:

```bash
r3e-faas-cli init
```

This command creates the following files:

```
hello-neo-faas/
├── functions/
│   └── hello.js
├── r3e.yaml
└── package.json
```

## Function Implementation

Open the `functions/hello.js` file and replace its contents with the following code:

```javascript
/**
 * A simple hello world function for Neo N3 FaaS
 * 
 * @param {Object} event - The event object
 * @param {Object} context - The context object
 * @returns {Object} - The response object
 */
export default async function(event, context) {
  // Log the event for debugging
  console.log('Event received:', event);
  
  // Get the name from the event parameters or use a default
  const name = event.params?.name || 'World';
  
  // Get the current block height from the Neo N3 blockchain
  let blockHeight;
  try {
    blockHeight = await context.neo.getCurrentBlockHeight();
  } catch (error) {
    console.error('Error getting block height:', error);
    blockHeight = 'Unknown';
  }
  
  // Return a response
  return {
    message: `Hello, ${name}!`,
    timestamp: new Date().toISOString(),
    blockHeight: blockHeight
  };
}
```

This function:
1. Receives an event object with optional parameters
2. Logs the event for debugging
3. Extracts the `name` parameter from the event or uses a default value
4. Gets the current block height from the Neo N3 blockchain
5. Returns a response with a greeting message, timestamp, and block height

## Configuration

Open the `r3e.yaml` file and update it with the following configuration:

```yaml
project:
  name: hello-neo-faas
  version: 0.1.0

functions:
  hello:
    handler: functions/hello.js
    runtime: javascript
    trigger:
      type: http
      path: /hello
      method: get
    environment:
      NODE_ENV: production
```

This configuration:
1. Sets the project name and version
2. Defines a function named `hello`
3. Specifies the handler file path
4. Sets the runtime to JavaScript
5. Configures an HTTP trigger with a GET method at the path `/hello`
6. Sets the `NODE_ENV` environment variable to `production`

## Local Testing

Before deploying, test the function locally:

```bash
r3e-faas-cli invoke-local --function hello
```

This should output something like:

```json
{
  "message": "Hello, World!",
  "timestamp": "2025-02-27T12:34:56.789Z",
  "blockHeight": "Unknown"
}
```

Test with parameters:

```bash
r3e-faas-cli invoke-local --function hello --params '{"name": "Neo"}'
```

This should output:

```json
{
  "message": "Hello, Neo!",
  "timestamp": "2025-02-27T12:34:56.789Z",
  "blockHeight": "Unknown"
}
```

## Deployment

Deploy the function to the Neo N3 FaaS platform:

```bash
r3e-faas-cli deploy
```

This command:
1. Packages the function code
2. Uploads it to the Neo N3 FaaS platform
3. Creates the necessary resources
4. Configures the HTTP endpoint

After successful deployment, you should see output similar to:

```
Deploying project: hello-neo-faas
Deploying function: hello... Done!
Function URL: https://faas.example.com/functions/hello
```

## Invoking the Function

### Using curl

```bash
curl https://faas.example.com/functions/hello
```

With parameters:

```bash
curl https://faas.example.com/functions/hello?name=Neo
```

### Using the CLI

```bash
r3e-faas-cli invoke --function hello
```

With parameters:

```bash
r3e-faas-cli invoke --function hello --params '{"name": "Neo"}'
```

## Monitoring

View the function logs:

```bash
r3e-faas-cli logs --function hello
```

To follow the logs in real-time:

```bash
r3e-faas-cli logs --function hello --follow
```

## Updating the Function

If you need to update the function, simply modify the code in `functions/hello.js` and redeploy:

```bash
r3e-faas-cli deploy
```

## Cleaning Up

To remove the function:

```bash
r3e-faas-cli remove --function hello
```

To remove the entire project:

```bash
r3e-faas-cli remove
```

## Next Steps

Now that you've created, deployed, and invoked a basic function, you can:

1. Explore more complex triggers like Neo N3 blockchain events
2. Use Oracle services for external data
3. Implement secure computing with TEE services
4. Create custom services

For more examples, see:
- [Neo N3 Event Trigger Example](./neo-event-trigger.md)
- [Oracle Service Example](./oracle-service.md)
- [TEE Service Example](./tee-service.md)
- [Service API Example](./service-api.md)

For more information, see the [Documentation](../README.md).
