# R3E Store

Storage abstractions for the R3E FaaS platform.

## Features

- Key-value store interface
- Memory-based storage implementation
- RocksDB-based storage implementation
- Sorted key-value store for range queries
- Batch operations for improved performance

## Usage

```rust
use r3e_store::{KvStore, SortedKvStore, PutInput};
use r3e_store::storage::MemoryStore;

// Create a memory-based store
let store = MemoryStore::new();

// Put a key-value pair
let input = PutInput {
    key: b"key",
    value: b"value",
    if_not_exists: false,
};
store.put("table", input)?;

// Get a value by key
let value = store.get("table", b"key")?;
assert_eq!(value, b"value");

// Delete a key-value pair
store.delete("table", b"key")?;
```

## RocksDB Usage

```rust
use r3e_store::{KvStore, SortedKvStore, PutInput, ScanInput};
use r3e_store::storage::RocksDBStore;

// Create a RocksDB-based store
let store = RocksDBStore::new("/path/to/rocksdb")?;

// Put a key-value pair
let input = PutInput {
    key: b"key",
    value: b"value",
    if_not_exists: false,
};
store.put("table", input)?;

// Scan key-value pairs
let scan_input = ScanInput {
    start_key: b"",
    start_exclusive: false,
    end_key: b"",
    end_inclusive: false,
    max_count: 100,
};
let output = store.scan("table", scan_input)?;
for (key, value) in output.kvs {
    println!("Key: {:?}, Value: {:?}", key, value);
}
```

## Configuration

```rust
use r3e_store::config::{Config, StorageType};

// Create a configuration for memory storage
let memory_config = Config {
    storage_type: StorageType::Memory,
    rocksdb_path: None,
    memory_capacity: Some(1000),
};

// Create a configuration for RocksDB storage
let rocksdb_config = Config {
    storage_type: StorageType::RocksDB,
    rocksdb_path: Some("/path/to/rocksdb".to_string()),
    memory_capacity: None,
};
```
