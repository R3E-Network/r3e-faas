// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trigger source types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TriggerSource {
    /// Blockchain event trigger
    Blockchain,

    /// Time-based trigger
    Time,

    /// Market price trigger
    Market,

    /// Custom event trigger
    Custom,
}

/// Trigger condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerCondition {
    /// Trigger source
    pub source: TriggerSource,

    /// Condition parameters
    pub params: HashMap<String, serde_json::Value>,
}

/// Blockchain trigger parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainTriggerParams {
    /// Blockchain network (e.g., "neo_n3", "ethereum")
    pub network: String,

    /// Contract address
    pub contract_address: Option<String>,

    /// Event name
    pub event_name: Option<String>,

    /// Method name
    pub method_name: Option<String>,

    /// Block number
    pub block_number: Option<u64>,
}

/// Time trigger parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeTriggerParams {
    /// Cron expression
    pub cron: String,

    /// Timezone
    pub timezone: Option<String>,
}

/// Market trigger parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTriggerParams {
    /// Asset pair (e.g., "NEO/USD")
    pub asset_pair: String,

    /// Condition type (e.g., "above", "below")
    pub condition: String,

    /// Price threshold
    pub price: f64,
}

/// Custom trigger parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTriggerParams {
    /// Custom event name
    pub event_name: String,

    /// Custom event data
    pub event_data: Option<serde_json::Value>,
}

/// Trigger error
#[derive(Debug, thiserror::Error)]
pub enum TriggerError {
    #[error("Invalid trigger source: {0}")]
    InvalidSource(String),

    #[error("Invalid trigger parameters: {0}")]
    InvalidParameters(String),

    #[error("Trigger evaluation error: {0}")]
    EvaluationError(String),

    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Callback execution error: {0}")]
    CallbackExecution(String),
}
