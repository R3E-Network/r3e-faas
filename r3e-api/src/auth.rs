// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::Config;
use crate::error::ApiError;
use crate::models::user::{User, UserRole};

/// JWT claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    
    /// Username
    pub username: String,
    
    /// User role
    pub role: String,
    
    /// Issued at
    pub iat: i64,
    
    /// Expiration
    pub exp: i64,
}

/// Authentication service
pub struct AuthService {
    /// Database pool
    db: PgPool,
    
    /// JWT secret
    jwt_secret: String,
    
    /// JWT expiration in seconds
    jwt_expiration: u64,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new(db: PgPool, config: &Config) -> Self {
        Self {
            db,
            jwt_secret: config.jwt_secret.clone(),
            jwt_expiration: config.jwt_expiration,
        }
    }
    
    /// Hash a password
    pub fn hash_password(&self, password: &str) -> Result<String, ApiError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| ApiError::Authentication(format!("Failed to hash password: {}", e)))
    }
    
    /// Verify a password
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, ApiError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| ApiError::Authentication(format!("Failed to parse password hash: {}", e)))?;
        
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
    
    /// Generate a JWT token
    pub fn generate_token(&self, user: &User) -> Result<String, ApiError> {
        let now = Utc::now();
        let expiration = now + Duration::seconds(self.jwt_expiration as i64);
        
        let claims = Claims {
            sub: user.id.to_string(),
            username: user.username.clone(),
            role: format!("{:?}", user.role).to_lowercase(),
            iat: now.timestamp(),
            exp: expiration.timestamp(),
        };
        
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| ApiError::Authentication(format!("Failed to generate token: {}", e)))
    }
    
    /// Verify a JWT token
    pub fn verify_token(&self, token: &str) -> Result<Claims, ApiError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| ApiError::Authentication(format!("Invalid token: {}", e)))
    }
    
    /// Get a user by ID
    pub async fn get_user_by_id(&self, id: Uuid) -> Result<User, ApiError> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| ApiError::Database(format!("Failed to get user: {}", e)))?
            .ok_or_else(|| ApiError::NotFound(format!("User not found: {}", id)))
    }
    
    /// Get a user by username or email
    pub async fn get_user_by_username_or_email(&self, username_or_email: &str) -> Result<User, ApiError> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1 OR email = $1")
            .bind(username_or_email)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| ApiError::Database(format!("Failed to get user: {}", e)))?
            .ok_or_else(|| ApiError::NotFound(format!("User not found: {}", username_or_email)))
    }
    
    /// Create a user
    pub async fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
        role: UserRole,
    ) -> Result<User, ApiError> {
        // Check if username or email already exists
        let existing_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1 OR email = $2")
            .bind(username)
            .bind(email)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| ApiError::Database(format!("Failed to check existing user: {}", e)))?;
        
        if let Some(user) = existing_user {
            if user.username == username {
                return Err(ApiError::Validation(format!("Username already exists: {}", username)));
            } else {
                return Err(ApiError::Validation(format!("Email already exists: {}", email)));
            }
        }
        
        // Hash the password
        let password_hash = self.hash_password(password)?;
        
        // Generate API key
        let api_key = format!("r3e_{}", Uuid::new_v4().to_string().replace("-", ""));
        
        // Create the user
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (username, email, password_hash, role, api_key, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING *",
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .bind(role as i32)
        .bind(api_key)
        .bind(Utc::now())
        .bind(Utc::now())
        .fetch_one(&self.db)
        .await
        .map_err(|e| ApiError::Database(format!("Failed to create user: {}", e)))?;
        
        Ok(user)
    }
    
    /// Update a user
    pub async fn update_user(
        &self,
        id: Uuid,
        username: Option<&str>,
        email: Option<&str>,
        password: Option<&str>,
        role: Option<UserRole>,
    ) -> Result<User, ApiError> {
        // Get the current user
        let user = self.get_user_by_id(id).await?;
        
        // Check if username or email already exists
        if let Some(username) = username {
            if username != user.username {
                let existing_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
                    .bind(username)
                    .fetch_optional(&self.db)
                    .await
                    .map_err(|e| ApiError::Database(format!("Failed to check existing user: {}", e)))?;
                
                if existing_user.is_some() {
                    return Err(ApiError::Validation(format!("Username already exists: {}", username)));
                }
            }
        }
        
        if let Some(email) = email {
            if email != user.email {
                let existing_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
                    .bind(email)
                    .fetch_optional(&self.db)
                    .await
                    .map_err(|e| ApiError::Database(format!("Failed to check existing user: {}", e)))?;
                
                if existing_user.is_some() {
                    return Err(ApiError::Validation(format!("Email already exists: {}", email)));
                }
            }
        }
        
        // Hash the password if provided
        let password_hash = if let Some(password) = password {
            Some(self.hash_password(password)?)
        } else {
            None
        };
        
        // Update the user
        let user = sqlx::query_as::<_, User>(
            "UPDATE users
             SET username = COALESCE($1, username),
                 email = COALESCE($2, email),
                 password_hash = COALESCE($3, password_hash),
                 role = COALESCE($4, role),
                 updated_at = $5
             WHERE id = $6
             RETURNING *",
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .bind(role.map(|r| r as i32))
        .bind(Utc::now())
        .bind(id)
        .fetch_one(&self.db)
        .await
        .map_err(|e| ApiError::Database(format!("Failed to update user: {}", e)))?;
        
        Ok(user)
    }
    
    /// Delete a user
    pub async fn delete_user(&self, id: Uuid) -> Result<(), ApiError> {
        // Check if user exists
        self.get_user_by_id(id).await?;
        
        // Delete the user
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.db)
            .await
            .map_err(|e| ApiError::Database(format!("Failed to delete user: {}", e)))?;
        
        Ok(())
    }
    
    /// Login a user
    pub async fn login(
        &self,
        username_or_email: &str,
        password: &str,
    ) -> Result<(User, String), ApiError> {
        // Get the user
        let user = self.get_user_by_username_or_email(username_or_email).await?;
        
        // Verify the password
        if !self.verify_password(password, &user.password_hash)? {
            return Err(ApiError::Authentication("Invalid password".to_string()));
        }
        
        // Generate a token
        let token = self.generate_token(&user)?;
        
        Ok((user, token))
    }
}

/// Authentication extractor
pub struct Auth {
    /// User
    pub user: User,
    
    /// Claims
    pub claims: Claims,
}

#[async_trait]
impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
    PgPool: FromRef<S>,
    Config: FromRef<S>,
{
    type Rejection = Response;
    
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Get the database pool and config
        let db = PgPool::from_ref(state);
        let config = Config::from_ref(state);
        
        // Create the auth service
        let auth_service = AuthService::new(db, &config);
        
        // Get the authorization header
        let headers = parts.headers.clone();
        let auth_header = headers
            .get("Authorization")
            .ok_or_else(|| ApiError::Authentication("Missing authorization header".to_string()).into_response())?
            .to_str()
            .map_err(|_| ApiError::Authentication("Invalid authorization header".to_string()).into_response())?;
        
        // Check if the header starts with "Bearer "
        if !auth_header.starts_with("Bearer ") {
            return Err(ApiError::Authentication("Invalid authorization header".to_string()).into_response());
        }
        
        // Extract the token
        let token = &auth_header[7..];
        
        // Verify the token
        let claims = auth_service
            .verify_token(token)
            .map_err(|e| e.into_response())?;
        
        // Get the user
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| ApiError::Authentication("Invalid user ID".to_string()).into_response())?;
        
        let user = auth_service
            .get_user_by_id(user_id)
            .await
            .map_err(|e| e.into_response())?;
        
        Ok(Self { user, claims })
    }
}

/// Role-based authorization
pub struct RequireRole(pub UserRole);

#[async_trait]
impl<S> FromRequestParts<S> for RequireRole
where
    S: Send + Sync,
    PgPool: FromRef<S>,
    Config: FromRef<S>,
{
    type Rejection = Response;
    
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Get the auth
        let Auth { user, .. } = Auth::from_request_parts(parts, state).await?;
        
        // Check if the user is an admin
        if user.role == UserRole::Admin {
            return Ok(Self(UserRole::Admin));
        }
        
        // Return the user's role
        Ok(Self(user.role))
    }
}

/// API key authentication
pub struct ApiKeyAuth {
    /// User
    pub user: User,
}

#[async_trait]
impl<S> FromRequestParts<S> for ApiKeyAuth
where
    S: Send + Sync,
    PgPool: FromRef<S>,
{
    type Rejection = Response;
    
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Get the database pool
        let db = PgPool::from_ref(state);
        
        // Get the API key header
        let headers = parts.headers.clone();
        let api_key = headers
            .get("X-API-Key")
            .ok_or_else(|| ApiError::Authentication("Missing API key".to_string()).into_response())?
            .to_str()
            .map_err(|_| ApiError::Authentication("Invalid API key".to_string()).into_response())?;
        
        // Get the user by API key
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE api_key = $1")
            .bind(api_key)
            .fetch_optional(&db)
            .await
            .map_err(|e| ApiError::Database(format!("Failed to get user: {}", e)).into_response())?
            .ok_or_else(|| ApiError::Authentication("Invalid API key".to_string()).into_response())?;
        
        Ok(Self { user })
    }
}
