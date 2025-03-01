// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Auto contract error
#[derive(Debug, Error)]
pub enum AutoContractError {
    #[error("Invalid contract: {0}")]
    InvalidContract(String),

    #[error("Invalid trigger: {0}")]
    InvalidTrigger(String),

    #[error("Execution error: {0}")]
    Execution(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

/// Auto contract trigger type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AutoContractTriggerType {
    /// Blockchain event trigger
    Blockchain,

    /// Time-based trigger
    Time,

    /// Market price trigger
    Market,

    /// Custom event trigger
    Custom,
}

/// Auto contract trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoContractTrigger {
    /// Trigger ID
    pub id: String,

    /// Trigger type
    pub trigger_type: AutoContractTriggerType,

    /// Trigger parameters
    pub params: HashMap<String, serde_json::Value>,
}

/// Auto contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoContract {
    /// Contract ID
    pub id: String,

    /// User ID
    pub user_id: String,

    /// Contract name
    pub name: String,

    /// Contract description
    pub description: Option<String>,

    /// Contract network
    pub network: String,

    /// Contract address
    pub contract_address: String,

    /// Contract method
    pub method: String,

    /// Contract parameters
    pub params: Vec<serde_json::Value>,

    /// Contract trigger
    pub trigger: AutoContractTrigger,

    /// Created timestamp
    pub created_at: u64,

    /// Updated timestamp
    pub updated_at: u64,

    /// Last execution timestamp
    pub last_execution: Option<u64>,

    /// Execution count
    pub execution_count: u64,

    /// Enabled flag
    pub enabled: bool,
}

/// Auto contract execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoContractExecution {
    /// Execution ID
    pub id: String,

    /// Contract ID
    pub contract_id: String,

    /// Execution timestamp
    pub timestamp: u64,

    /// Transaction hash
    pub tx_hash: Option<String>,

    /// Execution status
    pub status: AutoContractExecutionStatus,

    /// Execution result
    pub result: Option<serde_json::Value>,

    /// Error message
    pub error: Option<String>,
}

/// Auto contract execution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AutoContractExecutionStatus {
    /// Execution pending
    Pending,

    /// Execution successful
    Success,

    /// Execution failed
    Failed,
}
