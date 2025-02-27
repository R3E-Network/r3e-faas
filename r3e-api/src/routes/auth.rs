// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::auth::Auth;
use crate::error::ApiError;
use crate::models::user::{
    CreateUserRequest, LoginRequest, LoginResponse, UpdateUserRequest, User, UserProfile, UserRole,
};
use crate::service::ApiService;

/// Register a new user
async fn register(
    State(api_service): State<Arc<ApiService>>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<UserProfile>, ApiError> {
    // Validate the request
    request.validate().map_err(|e| ApiError::Validation(e.to_string()))?;
    
    // Create the user
    let user = api_service
        .auth_service
        .create_user(
            &request.username,
            &request.email,
            &request.password,
            request.role.unwrap_or_default(),
        )
        .await?;
    
    // Return the user profile
    Ok(Json(UserProfile::from(user)))
}

/// Login a user
async fn login(
    State(api_service): State<Arc<ApiService>>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    // Validate the request
    request.validate().map_err(|e| ApiError::Validation(e.to_string()))?;
    
    // Login the user
    let (user, token) = api_service
        .auth_service
        .login(&request.username_or_email, &request.password)
        .await?;
    
    // Return the login response
    Ok(Json(LoginResponse {
        user,
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in: api_service.config.jwt_expiration,
    }))
}

/// Get the current user
async fn me(auth: Auth) -> Result<Json<UserProfile>, ApiError> {
    // Return the user profile
    Ok(Json(UserProfile::from(auth.user)))
}

/// Get a user by ID
async fn get_user(
    State(api_service): State<Arc<ApiService>>,
    auth: Auth,
    Path(id): Path<Uuid>,
) -> Result<Json<UserProfile>, ApiError> {
    // Check if the user is an admin or the user is getting their own profile
    if auth.user.role != UserRole::Admin && auth.user.id != id {
        return Err(ApiError::Authorization(
            "You are not authorized to view this user".to_string(),
        ));
    }
    
    // Get the user
    let user = api_service.auth_service.get_user_by_id(id).await?;
    
    // Return the user profile
    Ok(Json(UserProfile::from(user)))
}

/// Update a user
async fn update_user(
    State(api_service): State<Arc<ApiService>>,
    auth: Auth,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<Json<UserProfile>, ApiError> {
    // Validate the request
    request.validate().map_err(|e| ApiError::Validation(e.to_string()))?;
    
    // Check if the user is an admin or the user is updating their own profile
    if auth.user.role != UserRole::Admin && auth.user.id != id {
        return Err(ApiError::Authorization(
            "You are not authorized to update this user".to_string(),
        ));
    }
    
    // Check if a non-admin user is trying to change their role
    if auth.user.role != UserRole::Admin && request.role.is_some() {
        return Err(ApiError::Authorization(
            "You are not authorized to change your role".to_string(),
        ));
    }
    
    // Update the user
    let user = api_service
        .auth_service
        .update_user(
            id,
            request.username.as_deref(),
            request.email.as_deref(),
            request.password.as_deref(),
            request.role,
        )
        .await?;
    
    // Return the user profile
    Ok(Json(UserProfile::from(user)))
}

/// Delete a user
async fn delete_user(
    State(api_service): State<Arc<ApiService>>,
    auth: Auth,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, ApiError> {
    // Check if the user is an admin or the user is deleting their own profile
    if auth.user.role != UserRole::Admin && auth.user.id != id {
        return Err(ApiError::Authorization(
            "You are not authorized to delete this user".to_string(),
        ));
    }
    
    // Delete the user
    api_service.auth_service.delete_user(id).await?;
    
    // Return success
    Ok(Json(()))
}

/// Auth routes
pub fn auth_routes(api_service: Arc<ApiService>) -> Router {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/me", get(me))
        .route("/users/:id", get(get_user))
        .route("/users/:id", post(update_user))
        .route("/users/:id", axum::routing::delete(delete_user))
        .with_state(api_service)
}
