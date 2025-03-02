// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Repository pattern implementations

use crate::rocksdb::DbResult;
use async_trait::async_trait;

pub mod service;
pub mod user;

/// Repository trait for database operations
#[async_trait]
pub trait DbRepository<T> {
    /// Create a new entity
    async fn create(&self, entity: T) -> DbResult<()>;
    
    /// Update an existing entity
    async fn update(&self, entity: T) -> DbResult<()>;
    
    /// Delete an entity
    async fn delete(&self, entity: T) -> DbResult<()>;
    
    /// Get an entity by ID
    async fn get(&self, id: String) -> DbResult<Option<T>>;
}
