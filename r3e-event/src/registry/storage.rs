// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::registry::{FunctionMetadata, RegistryError, TriggerType};

/// Storage interface for function metadata
pub trait FunctionStorage: Send + Sync {
    /// Store a function metadata
    fn store_function(&mut self, metadata: &FunctionMetadata) -> Result<(), RegistryError>;
    
    /// Get a function metadata by ID
    fn get_function(&self, id: &str) -> Result<FunctionMetadata, RegistryError>;
    
    /// List functions with optional filtering
    fn list_functions(
        &self,
        page_token: String,
        page_size: u32,
        trigger_type: Option<i32>,
    ) -> Result<Vec<FunctionMetadata>, RegistryError>;
    
    /// Delete a function by ID
    fn delete_function(&mut self, id: &str) -> Result<bool, RegistryError>;
}

/// In-memory implementation of function storage
pub struct MemoryStorage {
    functions: HashMap<String, FunctionMetadata>,
}

impl MemoryStorage {
    /// Create a new in-memory storage
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }
}

impl FunctionStorage for MemoryStorage {
    fn store_function(&mut self, metadata: &FunctionMetadata) -> Result<(), RegistryError> {
        self.functions.insert(metadata.id.clone(), metadata.clone());
        Ok(())
    }
    
    fn get_function(&self, id: &str) -> Result<FunctionMetadata, RegistryError> {
        self.functions
            .get(id)
            .cloned()
            .ok_or_else(|| RegistryError::NotFound(id.to_string()))
    }
    
    fn list_functions(
        &self,
        _page_token: String,
        _page_size: u32,
        trigger_type: Option<i32>,
    ) -> Result<Vec<FunctionMetadata>, RegistryError> {
        let mut functions = Vec::new();
        
        for metadata in self.functions.values() {
            // Filter by trigger type if specified
            if let Some(trigger_type) = trigger_type {
                if let Some(trigger) = &metadata.trigger {
                    if trigger.r#type != trigger_type {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            
            functions.push(metadata.clone());
        }
        
        Ok(functions)
    }
    
    fn delete_function(&mut self, id: &str) -> Result<bool, RegistryError> {
        Ok(self.functions.remove(id).is_some())
    }
}

/// File-based implementation of function storage
pub struct FileStorage {
    base_dir: std::path::PathBuf,
    functions: HashMap<String, FunctionMetadata>,
}

impl FileStorage {
    /// Create a new file-based storage
    pub fn new(base_dir: impl Into<std::path::PathBuf>) -> Result<Self, RegistryError> {
        let base_dir = base_dir.into();
        
        // Create the base directory if it doesn't exist
        std::fs::create_dir_all(&base_dir)?;
        
        // Load existing functions from the base directory
        let mut functions = HashMap::new();
        for entry in std::fs::read_dir(&base_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                let content = std::fs::read_to_string(&path)?;
                let metadata: FunctionMetadata = serde_json::from_str(&content)
                    .map_err(|e| RegistryError::Storage(e.to_string()))?;
                
                functions.insert(metadata.id.clone(), metadata);
            }
        }
        
        Ok(Self {
            base_dir,
            functions,
        })
    }
    
    /// Get the file path for a function ID
    fn get_file_path(&self, id: &str) -> std::path::PathBuf {
        self.base_dir.join(format!("{}.json", id))
    }
}

impl FunctionStorage for FileStorage {
    fn store_function(&mut self, metadata: &FunctionMetadata) -> Result<(), RegistryError> {
        // Store in memory
        self.functions.insert(metadata.id.clone(), metadata.clone());
        
        // Store on disk
        let path = self.get_file_path(&metadata.id);
        let content = serde_json::to_string_pretty(metadata)
            .map_err(|e| RegistryError::Storage(e.to_string()))?;
        
        std::fs::write(path, content)?;
        
        Ok(())
    }
    
    fn get_function(&self, id: &str) -> Result<FunctionMetadata, RegistryError> {
        self.functions
            .get(id)
            .cloned()
            .ok_or_else(|| RegistryError::NotFound(id.to_string()))
    }
    
    fn list_functions(
        &self,
        _page_token: String,
        _page_size: u32,
        trigger_type: Option<i32>,
    ) -> Result<Vec<FunctionMetadata>, RegistryError> {
        let mut functions = Vec::new();
        
        for metadata in self.functions.values() {
            // Filter by trigger type if specified
            if let Some(trigger_type) = trigger_type {
                if let Some(trigger) = &metadata.trigger {
                    if trigger.r#type != trigger_type {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            
            functions.push(metadata.clone());
        }
        
        Ok(functions)
    }
    
    fn delete_function(&mut self, id: &str) -> Result<bool, RegistryError> {
        // Remove from memory
        let exists = self.functions.remove(id).is_some();
        
        if exists {
            // Remove from disk
            let path = self.get_file_path(id);
            if path.exists() {
                std::fs::remove_file(path)?;
            }
        }
        
        Ok(exists)
    }
}
