// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_graphql::{InputObject, Object, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::function::{
    Function, FunctionStatus, Runtime, SecurityLevel, TriggerType,
};
use crate::models::service::{
    Service, ServiceStatus, ServiceSummary, ServiceType, ServiceVisibility,
};
use crate::models::user::{User, UserRole};

/// User object
#[derive(Debug, Clone, SimpleObject)]
pub struct UserObject {
    /// User ID
    pub id: Uuid,
    
    /// Username
    pub username: String,
    
    /// Email
    pub email: String,
    
    /// User role
    pub role: String,
    
    /// Created at
    pub created_at: DateTime<Utc>,
    
    /// Updated at
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserObject {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            role: format!("{:?}", user.role).to_lowercase(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

/// User input
#[derive(Debug, Clone, InputObject)]
pub struct UserInput {
    /// Username
    pub username: String,
    
    /// Email
    pub email: String,
    
    /// Password
    pub password: String,
    
    /// User role
    pub role: Option<String>,
}

/// User result
#[derive(Debug, Clone, SimpleObject)]
pub struct UserResult {
    /// Success
    pub success: bool,
    
    /// Message
    pub message: String,
    
    /// User
    pub user: Option<UserObject>,
    
    /// Token
    #[graphql(skip)]
    pub token: Option<String>,
}

/// Service object
#[derive(Debug, Clone, SimpleObject)]
pub struct ServiceObject {
    /// Service ID
    pub id: Uuid,
    
    /// User ID
    pub user_id: Uuid,
    
    /// Service name
    pub name: String,
    
    /// Service description
    pub description: Option<String>,
    
    /// Service type
    pub service_type: String,
    
    /// Service configuration
    pub config: serde_json::Value,
    
    /// Service status
    pub status: String,
    
    /// Service visibility
    pub visibility: String,
    
    /// Service version
    pub version: String,
    
    /// Created at
    pub created_at: DateTime<Utc>,
    
    /// Updated at
    pub updated_at: DateTime<Utc>,
}

impl From<Service> for ServiceObject {
    fn from(service: Service) -> Self {
        Self {
            id: service.id,
            user_id: service.user_id,
            name: service.name,
            description: service.description,
            service_type: format!("{:?}", service.service_type).to_lowercase(),
            config: service.config,
            status: format!("{:?}", service.status).to_lowercase(),
            visibility: format!("{:?}", service.visibility).to_lowercase(),
            version: service.version,
            created_at: service.created_at,
            updated_at: service.updated_at,
        }
    }
}

impl From<ServiceSummary> for ServiceObject {
    fn from(summary: ServiceSummary) -> Self {
        Self {
            id: summary.id,
            user_id: Uuid::nil(), // Not available in summary
            name: summary.name,
            description: summary.description,
            service_type: format!("{:?}", summary.service_type).to_lowercase(),
            config: serde_json::Value::Null, // Not available in summary
            status: format!("{:?}", summary.status).to_lowercase(),
            visibility: format!("{:?}", summary.visibility).to_lowercase(),
            version: "".to_string(), // Not available in summary
            created_at: summary.created_at,
            updated_at: summary.updated_at,
        }
    }
}

/// Service input
#[derive(Debug, Clone, InputObject)]
pub struct ServiceInput {
    /// Service name
    pub name: String,
    
    /// Service description
    pub description: Option<String>,
    
    /// Service type
    pub service_type: ServiceType,
    
    /// Service configuration
    pub config: serde_json::Value,
    
    /// Service status
    pub status: Option<ServiceStatus>,
    
    /// Service visibility
    pub visibility: Option<ServiceVisibility>,
}

/// Service result
#[derive(Debug, Clone, SimpleObject)]
pub struct ServiceResult {
    /// Success
    pub success: bool,
    
    /// Message
    pub message: String,
    
    /// Service
    pub service: Option<ServiceObject>,
}

/// Function object
#[derive(Debug, Clone, SimpleObject)]
pub struct FunctionObject {
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
    pub runtime: String,
    
    /// Function trigger type
    pub trigger_type: String,
    
    /// Function trigger configuration
    pub trigger_config: serde_json::Value,
    
    /// Function security level
    pub security_level: String,
    
    /// Function status
    pub status: String,
    
    /// Function version
    pub version: String,
    
    /// Function hash
    pub hash: String,
    
    /// Created at
    pub created_at: DateTime<Utc>,
    
    /// Updated at
    pub updated_at: DateTime<Utc>,
}

impl From<Function> for FunctionObject {
    fn from(function: Function) -> Self {
        Self {
            id: function.id,
            service_id: function.service_id,
            user_id: function.user_id,
            name: function.name,
            description: function.description,
            code: function.code,
            runtime: format!("{:?}", function.runtime).to_lowercase(),
            trigger_type: format!("{:?}", function.trigger_type).to_lowercase(),
            trigger_config: function.trigger_config,
            security_level: format!("{:?}", function.security_level).to_lowercase(),
            status: format!("{:?}", function.status).to_lowercase(),
            version: function.version,
            hash: function.hash,
            created_at: function.created_at,
            updated_at: function.updated_at,
        }
    }
}

/// Function input
#[derive(Debug, Clone, InputObject)]
pub struct FunctionInput {
    /// Service ID
    pub service_id: Uuid,
    
    /// Function name
    pub name: String,
    
    /// Function description
    pub description: Option<String>,
    
    /// Function code
    pub code: String,
    
    /// Function runtime
    pub runtime: Option<Runtime>,
    
    /// Function trigger type
    pub trigger_type: TriggerType,
    
    /// Function trigger configuration
    pub trigger_config: serde_json::Value,
    
    /// Function security level
    pub security_level: Option<SecurityLevel>,
    
    /// Function status
    pub status: Option<FunctionStatus>,
}

/// Function result
#[derive(Debug, Clone, SimpleObject)]
pub struct FunctionResult {
    /// Success
    pub success: bool,
    
    /// Message
    pub message: String,
    
    /// Function
    pub function: Option<FunctionObject>,
    
    /// Invocation result
    pub invocation_result: Option<serde_json::Value>,
    
    /// Execution time in milliseconds
    pub execution_time_ms: Option<u64>,
}
