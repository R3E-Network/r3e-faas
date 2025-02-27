# GraphQL Schema Reference for Neo N3 FaaS Platform

This reference provides detailed information about the GraphQL schema for the Neo N3 FaaS platform.

## Table of Contents

1. [Introduction](#introduction)
2. [Schema Overview](#schema-overview)
3. [Types](#types)
4. [Queries](#queries)
5. [Mutations](#mutations)
6. [Subscriptions](#subscriptions)
7. [Directives](#directives)
8. [Examples](#examples)

## Introduction

The Neo N3 FaaS platform provides a GraphQL API for managing functions, services, and other platform resources. GraphQL is a query language for APIs that allows clients to request exactly the data they need, making it more efficient than traditional REST APIs.

## Schema Overview

The GraphQL schema for the Neo N3 FaaS platform is organized around the following main types:

- **User**: Represents a user of the platform
- **Function**: Represents a function deployed on the platform
- **Service**: Represents a service deployed on the platform
- **Trigger**: Represents a trigger for a function
- **Environment**: Represents an environment for functions and services
- **Deployment**: Represents a deployment of a function or service
- **Log**: Represents a log entry for a function or service
- **Metric**: Represents a metric for a function or service

## Types

### User

```graphql
type User {
  id: ID!
  username: String!
  email: String!
  firstName: String
  lastName: String
  roles: [Role!]!
  createdAt: DateTime!
  updatedAt: DateTime!
  functions: [Function!]!
  services: [Service!]!
}

enum Role {
  ADMIN
  DEVELOPER
  USER
}
```

### Function

```graphql
type Function {
  id: ID!
  name: String!
  description: String
  runtime: Runtime!
  handler: String!
  trigger: Trigger!
  resources: Resources
  environment: Environment
  owner: User!
  createdAt: DateTime!
  updatedAt: DateTime!
  deployments: [Deployment!]!
  logs: [Log!]!
  metrics: [Metric!]!
}

enum Runtime {
  JAVASCRIPT
  TYPESCRIPT
  PYTHON
  RUST
}

type Resources {
  memory: Int!
  cpu: Float!
  timeout: Int!
}
```

### Service

```graphql
type Service {
  id: ID!
  name: String!
  description: String
  type: ServiceType!
  config: JSONObject!
  resources: Resources
  environment: Environment
  owner: User!
  createdAt: DateTime!
  updatedAt: DateTime!
  deployments: [Deployment!]!
  logs: [Log!]!
  metrics: [Metric!]!
}

enum ServiceType {
  ORACLE
  TEE
  BLOCKCHAIN
  CUSTOM
}
```

### Trigger

```graphql
type Trigger {
  id: ID!
  type: TriggerType!
  config: JSONObject!
}

enum TriggerType {
  HTTP
  NEO
  SCHEDULE
  EVENT
}

type HttpTrigger {
  path: String!
  method: HttpMethod!
  cors: Boolean!
}

enum HttpMethod {
  GET
  POST
  PUT
  DELETE
  PATCH
  OPTIONS
  HEAD
}

type NeoTrigger {
  event: NeoEvent!
  contract: String
  network: String!
}

enum NeoEvent {
  NEW_BLOCK
  NEW_TRANSACTION
  CONTRACT_NOTIFICATION
  CUSTOM
}

type ScheduleTrigger {
  schedule: String!
  timezone: String!
}

type EventTrigger {
  eventType: String!
  filter: JSONObject
}
```

### Environment

```graphql
type Environment {
  id: ID!
  name: String!
  network: String!
  rpcUrl: String!
  region: String!
  logLevel: LogLevel!
  debug: Boolean!
  createdAt: DateTime!
  updatedAt: DateTime!
}

enum LogLevel {
  DEBUG
  INFO
  WARN
  ERROR
}
```

### Deployment

```graphql
type Deployment {
  id: ID!
  version: String!
  status: DeploymentStatus!
  createdAt: DateTime!
  updatedAt: DateTime!
  logs: [Log!]!
}

enum DeploymentStatus {
  PENDING
  DEPLOYING
  DEPLOYED
  FAILED
  ROLLING_BACK
  ROLLED_BACK
}
```

### Log

```graphql
type Log {
  id: ID!
  level: LogLevel!
  message: String!
  timestamp: DateTime!
  metadata: JSONObject
}
```

### Metric

```graphql
type Metric {
  id: ID!
  name: String!
  value: Float!
  unit: String!
  timestamp: DateTime!
  dimensions: JSONObject
}
```

### Scalar Types

```graphql
scalar DateTime
scalar JSONObject
```

## Queries

### User Queries

```graphql
type Query {
  # Get the current user
  me: User!
  
  # Get a user by ID
  user(id: ID!): User
  
  # List users
  users(
    # Pagination
    first: Int
    after: String
    last: Int
    before: String
    
    # Filtering
    filter: UserFilter
    
    # Sorting
    orderBy: UserOrderBy
  ): UserConnection!
}

input UserFilter {
  username: String
  email: String
  role: Role
}

input UserOrderBy {
  field: UserOrderField!
  direction: OrderDirection!
}

enum UserOrderField {
  USERNAME
  EMAIL
  CREATED_AT
  UPDATED_AT
}

enum OrderDirection {
  ASC
  DESC
}

type UserConnection {
  edges: [UserEdge!]!
  pageInfo: PageInfo!
  totalCount: Int!
}

type UserEdge {
  node: User!
  cursor: String!
}

type PageInfo {
  hasNextPage: Boolean!
  hasPreviousPage: Boolean!
  startCursor: String
  endCursor: String
}
```

### Function Queries

```graphql
type Query {
  # Get a function by ID
  function(id: ID!): Function
  
  # Get a function by name
  functionByName(name: String!): Function
  
  # List functions
  functions(
    # Pagination
    first: Int
    after: String
    last: Int
    before: String
    
    # Filtering
    filter: FunctionFilter
    
    # Sorting
    orderBy: FunctionOrderBy
  ): FunctionConnection!
}

input FunctionFilter {
  name: String
  runtime: Runtime
  triggerType: TriggerType
  owner: ID
}

input FunctionOrderBy {
  field: FunctionOrderField!
  direction: OrderDirection!
}

enum FunctionOrderField {
  NAME
  RUNTIME
  CREATED_AT
  UPDATED_AT
}

type FunctionConnection {
  edges: [FunctionEdge!]!
  pageInfo: PageInfo!
  totalCount: Int!
}

type FunctionEdge {
  node: Function!
  cursor: String!
}
```

### Service Queries

```graphql
type Query {
  # Get a service by ID
  service(id: ID!): Service
  
  # Get a service by name
  serviceByName(name: String!): Service
  
  # List services
  services(
    # Pagination
    first: Int
    after: String
    last: Int
    before: String
    
    # Filtering
    filter: ServiceFilter
    
    # Sorting
    orderBy: ServiceOrderBy
  ): ServiceConnection!
}

input ServiceFilter {
  name: String
  type: ServiceType
  owner: ID
}

input ServiceOrderBy {
  field: ServiceOrderField!
  direction: OrderDirection!
}

enum ServiceOrderField {
  NAME
  TYPE
  CREATED_AT
  UPDATED_AT
}

type ServiceConnection {
  edges: [ServiceEdge!]!
  pageInfo: PageInfo!
  totalCount: Int!
}

type ServiceEdge {
  node: Service!
  cursor: String!
}
```

### Environment Queries

```graphql
type Query {
  # Get an environment by ID
  environment(id: ID!): Environment
  
  # Get an environment by name
  environmentByName(name: String!): Environment
  
  # List environments
  environments(
    # Pagination
    first: Int
    after: String
    last: Int
    before: String
    
    # Filtering
    filter: EnvironmentFilter
    
    # Sorting
    orderBy: EnvironmentOrderBy
  ): EnvironmentConnection!
}

input EnvironmentFilter {
  name: String
  network: String
  region: String
}

input EnvironmentOrderBy {
  field: EnvironmentOrderField!
  direction: OrderDirection!
}

enum EnvironmentOrderField {
  NAME
  NETWORK
  REGION
  CREATED_AT
  UPDATED_AT
}

type EnvironmentConnection {
  edges: [EnvironmentEdge!]!
  pageInfo: PageInfo!
  totalCount: Int!
}

type EnvironmentEdge {
  node: Environment!
  cursor: String!
}
```

### Deployment Queries

```graphql
type Query {
  # Get a deployment by ID
  deployment(id: ID!): Deployment
  
  # List deployments
  deployments(
    # Pagination
    first: Int
    after: String
    last: Int
    before: String
    
    # Filtering
    filter: DeploymentFilter
    
    # Sorting
    orderBy: DeploymentOrderBy
  ): DeploymentConnection!
}

input DeploymentFilter {
  functionId: ID
  serviceId: ID
  status: DeploymentStatus
}

input DeploymentOrderBy {
  field: DeploymentOrderField!
  direction: OrderDirection!
}

enum DeploymentOrderField {
  VERSION
  STATUS
  CREATED_AT
  UPDATED_AT
}

type DeploymentConnection {
  edges: [DeploymentEdge!]!
  pageInfo: PageInfo!
  totalCount: Int!
}

type DeploymentEdge {
  node: Deployment!
  cursor: String!
}
```

### Log Queries

```graphql
type Query {
  # Get logs
  logs(
    # Pagination
    first: Int
    after: String
    last: Int
    before: String
    
    # Filtering
    filter: LogFilter
    
    # Sorting
    orderBy: LogOrderBy
  ): LogConnection!
}

input LogFilter {
  functionId: ID
  serviceId: ID
  level: LogLevel
  startTime: DateTime
  endTime: DateTime
}

input LogOrderBy {
  field: LogOrderField!
  direction: OrderDirection!
}

enum LogOrderField {
  LEVEL
  TIMESTAMP
}

type LogConnection {
  edges: [LogEdge!]!
  pageInfo: PageInfo!
  totalCount: Int!
}

type LogEdge {
  node: Log!
  cursor: String!
}
```

### Metric Queries

```graphql
type Query {
  # Get metrics
  metrics(
    # Pagination
    first: Int
    after: String
    last: Int
    before: String
    
    # Filtering
    filter: MetricFilter
    
    # Sorting
    orderBy: MetricOrderBy
  ): MetricConnection!
}

input MetricFilter {
  functionId: ID
  serviceId: ID
  name: String
  startTime: DateTime
  endTime: DateTime
}

input MetricOrderBy {
  field: MetricOrderField!
  direction: OrderDirection!
}

enum MetricOrderField {
  NAME
  VALUE
  TIMESTAMP
}

type MetricConnection {
  edges: [MetricEdge!]!
  pageInfo: PageInfo!
  totalCount: Int!
}

type MetricEdge {
  node: Metric!
  cursor: String!
}
```

## Mutations

### User Mutations

```graphql
type Mutation {
  # Create a user
  createUser(input: CreateUserInput!): CreateUserPayload!
  
  # Update a user
  updateUser(input: UpdateUserInput!): UpdateUserPayload!
  
  # Delete a user
  deleteUser(input: DeleteUserInput!): DeleteUserPayload!
}

input CreateUserInput {
  username: String!
  email: String!
  password: String!
  firstName: String
  lastName: String
  roles: [Role!]!
}

type CreateUserPayload {
  user: User!
}

input UpdateUserInput {
  id: ID!
  username: String
  email: String
  firstName: String
  lastName: String
  roles: [Role!]
}

type UpdateUserPayload {
  user: User!
}

input DeleteUserInput {
  id: ID!
}

type DeleteUserPayload {
  id: ID!
}
```

### Function Mutations

```graphql
type Mutation {
  # Create a function
  createFunction(input: CreateFunctionInput!): CreateFunctionPayload!
  
  # Update a function
  updateFunction(input: UpdateFunctionInput!): UpdateFunctionPayload!
  
  # Delete a function
  deleteFunction(input: DeleteFunctionInput!): DeleteFunctionPayload!
  
  # Deploy a function
  deployFunction(input: DeployFunctionInput!): DeployFunctionPayload!
  
  # Invoke a function
  invokeFunction(input: InvokeFunctionInput!): InvokeFunctionPayload!
}

input CreateFunctionInput {
  name: String!
  description: String
  runtime: Runtime!
  handler: String!
  trigger: TriggerInput!
  resources: ResourcesInput
  environment: ID
  code: String!
}

input TriggerInput {
  type: TriggerType!
  http: HttpTriggerInput
  neo: NeoTriggerInput
  schedule: ScheduleTriggerInput
  event: EventTriggerInput
}

input HttpTriggerInput {
  path: String!
  method: HttpMethod!
  cors: Boolean!
}

input NeoTriggerInput {
  event: NeoEvent!
  contract: String
  network: String!
}

input ScheduleTriggerInput {
  schedule: String!
  timezone: String!
}

input EventTriggerInput {
  eventType: String!
  filter: JSONObject
}

input ResourcesInput {
  memory: Int!
  cpu: Float!
  timeout: Int!
}

type CreateFunctionPayload {
  function: Function!
}

input UpdateFunctionInput {
  id: ID!
  name: String
  description: String
  runtime: Runtime
  handler: String
  trigger: TriggerInput
  resources: ResourcesInput
  environment: ID
  code: String
}

type UpdateFunctionPayload {
  function: Function!
}

input DeleteFunctionInput {
  id: ID!
}

type DeleteFunctionPayload {
  id: ID!
}

input DeployFunctionInput {
  id: ID!
  version: String!
}

type DeployFunctionPayload {
  deployment: Deployment!
}

input InvokeFunctionInput {
  id: ID!
  params: JSONObject
}

type InvokeFunctionPayload {
  result: JSONObject!
}
```

### Service Mutations

```graphql
type Mutation {
  # Create a service
  createService(input: CreateServiceInput!): CreateServicePayload!
  
  # Update a service
  updateService(input: UpdateServiceInput!): UpdateServicePayload!
  
  # Delete a service
  deleteService(input: DeleteServiceInput!): DeleteServicePayload!
  
  # Deploy a service
  deployService(input: DeployServiceInput!): DeployServicePayload!
}

input CreateServiceInput {
  name: String!
  description: String
  type: ServiceType!
  config: JSONObject!
  resources: ResourcesInput
  environment: ID
}

type CreateServicePayload {
  service: Service!
}

input UpdateServiceInput {
  id: ID!
  name: String
  description: String
  type: ServiceType
  config: JSONObject
  resources: ResourcesInput
  environment: ID
}

type UpdateServicePayload {
  service: Service!
}

input DeleteServiceInput {
  id: ID!
}

type DeleteServicePayload {
  id: ID!
}

input DeployServiceInput {
  id: ID!
  version: String!
}

type DeployServicePayload {
  deployment: Deployment!
}
```

### Environment Mutations

```graphql
type Mutation {
  # Create an environment
  createEnvironment(input: CreateEnvironmentInput!): CreateEnvironmentPayload!
  
  # Update an environment
  updateEnvironment(input: UpdateEnvironmentInput!): UpdateEnvironmentPayload!
  
  # Delete an environment
  deleteEnvironment(input: DeleteEnvironmentInput!): DeleteEnvironmentPayload!
}

input CreateEnvironmentInput {
  name: String!
  network: String!
  rpcUrl: String!
  region: String!
  logLevel: LogLevel!
  debug: Boolean!
}

type CreateEnvironmentPayload {
  environment: Environment!
}

input UpdateEnvironmentInput {
  id: ID!
  name: String
  network: String
  rpcUrl: String
  region: String
  logLevel: LogLevel
  debug: Boolean
}

type UpdateEnvironmentPayload {
  environment: Environment!
}

input DeleteEnvironmentInput {
  id: ID!
}

type DeleteEnvironmentPayload {
  id: ID!
}
```

## Subscriptions

### Function Subscriptions

```graphql
type Subscription {
  # Subscribe to function logs
  functionLogs(functionId: ID!): Log!
  
  # Subscribe to function metrics
  functionMetrics(functionId: ID!): Metric!
  
  # Subscribe to function deployments
  functionDeployments(functionId: ID!): Deployment!
}
```

### Service Subscriptions

```graphql
type Subscription {
  # Subscribe to service logs
  serviceLogs(serviceId: ID!): Log!
  
  # Subscribe to service metrics
  serviceMetrics(serviceId: ID!): Metric!
  
  # Subscribe to service deployments
  serviceDeployments(serviceId: ID!): Deployment!
}
```

### Neo N3 Subscriptions

```graphql
type Subscription {
  # Subscribe to Neo N3 new block events
  neoNewBlock: NeoBlock!
  
  # Subscribe to Neo N3 new transaction events
  neoNewTransaction: NeoTransaction!
  
  # Subscribe to Neo N3 contract notification events
  neoContractNotification(contract: String!): NeoContractNotification!
}

type NeoBlock {
  height: Int!
  hash: String!
  time: DateTime!
  transactions: Int!
}

type NeoTransaction {
  hash: String!
  blockHeight: Int!
  time: DateTime!
  sender: String!
  receiver: String!
  asset: String!
  amount: Float!
}

type NeoContractNotification {
  contract: String!
  event: String!
  params: JSONObject!
  blockHeight: Int!
  time: DateTime!
}
```

## Directives

```graphql
# Requires authentication
directive @auth on FIELD_DEFINITION

# Requires a specific role
directive @role(role: Role!) on FIELD_DEFINITION

# Requires ownership of the resource
directive @owner on FIELD_DEFINITION

# Deprecated field or enum value
directive @deprecated(
  reason: String = "No longer supported"
) on FIELD_DEFINITION | ENUM_VALUE
```

## Examples

### Query Functions

```graphql
query GetFunctions {
  functions(
    first: 10,
    filter: {
      runtime: JAVASCRIPT,
      triggerType: NEO
    },
    orderBy: {
      field: CREATED_AT,
      direction: DESC
    }
  ) {
    edges {
      node {
        id
        name
        description
        runtime
        handler
        trigger {
          type
          ... on NeoTrigger {
            event
            contract
            network
          }
        }
        owner {
          username
        }
        createdAt
      }
      cursor
    }
    pageInfo {
      hasNextPage
      endCursor
    }
    totalCount
  }
}
```

### Create Function

```graphql
mutation CreateFunction {
  createFunction(
    input: {
      name: "neo-block-monitor",
      description: "Monitor Neo N3 blocks",
      runtime: JAVASCRIPT,
      handler: "functions/neo-block-monitor.js",
      trigger: {
        type: NEO,
        neo: {
          event: NEW_BLOCK,
          network: "mainnet"
        }
      },
      resources: {
        memory: 128,
        cpu: 0.1,
        timeout: 30
      },
      code: "export default async function(event, context) { console.log('New block:', event.data.blockHeight); }"
    }
  ) {
    function {
      id
      name
      description
      runtime
      handler
      trigger {
        type
        ... on NeoTrigger {
          event
          network
        }
      }
      createdAt
    }
  }
}
```

### Invoke Function

```graphql
mutation InvokeFunction {
  invokeFunction(
    input: {
      id: "function-id",
      params: {
        name: "Neo"
      }
    }
  ) {
    result
  }
}
```

### Subscribe to Function Logs

```graphql
subscription FunctionLogs {
  functionLogs(functionId: "function-id") {
    id
    level
    message
    timestamp
    metadata
  }
}
```

For more information, see the [API Reference](../api-reference.md) and [Architecture](../architecture.md) documents.
