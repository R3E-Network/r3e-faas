// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! # R3E Store
//! 
//! Storage abstractions for the R3E FaaS platform.

pub mod types;
pub mod error;
pub mod storage;
pub mod config;

// Legacy modules (to be migrated)
pub mod mem;
pub mod rocksdb;

#[cfg(test)]
pub mod mem_test;

// Re-export important types
pub use types::{MAX_TABLE_NAME_SIZE, MAX_KEY_SIZE, MAX_VALUE_SIZE, PutInput, ScanInput, ScanOutput};
pub use error::{PutError, GetError, DeleteError, ScanError, MultiPutError, MultiGetError, MultiDeleteError};
pub use storage::{KvStore, SortedKvStore, BatchKvStore, MemoryStore, RocksDBStore};
