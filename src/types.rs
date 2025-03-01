// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Type definitions for the key-value store

/// Maximum size for a table name in bytes
pub const MAX_TABLE_NAME_SIZE: usize = 64;

/// Maximum size for a key in bytes
pub const MAX_KEY_SIZE: usize = 1024;

/// Maximum size for a value in bytes
pub const MAX_VALUE_SIZE: usize = 10 * 1024 * 1024; // 10MB

/// Input for a Put operation
#[derive(Debug, Clone)]
pub struct PutInput<'a> {
    /// Key to write
    pub key: &'a [u8],

    /// Value to write
    pub value: &'a [u8],

    /// Only write if the key doesn't exist
    pub if_not_exists: bool,
}

/// Input for a Scan operation
#[derive(Debug, Clone)]
pub struct ScanInput<'a> {
    /// Start key (inclusive by default)
    pub start_key: &'a [u8],

    /// Start is exclusive
    pub start_exclusive: bool,

    /// End key (inclusive by default)
    pub end_key: &'a [u8],

    /// End is exclusive
    pub end_inclusive: bool,

    /// Maximum number of key-value pairs to return
    pub limit: usize,
}

impl<'a> ScanInput<'a> {
    /// Maximum count of results
    pub fn max_count(&self) -> usize {
        self.limit
    }
}

/// Output for a Scan operation
#[derive(Debug, Clone)]
pub struct ScanOutput {
    /// Key-value pairs
    pub kvs: Vec<(Vec<u8>, Vec<u8>)>,

    /// Whether there are more results
    pub has_more: bool,
} 