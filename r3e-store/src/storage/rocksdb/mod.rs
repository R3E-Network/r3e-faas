// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! RocksDB storage implementation

use crate::error::{
    DeleteError, GetError, MultiDeleteError, MultiGetError, MultiPutError, PutError, ScanError,
};
use crate::storage::{BatchKvStore, KvStore, SortedKvStore};
use crate::types::{PutInput, ScanInput, ScanOutput};
use rocksdb::{
    ColumnFamilyDescriptor, DBCompactionStyle, DBCompressionType, Direction, IteratorMode, Options,
    ReadOptions, SliceTransform, DB,
};
use std::path::Path;
use std::sync::Arc;

mod thread_safe;
pub use thread_safe::ThreadSafeRocksDBStore;

/// RocksDB store implementation
pub struct RocksDBStore {
    db: Option<Arc<DB>>,
    path: String,
}

impl RocksDBStore {
    /// Create a new RocksDB store
    pub fn new(path: &str) -> Self {
        Self {
            db: None,
            path: path.to_string(),
        }
    }

    /// Open the database
    pub fn open(&mut self) -> Result<(), PutError> {
        if self.db.is_some() {
            return Ok(());
        }

        let path = Path::new(&self.path);
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        opts.increase_parallelism(4);
        opts.set_write_buffer_size(64 * 1024 * 1024); // 64MB
        opts.set_max_write_buffer_number(4);
        opts.set_min_write_buffer_number_to_merge(1);
        opts.set_compaction_style(DBCompactionStyle::Level);
        opts.set_compression_type(DBCompressionType::Lz4);
        opts.set_bottommost_compression_type(DBCompressionType::Zstd);

        // Configure block cache and bloom filter
        let mut block_opts = rocksdb::BlockBasedOptions::default();
        block_opts.set_block_size(4096);
        block_opts.set_cache_index_and_filter_blocks(true);
        block_opts.set_bloom_filter(10, false);
        let cache = rocksdb::Cache::new_lru_cache(8 * 1024 * 1024);
        if let Ok(cache) = cache {
            block_opts.set_block_cache(&cache);
        }
        opts.set_block_based_table_factory(&block_opts);

        let db = if path.exists() {
            // Check which column families exist
            let existing_cfs = match DB::list_cf(&opts, &self.path) {
                Ok(cfs) => cfs,
                Err(_) => Vec::new(),
            };

            if existing_cfs.is_empty() {
                DB::open(&opts, &self.path).map_err(|e| PutError::Storage(e.to_string()))?
            } else {
                let mut cf_opts = Options::default();
                let cf_descriptors: Vec<ColumnFamilyDescriptor> = existing_cfs
                    .iter()
                    .map(|name| ColumnFamilyDescriptor::new(name.clone(), cf_opts.clone()))
                    .collect();

                DB::open_cf_descriptors(&opts, &self.path, cf_descriptors)
                    .map_err(|e| PutError::Storage(e.to_string()))?
            }
        } else {
            DB::open(&opts, &self.path).map_err(|e| PutError::Storage(e.to_string()))?
        };

        self.db = Some(Arc::new(db));
        Ok(())
    }

    /// Get the database instance
    fn get_db(&self) -> Result<Arc<DB>, PutError> {
        match &self.db {
            Some(db) => Ok(db.clone()),
            None => Err(PutError::Storage("Database not open".to_string())),
        }
    }

    /// Ensure a column family exists
    fn ensure_cf(&self, table: &str) -> Result<(), PutError> {
        let db = self.get_db()?;
        
        // Check if the column family exists
        if db.cf_handle(table).is_none() {
            // Create the column family if it doesn't exist
            let mut opts = Options::default();
            opts.set_prefix_extractor(SliceTransform::create_fixed_prefix(4));
            
            // We can't modify the Arc<DB> directly, so we need to clone it
            // This is a limitation of the RocksDB API
            // In a real implementation, we would need to use a Mutex to protect the DB
            return Err(PutError::Storage(format!("Column family {} not found and cannot be created dynamically", table)));
        }
        
        Ok(())
    }
}

impl KvStore for RocksDBStore {
    fn put(&self, table: &str, input: PutInput) -> Result<(), PutError> {
        let db = self.get_db()?;
        self.ensure_cf(table)?;
        
        let cf = db.cf_handle(table).ok_or_else(|| {
            PutError::Storage(format!("Column family {} not found", table))
        })?;
        
        db.put_cf(&cf, input.key, input.value)
            .map_err(|e| PutError::Storage(e.to_string()))?;
        
        Ok(())
    }

    fn get(&self, table: &str, key: &[u8]) -> Result<Option<Vec<u8>>, GetError> {
        let db = self.get_db().map_err(|e| GetError::Storage(e.to_string()))?;
        
        let cf = match db.cf_handle(table) {
            Some(cf) => cf,
            None => return Ok(None), // Column family doesn't exist
        };
        
        db.get_cf(&cf, key)
            .map_err(|e| GetError::Storage(e.to_string()))
    }

    fn delete(&self, table: &str, key: &[u8]) -> Result<Option<Vec<u8>>, DeleteError> {
        let db = self.get_db().map_err(|e| DeleteError::Storage(e.to_string()))?;
        
        let cf = match db.cf_handle(table) {
            Some(cf) => cf,
            None => return Ok(None), // Column family doesn't exist
        };
        
        // Get the value before deleting
        let value = db.get_cf(&cf, key)
            .map_err(|e| DeleteError::Storage(e.to_string()))?;
        
        // Delete the key
        db.delete_cf(&cf, key)
            .map_err(|e| DeleteError::Storage(e.to_string()))?;
        
        Ok(value)
    }
}

impl SortedKvStore for RocksDBStore {
    fn scan(&self, table: &str, input: ScanInput) -> Result<ScanOutput, ScanError> {
        let db = self.get_db().map_err(|e| ScanError::Storage(e.to_string()))?;
        
        let cf = match db.cf_handle(table) {
            Some(cf) => cf,
            None => return Ok(ScanOutput { 
                keys: Vec::new(),
                values: Vec::new(),
            }), // Column family doesn't exist
        };
        
        let mut read_opts = ReadOptions::default();
        read_opts.set_prefix_same_as_start(true);
        
        // Use prefix if provided
        let mode = if let Some(prefix) = input.prefix {
            IteratorMode::From(prefix, Direction::Forward)
        } else {
            IteratorMode::Start
        };
        
        let iter = db.iterator_cf_opt(&cf, read_opts, mode);
        
        let mut keys = Vec::new();
        let mut values = Vec::new();
        let mut count = 0;
        let limit = input.limit.unwrap_or(100);
        let offset = input.offset.unwrap_or(0);
        
        for result in iter {
            match result {
                Ok((key, value)) => {
                    // Check if we need to skip this item (offset)
                    if count < offset {
                        count += 1;
                        continue;
                    }
                    
                    // Check if we have a prefix and if the key starts with it
                    if let Some(prefix) = input.prefix {
                        if !key.starts_with(prefix) {
                            break;
                        }
                    }
                    
                    // Add to results
                    keys.push(key.to_vec());
                    values.push(value.to_vec());
                    count += 1;
                    
                    // Check limit
                    if count >= offset + limit {
                        break;
                    }
                }
                Err(e) => {
                    return Err(ScanError::Storage(e.to_string()));
                }
            }
        }
        
        Ok(ScanOutput { keys, values })
    }
}

impl BatchKvStore for RocksDBStore {
    fn multi_put(&self, inputs: &[(&str, PutInput)]) -> Result<(), MultiPutError> {
        let db = self.get_db().map_err(|e| MultiPutError::Storage(e.to_string()))?;
        
        let mut batch = rocksdb::WriteBatch::default();
        
        for (table, input) in inputs {
            // Ensure column family exists
            self.ensure_cf(table).map_err(|e| MultiPutError::Storage(e.to_string()))?;
            
            let cf = db.cf_handle(table).ok_or_else(|| {
                MultiPutError::Storage(format!("Column family {} not found", table))
            })?;
            
            batch.put_cf(&cf, &input.key, &input.value);
        }
        
        db.write(batch).map_err(|e| MultiPutError::Storage(e.to_string()))?;
        
        Ok(())
    }

    fn multi_get(&self, inputs: &[(&str, &[u8])]) -> Result<Vec<Option<Vec<u8>>>, MultiGetError> {
        let db = self.get_db().map_err(|e| MultiGetError::Storage(e.to_string()))?;
        
        let mut results = Vec::with_capacity(inputs.len());
        
        for (table, key) in inputs {
            let cf = match db.cf_handle(table) {
                Some(cf) => cf,
                None => {
                    results.push(None);
                    continue;
                }
            };
            
            match db.get_cf(&cf, key) {
                Ok(value) => results.push(value),
                Err(e) => return Err(MultiGetError::Storage(e.to_string())),
            }
        }
        
        Ok(results)
    }

    fn multi_delete(&self, inputs: &[(&str, &[u8])]) -> Result<Vec<Option<Vec<u8>>>, MultiDeleteError> {
        let db = self.get_db().map_err(|e| MultiDeleteError::Storage(e.to_string()))?;
        
        let mut results = Vec::with_capacity(inputs.len());
        let mut batch = rocksdb::WriteBatch::default();
        
        // First get all values
        for (table, key) in inputs {
            let cf = match db.cf_handle(table) {
                Some(cf) => cf,
                None => {
                    results.push(None);
                    continue;
                }
            };
            
            match db.get_cf(&cf, key) {
                Ok(value) => {
                    results.push(value);
                    batch.delete_cf(&cf, key);
                }
                Err(e) => return Err(MultiDeleteError::Storage(e.to_string())),
            }
        }
        
        // Then delete them all in one batch
        db.write(batch).map_err(|e| MultiDeleteError::Storage(e.to_string()))?;
        
        Ok(results)
    }
}
