# R3E Config

Configuration management for the R3E FaaS platform.

## Features

- Centralized configuration management
- Support for multiple configuration sources (files, environment variables)
- Default configuration values
- Configuration provider for easy access
- Type-safe configuration

## Usage

```rust
use r3e_config::{ConfigLoader, ConfigProvider, FaasConfig};
use r3e_config::loader::ConfigFormat;

// Load configuration from file
let config = ConfigLoader::load_from_file("config.yaml").unwrap();

// Load configuration from environment variables
let config = ConfigLoader::load_from_env().unwrap();

// Load configuration from multiple sources with precedence
let config = ConfigLoader::load(Some("config.yaml")).unwrap();

// Create a configuration provider
let provider = ConfigProvider::new(config);

// Get the configuration
let config = provider.get_config().await;

// Update the configuration
provider.update_config(new_config).await;

// Get a specific configuration value
let max_memory = provider.get(|config| config.runtime.js.max_memory_mb).await;

// Update a specific configuration value
provider.update(|config| {
    config.runtime.js.max_memory_mb = 256;
    Ok(())
}).await.unwrap();

// Save configuration to file
ConfigLoader::save_to_file(&config, "config.yaml", ConfigFormat::Yaml).unwrap();
```

## Environment Variables

Environment variables are prefixed with `R3E_FAAS` and use double underscores (`__`) as separators:

```
R3E_FAAS__GENERAL__ENVIRONMENT=production
R3E_FAAS__STORAGE__STORAGE_TYPE=rocksdb
R3E_FAAS__RUNTIME__JS__MAX_MEMORY_MB=256
R3E_FAAS__API__PORT=8080
```

## Configuration File

Configuration can be loaded from YAML or JSON files:

```yaml
general:
  environment: production
  instance_id: instance-1
  data_dir: /var/lib/r3e-faas

storage:
  storage_type: rocksdb
  rocksdb_path: /var/lib/r3e-faas/db

runtime:
  js:
    max_memory_mb: 256
    max_execution_time_ms: 10000
    enable_jit: false
  sandbox:
    enable_network: true
    enable_filesystem: false
    enable_environment: false
    allowed_domains:
      - api.example.com

services:
  oracle:
    enabled: true
    default_timeout_ms: 5000
    rate_limit: 100

api:
  host: 0.0.0.0
  port: 8080
  enable_cors: true
  cors_allowed_origins:
    - "*"
  enable_auth: true
  jwt_secret: your-secret-key

logging:
  level: info
  format: json
  file: /var/log/r3e-faas.log
```
