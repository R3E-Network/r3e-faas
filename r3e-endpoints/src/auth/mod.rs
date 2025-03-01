// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

pub mod key_rotation;

use crate::error::Error;
use crate::types::BlockchainType;
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, error, info, warn};

use r3e_secrets::service::SecretService;

/// JWT claims
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    /// Subject (wallet address)
    pub sub: String,

    /// Blockchain type
    pub blockchain_type: String,

    /// Connection ID
    pub connection_id: String,

    /// Issued at
    pub iat: u64,

    /// Expiration
    pub exp: u64,
}

/// Authentication service
pub struct AuthService {
    /// JWT secret
    jwt_secret: String,

    /// JWT expiration in seconds
    jwt_expiration: u64,

    /// Secret service for API keys
    secret_service: Arc<dyn SecretService>,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new(jwt_secret: String, jwt_expiration: u64, secret_service: Arc<dyn SecretService>) -> Self {
        Self {
            jwt_secret,
            jwt_expiration,
            secret_service,
        }
    }

    /// Generate JWT token
    pub fn generate_jwt_token(
        &self,
        address: &str,
        blockchain_type: &BlockchainType,
        connection_id: &str,
    ) -> Result<String, Error> {
        // Create JWT claims
        let now = Utc::now().timestamp() as u64;
        let claims = JwtClaims {
            sub: address.to_string(),
            blockchain_type: format!("{:?}", blockchain_type).to_lowercase(),
            connection_id: connection_id.to_string(),
            iat: now,
            exp: now + self.jwt_expiration,
        };

        // Create JWT token
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| Error::Internal(format!("Failed to create JWT token: {}", e)))
    }

    /// Verify JWT token
    pub fn verify_jwt_token(&self, token: &str) -> Result<JwtClaims, Error> {
        // Decode JWT token
        let token_data = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| Error::Authentication(format!("Invalid JWT token: {}", e)))?;

        Ok(token_data.claims)
    }

    /// Create API key
    pub async fn create_api_key(&self, user_id: &str) -> Result<(String, String), Error> {
        // Generate a new key
        let key_id = uuid::Uuid::new_v4().to_string();
        let key_value = uuid::Uuid::new_v4().to_string();

        // Generate a function key for encryption
        let function_key = r3e_secrets::SecretEncryption::generate_function_key();

        // Store the key in the secret service
        self.secret_service
            .store_secret(
                user_id,
                "api_keys",
                &key_id,
                key_value.as_bytes(),
                &function_key,
            )
            .await
            .map_err(|e| Error::Internal(format!("Failed to store API key: {}", e)))?;

        info!("Created API key: id={}, user_id={}", key_id, user_id);

        Ok((key_id, key_value))
    }

    /// Validate API key
    pub async fn validate_api_key(&self, key_id: &str, key_value: &str, user_id: &str) -> Result<bool, Error> {
        // Generate a function key for encryption (this would normally be retrieved from a secure store)
        let function_key = r3e_secrets::SecretEncryption::generate_function_key();

        // Get the stored key value
        let stored_value = match self.secret_service
            .get_secret(user_id, "api_keys", key_id, &function_key)
            .await {
                Ok(value) => String::from_utf8_lossy(&value).to_string(),
                Err(r3e_secrets::SecretError::NotFound(_)) => return Ok(false),
                Err(e) => return Err(Error::Internal(format!("Failed to get API key: {}", e))),
            };

        // Compare the key values
        Ok(stored_value == key_value)
    }

    /// Revoke API key
    pub async fn revoke_api_key(&self, key_id: &str, user_id: &str) -> Result<(), Error> {
        // Delete the key from the secret service
        self.secret_service
            .delete_secret(user_id, "api_keys", key_id)
            .await
            .map_err(|e| Error::Internal(format!("Failed to delete API key: {}", e)))?;

        info!("Revoked API key: id={}, user_id={}", key_id, user_id);

        Ok(())
    }
}
