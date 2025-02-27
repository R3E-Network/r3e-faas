// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

/// Function trigger type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TriggerType {
    /// HTTP trigger
    Http,
    
    /// Schedule trigger
    Schedule,
    
    /// Blockchain event trigger
    BlockchainEvent,
    
    /// Oracle event trigger
    OracleEvent,
    
    /// Message queue trigger
    MessageQueue,
}

/// Function runtime
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Runtime {
    /// JavaScript runtime
    JavaScript,
    
    /// TypeScript runtime
    TypeScript,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::JavaScript
    }
}

/// Function security level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SecurityLevel {
    /// Standard security level
    Standard,
    
    /// TEE security level
    Tee,
}

impl Default for SecurityLevel {
    fn default() -> Self {
        Self::Standard
    }
}

/// Function status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FunctionStatus {
    /// Creating
    Creating,
    
    /// Active
    Active,
    
    /// Inactive
    Inactive,
    
    /// Error
    Error,
}

impl Default for FunctionStatus {
    fn default() -> Self {
        Self::Creating
    }
}

/// Function model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Function {
    /// Function ID
    pub id: Uuid,
    
    /// Service ID
    pub service_id: Uuid,
    
    /// User ID
    pub user_id: Uuid,
    
    /// Function name
    pub name: String,
    
    /// Function description
    pub description: Option<String>,
    
    /// Function code
    pub code: String,
    
    /// Function runtime
    pub runtime: Runtime,
    
    /// Function trigger type
    pub trigger_type: TriggerType,
    
    /// Function trigger configuration
    pub trigger_config: serde_json::Value,
    
    /// Function security level
    pub security_level: SecurityLevel,
    
    /// Function status
    pub status: FunctionStatus,
    
    /// Function version
    pub version: String,
    
    /// Function hash
    pub hash: String,
    
    /// Created at
    pub created_at: DateTime<Utc>,
    
    /// Updated at
    pub updated_at: DateTime<Utc>,
}

/// Create function request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateFunctionRequest {
    /// Service ID
    pub service_id: Uuid,
    
    /// Function name
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    
    /// Function description
    #[validate(length(min = 0, max = 500))]
    pub description: Option<String>,
    
    /// Function code
    #[validate(length(min = 1, max = 1000000))]
    pub code: String,
    
    /// Function runtime
    pub runtime: Option<Runtime>,
    
    /// Function trigger type
    pub trigger_type: TriggerType,
    
    /// Function trigger configuration
    pub trigger_config: serde_json::Value,
    
    /// Function security level
    pub security_level: Option<SecurityLevel>,
}

/// Update function request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateFunctionRequest {
    /// Function name
    #[validate(length(min = 3, max = 50))]
    pub name: Option<String>,
    
    /// Function description
    #[validate(length(min = 0, max = 500))]
    pub description: Option<String>,
    
    /// Function code
    #[validate(length(min = 1, max = 1000000))]
    pub code: Option<String>,
    
    /// Function runtime
    pub runtime: Option<Runtime>,
    
    /// Function trigger type
    pub trigger_type: Option<TriggerType>,
    
    /// Function trigger configuration
    pub trigger_config: Option<serde_json::Value>,
    
    /// Function security level
    pub security_level: Option<SecurityLevel>,
    
    /// Function status
    pub status: Option<FunctionStatus>,
}

/// Function invocation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInvocationRequest {
    /// Function ID
    pub function_id: Uuid,
    
    /// Invocation input
    pub input: serde_json::Value,
}

/// Function invocation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInvocationResponse {
    /// Invocation ID
    pub invocation_id: Uuid,
    
    /// Function ID
    pub function_id: Uuid,
    
    /// Invocation result
    pub result: serde_json::Value,
    
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    
    /// Invocation status
    pub status: String,
    
    /// Error message
    pub error: Option<String>,
}

/// Function logs request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionLogsRequest {
    /// Function ID
    pub function_id: Uuid,
    
    /// Start time
    pub start_time: Option<DateTime<Utc>>,
    
    /// End time
    pub end_time: Option<DateTime<Utc>>,
    
    /// Limit
    pub limit: Option<u32>,
    
    /// Offset
    pub offset: Option<u32>,
}

/// Function log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionLogEntry {
    /// Log ID
    pub id: Uuid,
    
    /// Function ID
    pub function_id: Uuid,
    
    /// Invocation ID
    pub invocation_id: Option<Uuid>,
    
    /// Log level
    pub level: String,
    
    /// Log message
    pub message: String,
    
    /// Log timestamp
    pub timestamp: DateTime<Utc>,
}

/// Function logs response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionLogsResponse {
    /// Log entries
    pub logs: Vec<FunctionLogEntry>,
    
    /// Total count
    pub total_count: u32,
    
    /// Has more
    pub has_more: bool,
}
