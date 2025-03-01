// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_trait::async_trait;
use std::sync::Arc;

use crate::storage::SecretStorage;
use crate::{EncryptedSecret, SecretEncryption, SecretError};

/// Secret service trait
#[async_trait]
pub trait SecretService: Send + Sync {
    /// Store a secret
    async fn store_secret(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
        data: &[u8],
        function_key: &[u8],
    ) -> Result<(), SecretError>;

    /// Get a secret
    async fn get_secret(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
        function_key: &[u8],
    ) -> Result<Vec<u8>, SecretError>;

    /// Delete a secret
    async fn delete_secret(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
    ) -> Result<(), SecretError>;

    /// List secret IDs for a function
    async fn list_secret_ids(
        &self,
        user_id: &str,
        function_id: &str,
    ) -> Result<Vec<String>, SecretError>;
}

/// Secret service implementation
pub struct SecretServiceImpl {
    storage: Arc<dyn SecretStorage>,
}

impl SecretServiceImpl {
    /// Create a new secret service
    pub fn new(storage: Arc<dyn SecretStorage>) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl SecretService for SecretServiceImpl {
    async fn store_secret(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
        data: &[u8],
        function_key: &[u8],
    ) -> Result<(), SecretError> {
        // Create encryption service
        let encryption = SecretEncryption::new(function_key)?;

        // Encrypt data
        let (encrypted_data, nonce) = encryption.encrypt(data)?;

        // Create encrypted secret
        let secret = EncryptedSecret::new(
            user_id.to_string(),
            function_id.to_string(),
            Some(secret_id.to_string()),
            encrypted_data,
            nonce,
        );

        // Store secret
        self.storage.store_secret(secret).await
    }

    async fn get_secret(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
        function_key: &[u8],
    ) -> Result<Vec<u8>, SecretError> {
        // Get encrypted secret
        let secret = self
            .storage
            .get_secret(user_id, function_id, secret_id)
            .await?;

        // Create encryption service
        let encryption = SecretEncryption::new(function_key)?;

        // Decrypt data
        let decrypted_data = encryption.decrypt(&secret.encrypted_data, &secret.nonce)?;

        Ok(decrypted_data)
    }

    async fn delete_secret(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
    ) -> Result<(), SecretError> {
        self.storage
            .delete_secret(user_id, function_id, secret_id)
            .await
    }

    async fn list_secret_ids(
        &self,
        user_id: &str,
        function_id: &str,
    ) -> Result<Vec<String>, SecretError> {
        let secrets = self
            .storage
            .list_function_secrets(user_id, function_id)
            .await?;
        let secret_ids = secrets.iter().map(|s| s.id.clone()).collect();
        Ok(secret_ids)
    }
}
