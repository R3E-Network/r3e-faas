// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Storage traits for the R3E Store

use crate::error::{StoreError, StoreResult};
use crate::types::{PutInput, ScanInput, ScanOutput};

/// Key-value store trait
pub trait KvStore {
    /// Put a key-value pair into the store
    fn put(&self, table: &str, input: PutInput) -> StoreResult<()>;

    /// Get a value from the store
    fn get(&self, table: &str, key: &[u8]) -> StoreResult<Vec<u8>>;

    /// Delete a key-value pair from the store
    fn delete(&self, table: &str, key: &[u8]) -> StoreResult<Option<Vec<u8>>>;
}

/// Sorted key-value store trait (supporting range queries)
pub trait SortedKvStore: KvStore {
    /// Scan key-value pairs in a range
    fn scan(&self, table: &str, input: ScanInput) -> StoreResult<ScanOutput>;
}

/// Batch key-value store trait
pub trait BatchKvStore: KvStore {
    /// Put multiple key-value pairs into the store
    fn multi_put(&self, inputs: &[(&str, PutInput)]) -> StoreResult<()>;

    /// Get multiple values from the store
    fn multi_get(&self, inputs: &[(&str, &[u8])]) -> StoreResult<Vec<Option<Vec<u8>>>>;

    /// Delete multiple key-value pairs from the store
    fn multi_delete(&self, inputs: &[(&str, &[u8])]) -> StoreResult<Vec<Option<Vec<u8>>>>;
} 