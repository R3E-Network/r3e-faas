// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Error types for the Zero-Knowledge computing service.

use thiserror::Error;

/// Errors that can occur in the Zero-Knowledge computing service.
#[derive(Error, Debug)]
pub enum ZkError {
    /// Error occurred in the Zokrates provider.
    #[error("Zokrates provider error: {0}")]
    ZokratesError(String),

    /// Error occurred in the Bulletproofs provider.
    #[error("Bulletproofs provider error: {0}")]
    BulletproofsError(String),

    /// Error occurred during circuit compilation.
    #[error("Circuit compilation error: {0}")]
    CompilationError(String),

    /// Error occurred during key generation.
    #[error("Key generation error: {0}")]
    KeyGenerationError(String),

    /// Error occurred during proof generation.
    #[error("Proof generation error: {0}")]
    ProofGenerationError(String),

    /// Error occurred during proof verification.
    #[error("Proof verification error: {0}")]
    ProofVerificationError(String),

    /// Error occurred in the storage layer.
    #[error("Storage error: {0}")]
    StorageError(String),

    /// Error occurred during serialization or deserialization.
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Error occurred due to invalid input.
    #[error("Invalid input: {0}")]
    InvalidInputError(String),

    /// Error occurred due to missing data.
    #[error("Missing data: {0}")]
    MissingDataError(String),

    /// Error occurred due to unsupported platform.
    #[error("Unsupported platform: {0}")]
    UnsupportedPlatformError(String),

    /// Error occurred due to invalid configuration.
    #[error("Invalid configuration: {0}")]
    ConfigurationError(String),

    /// Error occurred due to IO operations.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Error occurred due to RocksDB operations.
    #[error("RocksDB error: {0}")]
    RocksDbError(String),

    /// Error occurred due to JSON operations.
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Unknown error.
    #[error("Unknown error: {0}")]
    UnknownError(String),
}

/// Result type for the Zero-Knowledge computing service.
pub type ZkResult<T> = Result<T, ZkError>;
