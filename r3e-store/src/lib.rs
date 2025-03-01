// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Storage module for R3E FaaS
//!
//! This module provides storage functionality for the R3E FaaS platform.

pub mod error;
pub mod repository;
pub mod rocksdb;
pub mod storage;
pub mod types;

// Re-export types
pub use error::{
    DeleteError, GetError, MultiDeleteError, MultiGetError, MultiPutError, PutError, ScanError,
};
pub use storage::{BatchKvStore, KvStore, MemoryStore, RocksDBStore, SortedKvStore};
pub use types::{
    PutInput, ScanInput, ScanOutput, MAX_KEY_SIZE, MAX_TABLE_NAME_SIZE, MAX_VALUE_SIZE,
};
