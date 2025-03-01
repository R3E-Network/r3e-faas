// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use crate::registry::{FunctionMetadata, RegistryError, TriggerType};
use r3e_store::rocksdb::RocksDBStore;
use std::path::Path;

/// RocksDB implementation of function storage
pub struct RocksDBFunctionStorage {
    db: RocksDBStore,
    cf_name: String,
}

impl RocksDBFunctionStorage {
    /// Create a new RocksDB function storage
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self, RegistryError> {
        let db = RocksDBStore::new(db_path).map_err(|e| RegistryError::Storage(e.to_string()))?;

        let cf_name = "functions".to_string();

        Ok(Self { db, cf_name })
    }
}

impl crate::registry::FunctionStorage for RocksDBFunctionStorage {
    fn store_function(&mut self, metadata: &FunctionMetadata) -> Result<(), RegistryError> {
        let key = metadata.id.as_bytes();
        let value =
            serde_json::to_vec(metadata).map_err(|e| RegistryError::Storage(e.to_string()))?;

        let input = r3e_store::PutInput {
            key,
            value: &value,
            if_not_exists: false,
        };

        self.db
            .put(&self.cf_name, input)
            .map_err(|e| RegistryError::Storage(format!("Failed to store function: {}", e)))
    }

    fn get_function(&self, id: &str) -> Result<FunctionMetadata, RegistryError> {
        let key = id.as_bytes();

        match self.db.get(&self.cf_name, key) {
            Ok(value) => {
                let metadata: FunctionMetadata = serde_json::from_slice(&value)
                    .map_err(|e| RegistryError::Storage(e.to_string()))?;
                Ok(metadata)
            }
            Err(r3e_store::GetError::NoSuchKey) => Err(RegistryError::NotFound(id.to_string())),
            Err(e) => Err(RegistryError::Storage(format!(
                "Failed to get function: {}",
                e
            ))),
        }
    }

    fn list_functions(
        &self,
        _page_token: String,
        page_size: u32,
        trigger_type: Option<i32>,
    ) -> Result<Vec<FunctionMetadata>, RegistryError> {
        let input = r3e_store::ScanInput {
            start_key: &[],
            start_exclusive: false,
            end_key: &[],
            end_inclusive: false,
            max_count: page_size,
        };

        let output = self
            .db
            .scan(&self.cf_name, input)
            .map_err(|e| RegistryError::Storage(format!("Failed to list functions: {}", e)))?;

        let mut functions = Vec::new();

        for (_, value) in output.kvs {
            let metadata: FunctionMetadata = serde_json::from_slice(&value)
                .map_err(|e| RegistryError::Storage(e.to_string()))?;

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

            functions.push(metadata);
        }

        Ok(functions)
    }

    fn delete_function(&mut self, id: &str) -> Result<bool, RegistryError> {
        let key = id.as_bytes();

        // Check if function exists
        let exists = match self.db.get(&self.cf_name, key) {
            Ok(_) => true,
            Err(r3e_store::GetError::NoSuchKey) => false,
            Err(e) => {
                return Err(RegistryError::Storage(format!(
                    "Failed to check function existence: {}",
                    e
                )))
            }
        };

        if exists {
            self.db
                .delete(&self.cf_name, key)
                .map_err(|e| RegistryError::Storage(format!("Failed to delete function: {}", e)))?;
        }

        Ok(exists)
    }
}
