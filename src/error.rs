// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Error types for the R3E Store

use thiserror::Error;

/// Error type for key-value operations
#[derive(Debug, Error)]
pub enum StoreError {
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub type StoreResult<T> = Result<T, StoreError>; 