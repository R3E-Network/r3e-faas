# Neo N3 Service Registration Example

This example demonstrates how to register a service with the Neo N3 FaaS platform. A service in the Neo N3 FaaS platform is a collection of related functions that work together to provide a specific capability or feature.

## Overview

The Service Registration example shows how to:

1. Define a service with multiple functions
2. Configure service metadata and dependencies
3. Register the service with the platform
4. Manage service permissions and access control
5. Deploy and test the service

## Files

- `config.yaml`: Configuration file for the service
- `register.js`: Script to register the service with the FaaS platform
- `functions/`: Directory containing the service's functions
  - `index.js`: Main function for the service
  - `auth.js`: Authentication function for the service
  - `data.js`: Data processing function for the service
- `client.js`: Client script to interact with the registered service

## Prerequisites

- Neo N3 FaaS platform installed and running
- Node.js installed for running the registration scripts
- API key for authenticating with the FaaS platform

## Setup

1. Configure your API key in the environment:

```bash
export R3E_API_KEY=your_api_key
```

2. Run the registration script:

```bash
node register.js
```

## How It Works

### 1. Service Definition

A service in the Neo N3 FaaS platform is defined by a configuration file (`config.yaml`) that specifies the service metadata, functions, dependencies, and other settings:

```yaml
# Service metadata
name: example-service
description: Example service for Neo N3 FaaS platform
version: 1.0.0

# Functions
functions:
  - name: index
    description: Main function for the service
    handler: functions/index.js:handler
    trigger:
      type: request
      config:
        http:
          path: "/example-service"
          methods: ["GET", "POST"]
  
  - name: auth
    description: Authentication function for the service
    handler: functions/auth.js:handler
    trigger:
      type: request
      config:
        http:
          path: "/example-service/auth"
          methods: ["POST"]
  
  - name: data
    description: Data processing function for the service
    handler: functions/data.js:handler
    trigger:
      type: request
      config:
        http:
          path: "/example-service/data"
          methods: ["GET", "POST"]

# Dependencies
dependencies:
  - name: neo-sdk
    version: "^1.0.0"
  - name: axios
    version: "^0.24.0"

# Permissions
permissions:
  invoke:
    - type: "user"
      id: "*"  # Allow all authenticated users
    - type: "role"
      id: "admin"  # Allow users with admin role
  manage:
    - type: "user"
      id: "owner"  # Only the owner can manage the service

# Resource limits
resources:
  memory: 256MB
  timeout: 60s
  
# Environment variables
environment:
  LOG_LEVEL: info
  NEO_NETWORK: testnet
```

### 2. Service Registration

The service is registered with the platform using the `register.js` script:

```javascript
// Register the service
const response = await axios.post(`${API_URL}/services`, {
  name: config.name,
  description: config.description,
  version: config.version,
  functions: config.functions,
  dependencies: config.dependencies,
  permissions: config.permissions,
  resources: config.resources,
  environment: config.environment
}, {
  headers: {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${API_KEY}`
  }
});
```

### 3. Service Functions

Each function in the service is defined in a separate file in the `functions/` directory. For example, the main function (`index.js`):

```javascript
/**
 * Main handler function for the service
 */
async function handler(request, user, context) {
  // Function implementation
}

module.exports = { handler };
```

### 4. Service Client

The client script (`client.js`) demonstrates how to interact with the registered service:

```javascript
// Invoke the service
const response = await axios.post(`${API_URL}/services/${serviceId}/invoke`, {
  function: 'index',
  data: {
    // Request data
  }
}, {
  headers: {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${API_KEY}`
  }
});
```

## Service Registration API

The Neo N3 FaaS platform provides a comprehensive API for service registration and management:

### Register Service

```
POST /api/services
```

### Get Service Details

```
GET /api/services/{serviceId}
```

### Update Service

```
PUT /api/services/{serviceId}
```

### Delete Service

```
DELETE /api/services/{serviceId}
```

### Invoke Service Function

```
POST /api/services/{serviceId}/invoke
```

## Service Lifecycle Management

The Neo N3 FaaS platform supports a complete service lifecycle:

1. **Development**: Create and test service functions locally
2. **Registration**: Register the service with the platform
3. **Deployment**: Deploy the service to the platform
4. **Monitoring**: Monitor service execution and performance
5. **Updating**: Update the service with new functions or configuration
6. **Versioning**: Create and manage service versions
7. **Deprecation**: Deprecate old service versions
8. **Deletion**: Delete services from the platform

## Error Handling

The registration script includes error handling to manage various failure scenarios:

```javascript
try {
  // Registration code
} catch (error) {
  console.error('Error registering service:');
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
- [Service Development Guide](../../docs/neo-n3/guides/service-development.md)
- [API Reference](../../docs/neo-n3/api-reference.md)
- [JavaScript SDK Reference](../../docs/neo-n3/reference/javascript-sdk.md)
