// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! In-memory storage implementation for the Fully Homomorphic Encryption service.

use crate::{
    FheCiphertext, FheCiphertextId, FheError, FheKeyPair, FheKeyPairId, FhePrivateKey,
    FhePrivateKeyId, FhePublicKey, FhePublicKeyId, FheResult, FheStorage,
};
use async_trait::async_trait;
use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, RwLock},
};

/// In-memory storage for Fully Homomorphic Encryption data.
#[derive(Debug)]
pub struct MemoryFheStorage {
    /// Key pairs stored in memory.
    key_pairs: Arc<RwLock<HashMap<FheKeyPairId, FheKeyPair>>>,
    /// Public keys stored in memory.
    public_keys: Arc<RwLock<HashMap<FhePublicKeyId, FhePublicKey>>>,
    /// Private keys stored in memory.
    private_keys: Arc<RwLock<HashMap<FhePrivateKeyId, FhePrivateKey>>>,
    /// Ciphertexts stored in memory.
    ciphertexts: Arc<RwLock<HashMap<FheCiphertextId, FheCiphertext>>>,
}

impl MemoryFheStorage {
    /// Create a new in-memory storage.
    pub fn new() -> Self {
        Self {
            key_pairs: Arc::new(RwLock::new(HashMap::new())),
            public_keys: Arc::new(RwLock::new(HashMap::new())),
            private_keys: Arc::new(RwLock::new(HashMap::new())),
            ciphertexts: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for MemoryFheStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl FheStorage for MemoryFheStorage {
    async fn store_key_pair(&self, key_pair: &FheKeyPair) -> FheResult<()> {
        let mut key_pairs = self
            .key_pairs
            .write()
            .map_err(|e| FheError::StorageError(format!("Failed to acquire write lock: {}", e)))?;
        key_pairs.insert(key_pair.id.clone(), key_pair.clone());
        Ok(())
    }

    async fn get_key_pair(&self, id: &FheKeyPairId) -> FheResult<FheKeyPair> {
        let key_pairs = self
            .key_pairs
            .read()
            .map_err(|e| FheError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        key_pairs
            .get(id)
            .cloned()
            .ok_or_else(|| FheError::MissingDataError(format!("Key pair not found: {}", id)))
    }

    async fn list_key_pairs(&self) -> FheResult<Vec<FheKeyPair>> {
        let key_pairs = self
            .key_pairs
            .read()
            .map_err(|e| FheError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(key_pairs.values().cloned().collect())
    }

    async fn delete_key_pair(&self, id: &FheKeyPairId) -> FheResult<()> {
        let mut key_pairs = self
            .key_pairs
            .write()
            .map_err(|e| FheError::StorageError(format!("Failed to acquire write lock: {}", e)))?;
        key_pairs.remove(id);
        Ok(())
    }

    async fn store_public_key(&self, key: &FhePublicKey) -> FheResult<()> {
        let mut public_keys = self
            .public_keys
            .write()
            .map_err(|e| FheError::StorageError(format!("Failed to acquire write lock: {}", e)))?;
        public_keys.insert(key.id.clone(), key.clone());
        Ok(())
    }

    async fn get_public_key(&self, id: &FhePublicKeyId) -> FheResult<FhePublicKey> {
        let public_keys = self
            .public_keys
            .read()
            .map_err(|e| FheError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        public_keys
            .get(id)
            .cloned()
            .ok_or_else(|| FheError::MissingDataError(format!("Public key not found: {}", id)))
    }

    async fn list_public_keys(&self) -> FheResult<Vec<FhePublicKey>> {
        let public_keys = self
            .public_keys
            .read()
            .map_err(|e| FheError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(public_keys.values().cloned().collect())
    }

    async fn delete_public_key(&self, id: &FhePublicKeyId) -> FheResult<()> {
        let mut public_keys = self
            .public_keys
            .write()
            .map_err(|e| FheError::StorageError(format!("Failed to acquire write lock: {}", e)))?;
        public_keys.remove(id);
        Ok(())
    }

    async fn store_private_key(&self, key: &FhePrivateKey) -> FheResult<()> {
        let mut private_keys = self
            .private_keys
            .write()
            .map_err(|e| FheError::StorageError(format!("Failed to acquire write lock: {}", e)))?;
        private_keys.insert(key.id.clone(), key.clone());
        Ok(())
    }

    async fn get_private_key(&self, id: &FhePrivateKeyId) -> FheResult<FhePrivateKey> {
        let private_keys = self
            .private_keys
            .read()
            .map_err(|e| FheError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        private_keys
            .get(id)
            .cloned()
            .ok_or_else(|| FheError::MissingDataError(format!("Private key not found: {}", id)))
    }

    async fn list_private_keys(&self) -> FheResult<Vec<FhePrivateKey>> {
        let private_keys = self
            .private_keys
            .read()
            .map_err(|e| FheError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(private_keys.values().cloned().collect())
    }

    async fn delete_private_key(&self, id: &FhePrivateKeyId) -> FheResult<()> {
        let mut private_keys = self
            .private_keys
            .write()
            .map_err(|e| FheError::StorageError(format!("Failed to acquire write lock: {}", e)))?;
        private_keys.remove(id);
        Ok(())
    }

    async fn store_ciphertext(&self, ciphertext: &FheCiphertext) -> FheResult<()> {
        let mut ciphertexts = self
            .ciphertexts
            .write()
            .map_err(|e| FheError::StorageError(format!("Failed to acquire write lock: {}", e)))?;
        ciphertexts.insert(ciphertext.id.clone(), ciphertext.clone());
        Ok(())
    }

    async fn get_ciphertext(&self, id: &FheCiphertextId) -> FheResult<FheCiphertext> {
        let ciphertexts = self
            .ciphertexts
            .read()
            .map_err(|e| FheError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        ciphertexts
            .get(id)
            .cloned()
            .ok_or_else(|| FheError::MissingDataError(format!("Ciphertext not found: {}", id)))
    }

    async fn list_ciphertexts(&self) -> FheResult<Vec<FheCiphertext>> {
        let ciphertexts = self
            .ciphertexts
            .read()
            .map_err(|e| FheError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(ciphertexts.values().cloned().collect())
    }

    async fn list_ciphertexts_by_public_key(
        &self,
        public_key_id: &FhePublicKeyId,
    ) -> FheResult<Vec<FheCiphertext>> {
        let ciphertexts = self
            .ciphertexts
            .read()
            .map_err(|e| FheError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(ciphertexts
            .values()
            .filter(|c| c.public_key_id == *public_key_id)
            .cloned()
            .collect())
    }

    async fn delete_ciphertext(&self, id: &FheCiphertextId) -> FheResult<()> {
        let mut ciphertexts = self
            .ciphertexts
            .write()
            .map_err(|e| FheError::StorageError(format!("Failed to acquire write lock: {}", e)))?;
        ciphertexts.remove(id);
        Ok(())
    }
}
