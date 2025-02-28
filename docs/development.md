# R3E FaaS Development Guide

This guide provides detailed information for developers who want to contribute to the R3E FaaS platform or extend its functionality.

## Development Environment Setup

### Prerequisites

- Rust 1.60+ with Cargo
- Node.js 16+
- Docker and Docker Compose (optional, for containerized development)
- Git

### Clone the Repository

```bash
git clone https://github.com/R3E-Network/r3e-faas.git
cd r3e-faas
```

### Using DevContainer (Recommended)

The easiest way to set up a development environment is to use the provided DevContainer configuration with Visual Studio Code:

1. Install the [Remote - Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension in VS Code
2. Open the cloned repository in VS Code
3. Click "Reopen in Container" when prompted, or use the command palette (F1) and select "Remote-Containers: Reopen in Container"

The DevContainer includes all necessary dependencies and tools for development.

### Manual Setup

If you prefer not to use DevContainer, you can set up the development environment manually:

1. Install Rust and Cargo:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install required dependencies:
   ```bash
   # Ubuntu/Debian
   sudo apt-get update
   sudo apt-get install -y build-essential pkg-config libssl-dev clang

   # macOS
   brew install openssl pkg-config

   # Windows
   # Install Visual Studio Build Tools with C++ support
   ```

3. Build the project:
   ```bash
   cargo build
   ```

### Using Docker Compose

You can also use Docker Compose for development:

```bash
# Start development environment
docker-compose -f docker-compose.dev.yml up
```

## Project Structure

The R3E FaaS platform is organized as a Rust workspace with multiple crates:

### Core Crates

- `r3e-core`: Common types, traits, and utilities
- `r3e-config`: Configuration management
- `r3e-store`: Storage abstractions and implementations
- `r3e-event`: Event handling and processing

### Service Crates

- `r3e-built-in-services`: Consolidated services (balance, identity, pricing, etc.)
- `r3e-neo-services`: Neo N3 blockchain integration
- `r3e-oracle`: Oracle services for external data
- `r3e-tee`: Trusted Execution Environment implementation
- `r3e-secrets`: Secret management with encryption
- `r3e-zk`: Zero-Knowledge Computing service
- `r3e-fhe`: Fully Homomorphic Encryption service

### Runtime Crates

- `r3e-deno`: JavaScript runtime based on deno-core with V8 engine
- `r3e-worker`: Worker node implementation

### API and Utilities

- `r3e-api`: HTTP API server

### Main Application

- `r3e`: Command-line interface and entry point

## Development Workflow

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p r3e-neo-services

# Run a specific test
cargo test -p r3e-neo-services gas_bank_test

# Run tests with verbose output
cargo test -- --nocapture
```

### Code Style and Linting

The project uses Rustfmt for code formatting and Clippy for linting:

```bash
# Format code
cargo fmt

# Run linter
cargo clippy
```

### Building Documentation

```bash
# Generate documentation
cargo doc --no-deps

# Open documentation in browser
cargo doc --no-deps --open
```

## Adding New Features

### Adding a New Service

1. Create a new crate or add to an existing one:
   ```bash
   # Create a new crate
   cargo new --lib r3e-new-service
   ```

2. Add the crate to the workspace in `Cargo.toml`:
   ```toml
   [workspace]
   members = [
       # ...
       "r3e-new-service",
   ]
   ```

3. Implement the service following the project's architecture:
   - Define types in `types.rs`
   - Implement error handling in `error.rs`
   - Create a service interface in `service.rs`
   - Implement storage in `storage/mod.rs`

4. Add tests for your service:
   - Unit tests within the crate
   - Integration tests in the `tests` directory

5. Expose the service through the JavaScript API:
   - Add JavaScript bindings in `r3e-deno/src/ext/`
   - Add JavaScript API in `r3e-deno/src/js/`

### Adding a New Provider

To add a new provider for an existing service (e.g., a new ZK provider):

1. Create a new file in the provider directory:
   ```bash
   touch r3e-zk/src/provider/new_provider.rs
   ```

2. Implement the provider trait:
   ```rust
   use crate::provider::Provider;
   
   pub struct NewProvider {
       // Provider-specific fields
   }
   
   impl Provider for NewProvider {
       // Implement required methods
   }
   ```

3. Register the provider in the provider module:
   ```rust
   // In r3e-zk/src/provider/mod.rs
   pub mod new_provider;
   pub use new_provider::NewProvider;
   ```

4. Add tests for the new provider:
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_new_provider() {
           // Test the provider
       }
   }
   ```

## JavaScript API Development

### Adding a New JavaScript API

1. Create a new JavaScript file in `r3e-deno/src/js/`:
   ```bash
   touch r3e-deno/src/js/new_service.js
   ```

2. Implement the JavaScript API:
   ```javascript
   // r3e-deno/src/js/new_service.js
   
   // Export the API
   export const newService = {
     async someFunction(param1, param2) {
       // Implementation
       const result = await Deno.core.ops.op_new_service_function(param1, param2);
       return result;
     }
   };
   ```

3. Create Rust bindings in `r3e-deno/src/ext/`:
   ```bash
   touch r3e-deno/src/ext/new_service.rs
   ```

4. Implement the Rust bindings:
   ```rust
   // r3e-deno/src/ext/new_service.rs
   
   use deno_core::op2;
   use deno_core::error::AnyError;
   use serde::{Deserialize, Serialize};
   
   #[op2]
   #[serde]
   pub fn op_new_service_function(param1: String, param2: u64) -> Result<String, AnyError> {
       // Implementation
       Ok("result".to_string())
   }
   ```

5. Register the extension in `r3e-deno/src/ext/mod.rs`:
   ```rust
   // r3e-deno/src/ext/mod.rs
   
   pub mod new_service;
   ```

6. Expose the API in `r3e-deno/src/js/r3e.js`:
   ```javascript
   // r3e-deno/src/js/r3e.js
   
   import { newService } from "./new_service.js";
   
   // Export the R3E API
   export const r3e = {
     // ...
     newService,
   };
   ```

## Testing

### Unit Testing

Write unit tests for individual components:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_component() {
        // Test implementation
        assert_eq!(2 + 2, 4);
    }
    
    #[tokio::test]
    async fn test_async_component() {
        // Async test implementation
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

### Integration Testing

Write integration tests in the `tests` directory:

```rust
// tests/integration/new_service_test.rs

use r3e_new_service::service::NewService;
use r3e_new_service::storage::InMemoryStorage;

#[tokio::test]
async fn test_new_service_integration() {
    // Set up test environment
    let storage = Arc::new(InMemoryStorage::new());
    let service = NewService::new(storage).unwrap();
    
    // Test the service
    let result = service.some_function("param").await;
    assert!(result.is_ok());
}
```

### Mock Implementations

Use mock implementations for testing:

```rust
// Mock RPC client for testing
struct MockRpcClient {
    // Mock fields
}

impl RpcClient for MockRpcClient {
    async fn call_rpc(&self, method: &str, params: &[Value]) -> Result<Value, Error> {
        // Mock implementation
        Ok(json!({"result": "mock_result"}))
    }
}
```

## Debugging

### Logging

The project uses the `log` crate for logging. Add log statements to your code:

```rust
use log::{debug, info, warn, error};

fn some_function() {
    debug!("Debug information");
    info!("Informational message");
    warn!("Warning message");
    error!("Error message");
}
```

Configure the log level in your environment:

```bash
# Set log level to debug
export RUST_LOG=debug

# Run with debug logging
RUST_LOG=debug cargo run
```

### Debugging with VS Code

1. Install the [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) extension
2. Create a `.vscode/launch.json` file:
   ```json
   {
     "version": "0.2.0",
     "configurations": [
       {
         "type": "lldb",
         "request": "launch",
         "name": "Debug executable",
         "cargo": {
           "args": ["build", "--bin=r3e"],
           "filter": {
             "name": "r3e",
             "kind": "bin"
           }
         },
         "args": [],
         "cwd": "${workspaceFolder}"
       },
       {
         "type": "lldb",
         "request": "launch",
         "name": "Debug unit tests",
         "cargo": {
           "args": ["test", "--no-run", "--lib"],
           "filter": {
             "name": "r3e-core",
             "kind": "lib"
           }
         },
         "args": [],
         "cwd": "${workspaceFolder}"
       }
     ]
   }
   ```

3. Set breakpoints in your code and start debugging

## Performance Profiling

### Using Flamegraph

1. Install the flamegraph tool:
   ```bash
   cargo install flamegraph
   ```

2. Generate a flamegraph:
   ```bash
   cargo flamegraph --bin r3e
   ```

3. Open the generated `flamegraph.svg` file in a browser

## Continuous Integration

The project uses GitHub Actions for CI/CD. The workflow is defined in `.github/workflows/build-and-test.yml`.

To run the CI checks locally:

```bash
# Install act
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash

# Run GitHub Actions locally
act
```

## Documentation

### Code Documentation

Document your code using Rustdoc:

```rust
/// This function does something important
///
/// # Arguments
///
/// * `param1` - The first parameter
/// * `param2` - The second parameter
///
/// # Returns
///
/// A Result containing the operation result or an error
///
/// # Examples
///
/// ```
/// let result = some_function("value", 42);
/// assert!(result.is_ok());
/// ```
pub fn some_function(param1: &str, param2: u64) -> Result<String, Error> {
    // Implementation
}
```

### User Documentation

Update user documentation in the `docs` directory:

- `api-reference.md`: API documentation
- `quickstart.md`: Quick start guide
- `installation.md`: Installation instructions
- `docker-development.md`: Docker development guide
- `docker-production.md`: Docker production guide
- `zk-service.md`: Zero-Knowledge Computing service documentation
- `fhe-service.md`: Fully Homomorphic Encryption service documentation

## Release Process

### Creating a Release

1. Update version numbers in `Cargo.toml` files
2. Update the CHANGELOG.md file
3. Create a git tag:
   ```bash
   git tag -a v0.1.0 -m "Release v0.1.0"
   git push origin v0.1.0
   ```

4. Create a GitHub release with release notes

### Publishing Crates

```bash
# Publish a crate to crates.io
cargo publish -p r3e-core
```

## Troubleshooting

### Common Issues

- **Compilation Errors**: Make sure you have the latest dependencies and Rust version
- **Runtime Errors**: Check the logs for error messages
- **Test Failures**: Run tests with `--nocapture` to see detailed output
- **Docker Issues**: Check Docker logs and make sure all services are running

### Getting Help

- Open an issue on GitHub
- Join the community chat
- Check the documentation

## Contributing Guidelines

### Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for your changes
5. Run the test suite
6. Submit a pull request

### Code Review Process

All pull requests will be reviewed by maintainers. The review process includes:

- Code quality check
- Test coverage check
- Documentation check
- Performance impact assessment

### Commit Message Guidelines

Follow the conventional commits format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types:
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Example:
```
feat(oracle): add support for custom data sources

Add support for custom data sources in the Oracle service.
This allows users to define their own data sources and use them in their functions.

Closes #123
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
