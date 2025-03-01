// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use uuid::Uuid;

pub mod audit;
pub mod rocksdb;
pub mod service;
pub mod storage;
pub mod vault;

/// Error types for secret management
#[derive(Debug, Error)]
pub enum SecretError {
    #[error("Encryption error: {0}")]
    Encryption(String),

    #[error("Decryption error: {0}")]
    Decryption(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Secret not found: {0}")]
    NotFound(String),

    #[error("Unauthorized access: {0}")]
    Unauthorized(String),
}

/// Encrypted secret data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedSecret {
    /// Secret ID
    pub id: String,

    /// User ID who owns the secret
    pub user_id: String,

    /// Function ID that can access the secret
    pub function_id: String,

    /// Encrypted secret data
    pub encrypted_data: Vec<u8>,

    /// Nonce used for encryption
    pub nonce: Vec<u8>,

    /// Creation timestamp
    pub created_at: u64,

    /// Last updated timestamp
    pub updated_at: u64,
}

impl EncryptedSecret {
    /// Create a new encrypted secret
    pub fn new(
        user_id: String,
        function_id: String,
        secret_id: Option<String>,
        encrypted_data: Vec<u8>,
        nonce: Vec<u8>,
    ) -> Self {
        let id = secret_id.unwrap_or_else(|| Uuid::new_v4().to_string());
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id,
            user_id,
            function_id,
            encrypted_data,
            nonce,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Secret encryption service
pub struct SecretEncryption {
    /// Function-specific encryption key
    function_key: Arc<Key<Aes256Gcm>>,
}

impl SecretEncryption {
    /// Create a new secret encryption service with the given function key
    pub fn new(function_key: &[u8]) -> Result<Self, SecretError> {
        if function_key.len() != 32 {
            return Err(SecretError::Encryption("Invalid key length".to_string()));
        }

        let key = Key::<Aes256Gcm>::from_slice(function_key);

        Ok(Self {
            function_key: Arc::new(key.clone()),
        })
    }

    /// Generate a random function key
    pub fn generate_function_key() -> [u8; 32] {
        let mut key = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut key);
        key
    }

    /// Encrypt data
    pub fn encrypt(&self, data: &[u8]) -> Result<(Vec<u8>, Vec<u8>), SecretError> {
        // Generate a random nonce
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Create cipher
        let cipher = Aes256Gcm::new(&self.function_key);

        // Encrypt data
        let encrypted_data = cipher
            .encrypt(nonce, data)
            .map_err(|e| SecretError::Encryption(e.to_string()))?;

        Ok((encrypted_data, nonce_bytes.to_vec()))
    }

    /// Decrypt data
    pub fn decrypt(
        &self,
        encrypted_data: &[u8],
        nonce_bytes: &[u8],
    ) -> Result<Vec<u8>, SecretError> {
        // Create nonce
        let nonce = Nonce::from_slice(nonce_bytes);

        // Create cipher
        let cipher = Aes256Gcm::new(&self.function_key);

        // Decrypt data
        let decrypted_data = cipher
            .decrypt(nonce, encrypted_data)
            .map_err(|e| SecretError::Decryption(e.to_string()))?;

        Ok(decrypted_data)
    }
}
