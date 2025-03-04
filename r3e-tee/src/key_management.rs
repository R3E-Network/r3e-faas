// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use crate::types::{KeyMetadata, KeyType, KeyUsage};
use crate::TeeError;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

/// Key management service trait
#[async_trait::async_trait]
pub trait KeyManagementService: Send + Sync {
    /// Generate a new key
    async fn generate_key(
        &self,
        key_type: KeyType,
        usage: Vec<KeyUsage>,
        algorithm: &str,
        size: u32,
        exportable: bool,
    ) -> Result<KeyMetadata, TeeError>;

    /// Import a key
    async fn import_key(
        &self,
        key_data: &[u8],
        key_type: KeyType,
        usage: Vec<KeyUsage>,
        algorithm: &str,
        exportable: bool,
    ) -> Result<KeyMetadata, TeeError>;

    /// Export a key (if exportable)
    async fn export_key(&self, key_id: &str) -> Result<Vec<u8>, TeeError>;

    /// Delete a key
    async fn delete_key(&self, key_id: &str) -> Result<bool, TeeError>;

    /// Get key metadata
    async fn get_key_metadata(&self, key_id: &str) -> Result<KeyMetadata, TeeError>;

    /// List all keys
    async fn list_keys(&self) -> Result<Vec<KeyMetadata>, TeeError>;

    /// Encrypt data using a key
    async fn encrypt(
        &self,
        key_id: &str,
        data: &[u8],
        iv: Option<&[u8]>,
    ) -> Result<Vec<u8>, TeeError>;

    /// Decrypt data using a key
    async fn decrypt(
        &self,
        key_id: &str,
        data: &[u8],
        iv: Option<&[u8]>,
    ) -> Result<Vec<u8>, TeeError>;

    /// Sign data using a key
    async fn sign(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, TeeError>;

    /// Verify a signature using a key
    async fn verify(&self, key_id: &str, data: &[u8], signature: &[u8]) -> Result<bool, TeeError>;

    /// Wrap a key using another key
    async fn wrap_key(&self, wrapping_key_id: &str, key_id: &str) -> Result<Vec<u8>, TeeError>;

    /// Unwrap a key using another key
    async fn unwrap_key(
        &self,
        unwrapping_key_id: &str,
        wrapped_key: &[u8],
        key_type: KeyType,
        usage: Vec<KeyUsage>,
        algorithm: &str,
        exportable: bool,
    ) -> Result<KeyMetadata, TeeError>;
}

/// In-memory key storage
#[derive(Default)]
pub struct InMemoryKeyStorage {
    /// Key metadata
    metadata: RwLock<HashMap<String, KeyMetadata>>,

    /// Key data
    key_data: RwLock<HashMap<String, Vec<u8>>>,
}

impl InMemoryKeyStorage {
    /// Create a new in-memory key storage
    pub fn new() -> Self {
        Self {
            metadata: RwLock::new(HashMap::new()),
            key_data: RwLock::new(HashMap::new()),
        }
    }

    /// Store a key
    pub fn store_key(&self, metadata: KeyMetadata, key_data: Vec<u8>) -> Result<(), TeeError> {
        let key_id = metadata.id.clone();

        // Store metadata
        {
            let mut metadata_map = self.metadata.write().map_err(|e| {
                TeeError::KeyManagement(format!("Failed to acquire metadata write lock: {}", e))
            })?;

            metadata_map.insert(key_id.clone(), metadata);
        }

        // Store key data
        {
            let mut key_data_map = self.key_data.write().map_err(|e| {
                TeeError::KeyManagement(format!("Failed to acquire key data write lock: {}", e))
            })?;

            key_data_map.insert(key_id, key_data);
        }

        Ok(())
    }

    /// Get key metadata
    pub fn get_metadata(&self, key_id: &str) -> Result<KeyMetadata, TeeError> {
        let metadata_map = self.metadata.read().map_err(|e| {
            TeeError::KeyManagement(format!("Failed to acquire metadata read lock: {}", e))
        })?;

        metadata_map
            .get(key_id)
            .cloned()
            .ok_or_else(|| TeeError::KeyManagement(format!("Key not found: {}", key_id)))
    }

    /// Get key data
    pub fn get_key_data(&self, key_id: &str) -> Result<Vec<u8>, TeeError> {
        let key_data_map = self.key_data.read().map_err(|e| {
            TeeError::KeyManagement(format!("Failed to acquire key data read lock: {}", e))
        })?;

        key_data_map
            .get(key_id)
            .cloned()
            .ok_or_else(|| TeeError::KeyManagement(format!("Key data not found: {}", key_id)))
    }

    /// Delete a key
    pub fn delete_key(&self, key_id: &str) -> Result<bool, TeeError> {
        // Delete metadata
        let metadata_removed = {
            let mut metadata_map = self.metadata.write().map_err(|e| {
                TeeError::KeyManagement(format!("Failed to acquire metadata write lock: {}", e))
            })?;

            metadata_map.remove(key_id).is_some()
        };

        // Delete key data
        let key_data_removed = {
            let mut key_data_map = self.key_data.write().map_err(|e| {
                TeeError::KeyManagement(format!("Failed to acquire key data write lock: {}", e))
            })?;

            key_data_map.remove(key_id).is_some()
        };

        Ok(metadata_removed && key_data_removed)
    }

    /// List all keys
    pub fn list_keys(&self) -> Result<Vec<KeyMetadata>, TeeError> {
        let metadata_map = self.metadata.read().map_err(|e| {
            TeeError::KeyManagement(format!("Failed to acquire metadata read lock: {}", e))
        })?;

        Ok(metadata_map.values().cloned().collect())
    }
}

/// Key management service implementation
pub struct KeyManagementServiceImpl {
    /// Key storage
    storage: Arc<InMemoryKeyStorage>,
    /// Is initialized
    initialized: AtomicBool,
}

impl KeyManagementServiceImpl {
    /// Create a new key management service
    pub fn new() -> Self {
        Self {
            storage: Arc::new(InMemoryKeyStorage::new()),
            initialized: AtomicBool::new(false),
        }
    }

    /// Initialize the key management service
    pub async fn initialize(&self) -> Result<(), TeeError> {
        // Set initialized to true
        self.initialized.store(true, Ordering::SeqCst);

        // Log initialization
        info!("Key management service initialized");

        Ok(())
    }

    /// Check if the key management service is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::SeqCst)
    }

    /// Generate a random key ID
    fn generate_key_id() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let key_id: u128 = rng.gen();
        format!("key-{:032x}", key_id)
    }

    /// Get current timestamp
    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// Generate a random key
    fn generate_random_key(algorithm: &str, size: u32) -> Result<Vec<u8>, TeeError> {
        use rand::RngCore;

        let key_size_bytes = match algorithm {
            "AES" => size as usize / 8,
            "RSA" => size as usize / 8,
            "EC" => match size {
                256 => 32,
                384 => 48,
                521 => 66,
                _ => {
                    return Err(TeeError::KeyManagement(format!(
                        "Unsupported EC key size: {}",
                        size
                    )))
                }
            },
            _ => {
                return Err(TeeError::KeyManagement(format!(
                    "Unsupported algorithm: {}",
                    algorithm
                )))
            }
        };

        let mut key_data = vec![0u8; key_size_bytes];
        rand::thread_rng().fill_bytes(&mut key_data);

        Ok(key_data)
    }
}

#[async_trait::async_trait]
impl KeyManagementService for KeyManagementServiceImpl {
    async fn generate_key(
        &self,
        key_type: KeyType,
        usage: Vec<KeyUsage>,
        algorithm: &str,
        size: u32,
        exportable: bool,
    ) -> Result<KeyMetadata, TeeError> {
        // Generate a random key ID
        let key_id = Self::generate_key_id();

        // Generate key data
        let key_data = Self::generate_random_key(algorithm, size)?;

        // Create key metadata
        let metadata = KeyMetadata {
            id: key_id,
            key_type,
            usage,
            algorithm: algorithm.to_string(),
            size,
            created_at: Self::current_timestamp(),
            expires_at: None,
            exportable,
        };

        // Store the key
        self.storage.store_key(metadata.clone(), key_data)?;

        Ok(metadata)
    }

    async fn import_key(
        &self,
        key_data: &[u8],
        key_type: KeyType,
        usage: Vec<KeyUsage>,
        algorithm: &str,
        exportable: bool,
    ) -> Result<KeyMetadata, TeeError> {
        // Generate a random key ID
        let key_id = Self::generate_key_id();

        // Calculate key size in bits
        let size = match algorithm {
            "AES" => (key_data.len() * 8) as u32,
            "RSA" => (key_data.len() * 8) as u32,
            "EC" => match key_data.len() {
                32 => 256,
                48 => 384,
                66 => 521,
                _ => {
                    return Err(TeeError::KeyManagement(format!(
                        "Invalid EC key size: {} bytes",
                        key_data.len()
                    )))
                }
            },
            _ => {
                return Err(TeeError::KeyManagement(format!(
                    "Unsupported algorithm: {}",
                    algorithm
                )))
            }
        };

        // Create key metadata
        let metadata = KeyMetadata {
            id: key_id,
            key_type,
            usage,
            algorithm: algorithm.to_string(),
            size,
            created_at: Self::current_timestamp(),
            expires_at: None,
            exportable,
        };

        // Store the key
        self.storage
            .store_key(metadata.clone(), key_data.to_vec())?;

        Ok(metadata)
    }

    async fn export_key(&self, key_id: &str) -> Result<Vec<u8>, TeeError> {
        // Get key metadata
        let metadata = self.storage.get_metadata(key_id)?;

        // Check if the key is exportable
        if !metadata.exportable {
            return Err(TeeError::KeyManagement(format!(
                "Key is not exportable: {}",
                key_id
            )));
        }

        // Get key data
        self.storage.get_key_data(key_id)
    }

    async fn delete_key(&self, key_id: &str) -> Result<bool, TeeError> {
        self.storage.delete_key(key_id)
    }

    async fn get_key_metadata(&self, key_id: &str) -> Result<KeyMetadata, TeeError> {
        self.storage.get_metadata(key_id)
    }

    async fn list_keys(&self) -> Result<Vec<KeyMetadata>, TeeError> {
        self.storage.list_keys()
    }

    async fn encrypt(
        &self,
        key_id: &str,
        data: &[u8],
        iv: Option<&[u8]>,
    ) -> Result<Vec<u8>, TeeError> {
        // Get key metadata
        let metadata = self.storage.get_metadata(key_id)?;

        // Check if the key can be used for encryption
        if !metadata.usage.contains(&KeyUsage::Encryption) {
            return Err(TeeError::KeyManagement(format!(
                "Key cannot be used for encryption: {}",
                key_id
            )));
        }

        // Get key data
        let key_data = self.storage.get_key_data(key_id)?;

        // Perform encryption based on the algorithm
        match metadata.algorithm.as_str() {
            "AES" => {
                use aes::cipher::generic_array::GenericArray;
                use aes::cipher::{BlockEncrypt, KeyInit};
                use aes::Aes256;

                // Create AES cipher
                let cipher = Aes256::new(GenericArray::from_slice(&key_data));

                // Generate IV if not provided
                let iv = if let Some(iv_data) = iv {
                    if iv_data.len() != 16 {
                        return Err(TeeError::KeyManagement("Invalid IV length".to_string()));
                    }
                    iv_data.to_vec()
                } else {
                    let mut iv = vec![0u8; 16];
                    rand::thread_rng().fill_bytes(&mut iv);
                    iv
                };

                // Pad data to block size
                let block_size = 16;
                let padding_len = block_size - (data.len() % block_size);
                let mut padded_data = data.to_vec();
                padded_data.extend(vec![padding_len as u8; padding_len]);

                // Encrypt data in blocks
                let mut result = iv.clone();
                let mut prev_block = GenericArray::from_slice(&iv);

                for chunk in padded_data.chunks(16) {
                    let mut block = GenericArray::clone_from_slice(chunk);
                    for (a, b) in block.iter_mut().zip(prev_block.iter()) {
                        *a ^= b;
                    }
                    cipher.encrypt_block(&mut block);
                    result.extend_from_slice(&block);
                    prev_block = block;
                }

                Ok(result)
            }
            _ => Err(TeeError::KeyManagement(format!(
                "Encryption not implemented for algorithm: {}",
                metadata.algorithm
            ))),
        }
    }

    async fn decrypt(
        &self,
        key_id: &str,
        data: &[u8],
        iv: Option<&[u8]>,
    ) -> Result<Vec<u8>, TeeError> {
        // Get key metadata
        let metadata = self.storage.get_metadata(key_id)?;

        // Check if the key can be used for decryption
        if !metadata.usage.contains(&KeyUsage::Decryption) {
            return Err(TeeError::KeyManagement(format!(
                "Key cannot be used for decryption: {}",
                key_id
            )));
        }

        // Get key data
        let key_data = self.storage.get_key_data(key_id)?;

        // Perform decryption based on the algorithm
        match metadata.algorithm.as_str() {
            "AES" => {
                use aes::cipher::generic_array::GenericArray;
                use aes::cipher::{BlockDecrypt, KeyInit};
                use aes::Aes256;

                // Create AES cipher
                let cipher = Aes256::new(GenericArray::from_slice(&key_data));

                // Get IV and ciphertext
                let (iv, ciphertext) = if let Some(iv_data) = iv {
                    if iv_data.len() != 16 {
                        return Err(TeeError::KeyManagement("Invalid IV length".to_string()));
                    }
                    (iv_data, data)
                } else {
                    if data.len() < 16 {
                        return Err(TeeError::KeyManagement("Data too short".to_string()));
                    }
                    (&data[..16], &data[16..])
                };

                // Decrypt data in blocks
                let mut result = Vec::new();
                let mut prev_block = GenericArray::from_slice(iv);

                for chunk in ciphertext.chunks(16) {
                    let mut block = GenericArray::clone_from_slice(chunk);
                    let saved_block = block.clone();
                    cipher.decrypt_block(&mut block);
                    for (a, b) in block.iter_mut().zip(prev_block.iter()) {
                        *a ^= b;
                    }
                    result.extend_from_slice(&block);
                    prev_block = saved_block;
                }

                // Remove padding
                let padding_len = *result.last().unwrap_or(&0) as usize;
                if padding_len <= 16 && padding_len <= result.len() {
                    result.truncate(result.len() - padding_len);
                }

                Ok(result)
            }
            _ => Err(TeeError::KeyManagement(format!(
                "Decryption not implemented for algorithm: {}",
                metadata.algorithm
            ))),
        }
    }

    async fn sign(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, TeeError> {
        // Get key metadata
        let metadata = self.storage.get_metadata(key_id)?;

        // Check if the key can be used for signing
        if !metadata.usage.contains(&KeyUsage::Signing) {
            return Err(TeeError::KeyManagement(format!(
                "Key cannot be used for signing: {}",
                key_id
            )));
        }

        // Get key data
        let key_data = self.storage.get_key_data(key_id)?;

        // Perform signing based on the algorithm
        match metadata.algorithm.as_str() {
            "HMAC" => {
                use hmac::{Hmac, Mac};
                use sha2::Sha256;

                // Create HMAC instance
                let mut mac = Hmac::<Sha256>::new_from_slice(&key_data).map_err(|e| {
                    TeeError::KeyManagement(format!("Failed to create HMAC: {}", e))
                })?;

                // Update with data
                mac.update(data);

                // Finalize and get result
                let result = mac.finalize().into_bytes();

                Ok(result.to_vec())
            }
            _ => Err(TeeError::KeyManagement(format!(
                "Signing not implemented for algorithm: {}",
                metadata.algorithm
            ))),
        }
    }

    async fn verify(&self, key_id: &str, data: &[u8], signature: &[u8]) -> Result<bool, TeeError> {
        // Get key metadata
        let metadata = self.storage.get_metadata(key_id)?;

        // Check if the key can be used for verification
        if !metadata.usage.contains(&KeyUsage::Verification) {
            return Err(TeeError::KeyManagement(format!(
                "Key cannot be used for verification: {}",
                key_id
            )));
        }

        // Get key data
        let key_data = self.storage.get_key_data(key_id)?;

        // Perform verification based on the algorithm
        match metadata.algorithm.as_str() {
            "HMAC" => {
                use hmac::{Hmac, Mac};
                use sha2::Sha256;

                // Create HMAC instance
                let mut mac = Hmac::<Sha256>::new_from_slice(&key_data).map_err(|e| {
                    TeeError::KeyManagement(format!("Failed to create HMAC: {}", e))
                })?;

                // Update with data
                mac.update(data);

                // Verify signature
                mac.verify_slice(signature)
                    .map(|_| true)
                    .map_err(|_| TeeError::KeyManagement("Invalid signature".to_string()))
            }
            _ => Err(TeeError::KeyManagement(format!(
                "Verification not implemented for algorithm: {}",
                metadata.algorithm
            ))),
        }
    }

    async fn wrap_key(&self, wrapping_key_id: &str, key_id: &str) -> Result<Vec<u8>, TeeError> {
        // Get wrapping key metadata
        let wrapping_metadata = self.storage.get_metadata(wrapping_key_id)?;

        // Check if the wrapping key can be used for key wrapping
        if !wrapping_metadata.usage.contains(&KeyUsage::KeyWrapping) {
            return Err(TeeError::KeyManagement(format!(
                "Key cannot be used for key wrapping: {}",
                wrapping_key_id
            )));
        }

        // Get key to be wrapped
        let key_data = self.storage.get_key_data(key_id)?;

        // Get wrapping key
        let wrapping_key_data = self.storage.get_key_data(wrapping_key_id)?;

        // Perform key wrapping based on the algorithm
        match wrapping_metadata.algorithm.as_str() {
            "AES" => {
                use aes::cipher::generic_array::GenericArray;
                use aes::cipher::{BlockEncrypt, KeyInit};
                use aes::Aes256;

                // Create AES cipher for key wrapping
                let cipher = Aes256::new(GenericArray::from_slice(&wrapping_key_data));

                // Implement AES Key Wrap (RFC 3394)
                let mut result = vec![0xA6; 8]; // Initial Value (IV)
                result.extend_from_slice(&key_data);

                // Pad key data to multiple of 8 bytes if needed
                let padding_len = (8 - (result.len() % 8)) % 8;
                result.extend(vec![0u8; padding_len]);

                // Perform key wrapping
                let n = (result.len() - 8) / 8;
                let mut blocks: Vec<_> = result.chunks(8).map(|c| c.to_vec()).collect();

                for j in 0..6 {
                    for i in 0..n {
                        let mut t = blocks[0].clone();
                        t.extend(&blocks[i + 1]);

                        // Encrypt block
                        let mut block = GenericArray::from_slice(&t);
                        cipher.encrypt_block(&mut block);

                        // Update blocks
                        let t = block.to_vec();
                        blocks[0] = t[..8].to_vec();
                        blocks[i + 1] = t[8..].to_vec();

                        // XOR with counter
                        let counter = ((n * j) + i + 1) as u64;
                        for k in 0..8 {
                            blocks[0][k] ^= ((counter >> (56 - (k * 8))) & 0xFF) as u8;
                        }
                    }
                }

                // Combine blocks
                result = blocks.into_iter().flatten().collect();
                for (i, byte) in key_data.iter().enumerate() {
                    result.push(byte ^ wrapping_key_data[i % wrapping_key_data.len()]);
                }

                Ok(result)
            }
            _ => Err(TeeError::KeyManagement(format!(
                "Key wrapping not implemented for algorithm: {}",
                wrapping_metadata.algorithm
            ))),
        }
    }

    async fn unwrap_key(
        &self,
        unwrapping_key_id: &str,
        wrapped_key: &[u8],
        key_type: KeyType,
        usage: Vec<KeyUsage>,
        algorithm: &str,
        exportable: bool,
    ) -> Result<KeyMetadata, TeeError> {
        // Get unwrapping key metadata
        let unwrapping_metadata = self.storage.get_metadata(unwrapping_key_id)?;

        // Check if the unwrapping key can be used for key unwrapping
        if !unwrapping_metadata.usage.contains(&KeyUsage::KeyUnwrapping) {
            return Err(TeeError::KeyManagement(format!(
                "Key cannot be used for key unwrapping: {}",
                unwrapping_key_id
            )));
        }

        // Get unwrapping key
        let unwrapping_key_data = self.storage.get_key_data(unwrapping_key_id)?;

        // Perform key unwrapping based on the algorithm
        let unwrapped_key = match unwrapping_metadata.algorithm.as_str() {
            "AES" => {
                use aes::cipher::generic_array::GenericArray;
                use aes::cipher::{BlockDecrypt, KeyInit};
                use aes::Aes256;

                // Create AES cipher for key unwrapping
                let cipher = Aes256::new(GenericArray::from_slice(&unwrapping_key_data));

                // Implement AES Key Unwrap (RFC 3394)
                if wrapped_key.len() < 24 || wrapped_key.len() % 8 != 0 {
                    return Err(TeeError::KeyManagement(
                        "Invalid wrapped key length".to_string(),
                    ));
                }

                let n = (wrapped_key.len() - 8) / 8;
                let mut blocks: Vec<_> = wrapped_key.chunks(8).map(|c| c.to_vec()).collect();

                // Perform key unwrapping
                for j in (0..6).rev() {
                    for i in (0..n).rev() {
                        // XOR with counter
                        let counter = ((n * j) + i + 1) as u64;
                        for k in 0..8 {
                            blocks[0][k] ^= ((counter >> (56 - (k * 8))) & 0xFF) as u8;
                        }

                        // Prepare block for decryption
                        let mut t = blocks[0].clone();
                        t.extend(&blocks[i + 1]);

                        // Decrypt block
                        let mut block = GenericArray::from_slice(&t);
                        cipher.decrypt_block(&mut block);

                        // Update blocks
                        let t = block.to_vec();
                        blocks[0] = t[..8].to_vec();
                        blocks[i + 1] = t[8..].to_vec();
                    }
                }

                // Verify initial value
                if blocks[0] != vec![0xA6; 8] {
                    return Err(TeeError::KeyManagement(
                        "Key unwrap verification failed".to_string(),
                    ));
                }

                // Remove IV and combine remaining blocks
                blocks.remove(0);
                let mut result: Vec<u8> = blocks.into_iter().flatten().collect();
                for (i, byte) in wrapped_key.iter().enumerate() {
                    result.push(byte ^ unwrapping_key_data[i % unwrapping_key_data.len()]);
                }

                result
            }
            _ => {
                return Err(TeeError::KeyManagement(format!(
                    "Key unwrapping not implemented for algorithm: {}",
                    unwrapping_metadata.algorithm
                )))
            }
        };

        // Import the unwrapped key
        self.import_key(&unwrapped_key, key_type, usage, algorithm, exportable)
            .await
    }
}
