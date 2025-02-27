# Neo N3 Function Registration Example

This example demonstrates how to register JavaScript functions with the Neo N3 FaaS platform. Function registration is the process of submitting a JavaScript function to the platform, configuring its triggers and permissions, and making it available for execution.

## Overview

The Function Registration example shows how to:

1. Create a JavaScript function that can be executed by the Neo N3 FaaS platform
2. Configure the function's metadata, including triggers and permissions
3. Register the function with the platform using the REST API
4. Verify the registration status and retrieve function details
5. Update and version functions

## Files

- `function.js`: The JavaScript function to be registered with the platform
- `config.yaml`: Configuration file for the function
- `register.js`: Script to register the function with the FaaS platform
- `client.js`: Client script to interact with the registered function

## Prerequisites

- Neo N3 FaaS platform installed and running
- Node.js installed for running the registration script
- API key for authenticating with the FaaS platform

## Setup

1. Configure your API key in the environment:

```bash
export R3E_API_KEY=your_api_key
```

2. Register the function using the registration script:

```bash
node register.js
```

## How It Works

The function registration process involves several steps:

### 1. Function Creation

First, you need to create a JavaScript function that follows the Neo N3 FaaS platform's function structure:

```javascript
/**
 * Example function for Neo N3 FaaS platform
 */
async function handler(request, user, context) {
  // Function implementation
  return {
    statusCode: 200,
    body: {
      message: "Hello from Neo N3 FaaS platform!"
    }
  };
}

// Export the handler function
module.exports = { handler };
```

### 2. Function Configuration

Next, you need to configure the function's metadata in a YAML file:

```yaml
# Function metadata
name: example-function
description: Example function for Neo N3 FaaS platform
runtime: javascript
handler: function.js:handler
version: 1.0.0

# Trigger configuration
trigger:
  type: request
  config:
    http:
      enabled: true
      path: "/example-function"
      methods: ["GET", "POST"]
```

### 3. Function Registration

Then, you register the function with the platform using the REST API:

```javascript
// Read the function code and configuration
const code = fs.readFileSync('function.js', 'utf8');
const config = yaml.load(fs.readFileSync('config.yaml', 'utf8'));

// Prepare the registration payload
const payload = {
  name: config.name,
  description: config.description,
  code: code,
  metadata: {
    runtime: config.runtime,
    trigger_type: config.trigger.type,
    trigger_config: config.trigger.config
  }
};

// Register the function
const response = await axios.post(`${API_URL}/functions`, payload, {
  headers: {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${API_KEY}`
  }
});
```

### 4. Function Verification

After registration, you can verify the function's status:

```javascript
// Get the function details
const functionId = response.data.id;
const functionDetails = await axios.get(`${API_URL}/functions/${functionId}`, {
  headers: {
    'Authorization': `Bearer ${API_KEY}`
  }
});

console.log('Function registered successfully!');
console.log('Function ID:', functionId);
console.log('Function URL:', `${API_URL}/functions/${functionId}`);
```

### 5. Function Invocation

Finally, you can invoke the registered function:

```javascript
// Invoke the function
const invocationResponse = await axios.post(`${API_URL}/functions/${functionId}/invoke`, {
  data: {
    message: "Hello, function!"
  }
}, {
  headers: {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${API_KEY}`
  }
});

console.log('Function invocation result:', invocationResponse.data);
```

## Function Versioning

The Neo N3 FaaS platform supports function versioning, allowing you to maintain multiple versions of a function:

```javascript
// Update the function with a new version
const updatePayload = {
  name: config.name,
  description: config.description,
  code: updatedCode,
  version: "1.1.0",
  metadata: {
    runtime: config.runtime,
    trigger_type: config.trigger.type,
    trigger_config: config.trigger.config
  }
};

// Update the function
const updateResponse = await axios.put(`${API_URL}/functions/${functionId}`, updatePayload, {
  headers: {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${API_KEY}`
  }
});
```

## Function Permissions

You can configure permissions for your function to control who can invoke it:

```yaml
# Permissions configuration
permissions:
  invoke:
    - type: "user"
      id: "*"  # Allow all authenticated users
    - type: "role"
      id: "admin"  # Allow users with admin role
  manage:
    - type: "user"
      id: "owner"  # Only the owner can manage the function
```

## Function Triggers

The Neo N3 FaaS platform supports various trigger types:

### HTTP Trigger

```yaml
trigger:
  type: request
  config:
    http:
      enabled: true
      path: "/example-function"
      methods: ["GET", "POST"]
```

### Blockchain Event Trigger

```yaml
trigger:
  type: blockchain
  config:
    neo:
      enabled: true
      events:
        - type: "block"
          enabled: true
        - type: "transaction"
          enabled: true
        - type: "notification"
          enabled: true
          contract_hash: "0x1234567890abcdef"
```

### Schedule Trigger

```yaml
trigger:
  type: schedule
  config:
    cron: "0 * * * *"  # Run every hour
```

## Error Handling

The registration process includes error handling to manage various failure scenarios:

```javascript
try {
  // Registration code
} catch (error) {
  console.error('Error registering function:');
  if (error.response) {
    console.error('Status:', error.response.status);
    console.error('Data:', error.response.data);
  } else {
    console.error(error.message);
  }
}
```

## Additional Resources

- [Neo N3 FaaS Platform Documentation](../../docs/neo-n3/README.md)
- [Function Development Guide](../../docs/neo-n3/guides/function-development.md)
- [API Reference](../../docs/neo-n3/api-reference.md)
- [JavaScript SDK Reference](../../docs/neo-n3/reference/javascript-sdk.md)
