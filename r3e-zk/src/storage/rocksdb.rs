// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! RocksDB storage implementation for the Zero-Knowledge computing service.

use crate::{
    ZkCircuit, ZkCircuitId, ZkError, ZkProof, ZkProofId, ZkProvingKey, ZkProvingKeyId, ZkResult,
    ZkStorage, ZkVerificationKey, ZkVerificationKeyId,
};
use async_trait::async_trait;
use log::{debug, error};
use rocksdb::{ColumnFamily, ColumnFamilyDescriptor, Options, DB};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::Mutex;

/// Column family names for RocksDB.
const CF_CIRCUITS: &str = "circuits";
const CF_PROVING_KEYS: &str = "proving_keys";
const CF_VERIFICATION_KEYS: &str = "verification_keys";
const CF_PROOFS: &str = "proofs";

/// RocksDB storage for Zero-Knowledge data.
#[derive(Debug)]
pub struct RocksDbZkStorage {
    /// Path to the RocksDB database.
    path: PathBuf,
    /// RocksDB database instance.
    db: Arc<Mutex<DB>>,
}

impl RocksDbZkStorage {
    /// Create a new RocksDB storage.
    pub fn new<P: AsRef<Path>>(path: P) -> ZkResult<Self> {
        let path = path.as_ref().to_path_buf();
        let db = Self::open_db(&path)?;
        Ok(Self {
            path,
            db: Arc::new(Mutex::new(db)),
        })
    }

    /// Open the RocksDB database.
    fn open_db(path: &Path) -> ZkResult<DB> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);

        let cf_opts = Options::default();
        let cf_descriptors = vec![
            ColumnFamilyDescriptor::new(CF_CIRCUITS, cf_opts.clone()),
            ColumnFamilyDescriptor::new(CF_PROVING_KEYS, cf_opts.clone()),
            ColumnFamilyDescriptor::new(CF_VERIFICATION_KEYS, cf_opts.clone()),
            ColumnFamilyDescriptor::new(CF_PROOFS, cf_opts),
        ];

        DB::open_cf_descriptors(&opts, path, cf_descriptors).map_err(|e| {
            error!("Failed to open RocksDB: {}", e);
            ZkError::StorageError(format!("Failed to open RocksDB: {}", e))
        })
    }

    /// Get a column family handle.
    async fn get_cf(&self, name: &str) -> ZkResult<Arc<ColumnFamily>> {
        let db = self.db.lock().await;
        db.cf_handle(name)
            .map(Arc::new)
            .ok_or_else(|| ZkError::StorageError(format!("Column family not found: {}", name)))
    }

    /// Serialize a value to bytes.
    fn serialize<T: Serialize>(value: &T) -> ZkResult<Vec<u8>> {
        serde_json::to_vec(value).map_err(|e| {
            error!("Serialization error: {}", e);
            ZkError::SerializationError(format!("Failed to serialize: {}", e))
        })
    }

    /// Deserialize bytes to a value.
    fn deserialize<T: DeserializeOwned>(bytes: &[u8]) -> ZkResult<T> {
        serde_json::from_slice(bytes).map_err(|e| {
            error!("Deserialization error: {}", e);
            ZkError::SerializationError(format!("Failed to deserialize: {}", e))
        })
    }

    /// Store a value in a column family.
    async fn store<K: AsRef<[u8]>, V: Serialize>(
        &self,
        cf_name: &str,
        key: K,
        value: &V,
    ) -> ZkResult<()> {
        let cf = self.get_cf(cf_name).await?;
        let value_bytes = Self::serialize(value)?;
        let mut db = self.db.lock().await;
        db.put_cf(&cf, key, value_bytes).map_err(|e| {
            error!("RocksDB put error: {}", e);
            ZkError::StorageError(format!("Failed to store data: {}", e))
        })
    }

    /// Get a value from a column family.
    async fn get<K: AsRef<[u8]>, V: DeserializeOwned>(
        &self,
        cf_name: &str,
        key: K,
    ) -> ZkResult<Option<V>> {
        let cf = self.get_cf(cf_name).await?;
        let db = self.db.lock().await;
        match db.get_cf(&cf, key).map_err(|e| {
            error!("RocksDB get error: {}", e);
            ZkError::StorageError(format!("Failed to get data: {}", e))
        })? {
            Some(bytes) => Ok(Some(Self::deserialize(&bytes)?)),
            None => Ok(None),
        }
    }

    /// Delete a value from a column family.
    async fn delete<K: AsRef<[u8]>>(&self, cf_name: &str, key: K) -> ZkResult<()> {
        let cf = self.get_cf(cf_name).await?;
        let mut db = self.db.lock().await;
        db.delete_cf(&cf, key).map_err(|e| {
            error!("RocksDB delete error: {}", e);
            ZkError::StorageError(format!("Failed to delete data: {}", e))
        })
    }

    /// List all values in a column family with a prefix.
    async fn list<V: DeserializeOwned, F: Fn(&[u8]) -> bool>(
        &self,
        cf_name: &str,
        filter: F,
    ) -> ZkResult<Vec<V>> {
        let cf = self.get_cf(cf_name).await?;
        let db = self.db.lock().await;
        let iter = db.iterator_cf(&cf, rocksdb::IteratorMode::Start);
        let mut results = Vec::new();

        for item in iter {
            let (key, value) = item.map_err(|e| {
                error!("RocksDB iterator error: {}", e);
                ZkError::StorageError(format!("Failed to iterate data: {}", e))
            })?;

            if filter(&key) {
                let value = Self::deserialize(&value)?;
                results.push(value);
            }
        }

        Ok(results)
    }
}

#[async_trait]
impl ZkStorage for RocksDbZkStorage {
    async fn store_circuit(&self, circuit: &ZkCircuit) -> ZkResult<()> {
        debug!("Storing circuit: {}", circuit.id);
        self.store(CF_CIRCUITS, circuit.id.to_string(), circuit)
            .await
    }

    async fn get_circuit(&self, id: &ZkCircuitId) -> ZkResult<ZkCircuit> {
        debug!("Getting circuit: {}", id);
        match self
            .get::<_, ZkCircuit>(CF_CIRCUITS, id.to_string())
            .await?
        {
            Some(circuit) => Ok(circuit),
            None => Err(ZkError::MissingDataError(format!(
                "Circuit not found: {}",
                id
            ))),
        }
    }

    async fn list_circuits(&self) -> ZkResult<Vec<ZkCircuit>> {
        debug!("Listing all circuits");
        self.list::<ZkCircuit, _>(CF_CIRCUITS, |_| true).await
    }

    async fn delete_circuit(&self, id: &ZkCircuitId) -> ZkResult<()> {
        debug!("Deleting circuit: {}", id);
        self.delete(CF_CIRCUITS, id.to_string()).await
    }

    async fn store_proving_key(&self, key: &ZkProvingKey) -> ZkResult<()> {
        debug!("Storing proving key: {}", key.id);
        self.store(CF_PROVING_KEYS, key.id.to_string(), key).await
    }

    async fn get_proving_key(&self, id: &ZkProvingKeyId) -> ZkResult<ZkProvingKey> {
        debug!("Getting proving key: {}", id);
        match self
            .get::<_, ZkProvingKey>(CF_PROVING_KEYS, id.to_string())
            .await?
        {
            Some(key) => Ok(key),
            None => Err(ZkError::MissingDataError(format!(
                "Proving key not found: {}",
                id
            ))),
        }
    }

    async fn list_proving_keys(&self, circuit_id: &ZkCircuitId) -> ZkResult<Vec<ZkProvingKey>> {
        debug!("Listing proving keys for circuit: {}", circuit_id);
        let keys = self
            .list::<ZkProvingKey, _>(CF_PROVING_KEYS, |_| true)
            .await?;
        Ok(keys
            .into_iter()
            .filter(|key| key.circuit_id == *circuit_id)
            .collect())
    }

    async fn delete_proving_key(&self, id: &ZkProvingKeyId) -> ZkResult<()> {
        debug!("Deleting proving key: {}", id);
        self.delete(CF_PROVING_KEYS, id.to_string()).await
    }

    async fn store_verification_key(&self, key: &ZkVerificationKey) -> ZkResult<()> {
        debug!("Storing verification key: {}", key.id);
        self.store(CF_VERIFICATION_KEYS, key.id.to_string(), key)
            .await
    }

    async fn get_verification_key(&self, id: &ZkVerificationKeyId) -> ZkResult<ZkVerificationKey> {
        debug!("Getting verification key: {}", id);
        match self
            .get::<_, ZkVerificationKey>(CF_VERIFICATION_KEYS, id.to_string())
            .await?
        {
            Some(key) => Ok(key),
            None => Err(ZkError::MissingDataError(format!(
                "Verification key not found: {}",
                id
            ))),
        }
    }

    async fn list_verification_keys(
        &self,
        circuit_id: &ZkCircuitId,
    ) -> ZkResult<Vec<ZkVerificationKey>> {
        debug!("Listing verification keys for circuit: {}", circuit_id);
        let keys = self
            .list::<ZkVerificationKey, _>(CF_VERIFICATION_KEYS, |_| true)
            .await?;
        Ok(keys
            .into_iter()
            .filter(|key| key.circuit_id == *circuit_id)
            .collect())
    }

    async fn delete_verification_key(&self, id: &ZkVerificationKeyId) -> ZkResult<()> {
        debug!("Deleting verification key: {}", id);
        self.delete(CF_VERIFICATION_KEYS, id.to_string()).await
    }

    async fn store_proof(&self, proof: &ZkProof) -> ZkResult<()> {
        debug!("Storing proof: {}", proof.id);
        self.store(CF_PROOFS, proof.id.to_string(), proof).await
    }

    async fn get_proof(&self, id: &ZkProofId) -> ZkResult<ZkProof> {
        debug!("Getting proof: {}", id);
        match self.get::<_, ZkProof>(CF_PROOFS, id.to_string()).await? {
            Some(proof) => Ok(proof),
            None => Err(ZkError::MissingDataError(format!(
                "Proof not found: {}",
                id
            ))),
        }
    }

    async fn list_proofs(&self, circuit_id: &ZkCircuitId) -> ZkResult<Vec<ZkProof>> {
        debug!("Listing proofs for circuit: {}", circuit_id);
        let proofs = self.list::<ZkProof, _>(CF_PROOFS, |_| true).await?;
        Ok(proofs
            .into_iter()
            .filter(|proof| proof.circuit_id == *circuit_id)
            .collect())
    }

    async fn delete_proof(&self, id: &ZkProofId) -> ZkResult<()> {
        debug!("Deleting proof: {}", id);
        self.delete(CF_PROOFS, id.to_string()).await
    }
}
