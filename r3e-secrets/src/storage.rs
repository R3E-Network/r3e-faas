// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{EncryptedSecret, SecretError};

/// Secret storage trait
#[async_trait]
pub trait SecretStorage: Send + Sync {
    /// Store a secret
    async fn store_secret(&self, secret: EncryptedSecret) -> Result<(), SecretError>;

    /// Get a secret
    async fn get_secret(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
    ) -> Result<EncryptedSecret, SecretError>;

    /// Delete a secret
    async fn delete_secret(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
    ) -> Result<(), SecretError>;

    /// List secrets for a function
    async fn list_function_secrets(
        &self,
        user_id: &str,
        function_id: &str,
    ) -> Result<Vec<EncryptedSecret>, SecretError>;
}

/// Memory-based implementation of SecretStorage
pub struct MemorySecretStorage {
    secrets: Arc<RwLock<HashMap<String, EncryptedSecret>>>,
}

impl MemorySecretStorage {
    /// Create a new memory-based secret storage
    pub fn new() -> Self {
        Self {
            secrets: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Generate a composite key for storing secrets
    fn generate_key(user_id: &str, function_id: &str, secret_id: &str) -> String {
        format!("{}:{}:{}", user_id, function_id, secret_id)
    }
}

#[async_trait]
impl SecretStorage for MemorySecretStorage {
    async fn store_secret(&self, secret: EncryptedSecret) -> Result<(), SecretError> {
        let key = Self::generate_key(&secret.user_id, &secret.function_id, &secret.id);
        let mut secrets = self.secrets.write().await;
        secrets.insert(key, secret);
        Ok(())
    }

    async fn get_secret(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
    ) -> Result<EncryptedSecret, SecretError> {
        let key = Self::generate_key(user_id, function_id, secret_id);
        let secrets = self.secrets.read().await;

        match secrets.get(&key) {
            Some(secret) => Ok(secret.clone()),
            None => Err(SecretError::NotFound(format!(
                "Secret not found: {}",
                secret_id
            ))),
        }
    }

    async fn delete_secret(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
    ) -> Result<(), SecretError> {
        let key = Self::generate_key(user_id, function_id, secret_id);
        let mut secrets = self.secrets.write().await;

        if secrets.remove(&key).is_none() {
            return Err(SecretError::NotFound(format!(
                "Secret not found: {}",
                secret_id
            )));
        }

        Ok(())
    }

    async fn list_function_secrets(
        &self,
        user_id: &str,
        function_id: &str,
    ) -> Result<Vec<EncryptedSecret>, SecretError> {
        let secrets = self.secrets.read().await;
        let prefix = format!("{}:{}", user_id, function_id);

        let function_secrets = secrets
            .values()
            .filter(|s| s.user_id == user_id && s.function_id == function_id)
            .cloned()
            .collect();

        Ok(function_secrets)
    }
}
