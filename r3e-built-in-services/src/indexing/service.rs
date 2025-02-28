// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_trait::async_trait;
use std::sync::Arc;
use crate::indexing::storage::IndexingStorage;
use crate::indexing::types::{IndexDefinition, IndexingError, IndexingQuery, IndexingResult, CollectionStats};

/// Trait defining the indexing service functionality
#[async_trait]
pub trait IndexingServiceTrait: Send + Sync {
    /// Query indexed data
    async fn query(&self, query: IndexingQuery) -> Result<IndexingResult, IndexingError>;
    
    /// Index new data
    async fn index_data(&self, collection: &str, data: serde_json::Value) -> Result<String, IndexingError>;
    
    /// Get a list of all collections
    async fn get_collections(&self) -> Result<Vec<String>, IndexingError>;
    
    /// Get statistics for a collection
    async fn get_collection_stats(&self, collection: &str) -> Result<CollectionStats, IndexingError>;
    
    /// Create a new index
    async fn create_index(&self, index_def: IndexDefinition) -> Result<(), IndexingError>;
    
    /// Delete a document by ID
    async fn delete_document(&self, collection: &str, id: &str) -> Result<bool, IndexingError>;
    
    /// Update a document by ID
    async fn update_document(&self, collection: &str, id: &str, data: serde_json::Value) -> Result<bool, IndexingError>;
    
    /// Get a document by ID
    async fn get_document(&self, collection: &str, id: &str) -> Result<Option<serde_json::Value>, IndexingError>;
}

/// Implementation of the indexing service
pub struct IndexingService<S: IndexingStorage> {
    /// Storage backend
    storage: Arc<S>,
}

impl<S: IndexingStorage> IndexingService<S> {
    /// Create a new indexing service
    pub fn new(storage: Arc<S>) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl<S: IndexingStorage> IndexingServiceTrait for IndexingService<S> {
    async fn query(&self, query: IndexingQuery) -> Result<IndexingResult, IndexingError> {
        self.storage.query(query).await
    }
    
    async fn index_data(&self, collection: &str, data: serde_json::Value) -> Result<String, IndexingError> {
        self.storage.index_data(collection, data).await
    }
    
    async fn get_collections(&self) -> Result<Vec<String>, IndexingError> {
        self.storage.get_collections().await
    }
    
    async fn get_collection_stats(&self, collection: &str) -> Result<CollectionStats, IndexingError> {
        self.storage.get_collection_stats(collection).await
    }
    
    async fn create_index(&self, index_def: IndexDefinition) -> Result<(), IndexingError> {
        self.storage.create_index(index_def).await
    }
    
    async fn delete_document(&self, collection: &str, id: &str) -> Result<bool, IndexingError> {
        self.storage.delete_document(collection, id).await
    }
    
    async fn update_document(&self, collection: &str, id: &str, data: serde_json::Value) -> Result<bool, IndexingError> {
        self.storage.update_document(collection, id, data).await
    }
    
    async fn get_document(&self, collection: &str, id: &str) -> Result<Option<serde_json::Value>, IndexingError> {
        self.storage.get_document(collection, id).await
    }
}
