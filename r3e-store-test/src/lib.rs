// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! # R3E Store
//!
//! Storage abstractions for the R3E FaaS platform.

pub mod config;
pub mod error;
pub mod storage;
pub mod types;
pub mod repository;

// Legacy modules (to be migrated)
pub mod mem;
pub mod rocksdb;

#[cfg(test)]
pub mod mem_test;

// Re-export important types
pub use error::{
    DeleteError, GetError, MultiDeleteError, MultiGetError, MultiPutError, PutError, ScanError,
};
pub use storage::{BatchKvStore, KvStore, MemoryStore, RocksDBStore, SortedKvStore};
pub use types::{
    PutInput, ScanInput, ScanOutput, MAX_KEY_SIZE, MAX_TABLE_NAME_SIZE, MAX_VALUE_SIZE,
};

// Re-export repository types
pub use repository::user::{User, UserRepository, CF_USERS};
pub use repository::service::{
    Service, ServiceRepository, CF_SERVICES, ServiceType, BlockchainType,
};
