// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

/// Service type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceType {
    /// Standard service
    Standard,
    
    /// Oracle service
    Oracle,
    
    /// TEE service
    Tee,
    
    /// Blockchain service
    Blockchain,
}

impl Default for ServiceType {
    fn default() -> Self {
        Self::Standard
    }
}

/// Service status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceStatus {
    /// Creating
    Creating,
    
    /// Active
    Active,
    
    /// Inactive
    Inactive,
    
    /// Error
    Error,
}

impl Default for ServiceStatus {
    fn default() -> Self {
        Self::Creating
    }
}

/// Service visibility
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceVisibility {
    /// Public service
    Public,
    
    /// Private service
    Private,
}

impl Default for ServiceVisibility {
    fn default() -> Self {
        Self::Private
    }
}

/// Service model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Service {
    /// Service ID
    pub id: Uuid,
    
    /// User ID
    pub user_id: Uuid,
    
    /// Service name
    pub name: String,
    
    /// Service description
    pub description: Option<String>,
    
    /// Service type
    pub service_type: ServiceType,
    
    /// Service configuration
    pub config: serde_json::Value,
    
    /// Service status
    pub status: ServiceStatus,
    
    /// Service visibility
    pub visibility: ServiceVisibility,
    
    /// Service version
    pub version: String,
    
    /// Created at
    pub created_at: DateTime<Utc>,
    
    /// Updated at
    pub updated_at: DateTime<Utc>,
}

/// Create service request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateServiceRequest {
    /// Service name
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    
    /// Service description
    #[validate(length(min = 0, max = 500))]
    pub description: Option<String>,
    
    /// Service type
    pub service_type: ServiceType,
    
    /// Service configuration
    pub config: serde_json::Value,
    
    /// Service visibility
    pub visibility: Option<ServiceVisibility>,
}

/// Update service request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateServiceRequest {
    /// Service name
    #[validate(length(min = 3, max = 50))]
    pub name: Option<String>,
    
    /// Service description
    #[validate(length(min = 0, max = 500))]
    pub description: Option<String>,
    
    /// Service configuration
    pub config: Option<serde_json::Value>,
    
    /// Service status
    pub status: Option<ServiceStatus>,
    
    /// Service visibility
    pub visibility: Option<ServiceVisibility>,
}

/// Service summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceSummary {
    /// Service ID
    pub id: Uuid,
    
    /// Service name
    pub name: String,
    
    /// Service description
    pub description: Option<String>,
    
    /// Service type
    pub service_type: ServiceType,
    
    /// Service status
    pub status: ServiceStatus,
    
    /// Service visibility
    pub visibility: ServiceVisibility,
    
    /// Function count
    pub function_count: u32,
    
    /// Created at
    pub created_at: DateTime<Utc>,
    
    /// Updated at
    pub updated_at: DateTime<Utc>,
}

/// Service list request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceListRequest {
    /// User ID
    pub user_id: Option<Uuid>,
    
    /// Service type
    pub service_type: Option<ServiceType>,
    
    /// Service status
    pub status: Option<ServiceStatus>,
    
    /// Service visibility
    pub visibility: Option<ServiceVisibility>,
    
    /// Search query
    pub query: Option<String>,
    
    /// Limit
    pub limit: Option<u32>,
    
    /// Offset
    pub offset: Option<u32>,
}

/// Service list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceListResponse {
    /// Services
    pub services: Vec<ServiceSummary>,
    
    /// Total count
    pub total_count: u32,
    
    /// Has more
    pub has_more: bool,
}

/// Service discovery request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryRequest {
    /// Service type
    pub service_type: Option<ServiceType>,
    
    /// Tags
    pub tags: Option<Vec<String>>,
    
    /// Search query
    pub query: Option<String>,
    
    /// Limit
    pub limit: Option<u32>,
    
    /// Offset
    pub offset: Option<u32>,
}

/// Service discovery response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryResponse {
    /// Services
    pub services: Vec<ServiceSummary>,
    
    /// Total count
    pub total_count: u32,
    
    /// Has more
    pub has_more: bool,
}
