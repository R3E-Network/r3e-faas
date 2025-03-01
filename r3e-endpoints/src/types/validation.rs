// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use regex::Regex;
use serde_json::Value;
use uuid::Uuid;
use validator::{Validate, ValidationError, ValidationErrors};

use crate::types::{
    BlockchainType, MessageSigningRequest, MetaTransactionRequest, ServiceInvocationRequest,
    WalletConnectionRequest,
};

/// Validate a UUID string
pub fn validate_uuid(uuid_str: &str) -> Result<(), ValidationError> {
    match Uuid::parse_str(uuid_str) {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("invalid_uuid")),
    }
}

/// Validate a function name
pub fn validate_function_name(name: &str) -> Result<(), ValidationError> {
    // Function name should be alphanumeric with underscores and hyphens
    let pattern = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    if !pattern.is_match(name) {
        return Err(ValidationError::new("invalid_function_name"));
    }

    // Function name should be between 1 and 64 characters
    if name.is_empty() || name.len() > 64 {
        return Err(ValidationError::new("function_name_length"));
    }

    Ok(())
}

/// Validate a timestamp
pub fn validate_timestamp(timestamp: &u64) -> Result<(), ValidationError> {
    // Timestamp should be within a reasonable range
    let current_time = chrono::Utc::now().timestamp() as u64;

    // Check if timestamp is too far in the past (more than 1 hour)
    if *timestamp < current_time.saturating_sub(3600) {
        return Err(ValidationError::new("timestamp_too_old"));
    }

    // Check if timestamp is too far in the future (more than 5 minutes)
    if *timestamp > current_time + 300 {
        return Err(ValidationError::new("timestamp_in_future"));
    }

    Ok(())
}

/// Validate a blockchain address
pub fn validate_blockchain_address(
    address: &str,
    blockchain_type: &BlockchainType,
) -> Result<(), ValidationError> {
    match blockchain_type {
        BlockchainType::NeoN3 => {
            // Neo N3 addresses are 34 characters long and start with 'N'
            if !address.starts_with('N') || address.len() != 34 {
                return Err(ValidationError::new("invalid_neo_address"));
            }

            // Check if address contains only valid characters
            let pattern = Regex::new(r"^[A-Za-z0-9]+$").unwrap();
            if !pattern.is_match(address) {
                return Err(ValidationError::new("invalid_neo_address_chars"));
            }
        }
        BlockchainType::Ethereum => {
            // Ethereum addresses are 42 characters long and start with '0x'
            if !address.starts_with("0x") || address.len() != 42 {
                return Err(ValidationError::new("invalid_ethereum_address"));
            }

            // Check if address contains only valid hex characters
            let pattern = Regex::new(r"^0x[0-9a-fA-F]{40}$").unwrap();
            if !pattern.is_match(address) {
                return Err(ValidationError::new("invalid_ethereum_address_chars"));
            }
        }
    }

    Ok(())
}

/// Validate a signature
pub fn validate_signature(signature: &str) -> Result<(), ValidationError> {
    // Signatures should be hex-encoded
    let pattern = Regex::new(r"^(0x)?[0-9a-fA-F]+$").unwrap();
    if !pattern.is_match(signature) {
        return Err(ValidationError::new("invalid_signature_format"));
    }

    // Signatures should be of reasonable length
    if signature.len() < 64 || signature.len() > 256 {
        return Err(ValidationError::new("invalid_signature_length"));
    }

    Ok(())
}

/// Validate JSON parameters
pub fn validate_json_params(params: &Value) -> Result<(), ValidationError> {
    // Check if params is an object
    if !params.is_object() {
        return Err(ValidationError::new("params_not_object"));
    }

    // Check for potential injection attacks
    let params_str = params.to_string();

    // Check for suspicious patterns
    let suspicious_patterns = [
        r"__proto__",
        r"constructor\s*\(",
        r"eval\s*\(",
        r"setTimeout\s*\(",
        r"setInterval\s*\(",
        r"Function\s*\(",
        r"<script",
        r"javascript:",
    ];

    for pattern in suspicious_patterns {
        let regex = Regex::new(pattern).unwrap();
        if regex.is_match(&params_str) {
            return Err(ValidationError::new("suspicious_params_content"));
        }
    }

    // Check for deeply nested objects (potential DoS attack)
    fn check_nesting_depth(value: &Value, current_depth: usize, max_depth: usize) -> bool {
        if current_depth > max_depth {
            return false;
        }

        match value {
            Value::Object(obj) => {
                for (_, v) in obj {
                    if !check_nesting_depth(v, current_depth + 1, max_depth) {
                        return false;
                    }
                }
            }
            Value::Array(arr) => {
                for v in arr {
                    if !check_nesting_depth(v, current_depth + 1, max_depth) {
                        return false;
                    }
                }
            }
            _ => {}
        }

        true
    }

    if !check_nesting_depth(params, 0, 10) {
        return Err(ValidationError::new("params_too_deeply_nested"));
    }

    Ok(())
}

/// Validate a service invocation request
pub fn validate_service_invocation_request(
    request: &ServiceInvocationRequest,
) -> Result<(), ValidationErrors> {
    let mut errors = ValidationErrors::new();

    // Validate service ID
    if let Err(e) = validate_uuid(&request.service_id.to_string()) {
        errors.add("service_id", e);
    }

    // Validate function name
    if let Err(e) = validate_function_name(&request.function) {
        errors.add("function", e);
    }

    // Validate parameters
    if let Err(e) = validate_json_params(&request.params) {
        errors.add("params", e);
    }

    // Validate timestamp
    if let Err(e) = validate_timestamp(&request.timestamp) {
        errors.add("timestamp", e);
    }

    // Validate signature if present
    if let Some(signature) = &request.signature {
        if let Err(e) = validate_signature(signature) {
            errors.add("signature", e);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate a wallet connection request
pub fn validate_wallet_connection_request(
    request: &WalletConnectionRequest,
) -> Result<(), ValidationErrors> {
    let mut errors = ValidationErrors::new();

    // Validate blockchain address
    if let Err(e) = validate_blockchain_address(&request.address, &request.blockchain_type) {
        errors.add("address", e);
    }

    // Validate signature
    if let Err(e) = validate_signature(&request.signature) {
        errors.add("signature", e);
    }

    // Validate message
    if request.message.is_empty() {
        errors.add("message", ValidationError::new("message_empty"));
    }

    // Validate timestamp
    if let Err(e) = validate_timestamp(&request.timestamp) {
        errors.add("timestamp", e);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate a message signing request
pub fn validate_message_signing_request(
    request: &MessageSigningRequest,
) -> Result<(), ValidationErrors> {
    let mut errors = ValidationErrors::new();

    // Validate connection ID
    if request.connection_id.is_empty() {
        errors.add("connection_id", ValidationError::new("connection_id_empty"));
    }

    // Validate message
    if request.message.is_empty() {
        errors.add("message", ValidationError::new("message_empty"));
    }

    // Validate timestamp
    if let Err(e) = validate_timestamp(&request.timestamp) {
        errors.add("timestamp", e);
    }

    // Validate domain and types if present (for EIP-712 signatures)
    if let Some(domain) = &request.domain {
        if !domain.is_object() {
            errors.add("domain", ValidationError::new("domain_not_object"));
        }
    }

    if let Some(types) = &request.types {
        if !types.is_object() {
            errors.add("types", ValidationError::new("types_not_object"));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate a meta transaction request
pub fn validate_meta_transaction_request(
    request: &MetaTransactionRequest,
) -> Result<(), ValidationErrors> {
    let mut errors = ValidationErrors::new();

    // Validate transaction data
    if request.tx_data.is_empty() {
        errors.add("tx_data", ValidationError::new("tx_data_empty"));
    }

    // Validate sender address
    if let Err(e) = validate_blockchain_address(&request.sender, &request.blockchain_type) {
        errors.add("sender", e);
    }

    // Validate signature
    if let Err(e) = validate_signature(&request.signature) {
        errors.add("signature", e);
    }

    // Validate deadline
    let current_time = chrono::Utc::now().timestamp() as u64;
    if request.deadline <= current_time {
        errors.add("deadline", ValidationError::new("deadline_expired"));
    }

    // Validate timestamp
    if let Err(e) = validate_timestamp(&request.timestamp) {
        errors.add("timestamp", e);
    }

    // Validate target contract if present (for Ethereum transactions)
    if let Some(target_contract) = &request.target_contract {
        if request.blockchain_type == BlockchainType::Ethereum {
            if let Err(e) = validate_blockchain_address(target_contract, &BlockchainType::Ethereum)
            {
                errors.add("target_contract", e);
            }
        } else {
            errors.add(
                "target_contract",
                ValidationError::new("target_contract_not_applicable"),
            );
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Middleware for validating requests
pub mod middleware {
    use axum::{
        extract::rejection::JsonRejection,
        http::StatusCode,
        response::{IntoResponse, Response},
        Json,
    };
    use serde_json::json;
    use validator::ValidationErrors;

    /// Convert validation errors to a user-friendly response
    pub fn validation_errors_to_response(errors: ValidationErrors) -> Response {
        let error_map = errors
            .field_errors()
            .iter()
            .map(|(field, errors)| {
                let error_messages: Vec<String> = errors
                    .iter()
                    .map(|error| error.message.clone().unwrap_or_else(|| error.code.clone()))
                    .collect();
                (*field, error_messages)
            })
            .collect::<serde_json::Map<_, _>>();

        (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Validation Error",
                "details": error_map
            })),
        )
            .into_response()
    }

    /// Handle JSON extraction errors
    pub fn handle_json_extraction_error(error: JsonRejection) -> Response {
        let status = StatusCode::BAD_REQUEST;
        let body = Json(json!({
            "error": "Invalid JSON",
            "details": error.to_string()
        }));

        (status, body).into_response()
    }
}
