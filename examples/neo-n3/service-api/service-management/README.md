# Neo N3 Service Management Example

This example demonstrates how to manage services in the Neo N3 FaaS platform. Service management includes operations such as listing, retrieving, updating, versioning, and deleting services, as well as managing service functions and dependencies.

## Overview

The Service Management example shows how to:

1. List all registered services in the platform
2. Retrieve details of a specific service
3. Update an existing service with new functions or configuration
4. Create and manage service versions
5. Delete services from the platform
6. Monitor service execution and logs
7. Manage service dependencies and permissions

## Files

- `config.yaml`: Configuration file for the service management example
- `manage.js`: Script to manage services in the FaaS platform
- `functions/`: Directory containing the service's functions
  - `index.js`: Main function for the service
  - `auth.js`: Authentication function for the service
  - `data.js`: Data processing function for the service
- `update.js`: Script to update the service with new functions or configuration
- `version.js`: Script to create and manage service versions
- `monitor.js`: Script to monitor service execution and logs
- `client.js`: Client script to interact with the managed service

## Prerequisites

- Neo N3 FaaS platform installed and running
- Node.js installed for running the management scripts
- API key for authenticating with the FaaS platform
- At least one service registered with the platform (you can use the Service Registration Example)

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

### 1. Service Management

The service management process involves several operations:

#### Listing Services

You can list all services registered in the platform:

```javascript
// List all services
const services = await listServices();

console.log('Total services:', services.length);
services.forEach((service, index) => {
  console.log(`${index + 1}. ${service.name} (ID: ${service.id})`);
  console.log(`   Description: ${service.description}`);
  console.log(`   Version: ${service.version}`);
});
```

#### Retrieving Service Details

You can retrieve detailed information about a specific service:

```javascript
// Get service details
const serviceDetails = await getServiceDetails(serviceId);

console.log('Service Name:', serviceDetails.name);
console.log('Service Description:', serviceDetails.description);
console.log('Service Version:', serviceDetails.version);
console.log('Number of Functions:', serviceDetails.functions.length);
```

#### Updating Services

You can update an existing service with new functions or configuration:

```javascript
// Update service
const updatePayload = {
  name: serviceDetails.name,
  description: "Updated service description",
  version: serviceDetails.version,
  functions: updatedFunctions,
  dependencies: serviceDetails.dependencies,
  permissions: serviceDetails.permissions,
  resources: serviceDetails.resources,
  environment: serviceDetails.environment
};

const updatedService = await updateService(serviceId, updatePayload);
```

#### Service Versioning

You can create and manage multiple versions of a service:

```javascript
// Create a new version
const versionPayload = {
  name: serviceDetails.name,
  description: serviceDetails.description,
  version: "1.1.0",
  functions: serviceDetails.functions,
  dependencies: serviceDetails.dependencies,
  permissions: serviceDetails.permissions,
  resources: serviceDetails.resources,
  environment: serviceDetails.environment
};

const newVersion = await createServiceVersion(serviceId, versionPayload);
```

#### Deleting Services

You can delete services from the platform:

```javascript
// Delete service
const deleted = await deleteService(serviceId);

if (deleted) {
  console.log('Service deleted successfully');
} else {
  console.error('Failed to delete service');
}
```

#### Service Monitoring

You can monitor service execution and logs:

```javascript
// Monitor service logs
const logs = await getServiceLogs(serviceId);

console.log('Service Logs:');
logs.forEach(log => {
  console.log(`[${log.timestamp}] ${log.level}: ${log.message}`);
});

// Monitor service executions
const executions = await getServiceExecutions(serviceId);

console.log('Service Executions:');
executions.forEach(execution => {
  console.log(`ID: ${execution.id}`);
  console.log(`Status: ${execution.status}`);
  console.log(`Started: ${execution.start_time}`);
  console.log(`Duration: ${execution.duration}ms`);
});
```

### 2. Function Management within Services

You can manage functions within a service:

#### Adding Functions

```javascript
// Add a new function to the service
const newFunction = {
  name: "newFunction",
  description: "A new function for the service",
  handler: "functions/newFunction.js:handler",
  trigger: {
    type: "request",
    config: {
      http: {
        path: "/example-service/new-function",
        methods: ["GET", "POST"]
      }
    }
  }
};

const updatedService = await addFunctionToService(serviceId, newFunction);
```

#### Updating Functions

```javascript
// Update an existing function in the service
const updatedFunction = {
  name: "existingFunction",
  description: "Updated function description",
  handler: "functions/existingFunction.js:handler",
  trigger: {
    type: "request",
    config: {
      http: {
        path: "/example-service/existing-function",
        methods: ["GET", "POST", "PUT"]
      }
    }
  }
};

const updatedService = await updateFunctionInService(serviceId, "existingFunction", updatedFunction);
```

#### Removing Functions

```javascript
// Remove a function from the service
const updatedService = await removeFunctionFromService(serviceId, "functionToRemove");
```

### 3. Dependency Management

You can manage service dependencies:

```javascript
// Add a new dependency to the service
const newDependency = {
  name: "new-dependency",
  version: "^1.0.0"
};

const updatedService = await addDependencyToService(serviceId, newDependency);

// Remove a dependency from the service
const updatedService = await removeDependencyFromService(serviceId, "dependency-to-remove");
```

### 4. Permission Management

You can manage service permissions:

```javascript
// Update service permissions
const newPermissions = {
  invoke: [
    { type: "user", id: "*" },
    { type: "role", id: "admin" }
  ],
  manage: [
    { type: "user", id: "owner" },
    { type: "role", id: "admin" }
  ]
};

const updatedService = await updateServicePermissions(serviceId, newPermissions);
```

## Service Management API

The Neo N3 FaaS platform provides a comprehensive API for service management:

### List Services

```
GET /api/services
```

### Get Service Details

```
GET /api/services/{serviceId}
```

### Update Service

```
PUT /api/services/{serviceId}
```

### Create Service Version

```
POST /api/services/{serviceId}/versions
```

### List Service Versions

```
GET /api/services/{serviceId}/versions
```

### Delete Service

```
DELETE /api/services/{serviceId}
```

### Get Service Logs

```
GET /api/services/{serviceId}/logs
```

### Get Service Executions

```
GET /api/services/{serviceId}/executions
```

### Add Function to Service

```
POST /api/services/{serviceId}/functions
```

### Update Function in Service

```
PUT /api/services/{serviceId}/functions/{functionName}
```

### Remove Function from Service

```
DELETE /api/services/{serviceId}/functions/{functionName}
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

The management scripts include error handling to manage various failure scenarios:

```javascript
try {
  // Management code
} catch (error) {
  console.error('Error managing service:');
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
