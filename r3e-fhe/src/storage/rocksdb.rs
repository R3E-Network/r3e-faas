// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! RocksDB storage implementation for the Fully Homomorphic Encryption service.

use crate::{
    FheCiphertext, FheCiphertextId, FheError, FheKeyPair, FheKeyPairId, FhePrivateKey,
    FhePrivateKeyId, FhePublicKey, FhePublicKeyId, FheResult, FheStorage,
};
use async_trait::async_trait;
use log::{debug, info};
use rocksdb::{ColumnFamilyDescriptor, Options, DB};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::Mutex;

/// Column family names for RocksDB.
const CF_KEY_PAIRS: &str = "key_pairs";
const CF_PUBLIC_KEYS: &str = "public_keys";
const CF_PRIVATE_KEYS: &str = "private_keys";
const CF_CIPHERTEXTS: &str = "ciphertexts";

/// RocksDB storage for Fully Homomorphic Encryption data.
#[derive(Debug)]
pub struct RocksDbFheStorage {
    /// Path to the RocksDB database.
    path: PathBuf,
    /// RocksDB database instance.
    db: Arc<Mutex<DB>>,
}

impl RocksDbFheStorage {
    /// Create a new RocksDB storage.
    pub fn new<P: AsRef<Path>>(path: P) -> FheResult<Self> {
        let path = path.as_ref().to_path_buf();
        info!("Initializing RocksDB storage at: {:?}", path);

        // Create column family options
        let mut cf_opts = Options::default();
        cf_opts.set_max_write_buffer_number(16);
        cf_opts.set_min_write_buffer_number_to_merge(1);
        cf_opts.set_max_background_jobs(4);
        cf_opts.set_level_zero_file_num_compaction_trigger(4);
        cf_opts.set_level_zero_slowdown_writes_trigger(20);
        cf_opts.set_level_zero_stop_writes_trigger(36);
        cf_opts.set_target_file_size_base(64 * 1024 * 1024); // 64MB
        cf_opts.set_max_bytes_for_level_base(512 * 1024 * 1024); // 512MB
        cf_opts.set_write_buffer_size(64 * 1024 * 1024); // 64MB
        cf_opts.set_compression_type(rocksdb::DBCompressionType::Lz4);

        // Create column family descriptors
        let cf_descriptors = vec![
            ColumnFamilyDescriptor::new(CF_KEY_PAIRS, cf_opts.clone()),
            ColumnFamilyDescriptor::new(CF_PUBLIC_KEYS, cf_opts.clone()),
            ColumnFamilyDescriptor::new(CF_PRIVATE_KEYS, cf_opts.clone()),
            ColumnFamilyDescriptor::new(CF_CIPHERTEXTS, cf_opts),
        ];

        // Create database options
        let mut db_opts = Options::default();
        db_opts.create_if_missing(true);
        db_opts.create_missing_column_families(true);

        // Open the database
        let db = DB::open_cf_descriptors(&db_opts, &path, cf_descriptors)
            .map_err(|e| FheError::RocksDbError(format!("Failed to open RocksDB: {}", e)))?;

        Ok(Self {
            path,
            db: Arc::new(Mutex::new(db)),
        })
    }

    /// Serialize a value to JSON.
    fn serialize<T: Serialize>(value: &T) -> FheResult<Vec<u8>> {
        serde_json::to_vec(value)
            .map_err(|e| FheError::SerializationError(format!("Failed to serialize: {}", e)))
    }

    /// Deserialize a value from JSON.
    fn deserialize<T: DeserializeOwned>(data: &[u8]) -> FheResult<T> {
        serde_json::from_slice(data)
            .map_err(|e| FheError::SerializationError(format!("Failed to deserialize: {}", e)))
    }

    /// Get a value from a column family.
    async fn get<T: DeserializeOwned, K: AsRef<[u8]> + Debug>(
        &self,
        cf_name: &str,
        key: K,
    ) -> FheResult<Option<T>> {
        let db = self.db.lock().await;
        let cf = db.cf_handle(cf_name).ok_or_else(|| {
            FheError::StorageError(format!("Column family not found: {}", cf_name))
        })?;

        match db.get_cf(cf, key.as_ref()) {
            Ok(Some(data)) => {
                let value = Self::deserialize(&data)?;
                Ok(Some(value))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(FheError::RocksDbError(format!(
                "Failed to get value from RocksDB: {}",
                e
            ))),
        }
    }

    /// Put a value into a column family.
    async fn put<T: Serialize, K: AsRef<[u8]> + Debug>(
        &self,
        cf_name: &str,
        key: K,
        value: &T,
    ) -> FheResult<()> {
        let db = self.db.lock().await;
        let cf = db.cf_handle(cf_name).ok_or_else(|| {
            FheError::StorageError(format!("Column family not found: {}", cf_name))
        })?;

        let serialized = Self::serialize(value)?;
        db.put_cf(cf, key.as_ref(), serialized)
            .map_err(|e| FheError::RocksDbError(format!("Failed to put value to RocksDB: {}", e)))
    }

    /// Delete a value from a column family.
    async fn delete<K: AsRef<[u8]> + Debug>(&self, cf_name: &str, key: K) -> FheResult<()> {
        let db = self.db.lock().await;
        let cf = db.cf_handle(cf_name).ok_or_else(|| {
            FheError::StorageError(format!("Column family not found: {}", cf_name))
        })?;

        db.delete_cf(cf, key.as_ref()).map_err(|e| {
            FheError::RocksDbError(format!("Failed to delete value from RocksDB: {}", e))
        })
    }

    /// List all values in a column family.
    async fn list<T: DeserializeOwned>(&self, cf_name: &str) -> FheResult<Vec<T>> {
        let db = self.db.lock().await;
        let cf = db.cf_handle(cf_name).ok_or_else(|| {
            FheError::StorageError(format!("Column family not found: {}", cf_name))
        })?;

        let mut values = Vec::new();
        let iter = db.iterator_cf(cf, rocksdb::IteratorMode::Start);
        for result in iter {
            match result {
                Ok((_, value)) => {
                    let deserialized = Self::deserialize(&value)?;
                    values.push(deserialized);
                }
                Err(e) => {
                    return Err(FheError::RocksDbError(format!(
                        "Failed to iterate over RocksDB: {}",
                        e
                    )))
                }
            }
        }

        Ok(values)
    }
}

#[async_trait]
impl FheStorage for RocksDbFheStorage {
    async fn store_key_pair(&self, key_pair: &FheKeyPair) -> FheResult<()> {
        debug!("Storing key pair: {}", key_pair.id);
        self.put(CF_KEY_PAIRS, key_pair.id.0.to_string(), key_pair)
            .await
    }

    async fn get_key_pair(&self, id: &FheKeyPairId) -> FheResult<FheKeyPair> {
        debug!("Getting key pair: {}", id);
        match self
            .get::<FheKeyPair, _>(CF_KEY_PAIRS, id.0.to_string())
            .await?
        {
            Some(key_pair) => Ok(key_pair),
            None => Err(FheError::MissingDataError(format!(
                "Key pair not found: {}",
                id
            ))),
        }
    }

    async fn list_key_pairs(&self) -> FheResult<Vec<FheKeyPair>> {
        debug!("Listing key pairs");
        self.list::<FheKeyPair>(CF_KEY_PAIRS).await
    }

    async fn delete_key_pair(&self, id: &FheKeyPairId) -> FheResult<()> {
        debug!("Deleting key pair: {}", id);
        self.delete(CF_KEY_PAIRS, id.0.to_string()).await
    }

    async fn store_public_key(&self, key: &FhePublicKey) -> FheResult<()> {
        debug!("Storing public key: {}", key.id);
        self.put(CF_PUBLIC_KEYS, key.id.0.to_string(), key).await
    }

    async fn get_public_key(&self, id: &FhePublicKeyId) -> FheResult<FhePublicKey> {
        debug!("Getting public key: {}", id);
        match self
            .get::<FhePublicKey, _>(CF_PUBLIC_KEYS, id.0.to_string())
            .await?
        {
            Some(key) => Ok(key),
            None => Err(FheError::MissingDataError(format!(
                "Public key not found: {}",
                id
            ))),
        }
    }

    async fn list_public_keys(&self) -> FheResult<Vec<FhePublicKey>> {
        debug!("Listing public keys");
        self.list::<FhePublicKey>(CF_PUBLIC_KEYS).await
    }

    async fn delete_public_key(&self, id: &FhePublicKeyId) -> FheResult<()> {
        debug!("Deleting public key: {}", id);
        self.delete(CF_PUBLIC_KEYS, id.0.to_string()).await
    }

    async fn store_private_key(&self, key: &FhePrivateKey) -> FheResult<()> {
        debug!("Storing private key: {}", key.id);
        self.put(CF_PRIVATE_KEYS, key.id.0.to_string(), key).await
    }

    async fn get_private_key(&self, id: &FhePrivateKeyId) -> FheResult<FhePrivateKey> {
        debug!("Getting private key: {}", id);
        match self
            .get::<FhePrivateKey, _>(CF_PRIVATE_KEYS, id.0.to_string())
            .await?
        {
            Some(key) => Ok(key),
            None => Err(FheError::MissingDataError(format!(
                "Private key not found: {}",
                id
            ))),
        }
    }

    async fn list_private_keys(&self) -> FheResult<Vec<FhePrivateKey>> {
        debug!("Listing private keys");
        self.list::<FhePrivateKey>(CF_PRIVATE_KEYS).await
    }

    async fn delete_private_key(&self, id: &FhePrivateKeyId) -> FheResult<()> {
        debug!("Deleting private key: {}", id);
        self.delete(CF_PRIVATE_KEYS, id.0.to_string()).await
    }

    async fn store_ciphertext(&self, ciphertext: &FheCiphertext) -> FheResult<()> {
        debug!("Storing ciphertext: {}", ciphertext.id);
        self.put(CF_CIPHERTEXTS, ciphertext.id.0.to_string(), ciphertext)
            .await
    }

    async fn get_ciphertext(&self, id: &FheCiphertextId) -> FheResult<FheCiphertext> {
        debug!("Getting ciphertext: {}", id);
        match self
            .get::<FheCiphertext, _>(CF_CIPHERTEXTS, id.0.to_string())
            .await?
        {
            Some(ciphertext) => Ok(ciphertext),
            None => Err(FheError::MissingDataError(format!(
                "Ciphertext not found: {}",
                id
            ))),
        }
    }

    async fn list_ciphertexts(&self) -> FheResult<Vec<FheCiphertext>> {
        debug!("Listing ciphertexts");
        self.list::<FheCiphertext>(CF_CIPHERTEXTS).await
    }

    async fn list_ciphertexts_by_public_key(
        &self,
        public_key_id: &FhePublicKeyId,
    ) -> FheResult<Vec<FheCiphertext>> {
        debug!("Listing ciphertexts by public key: {}", public_key_id);
        let all_ciphertexts = self.list::<FheCiphertext>(CF_CIPHERTEXTS).await?;
        Ok(all_ciphertexts
            .into_iter()
            .filter(|c| c.public_key_id == *public_key_id)
            .collect())
    }

    async fn delete_ciphertext(&self, id: &FheCiphertextId) -> FheResult<()> {
        debug!("Deleting ciphertext: {}", id);
        self.delete(CF_CIPHERTEXTS, id.0.to_string()).await
    }
}
