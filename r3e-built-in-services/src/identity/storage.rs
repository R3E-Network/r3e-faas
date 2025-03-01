// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use crate::identity::types::{IdentityError, IdentityProfile};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Trait defining the identity storage functionality
#[async_trait]
pub trait IdentityStorage: Send + Sync {
    /// Create a new identity
    async fn create_identity(&self, profile: IdentityProfile) -> Result<(), IdentityError>;

    /// Get an identity by DID
    async fn get_identity(&self, did: &str) -> Result<IdentityProfile, IdentityError>;

    /// Update an identity
    async fn update_identity(&self, profile: IdentityProfile) -> Result<(), IdentityError>;

    /// Delete an identity
    async fn delete_identity(&self, did: &str) -> Result<bool, IdentityError>;

    /// List all identities
    async fn list_identities(&self) -> Result<Vec<IdentityProfile>, IdentityError>;
}

/// In-memory implementation of the identity storage
pub struct MemoryIdentityStorage {
    /// Identities by DID
    identities: RwLock<HashMap<String, IdentityProfile>>,
}

impl MemoryIdentityStorage {
    /// Create a new memory-based identity storage
    pub fn new() -> Self {
        Self {
            identities: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl IdentityStorage for MemoryIdentityStorage {
    async fn create_identity(&self, profile: IdentityProfile) -> Result<(), IdentityError> {
        let mut identities = self
            .identities
            .write()
            .map_err(|e| IdentityError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        // Check if the identity already exists
        if identities.contains_key(&profile.did) {
            return Err(IdentityError::AlreadyExists(format!(
                "Identity already exists: {}",
                profile.did
            )));
        }

        // Store the identity
        identities.insert(profile.did.clone(), profile);

        Ok(())
    }

    async fn get_identity(&self, did: &str) -> Result<IdentityProfile, IdentityError> {
        let identities = self
            .identities
            .read()
            .map_err(|e| IdentityError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        // Get the identity
        identities
            .get(did)
            .cloned()
            .ok_or_else(|| IdentityError::NotFound(format!("Identity not found: {}", did)))
    }

    async fn update_identity(&self, profile: IdentityProfile) -> Result<(), IdentityError> {
        let mut identities = self
            .identities
            .write()
            .map_err(|e| IdentityError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        // Check if the identity exists
        if !identities.contains_key(&profile.did) {
            return Err(IdentityError::NotFound(format!(
                "Identity not found: {}",
                profile.did
            )));
        }

        // Update the identity
        identities.insert(profile.did.clone(), profile);

        Ok(())
    }

    async fn delete_identity(&self, did: &str) -> Result<bool, IdentityError> {
        let mut identities = self
            .identities
            .write()
            .map_err(|e| IdentityError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        // Delete the identity
        Ok(identities.remove(did).is_some())
    }

    async fn list_identities(&self) -> Result<Vec<IdentityProfile>, IdentityError> {
        let identities = self
            .identities
            .read()
            .map_err(|e| IdentityError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        // Get all identities
        Ok(identities.values().cloned().collect())
    }
}
