// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::auth::Auth;
use crate::error::ApiError;
use crate::models::service::{
    CreateServiceRequest, Service, ServiceDiscoveryRequest, ServiceDiscoveryResponse,
    ServiceListRequest, ServiceListResponse, ServiceStatus, ServiceSummary, UpdateServiceRequest,
};
use crate::service::ApiService;

/// List services handler
async fn list_services(
    State(api_service): State<Arc<ApiService>>,
    auth: Auth,
    Query(query): Query<ServiceListRequest>,
) -> Result<Json<ServiceListResponse>, ApiError> {
    // Get the services
    let (services, total_count) = api_service
        .service_service
        .list_services(
            query.user_id.unwrap_or(auth.user.id),
            query.service_type,
            query.status,
            query.visibility,
            query.query.as_deref(),
            query.limit.unwrap_or(10),
            query.offset.unwrap_or(0),
        )
        .await?;
    
    // Check if there are more services
    let has_more = total_count > (query.offset.unwrap_or(0) + query.limit.unwrap_or(10));
    
    // Return the response
    Ok(Json(ServiceListResponse {
        services,
        total_count,
        has_more,
    }))
}

/// Get service handler
async fn get_service(
    State(api_service): State<Arc<ApiService>>,
    auth: Auth,
    Path(id): Path<Uuid>,
) -> Result<Json<Service>, ApiError> {
    // Get the service
    let service = api_service.service_service.get_service(id).await?;
    
    // Check if the user owns the service
    if service.user_id != auth.user.id {
        return Err(ApiError::Authorization(
            "You are not authorized to view this service".to_string(),
        ));
    }
    
    // Return the service
    Ok(Json(service))
}

/// Create service handler
async fn create_service(
    State(api_service): State<Arc<ApiService>>,
    auth: Auth,
    Json(request): Json<CreateServiceRequest>,
) -> Result<Json<Service>, ApiError> {
    // Validate the request
    request.validate().map_err(|e| ApiError::Validation(e.to_string()))?;
    
    // Create the service
    let service = api_service
        .service_service
        .create_service(
            auth.user.id,
            &request.name,
            request.description.as_deref(),
            request.service_type,
            &request.config,
            request.visibility.unwrap_or_default(),
        )
        .await?;
    
    // Return the service
    Ok(Json(service))
}

/// Update service handler
async fn update_service(
    State(api_service): State<Arc<ApiService>>,
    auth: Auth,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateServiceRequest>,
) -> Result<Json<Service>, ApiError> {
    // Validate the request
    request.validate().map_err(|e| ApiError::Validation(e.to_string()))?;
    
    // Get the service
    let service = api_service.service_service.get_service(id).await?;
    
    // Check if the user owns the service
    if service.user_id != auth.user.id {
        return Err(ApiError::Authorization(
            "You are not authorized to update this service".to_string(),
        ));
    }
    
    // Update the service
    let service = api_service
        .service_service
        .update_service(
            id,
            request.name.as_deref(),
            request.description.as_deref(),
            request.config.as_ref(),
            request.status,
            request.visibility,
        )
        .await?;
    
    // Return the service
    Ok(Json(service))
}

/// Delete service handler
async fn delete_service(
    State(api_service): State<Arc<ApiService>>,
    auth: Auth,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, ApiError> {
    // Get the service
    let service = api_service.service_service.get_service(id).await?;
    
    // Check if the user owns the service
    if service.user_id != auth.user.id {
        return Err(ApiError::Authorization(
            "You are not authorized to delete this service".to_string(),
        ));
    }
    
    // Delete the service
    api_service.service_service.delete_service(id).await?;
    
    // Return success
    Ok(Json(()))
}

/// Discover services handler
async fn discover_services(
    State(api_service): State<Arc<ApiService>>,
    Query(query): Query<ServiceDiscoveryRequest>,
) -> Result<Json<ServiceDiscoveryResponse>, ApiError> {
    // Discover services
    let (services, total_count) = api_service
        .service_service
        .discover_services(
            query.service_type,
            query.tags.as_deref(),
            query.query.as_deref(),
            query.limit.unwrap_or(10),
            query.offset.unwrap_or(0),
        )
        .await?;
    
    // Check if there are more services
    let has_more = total_count > (query.offset.unwrap_or(0) + query.limit.unwrap_or(10));
    
    // Return the response
    Ok(Json(ServiceDiscoveryResponse {
        services,
        total_count,
        has_more,
    }))
}

/// Service routes
pub fn service_routes(api_service: Arc<ApiService>) -> Router {
    Router::new()
        .route("/services", get(list_services))
        .route("/services", post(create_service))
        .route("/services/:id", get(get_service))
        .route("/services/:id", post(update_service))
        .route("/services/:id", axum::routing::delete(delete_service))
        .route("/services/discover", get(discover_services))
        .with_state(api_service)
}
