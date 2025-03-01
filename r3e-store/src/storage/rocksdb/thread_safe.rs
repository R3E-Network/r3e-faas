// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Thread-safe RocksDB wrapper implementation

use crate::error::{
    DeleteError, GetError, MultiDeleteError, MultiGetError, MultiPutError, PutError, ScanError,
};
use crate::storage::{BatchKvStore, KvStore, SortedKvStore};
use crate::types::{PutInput, ScanInput, ScanOutput};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Thread-safe RocksDB store implementation
pub struct ThreadSafeRocksDBStore {
    inner: Arc<Mutex<super::RocksDBStore>>,
}

impl ThreadSafeRocksDBStore {
    /// Create a new thread-safe RocksDB store
    pub fn new(path: &str) -> Self {
        let inner = super::RocksDBStore::new(path);
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }
}

impl KvStore for ThreadSafeRocksDBStore {
    fn put(&self, table: &str, input: PutInput) -> Result<(), PutError> {
        // Execute synchronously to avoid thread-safety issues
        let inner = self.inner.blocking_lock();
        inner.put(table, input)
    }

    fn get(&self, table: &str, key: &[u8]) -> Result<Option<Vec<u8>>, GetError> {
        // Execute synchronously to avoid thread-safety issues
        let inner = self.inner.blocking_lock();
        inner.get(table, key)
    }

    fn delete(&self, table: &str, key: &[u8]) -> Result<Option<Vec<u8>>, DeleteError> {
        // Execute synchronously to avoid thread-safety issues
        let inner = self.inner.blocking_lock();
        inner.delete(table, key)
    }
}

impl SortedKvStore for ThreadSafeRocksDBStore {
    fn scan(&self, table: &str, input: ScanInput) -> Result<ScanOutput, ScanError> {
        // Execute synchronously to avoid thread-safety issues
        let inner = self.inner.blocking_lock();
        inner.scan(table, input)
    }
}

impl BatchKvStore for ThreadSafeRocksDBStore {
    fn multi_put(&self, inputs: &[(&str, PutInput)]) -> Result<(), MultiPutError> {
        // Execute synchronously to avoid thread-safety issues
        let inner = self.inner.blocking_lock();
        inner.multi_put(inputs)
    }

    fn multi_get(&self, inputs: &[(&str, &[u8])]) -> Result<Vec<Option<Vec<u8>>>, MultiGetError> {
        // Execute synchronously to avoid thread-safety issues
        let inner = self.inner.blocking_lock();
        inner.multi_get(inputs)
    }

    fn multi_delete(&self, inputs: &[(&str, &[u8])]) -> Result<Vec<Option<Vec<u8>>>, MultiDeleteError> {
        // Execute synchronously to avoid thread-safety issues
        let inner = self.inner.blocking_lock();
        inner.multi_delete(inputs)
    }
}
