// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

pub mod storage;
pub mod service;
pub mod rocksdb;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

use uuid::Uuid;

use crate::registry::storage::FunctionStorage;
pub use crate::registry::proto::*;

// Re-export generated proto types
pub mod proto {
    pub use crate::registry::*;
}

/// Function registry for managing user-provided JavaScript functions
pub struct FunctionRegistry {
    storage: Arc<RwLock<Box<dyn FunctionStorage>>>,
}

impl FunctionRegistry {
    /// Create a new function registry with the given storage backend
    pub fn new(storage: Box<dyn FunctionStorage>) -> Self {
        Self {
            storage: Arc::new(RwLock::new(storage)),
        }
    }

    /// Register a new function
    pub async fn register_function(
        &self,
        request: RegisterFunctionRequest,
    ) -> Result<RegisterFunctionResponse, RegistryError> {
        // Generate a unique ID for the function
        let id = Uuid::new_v4().to_string();
        
        // Get current timestamp
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Create function metadata
        let metadata = FunctionMetadata {
            id,
            name: request.name,
            description: request.description,
            version: 1, // Initial version
            created_at: now,
            updated_at: now,
            trigger: request.trigger,
            permissions: request.permissions,
            resources: request.resources,
            code: request.code,
        };
        
        // Store the function metadata
        self.storage.write().unwrap().store_function(&metadata)?;
        
        Ok(RegisterFunctionResponse { metadata: Some(metadata) })
    }

    /// Update an existing function
    pub async fn update_function(
        &self,
        request: UpdateFunctionRequest,
    ) -> Result<UpdateFunctionResponse, RegistryError> {
        // Get the existing function
        let mut metadata = self.storage.read().unwrap().get_function(&request.id)?;
        
        // Get current timestamp
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Update function metadata
        if let Some(name) = request.name {
            metadata.name = name;
        }
        
        if let Some(description) = request.description {
            metadata.description = description;
        }
        
        if let Some(trigger) = request.trigger {
            metadata.trigger = Some(trigger);
        }
        
        if let Some(permissions) = request.permissions {
            metadata.permissions = Some(permissions);
        }
        
        if let Some(resources) = request.resources {
            metadata.resources = Some(resources);
        }
        
        if let Some(code) = request.code {
            metadata.code = code;
        }
        
        // Increment version
        metadata.version += 1;
        metadata.updated_at = now;
        
        // Store the updated function metadata
        self.storage.write().unwrap().store_function(&metadata)?;
        
        Ok(UpdateFunctionResponse { metadata: Some(metadata) })
    }

    /// Get a function by ID
    pub async fn get_function(
        &self,
        request: GetFunctionRequest,
    ) -> Result<GetFunctionResponse, RegistryError> {
        let metadata = self.storage.read().unwrap().get_function(&request.id)?;
        Ok(GetFunctionResponse { metadata: Some(metadata) })
    }

    /// List functions with optional filtering
    pub async fn list_functions(
        &self,
        request: ListFunctionsRequest,
    ) -> Result<ListFunctionsResponse, RegistryError> {
        let functions = self.storage.read().unwrap().list_functions(
            request.page_token,
            request.page_size,
            request.trigger_type,
        )?;
        
        // For simplicity, we're not implementing pagination in this example
        Ok(ListFunctionsResponse {
            functions,
            next_page_token: "".to_string(),
        })
    }

    /// Delete a function by ID
    pub async fn delete_function(
        &self,
        request: DeleteFunctionRequest,
    ) -> Result<DeleteFunctionResponse, RegistryError> {
        let success = self.storage.write().unwrap().delete_function(&request.id)?;
        Ok(DeleteFunctionResponse { success })
    }
}

/// Error types for function registry operations
#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("function not found: {0}")]
    NotFound(String),
    
    #[error("storage error: {0}")]
    Storage(String),
    
    #[error("validation error: {0}")]
    Validation(String),
    
    #[error("internal error: {0}")]
    Internal(String),
}

impl From<std::io::Error> for RegistryError {
    fn from(err: std::io::Error) -> Self {
        RegistryError::Storage(err.to_string())
    }
}
