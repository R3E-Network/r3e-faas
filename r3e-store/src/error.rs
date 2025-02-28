// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Error types for the store crate.

use thiserror::Error;

/// Error type for put operations
#[derive(Debug, Error)]
pub enum PutError {
    /// Key already exists
    #[error("kv-put: key already exists")]
    AlreadyExists,

    /// Invalid table name
    #[error("kv-put: invalid table name")]
    InvalidTable,

    /// Key is too large
    #[error("kv-put: key is too large")]
    TooLargeKey,

    /// Value is too large
    #[error("kv-put: value is too large")]
    TooLargeValue,
}

/// Error type for get operations
#[derive(Debug, Error)]
pub enum GetError {
    /// No such key
    #[error("kv-get: no such key")]
    NoSuchKey,

    /// Invalid table name
    #[error("kv-get: invalid table name")]
    InvalidTable,

    /// Key is too large
    #[error("kv-get: key is too large")]
    TooLargeKey,
}

/// Error type for delete operations
#[derive(Debug, Error)]
pub enum DeleteError {
    /// Key is too large
    #[error("kv-delete: key is too large")]
    TooLargeKey,

    /// Invalid table name
    #[error("kv-delete: invalid table name")]
    InvalidTable,
}

/// Error type for scan operations
#[derive(Debug, Error)]
pub enum ScanError {
    /// Key is too large
    #[error("kv-scan: key is too large")]
    TooLargeKey,

    /// Invalid table name
    #[error("kv-scan: invalid table name")]
    InvalidTable,
}

/// Error type for multi-put operations
#[derive(Debug, Error)]
pub enum MultiPutError {
    /// Key already exists
    #[error("kv-multi-put: key already exists")]
    AlreadyExists,

    /// Invalid table name
    #[error("kv-multi-put: invalid table name")]
    InvalidTable,

    /// Key is too large
    #[error("kv-multi-put: key is too large")]
    TooLargeKey,

    /// Value is too large
    #[error("kv-multi-put: value is too large")]
    TooLargeValue,
}

/// Error type for multi-get operations
#[derive(Debug, Error)]
pub enum MultiGetError {
    /// Key is too large
    #[error("kv-multi-get: key is too large")]
    TooLargeKey,

    /// Invalid table name
    #[error("kv-multi-get: invalid table name")]
    InvalidTable,
}

/// Error type for multi-delete operations
#[derive(Debug, Error)]
pub enum MultiDeleteError {
    /// Key is too large
    #[error("kv-multi-delete: key is too large")]
    TooLargeKey,

    /// Invalid table name
    #[error("kv-multi-delete: invalid table name")]
    InvalidTable,
}
