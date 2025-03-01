// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use rocksdb::{ColumnFamilyDescriptor, IteratorMode, Options, DB};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum RocksDBError {
    #[error("RocksDB error: {0}")]
    DB(#[from] rocksdb::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Invalid table name")]
    InvalidTable,

    #[error("Key is too large")]
    TooLargeKey,

    #[error("Value is too large")]
    TooLargeValue,

    #[error("Key already exists")]
    AlreadyExists,

    #[error("No such key")]
    NoSuchKey,
}

pub struct RocksDBStore {
    db: DB,
    tables: Arc<Mutex<HashMap<String, bool>>>,
}

impl RocksDBStore {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, RocksDBError> {
        let mut options = Options::default();
        options.create_if_missing(true);
        options.create_missing_column_families(true);

        // Get existing column families
        let cf_names = match DB::list_cf(&options, &path) {
            Ok(names) => names,
            Err(_) => vec![], // If DB doesn't exist yet, start with empty list
        };

        // Create column family descriptors
        let mut cf_descriptors = Vec::new();
        for name in &cf_names {
            let cf_options = Options::default();
            cf_descriptors.push(ColumnFamilyDescriptor::new(name, cf_options));
        }

        // Open the database with column families
        let db = if cf_descriptors.is_empty() {
            DB::open(&options, path)?
        } else {
            DB::open_cf_descriptors(&options, path, cf_descriptors)?
        };

        // Build table map
        let mut tables = HashMap::new();
        for name in cf_names {
            tables.insert(name, true);
        }

        Ok(Self {
            db,
            tables: Arc::new(Mutex::new(tables)),
        })
    }

    fn ensure_table_exists(&self, table: &str) -> Result<(), RocksDBError> {
        if table.len() > MAX_TABLE_NAME_SIZE {
            return Err(RocksDBError::InvalidTable);
        }

        let mut tables = self.tables.lock().unwrap();
        if !tables.contains_key(table) {
            // Create the column family if it doesn't exist
            let cf_options = Options::default();
            self.db.create_cf(table, &cf_options)?;
            tables.insert(table.to_string(), true);
        }

        Ok(())
    }
}

impl KvStore for RocksDBStore {
    fn put(&self, table: &str, input: PutInput) -> Result<(), PutError> {
        if table.len() > MAX_TABLE_NAME_SIZE {
            return Err(PutError::InvalidTable);
        }

        if input.key.len() > MAX_KEY_SIZE {
            return Err(PutError::TooLargeKey);
        }

        if input.value.len() > MAX_VALUE_SIZE {
            return Err(PutError::TooLargeValue);
        }

        // Ensure table exists
        self.ensure_table_exists(table)
            .map_err(|_| PutError::InvalidTable)?;

        // Get column family handle
        let cf_handle = self.db.cf_handle(table).ok_or(PutError::InvalidTable)?;

        // Check if key exists when if_not_exists is true
        if input.if_not_exists {
            if let Ok(Some(_)) = self.db.get_cf(&cf_handle, input.key) {
                return Err(PutError::AlreadyExists);
            }
        }

        // Put the key-value pair
        self.db
            .put_cf(&cf_handle, input.key, input.value)
            .map_err(|_| PutError::InvalidTable)?;

        Ok(())
    }

    fn get(&self, table: &str, key: &[u8]) -> Result<Vec<u8>, GetError> {
        if table.len() > MAX_TABLE_NAME_SIZE {
            return Err(GetError::InvalidTable);
        }

        if key.len() > MAX_KEY_SIZE {
            return Err(GetError::TooLargeKey);
        }

        // Ensure table exists
        self.ensure_table_exists(table)
            .map_err(|_| GetError::InvalidTable)?;

        // Get column family handle
        let cf_handle = self.db.cf_handle(table).ok_or(GetError::InvalidTable)?;

        // Get the value
        match self
            .db
            .get_cf(&cf_handle, key)
            .map_err(|_| GetError::InvalidTable)?
        {
            Some(value) => Ok(value),
            None => Err(GetError::NoSuchKey),
        }
    }

    fn delete(&self, table: &str, key: &[u8]) -> Result<Option<Vec<u8>>, DeleteError> {
        if table.len() > MAX_TABLE_NAME_SIZE {
            return Err(DeleteError::InvalidTable);
        }

        if key.len() > MAX_KEY_SIZE {
            return Err(DeleteError::TooLargeKey);
        }

        // Ensure table exists
        self.ensure_table_exists(table)
            .map_err(|_| DeleteError::InvalidTable)?;

        // Get column family handle
        let cf_handle = self.db.cf_handle(table).ok_or(DeleteError::InvalidTable)?;

        // Get the value before deleting
        let value = match self
            .db
            .get_cf(&cf_handle, key)
            .map_err(|_| DeleteError::InvalidTable)?
        {
            Some(value) => Some(value),
            None => None,
        };

        // Delete the key
        if value.is_some() {
            self.db
                .delete_cf(&cf_handle, key)
                .map_err(|_| DeleteError::InvalidTable)?;
        }

        Ok(value)
    }
}

impl SortedKvStore for RocksDBStore {
    fn scan(&self, table: &str, input: ScanInput) -> Result<ScanOutput, ScanError> {
        if table.len() > MAX_TABLE_NAME_SIZE {
            return Err(ScanError::InvalidTable);
        }

        if input.start_key.len() > MAX_KEY_SIZE || input.end_key.len() > MAX_KEY_SIZE {
            return Err(ScanError::TooLargeKey);
        }

        // Ensure table exists
        self.ensure_table_exists(table)
            .map_err(|_| ScanError::InvalidTable)?;

        // Get column family handle
        let cf_handle = self.db.cf_handle(table).ok_or(ScanError::InvalidTable)?;

        // Create iterator
        let mut iter = self.db.iterator_cf(&cf_handle, IteratorMode::Start);

        // Skip to start key if provided
        if !input.start_key.is_empty() {
            iter = self.db.iterator_cf(
                &cf_handle,
                IteratorMode::From(input.start_key, rocksdb::Direction::Forward),
            );

            // Skip the first element if start is exclusive
            if input.start_exclusive {
                if let Some(Ok(_)) = iter.next() {
                    // Skip the start key
                }
            }
        }

        // Collect results
        let max_count = input.max_count();
        let mut kvs = Vec::with_capacity(max_count);
        let mut has_more = false;

        for item in iter {
            if kvs.len() >= max_count {
                has_more = true;
                break;
            }

            let (key, value) = item.map_err(|_| ScanError::InvalidTable)?;

            // Check end key if provided
            if !input.end_key.is_empty() {
                let cmp = key.as_ref().cmp(input.end_key);

                if cmp > 0 || (cmp == 0 && !input.end_inclusive) {
                    break;
                }
            }

            kvs.push((key.to_vec(), value.to_vec()));
        }

        Ok(ScanOutput { kvs, has_more })
    }
}

impl BatchKvStore for RocksDBStore {
    fn multi_put(&self, inputs: &[(&str, PutInput)]) -> Result<(), MultiPutError> {
        // Validate inputs
        for (table, input) in inputs {
            if table.len() > MAX_TABLE_NAME_SIZE {
                return Err(MultiPutError::InvalidTable);
            }

            if input.key.len() > MAX_KEY_SIZE {
                return Err(MultiPutError::TooLargeKey);
            }

            if input.value.len() > MAX_VALUE_SIZE {
                return Err(MultiPutError::TooLargeValue);
            }

            // Ensure table exists
            self.ensure_table_exists(table)
                .map_err(|_| MultiPutError::InvalidTable)?;
        }

        // Create a write batch
        let mut batch = rocksdb::WriteBatch::default();

        // Add operations to the batch
        for (table, input) in inputs {
            let cf_handle = self
                .db
                .cf_handle(table)
                .ok_or(MultiPutError::InvalidTable)?;

            // Check if key exists when if_not_exists is true
            if input.if_not_exists {
                if let Ok(Some(_)) = self.db.get_cf(&cf_handle, input.key) {
                    continue; // Skip this key if it already exists
                }
            }

            batch.put_cf(&cf_handle, input.key, input.value);
        }

        // Write the batch
        self.db
            .write(batch)
            .map_err(|_| MultiPutError::InvalidTable)?;

        Ok(())
    }

    fn multi_get(&self, inputs: &[(&str, &[u8])]) -> Result<Vec<Option<Vec<u8>>>, MultiGetError> {
        // Validate inputs
        for (table, key) in inputs {
            if table.len() > MAX_TABLE_NAME_SIZE {
                return Err(MultiGetError::InvalidTable);
            }

            if key.len() > MAX_KEY_SIZE {
                return Err(MultiGetError::TooLargeKey);
            }

            // Ensure table exists
            self.ensure_table_exists(table)
                .map_err(|_| MultiGetError::InvalidTable)?;
        }

        // Get values
        let mut results = Vec::with_capacity(inputs.len());

        for (table, key) in inputs {
            let cf_handle = self
                .db
                .cf_handle(table)
                .ok_or(MultiGetError::InvalidTable)?;

            match self
                .db
                .get_cf(&cf_handle, *key)
                .map_err(|_| MultiGetError::InvalidTable)?
            {
                Some(value) => results.push(Some(value)),
                None => results.push(None),
            }
        }

        Ok(results)
    }

    fn multi_delete(
        &self,
        inputs: &[(&str, &[u8])],
    ) -> Result<Vec<Option<Vec<u8>>>, MultiDeleteError> {
        // Validate inputs
        for (table, key) in inputs {
            if table.len() > MAX_TABLE_NAME_SIZE {
                return Err(MultiDeleteError::InvalidTable);
            }

            if key.len() > MAX_KEY_SIZE {
                return Err(MultiDeleteError::TooLargeKey);
            }

            // Ensure table exists
            self.ensure_table_exists(table)
                .map_err(|_| MultiDeleteError::InvalidTable)?;
        }

        // Get values and delete
        let mut results = Vec::with_capacity(inputs.len());
        let mut batch = rocksdb::WriteBatch::default();

        for (table, key) in inputs {
            let cf_handle = self
                .db
                .cf_handle(table)
                .ok_or(MultiDeleteError::InvalidTable)?;

            // Get the value before deleting
            let value = match self
                .db
                .get_cf(&cf_handle, *key)
                .map_err(|_| MultiDeleteError::InvalidTable)?
            {
                Some(value) => {
                    batch.delete_cf(&cf_handle, *key);
                    Some(value)
                }
                None => None,
            };

            results.push(value);
        }

        // Write the batch
        self.db
            .write(batch)
            .map_err(|_| MultiDeleteError::InvalidTable)?;

        Ok(results)
    }
}
