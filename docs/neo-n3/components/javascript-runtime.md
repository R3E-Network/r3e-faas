# JavaScript Runtime

This document provides detailed information about the JavaScript runtime in the Neo N3 FaaS platform.

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Deno Core Integration](#deno-core-integration)
4. [JavaScript API](#javascript-api)
5. [Neo N3 JavaScript Bindings](#neo-n3-javascript-bindings)
6. [Oracle Service Bindings](#oracle-service-bindings)
7. [TEE Service Bindings](#tee-service-bindings)
8. [Function Execution](#function-execution)
9. [Sandboxing and Security](#sandboxing-and-security)
10. [Resource Management](#resource-management)
11. [Best Practices](#best-practices)

## Overview

The JavaScript runtime is a core component of the Neo N3 FaaS platform. It provides the execution environment for JavaScript functions, allowing developers to write and deploy serverless functions that can interact with the Neo N3 blockchain, oracle services, and TEE services.

## Architecture

The JavaScript runtime follows a modular architecture with several key components:

```
                      +------------------------+
                      |                        |
                      |   JavaScript Runtime   |
                      |                        |
                      +------------+-----------+
                                   |
                                   v
                      +------------+-----------+
                      |                        |
                      |    Deno Core (V8)      |
                      |                        |
                      +------------+-----------+
                                   |
                                   v
+----------------+    +------------+-----------+    +----------------+
|                |    |                        |    |                |
| Neo N3 Bindings|<-->|    JavaScript API      |<-->| Oracle Bindings|
|                |    |                        |    |                |
+----------------+    +------------+-----------+    +----------------+
                                   |
                                   v
+----------------+    +------------+-----------+    +----------------+
|                |    |                        |    |                |
| TEE Bindings   |<-->|    Core Bindings       |<-->| Storage Bindings|
|                |    |                        |    |                |
+----------------+    +------------------------+    +----------------+
```

- **JavaScript Runtime**: The main component that provides the execution environment for JavaScript functions.
- **Deno Core (V8)**: The JavaScript engine that executes JavaScript code.
- **JavaScript API**: The API that JavaScript functions can use to interact with the platform.
- **Neo N3 Bindings**: JavaScript bindings for interacting with the Neo N3 blockchain.
- **Oracle Bindings**: JavaScript bindings for interacting with oracle services.
- **TEE Bindings**: JavaScript bindings for interacting with TEE services.
- **Core Bindings**: JavaScript bindings for core platform services.
- **Storage Bindings**: JavaScript bindings for storage services.

## Deno Core Integration

The JavaScript runtime is built on top of Deno Core, which is a minimal JavaScript runtime based on the V8 JavaScript engine. Deno Core provides a secure and efficient execution environment for JavaScript code.

The Deno Core integration is implemented in the `r3e-deno` crate, which provides the JavaScript runtime for the platform. The `r3e-deno/src/lib.rs` file implements the main runtime, and the `r3e-deno/src/ext` directory contains the extensions that provide JavaScript bindings for various services.

```rust
// r3e-deno/src/lib.rs
use deno_core::error::AnyError;
use deno_core::op_fn_meta;
use deno_core::Extension;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;

pub struct DenoRuntime {
    js_runtime: JsRuntime,
}

impl DenoRuntime {
    pub fn new() -> Self {
        let extensions = vec![
            ext::core::init(),
            ext::neo::init(),
            ext::oracle::init(),
            ext::tee::init(),
            // ...
        ];
        
        let js_runtime = JsRuntime::new(RuntimeOptions {
            extensions,
            ..Default::default()
        });
        
        Self { js_runtime }
    }
    
    pub async fn execute_function(&mut self, code: &str, args: &[u8]) -> Result<Vec<u8>, AnyError> {
        // Execute the function with the given arguments
        // ...
    }
    
    // ...
}
```

## JavaScript API

The JavaScript API provides a set of functions and objects that JavaScript functions can use to interact with the platform. The API is organized into several modules, each providing access to different platform services.

### Core API

The Core API provides access to core platform services such as logging, environment variables, HTTP requests, and more.

```javascript
// Example of using the core API
import { log, env, http } from 'r3e';

// Logging
log.info('Hello, Neo N3 FaaS!');
log.debug('Debug information');
log.warn('Warning message');
log.error('Error message');

// Environment variables
const apiKey = env.get('API_KEY');
const debug = env.get('DEBUG', 'false') === 'true';

// HTTP requests
const response = await http.fetch('https://api.example.com/data', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${apiKey}`
  },
  body: JSON.stringify({ query: 'example' })
});

const data = await response.json();
```

### Storage API

The Storage API provides access to storage services for persisting data between function invocations.

```javascript
// Example of using the storage API
import { storage } from 'r3e';

// Store data
await storage.set('key', { value: 'example' });

// Retrieve data
const data = await storage.get('key');

// Delete data
await storage.delete('key');

// List keys
const keys = await storage.list('prefix');
```

## Neo N3 JavaScript Bindings

The Neo N3 JavaScript bindings provide access to the Neo N3 blockchain from JavaScript functions. The bindings are implemented in the `r3e-deno/src/ext/neo.rs` file and exposed to JavaScript through the `r3e-deno/src/js/neo.js` file.

```javascript
// Example of using the Neo N3 API
import { neo } from 'r3e';

// Get blockchain information
const blockCount = await neo.getBlockCount();
const block = await neo.getBlock(blockCount - 1);
const transaction = await neo.getTransaction(block.tx[0]);

// Interact with smart contracts
const contract = await neo.getContract('0x1234567890abcdef');
const result = await contract.call('balanceOf', ['NXV7ZhHiyM1aHXwvUBCta7dGNP1tHwfoQG']);

// Create and send transactions
const wallet = await neo.getWallet('my-wallet');
const tx = await wallet.createTransaction({
  type: 'transfer',
  asset: 'NEO',
  from: wallet.address,
  to: 'NZHf1NJvz1tvELGLWZjhpb3NqZJFFUYpxT',
  amount: 10
});

const signedTx = await wallet.signTransaction(tx);
const txHash = await neo.sendTransaction(signedTx);
```

The Neo N3 JavaScript bindings are implemented in Rust using the Deno Core extension system:

```rust
// r3e-deno/src/ext/neo.rs
use deno_core::error::AnyError;
use deno_core::op_fn_meta;
use deno_core::Extension;
use deno_core::OpState;
use deno_core::ZeroCopyBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct GetBlockCountArgs {}

#[derive(Debug, Serialize)]
struct GetBlockCountResult {
    block_count: u32,
}

fn op_get_block_count(
    state: &mut OpState,
    args: GetBlockCountArgs,
    _: (),
) -> Result<GetBlockCountResult, AnyError> {
    // Get the Neo client from the state
    let client = state.borrow::<NeoClient>();
    
    // Call the Neo client to get the block count
    let block_count = client.get_block_count()?;
    
    Ok(GetBlockCountResult { block_count })
}

pub fn init() -> Extension {
    Extension::builder()
        .ops(vec![
            ("op_get_block_count", op_fn_meta!(op_get_block_count)),
            // ...
        ])
        .js(include_str!("../js/neo.js"))
        .state(|state| {
            // Initialize the Neo client
            let client = NeoClient::new();
            state.put(client);
            Ok(())
        })
        .build()
}
```

## Oracle Service Bindings

The Oracle Service JavaScript bindings provide access to oracle services from JavaScript functions. The bindings are implemented in the `r3e-deno/src/ext/oracle.rs` file and exposed to JavaScript through the `r3e-deno/src/js/oracle.js` file.

```javascript
// Example of using the Oracle API
import { oracle } from 'r3e';

// Get price data
const neoPrice = await oracle.getPrice('NEO', 'USD');
const gasPrice = await oracle.getPrice('GAS', 'USD');

// Get historical price data
const neoPriceHistory = await oracle.getPriceHistory('NEO', 'USD', {
  from: '2023-01-01',
  to: '2023-01-31',
  interval: 'day'
});

// Generate random numbers
const randomNumber = await oracle.getRandomNumber(1, 100);
const randomBytes = await oracle.getRandomBytes(32);

// Get weather data
const weather = await oracle.getWeather('New York');

// Get sports results
const sportsResults = await oracle.getSportsResults('NBA');
```

The Oracle Service JavaScript bindings are implemented in Rust using the Deno Core extension system:

```rust
// r3e-deno/src/ext/oracle.rs
use deno_core::error::AnyError;
use deno_core::op_fn_meta;
use deno_core::Extension;
use deno_core::OpState;
use deno_core::ZeroCopyBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct GetPriceArgs {
    asset: String,
    currency: String,
}

#[derive(Debug, Serialize)]
struct GetPriceResult {
    price: f64,
    timestamp: u64,
}

fn op_get_price(
    state: &mut OpState,
    args: GetPriceArgs,
    _: (),
) -> Result<GetPriceResult, AnyError> {
    // Get the Oracle client from the state
    let client = state.borrow::<OracleClient>();
    
    // Call the Oracle client to get the price
    let (price, timestamp) = client.get_price(&args.asset, &args.currency)?;
    
    Ok(GetPriceResult { price, timestamp })
}

pub fn init() -> Extension {
    Extension::builder()
        .ops(vec![
            ("op_get_price", op_fn_meta!(op_get_price)),
            // ...
        ])
        .js(include_str!("../js/oracle.js"))
        .state(|state| {
            // Initialize the Oracle client
            let client = OracleClient::new();
            state.put(client);
            Ok(())
        })
        .build()
}
```

## TEE Service Bindings

The TEE Service JavaScript bindings provide access to Trusted Execution Environment (TEE) services from JavaScript functions. The bindings are implemented in the `r3e-deno/src/ext/tee.rs` file and exposed to JavaScript through the `r3e-deno/src/js/tee.js` file.

```javascript
// Example of using the TEE API
import { tee } from 'r3e';

// Execute code in TEE
const result = await tee.execute(async (secureContext) => {
  // Generate key pair
  const keyPair = await secureContext.crypto.generateKeyPair();
  
  // Sign data
  const signature = await secureContext.crypto.sign(keyPair.privateKey, 'data to sign');
  
  // Return public key and signature
  return {
    publicKey: keyPair.publicKey,
    signature
  };
});

// Verify attestation
const attestation = await tee.getAttestation();
const isValid = await tee.verifyAttestation(attestation);

// Store data in secure storage
await tee.secureStorage.set('key', { value: 'sensitive data' });

// Retrieve data from secure storage
const data = await tee.secureStorage.get('key');
```

The TEE Service JavaScript bindings are implemented in Rust using the Deno Core extension system:

```rust
// r3e-deno/src/ext/tee.rs
use deno_core::error::AnyError;
use deno_core::op_fn_meta;
use deno_core::Extension;
use deno_core::OpState;
use deno_core::ZeroCopyBuf;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct ExecuteArgs {
    code: String,
    args: Value,
}

#[derive(Debug, Serialize)]
struct ExecuteResult {
    result: Value,
}

fn op_execute(
    state: &mut OpState,
    args: ExecuteArgs,
    _: (),
) -> Result<ExecuteResult, AnyError> {
    // Get the TEE client from the state
    let client = state.borrow::<TeeClient>();
    
    // Call the TEE client to execute the code
    let result = client.execute(&args.code, &args.args)?;
    
    Ok(ExecuteResult { result })
}

pub fn init() -> Extension {
    Extension::builder()
        .ops(vec![
            ("op_execute", op_fn_meta!(op_execute)),
            // ...
        ])
        .js(include_str!("../js/tee.js"))
        .state(|state| {
            // Initialize the TEE client
            let client = TeeClient::new();
            state.put(client);
            Ok(())
        })
        .build()
}
```

## Function Execution

The JavaScript runtime executes functions in response to events or API requests. The function execution process involves several steps:

1. **Function Loading**: The function code is loaded from storage.
2. **Function Compilation**: The function code is compiled by the V8 JavaScript engine.
3. **Function Execution**: The function is executed with the provided event and context.
4. **Result Processing**: The function result is processed and returned.

```rust
// r3e-deno/src/lib.rs
impl DenoRuntime {
    // ...
    
    pub async fn execute_function(&mut self, code: &str, event: &[u8]) -> Result<Vec<u8>, AnyError> {
        // Load the function code
        let module_id = self.js_runtime.load_module("function.js", code)?;
        
        // Create the event and context objects
        let event_str = std::str::from_utf8(event)?;
        let context_str = r#"{ "neo": {}, "oracle": {}, "tee": {} }"#;
        
        // Execute the function
        let result = self.js_runtime.execute_module(module_id)?;
        let function = self.js_runtime.get_module_namespace(module_id)?;
        let args = vec![event_str, context_str];
        let result = self.js_runtime.call_function(&function, "default", args)?;
        
        // Process the result
        let result_bytes = serde_json::to_vec(&result)?;
        
        Ok(result_bytes)
    }
    
    // ...
}
```

## Sandboxing and Security

The JavaScript runtime provides a sandboxed environment for executing JavaScript functions. The sandbox restricts access to system resources and provides controlled access to platform services.

### Permissions

The JavaScript runtime uses a permission system to control access to platform services. Functions can request permissions to access specific services, and the platform can grant or deny these permissions based on the function's configuration.

```javascript
// Example of using permissions
import { permissions } from 'r3e';

// Request permissions
await permissions.request('neo:read');
await permissions.request('oracle:price');
await permissions.request('tee:execute');

// Check permissions
const hasNeoReadPermission = await permissions.check('neo:read');
const hasOraclePricePermission = await permissions.check('oracle:price');
const hasTeeExecutePermission = await permissions.check('tee:execute');
```

### Resource Isolation

The JavaScript runtime isolates functions from each other and from the host system. Each function runs in its own V8 isolate, which provides memory isolation and prevents functions from interfering with each other.

### Memory Limits

The JavaScript runtime enforces memory limits to prevent functions from consuming too much memory. Functions that exceed their memory limit are terminated.

```javascript
// Example of memory limit configuration
{
  "function": {
    "name": "my-function",
    "memory_limit": 128, // MB
    // ...
  }
}
```

## Resource Management

The JavaScript runtime manages resources such as CPU, memory, and network connections to ensure fair and efficient use of system resources.

### CPU Limits

The JavaScript runtime enforces CPU limits to prevent functions from consuming too much CPU time. Functions that exceed their CPU limit are terminated.

```javascript
// Example of CPU limit configuration
{
  "function": {
    "name": "my-function",
    "cpu_limit": 100, // ms
    // ...
  }
}
```

### Concurrency

The JavaScript runtime supports concurrent execution of functions. The platform can execute multiple functions in parallel, with each function running in its own V8 isolate.

```javascript
// Example of concurrency configuration
{
  "worker": {
    "max_concurrent_functions": 10,
    // ...
  }
}
```

### Resource Cleanup

The JavaScript runtime automatically cleans up resources when a function completes or is terminated. This includes closing network connections, releasing memory, and cleaning up temporary files.

## Best Practices

### Error Handling

Functions should handle errors gracefully and provide meaningful error messages.

```javascript
// Example of error handling
export default async function(event, context) {
  try {
    const blockCount = await context.neo.getBlockCount();
    return { blockCount };
  } catch (error) {
    console.error(`Error: ${error.message}`);
    return { error: error.message };
  }
}
```

### Async/Await

Functions should use async/await for asynchronous operations to improve readability and error handling.

```javascript
// Example of using async/await
export default async function(event, context) {
  const blockCount = await context.neo.getBlockCount();
  const block = await context.neo.getBlock(blockCount - 1);
  const transaction = await context.neo.getTransaction(block.tx[0]);
  
  return {
    blockCount,
    block,
    transaction
  };
}
```

### Resource Cleanup

Functions should clean up resources when they are no longer needed.

```javascript
// Example of resource cleanup
export default async function(event, context) {
  let client = null;
  
  try {
    client = await context.neo.createClient();
    const blockCount = await client.getBlockCount();
    return { blockCount };
  } catch (error) {
    console.error(`Error: ${error.message}`);
    return { error: error.message };
  } finally {
    if (client) {
      await client.close();
    }
  }
}
```

### Caching

Functions should use caching to reduce the number of external service calls and improve performance.

```javascript
// Example of caching
export default async function(event, context) {
  const cacheKey = "blockcount";
  let blockCount = await context.cache.get(cacheKey);
  
  if (!blockCount) {
    blockCount = await context.neo.getBlockCount();
    await context.cache.set(cacheKey, blockCount, { ttl: 60 }); // Cache for 60 seconds
  }
  
  return { blockCount };
}
```

### Retry Logic

Functions should implement retry logic for external service calls to handle temporary failures.

```javascript
// Example of retry logic
export default async function(event, context) {
  const maxRetries = 3;
  let retries = 0;
  
  while (retries < maxRetries) {
    try {
      const blockCount = await context.neo.getBlockCount();
      return { blockCount };
    } catch (error) {
      retries++;
      if (retries >= maxRetries) throw error;
      await new Promise(resolve => setTimeout(resolve, 1000 * Math.pow(2, retries)));
    }
  }
}
```

### Input Validation

Functions should validate input parameters to prevent errors and security vulnerabilities.

```javascript
// Example of input validation
export default async function(event, context) {
  // Validate input parameters
  if (!event.address || typeof event.address !== 'string') {
    return { error: 'Invalid address parameter' };
  }
  
  // Use validated parameters
  const balance = await context.neo.getBalance(event.address);
  
  return { balance };
}
```

### Logging

Functions should use logging to provide visibility into their execution.

```javascript
// Example of logging
export default async function(event, context) {
  console.log('Function started', { event });
  
  try {
    const blockCount = await context.neo.getBlockCount();
    console.log('Got block count', { blockCount });
    
    const block = await context.neo.getBlock(blockCount - 1);
    console.log('Got block', { blockHash: block.hash });
    
    return {
      blockCount,
      block
    };
  } catch (error) {
    console.error('Function error', { error: error.message });
    return { error: error.message };
  } finally {
    console.log('Function completed');
  }
}
```
