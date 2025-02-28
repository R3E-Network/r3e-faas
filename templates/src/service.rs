// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Service implementation for the crate.

use crate::error::{Error, Result};
use crate::storage::Storage;
use crate::types::MainType;
use std::sync::Arc;

/// Service for managing MainType
pub struct Service {
    storage: Arc<dyn Storage>,
}

impl Service {
    /// Create a new service with the given storage
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        Self { storage }
    }
    
    /// Create a new MainType
    pub async fn create(&self, name: String, description: Option<String>) -> Result<MainType> {
        let id = uuid::Uuid::new_v4().to_string();
        
        let main_type = MainType {
            id,
            name,
            description,
        };
        
        self.storage.store(&main_type).await?;
        
        Ok(main_type)
    }
    
    /// Get a MainType by ID
    pub async fn get(&self, id: &str) -> Result<MainType> {
        self.storage.get(id).await
    }
    
    /// Update a MainType
    pub async fn update(&self, id: &str, name: Option<String>, description: Option<String>) -> Result<MainType> {
        let mut main_type = self.storage.get(id).await?;
        
        if let Some(name) = name {
            main_type.name = name;
        }
        
        if let Some(description) = description {
            main_type.description = description;
        }
        
        self.storage.store(&main_type).await?;
        
        Ok(main_type)
    }
    
    /// Delete a MainType
    pub async fn delete(&self, id: &str) -> Result<()> {
        self.storage.delete(id).await
    }
    
    /// List all MainTypes
    pub async fn list(&self) -> Result<Vec<MainType>> {
        self.storage.list().await
    }
}
