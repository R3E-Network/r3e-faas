// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Error types for the core crate.

use thiserror::Error;

/// Error type for the core crate
#[derive(Debug, Error)]
pub enum Error {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// V8 error
    #[error("V8 error: {0}")]
    V8(String),

    /// Signal hook error
    #[error("Signal hook error: {0}")]
    SignalHook(String),
    
    /// Authentication error
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    /// Authorization error
    #[error("Authorization error: {0}")]
    Authorization(String),
    
    /// Resource error
    #[error("Resource error: {0}")]
    Resource(String),
    
    /// Execution error
    #[error("Execution error: {0}")]
    Execution(String),
    
    /// External service error
    #[error("External service error: {0}")]
    ExternalService(String),
    
    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    /// Database error
    #[error("Database error: {0}")]
    Database(String),
    
    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),
    
    /// Rate limit error
    #[error("Rate limit error: {0}")]
    RateLimit(String),
    
    /// Not found error
    #[error("Not found error: {0}")]
    NotFound(String),
}

/// Result type for the core crate
pub type Result<T> = std::result::Result<T, Error>;
