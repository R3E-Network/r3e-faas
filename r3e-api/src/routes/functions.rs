// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::auth::Auth;
use crate::error::ApiError;
use crate::models::function::{
    CreateFunctionRequest, Function, FunctionInvocationRequest, FunctionInvocationResponse,
    FunctionLogsRequest, FunctionLogsResponse, FunctionStatus, UpdateFunctionRequest,
};
use crate::service::ApiService;

/// List functions query
#[derive(Debug, Deserialize)]
pub struct ListFunctionsQuery {
    /// Service ID
    pub service_id: Option<Uuid>,
    
    /// Status
    pub status: Option<String>,
    
    /// Trigger type
    pub trigger_type: Option<String>,
    
    /// Search query
    pub query: Option<String>,
    
    /// Limit
    pub limit: Option<u32>,
    
    /// Offset
    pub offset: Option<u32>,
}

/// List functions response
#[derive(Debug, Serialize)]
pub struct ListFunctionsResponse {
    /// Functions
    pub functions: Vec<Function>,
    
    /// Total count
    pub total_count: u32,
    
    /// Has more
    pub has_more: bool,
}

/// List functions handler
async fn list_functions(
    State(api_service): State<Arc<ApiService>>,
    auth: Auth,
    Query(query): Query<ListFunctionsQuery>,
) -> Result<Json<ListFunctionsResponse>, ApiError> {
    // Get the functions
    let (functions, total_count) = api_service
        .function_service
        .list_functions(
            auth.user.id,
            query.service_id,
            query.status.as_deref().map(|s| s.parse().ok()).flatten(),
            query.trigger_type.as_deref(),
            query.query.as_deref(),
            query.limit.unwrap_or(10),
            query.offset.unwrap_or(0),
        )
        .await?;
    
    // Check if there are more functions
    let has_more = total_count > (query.offset.unwrap_or(0) + query.limit.unwrap_or(10));
    
    // Return the response
    Ok(Json(ListFunctionsResponse {
        functions,
        total_count,
        has_more,
    }))
}

/// Get function handler
async fn get_function(
    State(api_service): State<Arc<ApiService>>,
    auth: Auth,
    Path(id): Path<Uuid>,
) -> Result<Json<Function>, ApiError> {
    // Get the function
    let function = api_service.function_service.get_function(id).await?;
    
    // Check if the user owns the function
    if function.user_id != auth.user.id {
        return Err(ApiError::Authorization(
            "You are not authorized to view this function".to_string(),
        ));
    }
    
    // Return the function
    Ok(Json(function))
}

/// Create function handler
async fn create_function(
    State(api_service): State<Arc<ApiService>>,
    auth: Auth,
    Json(request): Json<CreateFunctionRequest>,
) -> Result<Json<Function>, ApiError> {
    // Validate the request
    request.validate().map_err(|e| ApiError::Validation(e.to_string()))?;
    
    // Check if the user owns the service
    let service = api_service
        .service_service
        .get_service(request.service_id)
        .await?;
    
    if service.user_id != auth.user.id {
        return Err(ApiError::Authorization(
            "You are not authorized to create functions for this service".to_string(),
        ));
    }
    
    // Create the function
    let function = api_service
        .function_service
        .create_function(
            auth.user.id,
            request.service_id,
            &request.name,
            request.description.as_deref(),
            &request.code,
            request.runtime.unwrap_or_default(),
            request.trigger_type,
            &request.trigger_config,
            request.security_level.unwrap_or_default(),
        )
        .await?;
    
    // Return the function
    Ok(Json(function))
}

/// Update function handler
async fn update_function(
    State(api_service): State<Arc<ApiService>>,
    auth: Auth,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateFunctionRequest>,
) -> Result<Json<Function>, ApiError> {
    // Validate the request
    request.validate().map_err(|e| ApiError::Validation(e.to_string()))?;
    
    // Get the function
    let function = api_service.function_service.get_function(id).await?;
    
    // Check if the user owns the function
    if function.user_id != auth.user.id {
        return Err(ApiError::Authorization(
            "You are not authorized to update this function".to_string(),
        ));
    }
    
    // Update the function
    let function = api_service
        .function_service
        .update_function(
            id,
            request.name.as_deref(),
            request.description.as_deref(),
            request.code.as_deref(),
            request.runtime,
            request.trigger_type,
            request.trigger_config.as_ref(),
            request.security_level,
            request.status,
        )
        .await?;
    
    // Return the function
    Ok(Json(function))
}

/// Delete function handler
async fn delete_function(
    State(api_service): State<Arc<ApiService>>,
    auth: Auth,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, ApiError> {
    // Get the function
    let function = api_service.function_service.get_function(id).await?;
    
    // Check if the user owns the function
    if function.user_id != auth.user.id {
        return Err(ApiError::Authorization(
            "You are not authorized to delete this function".to_string(),
        ));
    }
    
    // Delete the function
    api_service.function_service.delete_function(id).await?;
    
    // Return success
    Ok(Json(()))
}

/// Invoke function handler
async fn invoke_function(
    State(api_service): State<Arc<ApiService>>,
    auth: Auth,
    Path(id): Path<Uuid>,
    Json(request): Json<FunctionInvocationRequest>,
) -> Result<Json<FunctionInvocationResponse>, ApiError> {
    // Get the function
    let function = api_service.function_service.get_function(id).await?;
    
    // Check if the function is active
    if function.status != FunctionStatus::Active {
        return Err(ApiError::Validation(
            "Function is not active".to_string(),
        ));
    }
    
    // Check if the user owns the function or the function's service is public
    let service = api_service
        .service_service
        .get_service(function.service_id)
        .await?;
    
    if function.user_id != auth.user.id && service.visibility != crate::models::service::ServiceVisibility::Public {
        return Err(ApiError::Authorization(
            "You are not authorized to invoke this function".to_string(),
        ));
    }
    
    // Invoke the function
    let response = api_service
        .function_service
        .invoke_function(id, &request.input)
        .await?;
    
    // Return the response
    Ok(Json(response))
}

/// Get function logs handler
async fn get_function_logs(
    State(api_service): State<Arc<ApiService>>,
    auth: Auth,
    Path(id): Path<Uuid>,
    Query(query): Query<FunctionLogsRequest>,
) -> Result<Json<FunctionLogsResponse>, ApiError> {
    // Get the function
    let function = api_service.function_service.get_function(id).await?;
    
    // Check if the user owns the function
    if function.user_id != auth.user.id {
        return Err(ApiError::Authorization(
            "You are not authorized to view logs for this function".to_string(),
        ));
    }
    
    // Get the logs
    let logs = api_service
        .function_service
        .get_function_logs(
            id,
            query.start_time,
            query.end_time,
            query.limit.unwrap_or(100),
            query.offset.unwrap_or(0),
        )
        .await?;
    
    // Return the logs
    Ok(Json(logs))
}

/// Function routes
pub fn function_routes(api_service: Arc<ApiService>) -> Router {
    Router::new()
        .route("/functions", get(list_functions))
        .route("/functions", post(create_function))
        .route("/functions/:id", get(get_function))
        .route("/functions/:id", post(update_function))
        .route("/functions/:id", axum::routing::delete(delete_function))
        .route("/functions/:id/invoke", post(invoke_function))
        .route("/functions/:id/logs", get(get_function_logs))
        .with_state(api_service)
}
