// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Error types for the Fully Homomorphic Encryption service.

use thiserror::Error;

/// Errors that can occur in the Fully Homomorphic Encryption service.
#[derive(Error, Debug)]
pub enum FheError {
    /// Error occurred in the TFHE scheme.
    #[error("TFHE scheme error: {0}")]
    TfheError(String),

    /// Error occurred in the OpenFHE scheme.
    #[error("OpenFHE scheme error: {0}")]
    OpenFheError(String),

    /// Error occurred during key generation.
    #[error("Key generation error: {0}")]
    KeyGenerationError(String),

    /// Error occurred during encryption.
    #[error("Encryption error: {0}")]
    EncryptionError(String),

    /// Error occurred during decryption.
    #[error("Decryption error: {0}")]
    DecryptionError(String),

    /// Error occurred during homomorphic operation.
    #[error("Homomorphic operation error: {0}")]
    HomomorphicOperationError(String),

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

    /// Error occurred due to unsupported scheme.
    #[error("Unsupported scheme: {0}")]
    UnsupportedSchemeError(String),

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

/// Result type for the Fully Homomorphic Encryption service.
pub type FheResult<T> = Result<T, FheError>;
