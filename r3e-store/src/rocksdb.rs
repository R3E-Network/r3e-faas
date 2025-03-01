// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_trait::async_trait;
use log::{debug, error, info, warn};
use ::rocksdb::{
    BlockBasedOptions, Cache, ColumnFamilyDescriptor, DBCompactionStyle, 
    DBCompressionType, Direction, Error as RocksError, IteratorMode, Options, ReadOptions, 
    SliceTransform, WriteBatch, DB,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use thiserror::Error;

use crate::*;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("RocksDB error: {0}")]
    RocksDb(#[from] RocksError),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Column family not found: {0}")]
    ColumnFamilyNotFound(String),

    #[error("Key not found: {0}")]
    KeyNotFound(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Database not open")]
    NotOpen,

    #[error("Database already open")]
    AlreadyOpen,

    #[error("Transaction failed: {0}")]
    TransactionFailed(String),
}

/// Result type for RocksDB operations
pub type DbResult<T> = Result<T, DbError>;

/// Configuration for a RocksDB column family
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColumnFamilyConfig {
    /// Name of the column family
    pub name: String,

    /// Optional prefix extractor
    pub prefix_extractor: Option<usize>,

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

/// Configuration for RocksDB
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RocksDbConfig {
    /// Path to the database
    pub path: String,

    /// Create the database if it doesn't exist
    #[serde(default = "default_true")]
    pub create_if_missing: bool,

    /// Create missing column families
    #[serde(default = "default_true")]
    pub create_missing_column_families: bool,

    /// Increase parallelism for background operations
    #[serde(default = "default_parallelism")]
    pub increase_parallelism: i32,

    /// Optimize for point lookups
    #[serde(default = "default_true")]
    pub optimize_point_lookup: bool,

    /// Optimize for level style compaction
    #[serde(default = "default_true")]
    pub optimize_level_style_compaction: bool,

    /// Write buffer size in bytes (default: 64MB)
    #[serde(default = "default_write_buffer_size")]
    pub write_buffer_size: usize,

    /// Max write buffer number (default: 3)
    #[serde(default = "default_max_write_buffer_number")]
    pub max_write_buffer_number: i32,

    /// Min write buffer number to merge (default: 1)
    #[serde(default = "default_min_write_buffer_number_to_merge")]
    pub min_write_buffer_number_to_merge: i32,

    /// Column families configuration
    #[serde(default = "Vec::new")]
    pub column_families: Vec<ColumnFamilyConfig>,
}

fn default_parallelism() -> i32 {
    num_cpus::get() as i32
}

fn default_write_buffer_size() -> usize {
    64 * 1024 * 1024
}

fn default_max_write_buffer_number() -> i32 {
    3
}

fn default_min_write_buffer_number_to_merge() -> i32 {
    1
}

/// RocksDB client wrapper
pub struct RocksDbClient {
    db: Option<Arc<DB>>,
    config: RocksDbConfig,
    // We store column family names to track which ones we know about
    column_families: Arc<Mutex<HashSet<String>>>,
}

impl RocksDbClient {
    /// Create a new RocksDB client
    pub fn new(config: RocksDbConfig) -> Self {
        Self {
            db: None,
            config,
            column_families: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// Open the database
    pub fn open(&mut self) -> DbResult<()> {
        if self.db.is_some() {
            return Err(DbError::AlreadyOpen);
        }

        let path = Path::new(&self.config.path);
        let mut opts = Options::default();
        self.configure_db_options(&mut opts);

        // Create column family descriptors
        let cf_names: Vec<String> = self
            .config
            .column_families
            .iter()
            .map(|cf| cf.name.clone())
            .collect();

        let cf_descriptors: Vec<ColumnFamilyDescriptor> = self
            .config
            .column_families
            .iter()
            .map(|cf_config| {
                let mut cf_opts = Options::default();
                self.configure_cf_options(&mut cf_opts, cf_config);
                ColumnFamilyDescriptor::new(cf_config.name.clone(), cf_opts)
            })
            .collect();

        let db = if path.exists() {
            info!("Opening existing RocksDB at {}", self.config.path);

            // Check which column families exist
            let existing_cfs = match DB::list_cf(&opts, &self.config.path) {
                Ok(cfs) => cfs,
                Err(e) => {
                    error!("Failed to list column families: {}", e);
                    if self.config.create_if_missing {
                        Vec::new()
                    } else {
                        return Err(DbError::RocksDb(e));
                    }
                }
            };

            debug!("Existing column families: {:?}", existing_cfs);

            // Open the database with existing column families
            let db = if existing_cfs.is_empty() {
                DB::open(&opts, &self.config.path)?
            } else {
                let mut open_cfs = Vec::new();

                // Create descriptors for existing column families
                for cf_name in &existing_cfs {
                    // Find configuration for this CF
                    let cf_config = self
                        .config
                        .column_families
                        .iter()
                        .find(|cf| cf.name == *cf_name)
                        .cloned()
                        .unwrap_or_else(|| {
                            warn!(
                                "No configuration found for column family {}. Using defaults.",
                                cf_name
                            );
                            ColumnFamilyConfig {
                                name: cf_name.clone(),
                                prefix_extractor: None,
                                block_size: default_block_size(),
                                block_cache_size: default_block_cache_size(),
                                bloom_filter_bits: default_bloom_filter_bits(),
                                cache_index_and_filter_blocks: default_true(),
                            }
                        });

                    let mut cf_opts = Options::default();
                    self.configure_cf_options(&mut cf_opts, &cf_config);
                    open_cfs.push(ColumnFamilyDescriptor::new(cf_name, cf_opts));
                }

                DB::open_cf_descriptors(&opts, &self.config.path, open_cfs)?
            };

            // Create missing column families if configured
            if self.config.create_missing_column_families {
                // Create a copy of existing_cfs to avoid ownership issues
                let existing_cfs_copy = existing_cfs.clone();
                let existing_cfs_set: HashSet<String> = existing_cfs_copy.into_iter().collect();
                for cf_config in &self.config.column_families {
                    if !existing_cfs_set.contains(&cf_config.name) {
                        info!("Creating column family: {}", cf_config.name);
                        let mut cf_opts = Options::default();
                        self.configure_cf_options(&mut cf_opts, cf_config);
                        db.create_cf(&cf_config.name, &cf_opts)?;
                    }
                }
            }

            db
        } else {
            if !self.config.create_if_missing {
                return Err(DbError::InvalidConfig(format!(
                    "Database path {} does not exist and create_if_missing is false",
                    self.config.path
                )));
            }

            info!("Creating new RocksDB at {}", self.config.path);

            if cf_descriptors.is_empty() {
                DB::open(&opts, &self.config.path)?
            } else {
                DB::open_cf_descriptors(&opts, &self.config.path, cf_descriptors)?
            }
        };

        // Store column family names
        let mut cf_set = self.column_families.lock().unwrap();
        for cf_name in &cf_names {
            match db.cf_handle(cf_name) {
                Some(_) => {
                    cf_set.insert(cf_name.clone());
                }
                None => {
                    warn!("Failed to get column family handle for {}", cf_name);
                }
            }
        }

        self.db = Some(Arc::new(db));
        info!("RocksDB opened successfully at {}", self.config.path);
        Ok(())
    }

    /// Configure database options
    fn configure_db_options(&self, opts: &mut Options) {
        opts.create_if_missing(self.config.create_if_missing);
        opts.create_missing_column_families(self.config.create_missing_column_families);
        opts.increase_parallelism(self.config.increase_parallelism);

        if self.config.optimize_point_lookup {
            opts.optimize_for_point_lookup(128 * 1024 * 1024); // 128MB
        }

        if self.config.optimize_level_style_compaction {
            opts.optimize_level_style_compaction(512 * 1024 * 1024); // 512MB
        }

        opts.set_write_buffer_size(self.config.write_buffer_size);
        opts.set_max_write_buffer_number(self.config.max_write_buffer_number);
        opts.set_min_write_buffer_number_to_merge(self.config.min_write_buffer_number_to_merge);
        opts.set_compaction_style(DBCompactionStyle::Level);
        opts.set_compression_type(DBCompressionType::Lz4);
        opts.set_bottommost_compression_type(DBCompressionType::Zstd);
    }

    /// Configure column family options
    fn configure_cf_options(&self, opts: &mut Options, cf_config: &ColumnFamilyConfig) {
        // Set prefix extractor if configured
        if let Some(prefix_len) = cf_config.prefix_extractor {
            opts.set_prefix_extractor(SliceTransform::create_fixed_prefix(prefix_len));
        }

        // Configure block cache and bloom filter
        let mut block_opts = BlockBasedOptions::default();
        block_opts.set_block_size(cf_config.block_size);
        block_opts.set_cache_index_and_filter_blocks(cf_config.cache_index_and_filter_blocks);
        block_opts.set_bloom_filter(cf_config.bloom_filter_bits as f64, false);
        block_opts.set_block_cache(&Cache::new_lru_cache(cf_config.block_cache_size));
        opts.set_block_based_table_factory(&block_opts);
    }

    /// Get the RocksDB instance
    fn get_db(&self) -> DbResult<Arc<DB>> {
        match &self.db {
            Some(db) => Ok(db.clone()),
            None => Err(DbError::NotOpen),
        }
    }

    /// Get a column family handle
    fn get_cf_handle(&self, cf_name: &str) -> DbResult<String> {
        let db = self.get_db()?;
        
        // Check if we know about this column family
        let cf_set = self.column_families.lock().unwrap();
        let known = cf_set.contains(cf_name);
        drop(cf_set); // Release the lock
        
        if !known {
            // Try to get the CF handle from the database
            match db.cf_handle(cf_name) {
                Some(_) => {
                    // Add to our set of known column families
                    let mut cf_set = self.column_families.lock().unwrap();
                    cf_set.insert(cf_name.to_string());
                }
                None => return Err(DbError::ColumnFamilyNotFound(cf_name.to_string())),
            }
        }
        
        // Return the column family name
        Ok(cf_name.to_string())
    }

    /// Put a value in a column family
    pub fn put_cf<K, V>(&self, cf_name: &str, key: K, value: &V) -> DbResult<()>
    where
        K: AsRef<[u8]>,
        V: Serialize,
    {
        let db = self.get_db()?;
        let cf_name = self.get_cf_handle(cf_name)?;

        let serialized_value =
            bincode::serialize(value).map_err(|e| DbError::Serialization(e.to_string()))?;

        let cf = db.cf_handle(&cf_name).ok_or_else(|| DbError::ColumnFamilyNotFound(cf_name.clone()))?;
        db.put_cf(cf, key, serialized_value)?;
        Ok(())
    }

    /// Get a value from a column family
    pub fn get_cf<K, V>(&self, cf_name: &str, key: K) -> DbResult<Option<V>>
    where
        K: AsRef<[u8]>,
        V: DeserializeOwned,
    {
        let db = self.get_db()?;
        let cf_name = self.get_cf_handle(cf_name)?;

        let cf = db.cf_handle(&cf_name).ok_or_else(|| DbError::ColumnFamilyNotFound(cf_name.clone()))?;
        match db.get_cf(cf, key)? {
            Some(data) => {
                let value = bincode::deserialize(&data)
                    .map_err(|e| DbError::Deserialization(e.to_string()))?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    /// Delete a key from a column family
    pub fn delete_cf<K>(&self, cf_name: &str, key: K) -> DbResult<()>
    where
        K: AsRef<[u8]>,
    {
        let db = self.get_db()?;
        let cf = self.get_cf_handle(cf_name)?;

        db.delete_cf(&cf, key)?;
        Ok(())
    }

    /// Check if a key exists in a column family
    pub fn exists_cf<K>(&self, cf_name: &str, key: K) -> DbResult<bool>
    where
        K: AsRef<[u8]>,
    {
        let db = self.get_db()?;
        let cf = self.get_cf_handle(cf_name)?;

        let value = db.get_cf(&cf, key)?;
        Ok(value.is_some())
    }

    /// Iterate over a column family and collect results
    pub fn iter_cf<V>(
        &self,
        cf_name: &str,
        mode: IteratorMode,
    ) -> DbResult<Vec<(Box<[u8]>, V)>>
    where
        V: DeserializeOwned,
    {
        let db = self.get_db()?;
        let cf = self.get_cf_handle(cf_name)?;

        let iter = db.iterator_cf(&cf, mode);

        let results: Vec<(Box<[u8]>, V)> = iter
            .map(|result| {
                result.map_err(DbError::RocksDb).and_then(|(key, value)| {
                    let deserialized = bincode::deserialize(&value)
                        .map_err(|e| DbError::Deserialization(e.to_string()))?;
                    Ok((key, deserialized))
                })
            })
            .filter_map(|result| match result {
                Ok(item) => Some(item),
                Err(e) => {
                    error!("Error iterating over column family {}: {}", cf_name, e);
                    None
                }
            })
            .collect();

        Ok(results)
    }

    /// Iterate over keys with a common prefix and collect results
    pub fn prefix_iter_cf<P, V>(
        &self,
        cf_name: &str,
        prefix: P,
    ) -> DbResult<Vec<(Box<[u8]>, V)>>
    where
        P: AsRef<[u8]>,
        V: DeserializeOwned,
    {
        let db = self.get_db()?;
        let cf = self.get_cf_handle(cf_name)?;

        let mut read_opts = ReadOptions::default();
        read_opts.set_prefix_same_as_start(true);

        let iter = db.iterator_cf_opt(
            &cf,
            read_opts,
            IteratorMode::From(prefix.as_ref(), Direction::Forward),
        );

        let prefix_bytes = prefix.as_ref().to_vec();
        let results: Vec<(Box<[u8]>, V)> = iter
            .take_while(move |result| match result {
                Ok((key, _)) => key.starts_with(&prefix_bytes),
                Err(_) => false,
            })
            .map(|result| {
                result.map_err(DbError::RocksDb).and_then(|(key, value)| {
                    let deserialized = bincode::deserialize(&value)
                        .map_err(|e| DbError::Deserialization(e.to_string()))?;
                    Ok((key, deserialized))
                })
            })
            .filter_map(|result| match result {
                Ok(item) => Some(item),
                Err(e) => {
                    error!(
                        "Error iterating over prefix in column family {}: {}",
                        cf_name, e
                    );
                    None
                }
            })
            .collect();

        Ok(results)
    }

    /// Execute a batch of operations
    pub fn batch<F>(&self, f: F) -> DbResult<()>
    where
        F: FnOnce(&mut WriteBatch) -> DbResult<()>,
    {
        let db = self.get_db()?;
        let mut batch = WriteBatch::default();

        f(&mut batch)?;

        db.write(batch)?;
        Ok(())
    }

    /// Execute a batch of operations in a column family
    pub fn batch_cf<F>(&self, cf_name: &str, f: F) -> DbResult<()>
    where
        F: FnOnce(&mut WriteBatch, &ColumnFamily) -> DbResult<()>,
    {
        let db = self.get_db()?;
        let cf = self.get_cf_handle(cf_name)?;
        let mut batch = WriteBatch::default();

        f(&mut batch, cf)?;

        db.write(batch)?;
        Ok(())
    }

    /// Get database path
    pub fn path(&self) -> PathBuf {
        PathBuf::from(&self.config.path)
    }

    /// Flush the database
    pub fn flush(&self) -> DbResult<()> {
        let db = self.get_db()?;
        db.flush()?;
        Ok(())
    }

    /// Flush a column family
    pub fn flush_cf(&self, cf_name: &str) -> DbResult<()> {
        let db = self.get_db()?;
        let cf = self.get_cf_handle(cf_name)?;

        db.flush_cf(&cf)?;
        Ok(())
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
        let cf = self.get_cf_handle(cf_name)?;

        db.compact_range_cf::<&[u8], &[u8]>(&cf, None, None);
        Ok(())
    }
}

/// Asynchronous RocksDB client
pub struct AsyncRocksDbClient {
    pub inner: Arc<RocksDbClient>,
}

impl AsyncRocksDbClient {
    /// Create a new async RocksDB client
    pub fn new(config: RocksDbConfig) -> Self {
        let mut client = RocksDbClient::new(config);
        // Open the database synchronously
        if let Err(e) = client.open() {
            error!("Failed to open RocksDB: {}", e);
        }
        Self {
            inner: Arc::new(client),
        }
    }

    /// Put a value in a column family
    pub async fn put_cf<K, V>(&self, cf_name: &str, key: K, value: &V) -> DbResult<()>
    where
        K: AsRef<[u8]> + Send + 'static,
        V: Serialize + Send + 'static,
    {
        let inner = self.inner.clone();
        let cf_name = cf_name.to_string();
        let serialized_value =
            bincode::serialize(value).map_err(|e| DbError::Serialization(e.to_string()))?;
        let key_bytes = key.as_ref().to_vec();

        tokio::task::spawn_blocking(move || {
            inner.put_cf(&cf_name, key_bytes, &serialized_value)
        })
        .await
        .map_err(|e| DbError::TransactionFailed(e.to_string()))??;

        Ok(())
    }

    /// Get a value from a column family
    pub async fn get_cf<K, V>(&self, cf_name: &str, key: K) -> DbResult<Option<V>>
    where
        K: AsRef<[u8]> + Send + 'static,
        V: DeserializeOwned + Send + 'static,
    {
        let inner = self.inner.clone();
        let cf_name = cf_name.to_string();
        let key_bytes = key.as_ref().to_vec();

        tokio::task::spawn_blocking(move || {
            inner.get_cf(&cf_name, key_bytes)
        })
        .await
        .map_err(|e| DbError::TransactionFailed(e.to_string()))?
    }

    /// Delete a key from a column family
    pub async fn delete_cf<K>(&self, cf_name: &str, key: K) -> DbResult<()>
    where
        K: AsRef<[u8]> + Send + 'static,
    {
        let inner = self.inner.clone();
        let cf_name = cf_name.to_string();
        let key_bytes = key.as_ref().to_vec();

        tokio::task::spawn_blocking(move || {
            inner.delete_cf(&cf_name, key_bytes)
        })
        .await
        .map_err(|e| DbError::TransactionFailed(e.to_string()))??;

        Ok(())
    }

    /// Check if a key exists in a column family
    pub async fn exists_cf<K>(&self, cf_name: &str, key: K) -> DbResult<bool>
    where
        K: AsRef<[u8]> + Send + 'static,
    {
        let inner = self.inner.clone();
        let cf_name = cf_name.to_string();
        let key_bytes = key.as_ref().to_vec();

        tokio::task::spawn_blocking(move || {
            inner.exists_cf(&cf_name, key_bytes)
        })
        .await
        .map_err(|e| DbError::TransactionFailed(e.to_string()))?
    }

    /// Execute a batch of operations
    pub async fn batch<F, Fut>(&self, f: F) -> DbResult<()>
    where
        F: FnOnce(Arc<RocksDbClient>) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = DbResult<()>> + Send + 'static,
    {
        let inner = self.inner.clone();

        tokio::task::spawn(async move { f(inner).await })
            .await
            .map_err(|e| DbError::TransactionFailed(e.to_string()))??;

        Ok(())
    }

    /// Flush the database
    pub async fn flush(&self) -> DbResult<()> {
        let inner = self.inner.clone();

        tokio::task::spawn_blocking(move || inner.flush())
            .await
            .map_err(|e| DbError::TransactionFailed(e.to_string()))??;

        Ok(())
    }

    /// Compact the database
    pub async fn compact(&self) -> DbResult<()> {
        let inner = self.inner.clone();

        tokio::task::spawn_blocking(move || inner.compact())
            .await
            .map_err(|e| DbError::TransactionFailed(e.to_string()))??;

        Ok(())
    }
}

/// Database repository trait for domain entities
#[async_trait]
pub trait DbRepository<T, ID> {
    /// Get entity by ID
    async fn get_by_id(&self, id: &ID) -> DbResult<Option<T>>;

    /// Save entity
    async fn save(&self, entity: &T) -> DbResult<()>;

    /// Delete entity
    async fn delete(&self, id: &ID) -> DbResult<()>;

    /// Check if entity exists
    async fn exists(&self, id: &ID) -> DbResult<bool>;

    /// List all entities
    async fn list_all(&self) -> DbResult<Vec<T>>;
}

/// Default implementation for most common operations
#[macro_export]
macro_rules! impl_db_repository {
    ($repo:ident, $entity:ty, $id:ty, $cf_name:expr, $id_fn:expr) => {
        #[async_trait::async_trait]
        impl $crate::rocksdb::DbRepository<$entity, $id> for $repo {
            async fn get_by_id(&self, id: &$id) -> $crate::rocksdb::DbResult<Option<$entity>> {
                let key = format!("{}", id);
                self.db.get_cf($cf_name, key).await
            }

            async fn save(&self, entity: &$entity) -> $crate::rocksdb::DbResult<()> {
                let id = $id_fn(entity);
                let key = format!("{}", id);
                self.db.put_cf($cf_name, key, entity).await
            }

            async fn delete(&self, id: &$id) -> $crate::rocksdb::DbResult<()> {
                let key = format!("{}", id);
                self.db.delete_cf($cf_name, key).await
            }

            async fn exists(&self, id: &$id) -> $crate::rocksdb::DbResult<bool> {
                let key = format!("{}", id);
                self.db.exists_cf($cf_name, key).await
            }

            async fn list_all(&self) -> $crate::rocksdb::DbResult<Vec<$entity>> {
                // This is a simplified implementation that loads all entities in memory
                // For a real implementation with large datasets, consider using pagination
                let inner = self.db.inner.clone();
                let cf_name = $cf_name.to_string();

                tokio::task::spawn_blocking(move || {
                    let iter = inner.iter_cf::<$entity>(&cf_name, ::rocksdb::IteratorMode::Start)?;
                    let entities: Vec<$entity> = iter.into_iter().map(|(_, v)| v).collect();
                    Ok(entities)
                })
                .await
                .map_err(|e| $crate::rocksdb::DbError::TransactionFailed(e.to_string()))?
            }
        }
    };
}

// Re-export
pub use impl_db_repository;
