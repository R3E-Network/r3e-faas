// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

pub mod attestation;
pub mod enclave;
pub mod key_management;
pub mod provider;
pub mod service;
pub mod types;

/// TEE service error types
#[derive(Debug, Error)]
pub enum TeeError {
    #[error("initialization error: {0}")]
    Initialization(String),
    
    #[error("attestation error: {0}")]
    Attestation(String),
    
    #[error("key management error: {0}")]
    KeyManagement(String),
    
    #[error("enclave error: {0}")]
    Enclave(String),
    
    #[error("provider error: {0}")]
    Provider(String),
    
    #[error("validation error: {0}")]
    Validation(String),
    
    #[error("execution error: {0}")]
    Execution(String),
    
    #[error("internal error: {0}")]
    Internal(String),
}

/// TEE platform type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeePlatform {
    /// Intel SGX
    Sgx,
    
    /// AMD SEV
    Sev,
    
    /// ARM TrustZone
    TrustZone,
    
    /// AWS Nitro Enclaves
    Nitro,
    
    /// Google Cloud Confidential Computing
    GoogleConfidential,
    
    /// Azure Confidential Computing
    AzureConfidential,
    
    /// Simulated TEE (for development and testing)
    Simulated,
}

/// TEE security level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeeSecurityLevel {
    /// Debug mode (not secure for production)
    Debug,
    
    /// Pre-production mode (limited security)
    PreProduction,
    
    /// Production mode (full security)
    Production,
}

/// TEE attestation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationReport {
    /// TEE platform
    pub platform: TeePlatform,
    
    /// TEE security level
    pub security_level: TeeSecurityLevel,
    
    /// Measurement of the enclave code (MRENCLAVE for SGX)
    pub code_hash: String,
    
    /// Measurement of the enclave signer (MRSIGNER for SGX)
    pub signer_hash: String,
    
    /// Product ID
    pub product_id: u16,
    
    /// Security version number
    pub security_version: u16,
    
    /// Attributes
    pub attributes: u64,
    
    /// Extended product ID
    pub extended_product_id: Vec<u8>,
    
    /// Signature
    pub signature: Vec<u8>,
    
    /// Additional platform-specific data
    pub platform_data: serde_json::Value,
}

/// TEE execution request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeeExecutionRequest {
    /// Unique request ID
    pub id: String,
    
    /// Function code to execute
    pub code: String,
    
    /// Function input data
    pub input: serde_json::Value,
    
    /// Required TEE platform (optional)
    pub platform: Option<TeePlatform>,
    
    /// Required security level (optional)
    pub security_level: Option<TeeSecurityLevel>,
    
    /// Required attestation (optional)
    pub require_attestation: bool,
    
    /// Execution timeout in milliseconds
    pub timeout_ms: u64,
    
    /// Memory limit in MB
    pub memory_limit_mb: u32,
}

/// TEE execution response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeeExecutionResponse {
    /// Request ID
    pub request_id: String,
    
    /// Execution result
    pub result: serde_json::Value,
    
    /// Attestation report (if requested)
    pub attestation: Option<AttestationReport>,
    
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    
    /// Memory usage in MB
    pub memory_usage_mb: u32,
    
    /// Error message (if any)
    pub error: Option<String>,
}

/// TEE service trait
#[async_trait::async_trait]
pub trait TeeService: Send + Sync {
    /// Get the supported TEE platforms
    fn supported_platforms(&self) -> Vec<TeePlatform>;
    
    /// Execute a function in the TEE
    async fn execute(&self, request: TeeExecutionRequest) -> Result<TeeExecutionResponse, TeeError>;
    
    /// Generate an attestation report
    async fn generate_attestation(&self, platform: TeePlatform) -> Result<AttestationReport, TeeError>;
    
    /// Verify an attestation report
    async fn verify_attestation(&self, attestation: &AttestationReport) -> Result<bool, TeeError>;
}

/// TEE provider trait
#[async_trait::async_trait]
pub trait TeeProvider: Send + Sync {
    /// Get the provider name
    fn name(&self) -> &str;
    
    /// Get the provider description
    fn description(&self) -> &str;
    
    /// Get the supported TEE platform
    fn platform(&self) -> TeePlatform;
    
    /// Initialize the TEE provider
    async fn initialize(&self) -> Result<(), TeeError>;
    
    /// Execute a function in the TEE
    async fn execute(&self, code: &str, input: &serde_json::Value) -> Result<serde_json::Value, TeeError>;
    
    /// Generate an attestation report
    async fn generate_attestation(&self) -> Result<AttestationReport, TeeError>;
    
    /// Verify an attestation report
    async fn verify_attestation(&self, attestation: &AttestationReport) -> Result<bool, TeeError>;
}
