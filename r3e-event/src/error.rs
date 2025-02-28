// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Error types for the event crate.

use thiserror::Error;

/// Error type for the event crate
#[derive(Debug, Error)]
pub enum Error {
    /// Invalid input error
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    /// Storage error
    #[error("Storage error: {0}")]
    Storage(String),
    
    /// Not found error
    #[error("Not found: {0}")]
    NotFound(String),
    
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    /// Source error
    #[error("Source error: {0}")]
    Source(String),
    
    /// Registry error
    #[error("Registry error: {0}")]
    Registry(String),
    
    /// Trigger error
    #[error("Trigger error: {0}")]
    Trigger(String),
}

/// Result type for the event crate
pub type Result<T> = std::result::Result<T, Error>;
