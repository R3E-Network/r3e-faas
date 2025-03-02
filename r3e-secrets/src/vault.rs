// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::storage::SecretStorage;
use crate::{EncryptedSecret, SecretEncryption, SecretError};

/// Secret metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretMetadata {
    /// Secret ID
    pub id: String,

    /// User ID who owns the secret
    pub user_id: String,

    /// Function ID that can access the secret
    pub function_id: String,

    /// Secret name
    pub name: String,

    /// Secret description
    pub description: Option<String>,

    /// Secret tags
    pub tags: Vec<String>,

    /// Creation timestamp
    pub created_at: u64,

    /// Last updated timestamp
    pub updated_at: u64,

    /// Expiration timestamp (0 = never expires)
    pub expires_at: u64,

    /// Rotation period in seconds (0 = never rotates)
    pub rotation_period: u64,

    /// Last rotation timestamp
    pub last_rotated_at: u64,

    /// Version of the secret
    pub version: u32,

    /// Previous versions of the secret (limited history)
    pub previous_versions: Vec<String>,
}

impl SecretMetadata {
    /// Create new secret metadata
    pub fn new(
        user_id: String,
        function_id: String,
        name: String,
        description: Option<String>,
        tags: Vec<String>,
        expires_in: Option<u64>,
        rotation_period: Option<u64>,
    ) -> Self {
        let id = Uuid::new_v4().to_string();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let expires_at = expires_in.map(|e| now + e).unwrap_or(0);

        Self {
            id,
            user_id,
            function_id,
            name,
            description,
            tags,
            created_at: now,
            updated_at: now,
            expires_at,
            rotation_period: rotation_period.unwrap_or(0),
            last_rotated_at: now,
            version: 1,
            previous_versions: Vec::new(),
        }
    }

    /// Check if the secret is expired
    pub fn is_expired(&self) -> bool {
        if self.expires_at == 0 {
            return false;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        now > self.expires_at
    }

    /// Check if the secret needs rotation
    pub fn needs_rotation(&self) -> bool {
        if self.rotation_period == 0 {
            return false;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        now > self.last_rotated_at + self.rotation_period
    }

    /// Update the metadata for rotation
    pub fn rotate(&mut self, previous_version_id: String) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.last_rotated_at = now;
        self.updated_at = now;
        self.version += 1;

        // Keep only the last 5 versions
        self.previous_versions.push(previous_version_id);
        if self.previous_versions.len() > 5 {
            self.previous_versions.remove(0);
        }
    }
}

/// Secret vault for storing sensitive credentials
#[derive(Clone)]
pub struct SecretVault {
    /// Secret storage
    storage: Arc<dyn SecretStorage>,

    /// Metadata storage
    metadata: Arc<RwLock<HashMap<String, SecretMetadata>>>,

    /// Master key for the vault
    master_key: [u8; 32],

    /// Key rotation schedule in seconds (0 = never rotates)
    key_rotation_schedule: u64,

    /// Last key rotation timestamp
    last_key_rotation: Arc<RwLock<u64>>,
}

impl SecretVault {
    /// Create a new secret vault
    pub fn new(storage: Arc<dyn SecretStorage>, master_key: [u8; 32]) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            storage,
            metadata: Arc::new(RwLock::new(HashMap::new())),
            master_key,
            key_rotation_schedule: 30 * 24 * 60 * 60, // 30 days by default
            last_key_rotation: Arc::new(RwLock::new(now)),
        }
    }

    /// Generate a random master key
    pub fn generate_master_key() -> [u8; 32] {
        SecretEncryption::generate_function_key()
    }

    /// Set the key rotation schedule
    pub fn set_key_rotation_schedule(&mut self, rotation_period: u64) {
        self.key_rotation_schedule = rotation_period;
    }

    /// Check if the master key needs rotation
    pub async fn needs_key_rotation(&self) -> bool {
        if self.key_rotation_schedule == 0 {
            return false;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let last_rotation = *self.last_key_rotation.read().await;
        now > last_rotation + self.key_rotation_schedule
    }

    /// Rotate the master key
    pub async fn rotate_master_key(&mut self, new_master_key: [u8; 32]) -> Result<(), SecretError> {
        // Get all secrets
        let metadata = self.metadata.read().await;
        let mut rotated_secrets = Vec::new();

        // Re-encrypt all secrets with the new key
        for meta in metadata.values() {
            if meta.is_expired() {
                continue;
            }

            // Get the secret
            let secret = self
                .storage
                .get_secret(&meta.user_id, &meta.function_id, &meta.id)
                .await?;

            // Decrypt with old key
            let old_encryption = SecretEncryption::new(&self.master_key)?;
            let decrypted_data = old_encryption.decrypt(&secret.encrypted_data, &secret.nonce)?;

            // Encrypt with new key
            let new_encryption = SecretEncryption::new(&new_master_key)?;
            let (encrypted_data, nonce) = new_encryption.encrypt(&decrypted_data)?;

            // Create new encrypted secret
            let new_secret = EncryptedSecret::new(
                meta.user_id.clone(),
                meta.function_id.clone(),
                Some(meta.id.clone()),
                encrypted_data,
                nonce,
            );

            rotated_secrets.push(new_secret);
        }

        // Update the master key
        self.master_key = new_master_key;

        // Store all re-encrypted secrets
        for secret in rotated_secrets {
            self.storage.store_secret(secret).await?;
        }

        // Update the last rotation timestamp
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        *self.last_key_rotation.write().await = now;

        Ok(())
    }

    /// Store a secret
    pub async fn store_secret(
        &self,
        user_id: &str,
        function_id: &str,
        name: &str,
        value: &[u8],
        description: Option<String>,
        tags: Vec<String>,
        expires_in: Option<u64>,
        rotation_period: Option<u64>,
    ) -> Result<String, SecretError> {
        // Create encryption service
        let encryption = SecretEncryption::new(&self.master_key)?;

        // Encrypt data
        let (encrypted_data, nonce) = encryption.encrypt(value)?;

        // Create metadata
        let metadata = SecretMetadata::new(
            user_id.to_string(),
            function_id.to_string(),
            name.to_string(),
            description,
            tags,
            expires_in,
            rotation_period,
        );

        // Create encrypted secret
        let secret = EncryptedSecret::new(
            user_id.to_string(),
            function_id.to_string(),
            Some(metadata.id.clone()),
            encrypted_data,
            nonce,
        );

        // Store secret
        self.storage.store_secret(secret).await?;

        // Store metadata
        let mut metadata_map = self.metadata.write().await;
        metadata_map.insert(metadata.id.clone(), metadata.clone());

        Ok(metadata.id)
    }

    /// Get a secret
    pub async fn get_secret(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
    ) -> Result<(Vec<u8>, SecretMetadata), SecretError> {
        // Get metadata
        let metadata_map = self.metadata.read().await;
        let metadata = metadata_map
            .get(secret_id)
            .ok_or_else(|| SecretError::NotFound(format!("Secret not found: {}", secret_id)))?
            .clone();

        // Check if the secret is expired
        if metadata.is_expired() {
            return Err(SecretError::NotFound(format!(
                "Secret expired: {}",
                secret_id
            )));
        }

        // Check if the user has access
        if metadata.user_id != user_id || metadata.function_id != function_id {
            return Err(SecretError::Unauthorized(format!(
                "Unauthorized access to secret: {}",
                secret_id
            )));
        }

        // Get encrypted secret
        let secret = self
            .storage
            .get_secret(user_id, function_id, secret_id)
            .await?;

        // Create encryption service
        let encryption = SecretEncryption::new(&self.master_key)?;

        // Decrypt data
        let decrypted_data = encryption.decrypt(&secret.encrypted_data, &secret.nonce)?;

        // Check if the secret needs rotation
        if metadata.needs_rotation() {
            // Schedule rotation (in a real implementation, this would be done asynchronously)
            // For now, we'll just log it
            println!("Secret {} needs rotation", secret_id);
        }

        Ok((decrypted_data, metadata))
    }

    /// Rotate a secret
    pub async fn rotate_secret(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
        new_value: &[u8],
    ) -> Result<(), SecretError> {
        // Get metadata
        let mut metadata_map = self.metadata.write().await;
        let metadata = metadata_map
            .get_mut(secret_id)
            .ok_or_else(|| SecretError::NotFound(format!("Secret not found: {}", secret_id)))?;

        // Check if the user has access
        if metadata.user_id != user_id || metadata.function_id != function_id {
            return Err(SecretError::Unauthorized(format!(
                "Unauthorized access to secret: {}",
                secret_id
            )));
        }

        // Create encryption service
        let encryption = SecretEncryption::new(&self.master_key)?;

        // Encrypt new data
        let (encrypted_data, nonce) = encryption.encrypt(new_value)?;

        // Create a new version of the secret
        let previous_version_id = Uuid::new_v4().to_string();
        let previous_secret = EncryptedSecret::new(
            user_id.to_string(),
            function_id.to_string(),
            Some(previous_version_id.clone()),
            encrypted_data.clone(),
            nonce.clone(),
        );

        // Store the previous version
        self.storage.store_secret(previous_secret).await?;

        // Update the current secret
        let current_secret = EncryptedSecret::new(
            user_id.to_string(),
            function_id.to_string(),
            Some(secret_id.to_string()),
            encrypted_data,
            nonce,
        );

        // Store the updated secret
        self.storage.store_secret(current_secret).await?;

        // Update metadata
        metadata.rotate(previous_version_id);

        Ok(())
    }

    /// Delete a secret
    pub async fn delete_secret(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
    ) -> Result<(), SecretError> {
        // Get metadata
        let mut metadata_map = self.metadata.write().await;
        let metadata = metadata_map
            .get(secret_id)
            .ok_or_else(|| SecretError::NotFound(format!("Secret not found: {}", secret_id)))?;

        // Check if the user has access
        if metadata.user_id != user_id || metadata.function_id != function_id {
            return Err(SecretError::Unauthorized(format!(
                "Unauthorized access to secret: {}",
                secret_id
            )));
        }

        // Clone the previous_versions to avoid borrow issues
        let previous_versions = metadata.previous_versions.clone();

        // Delete the secret
        self.storage
            .delete_secret(user_id, function_id, secret_id)
            .await?;

        // Delete metadata
        metadata_map.remove(secret_id);

        // Delete previous versions
        for version_id in &previous_versions {
            // Ignore errors when deleting previous versions
            let _ = self
                .storage
                .delete_secret(user_id, function_id, version_id)
                .await;
        }

        Ok(())
    }

    /// List secrets for a function
    pub async fn list_secrets(
        &self,
        user_id: &str,
        function_id: &str,
    ) -> Result<Vec<SecretMetadata>, SecretError> {
        // Get metadata
        let metadata_map = self.metadata.read().await;

        // Filter metadata by user and function
        let function_metadata = metadata_map
            .values()
            .filter(|m| m.user_id == user_id && m.function_id == function_id && !m.is_expired())
            .cloned()
            .collect();

        Ok(function_metadata)
    }

    /// Get secret metadata
    pub async fn get_secret_metadata(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
    ) -> Result<SecretMetadata, SecretError> {
        // Get metadata
        let metadata_map = self.metadata.read().await;
        let metadata = metadata_map
            .get(secret_id)
            .ok_or_else(|| SecretError::NotFound(format!("Secret not found: {}", secret_id)))?
            .clone();

        // Check if the secret is expired
        if metadata.is_expired() {
            return Err(SecretError::NotFound(format!(
                "Secret expired: {}",
                secret_id
            )));
        }

        // Check if the user has access
        if metadata.user_id != user_id || metadata.function_id != function_id {
            return Err(SecretError::Unauthorized(format!(
                "Unauthorized access to secret: {}",
                secret_id
            )));
        }

        Ok(metadata)
    }

    /// Update secret metadata
    pub async fn update_secret_metadata(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
        name: Option<String>,
        description: Option<String>,
        tags: Option<Vec<String>>,
        expires_in: Option<u64>,
        rotation_period: Option<u64>,
    ) -> Result<(), SecretError> {
        // Get metadata
        let mut metadata_map = self.metadata.write().await;
        let metadata = metadata_map
            .get_mut(secret_id)
            .ok_or_else(|| SecretError::NotFound(format!("Secret not found: {}", secret_id)))?;

        // Check if the user has access
        if metadata.user_id != user_id || metadata.function_id != function_id {
            return Err(SecretError::Unauthorized(format!(
                "Unauthorized access to secret: {}",
                secret_id
            )));
        }

        // Update metadata
        if let Some(name) = name {
            metadata.name = name;
        }

        if let Some(description) = description {
            metadata.description = Some(description);
        }

        if let Some(tags) = tags {
            metadata.tags = tags;
        }

        if let Some(expires_in) = expires_in {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            metadata.expires_at = if expires_in == 0 { 0 } else { now + expires_in };
        }

        if let Some(rotation_period) = rotation_period {
            metadata.rotation_period = rotation_period;
        }

        // Update timestamp
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        metadata.updated_at = now;

        Ok(())
    }
}

/// Vault service trait
#[async_trait]
pub trait VaultService: Send + Sync {
    /// Store a secret
    async fn store_secret(
        &self,
        user_id: &str,
        function_id: &str,
        name: &str,
        value: &[u8],
        description: Option<String>,
        tags: Vec<String>,
        expires_in: Option<u64>,
        rotation_period: Option<u64>,
    ) -> Result<String, SecretError>;

    /// Get a secret
    async fn get_secret(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
    ) -> Result<Vec<u8>, SecretError>;

    /// Rotate a secret
    async fn rotate_secret(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
        new_value: &[u8],
    ) -> Result<(), SecretError>;

    /// Delete a secret
    async fn delete_secret(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
    ) -> Result<(), SecretError>;

    /// List secrets for a function
    async fn list_secrets(
        &self,
        user_id: &str,
        function_id: &str,
    ) -> Result<Vec<SecretMetadata>, SecretError>;

    /// Get secret metadata
    async fn get_secret_metadata(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
    ) -> Result<SecretMetadata, SecretError>;

    /// Update secret metadata
    async fn update_secret_metadata(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
        name: Option<String>,
        description: Option<String>,
        tags: Option<Vec<String>>,
        expires_in: Option<u64>,
        rotation_period: Option<u64>,
    ) -> Result<(), SecretError>;

    /// Check if the vault needs key rotation
    async fn needs_key_rotation(&self) -> bool;

    /// Rotate the vault master key
    async fn rotate_master_key(&self, new_master_key: [u8; 32]) -> Result<(), SecretError>;
}

#[async_trait]
impl VaultService for SecretVault {
    async fn store_secret(
        &self,
        user_id: &str,
        function_id: &str,
        name: &str,
        value: &[u8],
        description: Option<String>,
        tags: Vec<String>,
        expires_in: Option<u64>,
        rotation_period: Option<u64>,
    ) -> Result<String, SecretError> {
        self.store_secret(
            user_id,
            function_id,
            name,
            value,
            description,
            tags,
            expires_in,
            rotation_period,
        )
        .await
    }

    async fn get_secret(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
    ) -> Result<Vec<u8>, SecretError> {
        let (data, _) = self.get_secret(user_id, function_id, secret_id).await?;
        Ok(data)
    }

    async fn rotate_secret(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
        new_value: &[u8],
    ) -> Result<(), SecretError> {
        self.rotate_secret(user_id, function_id, secret_id, new_value)
            .await
    }

    async fn delete_secret(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
    ) -> Result<(), SecretError> {
        self.delete_secret(user_id, function_id, secret_id).await
    }

    async fn list_secrets(
        &self,
        user_id: &str,
        function_id: &str,
    ) -> Result<Vec<SecretMetadata>, SecretError> {
        self.list_secrets(user_id, function_id).await
    }

    async fn get_secret_metadata(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
    ) -> Result<SecretMetadata, SecretError> {
        self.get_secret_metadata(user_id, function_id, secret_id)
            .await
    }

    async fn update_secret_metadata(
        &self,
        user_id: &str,
        function_id: &str,
        secret_id: &str,
        name: Option<String>,
        description: Option<String>,
        tags: Option<Vec<String>>,
        expires_in: Option<u64>,
        rotation_period: Option<u64>,
    ) -> Result<(), SecretError> {
        self.update_secret_metadata(
            user_id,
            function_id,
            secret_id,
            name,
            description,
            tags,
            expires_in,
            rotation_period,
        )
        .await
    }

    async fn needs_key_rotation(&self) -> bool {
        self.needs_key_rotation().await
    }

    async fn rotate_master_key(&self, new_master_key: [u8; 32]) -> Result<(), SecretError> {
        // Use a regular variable instead of a mutable one since no mutation is happening
        let vault = self.clone();
        vault.rotate_master_key(new_master_key).await
    }
}
