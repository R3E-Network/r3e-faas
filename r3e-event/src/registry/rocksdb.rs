// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use crate::registry::FunctionMetadata;
use crate::registry::RegistryError;
use r3e_store::RocksDBStore;
use r3e_store::rocksdb::RocksDbConfig;
use std::path::Path;

/// RocksDB implementation of function storage
pub struct RocksDBFunctionStorage {
    db: RocksDBStore,
    cf_name: String,
}

impl RocksDBFunctionStorage {
    /// Create a new RocksDB function storage
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self, RegistryError> {
        let config = RocksDbConfig {
            path: db_path.as_ref().to_string_lossy().to_string(),
            ..Default::default()
        };
        
        let db = RocksDBStore::new(config);
        
        // Open the database
        db.open().map_err(|e| RegistryError::Storage(format!("Failed to open RocksDB store: {}", e)))?;
        
        let cf_name = "functions".to_string();
        
        // Create column family if it doesn't exist
        db.create_cf_if_missing(&cf_name)
            .map_err(|e| RegistryError::Storage(format!("Failed to create column family: {}", e)))?;

        Ok(Self { db, cf_name })
    }
}

impl crate::registry::FunctionStorage for RocksDBFunctionStorage {
    fn store_function(&mut self, metadata: &FunctionMetadata) -> Result<(), RegistryError> {
        let key = &metadata.id;
        let value =
            serde_json::to_vec(metadata).map_err(|e| RegistryError::Storage(e.to_string()))?;

        self.db
            .put_cf(&self.cf_name, key, &value)
            .map_err(|e| RegistryError::Storage(format!("Failed to store function: {}", e)))
    }

    fn get_function(&self, id: &str) -> Result<FunctionMetadata, RegistryError> {
        match self.db.get_cf::<_, Vec<u8>>(&self.cf_name, id) {
            Ok(Some(value)) => {
                let metadata: FunctionMetadata = serde_json::from_slice(&value)
                    .map_err(|e| RegistryError::Storage(e.to_string()))?;
                Ok(metadata)
            }
            Ok(None) => Err(RegistryError::NotFound(format!(
                "Function not found: {}",
                id
            ))),
            Err(e) => Err(RegistryError::Storage(format!("Failed to get function: {}", e))),
        }
    }

    fn list_functions(
        &self,
        _page_token: String,
        page_size: u32,
        trigger_type: String,
    ) -> Result<Vec<FunctionMetadata>, RegistryError> {
        // Create a prefix iterator to collect the results
        let iter: Box<dyn Iterator<Item = (Box<[u8]>, Box<[u8]>)> + Send> = 
            self.db.prefix_iter_cf(&self.cf_name, b"")
            .map_err(|e| RegistryError::Storage(format!("Failed to scan functions: {}", e)))?;
        
        let mut functions = Vec::new();
        let mut count = 0;
        
        for (_, value_boxed) in iter {
            if count >= page_size {
                break;
            }
            
            let value_vec = value_boxed.to_vec();
            
            let metadata: FunctionMetadata = serde_json::from_slice(&value_vec)
                .map_err(|e| RegistryError::Storage(e.to_string()))?;
            
            // If trigger_type is empty, include all functions
            if trigger_type.is_empty() || metadata.trigger.as_ref().map_or(false, |t| t.trigger_type == trigger_type) {
                functions.push(metadata);
                count += 1;
            }
        }

        Ok(functions)
    }

    fn delete_function(&mut self, id: &str) -> Result<bool, RegistryError> {
        // Check if function exists
        let exists = match self.db.get_cf::<_, Vec<u8>>(&self.cf_name, id) {
            Ok(Some(_)) => true,
            Ok(None) => false,
            Err(e) => return Err(RegistryError::Storage(format!("Failed to get function: {}", e))),
        };

        if !exists {
            return Ok(false);
        }

        self.db
            .delete_cf(&self.cf_name, id)
            .map_err(|e| RegistryError::Storage(format!("Failed to delete function: {}", e)))?;

        Ok(true)
    }
}
