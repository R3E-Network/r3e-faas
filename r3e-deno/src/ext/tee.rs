// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use deno_core::error::AnyError;
use deno_core::op2;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use r3e_tee::service::create_default_neo_tee_service;
use r3e_tee::types::{ExecutionOptions, NeoTeeRequest, NeoTeeResponse};
use r3e_tee::{
    AttestationReport, TeeError, TeeExecutionRequest, TeeExecutionResponse, TeePlatform,
    TeeSecurityLevel, TeeService,
};

// TEE execution operations

#[derive(Debug, Serialize, Deserialize)]
pub struct TeeExecutionConfig {
    pub id: String,
    pub code: String,
    pub input: serde_json::Value,
    pub platform: Option<String>,
    pub security_level: Option<String>,
    pub require_attestation: bool,
    pub timeout_ms: u64,
    pub memory_limit_mb: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeeExecutionResult {
    pub request_id: String,
    pub result: serde_json::Value,
    pub attestation: Option<AttestationReport>,
    pub execution_time_ms: u64,
    pub memory_usage_mb: u32,
    pub error: Option<String>,
}

#[op2]
#[serde]
pub fn op_tee_execute(
    #[serde] config: TeeExecutionConfig,
    #[state] tee_service: &Arc<dyn TeeService>,
) -> Result<TeeExecutionResult, AnyError> {
    // Convert platform string to enum
    let platform = match config.platform.as_deref() {
        Some("sgx") => Some(TeePlatform::Sgx),
        Some("sev") => Some(TeePlatform::Sev),
        Some("trustzone") => Some(TeePlatform::TrustZone),
        Some("simulated") => Some(TeePlatform::Simulated),
        None => None,
        _ => {
            return Err(AnyError::msg(format!(
                "Unsupported TEE platform: {}",
                config.platform.unwrap_or_default()
            )))
        }
    };

    // Convert security level string to enum
    let security_level = match config.security_level.as_deref() {
        Some("debug") => Some(TeeSecurityLevel::Debug),
        Some("preproduction") => Some(TeeSecurityLevel::PreProduction),
        Some("production") => Some(TeeSecurityLevel::Production),
        None => None,
        _ => {
            return Err(AnyError::msg(format!(
                "Unsupported TEE security level: {}",
                config.security_level.unwrap_or_default()
            )))
        }
    };

    // Create execution request
    let request = TeeExecutionRequest {
        id: config.id,
        code: config.code,
        input: config.input,
        platform,
        security_level,
        require_attestation: config.require_attestation,
        timeout_ms: config.timeout_ms,
        memory_limit_mb: config.memory_limit_mb,
    };

    // Execute the request
    let rt = tokio::runtime::Runtime::new().unwrap();
    let response = rt.block_on(async {
        tee_service
            .execute(request)
            .await
            .map_err(|e| AnyError::msg(format!("Failed to execute TEE request: {}", e)))
    })?;

    // Convert response to result
    let result = TeeExecutionResult {
        request_id: response.request_id,
        result: response.result,
        attestation: response.attestation,
        execution_time_ms: response.execution_time_ms,
        memory_usage_mb: response.memory_usage_mb,
        error: response.error,
    };

    Ok(result)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeeAttestationConfig {
    pub platform: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeeAttestationResult {
    pub attestation: AttestationReport,
}

#[op2]
#[serde]
pub fn op_tee_generate_attestation(
    #[serde] config: TeeAttestationConfig,
    #[state] tee_service: &Arc<dyn TeeService>,
) -> Result<TeeAttestationResult, AnyError> {
    // Convert platform string to enum
    let platform = match config.platform.as_str() {
        "sgx" => TeePlatform::Sgx,
        "sev" => TeePlatform::Sev,
        "trustzone" => TeePlatform::TrustZone,
        "simulated" => TeePlatform::Simulated,
        _ => {
            return Err(AnyError::msg(format!(
                "Unsupported TEE platform: {}",
                config.platform
            )))
        }
    };

    // Generate attestation
    let rt = tokio::runtime::Runtime::new().unwrap();
    let attestation = rt.block_on(async {
        tee_service
            .generate_attestation(platform)
            .await
            .map_err(|e| AnyError::msg(format!("Failed to generate attestation: {}", e)))
    })?;

    Ok(TeeAttestationResult { attestation })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeeVerifyAttestationConfig {
    pub attestation: AttestationReport,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeeVerifyAttestationResult {
    pub is_valid: bool,
}

#[op2]
#[serde]
pub fn op_tee_verify_attestation(
    #[serde] config: TeeVerifyAttestationConfig,
    #[state] tee_service: &Arc<dyn TeeService>,
) -> Result<TeeVerifyAttestationResult, AnyError> {
    // Verify attestation
    let rt = tokio::runtime::Runtime::new().unwrap();
    let is_valid = rt.block_on(async {
        tee_service
            .verify_attestation(&config.attestation)
            .await
            .map_err(|e| AnyError::msg(format!("Failed to verify attestation: {}", e)))
    })?;

    Ok(TeeVerifyAttestationResult { is_valid })
}

// Neo N3 specific TEE operations

#[derive(Debug, Serialize, Deserialize)]
pub struct NeoTeeExecutionConfig {
    pub script_hash: String,
    pub operation: String,
    pub args: Vec<serde_json::Value>,
    pub signer: Option<String>,
    pub gas: Option<u64>,
    pub system_fee: Option<u64>,
    pub network_fee: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeoTeeExecutionResult {
    pub tx_hash: Option<String>,
    pub result: serde_json::Value,
    pub vm_state: String,
    pub gas_consumed: u64,
    pub exception: Option<String>,
    pub stack: Vec<serde_json::Value>,
}

#[op2]
#[serde]
pub fn op_neo_tee_execute(
    #[serde] config: NeoTeeExecutionConfig,
    #[state] tee_service: &Arc<dyn TeeService>,
) -> Result<NeoTeeExecutionResult, AnyError> {
    // Check if the service is a Neo TEE service
    let neo_tee_service = match tee_service.downcast_ref::<r3e_tee::service::NeoTeeService>() {
        Some(service) => service,
        None => {
            return Err(AnyError::msg(
                "The provided TEE service is not a Neo TEE service",
            ))
        }
    };

    // Create Neo TEE request
    let request = NeoTeeRequest {
        script_hash: config.script_hash,
        operation: config.operation,
        args: config.args,
        signer: config.signer,
        gas: config.gas,
        system_fee: config.system_fee,
        network_fee: config.network_fee,
    };

    // Execute the request
    let rt = tokio::runtime::Runtime::new().unwrap();
    let response = rt.block_on(async {
        neo_tee_service
            .execute_neo_request(&request)
            .await
            .map_err(|e| AnyError::msg(format!("Failed to execute Neo TEE request: {}", e)))
    })?;

    // Convert response to result
    let result = NeoTeeExecutionResult {
        tx_hash: response.tx_hash,
        result: response.result,
        vm_state: response.vm_state,
        gas_consumed: response.gas_consumed,
        exception: response.exception,
        stack: response.stack,
    };

    Ok(result)
}
