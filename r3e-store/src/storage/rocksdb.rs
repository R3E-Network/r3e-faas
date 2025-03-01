// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use crate::error::{
    DeleteError, GetError, MultiDeleteError, MultiGetError, MultiPutError, PutError, ScanError,
};
use crate::storage::{BatchKvStore, KvStore, SortedKvStore};
use crate::types::{PutInput, ScanInput, ScanOutput};
use rocksdb::{ColumnFamilyDescriptor, DBCompactionStyle, DBCompressionType, Options, DB};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

/// RocksDB configuration
#[derive(Debug, Clone)]
pub struct RocksDBConfig {
    /// Path to the database
    pub path: String,
    /// Column families
    pub column_families: Vec<String>,
}

/// RocksDB implementation of the KvStore trait
pub struct RocksDBStore {
    /// Database instance
    db: Arc<DB>,
    /// Column families
    column_families: Arc<Mutex<HashMap<String, String>>>,
}

impl RocksDBStore {
    /// Create a new RocksDB store
    pub fn new(config: RocksDBConfig) -> Result<Self, String> {
        // Create options
        let mut options = Options::default();
        options.create_if_missing(true);
        options.create_missing_column_families(true);
        options.set_compaction_style(DBCompactionStyle::Level);
        options.set_compression_type(DBCompressionType::Lz4);
        options.set_max_open_files(1000);
        options.set_keep_log_file_num(10);
        options.set_max_total_wal_size(1024 * 1024 * 1024); // 1GB
        options.set_write_buffer_size(64 * 1024 * 1024); // 64MB
        options.set_target_file_size_base(64 * 1024 * 1024); // 64MB
        options.set_max_write_buffer_number(3);
        options.set_max_background_jobs(4);
        options.set_level_zero_file_num_compaction_trigger(4);
        options.set_level_zero_slowdown_writes_trigger(20);
        options.set_level_zero_stop_writes_trigger(36);

        // Create column family descriptors
        let mut cf_descriptors = Vec::new();
        for cf_name in &config.column_families {
            let mut cf_options = Options::default();
            cf_options.set_compaction_style(DBCompactionStyle::Level);
            cf_options.set_compression_type(DBCompressionType::Lz4);
            cf_options.set_write_buffer_size(64 * 1024 * 1024); // 64MB
            cf_options.set_target_file_size_base(64 * 1024 * 1024); // 64MB
            cf_options.set_max_write_buffer_number(3);
            cf_options.set_level_zero_file_num_compaction_trigger(4);
            cf_options.set_level_zero_slowdown_writes_trigger(20);
            cf_options.set_level_zero_stop_writes_trigger(36);

            cf_descriptors.push(ColumnFamilyDescriptor::new(cf_name, cf_options));
        }

        // Open the database
        let db = match DB::open_cf_descriptors(&options, &config.path, cf_descriptors) {
            Ok(db) => db,
            Err(e) => return Err(format!("Failed to open RocksDB: {}", e)),
        };

        // Create column families map
        let mut column_families = HashMap::new();
        for cf_name in &config.column_families {
            column_families.insert(cf_name.clone(), cf_name.clone());
        }

        Ok(Self {
            db: Arc::new(db),
            column_families: Arc::new(Mutex::new(column_families)),
        })
    }

    /// Get or create a column family
    fn get_or_create_cf(&self, table: &str) -> Result<String, String> {
        let mut column_families = match self.column_families.lock() {
            Ok(cf) => cf,
            Err(e) => return Err(format!("Failed to lock column families: {}", e)),
        };

        if !column_families.contains_key(table) {
            // Create the column family
            match self.db.create_cf(table, &Options::default()) {
                Ok(_) => {
                    column_families.insert(table.to_string(), table.to_string());
                }
                Err(e) => return Err(format!("Failed to create column family: {}", e)),
            }
        }

        Ok(table.to_string())
    }
}

impl KvStore for RocksDBStore {
    fn put(&self, table: &str, input: PutInput) -> Result<(), PutError> {
        // Get or create the column family
        let cf_name = match self.get_or_create_cf(table) {
            Ok(cf) => cf,
            Err(e) => return Err(PutError::Storage(e)),
        };

        // Get the column family handle
        let cf_handle = match self.db.cf_handle(&cf_name) {
            Some(cf) => cf,
            None => return Err(PutError::Storage(format!("Column family not found: {}", cf_name))),
        };

        // Put the key-value pair
        match self.db.put_cf(cf_handle, &input.key, &input.value) {
            Ok(_) => Ok(()),
            Err(e) => Err(PutError::Storage(e.to_string())),
        }
    }

    fn get(&self, table: &str, key: &[u8]) -> Result<Vec<u8>, GetError> {
        // Get or create the column family
        let cf_name = match self.get_or_create_cf(table) {
            Ok(cf) => cf,
            Err(e) => return Err(GetError::Storage(e)),
        };

        // Get the column family handle
        let cf_handle = match self.db.cf_handle(&cf_name) {
            Some(cf) => cf,
            None => return Err(GetError::Storage(format!("Column family not found: {}", cf_name))),
        };

        // Get the value
        match self.db.get_cf(cf_handle, key) {
            Ok(Some(value)) => Ok(value),
            Ok(None) => Err(GetError::NotFound),
            Err(e) => Err(GetError::Storage(e.to_string())),
        }
    }

    fn delete(&self, table: &str, key: &[u8]) -> Result<Option<Vec<u8>>, DeleteError> {
        // Get or create the column family
        let cf_name = match self.get_or_create_cf(table) {
            Ok(cf) => cf,
            Err(e) => return Err(DeleteError::Storage(e)),
        };

        // Get the column family handle
        let cf_handle = match self.db.cf_handle(&cf_name) {
            Some(cf) => cf,
            None => {
                return Err(DeleteError::Storage(format!(
                    "Column family not found: {}",
                    cf_name
                )))
            }
        };

        // Get the value first
        let value = match self.db.get_cf(cf_handle, key) {
            Ok(value) => value,
            Err(e) => return Err(DeleteError::Storage(e.to_string())),
        };

        // Delete the key-value pair
        match self.db.delete_cf(cf_handle, key) {
            Ok(_) => Ok(value),
            Err(e) => Err(DeleteError::Storage(e.to_string())),
        }
    }
}

impl SortedKvStore for RocksDBStore {
    fn scan(&self, table: &str, input: ScanInput) -> Result<ScanOutput, ScanError> {
        // Get or create the column family
        let cf_name = match self.get_or_create_cf(table) {
            Ok(cf) => cf,
            Err(e) => return Err(ScanError::Storage(e)),
        };

        // Get the column family handle
        let cf_handle = match self.db.cf_handle(&cf_name) {
            Some(cf) => cf,
            None => {
                return Err(ScanError::Storage(format!(
                    "Column family not found: {}",
                    cf_name
                )))
            }
        };

        // Create the iterator
        let prefix = input.prefix.unwrap_or_default();
        let iter = self.db.prefix_iterator_cf(cf_handle, &prefix);

        // Collect the results
        let mut keys = Vec::new();
        let mut values = Vec::new();
        let limit = input.limit.unwrap_or(100);
        let offset = input.offset.unwrap_or(0);

        for (i, (key, value)) in iter.enumerate() {
            if i < offset {
                continue;
            }
            if keys.len() >= limit {
                break;
            }
            keys.push(key.to_vec());
            values.push(value.to_vec());
        }

        Ok(ScanOutput { keys, values })
    }
}

impl BatchKvStore for RocksDBStore {
    fn multi_put(&self, inputs: &[(&str, PutInput)]) -> Result<(), MultiPutError> {
        // Create a write batch
        let mut batch = rocksdb::WriteBatch::default();

        // Add all operations to the batch
        for (table, input) in inputs {
            // Get or create the column family
            let cf_name = match self.get_or_create_cf(table) {
                Ok(cf) => cf,
                Err(e) => return Err(MultiPutError::Storage(e)),
            };

            // Get the column family handle
            let cf_handle = match self.db.cf_handle(&cf_name) {
                Some(cf) => cf,
                None => {
                    return Err(MultiPutError::Storage(format!(
                        "Column family not found: {}",
                        cf_name
                    )))
                }
            };

            // Add the operation to the batch
            batch.put_cf(cf_handle, &input.key, &input.value);
        }

        // Write the batch
        match self.db.write(batch) {
            Ok(_) => Ok(()),
            Err(e) => Err(MultiPutError::Storage(e.to_string())),
        }
    }

    fn multi_get(&self, inputs: &[(&str, &[u8])]) -> Result<Vec<Option<Vec<u8>>>, MultiGetError> {
        let mut results = Vec::with_capacity(inputs.len());

        for (table, key) in inputs {
            // Get or create the column family
            let cf_name = match self.get_or_create_cf(table) {
                Ok(cf) => cf,
                Err(e) => return Err(MultiGetError::Storage(e)),
            };

            // Get the column family handle
            let cf_handle = match self.db.cf_handle(&cf_name) {
                Some(cf) => cf,
                None => {
                    return Err(MultiGetError::Storage(format!(
                        "Column family not found: {}",
                        cf_name
                    )))
                }
            };

            // Get the value
            match self.db.get_cf(cf_handle, key) {
                Ok(value) => results.push(value),
                Err(e) => return Err(MultiGetError::Storage(e.to_string())),
            }
        }

        Ok(results)
    }

    fn multi_delete(
        &self,
        inputs: &[(&str, &[u8])],
    ) -> Result<Vec<Option<Vec<u8>>>, MultiDeleteError> {
        let mut results = Vec::with_capacity(inputs.len());
        let mut batch = rocksdb::WriteBatch::default();

        for (table, key) in inputs {
            // Get or create the column family
            let cf_name = match self.get_or_create_cf(table) {
                Ok(cf) => cf,
                Err(e) => return Err(MultiDeleteError::Storage(e)),
            };

            // Get the column family handle
            let cf_handle = match self.db.cf_handle(&cf_name) {
                Some(cf) => cf,
                None => {
                    return Err(MultiDeleteError::Storage(format!(
                        "Column family not found: {}",
                        cf_name
                    )))
                }
            };

            // Get the value first
            let value = match self.db.get_cf(cf_handle, key) {
                Ok(value) => {
                    results.push(value);
                    Ok(())
                }
                Err(e) => Err(MultiDeleteError::Storage(e.to_string())),
            };

            // Check if we had an error getting the value
            if value.is_err() {
                return value;
            }

            // Add the delete operation to the batch
            batch.delete_cf(cf_handle, key);
        }

        // Write the batch
        match self.db.write(batch) {
            Ok(_) => Ok(results),
            Err(e) => Err(MultiDeleteError::Storage(e.to_string())),
        }
    }
}
