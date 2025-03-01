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
}

/// Result type for the core crate
pub type Result<T> = std::result::Result<T, Error>;
