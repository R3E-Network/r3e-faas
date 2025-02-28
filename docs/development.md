# Development Guide

This guide provides information for developers who want to contribute to the R3E FaaS platform or build applications on top of it.

## Project Structure

The R3E FaaS platform is organized as a Rust workspace with multiple crates:

- **Core Crates**:
  - `r3e-core`: Common types, traits, and utilities
  - `r3e-config`: Configuration management
  - `r3e-store`: Storage abstractions and implementations
  - `r3e-event`: Event handling and processing

- **Service Crates**:
  - `r3e-built-in-services`: Consolidated services (oracle, gas bank, TEE, etc.)
  - `r3e-neo-services`: Neo N3 blockchain integration
  - `r3e-oracle`: Oracle services for external data
  - `r3e-tee`: Trusted Execution Environment implementation
  - `r3e-secrets`: Secret management with encryption

- **Runtime Crates**:
  - `r3e-deno`: JavaScript runtime based on deno-core with V8 engine
  - `r3e-runtime`: Function execution runtime
  - `r3e-worker`: Worker node implementation
  - `r3e-scheduler`: Function scheduling and distribution

- **API and Utilities**:
  - `r3e-api`: HTTP API server
  - `r3e-proc-macros`: Procedural macros for code generation
  - `r3e-runlog`: Function execution logging
  - `r3e-stock`: Blockchain history and data querying

- **Main Application**:
  - `r3e`: Command-line interface and entry point

## Development Environment

### Setting Up a Development Environment

1. Clone the repository:

```bash
git clone https://github.com/R3E-Network/r3e-faas.git
cd r3e-faas
```

2. Install development dependencies:

```bash
# Install system dependencies
sudo apt-get update && sudo apt-get install -y \
    build-essential \
    cmake \
    libclang-dev \
    libssl-dev \
    pkg-config \
    librocksdb-dev
```

3. Build the project:

```bash
cargo build
```

### Using Docker for Development

For a containerized development environment, you can use Docker:

```bash
# Build the development Docker image
docker build -f Dockerfile.dev -t r3e-faas-dev .

# Run the container with mounted source code
docker run -v $(pwd):/app -p 8080:8080 r3e-faas-dev
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p r3e-core

# Run a specific test
cargo test -p r3e-core -- test_name
```

### Writing Tests

When writing tests, follow these guidelines:

1. Unit tests should be placed in the same file as the code they test, using the `#[cfg(test)]` attribute.
2. Integration tests should be placed in the `tests` directory of each crate.
3. Use mock implementations for external dependencies.

Example:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Arrange
        let input = 42;
        
        // Act
        let result = function_name(input);
        
        // Assert
        assert_eq!(result, expected_result);
    }
}
```

## Debugging

### Logging

The platform uses the `log` crate for logging. You can enable debug logs by setting the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug cargo run --bin r3e -- worker
```

### Debugging with VS Code

1. Install the Rust extension for VS Code.
2. Create a `.vscode/launch.json` file with the following content:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug r3e worker",
            "cargo": {
                "args": ["build", "--bin=r3e", "--package=r3e"],
                "filter": {
                    "name": "r3e",
                    "kind": "bin"
                }
            },
            "args": ["worker", "--config", "config/r3e-faas.yaml"],
            "cwd": "${workspaceFolder}",
            "env": {
                "RUST_LOG": "debug"
            }
        }
    ]
}
```

3. Press F5 to start debugging.

## Code Style and Guidelines

### Rust Code Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).
- Use `rustfmt` to format your code: `cargo fmt`.
- Use `clippy` to catch common mistakes: `cargo clippy`.
- Document public APIs with doc comments.

### JavaScript Code Style

- Use ES6+ features.
- Follow the [Airbnb JavaScript Style Guide](https://github.com/airbnb/javascript).
- Document functions with JSDoc comments.

## Pull Request Process

1. Fork the repository and create a feature branch.
2. Make your changes and add tests.
3. Ensure all tests pass: `cargo test`.
4. Format your code: `cargo fmt`.
5. Run clippy: `cargo clippy`.
6. Submit a pull request with a clear description of the changes.

## Next Steps

- [API Reference](./api-reference.md)
- [Docker Development](./docker-development.md)
- [Docker Production](./docker-production.md)
