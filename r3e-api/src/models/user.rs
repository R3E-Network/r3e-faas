// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

/// User role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    /// Admin user
    Admin,
    
    /// Developer user
    Developer,
    
    /// Viewer user
    Viewer,
}

impl Default for UserRole {
    fn default() -> Self {
        Self::Developer
    }
}

/// User model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    /// User ID
    pub id: Uuid,
    
    /// Username
    pub username: String,
    
    /// Email
    pub email: String,
    
    /// Password hash
    #[serde(skip_serializing)]
    pub password_hash: String,
    
    /// User role
    pub role: UserRole,
    
    /// API key
    #[serde(skip_serializing)]
    pub api_key: Option<String>,
    
    /// Created at
    pub created_at: DateTime<Utc>,
    
    /// Updated at
    pub updated_at: DateTime<Utc>,
}

/// Create user request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    /// Username
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    
    /// Email
    #[validate(email)]
    pub email: String,
    
    /// Password
    #[validate(length(min = 8, max = 100))]
    pub password: String,
    
    /// User role
    pub role: Option<UserRole>,
}

/// Update user request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserRequest {
    /// Username
    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,
    
    /// Email
    #[validate(email)]
    pub email: Option<String>,
    
    /// Password
    #[validate(length(min = 8, max = 100))]
    pub password: Option<String>,
    
    /// User role
    pub role: Option<UserRole>,
}

/// Login request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    /// Username or email
    #[validate(length(min = 3, max = 100))]
    pub username_or_email: String,
    
    /// Password
    #[validate(length(min = 8, max = 100))]
    pub password: String,
}

/// Login response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    /// User
    pub user: User,
    
    /// Access token
    pub access_token: String,
    
    /// Token type
    pub token_type: String,
    
    /// Expires in seconds
    pub expires_in: u64,
}

/// User profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    /// User ID
    pub id: Uuid,
    
    /// Username
    pub username: String,
    
    /// Email
    pub email: String,
    
    /// User role
    pub role: UserRole,
    
    /// Created at
    pub created_at: DateTime<Utc>,
    
    /// Updated at
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserProfile {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            role: user.role,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
