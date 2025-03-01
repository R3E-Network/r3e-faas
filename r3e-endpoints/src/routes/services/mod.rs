// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::Error, service::EndpointService, types::ServiceInvocationRequest,
    types::ServiceInvocationResponse, utils::verify_jwt_token,
};

/// Service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    /// Service ID
    pub id: Uuid,

    /// Service name
    pub name: String,

    /// Service description
    pub description: String,

    /// Service type
    pub service_type: String,

    /// Service version
    pub version: String,

    /// Service functions
    pub functions: Vec<ServiceFunction>,

    /// Created at
    pub created_at: u64,

    /// Updated at
    pub updated_at: u64,
}

/// Service function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceFunction {
    /// Function name
    pub name: String,

    /// Function description
    pub description: String,

    /// Function parameters
    pub parameters: Vec<ServiceFunctionParameter>,

    /// Function return type
    pub return_type: String,

    /// Function requires authentication
    pub requires_auth: bool,

    /// Function requires signature
    pub requires_signature: bool,
}

/// Service function parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceFunctionParameter {
    /// Parameter name
    pub name: String,

    /// Parameter description
    pub description: String,

    /// Parameter type
    pub parameter_type: String,

    /// Parameter is required
    pub required: bool,
}

/// List services handler
pub async fn list_services(
    State(service): State<Arc<EndpointService>>,
) -> Result<Json<Vec<Service>>, Error> {
    // Fetch services from the database
    let services = service
        .db_client
        .list_services()
        .await
        .map_err(|e| Error::Internal(format!("Database error: {}", e)))?;

    // If no services found, return empty array instead of error
    if services.is_empty() {
        log::info!("No services found in database");
    } else {
        log::info!("Found {} services", services.len());
    }

    Ok(Json(services))
}

/// Get service handler
pub async fn get_service(
    State(service): State<Arc<EndpointService>>,
    Path(id): Path<String>,
) -> Result<Json<Service>, Error> {
    // Parse the service ID
    let service_id = Uuid::parse_str(&id)
        .map_err(|e| Error::Validation(format!("Invalid service ID: {}", e)))?;

    // Fetch the service from the database
    let service_data = service
        .db_client
        .get_service(&service_id)
        .await
        .map_err(|e| Error::Internal(format!("Database error: {}", e)))?;

    // Check if service exists
    let service_data = match service_data {
        Some(service) => service,
        None => {
            log::warn!("Service not found: {}", service_id);
            return Err(Error::NotFound(format!(
                "Service not found: {}",
                service_id
            )));
        }
    };

    // Check if service is enabled
    if !service_data.is_enabled {
        log::warn!("Service is disabled: {}", service_id);
        return Err(Error::Forbidden("Service is disabled".into()));
    }

    log::info!("Service found: {} ({})", service_data.name, service_id);
    Ok(Json(service_data))
}

/// Invoke service handler
pub async fn invoke_service(
    State(service): State<Arc<EndpointService>>,
    Path(id): Path<String>,
    Json(request): Json<ServiceInvocationRequest>,
) -> Result<Json<ServiceInvocationResponse>, Error> {
    // Parse the service ID
    let service_id = Uuid::parse_str(&id)
        .map_err(|e| Error::Validation(format!("Invalid service ID: {}", e)))?;

    // Fetch the service from the database
    let service_data = service
        .db_client
        .get_service(&service_id)
        .await
        .map_err(|e| Error::Internal(format!("Database error: {}", e)))?;

    // Check if service exists
    let service_data = match service_data {
        Some(s) => s,
        None => {
            log::warn!("Service not found: {}", service_id);
            return Err(Error::NotFound(format!(
                "Service not found: {}",
                service_id
            )));
        }
    };

    // Check if service is enabled
    if !service_data.is_enabled {
        log::warn!("Service is disabled: {}", service_id);
        return Err(Error::Forbidden("Service is disabled".into()));
    }

    // Find the function in the service
    let function = service_data
        .functions
        .iter()
        .find(|f| f.name == request.function);

    // Check if function exists
    let function = match function {
        Some(f) => f,
        None => {
            log::warn!("Function not found: {}.{}", service_id, request.function);
            return Err(Error::NotFound(format!(
                "Function not found: {}.{}",
                service_id, request.function
            )));
        }
    };

    // Check function auth requirements
    if function.requires_auth && request.auth_token.is_none() {
        log::warn!(
            "Auth token required for function: {}.{}",
            service_id,
            request.function
        );
        return Err(Error::Authentication("Auth token required".into()));
    }

    // Verify authentication if present
    if let Some(token) = &request.auth_token {
        match crate::utils::verify_jwt_token(token, &service.config.jwt_secret) {
            Ok(_) => {
                log::debug!(
                    "Auth token verified for function: {}.{}",
                    service_id,
                    request.function
                );
            }
            Err(e) => {
                log::warn!(
                    "Invalid auth token for function: {}.{}: {}",
                    service_id,
                    request.function,
                    e
                );
                return Err(Error::Authentication("Invalid auth token".into()));
            }
        }
    }

    // Check signature requirements
    if function.requires_signature && request.signature.is_none() {
        log::warn!(
            "Signature required for function: {}.{}",
            service_id,
            request.function
        );
        return Err(Error::Authentication("Signature required".into()));
    }

    // Verify signature if present
    if let Some(sig_data) = &request.signature {
        let signature_valid = crate::utils::verify_signature(
            &sig_data.blockchain_type,
            &sig_data.signature_curve,
            &sig_data.address,
            &serde_json::to_string(&request.parameters).unwrap_or_default(),
            &sig_data.signature,
        )?;

        if !signature_valid {
            log::warn!(
                "Invalid signature for function: {}.{}",
                service_id,
                request.function
            );
            return Err(Error::Authentication("Invalid signature".into()));
        }
    }

    // Validate parameters
    for param_def in &function.parameters {
        if param_def.required {
            if !request.parameters.contains_key(&param_def.name) {
                log::warn!(
                    "Required parameter missing: {}.{}.{}",
                    service_id,
                    request.function,
                    param_def.name
                );
                return Err(Error::Validation(format!(
                    "Required parameter missing: {}",
                    param_def.name
                )));
            }
        }
    }

    // Generate invocation ID
    let invocation_id = Uuid::new_v4().to_string();

    // Record start time
    let start_time = Utc::now().timestamp_millis();

    // Execute the service function
    let result = match service
        .service_registry
        .invoke_service(
            &service_id,
            &request.function,
            &request.parameters,
            request.auth_token.as_deref(),
            request.signature.as_ref(),
        )
        .await
    {
        Ok(result) => {
            log::info!(
                "Service function executed successfully: {}.{} ({})",
                service_id,
                request.function,
                invocation_id
            );
            result
        }
        Err(e) => {
            log::error!(
                "Service function execution failed: {}.{} ({}): {}",
                service_id,
                request.function,
                invocation_id,
                e
            );

            // Record the invocation in the database as failed
            let _ = service
                .db_client
                .record_service_invocation(
                    &invocation_id,
                    &service_id,
                    &request.function,
                    &request.parameters,
                    None,
                    Some(&e.to_string()),
                    "error",
                    start_time,
                    Utc::now().timestamp_millis(),
                )
                .await;

            return Err(Error::Internal(format!("Service execution failed: {}", e)));
        }
    };

    // Calculate execution time
    let end_time = Utc::now().timestamp_millis();
    let execution_time = end_time - start_time;

    // Record the invocation in the database
    let _ = service
        .db_client
        .record_service_invocation(
            &invocation_id,
            &service_id,
            &request.function,
            &request.parameters,
            Some(&result),
            None,
            "success",
            start_time,
            end_time,
        )
        .await;

    // Construct response
    let response = ServiceInvocationResponse {
        invocation_id,
        result,
        status: "success".to_string(),
        error: None,
        execution_time_ms: execution_time as u64,
        timestamp: Utc::now().timestamp() as u64,
    };

    Ok(Json(response))
}
