// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use deno_core::error::AnyError;
use deno_core::op2;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use r3e_oracle::service::create_oracle_request;
use r3e_oracle::types::{PriceRequest, PriceResponse, RandomMethod, RandomRequest, RandomResponse};
use r3e_oracle::{
    OracleError, OracleRequest, OracleRequestStatus, OracleRequestType, OracleResponse,
    OracleService,
};

// Oracle request operations

#[derive(Debug, Serialize, Deserialize)]
pub struct OracleRequestConfig {
    pub request_type: String,
    pub data: serde_json::Value,
    pub callback_url: Option<String>,
    pub requester_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OracleRequestResult {
    pub request_id: String,
}

#[op2]
#[serde]
pub fn op_oracle_submit_request(
    #[serde] config: OracleRequestConfig,
    #[state] oracle_service: &Arc<dyn OracleService>,
) -> Result<OracleRequestResult, AnyError> {
    // Convert request type string to enum
    let request_type = match config.request_type.as_str() {
        "price" => OracleRequestType::Price,
        "random" => OracleRequestType::Random,
        "weather" => OracleRequestType::Weather,
        "sports" => OracleRequestType::Sports,
        "custom" => OracleRequestType::Custom,
        _ => {
            return Err(AnyError::msg(format!(
                "Unsupported request type: {}",
                config.request_type
            )))
        }
    };

    // Convert data to string
    let data = serde_json::to_string(&config.data)
        .map_err(|e| AnyError::msg(format!("Failed to serialize request data: {}", e)))?;

    // Create oracle request
    let request =
        create_oracle_request(request_type, data, config.callback_url, config.requester_id);

    // Store request ID for response
    let request_id = request.id.clone();

    // Submit request
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        oracle_service
            .submit_request(request)
            .await
            .map_err(|e| AnyError::msg(format!("Failed to submit request: {}", e)))
    })?;

    Ok(OracleRequestResult { request_id })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OracleStatusResult {
    pub status: String,
}

#[op2]
#[serde]
pub fn op_oracle_get_request_status(
    #[string] request_id: String,
    #[state] oracle_service: &Arc<dyn OracleService>,
) -> Result<OracleStatusResult, AnyError> {
    // Get request status
    let rt = tokio::runtime::Runtime::new().unwrap();
    let status = rt.block_on(async {
        oracle_service
            .get_request_status(&request_id)
            .await
            .map_err(|e| AnyError::msg(format!("Failed to get request status: {}", e)))
    })?;

    // Convert status to string
    let status_str = match status {
        OracleRequestStatus::Pending => "pending",
        OracleRequestStatus::Processing => "processing",
        OracleRequestStatus::Completed => "completed",
        OracleRequestStatus::Failed => "failed",
    };

    Ok(OracleStatusResult {
        status: status_str.to_string(),
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OracleResponseResult {
    pub data: serde_json::Value,
    pub status_code: u32,
    pub timestamp: u64,
    pub error: Option<String>,
}

#[op2]
#[serde]
pub fn op_oracle_get_response(
    #[string] request_id: String,
    #[state] oracle_service: &Arc<dyn OracleService>,
) -> Result<OracleResponseResult, AnyError> {
    // Get response
    let rt = tokio::runtime::Runtime::new().unwrap();
    let response = rt.block_on(async {
        oracle_service
            .get_response(&request_id)
            .await
            .map_err(|e| AnyError::msg(format!("Failed to get response: {}", e)))
    })?;

    // Parse response data
    let data = serde_json::from_str(&response.data)
        .unwrap_or_else(|_| serde_json::Value::String(response.data.clone()));

    Ok(OracleResponseResult {
        data,
        status_code: response.status_code,
        timestamp: response.timestamp,
        error: response.error,
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OracleCancelResult {
    pub success: bool,
}

#[op2]
#[serde]
pub fn op_oracle_cancel_request(
    #[string] request_id: String,
    #[state] oracle_service: &Arc<dyn OracleService>,
) -> Result<OracleCancelResult, AnyError> {
    // Cancel request
    let rt = tokio::runtime::Runtime::new().unwrap();
    let success = rt.block_on(async {
        oracle_service
            .cancel_request(&request_id)
            .await
            .map_err(|e| AnyError::msg(format!("Failed to cancel request: {}", e)))
    })?;

    Ok(OracleCancelResult { success })
}

// Convenience operations for specific oracle services

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceRequestConfig {
    pub symbol: String,
    pub currency: Option<String>,
    pub sources: Option<Vec<String>>,
    pub requester_id: String,
}

#[op2]
#[serde]
pub fn op_oracle_get_price(
    #[serde] config: PriceRequestConfig,
    #[state] oracle_service: &Arc<dyn OracleService>,
) -> Result<OracleRequestResult, AnyError> {
    // Create price request
    let price_request = PriceRequest {
        symbol: config.symbol,
        currency: config.currency.unwrap_or_else(|| "USD".to_string()),
        sources: config.sources.unwrap_or_default(),
    };

    // Convert to JSON
    let data = serde_json::to_value(price_request)
        .map_err(|e| AnyError::msg(format!("Failed to serialize price request: {}", e)))?;

    // Create oracle request config
    let oracle_config = OracleRequestConfig {
        request_type: "price".to_string(),
        data,
        callback_url: None,
        requester_id: config.requester_id,
    };

    // Submit request
    op_oracle_submit_request(oracle_config, oracle_service)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomRequestConfig {
    pub min: Option<u64>,
    pub max: Option<u64>,
    pub count: Option<u32>,
    pub method: Option<String>,
    pub seed: Option<String>,
    pub requester_id: String,
}

#[op2]
#[serde]
pub fn op_oracle_get_random(
    #[serde] config: RandomRequestConfig,
    #[state] oracle_service: &Arc<dyn OracleService>,
) -> Result<OracleRequestResult, AnyError> {
    // Convert method string to enum
    let method = match config.method.as_deref() {
        Some("secure") => RandomMethod::Secure,
        Some("blockchain") => RandomMethod::Blockchain,
        Some("vrf") => RandomMethod::Vrf,
        None => RandomMethod::Secure,
        _ => {
            return Err(AnyError::msg(format!(
                "Unsupported random method: {}",
                config.method.unwrap_or_default()
            )))
        }
    };

    // Create random request
    let random_request = RandomRequest {
        min: config.min.unwrap_or(0),
        max: config.max.unwrap_or(u64::MAX),
        count: config.count.unwrap_or(1),
        method,
        seed: config.seed,
    };

    // Convert to JSON
    let data = serde_json::to_value(random_request)
        .map_err(|e| AnyError::msg(format!("Failed to serialize random request: {}", e)))?;

    // Create oracle request config
    let oracle_config = OracleRequestConfig {
        request_type: "random".to_string(),
        data,
        callback_url: None,
        requester_id: config.requester_id,
    };

    // Submit request
    op_oracle_submit_request(oracle_config, oracle_service)
}
