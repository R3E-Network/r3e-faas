// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_trait::async_trait;
use r3e_store::rocksdb::RocksDBStore;
use std::path::Path;
use std::sync::Arc;

use crate::storage::SecretStorage;
use crate::{EncryptedSecret, SecretError};

/// RocksDB implementation of SecretStorage
pub struct RocksDBSecretStorage {
    db: Arc<RocksDBStore>,
    secrets_cf: String,
}

impl RocksDBSecretStorage {
    /// Create a new RocksDB secret storage
    pub async fn new<P: AsRef<Path>>(db_path: P) -> Result<Self, SecretError> {
        let db = RocksDBStore::new(db_path)
            .map_err(|e| SecretError::Storage(format!("Failed to create RocksDB store: {}", e)))?;

        let secrets_cf = "secrets".to_string();

        Ok(Self {
            db: Arc::new(db),
            secrets_cf,
        })
    }

    /// Generate a composite key for storing secrets
    fn generate_key(user_id: &str, function_id: &str, secret_id: &str) -> String {
        format!("{}:{}:{}", user_id, function_id, secret_id)
    }
}

#[async_trait]
impl SecretStorage for RocksDBSecretStorage {
    async fn store_secret(&self, secret: EncryptedSecret) -> Result<(), SecretError> {
        let key = Self::generate_key(&secret.user_id, &secret.function_id, &secret.id)
            .as_bytes()
            .to_vec();
        let value = serde_json::to_vec(&secret)
            .map_err(|e| SecretError::Storage(format!("Failed to serialize secret: {}", e)))?;

        let input = r3e_store::PutInput {
            key: &key,
            value: &value,
            if_not_exists: false,
        };

        self.db
            .put(&self.secrets_cf, input)
            .map_err(|e| SecretError::Storage(format!("Failed to store secret: {}", e)))?;

        Ok(())
    }

    async fn get_secret(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
    ) -> Result<EncryptedSecret, SecretError> {
        let key = Self::generate_key(user_id, function_id, secret_id)
            .as_bytes()
            .to_vec();

        match self.db.get(&self.secrets_cf, &key) {
            Ok(value) => {
                let secret = serde_json::from_slice::<EncryptedSecret>(&value).map_err(|e| {
                    SecretError::Storage(format!("Failed to deserialize secret: {}", e))
                })?;
                Ok(secret)
            }
            Err(r3e_store::GetError::NoSuchKey) => Err(SecretError::NotFound(format!(
                "Secret not found: {}",
                secret_id
            ))),
            Err(e) => Err(SecretError::Storage(format!("Failed to get secret: {}", e))),
        }
    }

    async fn delete_secret(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
    ) -> Result<(), SecretError> {
        let key = Self::generate_key(user_id, function_id, secret_id)
            .as_bytes()
            .to_vec();

        // Check if secret exists
        match self.db.get(&self.secrets_cf, &key) {
            Ok(_) => {
                self.db
                    .delete(&self.secrets_cf, &key)
                    .map_err(|e| SecretError::Storage(format!("Failed to delete secret: {}", e)))?;
                Ok(())
            }
            Err(r3e_store::GetError::NoSuchKey) => Err(SecretError::NotFound(format!(
                "Secret not found: {}",
                secret_id
            ))),
            Err(e) => Err(SecretError::Storage(format!("Failed to get secret: {}", e))),
        }
    }

    async fn list_function_secrets(
        &self,
        user_id: &str,
        function_id: &str,
    ) -> Result<Vec<EncryptedSecret>, SecretError> {
        let prefix = format!("{}:{}", user_id, function_id);
        let prefix_bytes = prefix.as_bytes();

        let input = r3e_store::ScanInput {
            start_key: prefix_bytes,
            start_exclusive: false,
            end_key: &[],
            end_inclusive: false,
            max_count: 1000, // Reasonable limit
        };

        let output = self
            .db
            .scan(&self.secrets_cf, input)
            .map_err(|e| SecretError::Storage(format!("Failed to scan secrets: {}", e)))?;

        let mut secrets = Vec::new();

        for (_, value) in output.kvs {
            let secret = serde_json::from_slice::<EncryptedSecret>(&value).map_err(|e| {
                SecretError::Storage(format!("Failed to deserialize secret: {}", e))
            })?;

            if secret.user_id == user_id && secret.function_id == function_id {
                secrets.push(secret);
            }
        }

        Ok(secrets)
    }
}
