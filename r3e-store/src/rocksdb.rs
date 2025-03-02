// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use bincode::{deserialize, serialize};
use log::error;
use rocksdb::{
    ColumnFamilyDescriptor, Direction, IteratorMode, Options, ReadOptions,
    WriteBatch, DB,
};
use serde::{de::DeserializeOwned, Serialize, Deserialize};
use std::{
    collections::HashMap,
    fmt::Debug,
    path::Path,
    sync::{Arc, Mutex},
};
use async_trait::async_trait;
use thiserror::Error;

/// Database result type
pub type DbResult<T> = std::result::Result<T, DbError>;

/// Thread-safe iterator wrapper that collects results
pub struct ThreadSafeIterator<T> {
    items: Vec<T>,
}

impl<T> ThreadSafeIterator<T> {
    /// Create a new thread-safe iterator
    fn new<I>(iter: I) -> Self 
    where 
        I: Iterator<Item = T>,
    {
        Self {
            items: iter.collect(),
        }
    }
}

impl<T> Iterator for ThreadSafeIterator<T> 
{
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }
}

/// Database error type
#[derive(Debug, Error)]
pub enum DbError {
    /// RocksDB error
    #[error("RocksDB error: {0}")]
    RocksDb(#[from] rocksdb::Error),
    
    /// IO error
    #[error("IO error: {0}")]
    IO(String),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    /// Deserialization error
    #[error("Deserialization error: {0}")]
    Deserialization(String),
    
    /// Column family does not exist
    #[error("Column family not found: {0}")]
    ColumnFamilyNotFound(String),
    
    /// Default column family required
    #[error("Default column family required")]
    DefaultCfRequired,
    
    /// Database not open
    #[error("Database not open")]
    NotOpen,
    
    /// Invalid path
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    
    /// Database already open
    #[error("Database already open")]
    AlreadyOpen,
    
    /// Tokio error
    #[error("Tokio error: {0}")]
    Tokio(String),
    
    /// Task join error
    #[error("Task join error: {0}")]
    TaskJoin(String),
    
    /// UTF-8 error
    #[error("UTF-8 error: {0}")]
    Utf8Error(String),
    
    /// Other error
    #[error("Other error: {0}")]
    Other(String),
}

/// Convert bincode errors to DbError
impl From<Box<bincode::ErrorKind>> for DbError {
    fn from(error: Box<bincode::ErrorKind>) -> Self {
        DbError::Deserialization(error.to_string())
    }
}

/// Prefix extractor type
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PrefixExtractor {
    /// Fixed prefix extractor
    Fixed(usize),
    /// Custom prefix extractor
    Custom(String, String),
}

/// Configuration for a column family
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColumnFamilyConfig {
    /// Name of the column family
    pub name: String,

    /// Optional prefix extractor
    pub prefix_extractor: Option<PrefixExtractor>,

    /// Block size in bytes (default: 4096)
    #[serde(default = "default_block_size")]
    pub block_size: usize,

    /// Block cache size in bytes (default: 8MB)
    #[serde(default = "default_block_cache_size")]
    pub block_cache_size: usize,

    /// Bloom filter bits per key (default: 10)
    #[serde(default = "default_bloom_filter_bits")]
    pub bloom_filter_bits: i32,

    /// Whether to cache index and filter blocks (default: true)
    #[serde(default = "default_true")]
    pub cache_index_and_filter_blocks: bool,

    /// Compression type (default: Lz4)
    #[serde(default = "default_compression_type")]
    pub compression_type: Compression,

    /// Options for the column family
    pub options: HashMap<String, String>,
}

fn default_block_size() -> usize {
    4096
}

fn default_block_cache_size() -> usize {
    8 * 1024 * 1024
}

fn default_bloom_filter_bits() -> i32 {
    10
}

fn default_true() -> bool {
    true
}

fn default_compression_type() -> Compression {
    Compression::Lz4
}

/// Column family descriptor with options
#[derive(Clone, Debug)]
pub struct ColumnFamilyDescriptorWithConfig {
    /// Column family name
    pub name: String,
    
    /// Column family options
    pub config: ColumnFamilyConfig,
}

/// RocksDB configuration
#[derive(Debug, Clone)]
pub struct RocksDbConfig {
    /// Database path
    pub path: String,
    /// Create the DB if it doesn't exist
    pub create_if_missing: bool,
    /// Create missing column families if they don't exist
    pub create_missing_column_families: bool,
    /// Parallelism level (number of threads)
    pub parallelism: i32,
    /// Optimize for point lookups (read-heavy workloads)
    pub optimize_point_lookup: bool,
    /// Optimize for small data
    pub optimize_small_db: bool,
    /// Use universal compaction
    pub use_universal_compaction: bool,
    /// Default compression type
    pub compression_type: Compression,
    /// Default column families
    pub default_cf_names: Vec<String>,
    /// Disable WAL
    pub disable_wal: bool,
    /// Prefix extractor
    pub prefix_extractor: Option<PrefixExtractor>,
    /// Block size
    pub block_size: usize,
    /// Block cache size
    pub block_cache_size: usize,
    /// Bloom filter bits
    pub bloom_filter_bits: i32,
}

impl Default for RocksDbConfig {
    fn default() -> Self {
        Self {
            path: "data/db".to_string(),
            create_if_missing: true,
            create_missing_column_families: true,
            parallelism: 4,
            optimize_point_lookup: true,
            optimize_small_db: true,
            use_universal_compaction: false,
            compression_type: Compression::Lz4,
            default_cf_names: vec![],
            disable_wal: false,
            prefix_extractor: None,
            block_size: 4096,
            block_cache_size: 8 * 1024 * 1024,
            bloom_filter_bits: 10,
        }
    }
}

/// Compression type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Compression {
    /// No compression
    None,
    /// Snappy compression
    Snappy,
    /// Zlib compression
    Zlib,
    /// Lz4 compression
    Lz4,
    /// Zstd compression
    Zstd,
}

impl From<Compression> for rocksdb::DBCompressionType {
    fn from(compression: Compression) -> Self {
        match compression {
            Compression::None => rocksdb::DBCompressionType::None,
            Compression::Snappy => rocksdb::DBCompressionType::Snappy,
            Compression::Zlib => rocksdb::DBCompressionType::Zlib,
            Compression::Lz4 => rocksdb::DBCompressionType::Lz4,
            Compression::Zstd => rocksdb::DBCompressionType::Zstd,
        }
    }
}

/// Optimize the database options
fn optimize_db_options(options: &mut Options, config: &RocksDbConfig) {
    options.create_if_missing(config.create_if_missing);
    options.create_missing_column_families(config.create_missing_column_families);
    
    // Set parallelism
    options.increase_parallelism(config.parallelism);
    
    // Optimize for point lookups if needed
    if config.optimize_point_lookup {
        options.optimize_for_point_lookup(128 * 1024 * 1024); // 128 MB cache
    }
    
    // Set compression
    options.set_compression_type(config.compression_type.into());
}

/// Optimize the column family options
fn optimize_cf_options(options: &mut Options, config: &RocksDbConfig) {
    // Set compression
    options.set_compression_type(config.compression_type.into());
    
    // Set other optimizations as needed
    if config.optimize_point_lookup {
        options.optimize_for_point_lookup(128 * 1024 * 1024); // 128 MB cache
    }
}

/// RocksDB client wrapper
pub struct RocksDbClient {
    /// The database instance
    db: Arc<Mutex<Option<Arc<DB>>>>,
    
    /// Database configuration
    config: RocksDbConfig,
    
    /// Cache for column family handles
    cf_handles: Arc<Mutex<HashMap<String, String>>>,
    
    /// Column family options
    cf_options: Arc<Mutex<HashMap<String, ColumnFamilyConfig>>>,
}

impl RocksDbClient {
    /// Create a new RocksDB client
    pub fn new(config: RocksDbConfig) -> Self {
        Self {
            db: Arc::new(Mutex::new(None)),
            config,
            cf_handles: Arc::new(Mutex::new(HashMap::new())),
            cf_options: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Open the database
    pub fn open(&self) -> DbResult<()> {
        let mut db_lock = self.db.lock().unwrap();
        
        if db_lock.is_some() {
            return Ok(());
        }
        
        // Create the database directory if it doesn't exist
        let db_path = Path::new(&self.config.path);
        if !db_path.exists() {
            std::fs::create_dir_all(db_path).map_err(|e| DbError::IO(e.to_string()))?;
        }
        
        // Create the database options
        let mut options = Options::default();
        optimize_db_options(&mut options, &self.config);
        
        // Create column family descriptors
        let cf_configs = self.config.default_cf_names.iter().map(|cf_name| ColumnFamilyConfig {
            name: cf_name.clone(),
            prefix_extractor: self.config.prefix_extractor.clone(),
            block_size: self.config.block_size,
            block_cache_size: self.config.block_cache_size,
            bloom_filter_bits: self.config.bloom_filter_bits,
            cache_index_and_filter_blocks: true,
            compression_type: self.config.compression_type,
            options: HashMap::new(),
        }).collect::<Vec<_>>();
        
        let mut cf_descriptors = Vec::new();
        
        for cf_config in cf_configs {
            let mut cf_options = Options::default();
            optimize_cf_options(&mut cf_options, &self.config);
            cf_descriptors.push(ColumnFamilyDescriptor::new(&cf_config.name, cf_options.clone()));
            
            let mut cf_options_map = self.cf_options.lock().unwrap();
            cf_options_map.insert(cf_config.name.clone(), cf_config.clone());
        }
        
        // Open the database with all column families
        let db = DB::open_cf_descriptors(&options, &self.config.path, cf_descriptors)
            .map_err(|e| DbError::RocksDb(e))?;
        
        // Wrap the DB in an Arc
        *db_lock = Some(Arc::new(db));
        
        Ok(())
    }
    
    /// Get access to the database
    fn get_db(&self) -> DbResult<Arc<DB>> {
        // Lock the mutex and get a reference to the Option<Arc<DB>>
        let guard = self.db.lock().unwrap();
        
        // Check if the database is open
        match &*guard {
            Some(arc_db) => Ok(Arc::clone(arc_db)),
            None => Err(DbError::NotOpen),
        }
    }
    
    /// Get a column family handle by name
    fn get_cf_handle_key(&self, cf_name: &str) -> DbResult<String> {
        let db = self.get_db()?;
        
        // Create handle key
        let handle_key = format!("cf_handle:{}", cf_name);
        
        // Check if the CF exists
        if db.cf_handle(cf_name).is_some() {
            Ok(handle_key)
        } else {
            Err(DbError::ColumnFamilyNotFound(cf_name.to_string()))
        }
    }
    
    /// Iterate over a column family
    pub fn iter_cf<V>(
        &self,
        cf_name: &str,
        mode: IteratorMode<'_>,
    ) -> DbResult<Box<dyn Iterator<Item = (Box<[u8]>, V)> + Send>>
    where
        V: DeserializeOwned + Send + 'static,
    {
        let db = self.get_db()?;
        
        let cf_handle = match db.cf_handle(cf_name) {
            Some(handle) => handle,
            None => return Err(DbError::ColumnFamilyNotFound(cf_name.to_string())),
        };
        
        // Get the iterator
        let db_iter = db.iterator_cf(&cf_handle, mode);
        
        // Map the iterator to deserialize values
        let iter = db_iter
            .filter_map(move |result| {
                match result {
                    Ok((k, v)) => {
                        match deserialize::<V>(&v) {
                            Ok(value) => Some((k, value)),
                            Err(e) => {
                                error!("Failed to deserialize value: {}", e);
                                None
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error iterating: {}", e);
                        None
                    }
                }
            });
        
        Ok(Box::new(ThreadSafeIterator::new(iter)))
    }

    /// Iterate over a column family with a prefix
    pub fn prefix_iter_cf<V>(
        &self,
        cf_name: &str,
        prefix: &[u8],
    ) -> DbResult<Box<dyn Iterator<Item = (Box<[u8]>, V)> + Send>>
    where
        V: DeserializeOwned + Send + 'static,
    {
        let db = self.get_db()?;
        
        let cf_handle = match db.cf_handle(cf_name) {
            Some(handle) => handle,
            None => return Err(DbError::ColumnFamilyNotFound(cf_name.to_string())),
        };
        
        // Setup read options with prefix seek
        let mut opts = ReadOptions::default();
        opts.set_prefix_same_as_start(true);
        
        // Create an iterator with the prefix
        let mode = IteratorMode::From(prefix, Direction::Forward);
        let db_iter = db.iterator_cf_opt(&cf_handle, opts, mode);
        
        // Filter by prefix and deserialize values
        let iter = db_iter
            .take_while(move |result| {
                match result {
                    Ok((k, _)) => k.starts_with(prefix),
                    Err(_) => false,
                }
            })
            .filter_map(move |result| {
                match result {
                    Ok((k, v)) => {
                        match deserialize::<V>(&v) {
                            Ok(value) => Some((k, value)),
                            Err(e) => {
                                error!("Failed to deserialize value: {}", e);
                                None
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error iterating: {}", e);
                        None
                    }
                }
            });
        
        Ok(Box::new(ThreadSafeIterator::new(iter)))
    }

    /// Get a value from a column family
    pub fn get_cf<K, V>(&self, cf_name: &str, key: K) -> DbResult<Option<V>>
    where
        K: AsRef<[u8]>,
        V: DeserializeOwned,
    {
        let db = self.get_db()?;
        let cf_handle = match db.cf_handle(cf_name) {
            Some(handle) => handle,
            None => return Err(DbError::ColumnFamilyNotFound(cf_name.to_string())),
        };
        
        let result = db.get_cf(&cf_handle, key.as_ref()).map_err(DbError::RocksDb)?;
        if let Some(value) = result {
            let deserialized = deserialize(&value)?;
            Ok(Some(deserialized))
        } else {
            Ok(None)
        }
    }

    /// Put a value in a column family
    pub fn put_cf<K, V>(&self, cf_name: &str, key: K, value: &V) -> DbResult<()>
    where
        K: AsRef<[u8]>,
        V: Serialize,
    {
        let db = self.get_db()?;
        let cf_handle = match db.cf_handle(cf_name) {
            Some(handle) => handle,
            None => return Err(DbError::ColumnFamilyNotFound(cf_name.to_string())),
        };
        
        let bytes = serialize(value)
            .map_err(|e| DbError::Serialization(e.to_string()))?;
        
        db.put_cf(&cf_handle, key.as_ref(), bytes).map_err(DbError::RocksDb)
    }

    /// Delete a key from a column family
    pub fn delete_cf<K>(&self, cf_name: &str, key: K) -> DbResult<()>
    where
        K: AsRef<[u8]>,
    {
        let db = self.get_db()?;
        let cf_handle = match db.cf_handle(cf_name) {
            Some(handle) => handle,
            None => return Err(DbError::ColumnFamilyNotFound(cf_name.to_string())),
        };
        
        db.delete_cf(&cf_handle, key.as_ref()).map_err(DbError::RocksDb)
    }

    /// Check if a key exists in a column family
    pub fn exists_cf<K>(&self, cf_name: &str, key: K) -> DbResult<bool>
    where
        K: AsRef<[u8]>,
    {
        // Just use get_cf to avoid borrow issues
        match self.get_cf::<_, Vec<u8>>(cf_name, key) {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Get all column family names
    pub fn get_cf_names(&self) -> Vec<String> {
        match self.get_db() {
            Ok(_db) => {
                match DB::list_cf(&Options::default(), &self.config.path) {
                    Ok(names) => names.into_iter().map(|s| s.to_string()).collect(),
                    Err(_) => Vec::new(),
                }
            }
            Err(_) => Vec::new(),
        }
    }

    /// Create a column family if it doesn't exist
    pub fn create_cf_if_missing(&self, cf_name: &str) -> DbResult<()> {
        let db = self.get_db()?;
        
        // Check if the column family already exists
        if db.cf_handle(cf_name).is_none() {
            // Column family doesn't exist, create it
            let mut options = Options::default();
            optimize_cf_options(&mut options, &self.config);
            
            // Create the column family
            db.create_cf(cf_name, &options).map_err(|e| DbError::RocksDb(e))?;
            
            // Verify creation was successful
            if db.cf_handle(cf_name).is_none() {
                return Err(DbError::ColumnFamilyNotFound(cf_name.to_string()));
            }
        }
        
        Ok(())
    }

    /// Execute a batch of operations
    pub fn batch<F>(&self, f: F) -> DbResult<()>
    where
        F: FnOnce(&mut WriteBatch) -> DbResult<()>,
    {
        let db = self.get_db()?;
        let mut batch = WriteBatch::default();
        
        f(&mut batch)?;
        
        db.write(batch).map_err(|e| DbError::RocksDb(e))
    }

    /// Execute a batch of operations on a column family
    pub fn batch_cf<F, T>(&self, cf_name: &str, f: F) -> DbResult<T>
    where
        F: FnOnce(&mut WriteBatch) -> DbResult<T>,
        T: Send + 'static,
    {
        let db = self.get_db()?;
        let _cf_handle = match db.cf_handle(cf_name) {
            Some(handle) => handle,
            None => return Err(DbError::ColumnFamilyNotFound(cf_name.to_string())),
        };
        
        // Create batch
        let mut batch = WriteBatch::default();
        
        // Call the function to fill the batch
        let result = f(&mut batch)?;
        
        // Execute the batch via RocksDbClient's write_batch
        db.write(batch)?;
        
        // Return the result
        Ok(result)
    }

    /// Flush the database
    pub fn flush(&self) -> DbResult<()> {
        let db = self.get_db()?;
        db.flush().map_err(DbError::RocksDb)
    }

    /// Flush a column family
    pub fn flush_cf(&self, cf_name: &str) -> DbResult<()> {
        let db = self.get_db()?;
        let cf_handle = match db.cf_handle(cf_name) {
            Some(handle) => handle,
            None => return Err(DbError::ColumnFamilyNotFound(cf_name.to_string())),
        };
        
        db.flush_cf(&cf_handle).map_err(DbError::RocksDb)
    }

    /// Compact the database
    pub fn compact(&self) -> DbResult<()> {
        let db = self.get_db()?;
        db.compact_range::<&[u8], &[u8]>(None, None);
        Ok(())
    }

    /// Compact a column family
    pub fn compact_cf(&self, cf_name: &str) -> DbResult<()> {
        let db = self.get_db()?;
        let cf_handle = match db.cf_handle(cf_name) {
            Some(handle) => handle,
            None => return Err(DbError::ColumnFamilyNotFound(cf_name.to_string())),
        };
        
        db.compact_range_cf::<&[u8], &[u8]>(&cf_handle, None, None);
        Ok(())
    }

    /// Write a batch of operations to the database
    pub fn write_batch(&self, operations: Vec<BatchOperation>) -> DbResult<()> {
        let db = self.get_db()?;
        let mut batch = WriteBatch::default();
        
        for operation in operations {
            match operation {
                BatchOperation::Put { cf_name, key, value } => {
                    if let Some(handle) = db.cf_handle(&cf_name) {
                        batch.put_cf(&handle, key, value);
                    } else {
                        return Err(DbError::ColumnFamilyNotFound(cf_name));
                    }
                }
                BatchOperation::Delete { cf_name, key } => {
                    if let Some(handle) = db.cf_handle(&cf_name) {
                        batch.delete_cf(&handle, key);
                    } else {
                        return Err(DbError::ColumnFamilyNotFound(cf_name));
                    }
                }
            }
        }
        
        db.write(batch).map_err(|e| DbError::RocksDb(e))
    }

    /// Check if a column family exists
    pub fn column_family_exists(&self, cf_name: &str) -> DbResult<bool> {
        let db = self.get_db()?;
        
        let cf_handle = db.cf_handle(cf_name);
        let cf_exists = cf_handle.is_some();
        
        Ok(cf_exists)
    }

    /// Drop a column family
    pub fn drop_cf(&self, cf_name: &str) -> DbResult<()> {
        let db = self.get_db()?;
        
        // Check if CF exists before attempting to drop
        let cf_exists = match db.cf_handle(cf_name) {
            Some(_handle) => true,
            None => false,
        };
        
        if cf_exists {
            db.drop_cf(cf_name).map_err(DbError::RocksDb)?;
            Ok(())
        } else {
            Err(DbError::ColumnFamilyNotFound(cf_name.to_string()))
        }
    }

    /// Create a backup
    pub fn create_backup(&self) -> DbResult<String> {
        let db = self.get_db()?;
        
        // Generate a backup ID
        let backup_id = format!("backup_{}", chrono::Utc::now().timestamp());
        
        // Get all column family names by listing them
        let cf_list = self.list_column_families()?;
        
        // Process each column family
        for cf_name in cf_list {
            if let Some(_handle) = db.cf_handle(&cf_name) {
                // TODO: implement actual backup logic
            }
        }
        
        Ok(backup_id)
    }

    /// Restore from a backup
    pub fn restore_backup(&self, _backup_id: &str) -> DbResult<()> {
        // Implementation 
        Ok(())
    }

    /// Delete all keys with a prefix in a column family
    pub fn delete_prefix_cf(&self, cf_name: &str, prefix: &[u8]) -> DbResult<()> {
        // Get all keys with the given prefix using the prefix iterator
        let keys: Vec<Box<[u8]>> = {
            let iter = self.prefix_iter_cf::<Vec<u8>>(cf_name, prefix)?;
            iter.map(|(k, _)| k).collect()
        };
        
        if keys.is_empty() {
            return Ok(());
        }
        
        // Get a handle to the DB
        let db = self.get_db()?;
        
        // Get the column family handle
        let cf_handle = db.cf_handle(cf_name)
            .ok_or_else(|| DbError::ColumnFamilyNotFound(cf_name.to_string()))?;
        
        // Use a batch operation to delete all keys
        let mut batch = WriteBatch::default();
        
        for key in keys {
            batch.delete_cf(&cf_handle, key);
        }
        
        db.write(batch).map_err(|e| DbError::RocksDb(e))?;
        Ok(())
    }
    
    /// List all column families
    pub fn list_column_families(&self) -> DbResult<Vec<String>> {
        // Get the DB path from the config
        let path = &self.config.path;
        
        // Use the static list_column_families method
        match DB::list_cf(&Options::default(), path) {
            Ok(cf_names) => Ok(cf_names),
            Err(e) => Err(DbError::RocksDb(e)),
        }
    }
}

/// Batch operation type for the write_batch method
#[derive(Debug, Clone)]
pub enum BatchOperation {
    /// Put operation
    Put {
        /// Column family name
        cf_name: String,
        /// Key
        key: Vec<u8>,
        /// Value
        value: Vec<u8>,
    },
    /// Delete operation
    Delete {
        /// Column family name
        cf_name: String,
        /// Key
        key: Vec<u8>,
    },
}

/// Async RocksDB client implementation
#[derive(Clone)]
pub struct AsyncRocksDbClient {
    /// Inner RocksDB client
    db: Arc<RocksDbClient>
}

impl AsyncRocksDbClient {
    /// Create a new async RocksDB client
    pub fn new(config: RocksDbConfig) -> Self {
        let db = Arc::new(RocksDbClient::new(config));
        Self { db }
    }
    
    /// Get a value from a column family
    pub async fn get_cf<K, V>(&self, cf_name: &str, key: K) -> DbResult<Option<V>>
    where
        K: AsRef<[u8]> + Send + 'static,
        V: DeserializeOwned + Send + 'static,
    {
        let db = self.db.clone();
        let cf_name = cf_name.to_string();
        let key_bytes = key.as_ref().to_vec();
        
        let result = tokio::task::spawn_blocking(move || {
            // Get the DB client
            let rocks_db = match db.get_db() {
                Ok(db) => db,
                Err(e) => return Err(e),
            };
            
            // Get the column family handle
            let cf_handle = match rocks_db.cf_handle(&cf_name) {
                Some(handle) => handle,
                None => return Err(DbError::ColumnFamilyNotFound(cf_name)),
            };
            
            // Get the value
            match rocks_db.get_cf(&cf_handle, &key_bytes) {
                Ok(Some(bytes)) => {
                    // Deserialize the value
                    match deserialize(&bytes) {
                        Ok(value) => Ok(Some(value)),
                        Err(e) => Err(DbError::Deserialization(e.to_string())),
                    }
                },
                Ok(None) => Ok(None),
                Err(e) => Err(DbError::RocksDb(e)),
            }
        }).await;
        
        match result {
            Ok(r) => r,
            Err(e) => Err(DbError::Tokio(e.to_string())),
        }
    }

    /// Check if a key exists in a column family
    pub async fn exists_cf<K>(&self, cf_name: &str, key: K) -> DbResult<bool>
    where
        K: AsRef<[u8]> + Send + 'static,
    {
        let db = self.db.clone();
        let cf_name = cf_name.to_string();
        let key_bytes = key.as_ref().to_vec();
        
        tokio::task::spawn_blocking(move || {
            // Get the DB client
            let rocks_db = match db.get_db() {
                Ok(db) => db,
                Err(e) => return Err(e),
            };
            
            // Get the column family handle
            let cf_handle = match rocks_db.cf_handle(&cf_name) {
                Some(handle) => handle,
                None => return Err(DbError::ColumnFamilyNotFound(cf_name)),
            };
            
            // Get the value
            match rocks_db.get_cf(&cf_handle, &key_bytes) {
                Ok(Some(_)) => Ok(true),
                Ok(None) => Ok(false),
                Err(e) => Err(DbError::RocksDb(e)),
            }
        }).await.map_err(|e| DbError::Tokio(e.to_string()))?
    }

    /// Put a value in a column family
    pub async fn put_cf<K, V>(&self, cf_name: &str, key: K, value: V) -> DbResult<()>
    where
        K: AsRef<[u8]> + Send + 'static,
        V: Serialize + Send + 'static,
    {
        let db = self.db.clone();
        let cf_name = cf_name.to_string();
        let key_bytes = key.as_ref().to_vec();
        let value_bytes = serialize(&value)
            .map_err(|e| DbError::Serialization(e.to_string()))?;
        
        tokio::task::spawn_blocking(move || {
            // Get the DB client
            let rocks_db = match db.get_db() {
                Ok(db) => db,
                Err(e) => return Err(e),
            };
            
            // Get the column family handle
            let cf_handle = match rocks_db.cf_handle(&cf_name) {
                Some(handle) => handle,
                None => return Err(DbError::ColumnFamilyNotFound(cf_name)),
            };
            
            rocks_db.put_cf(&cf_handle, &key_bytes, &value_bytes)
                .map_err(DbError::RocksDb)
        }).await.map_err(|e| DbError::Tokio(e.to_string()))?
    }

    /// Delete a key from a column family
    pub async fn delete_cf<K>(&self, cf_name: &str, key: K) -> DbResult<()>
    where
        K: AsRef<[u8]> + Send + 'static,
    {
        let db = self.db.clone();
        let cf_name = cf_name.to_string();
        let key_bytes = key.as_ref().to_vec();
        
        tokio::task::spawn_blocking(move || {
            // Get the DB client
            let rocks_db = match db.get_db() {
                Ok(db) => db,
                Err(e) => return Err(e),
            };
            
            // Get the column family handle
            let cf_handle = match rocks_db.cf_handle(&cf_name) {
                Some(handle) => handle,
                None => return Err(DbError::ColumnFamilyNotFound(cf_name)),
            };
            
            rocks_db.delete_cf(&cf_handle, &key_bytes)
                .map_err(DbError::RocksDb)
        }).await.map_err(|e| DbError::Tokio(e.to_string()))?
    }

    /// Iterate over a column family
    pub async fn iter_cf<V>(
        &self,
        cf_name: &str,
        _mode: IteratorMode<'_>,
    ) -> DbResult<Box<dyn Iterator<Item = (Box<[u8]>, V)> + Send>>
    where
        V: DeserializeOwned + Send + 'static,
    {
        let db = self.db.clone();
        let cf_name = cf_name.to_string();
        
        // Create a start iterator mode for the spawned task
        let mode_start = IteratorMode::Start;
        
        tokio::task::spawn_blocking(move || {
            db.iter_cf::<V>(&cf_name, mode_start)
        }).await.map_err(|e| DbError::Tokio(e.to_string()))?
    }
    
    /// Iterate over a column family with a prefix
    pub async fn prefix_iter_cf<V>(
        &self,
        cf_name: &str,
        prefix: &[u8],
    ) -> DbResult<Box<dyn Iterator<Item = (Box<[u8]>, V)> + Send>>
    where
        V: DeserializeOwned + Send + 'static,
    {
        let db = self.db.clone();
        let cf_name = cf_name.to_string();
        let prefix = prefix.to_vec();
        
        tokio::task::spawn_blocking(move || {
            db.prefix_iter_cf::<V>(&cf_name, &prefix)
        }).await.map_err(|e| DbError::Tokio(e.to_string()))?
    }
    
    /// Collect all key-value pairs with a given prefix
    pub async fn collect_prefix<V>(
        &self,
        cf_name: &str,
        prefix: &[u8],
    ) -> DbResult<Vec<(Box<[u8]>, V)>>
    where
        V: DeserializeOwned + Send + 'static,
    {
        let db = self.db.clone();
        let cf_name = cf_name.to_string();
        let prefix = prefix.to_vec();
        
        tokio::task::spawn_blocking(move || {
            let iter = db.prefix_iter_cf::<V>(&cf_name, &prefix)?;
            Ok(iter.collect::<Vec<_>>())
        }).await.map_err(|e| DbError::Tokio(e.to_string()))?
    }
    
    /// Collect all key-value pairs from a column family
    pub async fn collect_cf<V>(&self, cf_name: &str) -> DbResult<Vec<(String, V)>>
    where
        V: DeserializeOwned + Send + 'static,
    {
        let db = self.db.clone();
        let cf_name = cf_name.to_string();
        
        tokio::task::spawn_blocking(move || {
            let iter = db.iter_cf::<V>(&cf_name, IteratorMode::Start)?;
            let result: Vec<(String, V)> = iter
                .map(|(k, v)| {
                    let key_str = String::from_utf8_lossy(&k).to_string();
                    (key_str, v)
                })
                .collect();
            Ok(result)
        }).await.map_err(|e| DbError::Tokio(e.to_string()))?
    }
    
    /// Execute a batch of operations
    pub async fn write_batch(&self, ops: Vec<BatchOperation>) -> DbResult<()> {
        let db = self.db.clone();
        
        tokio::task::spawn_blocking(move || {
            let mut batch = WriteBatch::default();
            
            for op in ops {
                match op {
                    BatchOperation::Put { cf_name, key, value } => {
                        if let Some(handle) = db.get_db()?.cf_handle(&cf_name) {
                            batch.put_cf(&handle, key, value);
                        } else {
                            return Err(DbError::ColumnFamilyNotFound(cf_name));
                        }
                    },
                    BatchOperation::Delete { cf_name, key } => {
                        if let Some(handle) = db.get_db()?.cf_handle(&cf_name) {
                            batch.delete_cf(&handle, key);
                        } else {
                            return Err(DbError::ColumnFamilyNotFound(cf_name));
                        }
                    },
                }
            }
            
            db.get_db()?.write(batch).map_err(DbError::RocksDb)
        }).await.map_err(|e| DbError::Tokio(e.to_string()))?
    }
    
    /// Flush a column family
    pub async fn flush_cf(&self, cf_name: &str) -> DbResult<()> {
        let db = self.db.clone();
        let cf_name = cf_name.to_string();
        
        tokio::task::spawn_blocking(move || {
            // Get the DB client
            let rocks_db = match db.get_db() {
                Ok(db) => db,
                Err(e) => return Err(e),
            };
            
            // Get the column family handle
            let cf_handle = match rocks_db.cf_handle(&cf_name) {
                Some(handle) => handle,
                None => return Err(DbError::ColumnFamilyNotFound(cf_name)),
            };
            
            rocks_db.flush_cf(&cf_handle)
                .map_err(DbError::RocksDb)
        }).await.map_err(|e| DbError::Tokio(e.to_string()))?
    }
    
    /// Compact a column family
    pub async fn compact_cf(&self, cf_name: &str) -> DbResult<()> {
        let db = self.db.clone();
        let cf_name = cf_name.to_string();
        
        tokio::task::spawn_blocking(move || {
            // Get the DB client
            let rocks_db = match db.get_db() {
                Ok(db) => db,
                Err(e) => return Err(e),
            };
            
            // Get the column family handle
            let cf_handle = match rocks_db.cf_handle(&cf_name) {
                Some(handle) => handle,
                None => return Err(DbError::ColumnFamilyNotFound(cf_name)),
            };
            
            rocks_db.compact_range_cf::<&[u8], &[u8]>(&cf_handle, None, None);
            Ok(())
        }).await.map_err(|e| DbError::Tokio(e.to_string()))?
    }
}

/// Macro to implement repository helper methods for a type
#[macro_export]
macro_rules! repository_impl {
    ($struct_name:ident, $client_type:ident, $entity_type:ident, $key_fn:expr) => {
        #[async_trait::async_trait]
        impl crate::repository::DbRepository<$entity_type> for $struct_name {
            async fn create(&self, entity: $entity_type) -> DbResult<()> {
                let key = ($key_fn)(&entity);
                self.db.put_cf(Self::cf_name().as_str(), key, entity).await
            }

            async fn update(&self, entity: $entity_type) -> DbResult<()> {
                let key = ($key_fn)(&entity);
                self.db.put_cf(Self::cf_name().as_str(), key, entity).await
            }

            async fn delete(&self, entity: $entity_type) -> DbResult<()> {
                let key = ($key_fn)(&entity);
                self.db.delete_cf(Self::cf_name().as_str(), key).await
            }

            async fn get(&self, id: String) -> DbResult<Option<$entity_type>> {
                self.db.get_cf::<_, $entity_type>(Self::cf_name().as_str(), id).await
            }
        }
    };
}

/// Database repository trait for domain entities
#[async_trait]
pub trait DbRepository<T, ID> {
    /// Get entity by ID
    async fn get_by_id(&self, id: ID) -> DbResult<Option<T>>;

    /// Save entity
    async fn save(&self, entity: T) -> DbResult<()>;

    /// Delete entity
    async fn delete(&self, id: ID) -> DbResult<()>;

    /// Check if entity exists
    async fn exists(&self, id: ID) -> DbResult<bool>;

    /// List all entities
    async fn list_all(&self) -> DbResult<Vec<T>>;
}

/// Re-export
pub use crate::repository_impl;

/// Wrapper type for serializing RocksDB keys
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DbKey(pub String);

impl From<Box<[u8]>> for DbKey {
    fn from(value: Box<[u8]>) -> Self {
        DbKey(String::from_utf8_lossy(&value).to_string())
    }
}

impl From<DbKey> for String {
    fn from(key: DbKey) -> Self {
        key.0
    }
}

impl AsRef<[u8]> for DbKey {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
