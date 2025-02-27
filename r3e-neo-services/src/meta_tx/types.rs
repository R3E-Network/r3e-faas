// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use serde::{Deserialize, Serialize};
use crate::types::FeeModel;

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
