// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use serde::{Deserialize, Serialize};
use crate::types::FeeModel;

/// Blockchain type for meta transactions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockchainType {
    /// Neo N3 blockchain
    #[serde(rename = "neo")]
    NeoN3,
    
    /// Ethereum blockchain
    #[serde(rename = "ethereum")]
    Ethereum,
}

impl Default for BlockchainType {
    fn default() -> Self {
        BlockchainType::NeoN3
    }
}

/// Signature curve type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignatureCurve {
    /// secp256r1 (used by Neo)
    #[serde(rename = "secp256r1")]
    Secp256r1,
    
    /// secp256k1 (used by Ethereum)
    #[serde(rename = "secp256k1")]
    Secp256k1,
}

impl Default for SignatureCurve {
    fn default() -> Self {
        SignatureCurve::Secp256r1
    }
}

/// Meta transaction request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaTxRequest {
    /// Transaction data (serialized transaction)
    pub tx_data: String,
    /// Sender address
    pub sender: String,
    /// Signature
    pub signature: String,
    /// Nonce
    pub nonce: u64,
    /// Deadline (timestamp)
    pub deadline: u64,
    /// Fee model
    pub fee_model: FeeModel,
    /// Fee amount
    pub fee_amount: u64,
    /// Timestamp
    pub timestamp: u64,
    /// Blockchain type (neo or ethereum)
    #[serde(default)]
    pub blockchain_type: BlockchainType,
    /// Signature curve (secp256r1 or secp256k1)
    #[serde(default)]
    pub signature_curve: SignatureCurve,
    /// Target contract address
    pub target_contract: Option<String>,
}

/// Meta transaction response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaTxResponse {
    /// Request ID
    pub request_id: String,
    /// Original transaction hash
    pub original_hash: String,
    /// Relayed transaction hash
    pub relayed_hash: Option<String>,
    /// Status
    pub status: String,
    /// Error message
    pub error: Option<String>,
    /// Timestamp
    pub timestamp: u64,
}

/// Meta transaction status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetaTxStatus {
    /// Pending
    Pending,
    /// Submitted
    Submitted,
    /// Confirmed
    Confirmed,
    /// Failed
    Failed,
    /// Expired
    Expired,
    /// Rejected
    Rejected,
}

impl ToString for MetaTxStatus {
    fn to_string(&self) -> String {
        match self {
            MetaTxStatus::Pending => "pending".to_string(),
            MetaTxStatus::Submitted => "submitted".to_string(),
            MetaTxStatus::Confirmed => "confirmed".to_string(),
            MetaTxStatus::Failed => "failed".to_string(),
            MetaTxStatus::Expired => "expired".to_string(),
            MetaTxStatus::Rejected => "rejected".to_string(),
        }
    }
}

/// Meta transaction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaTxRecord {
    /// Request ID
    pub request_id: String,
    /// Transaction request
    pub request: MetaTxRequest,
    /// Transaction response
    pub response: Option<MetaTxResponse>,
    /// Status
    pub status: MetaTxStatus,
    /// Created timestamp
    pub created_at: u64,
    /// Updated timestamp
    pub updated_at: u64,
}
