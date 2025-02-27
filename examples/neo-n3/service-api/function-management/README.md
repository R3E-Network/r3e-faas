# Neo N3 Function Management Example

This example demonstrates how to manage JavaScript functions in the Neo N3 FaaS platform. Function management includes operations such as listing, retrieving, updating, versioning, and deleting functions.

## Overview

The Function Management example shows how to:

1. List all registered functions in the platform
2. Retrieve details of a specific function
3. Update an existing function with new code or configuration
4. Create and manage function versions
5. Delete functions from the platform
6. Monitor function execution and logs

## Files

- `config.yaml`: Configuration file for the function management example
- `manage.js`: Script to manage functions in the FaaS platform
- `function.js`: Sample function to be used in the management example
- `update.js`: Script to update the function with new code or configuration
- `version.js`: Script to create and manage function versions
- `monitor.js`: Script to monitor function execution and logs

## Prerequisites

- Neo N3 FaaS platform installed and running
- Node.js installed for running the management scripts
- API key for authenticating with the FaaS platform
- At least one function registered with the platform (you can use the Function Registration Example)

## Setup

1. Configure your API key in the environment:

```bash
export R3E_API_KEY=your_api_key
```

2. Run the management script:

```bash
node manage.js
```

## How It Works

The function management process involves several operations:

### 1. Listing Functions

You can list all functions registered in the platform:

```javascript
// List all functions
const functions = await listFunctions();

console.log('Total functions:', functions.length);
functions.forEach((func, index) => {
  console.log(`${index + 1}. ${func.name} (ID: ${func.id})`);
  console.log(`   Description: ${func.description}`);
  console.log(`   Version: ${func.metadata.version}`);
});
```

### 2. Retrieving Function Details

You can retrieve detailed information about a specific function:

```javascript
// Get function details
const functionDetails = await getFunctionDetails(functionId);

console.log('Function Name:', functionDetails.name);
console.log('Function Description:', functionDetails.description);
console.log('Function Version:', functionDetails.metadata.version);
console.log('Function Runtime:', functionDetails.metadata.runtime);
console.log('Function Handler:', functionDetails.metadata.handler);
console.log('Trigger Type:', functionDetails.metadata.trigger_type);
```

### 3. Updating Functions

You can update an existing function with new code or configuration:

```javascript
// Update function
const updatePayload = {
  name: functionDetails.name,
  description: "Updated function description",
  code: updatedCode,
  metadata: {
    runtime: functionDetails.metadata.runtime,
    handler: functionDetails.metadata.handler,
    trigger_type: functionDetails.metadata.trigger_type,
    trigger_config: functionDetails.metadata.trigger_config
  }
};

const updatedFunction = await updateFunction(functionId, updatePayload);
```

### 4. Function Versioning

You can create and manage multiple versions of a function:

```javascript
// Create a new version
const versionPayload = {
  name: functionDetails.name,
  description: functionDetails.description,
  code: functionDetails.code,
  version: "1.1.0",
  metadata: functionDetails.metadata
};

const newVersion = await createFunctionVersion(functionId, versionPayload);
```

### 5. Deleting Functions

You can delete functions from the platform:

```javascript
// Delete function
const deleted = await deleteFunction(functionId);

if (deleted) {
  console.log('Function deleted successfully');
} else {
  console.error('Failed to delete function');
}
```

### 6. Function Monitoring

You can monitor function execution and logs:

```javascript
// Monitor function logs
const logs = await getFunctionLogs(functionId);

console.log('Function Logs:');
logs.forEach(log => {
  console.log(`[${log.timestamp}] ${log.level}: ${log.message}`);
});

// Monitor function executions
const executions = await getFunctionExecutions(functionId);

console.log('Function Executions:');
executions.forEach(execution => {
  console.log(`ID: ${execution.id}`);
  console.log(`Status: ${execution.status}`);
  console.log(`Started: ${execution.start_time}`);
  console.log(`Duration: ${execution.duration}ms`);
});
```

## Function Management API

The Neo N3 FaaS platform provides a comprehensive API for function management:

### List Functions

```
GET /api/functions
```

### Get Function Details

```
GET /api/functions/{functionId}
```

### Update Function

```
PUT /api/functions/{functionId}
```

### Create Function Version

```
POST /api/functions/{functionId}/versions
```

### List Function Versions

```
GET /api/functions/{functionId}/versions
```

### Delete Function

```
DELETE /api/functions/{functionId}
```

### Get Function Logs

```
GET /api/functions/{functionId}/logs
```

### Get Function Executions

```
GET /api/functions/{functionId}/executions
```

## Function Lifecycle Management

The Neo N3 FaaS platform supports a complete function lifecycle:

1. **Development**: Create and test functions locally
2. **Registration**: Register functions with the platform
3. **Deployment**: Deploy functions to the platform
4. **Monitoring**: Monitor function execution and performance
5. **Updating**: Update functions with new code or configuration
6. **Versioning**: Create and manage function versions
7. **Deprecation**: Deprecate old function versions
8. **Deletion**: Delete functions from the platform

## Error Handling

The management scripts include error handling to manage various failure scenarios:

```javascript
try {
  // Management code
} catch (error) {
  console.error('Error managing function:');
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
