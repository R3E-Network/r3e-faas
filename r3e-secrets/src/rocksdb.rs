// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_trait::async_trait;
use r3e_store::RocksDBStore;
use r3e_store::rocksdb::RocksDbConfig;
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
        let config = RocksDbConfig {
            path: db_path.as_ref().to_string_lossy().to_string(),
            ..Default::default()
        };
        
        let db = RocksDBStore::new(config);
        
        // Open the database
        db.open().map_err(|e| SecretError::Storage(format!("Failed to open RocksDB store: {}", e)))?;
        
        // Create column family if it doesn't exist
        let secrets_cf = "secrets".to_string();
        db.create_cf_if_missing(&secrets_cf)
            .map_err(|e| SecretError::Storage(format!("Failed to create column family: {}", e)))?;

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
        let key = Self::generate_key(&secret.user_id, &secret.function_id, &secret.id);
        let value = serde_json::to_vec(&secret)
            .map_err(|e| SecretError::Storage(format!("Failed to serialize secret: {}", e)))?;

        self.db
            .put_cf(&self.secrets_cf, key, &value)
            .map_err(|e| SecretError::Storage(format!("Failed to store secret: {}", e)))?;

        Ok(())
    }

    async fn get_secret(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
    ) -> Result<EncryptedSecret, SecretError> {
        let key = Self::generate_key(user_id, function_id, secret_id);

        match self.db.get_cf::<_, Vec<u8>>(&self.secrets_cf, key) {
            Ok(Some(value)) => {
                let secret = serde_json::from_slice::<EncryptedSecret>(&value).map_err(|e| {
                    SecretError::Storage(format!("Failed to deserialize secret: {}", e))
                })?;
                Ok(secret)
            }
            Ok(None) => Err(SecretError::NotFound(format!(
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
        let key = Self::generate_key(user_id, function_id, secret_id);

        // Check if secret exists
        match self.db.get_cf::<_, Vec<u8>>(&self.secrets_cf, &key) {
            Ok(Some(_)) => {
                self.db
                    .delete_cf(&self.secrets_cf, &key)
                    .map_err(|e| SecretError::Storage(format!("Failed to delete secret: {}", e)))?;
                Ok(())
            }
            Ok(None) => Err(SecretError::NotFound(format!(
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
        
        // Get an iterator for the prefix and manually iterate through it
        let db = self.db.clone();
        let cf_name = self.secrets_cf.clone();
        
        // Create a prefix iterator and collect the results manually
        let iter: Box<dyn Iterator<Item = (Box<[u8]>, Box<[u8]>)> + Send> = 
            db.prefix_iter_cf(&cf_name, prefix.as_bytes())
            .map_err(|e| SecretError::Storage(format!("Failed to scan secrets: {}", e)))?;
        
        let mut secrets = Vec::new();
        
        // Iterate through key-value pairs using an iterator
        for pair in iter {
            // Destructure the pair into key and value (as boxed slices)
            let (_, value_boxed) = pair;
            
            // Convert boxed slice to Vec<u8> for use with serde_json
            let value_vec = value_boxed.to_vec();
            
            // Deserialize the value
            let secret = serde_json::from_slice::<EncryptedSecret>(&value_vec).map_err(|e| {
                SecretError::Storage(format!("Failed to deserialize secret: {}", e))
            })?;

            if secret.user_id == user_id && secret.function_id == function_id {
                secrets.push(secret);
            }
        }

        Ok(secrets)
    }
}
