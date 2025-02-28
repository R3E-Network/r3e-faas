// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Memory-based storage implementation.

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::error::{Error, Result};
use crate::storage::Storage;
use crate::types::MainType;

/// Memory-based storage implementation
pub struct MemoryStorage {
    data: Arc<RwLock<HashMap<String, MainType>>>,
}

impl MemoryStorage {
    /// Create a new memory-based storage
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl Storage for MemoryStorage {
    async fn store(&self, main_type: &MainType) -> Result<()> {
        let mut data = self.data.write().await;
        data.insert(main_type.id.clone(), main_type.clone());
        Ok(())
    }
    
    async fn get(&self, id: &str) -> Result<MainType> {
        let data = self.data.read().await;
        
        match data.get(id) {
            Some(main_type) => Ok(main_type.clone()),
            None => Err(Error::NotFound(format!("MainType not found: {}", id))),
        }
    }
    
    async fn delete(&self, id: &str) -> Result<()> {
        let mut data = self.data.write().await;
        
        if data.remove(id).is_none() {
            return Err(Error::NotFound(format!("MainType not found: {}", id)));
        }
        
        Ok(())
    }
    
    async fn list(&self) -> Result<Vec<MainType>> {
        let data = self.data.read().await;
        let main_types = data.values().cloned().collect();
        Ok(main_types)
    }
}
