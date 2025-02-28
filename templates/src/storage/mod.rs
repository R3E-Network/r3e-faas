// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Storage trait and implementations for the crate.

use async_trait::async_trait;
use crate::error::Result;
use crate::types::MainType;

pub mod memory;
pub mod rocksdb;

/// Storage trait for MainType
#[async_trait]
pub trait Storage: Send + Sync {
    /// Store a MainType
    async fn store(&self, main_type: &MainType) -> Result<()>;
    
    /// Get a MainType by ID
    async fn get(&self, id: &str) -> Result<MainType>;
    
    /// Delete a MainType by ID
    async fn delete(&self, id: &str) -> Result<()>;
    
    /// List all MainTypes
    async fn list(&self) -> Result<Vec<MainType>>;
}

pub use memory::MemoryStorage;
pub use rocksdb::RocksDBStorage;
