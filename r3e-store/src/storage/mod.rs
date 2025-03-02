// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Storage trait definitions for the store crate.

use crate::error::{
    DeleteError, GetError, MultiDeleteError, MultiGetError, MultiPutError, PutError, ScanError,
};
use crate::types::{PutInput, ScanInput, ScanOutput};

/// Key-value store trait
pub trait KvStore {
    /// Put a key-value pair
    fn put(&self, table: &str, input: PutInput) -> Result<(), PutError>;

    /// Get a value by key
    fn get(&self, table: &str, key: &[u8]) -> Result<Vec<u8>, GetError>;

    /// Delete a key-value pair
    fn delete(&self, table: &str, key: &[u8]) -> Result<Option<Vec<u8>>, DeleteError>;
}

/// Sorted key-value store trait
pub trait SortedKvStore: KvStore {
    /// Scan key-value pairs
    fn scan(&self, table: &str, input: ScanInput) -> Result<ScanOutput, ScanError>;
}

/// Batch key-value store trait
pub trait BatchKvStore: KvStore {
    /// Put multiple key-value pairs
    fn multi_put(&self, inputs: &[(&str, PutInput)]) -> Result<(), MultiPutError>;

    /// Get multiple values by keys
    fn multi_get(&self, inputs: &[(&str, &[u8])]) -> Result<Vec<Option<Vec<u8>>>, MultiGetError>;

    /// Delete multiple key-value pairs
    fn multi_delete(
        &self,
        inputs: &[(&str, &[u8])],
    ) -> Result<Vec<Option<Vec<u8>>>, MultiDeleteError>;
}

pub mod memory;

// Re-export RocksDBStore
pub use crate::RocksDBStore;
