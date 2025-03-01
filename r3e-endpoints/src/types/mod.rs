// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

/// Blockchain type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlockchainType {
    /// Neo N3 blockchain
    NeoN3,

    /// Ethereum blockchain
    Ethereum,
}

/// Signature curve
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SignatureCurve {
    /// secp256r1 curve (used by Neo N3)
    Secp256r1,

    /// secp256k1 curve (used by Ethereum)
    Secp256k1,
}

/// Wallet connection request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConnectionRequest {
    /// Blockchain type
    pub blockchain_type: BlockchainType,

    /// Wallet address
    pub address: String,

    /// Public key
    pub public_key: Option<String>,

    /// Signature curve
    pub signature_curve: SignatureCurve,

    /// Signature of the connection message
    pub signature: String,

    /// Message that was signed
    pub message: String,

    /// Timestamp
    pub timestamp: u64,
}

/// Wallet connection response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConnectionResponse {
    /// Connection ID
    pub connection_id: String,

    /// Blockchain type
    pub blockchain_type: BlockchainType,

    /// Wallet address
    pub address: String,

    /// JWT token
    pub token: String,

    /// Token expiration
    pub expires_at: u64,
}

/// Message signing request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSigningRequest {
    /// Connection ID
    pub connection_id: String,

    /// Message to sign
    pub message: String,

    /// Domain (for EIP-712 signatures)
    pub domain: Option<serde_json::Value>,

    /// Types (for EIP-712 signatures)
    pub types: Option<serde_json::Value>,

    /// Timestamp
    pub timestamp: u64,
}

/// Message signing response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSigningResponse {
    /// Request ID
    pub request_id: String,

    /// Message hash
    pub message_hash: String,

    /// Status
    pub status: String,

    /// Timestamp
    pub timestamp: u64,
}

mod validation;
pub use validation::*;

/// Service invocation request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ServiceInvocationRequest {
    /// Service ID
    #[validate(custom = "validate_uuid")]
    pub service_id: Uuid,

    /// Function name
    #[validate(custom = "validate_function_name")]
    pub function: String,

    /// Parameters
    pub params: serde_json::Value,

    /// Signature
    pub signature: Option<String>,

    /// Timestamp
    #[validate(custom = "validate_timestamp")]
    pub timestamp: u64,
}

/// Service invocation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInvocationResponse {
    /// Invocation ID
    pub invocation_id: String,

    /// Result
    pub result: serde_json::Value,

    /// Status
    pub status: String,

    /// Error
    pub error: Option<String>,

    /// Execution time (ms)
    pub execution_time_ms: u64,

    /// Timestamp
    pub timestamp: u64,
}

/// Meta transaction request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaTransactionRequest {
    /// Transaction data
    pub tx_data: String,

    /// Sender address
    pub sender: String,

    /// Signature
    pub signature: String,

    /// Nonce
    pub nonce: u64,

    /// Deadline
    pub deadline: u64,

    /// Blockchain type
    pub blockchain_type: BlockchainType,

    /// Target contract (for Ethereum transactions)
    pub target_contract: Option<String>,

    /// Signature curve
    pub signature_curve: SignatureCurve,

    /// Timestamp
    pub timestamp: u64,
}

/// Meta transaction response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaTransactionResponse {
    /// Request ID
    pub request_id: String,

    /// Original transaction hash
    pub original_hash: String,

    /// Relayed transaction hash
    pub relayed_hash: Option<String>,

    /// Status
    pub status: String,

    /// Error
    pub error: Option<String>,

    /// Timestamp
    pub timestamp: u64,
}
