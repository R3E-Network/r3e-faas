// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use rand::Rng;
use sha2::Sha256;
use std::collections::BTreeMap;
use uuid::Uuid;

use crate::error::ApiError;

/// Hash a password using Argon2
pub fn hash_password(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| ApiError::Server(format!("Failed to hash password: {}", e)))?
        .to_string();
    
    Ok(password_hash)
}

/// Verify a password against a hash
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, ApiError> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| ApiError::Server(format!("Failed to parse password hash: {}", e)))?;
    
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Generate a JWT token
pub fn generate_jwt(
    user_id: Uuid,
    username: &str,
    role: &str,
    secret: &str,
    expiration: u64,
) -> Result<String, ApiError> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())
        .map_err(|e| ApiError::Server(format!("Failed to create HMAC key: {}", e)))?;
    
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(expiration as i64))
        .ok_or_else(|| ApiError::Server("Failed to calculate token expiration".to_string()))?
        .timestamp();
    
    let mut claims = BTreeMap::new();
    claims.insert("sub", user_id.to_string());
    claims.insert("username", username.to_string());
    claims.insert("role", role.to_string());
    claims.insert("exp", expiration.to_string());
    
    let token = claims
        .sign_with_key(&key)
        .map_err(|e| ApiError::Server(format!("Failed to sign JWT: {}", e)))?;
    
    Ok(token)
}

/// Verify a JWT token
pub fn verify_jwt(token: &str, secret: &str) -> Result<BTreeMap<String, String>, ApiError> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())
        .map_err(|e| ApiError::Server(format!("Failed to create HMAC key: {}", e)))?;
    
    let claims: BTreeMap<String, String> = token
        .verify_with_key(&key)
        .map_err(|e| ApiError::Authentication(format!("Invalid token: {}", e)))?;
    
    // Check if the token has expired
    if let Some(exp) = claims.get("exp") {
        let exp = exp
            .parse::<i64>()
            .map_err(|e| ApiError::Authentication(format!("Invalid token expiration: {}", e)))?;
        
        let now = chrono::Utc::now().timestamp();
        
        if now > exp {
            return Err(ApiError::Authentication("Token has expired".to_string()));
        }
    }
    
    Ok(claims)
}

/// Generate a random API key
pub fn generate_api_key() -> String {
    let mut rng = rand::thread_rng();
    let key: Vec<u8> = (0..32).map(|_| rng.gen::<u8>()).collect();
    
    base64::encode(&key)
}

/// Sign a message using HMAC-SHA256
pub fn sign_message(message: &str, secret: &str) -> Result<String, ApiError> {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .map_err(|e| ApiError::Server(format!("Failed to create HMAC: {}", e)))?;
    
    mac.update(message.as_bytes());
    
    let result = mac.finalize();
    let signature = base64::encode(result.into_bytes());
    
    Ok(signature)
}

/// Verify a message signature
pub fn verify_signature(message: &str, signature: &str, secret: &str) -> Result<bool, ApiError> {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .map_err(|e| ApiError::Server(format!("Failed to create HMAC: {}", e)))?;
    
    mac.update(message.as_bytes());
    
    let signature = base64::decode(signature)
        .map_err(|e| ApiError::Validation(format!("Invalid signature: {}", e)))?;
    
    Ok(mac.verify_slice(&signature).is_ok())
}
