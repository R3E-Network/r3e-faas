// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use tracing::{debug, error, info, warn};

use crate::error::Error;
use r3e_secrets::{SecretEncryption, SecretError};
use r3e_secrets::service::SecretService;

/// API key with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    /// Key ID
    pub id: String,
    
    /// User ID
    pub user_id: String,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Expiration timestamp
    pub expires_at: DateTime<Utc>,
    
    /// Previous key ID (for rotation)
    pub previous_key_id: Option<String>,
    
    /// Rotation count
    pub rotation_count: u32,
}

/// API key rotation service
pub struct KeyRotationService {
    /// Secret service for storing API keys
    secret_service: Arc<dyn SecretService>,
    
    /// API key metadata
    api_keys: Arc<RwLock<HashMap<String, ApiKey>>>,
    
    /// Default key expiration period in days
    default_expiration_days: i64,
    
    /// Default rotation period in days (when to trigger rotation)
    default_rotation_days: i64,
}

impl KeyRotationService {
    /// Create a new key rotation service
    pub fn new(secret_service: Arc<dyn SecretService>) -> Self {
        Self {
            secret_service,
            api_keys: Arc::new(RwLock::new(HashMap::new())),
            default_expiration_days: 30,
            default_rotation_days: 15,
        }
    }
    
    /// Set default expiration period
    pub fn set_default_expiration(&mut self, days: i64) {
        self.default_expiration_days = days;
    }
    
    /// Set default rotation period
    pub fn set_default_rotation(&mut self, days: i64) {
        self.default_rotation_days = days;
    }
    
    /// Create a new API key
    pub async fn create_key(&self, user_id: &str) -> Result<(String, ApiKey), Error> {
        // Generate a new key
        let key_id = Uuid::new_v4().to_string();
        let key_value = Uuid::new_v4().to_string();
        
        // Generate a function key for encryption
        let function_key = SecretEncryption::generate_function_key();
        
        // Store the key in the secret service
        self.secret_service
            .store_secret(
                user_id,
                "api_keys",
                &key_id,
                key_value.as_bytes(),
                &function_key,
            )
            .await
            .map_err(|e| Error::Internal(format!("Failed to store API key: {}", e)))?;
        
        // Create API key metadata
        let now = Utc::now();
        let api_key = ApiKey {
            id: key_id.clone(),
            user_id: user_id.to_string(),
            created_at: now,
            expires_at: now + Duration::days(self.default_expiration_days),
            previous_key_id: None,
            rotation_count: 0,
        };
        
        // Store API key metadata
        let mut guard = self.api_keys.write().unwrap();
        guard.insert(key_id.clone(), api_key.clone());
        
        info!(
            "Created API key: id={}, user_id={}, expires_at={}",
            key_id, user_id, api_key.expires_at
        );
        
        Ok((key_value, api_key))
    }
    
    /// Rotate an API key
    pub async fn rotate_key(&self, key_id: &str, user_id: &str) -> Result<(String, ApiKey), Error> {
        // Get the current key metadata
        let current_key = {
            let guard = self.api_keys.read().unwrap();
            guard.get(key_id).cloned().ok_or_else(|| {
                Error::NotFound(format!("API key not found: {}", key_id))
            })?
        };
        
        // Verify the user owns this key
        if current_key.user_id != user_id {
            return Err(Error::Unauthorized("Not authorized to rotate this key".into()));
        }
        
        // Generate a new key
        let new_key_id = Uuid::new_v4().to_string();
        let new_key_value = Uuid::new_v4().to_string();
        
        // Generate a function key for encryption
        let function_key = SecretEncryption::generate_function_key();
        
        // Store the new key in the secret service
        self.secret_service
            .store_secret(
                user_id,
                "api_keys",
                &new_key_id,
                new_key_value.as_bytes(),
                &function_key,
            )
            .await
            .map_err(|e| Error::Internal(format!("Failed to store API key: {}", e)))?;
        
        // Create new API key metadata
        let now = Utc::now();
        let new_api_key = ApiKey {
            id: new_key_id.clone(),
            user_id: user_id.to_string(),
            created_at: now,
            expires_at: now + Duration::days(self.default_expiration_days),
            previous_key_id: Some(key_id.to_string()),
            rotation_count: current_key.rotation_count + 1,
        };
        
        // Store new API key metadata
        let mut guard = self.api_keys.write().unwrap();
        guard.insert(new_key_id.clone(), new_api_key.clone());
        
        // Keep the old key valid for a grace period (1 day)
        let grace_period = Duration::days(1);
        
        // Update old key metadata
        let mut old_key = current_key.clone();
        old_key.expires_at = now + grace_period;
        guard.insert(key_id.to_string(), old_key);
        
        info!(
            "Rotated API key: old_id={}, new_id={}, user_id={}, expires_at={}",
            key_id, new_key_id, user_id, new_api_key.expires_at
        );
        
        Ok((new_key_value, new_api_key))
    }
    
    /// Validate an API key
    pub async fn validate_key(&self, key_id: &str, key_value: &str, user_id: &str) -> Result<bool, Error> {
        // Get the key metadata
        let api_key = {
            let guard = self.api_keys.read().unwrap();
            guard.get(key_id).cloned().ok_or_else(|| {
                Error::NotFound(format!("API key not found: {}", key_id))
            })?
        };
        
        // Check if the key belongs to the user
        if api_key.user_id != user_id {
            return Err(Error::Unauthorized("Not authorized to access this key".into()));
        }
        
        // Check if the key has expired
        if api_key.expires_at < Utc::now() {
            return Ok(false);
        }
        
        // Generate a function key for encryption (this would normally be retrieved from a secure store)
        let function_key = SecretEncryption::generate_function_key();
        
        // Get the stored key value
        let stored_value = match self.secret_service
            .get_secret(user_id, "api_keys", key_id, &function_key)
            .await {
                Ok(value) => String::from_utf8_lossy(&value).to_string(),
                Err(SecretError::NotFound(_)) => return Ok(false),
                Err(e) => return Err(Error::Internal(format!("Failed to get API key: {}", e))),
            };
        
        // Compare the key values
        Ok(stored_value == key_value)
    }
    
    /// Revoke an API key
    pub async fn revoke_key(&self, key_id: &str, user_id: &str) -> Result<(), Error> {
        // Get the key metadata
        let api_key = {
            let guard = self.api_keys.read().unwrap();
            guard.get(key_id).cloned().ok_or_else(|| {
                Error::NotFound(format!("API key not found: {}", key_id))
            })?
        };
        
        // Check if the key belongs to the user
        if api_key.user_id != user_id {
            return Err(Error::Unauthorized("Not authorized to revoke this key".into()));
        }
        
        // Delete the key from the secret service
        self.secret_service
            .delete_secret(user_id, "api_keys", key_id)
            .await
            .map_err(|e| Error::Internal(format!("Failed to delete API key: {}", e)))?;
        
        // Remove the key metadata
        let mut guard = self.api_keys.write().unwrap();
        guard.remove(key_id);
        
        info!("Revoked API key: id={}, user_id={}", key_id, user_id);
        
        Ok(())
    }
    
    /// Check if a key needs rotation
    pub fn needs_rotation(&self, key_id: &str) -> Result<bool, Error> {
        let guard = self.api_keys.read().unwrap();
        let api_key = guard.get(key_id).ok_or_else(|| {
            Error::NotFound(format!("API key not found: {}", key_id))
        })?;
        
        // Calculate the rotation threshold
        let rotation_threshold = api_key.created_at + Duration::days(self.default_rotation_days);
        
        // Check if we've passed the rotation threshold
        Ok(Utc::now() > rotation_threshold)
    }
    
    /// Get all keys for a user
    pub fn get_user_keys(&self, user_id: &str) -> Vec<ApiKey> {
        let guard = self.api_keys.read().unwrap();
        guard.values()
            .filter(|key| key.user_id == user_id)
            .cloned()
            .collect()
    }
    
    /// Load keys from storage
    pub async fn load_keys(&self, user_id: &str) -> Result<(), Error> {
        // Get all secret IDs for the user's API keys
        let secret_ids = self.secret_service
            .list_secret_ids(user_id, "api_keys")
            .await
            .map_err(|e| Error::Internal(format!("Failed to list API keys: {}", e)))?;
        
        // For a real implementation, we would also load the metadata for each key
        // Here we're just creating placeholder metadata
        let now = Utc::now();
        let mut guard = self.api_keys.write().unwrap();
        
        for key_id in secret_ids {
            // Create placeholder metadata
            let api_key = ApiKey {
                id: key_id.clone(),
                user_id: user_id.to_string(),
                created_at: now - Duration::days(1), // Assume created yesterday
                expires_at: now + Duration::days(self.default_expiration_days - 1),
                previous_key_id: None,
                rotation_count: 0,
            };
            
            guard.insert(key_id, api_key);
        }
        
        Ok(())
    }
}
