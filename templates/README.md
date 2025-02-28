# Crate Name

Brief description of the crate.

## Features

- Feature 1
- Feature 2
- Feature 3

## Usage

```rust
use crate_name::Service;
use crate_name::storage::MemoryStorage;

// Create a new service with memory storage
let storage = MemoryStorage::new();
let service = Service::new(Arc::new(storage));

// Create a new MainType
let main_type = service.create("Name".to_string(), Some("Description".to_string())).await?;

// Get a MainType by ID
let main_type = service.get(&main_type.id).await?;

// Update a MainType
let main_type = service.update(&main_type.id, Some("New Name".to_string()), None).await?;

// Delete a MainType
service.delete(&main_type.id).await?;

// List all MainTypes
let main_types = service.list().await?;
```

## Configuration

```rust
use crate_name::config::{Config, StorageType};

// Create a configuration for memory storage
let memory_config = Config {
    storage_type: StorageType::Memory,
    rocksdb_path: None,
    rocksdb_cf_name: None,
};

// Create a configuration for RocksDB storage
let rocksdb_config = Config {
    storage_type: StorageType::RocksDB,
    rocksdb_path: Some("/path/to/rocksdb".to_string()),
    rocksdb_cf_name: Some("main_types".to_string()),
};
```
