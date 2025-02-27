# Neo N3 GraphQL API Example

This example demonstrates how to use the GraphQL API in the Neo N3 FaaS platform. GraphQL provides a flexible and efficient way to query and manipulate data in the platform, allowing clients to request exactly what they need and nothing more.

## Overview

The GraphQL API example shows how to:

1. Query services, functions, and executions using GraphQL
2. Perform mutations to create, update, and delete resources
3. Subscribe to real-time events
4. Use GraphQL variables, fragments, and directives
5. Implement custom GraphQL resolvers

## Files

- `config.yaml`: Configuration file for the GraphQL API example
- `schema.graphql`: GraphQL schema definition
- `client.js`: Client script to interact with the GraphQL API
- `server.js`: Server script to run a local GraphQL server
- `resolvers/`: Directory containing GraphQL resolvers
  - `query.js`: Query resolvers
  - `mutation.js`: Mutation resolvers
  - `subscription.js`: Subscription resolvers
  - `types.js`: Type resolvers

## Prerequisites

- Neo N3 FaaS platform installed and running
- Node.js installed for running the client and server scripts
- API key for authenticating with the FaaS platform

## Setup

1. Configure your API key in the environment:

```bash
export R3E_API_KEY=your_api_key
```

2. Run the GraphQL server:

```bash
node server.js
```

3. In a separate terminal, run the client script:

```bash
node client.js
```

## How It Works

### 1. GraphQL Schema

The GraphQL schema defines the types, queries, mutations, and subscriptions available in the API:

```graphql
type Query {
  # Services
  services: [Service!]!
  service(id: ID!): Service

  # Functions
  functions: [Function!]!
  function(id: ID!): Function

  # Executions
  executions(limit: Int): [Execution!]!
  execution(id: ID!): Execution
}

type Mutation {
  # Services
  createService(input: CreateServiceInput!): Service!
  updateService(id: ID!, input: UpdateServiceInput!): Service!
  deleteService(id: ID!): Boolean!

  # Functions
  createFunction(input: CreateFunctionInput!): Function!
  updateFunction(id: ID!, input: UpdateFunctionInput!): Function!
  deleteFunction(id: ID!): Boolean!

  # Invocations
  invokeFunction(id: ID!, input: InvokeFunctionInput!): InvocationResult!
}

type Subscription {
  # Real-time events
  serviceCreated: Service!
  functionInvoked: Execution!
  logEntryAdded: LogEntry!
}
```

### 2. GraphQL Queries

You can query data from the platform using GraphQL queries:

```graphql
# Query all services
query GetServices {
  services {
    id
    name
    description
    version
    functions {
      id
      name
    }
  }
}

# Query a specific service with its functions
query GetService($id: ID!) {
  service(id: $id) {
    id
    name
    description
    version
    functions {
      id
      name
      description
      handler
      trigger {
        type
        ... on HttpTrigger {
          path
          methods
        }
        ... on EventTrigger {
          source
          event
        }
      }
    }
  }
}
```

### 3. GraphQL Mutations

You can modify data in the platform using GraphQL mutations:

```graphql
# Create a new service
mutation CreateService($input: CreateServiceInput!) {
  createService(input: $input) {
    id
    name
    description
    version
  }
}

# Update an existing service
mutation UpdateService($id: ID!, $input: UpdateServiceInput!) {
  updateService(id: $id, input: $input) {
    id
    name
    description
    version
  }
}

# Delete a service
mutation DeleteService($id: ID!) {
  deleteService(id: $id)
}
```

### 4. GraphQL Subscriptions

You can subscribe to real-time events using GraphQL subscriptions:

```graphql
# Subscribe to service creation events
subscription OnServiceCreated {
  serviceCreated {
    id
    name
    description
    version
  }
}

# Subscribe to function invocation events
subscription OnFunctionInvoked {
  functionInvoked {
    id
    function {
      id
      name
    }
    status
    startTime
    endTime
    duration
  }
}
```

### 5. GraphQL Variables

You can use variables to parameterize your queries:

```javascript
// Query with variables
const query = `
  query GetService($id: ID!) {
    service(id: $id) {
      id
      name
      description
      version
    }
  }
`;

const variables = {
  id: 'service-123'
};

const result = await graphql(query, variables);
```

### 6. GraphQL Fragments

You can use fragments to reuse parts of your queries:

```graphql
# Define a fragment
fragment ServiceFields on Service {
  id
  name
  description
  version
}

# Use the fragment in a query
query GetServices {
  services {
    ...ServiceFields
    functions {
      id
      name
    }
  }
}
```

### 7. GraphQL Directives

You can use directives to conditionally include fields:

```graphql
# Query with directives
query GetService($id: ID!, $includeFunctions: Boolean!) {
  service(id: $id) {
    id
    name
    description
    version
    functions @include(if: $includeFunctions) {
      id
      name
      description
    }
  }
}
```

## GraphQL API Endpoints

The Neo N3 FaaS platform provides the following GraphQL API endpoints:

### Main GraphQL Endpoint

```
POST /api/graphql
```

This endpoint accepts GraphQL queries, mutations, and variables.

### GraphQL Playground

```
GET /api/graphql
```

This endpoint provides a GraphQL Playground interface for exploring the API.

### GraphQL Subscriptions

```
WS /api/graphql/subscriptions
```

This endpoint provides WebSocket-based GraphQL subscriptions.

## Error Handling

GraphQL errors are returned in the `errors` field of the response:

```json
{
  "data": null,
  "errors": [
    {
      "message": "Service not found",
      "locations": [
        {
          "line": 2,
          "column": 3
        }
      ],
      "path": [
        "service"
      ],
      "extensions": {
        "code": "NOT_FOUND"
      }
    }
  ]
}
```

## Authentication

The GraphQL API requires authentication using an API key or JWT token:

```javascript
// Using API key
const headers = {
  'Authorization': `Bearer ${apiKey}`
};

// Using JWT token
const headers = {
  'Authorization': `Bearer ${jwtToken}`
};
```

## Additional Resources

- [Neo N3 FaaS Platform Documentation](../../docs/neo-n3/README.md)
- [GraphQL Schema Reference](../../docs/neo-n3/reference/graphql-schema.md)
- [API Reference](../../docs/neo-n3/api-reference.md)
- [JavaScript SDK Reference](../../docs/neo-n3/reference/javascript-sdk.md)
