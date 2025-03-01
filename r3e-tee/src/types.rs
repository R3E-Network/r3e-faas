// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// TEE execution mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionMode {
    /// Synchronous execution
    Sync,

    /// Asynchronous execution
    Async,
}

/// TEE function metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionMetadata {
    /// Function name
    pub name: String,

    /// Function description
    pub description: String,

    /// Function version
    pub version: String,

    /// Function author
    pub author: String,

    /// Function creation timestamp
    pub created_at: u64,

    /// Function update timestamp
    pub updated_at: u64,

    /// Function hash
    pub hash: String,

    /// Function signature
    pub signature: Option<String>,
}

/// TEE memory protection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryProtection {
    /// No memory protection
    None,

    /// Encryption only
    Encryption,

    /// Integrity only
    Integrity,

    /// Encryption and integrity
    EncryptionAndIntegrity,
}

/// TEE execution options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionOptions {
    /// Execution mode
    pub mode: ExecutionMode,

    /// Memory protection
    pub memory_protection: MemoryProtection,

    /// Memory limit in MB
    pub memory_limit_mb: u32,

    /// Execution timeout in milliseconds
    pub timeout_ms: u64,

    /// Enable debug mode
    pub debug: bool,

    /// Additional options
    pub additional_options: HashMap<String, serde_json::Value>,
}

impl Default for ExecutionOptions {
    fn default() -> Self {
        Self {
            mode: ExecutionMode::Sync,
            memory_protection: MemoryProtection::EncryptionAndIntegrity,
            memory_limit_mb: 128,
            timeout_ms: 30000,
            debug: false,
            additional_options: HashMap::new(),
        }
    }
}

/// TEE execution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStats {
    /// Execution time in milliseconds
    pub execution_time_ms: u64,

    /// Memory usage in MB
    pub memory_usage_mb: u32,

    /// CPU usage in percentage
    pub cpu_usage_percent: f32,

    /// I/O operations
    pub io_operations: u32,

    /// Network operations
    pub network_operations: u32,
}

/// TEE key type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyType {
    /// Symmetric key
    Symmetric,

    /// Asymmetric key (private)
    AsymmetricPrivate,

    /// Asymmetric key (public)
    AsymmetricPublic,
}

/// TEE key usage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyUsage {
    /// Encryption
    Encryption,

    /// Decryption
    Decryption,

    /// Signing
    Signing,

    /// Verification
    Verification,

    /// Key wrapping
    KeyWrapping,

    /// Key unwrapping
    KeyUnwrapping,
}

/// TEE key metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    /// Key ID
    pub id: String,

    /// Key type
    pub key_type: KeyType,

    /// Key usage
    pub usage: Vec<KeyUsage>,

    /// Key algorithm
    pub algorithm: String,

    /// Key size in bits
    pub size: u32,

    /// Key creation timestamp
    pub created_at: u64,

    /// Key expiration timestamp
    pub expires_at: Option<u64>,

    /// Key is exportable
    pub exportable: bool,
}

/// TEE attestation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttestationType {
    /// Local attestation
    Local,

    /// Remote attestation
    Remote,
}

/// TEE attestation options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationOptions {
    /// Attestation type
    pub attestation_type: AttestationType,

    /// Include platform data
    pub include_platform_data: bool,

    /// Include user data
    pub user_data: Option<Vec<u8>>,

    /// Nonce for freshness
    pub nonce: Option<Vec<u8>>,
}

impl Default for AttestationOptions {
    fn default() -> Self {
        Self {
            attestation_type: AttestationType::Remote,
            include_platform_data: true,
            user_data: None,
            nonce: None,
        }
    }
}

/// Neo N3 specific TEE request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoTeeRequest {
    /// Script hash of the contract
    pub script_hash: String,

    /// Operation to invoke
    pub operation: String,

    /// Arguments for the operation
    pub args: Vec<serde_json::Value>,

    /// Signer account
    pub signer: Option<String>,

    /// Gas for execution
    pub gas: Option<u64>,

    /// System fee
    pub system_fee: Option<u64>,

    /// Network fee
    pub network_fee: Option<u64>,
}

/// Neo N3 specific TEE response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoTeeResponse {
    /// Transaction hash
    pub tx_hash: Option<String>,

    /// Execution result
    pub result: serde_json::Value,

    /// VM state
    pub vm_state: String,

    /// Gas consumed
    pub gas_consumed: u64,

    /// Exception message (if any)
    pub exception: Option<String>,

    /// Stack items
    pub stack: Vec<serde_json::Value>,
}
