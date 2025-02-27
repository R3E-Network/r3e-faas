// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Abstract account policy type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyType {
    /// Single signature policy
    SingleSig,
    /// Multi-signature policy
    MultiSig,
    /// Threshold signature policy
    ThresholdSig,
    /// Time-locked policy
    TimeLocked,
    /// Custom policy
    Custom(String),
}

/// Abstract account policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountPolicy {
    /// Policy type
    pub policy_type: PolicyType,
    /// Policy parameters
    pub parameters: HashMap<String, String>,
    /// Required signatures
    pub required_signatures: u32,
    /// Total signatures
    pub total_signatures: u32,
    /// Time lock (if applicable)
    pub time_lock: Option<u64>,
    /// Custom policy script (if applicable)
    pub custom_script: Option<String>,
}

/// Abstract account operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountOperation {
    /// Transfer operation
    Transfer {
        /// Asset type
        asset: String,
        /// Recipient address
        to: String,
        /// Amount
        amount: String,
    },
    /// Contract invocation operation
    Invoke {
        /// Contract hash
        contract: String,
        /// Method name
        method: String,
        /// Parameters
        params: Vec<String>,
    },
    /// Add controller operation
    AddController {
        /// Controller address
        address: String,
        /// Controller weight
        weight: u32,
    },
    /// Remove controller operation
    RemoveController {
        /// Controller address
        address: String,
    },
    /// Update policy operation
    UpdatePolicy {
        /// New policy
        policy: AccountPolicy,
    },
    /// Recovery operation
    Recover {
        /// New owner address
        new_owner: String,
    },
    /// Custom operation
    Custom {
        /// Operation type
        operation_type: String,
        /// Operation data
        data: String,
    },
}

/// Abstract account operation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountOperationRequest {
    /// Account address
    pub account_address: String,
    /// Operation
    pub operation: AccountOperation,
    /// Signatures
    pub signatures: Vec<AccountSignature>,
    /// Nonce
    pub nonce: u64,
    /// Deadline (timestamp)
    pub deadline: u64,
    /// Timestamp
    pub timestamp: u64,
}

/// Abstract account operation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountOperationResponse {
    /// Request ID
    pub request_id: String,
    /// Account address
    pub account_address: String,
    /// Operation
    pub operation: AccountOperation,
    /// Transaction hash
    pub tx_hash: Option<String>,
    /// Status
    pub status: String,
    /// Error message
    pub error: Option<String>,
    /// Timestamp
    pub timestamp: u64,
}

/// Abstract account signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSignature {
    /// Signer address
    pub signer: String,
    /// Signature data
    pub signature: String,
    /// Signature type
    pub signature_type: String,
    /// Timestamp
    pub timestamp: u64,
}

/// Abstract account controller
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountController {
    /// Controller address
    pub address: String,
    /// Controller weight
    pub weight: u32,
    /// Controller type
    pub controller_type: String,
    /// Added timestamp
    pub added_at: u64,
    /// Status
    pub status: String,
}

/// Abstract account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbstractAccount {
    /// Account address
    pub address: String,
    /// Account owner
    pub owner: String,
    /// Account controllers
    pub controllers: Vec<AccountController>,
    /// Recovery addresses
    pub recovery_addresses: Vec<String>,
    /// Account policy
    pub policy: AccountPolicy,
    /// Account contract hash
    pub contract_hash: String,
    /// Account creation timestamp
    pub created_at: u64,
    /// Account status
    pub status: String,
    /// Account metadata
    pub metadata: HashMap<String, String>,
}

/// Abstract account creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountCreationRequest {
    /// Account owner
    pub owner: String,
    /// Account controllers
    pub controllers: Vec<AccountController>,
    /// Recovery addresses
    pub recovery_addresses: Vec<String>,
    /// Account policy
    pub policy: AccountPolicy,
    /// Account metadata
    pub metadata: HashMap<String, String>,
    /// Signature
    pub signature: String,
    /// Timestamp
    pub timestamp: u64,
}

/// Abstract account status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountStatus {
    /// Active
    Active,
    /// Locked
    Locked,
    /// Recovered
    Recovered,
    /// Disabled
    Disabled,
}

impl ToString for AccountStatus {
    fn to_string(&self) -> String {
        match self {
            AccountStatus::Active => "active".to_string(),
            AccountStatus::Locked => "locked".to_string(),
            AccountStatus::Recovered => "recovered".to_string(),
            AccountStatus::Disabled => "disabled".to_string(),
        }
    }
}

/// Abstract account operation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationStatus {
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

impl ToString for OperationStatus {
    fn to_string(&self) -> String {
        match self {
            OperationStatus::Pending => "pending".to_string(),
            OperationStatus::Submitted => "submitted".to_string(),
            OperationStatus::Confirmed => "confirmed".to_string(),
            OperationStatus::Failed => "failed".to_string(),
            OperationStatus::Expired => "expired".to_string(),
            OperationStatus::Rejected => "rejected".to_string(),
        }
    }
}

/// Abstract account operation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountOperationRecord {
    /// Request ID
    pub request_id: String,
    /// Account address
    pub account_address: String,
    /// Operation request
    pub request: AccountOperationRequest,
    /// Operation response
    pub response: Option<AccountOperationResponse>,
    /// Status
    pub status: OperationStatus,
    /// Created timestamp
    pub created_at: u64,
    /// Updated timestamp
    pub updated_at: u64,
}
