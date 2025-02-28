// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! RocksDB-based storage implementation.

use async_trait::async_trait;
use r3e_store::rocksdb::RocksDBStore;
use std::path::Path;
use std::sync::Arc;

use crate::error::{Error, Result};
use crate::storage::Storage;
use crate::types::MainType;

/// RocksDB-based storage implementation
pub struct RocksDBStorage {
    db: Arc<RocksDBStore>,
    cf_name: String,
}

impl RocksDBStorage {
    /// Create a new RocksDB-based storage
    pub fn new<P: AsRef<Path>>(db_path: P, cf_name: &str) -> Result<Self> {
        let db = RocksDBStore::new(db_path)
            .map_err(|e| Error::Storage(format!("Failed to create RocksDB store: {}", e)))?;
        
        Ok(Self {
            db: Arc::new(db),
            cf_name: cf_name.to_string(),
        })
    }
}

#[async_trait]
impl Storage for RocksDBStorage {
    async fn store(&self, main_type: &MainType) -> Result<()> {
        let key = main_type.id.as_bytes();
        let value = serde_json::to_vec(main_type)
            .map_err(|e| Error::Serialization(e))?;
        
        let input = r3e_store::PutInput {
            key,
            value: &value,
            if_not_exists: false,
        };
        
        self.db.put(&self.cf_name, input)
            .map_err(|e| Error::Storage(format!("Failed to store MainType: {}", e)))?;
        
        Ok(())
    }
    
    async fn get(&self, id: &str) -> Result<MainType> {
        let key = id.as_bytes();
        
        match self.db.get(&self.cf_name, key) {
            Ok(value) => {
                let main_type = serde_json::from_slice::<MainType>(&value)
                    .map_err(|e| Error::Serialization(e))?;
                Ok(main_type)
            },
            Err(r3e_store::GetError::NoSuchKey) => {
                Err(Error::NotFound(format!("MainType not found: {}", id)))
            },
            Err(e) => {
                Err(Error::Storage(format!("Failed to get MainType: {}", e)))
            }
        }
    }
    
    async fn delete(&self, id: &str) -> Result<()> {
        let key = id.as_bytes();
        
        // Check if MainType exists
        match self.db.get(&self.cf_name, key) {
            Ok(_) => {
                self.db.delete(&self.cf_name, key)
                    .map_err(|e| Error::Storage(format!("Failed to delete MainType: {}", e)))?;
                Ok(())
            },
            Err(r3e_store::GetError::NoSuchKey) => {
                Err(Error::NotFound(format!("MainType not found: {}", id)))
            },
            Err(e) => {
                Err(Error::Storage(format!("Failed to get MainType: {}", e)))
            }
        }
    }
    
    async fn list(&self) -> Result<Vec<MainType>> {
        let input = r3e_store::ScanInput {
            start_key: &[],
            start_exclusive: false,
            end_key: &[],
            end_inclusive: false,
            max_count: 1000, // Reasonable limit
        };
        
        let output = self.db.scan(&self.cf_name, input)
            .map_err(|e| Error::Storage(format!("Failed to scan MainTypes: {}", e)))?;
        
        let mut main_types = Vec::new();
        
        for (_, value) in output.kvs {
            let main_type = serde_json::from_slice::<MainType>(&value)
                .map_err(|e| Error::Serialization(e))?;
            main_types.push(main_type);
        }
        
        Ok(main_types)
    }
}
