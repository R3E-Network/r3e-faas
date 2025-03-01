// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use crate::indexing::types::{
    CollectionStats, IndexDefinition, IndexingError, IndexingQuery, IndexingResult,
};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// Trait defining the indexing storage functionality
#[async_trait]
pub trait IndexingStorage: Send + Sync {
    /// Query indexed data
    async fn query(&self, query: IndexingQuery) -> Result<IndexingResult, IndexingError>;

    /// Index new data
    async fn index_data(
        &self,
        collection: &str,
        data: serde_json::Value,
    ) -> Result<String, IndexingError>;

    /// Get a list of all collections
    async fn get_collections(&self) -> Result<Vec<String>, IndexingError>;

    /// Get statistics for a collection
    async fn get_collection_stats(
        &self,
        collection: &str,
    ) -> Result<CollectionStats, IndexingError>;

    /// Create a new index
    async fn create_index(&self, index_def: IndexDefinition) -> Result<(), IndexingError>;

    /// Delete a document by ID
    async fn delete_document(&self, collection: &str, id: &str) -> Result<bool, IndexingError>;

    /// Update a document by ID
    async fn update_document(
        &self,
        collection: &str,
        id: &str,
        data: serde_json::Value,
    ) -> Result<bool, IndexingError>;

    /// Get a document by ID
    async fn get_document(
        &self,
        collection: &str,
        id: &str,
    ) -> Result<Option<serde_json::Value>, IndexingError>;
}

/// In-memory implementation of the indexing storage
pub struct MemoryIndexingStorage {
    /// Collections and their documents
    collections: RwLock<HashMap<String, HashMap<String, serde_json::Value>>>,

    /// Indexes for collections
    indexes: RwLock<HashMap<String, Vec<IndexDefinition>>>,
}

impl MemoryIndexingStorage {
    /// Create a new memory-based indexing storage
    pub fn new() -> Self {
        Self {
            collections: RwLock::new(HashMap::new()),
            indexes: RwLock::new(HashMap::new()),
        }
    }

    /// Get or create a collection
    fn get_or_create_collection(&self, collection: &str) -> Result<(), IndexingError> {
        let mut collections = self
            .collections
            .write()
            .map_err(|e| IndexingError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        if !collections.contains_key(collection) {
            collections.insert(collection.to_string(), HashMap::new());
        }

        Ok(())
    }

    /// Apply filter to documents
    fn apply_filter(
        &self,
        documents: &HashMap<String, serde_json::Value>,
        filter: &serde_json::Value,
    ) -> Vec<serde_json::Value> {
        // This is a simplified implementation that would need to be expanded for a real system
        // In a real implementation, we would parse the filter and apply it to each document

        documents.values().cloned().collect()
    }

    /// Apply sort to documents
    fn apply_sort(
        &self,
        mut documents: Vec<serde_json::Value>,
        sort: &serde_json::Value,
    ) -> Vec<serde_json::Value> {
        // This is a simplified implementation that would need to be expanded for a real system
        // In a real implementation, we would parse the sort criteria and sort the documents

        documents
    }

    /// Apply pagination to documents
    fn apply_pagination(
        &self,
        documents: Vec<serde_json::Value>,
        skip: Option<u32>,
        limit: Option<u32>,
    ) -> Vec<serde_json::Value> {
        let skip = skip.unwrap_or(0) as usize;
        let limit = limit.unwrap_or(100) as usize;

        documents.into_iter().skip(skip).take(limit).collect()
    }
}

#[async_trait]
impl IndexingStorage for MemoryIndexingStorage {
    async fn query(&self, query: IndexingQuery) -> Result<IndexingResult, IndexingError> {
        let collections = self
            .collections
            .read()
            .map_err(|e| IndexingError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        let collection_data = collections.get(&query.collection).ok_or_else(|| {
            IndexingError::NotFound(format!("Collection not found: {}", query.collection))
        })?;

        // Apply filter
        let filtered_docs = self.apply_filter(collection_data, &query.filter);

        // Apply sort if provided
        let sorted_docs = if let Some(sort) = &query.sort {
            self.apply_sort(filtered_docs, sort)
        } else {
            filtered_docs
        };

        // Apply pagination
        let paginated_docs = self.apply_pagination(sorted_docs, query.skip, query.limit);

        // Create result
        let result = IndexingResult {
            data: paginated_docs.clone(),
            total: filtered_docs.len() as u32,
            page: (query.skip.unwrap_or(0) / query.limit.unwrap_or(100)) + 1,
            page_size: query.limit.unwrap_or(100),
        };

        Ok(result)
    }

    async fn index_data(
        &self,
        collection: &str,
        mut data: serde_json::Value,
    ) -> Result<String, IndexingError> {
        // Ensure the collection exists
        self.get_or_create_collection(collection)?;

        // Generate an ID if not provided
        let id = if let Some(id_value) = data.get("_id") {
            id_value
                .as_str()
                .unwrap_or_else(|| Uuid::new_v4().to_string().as_str())
                .to_string()
        } else {
            let id = Uuid::new_v4().to_string();
            data["_id"] = serde_json::Value::String(id.clone());
            id
        };

        // Store the document
        let mut collections = self
            .collections
            .write()
            .map_err(|e| IndexingError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        if let Some(collection_data) = collections.get_mut(collection) {
            collection_data.insert(id.clone(), data);
        }

        Ok(id)
    }

    async fn get_collections(&self) -> Result<Vec<String>, IndexingError> {
        let collections = self
            .collections
            .read()
            .map_err(|e| IndexingError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        Ok(collections.keys().cloned().collect())
    }

    async fn get_collection_stats(
        &self,
        collection: &str,
    ) -> Result<CollectionStats, IndexingError> {
        let collections = self
            .collections
            .read()
            .map_err(|e| IndexingError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        let collection_data = collections.get(collection).ok_or_else(|| {
            IndexingError::NotFound(format!("Collection not found: {}", collection))
        })?;

        let indexes = self
            .indexes
            .read()
            .map_err(|e| IndexingError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        let index_count = indexes.get(collection).map_or(0, |idx| idx.len() as u32);

        // Calculate size (this is a rough estimate)
        let size_bytes = collection_data
            .values()
            .map(|v| serde_json::to_string(v).unwrap_or_default().len() as u64)
            .sum();

        Ok(CollectionStats {
            name: collection.to_string(),
            document_count: collection_data.len() as u64,
            size_bytes,
            index_count,
        })
    }

    async fn create_index(&self, index_def: IndexDefinition) -> Result<(), IndexingError> {
        // Ensure the collection exists
        self.get_or_create_collection(&index_def.collection)?;

        // Add the index definition
        let mut indexes = self
            .indexes
            .write()
            .map_err(|e| IndexingError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        let collection_indexes = indexes
            .entry(index_def.collection.clone())
            .or_insert_with(Vec::new);
        collection_indexes.push(index_def);

        Ok(())
    }

    async fn delete_document(&self, collection: &str, id: &str) -> Result<bool, IndexingError> {
        let mut collections = self
            .collections
            .write()
            .map_err(|e| IndexingError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        let collection_data = collections.get_mut(collection).ok_or_else(|| {
            IndexingError::NotFound(format!("Collection not found: {}", collection))
        })?;

        Ok(collection_data.remove(id).is_some())
    }

    async fn update_document(
        &self,
        collection: &str,
        id: &str,
        data: serde_json::Value,
    ) -> Result<bool, IndexingError> {
        let mut collections = self
            .collections
            .write()
            .map_err(|e| IndexingError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        let collection_data = collections.get_mut(collection).ok_or_else(|| {
            IndexingError::NotFound(format!("Collection not found: {}", collection))
        })?;

        if !collection_data.contains_key(id) {
            return Ok(false);
        }

        // Update the document
        collection_data.insert(id.to_string(), data);

        Ok(true)
    }

    async fn get_document(
        &self,
        collection: &str,
        id: &str,
    ) -> Result<Option<serde_json::Value>, IndexingError> {
        let collections = self
            .collections
            .read()
            .map_err(|e| IndexingError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        let collection_data = collections.get(collection).ok_or_else(|| {
            IndexingError::NotFound(format!("Collection not found: {}", collection))
        })?;

        Ok(collection_data.get(id).cloned())
    }
}
