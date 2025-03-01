// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use governor::clock::{DefaultClock, ReasonableClock};
use governor::state::{InMemoryState, NotKeyed};
use governor::{Quota, RateLimiter};
use hmac::{Hmac, Mac};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use tokio::sync::RwLock;

use crate::OracleError;

/// JWT claims for Oracle API authentication
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,

    /// Issued at timestamp
    pub iat: u64,

    /// Expiration timestamp
    pub exp: u64,

    /// API key ID
    pub kid: String,

    /// Permissions
    pub permissions: Vec<String>,
}

/// API key for Oracle service authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    /// API key ID
    pub id: String,

    /// API key secret
    pub secret: String,

    /// User ID
    pub user_id: String,

    /// API key name
    pub name: String,

    /// Creation timestamp
    pub created_at: u64,

    /// Expiration timestamp (optional)
    pub expires_at: Option<u64>,

    /// Permissions
    pub permissions: Vec<String>,

    /// Rate limit (requests per minute)
    pub rate_limit: u32,
}

/// Authentication service for Oracle API
pub struct AuthService {
    /// API keys by ID
    api_keys: Arc<RwLock<HashMap<String, ApiKey>>>,

    /// Rate limiters by API key ID
    rate_limiters:
        Arc<RwLock<HashMap<String, Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>>>>,

    /// JWT secret
    jwt_secret: String,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new(jwt_secret: String) -> Self {
        Self {
            api_keys: Arc::new(RwLock::new(HashMap::new())),
            rate_limiters: Arc::new(RwLock::new(HashMap::new())),
            jwt_secret,
        }
    }

    /// Register a new API key
    pub async fn register_api_key(&self, api_key: ApiKey) -> Result<(), OracleError> {
        // Create a rate limiter for the API key
        let rate_limiter = Arc::new(RateLimiter::direct(Quota::per_minute(
            std::num::NonZeroU32::new(api_key.rate_limit)
                .unwrap_or(std::num::NonZeroU32::new(60).unwrap()),
        )));

        // Store the API key and rate limiter
        self.api_keys
            .write()
            .await
            .insert(api_key.id.clone(), api_key);
        self.rate_limiters
            .write()
            .await
            .insert(api_key.id.clone(), rate_limiter);

        Ok(())
    }

    /// Revoke an API key
    pub async fn revoke_api_key(&self, api_key_id: &str) -> Result<(), OracleError> {
        // Remove the API key and rate limiter
        self.api_keys.write().await.remove(api_key_id);
        self.rate_limiters.write().await.remove(api_key_id);

        Ok(())
    }

    /// Authenticate a request using an API key
    pub async fn authenticate(
        &self,
        api_key_id: &str,
        signature: &str,
        timestamp: u64,
        payload: &str,
    ) -> Result<ApiKey, OracleError> {
        // Get the API key
        let api_key = self
            .api_keys
            .read()
            .await
            .get(api_key_id)
            .cloned()
            .ok_or_else(|| OracleError::Authentication("Invalid API key".to_string()))?;

        // Check if the API key has expired
        if let Some(expires_at) = api_key.expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            if now > expires_at {
                return Err(OracleError::Authentication(
                    "API key has expired".to_string(),
                ));
            }
        }

        // Verify the signature
        let message = format!("{}{}", timestamp, payload);
        let mut mac = Hmac::<Sha256>::new_from_slice(api_key.secret.as_bytes())
            .map_err(|_| OracleError::Authentication("Failed to create HMAC".to_string()))?;

        mac.update(message.as_bytes());
        let expected_signature = hex::encode(mac.finalize().into_bytes());

        if signature != expected_signature {
            return Err(OracleError::Authentication("Invalid signature".to_string()));
        }

        // Check the timestamp (prevent replay attacks)
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        if now - timestamp > 300 {
            return Err(OracleError::Authentication(
                "Request has expired".to_string(),
            ));
        }

        // Apply rate limiting
        if let Some(rate_limiter) = self.rate_limiters.read().await.get(api_key_id) {
            if let Err(_) = rate_limiter.check() {
                return Err(OracleError::RateLimit("Rate limit exceeded".to_string()));
            }
        }

        Ok(api_key)
    }

    /// Generate a JWT token for a user
    pub async fn generate_token(
        &self,
        user_id: &str,
        api_key_id: &str,
    ) -> Result<String, OracleError> {
        // Get the API key
        let api_key = self
            .api_keys
            .read()
            .await
            .get(api_key_id)
            .cloned()
            .ok_or_else(|| OracleError::Authentication("Invalid API key".to_string()))?;

        // Create JWT claims
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let claims = Claims {
            sub: user_id.to_string(),
            iat: now,
            exp: now + 3600, // 1 hour expiration
            kid: api_key_id.to_string(),
            permissions: api_key.permissions.clone(),
        };

        // Encode the JWT
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| OracleError::Authentication(format!("Failed to generate token: {}", e)))?;

        Ok(token)
    }

    /// Verify a JWT token
    pub fn verify_token(&self, token: &str) -> Result<Claims, OracleError> {
        // Decode and verify the JWT
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| OracleError::Authentication(format!("Invalid token: {}", e)))?;

        // Check if the token has expired
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        if token_data.claims.exp < now {
            return Err(OracleError::Authentication("Token has expired".to_string()));
        }

        Ok(token_data.claims)
    }

    /// Check if a user has a specific permission
    pub fn has_permission(&self, claims: &Claims, permission: &str) -> bool {
        claims.permissions.contains(&permission.to_string())
    }
}
