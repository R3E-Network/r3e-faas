# Service Development Guide for Neo N3 FaaS Platform

This guide provides detailed information about developing services for the Neo N3 FaaS platform.

## Table of Contents

1. [Introduction](#introduction)
2. [Service Basics](#service-basics)
3. [Service Types](#service-types)
4. [Service Configuration](#service-configuration)
5. [Service Lifecycle](#service-lifecycle)
6. [Service API](#service-api)
7. [Service Discovery](#service-discovery)
8. [Testing and Debugging](#testing-and-debugging)
9. [Best Practices](#best-practices)

## Introduction

Services are a core component of the Neo N3 FaaS platform. They provide reusable functionality that can be used by functions and other services. Services can be used to access external data, perform complex computations, or provide specialized functionality.

## Service Basics

### Service Structure

A service is a modular component that provides a specific functionality. Services can be implemented in Rust or JavaScript, depending on the requirements.

#### Rust Service

```rust
// src/lib.rs
use r3e_core::service::{Service, ServiceContext, ServiceResult};

#[derive(Debug)]
pub struct MyService {
    config: MyServiceConfig,
}

#[derive(Debug, Deserialize)]
pub struct MyServiceConfig {
    pub api_key: String,
    pub endpoint: String,
}

impl Service for MyService {
    type Config = MyServiceConfig;
    
    fn new(config: Self::Config) -> Self {
        MyService { config }
    }
    
    async fn init(&self, ctx: &ServiceContext) -> ServiceResult<()> {
        // Initialize service
        Ok(())
    }
    
    async fn start(&self, ctx: &ServiceContext) -> ServiceResult<()> {
        // Start service
        Ok(())
    }
    
    async fn stop(&self, ctx: &ServiceContext) -> ServiceResult<()> {
        // Stop service
        Ok(())
    }
}

// Register service
#[no_mangle]
pub fn register() -> Box<dyn Service> {
    Box::new(MyService::new(MyServiceConfig {
        api_key: "default-api-key".to_string(),
        endpoint: "https://api.example.com".to_string(),
    }))
}
```

#### JavaScript Service

```javascript
// service.js
export default class MyService {
  constructor(config) {
    this.config = config;
  }
  
  async init(ctx) {
    // Initialize service
    ctx.log.info("Initializing service");
    return true;
  }
  
  async start(ctx) {
    // Start service
    ctx.log.info("Starting service");
    return true;
  }
  
  async stop(ctx) {
    // Stop service
    ctx.log.info("Stopping service");
    return true;
  }
  
  async getData(params) {
    // Get data from external source
    const response = await fetch(`${this.config.endpoint}/data?key=${this.config.apiKey}`);
    return response.json();
  }
}
```

### Service Context

The service context provides access to platform services and utilities:

- **log**: Logging utilities
- **env**: Environment variables
- **storage**: Persistent storage
- **neo**: Neo N3 blockchain access
- **registry**: Service registry

```rust
// Using the service context
async fn start(&self, ctx: &ServiceContext) -> ServiceResult<()> {
    // Log information
    ctx.log.info("Starting service");
    
    // Access environment variables
    let environment = ctx.env.get("NODE_ENV").unwrap_or("development");
    
    // Access persistent storage
    let value = ctx.storage.get("key").await?;
    ctx.storage.set("key", "value").await?;
    
    // Access Neo N3 blockchain
    let block_height = ctx.neo.get_current_block_height().await?;
    
    // Access service registry
    let service = ctx.registry.get_service("oracle").await?;
    
    Ok(())
}
```

## Service Types

The Neo N3 FaaS platform supports several types of services:

### Standard Services

Standard services provide general-purpose functionality that can be used by functions and other services.

```rust
// Standard service
#[derive(Debug)]
pub struct StandardService {
    config: StandardServiceConfig,
}

#[derive(Debug, Deserialize)]
pub struct StandardServiceConfig {
    pub name: String,
}

impl Service for StandardService {
    type Config = StandardServiceConfig;
    
    // Implementation details
}
```

### Oracle Services

Oracle services provide access to external data sources such as price feeds, random number generation, and other real-world data.

```rust
// Oracle service
#[derive(Debug)]
pub struct OracleService {
    config: OracleServiceConfig,
}

#[derive(Debug, Deserialize)]
pub struct OracleServiceConfig {
    pub sources: Vec<String>,
    pub assets: Vec<String>,
    pub update_interval: u64,
}

impl Service for OracleService {
    type Config = OracleServiceConfig;
    
    // Implementation details
}

impl OracleService {
    pub async fn get_price(&self, asset: &str, currency: &str) -> Result<f64, Error> {
        // Get price from external sources
    }
    
    pub async fn get_random_number(&self, min: u64, max: u64) -> Result<u64, Error> {
        // Generate random number
    }
}
```

### TEE Services

TEE services provide secure execution environments for sensitive code and data.

```rust
// TEE service
#[derive(Debug)]
pub struct TeeService {
    config: TeeServiceConfig,
}

#[derive(Debug, Deserialize)]
pub struct TeeServiceConfig {
    pub provider: String,
}

impl Service for TeeService {
    type Config = TeeServiceConfig;
    
    // Implementation details
}

impl TeeService {
    pub async fn execute<F, R>(&self, f: F) -> Result<R, Error>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        // Execute function in TEE
    }
}
```

### Blockchain Services

Blockchain services provide access to blockchain networks and smart contracts.

```rust
// Blockchain service
#[derive(Debug)]
pub struct BlockchainService {
    config: BlockchainServiceConfig,
}

#[derive(Debug, Deserialize)]
pub struct BlockchainServiceConfig {
    pub network: String,
    pub rpc_url: String,
}

impl Service for BlockchainService {
    type Config = BlockchainServiceConfig;
    
    // Implementation details
}

impl BlockchainService {
    pub async fn get_block(&self, height: u64) -> Result<Block, Error> {
        // Get block from blockchain
    }
    
    pub async fn get_transaction(&self, hash: &str) -> Result<Transaction, Error> {
        // Get transaction from blockchain
    }
    
    pub async fn call_contract(&self, contract: &str, method: &str, args: &[Value]) -> Result<Value, Error> {
        // Call contract method
    }
}
```

## Service Configuration

Services are configured using the `r3e.yaml` file in the project root directory.

```yaml
services:
  standard-service:
    type: standard
    config:
      name: "standard-service"
  
  oracle-service:
    type: oracle
    config:
      sources: ["coinGecko", "binance"]
      assets: ["NEO", "GAS"]
      update_interval: 60
  
  tee-service:
    type: tee
    config:
      provider: "sgx"
  
  blockchain-service:
    type: blockchain
    config:
      network: "mainnet"
      rpc_url: "https://n3seed1.ngd.network:10332"
```

### Configuration Options

#### Standard Service

```yaml
type: standard
config:
  name: "standard-service"
  # Additional configuration options
```

#### Oracle Service

```yaml
type: oracle
config:
  sources: ["coinGecko", "binance"] # Data sources
  assets: ["NEO", "GAS"] # Assets to track
  update_interval: 60 # Update interval in seconds
  cache_ttl: 300 # Cache time-to-live in seconds
  rate_limit: 100 # Rate limit in requests per minute
```

#### TEE Service

```yaml
type: tee
config:
  provider: "sgx" # TEE provider (sgx, sev, trustzone)
  enclave_path: "/path/to/enclave.so" # Path to enclave binary
  attestation: true # Enable attestation
  key_management: true # Enable key management
```

#### Blockchain Service

```yaml
type: blockchain
config:
  network: "mainnet" # Network (mainnet, testnet)
  rpc_url: "https://n3seed1.ngd.network:10332" # RPC URL
  wallet_path: "/path/to/wallet.json" # Path to wallet file
  wallet_password: "${WALLET_PASSWORD}" # Wallet password (reference to secret)
```

## Service Lifecycle

Services go through several stages in their lifecycle:

1. **Created**: Service is created but not initialized
2. **Initialized**: Service is initialized but not started
3. **Started**: Service is started and running
4. **Stopped**: Service is stopped but can be restarted
5. **Destroyed**: Service is destroyed and cannot be restarted

```rust
// Service lifecycle
impl Service for MyService {
    // ...
    
    async fn init(&self, ctx: &ServiceContext) -> ServiceResult<()> {
        // Initialize service
        ctx.log.info("Initializing service");
        
        // Perform initialization tasks
        // - Connect to external services
        // - Load configuration
        // - Initialize resources
        
        Ok(())
    }
    
    async fn start(&self, ctx: &ServiceContext) -> ServiceResult<()> {
        // Start service
        ctx.log.info("Starting service");
        
        // Perform start tasks
        // - Start background tasks
        // - Register with service registry
        // - Start listening for events
        
        Ok(())
    }
    
    async fn stop(&self, ctx: &ServiceContext) -> ServiceResult<()> {
        // Stop service
        ctx.log.info("Stopping service");
        
        // Perform stop tasks
        // - Stop background tasks
        // - Unregister from service registry
        // - Close connections
        
        Ok(())
    }
    
    async fn destroy(&self, ctx: &ServiceContext) -> ServiceResult<()> {
        // Destroy service
        ctx.log.info("Destroying service");
        
        // Perform cleanup tasks
        // - Release resources
        // - Delete temporary files
        
        Ok(())
    }
}
```

## Service API

Services can expose APIs that can be used by functions and other services.

### Rust API

```rust
// Service API
impl MyService {
    pub async fn get_data(&self, params: &Params) -> Result<Data, Error> {
        // Get data
    }
    
    pub async fn process_data(&self, data: &Data) -> Result<ProcessedData, Error> {
        // Process data
    }
}

// Using the service API
async fn handle_request(ctx: &ServiceContext) -> Result<Response, Error> {
    let service = ctx.registry.get_service::<MyService>("my-service").await?;
    let data = service.get_data(&params).await?;
    let processed_data = service.process_data(&data).await?;
    Ok(Response::new(processed_data))
}
```

### JavaScript API

```javascript
// Service API
class MyService {
  // ...
  
  async getData(params) {
    // Get data
  }
  
  async processData(data) {
    // Process data
  }
}

// Using the service API
async function handleRequest(ctx) {
  const service = await ctx.registry.getService("my-service");
  const data = await service.getData(params);
  const processedData = await service.processData(data);
  return { processedData };
}
```

### HTTP API

Services can expose HTTP APIs that can be accessed by external clients.

```rust
// HTTP API
async fn handle_http_request(req: Request) -> Result<Response, Error> {
    match (req.method(), req.path()) {
        (Method::GET, "/data") => {
            let params = req.query::<Params>()?;
            let data = service.get_data(&params).await?;
            Ok(Response::json(data))
        }
        (Method::POST, "/data/process") => {
            let data = req.body::<Data>()?;
            let processed_data = service.process_data(&data).await?;
            Ok(Response::json(processed_data))
        }
        _ => Ok(Response::not_found()),
    }
}
```

### GraphQL API

Services can expose GraphQL APIs that provide flexible query capabilities.

```rust
// GraphQL API
#[derive(GraphQLObject)]
struct Data {
    id: String,
    name: String,
    value: f64,
}

#[derive(GraphQLInputObject)]
struct DataInput {
    name: String,
    value: f64,
}

struct Query;

#[graphql_object]
impl Query {
    async fn data(id: String) -> Result<Data, Error> {
        let data = service.get_data(&id).await?;
        Ok(data)
    }
    
    async fn data_list(limit: Option<i32>, offset: Option<i32>) -> Result<Vec<Data>, Error> {
        let data_list = service.get_data_list(limit.unwrap_or(10), offset.unwrap_or(0)).await?;
        Ok(data_list)
    }
}

struct Mutation;

#[graphql_object]
impl Mutation {
    async fn create_data(input: DataInput) -> Result<Data, Error> {
        let data = service.create_data(&input).await?;
        Ok(data)
    }
    
    async fn update_data(id: String, input: DataInput) -> Result<Data, Error> {
        let data = service.update_data(&id, &input).await?;
        Ok(data)
    }
    
    async fn delete_data(id: String) -> Result<bool, Error> {
        let success = service.delete_data(&id).await?;
        Ok(success)
    }
}
```

## Service Discovery

The Neo N3 FaaS platform provides service discovery capabilities, allowing services to discover and communicate with each other.

### Service Registry

```rust
// Service registry
async fn start(&self, ctx: &ServiceContext) -> ServiceResult<()> {
    // Register service
    ctx.registry.register_service("my-service", self).await?;
    
    // Get service
    let other_service = ctx.registry.get_service::<OtherService>("other-service").await?;
    
    // Use service
    let data = other_service.get_data(&params).await?;
    
    Ok(())
}
```

### Service Discovery API

```rust
// Service discovery API
async fn discover_services(ctx: &ServiceContext) -> Result<Vec<ServiceInfo>, Error> {
    // Discover all services
    let services = ctx.registry.get_services().await?;
    
    // Discover services by type
    let oracle_services = ctx.registry.get_services_by_type("oracle").await?;
    
    // Discover services by tag
    let price_services = ctx.registry.get_services_by_tag("price").await?;
    
    Ok(services)
}
```

## Testing and Debugging

### Unit Testing

Services can be unit tested using standard Rust or JavaScript testing frameworks.

```rust
// Unit testing
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_get_data() {
        // Create service
        let config = MyServiceConfig {
            api_key: "test-api-key".to_string(),
            endpoint: "https://api.example.com".to_string(),
        };
        let service = MyService::new(config);
        
        // Create mock context
        let ctx = MockServiceContext::new();
        
        // Initialize service
        service.init(&ctx).await.unwrap();
        
        // Test service
        let params = Params { id: "test-id".to_string() };
        let data = service.get_data(&params).await.unwrap();
        
        // Assert results
        assert_eq!(data.id, "test-id");
        assert_eq!(data.name, "Test Data");
        assert_eq!(data.value, 42.0);
    }
}
```

### Integration Testing

Services can be integration tested using the platform's testing utilities.

```rust
// Integration testing
#[tokio::test]
async fn test_service_integration() {
    // Create test environment
    let env = TestEnvironment::new();
    
    // Deploy service
    let service_id = env.deploy_service("my-service", MyServiceConfig {
        api_key: "test-api-key".to_string(),
        endpoint: "https://api.example.com".to_string(),
    }).await.unwrap();
    
    // Get service
    let service = env.get_service::<MyService>(service_id).await.unwrap();
    
    // Test service
    let params = Params { id: "test-id".to_string() };
    let data = service.get_data(&params).await.unwrap();
    
    // Assert results
    assert_eq!(data.id, "test-id");
    assert_eq!(data.name, "Test Data");
    assert_eq!(data.value, 42.0);
}
```

### Debugging

The Neo N3 FaaS platform provides tools for debugging services.

```bash
# View service logs
r3e-faas-cli logs --service my-service

# Follow service logs
r3e-faas-cli logs --service my-service --follow

# View service metrics
r3e-faas-cli metrics --service my-service

# View service status
r3e-faas-cli status --service my-service

# Inspect service configuration
r3e-faas-cli inspect --service my-service
```

## Best Practices

### Service Design

- **Keep services focused**: Each service should have a clear and specific purpose.
- **Design for reusability**: Services should be designed to be reused by multiple functions and other services.
- **Use dependency injection**: Services should receive their dependencies through constructor parameters or configuration.
- **Follow the single responsibility principle**: Each service should have a single responsibility.
- **Use interfaces**: Define clear interfaces for your services to make them easier to use and test.

### Performance

- **Optimize for throughput**: Services should be designed to handle high throughput.
- **Use caching**: Cache expensive operations to improve performance.
- **Implement connection pooling**: Reuse connections to external services to improve performance.
- **Use asynchronous programming**: Use asynchronous programming to improve concurrency.
- **Implement timeouts**: Implement timeouts for external service calls to prevent hanging services.

### Security

- **Follow the principle of least privilege**: Services should only have the permissions they need.
- **Validate input**: Validate all input to prevent security vulnerabilities.
- **Use secure communication**: Use HTTPS for external service calls.
- **Implement proper authentication and authorization**: Verify the identity of clients and ensure they have the necessary permissions.
- **Protect sensitive data**: Encrypt sensitive data at rest and in transit.

### Reliability

- **Implement retry logic**: Implement retry logic for external service calls to handle transient failures.
- **Use circuit breakers**: Use circuit breakers to prevent cascading failures.
- **Implement graceful degradation**: Services should degrade gracefully when dependencies are unavailable.
- **Monitor service health**: Monitor service health to detect and address issues.
- **Implement proper error handling**: Handle errors gracefully and provide meaningful error messages.

### Monitoring

- **Log important events**: Log important events to help with debugging and monitoring.
- **Use structured logging**: Use structured logging to make logs easier to search and analyze.
- **Monitor service performance**: Monitor service performance to identify bottlenecks and issues.
- **Set up alerts**: Set up alerts for abnormal behavior or errors.
- **Implement distributed tracing**: Implement distributed tracing to track requests across multiple services.

For more information, see the [API Reference](../api-reference.md) and [Architecture](../architecture.md) documents.
