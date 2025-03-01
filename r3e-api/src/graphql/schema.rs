// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_graphql::{Context, EmptySubscription, Object, Schema, SimpleObject};
use std::sync::Arc;
use uuid::Uuid;

use crate::auth::Auth;
use crate::error::ApiError;
use crate::graphql::types::{
    FunctionInput, FunctionObject, FunctionResult, ServiceInput, ServiceObject, ServiceResult,
    UserInput, UserObject, UserResult,
};
use crate::service::ApiService;

/// API GraphQL schema
pub type ApiSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// Create the GraphQL schema
pub fn create_schema(api_service: Arc<ApiService>) -> ApiSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(api_service)
        .finish()
}

/// GraphQL query root
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get the current user
    async fn me(&self, ctx: &Context<'_>) -> Result<UserObject, ApiError> {
        let auth = ctx
            .data::<Auth>()
            .map_err(|e| ApiError::Authentication(format!("Authentication required: {}", e)))?;

        Ok(UserObject::from(auth.user.clone()))
    }

    /// Get a user by ID
    async fn user(&self, ctx: &Context<'_>, id: Uuid) -> Result<UserObject, ApiError> {
        let auth = ctx
            .data::<Auth>()
            .map_err(|e| ApiError::Authentication(format!("Authentication required: {}", e)))?;

        let api_service = ctx
            .data::<Arc<ApiService>>()
            .map_err(|e| ApiError::Server(format!("Failed to get API service: {}", e)))?;

        // Check if the user is an admin or the user is getting their own profile
        if auth.user.role != crate::models::user::UserRole::Admin && auth.user.id != id {
            return Err(ApiError::Authorization(
                "You are not authorized to view this user".to_string(),
            ));
        }

        // Get the user
        let user = api_service.auth_service.get_user_by_id(id).await?;

        Ok(UserObject::from(user))
    }

    /// Get a service by ID
    async fn service(&self, ctx: &Context<'_>, id: Uuid) -> Result<ServiceObject, ApiError> {
        let auth = ctx
            .data::<Auth>()
            .map_err(|e| ApiError::Authentication(format!("Authentication required: {}", e)))?;

        let api_service = ctx
            .data::<Arc<ApiService>>()
            .map_err(|e| ApiError::Server(format!("Failed to get API service: {}", e)))?;

        // Get the service
        let service = api_service.service_service.get_service(id).await?;

        // Check if the user owns the service
        if service.user_id != auth.user.id {
            return Err(ApiError::Authorization(
                "You are not authorized to view this service".to_string(),
            ));
        }

        Ok(ServiceObject::from(service))
    }

    /// List services
    async fn services(
        &self,
        ctx: &Context<'_>,
        user_id: Option<Uuid>,
        service_type: Option<String>,
        status: Option<String>,
        visibility: Option<String>,
        query: Option<String>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<ServiceObject>, ApiError> {
        let auth = ctx
            .data::<Auth>()
            .map_err(|e| ApiError::Authentication(format!("Authentication required: {}", e)))?;

        let api_service = ctx
            .data::<Arc<ApiService>>()
            .map_err(|e| ApiError::Server(format!("Failed to get API service: {}", e)))?;

        // Parse the service type
        let service_type = if let Some(service_type) = service_type {
            Some(service_type.parse().map_err(|_| {
                ApiError::Validation(format!("Invalid service type: {}", service_type))
            })?)
        } else {
            None
        };

        // Parse the status
        let status = if let Some(status) = status {
            Some(
                status
                    .parse()
                    .map_err(|_| ApiError::Validation(format!("Invalid status: {}", status)))?,
            )
        } else {
            None
        };

        // Parse the visibility
        let visibility =
            if let Some(visibility) = visibility {
                Some(visibility.parse().map_err(|_| {
                    ApiError::Validation(format!("Invalid visibility: {}", visibility))
                })?)
            } else {
                None
            };

        // Get the services
        let (services, _) = api_service
            .service_service
            .list_services(
                user_id.unwrap_or(auth.user.id),
                service_type,
                status,
                visibility,
                query.as_deref(),
                limit.unwrap_or(10),
                offset.unwrap_or(0),
            )
            .await?;

        Ok(services.into_iter().map(ServiceObject::from).collect())
    }

    /// Get a function by ID
    async fn function(&self, ctx: &Context<'_>, id: Uuid) -> Result<FunctionObject, ApiError> {
        let auth = ctx
            .data::<Auth>()
            .map_err(|e| ApiError::Authentication(format!("Authentication required: {}", e)))?;

        let api_service = ctx
            .data::<Arc<ApiService>>()
            .map_err(|e| ApiError::Server(format!("Failed to get API service: {}", e)))?;

        // Get the function
        let function = api_service.function_service.get_function(id).await?;

        // Check if the user owns the function
        if function.user_id != auth.user.id {
            return Err(ApiError::Authorization(
                "You are not authorized to view this function".to_string(),
            ));
        }

        Ok(FunctionObject::from(function))
    }

    /// List functions
    async fn functions(
        &self,
        ctx: &Context<'_>,
        service_id: Option<Uuid>,
        status: Option<String>,
        trigger_type: Option<String>,
        query: Option<String>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<FunctionObject>, ApiError> {
        let auth = ctx
            .data::<Auth>()
            .map_err(|e| ApiError::Authentication(format!("Authentication required: {}", e)))?;

        let api_service = ctx
            .data::<Arc<ApiService>>()
            .map_err(|e| ApiError::Server(format!("Failed to get API service: {}", e)))?;

        // Parse the status
        let status = if let Some(status) = status {
            Some(
                status
                    .parse()
                    .map_err(|_| ApiError::Validation(format!("Invalid status: {}", status)))?,
            )
        } else {
            None
        };

        // Get the functions
        let (functions, _) = api_service
            .function_service
            .list_functions(
                auth.user.id,
                service_id,
                status,
                trigger_type.as_deref(),
                query.as_deref(),
                limit.unwrap_or(10),
                offset.unwrap_or(0),
            )
            .await?;

        Ok(functions.into_iter().map(FunctionObject::from).collect())
    }

    /// Discover services
    async fn discover_services(
        &self,
        ctx: &Context<'_>,
        service_type: Option<String>,
        tags: Option<Vec<String>>,
        query: Option<String>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<ServiceObject>, ApiError> {
        let api_service = ctx
            .data::<Arc<ApiService>>()
            .map_err(|e| ApiError::Server(format!("Failed to get API service: {}", e)))?;

        // Parse the service type
        let service_type = if let Some(service_type) = service_type {
            Some(service_type.parse().map_err(|_| {
                ApiError::Validation(format!("Invalid service type: {}", service_type))
            })?)
        } else {
            None
        };

        // Discover services
        let (services, _) = api_service
            .service_service
            .discover_services(
                service_type,
                tags.as_deref(),
                query.as_deref(),
                limit.unwrap_or(10),
                offset.unwrap_or(0),
            )
            .await?;

        Ok(services.into_iter().map(ServiceObject::from).collect())
    }
}

/// GraphQL mutation root
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Register a new user
    async fn register(&self, ctx: &Context<'_>, input: UserInput) -> Result<UserResult, ApiError> {
        let api_service = ctx
            .data::<Arc<ApiService>>()
            .map_err(|e| ApiError::Server(format!("Failed to get API service: {}", e)))?;

        // Create the user
        let user = api_service
            .auth_service
            .create_user(
                &input.username,
                &input.email,
                &input.password,
                input.role.unwrap_or_default(),
            )
            .await?;

        Ok(UserResult {
            success: true,
            message: "User registered successfully".to_string(),
            user: Some(UserObject::from(user)),
        })
    }

    /// Login a user
    async fn login(
        &self,
        ctx: &Context<'_>,
        username_or_email: String,
        password: String,
    ) -> Result<UserResult, ApiError> {
        let api_service = ctx
            .data::<Arc<ApiService>>()
            .map_err(|e| ApiError::Server(format!("Failed to get API service: {}", e)))?;

        // Login the user
        let (user, token) = api_service
            .auth_service
            .login(&username_or_email, &password)
            .await?;

        Ok(UserResult {
            success: true,
            message: "User logged in successfully".to_string(),
            user: Some(UserObject::from(user)),
            token: Some(token),
        })
    }

    /// Update a user
    async fn update_user(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        input: UserInput,
    ) -> Result<UserResult, ApiError> {
        let auth = ctx
            .data::<Auth>()
            .map_err(|e| ApiError::Authentication(format!("Authentication required: {}", e)))?;

        let api_service = ctx
            .data::<Arc<ApiService>>()
            .map_err(|e| ApiError::Server(format!("Failed to get API service: {}", e)))?;

        // Check if the user is an admin or the user is updating their own profile
        if auth.user.role != crate::models::user::UserRole::Admin && auth.user.id != id {
            return Err(ApiError::Authorization(
                "You are not authorized to update this user".to_string(),
            ));
        }

        // Check if a non-admin user is trying to change their role
        if auth.user.role != crate::models::user::UserRole::Admin && input.role.is_some() {
            return Err(ApiError::Authorization(
                "You are not authorized to change your role".to_string(),
            ));
        }

        // Update the user
        let user = api_service
            .auth_service
            .update_user(
                id,
                Some(&input.username),
                Some(&input.email),
                Some(&input.password),
                input.role,
            )
            .await?;

        Ok(UserResult {
            success: true,
            message: "User updated successfully".to_string(),
            user: Some(UserObject::from(user)),
        })
    }

    /// Delete a user
    async fn delete_user(&self, ctx: &Context<'_>, id: Uuid) -> Result<UserResult, ApiError> {
        let auth = ctx
            .data::<Auth>()
            .map_err(|e| ApiError::Authentication(format!("Authentication required: {}", e)))?;

        let api_service = ctx
            .data::<Arc<ApiService>>()
            .map_err(|e| ApiError::Server(format!("Failed to get API service: {}", e)))?;

        // Check if the user is an admin or the user is deleting their own profile
        if auth.user.role != crate::models::user::UserRole::Admin && auth.user.id != id {
            return Err(ApiError::Authorization(
                "You are not authorized to delete this user".to_string(),
            ));
        }

        // Delete the user
        api_service.auth_service.delete_user(id).await?;

        Ok(UserResult {
            success: true,
            message: "User deleted successfully".to_string(),
            user: None,
        })
    }

    /// Create a service
    async fn create_service(
        &self,
        ctx: &Context<'_>,
        input: ServiceInput,
    ) -> Result<ServiceResult, ApiError> {
        let auth = ctx
            .data::<Auth>()
            .map_err(|e| ApiError::Authentication(format!("Authentication required: {}", e)))?;

        let api_service = ctx
            .data::<Arc<ApiService>>()
            .map_err(|e| ApiError::Server(format!("Failed to get API service: {}", e)))?;

        // Create the service
        let service = api_service
            .service_service
            .create_service(
                auth.user.id,
                &input.name,
                input.description.as_deref(),
                input.service_type,
                &input.config,
                input.visibility.unwrap_or_default(),
            )
            .await?;

        Ok(ServiceResult {
            success: true,
            message: "Service created successfully".to_string(),
            service: Some(ServiceObject::from(service)),
        })
    }

    /// Update a service
    async fn update_service(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        input: ServiceInput,
    ) -> Result<ServiceResult, ApiError> {
        let auth = ctx
            .data::<Auth>()
            .map_err(|e| ApiError::Authentication(format!("Authentication required: {}", e)))?;

        let api_service = ctx
            .data::<Arc<ApiService>>()
            .map_err(|e| ApiError::Server(format!("Failed to get API service: {}", e)))?;

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
                Some(&input.name),
                input.description.as_deref(),
                Some(&input.config),
                input.status,
                input.visibility,
            )
            .await?;

        Ok(ServiceResult {
            success: true,
            message: "Service updated successfully".to_string(),
            service: Some(ServiceObject::from(service)),
        })
    }

    /// Delete a service
    async fn delete_service(&self, ctx: &Context<'_>, id: Uuid) -> Result<ServiceResult, ApiError> {
        let auth = ctx
            .data::<Auth>()
            .map_err(|e| ApiError::Authentication(format!("Authentication required: {}", e)))?;

        let api_service = ctx
            .data::<Arc<ApiService>>()
            .map_err(|e| ApiError::Server(format!("Failed to get API service: {}", e)))?;

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

        Ok(ServiceResult {
            success: true,
            message: "Service deleted successfully".to_string(),
            service: None,
        })
    }

    /// Create a function
    async fn create_function(
        &self,
        ctx: &Context<'_>,
        input: FunctionInput,
    ) -> Result<FunctionResult, ApiError> {
        let auth = ctx
            .data::<Auth>()
            .map_err(|e| ApiError::Authentication(format!("Authentication required: {}", e)))?;

        let api_service = ctx
            .data::<Arc<ApiService>>()
            .map_err(|e| ApiError::Server(format!("Failed to get API service: {}", e)))?;

        // Check if the user owns the service
        let service = api_service
            .service_service
            .get_service(input.service_id)
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
                input.service_id,
                &input.name,
                input.description.as_deref(),
                &input.code,
                input.runtime.unwrap_or_default(),
                input.trigger_type,
                &input.trigger_config,
                input.security_level.unwrap_or_default(),
            )
            .await?;

        Ok(FunctionResult {
            success: true,
            message: "Function created successfully".to_string(),
            function: Some(FunctionObject::from(function)),
        })
    }

    /// Update a function
    async fn update_function(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        input: FunctionInput,
    ) -> Result<FunctionResult, ApiError> {
        let auth = ctx
            .data::<Auth>()
            .map_err(|e| ApiError::Authentication(format!("Authentication required: {}", e)))?;

        let api_service = ctx
            .data::<Arc<ApiService>>()
            .map_err(|e| ApiError::Server(format!("Failed to get API service: {}", e)))?;

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
                Some(&input.name),
                input.description.as_deref(),
                Some(&input.code),
                input.runtime,
                Some(input.trigger_type),
                Some(&input.trigger_config),
                input.security_level,
                input.status,
            )
            .await?;

        Ok(FunctionResult {
            success: true,
            message: "Function updated successfully".to_string(),
            function: Some(FunctionObject::from(function)),
        })
    }

    /// Delete a function
    async fn delete_function(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<FunctionResult, ApiError> {
        let auth = ctx
            .data::<Auth>()
            .map_err(|e| ApiError::Authentication(format!("Authentication required: {}", e)))?;

        let api_service = ctx
            .data::<Arc<ApiService>>()
            .map_err(|e| ApiError::Server(format!("Failed to get API service: {}", e)))?;

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

        Ok(FunctionResult {
            success: true,
            message: "Function deleted successfully".to_string(),
            function: None,
        })
    }

    /// Invoke a function
    async fn invoke_function(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        input: serde_json::Value,
    ) -> Result<FunctionResult, ApiError> {
        let auth = ctx
            .data::<Auth>()
            .map_err(|e| ApiError::Authentication(format!("Authentication required: {}", e)))?;

        let api_service = ctx
            .data::<Arc<ApiService>>()
            .map_err(|e| ApiError::Server(format!("Failed to get API service: {}", e)))?;

        // Get the function
        let function = api_service.function_service.get_function(id).await?;

        // Check if the function is active
        if function.status != crate::models::function::FunctionStatus::Active {
            return Err(ApiError::Validation("Function is not active".to_string()));
        }

        // Check if the user owns the function or the function's service is public
        let service = api_service
            .service_service
            .get_service(function.service_id)
            .await?;

        if function.user_id != auth.user.id
            && service.visibility != crate::models::service::ServiceVisibility::Public
        {
            return Err(ApiError::Authorization(
                "You are not authorized to invoke this function".to_string(),
            ));
        }

        // Invoke the function
        let response = api_service
            .function_service
            .invoke_function(id, &input)
            .await?;

        Ok(FunctionResult {
            success: true,
            message: "Function invoked successfully".to_string(),
            function: Some(FunctionObject::from(function)),
            invocation_result: Some(response.result),
            execution_time_ms: Some(response.execution_time_ms),
        })
    }
}
