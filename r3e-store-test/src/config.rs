// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Configuration for the store crate.

use serde::{Deserialize, Serialize};

/// Configuration for the store crate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Storage type
    pub storage_type: StorageType,

    /// RocksDB path (if using RocksDB storage)
    pub rocksdb_path: Option<String>,

    /// Memory store capacity (if using Memory storage)
    pub memory_capacity: Option<usize>,
}

/// Storage type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageType {
    /// Memory storage
    Memory,

    /// RocksDB storage
    RocksDB,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            storage_type: StorageType::Memory,
            rocksdb_path: None,
            memory_capacity: None,
        }
    }
}
