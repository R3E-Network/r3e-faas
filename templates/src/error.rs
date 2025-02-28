// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Error types for the crate.

use thiserror::Error;

/// Error type for the crate
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
}

/// Result type for the crate
pub type Result<T> = std::result::Result<T, Error>;
