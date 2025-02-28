# R3E Core

Core functionality and shared types for the R3E FaaS platform.

## Features

- V8 JavaScript engine initialization and finalization
- Signal handling for graceful shutdown
- Platform detection
- Version information
- Common error types
- Configuration management

## Usage

```rust
use r3e_core::{v8_initialize, v8_finalize, signal_hooks, VERSION};
use r3e_core::config::Config;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

// Initialize V8 engine
v8_initialize();

// Register signal hooks for graceful shutdown
let shutdown_flag = Arc::new(AtomicBool::new(false));
signal_hooks("my_service", shutdown_flag.clone())?;

// Get version information
println!("R3E FaaS version: {}", VERSION);

// Clean up
v8_finalize();
```

## Configuration

```rust
use r3e_core::config::{Config, V8Config};

// Create a configuration with default values
let config = Config::default();

// Create a custom configuration
let custom_config = Config {
    log_config: Some("./config/log.yaml".to_string()),
    v8: V8Config {
        worker_threads: 4,
        background_compilation: true,
    },
};
```
