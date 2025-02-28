// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IndexingError {
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Query error: {0}")]
    Query(String),
    
    #[error("Data error: {0}")]
    Data(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingQuery {
    /// Collection to query
    pub collection: String,
    
    /// Filter criteria in JSON format
    pub filter: serde_json::Value,
    
    /// Optional sort criteria
    pub sort: Option<serde_json::Value>,
    
    /// Optional limit for results
    pub limit: Option<u32>,
    
    /// Optional number of results to skip
    pub skip: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingResult {
    /// Result data as JSON objects
    pub data: Vec<serde_json::Value>,
    
    /// Total number of matching documents
    pub total: u32,
    
    /// Current page number
    pub page: u32,
    
    /// Page size
    pub page_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexDefinition {
    /// Collection name
    pub collection: String,
    
    /// Fields to index
    pub fields: Vec<String>,
    
    /// Index type (e.g., "unique", "text", "geo")
    pub index_type: String,
    
    /// Optional index name
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionStats {
    /// Collection name
    pub name: String,
    
    /// Number of documents
    pub document_count: u64,
    
    /// Size in bytes
    pub size_bytes: u64,
    
    /// Number of indexes
    pub index_count: u32,
}
