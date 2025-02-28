// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Type definitions for the store crate.

use serde::{Deserialize, Serialize};

/// Maximum table name size in bytes
pub const MAX_TABLE_NAME_SIZE: usize = 128; // 128 bytes

/// Maximum key size in bytes
pub const MAX_KEY_SIZE: usize = 1024; // 1 KB

/// Maximum value size in bytes
pub const MAX_VALUE_SIZE: usize = 4 * 1024 * 1024; // 4 MB

/// Input for put operations
pub struct PutInput<'k, 'v> {
    /// Key to store
    pub key: &'k [u8],
    
    /// Value to store
    pub value: &'v [u8],
    
    /// If true, only put if the key doesn't exist
    pub if_not_exists: bool,
}

/// Input for scan operations
#[derive(Debug, Clone)]
pub struct ScanInput<'k, 'v> {
    /// Start key (empty means from the start of the table)
    pub start_key: &'k [u8],

    /// If true, the start key is excluded, otherwise included
    pub start_exclusive: bool,

    /// End key (empty means to the end of the table)
    pub end_key: &'v [u8],

    /// If true, the end key is included, otherwise excluded
    pub end_inclusive: bool,

    /// Maximum number of items to return (0 means 100)
    pub max_count: u32,
}

impl<'k, 'v> ScanInput<'k, 'v> {
    /// Get the effective maximum count
    pub fn max_count(&self) -> usize {
        if self.max_count == 0 {
            100
        } else {
            self.max_count as usize
        }
    }
}

/// Output for scan operations
#[derive(Debug, Clone)]
pub struct ScanOutput {
    /// Key-value pairs
    pub kvs: Vec<(Vec<u8>, Vec<u8>)>,
    
    /// True if there are more items to scan
    pub has_more: bool,
}
