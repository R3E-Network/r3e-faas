// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;

use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::Error, service::EndpointService, utils::generate_jwt_token};

/// Wallet connection request
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectWalletRequest {
    /// Blockchain type (ethereum, neo_n3, solana, etc.)
    pub blockchain_type: String,

    /// Wallet address
    pub address: String,
}

/// Challenge response for wallet connection
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectWalletResponse {
    /// Challenge message to sign
    pub challenge: String,

    /// Challenge ID for verification
    pub challenge_id: String,

    /// Challenge expiration timestamp
    pub expires_at: u64,
}

/// Wallet authentication request
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticateWalletRequest {
    /// Challenge ID from the connection step
    pub challenge_id: String,

    /// Blockchain type (ethereum, neo_n3, solana, etc.)
    pub blockchain_type: String,

    /// Wallet address
    pub address: String,

    /// Signature of the challenge message
    pub signature: String,

    /// Signature curve (if needed)
    pub signature_curve: Option<String>,
}

/// Wallet authentication response
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticateWalletResponse {
    /// User ID
    pub user_id: String,

    /// Wallet address
    pub address: String,

    /// Blockchain type
    pub blockchain_type: String,

    /// JWT token
    pub token: String,

    /// Token expiration
    pub expires_at: u64,
}

/// Connect wallet handler - Step 1 of wallet authentication
pub async fn connect_wallet(
    State(service): State<Arc<EndpointService>>,
    Json(request): Json<ConnectWalletRequest>,
) -> Result<Json<ConnectWalletResponse>, Error> {
    // Validate the blockchain type
    let blockchain_type = match request.blockchain_type.to_lowercase().as_str() {
        "ethereum" | "neo_n3" | "solana" => request.blockchain_type.to_lowercase(),
        _ => {
            return Err(Error::Validation(format!(
                "Unsupported blockchain type: {}",
                request.blockchain_type
            )))
        }
    };

    // Validate the wallet address format based on blockchain type
    validate_address(&blockchain_type, &request.address)?;

    // Generate a random challenge
    let challenge_id = Uuid::new_v4().to_string();
    let timestamp = Utc::now().timestamp() as u64;

    // Create a challenge message that includes address and timestamp to prevent replay attacks
    let challenge = format!(
        "Sign this message to authenticate with R3E FaaS platform.\n\nWallet: {}\nTimestamp: {}\nNonce: {}",
        request.address,
        timestamp,
        challenge_id
    );

    // Store the challenge in the database with expiration
    let expires_at = timestamp + 300; // 5 minutes expiration
    service
        .db_client
        .store_auth_challenge(
            &challenge_id,
            &request.address,
            &blockchain_type,
            &challenge,
            expires_at,
        )
        .await
        .map_err(|e| Error::Internal(format!("Failed to store challenge: {}", e)))?;

    // Return the challenge for the user to sign
    let response = ConnectWalletResponse {
        challenge,
        challenge_id,
        expires_at,
    };

    log::info!(
        "Generated challenge for wallet: {} ({})",
        request.address,
        blockchain_type
    );
    Ok(Json(response))
}

/// Authenticate wallet handler - Step 2 of wallet authentication
pub async fn authenticate_wallet(
    State(service): State<Arc<EndpointService>>,
    Json(request): Json<AuthenticateWalletRequest>,
) -> Result<Json<AuthenticateWalletResponse>, Error> {
    // Retrieve the challenge from the database
    let challenge = service
        .db_client
        .get_auth_challenge(&request.challenge_id)
        .await
        .map_err(|e| Error::Internal(format!("Database error: {}", e)))?;

    // Check if challenge exists
    let challenge = match challenge {
        Some(c) => c,
        None => {
            log::warn!("Challenge not found: {}", request.challenge_id);
            return Err(Error::Authentication("Invalid or expired challenge".into()));
        }
    };

    // Check if challenge has expired
    let now = Utc::now().timestamp() as u64;
    if now > challenge.expires_at {
        log::warn!("Challenge expired: {}", request.challenge_id);
        return Err(Error::Authentication("Challenge has expired".into()));
    }

    // Verify that the address and blockchain type match the stored challenge
    if request.address != challenge.address || request.blockchain_type != challenge.blockchain_type
    {
        log::warn!(
            "Address/blockchain mismatch for challenge: {} (got: {}/{}, expected: {}/{})",
            request.challenge_id,
            request.address,
            request.blockchain_type,
            challenge.address,
            challenge.blockchain_type
        );
        return Err(Error::Authentication(
            "Address or blockchain type mismatch".into(),
        ));
    }

    // Get the default signature curve if not provided
    let signature_curve =
        request
            .signature_curve
            .unwrap_or_else(|| match request.blockchain_type.as_str() {
                "ethereum" => "secp256k1".to_string(),
                "neo_n3" => "secp256r1".to_string(),
                "solana" => "ed25519".to_string(),
                _ => "unknown".to_string(),
            });

    // Verify the signature
    let signature_valid = crate::utils::verify_signature(
        &request.blockchain_type,
        &signature_curve,
        &request.address,
        &challenge.message,
        &request.signature,
    )?;

    if !signature_valid {
        log::warn!("Invalid signature for wallet: {}", request.address);
        return Err(Error::Authentication("Invalid signature".into()));
    }

    // Look up user by wallet address, or create a new user if not found
    let user = service
        .db_client
        .find_user_by_wallet_address(&request.blockchain_type, &request.address)
        .await
        .map_err(|e| Error::Internal(format!("Database error: {}", e)))?;

    let user_id = match user {
        Some(user) => {
            log::info!("Existing user found for wallet: {}", request.address);
            user.id
        }
        None => {
            // Create a new user for this wallet address
            let user_id = Uuid::new_v4().to_string();

            service
                .db_client
                .create_wallet_user(&user_id, &request.address, &request.blockchain_type)
                .await
                .map_err(|e| Error::Internal(format!("Failed to create user: {}", e)))?;

            log::info!("Created new user for wallet: {}", request.address);
            user_id
        }
    };

    // Create a new session
    let connection_id = Uuid::new_v4().to_string();

    // Generate JWT token
    let token = generate_jwt_token(
        &request.address,
        &request.blockchain_type,
        &connection_id,
        &service.config.jwt_secret,
        service.config.jwt_expiration,
    )?;

    // Store the session in the database
    service
        .db_client
        .create_session(&user_id, &connection_id, &token)
        .await
        .map_err(|e| Error::Internal(format!("Failed to create session: {}", e)))?;

    // Delete the used challenge
    let _ = service
        .db_client
        .delete_auth_challenge(&request.challenge_id)
        .await;

    let response = AuthenticateWalletResponse {
        user_id,
        address: request.address,
        blockchain_type: request.blockchain_type,
        token,
        expires_at: Utc::now().timestamp() as u64 + service.config.jwt_expiration,
    };

    log::info!("Wallet authentication successful: {}", request.address);
    Ok(Json(response))
}

/// Helper function to validate wallet address format
fn validate_address(blockchain_type: &str, address: &str) -> Result<(), Error> {
    match blockchain_type {
        "ethereum" => {
            // Ethereum addresses are 0x followed by 40 hex characters
            if !address.starts_with("0x") || address.len() != 42 {
                return Err(Error::Validation("Invalid Ethereum address format".into()));
            }

            // Check if the remainder is valid hex
            if let Err(e) = hex::decode(&address[2..]) {
                return Err(Error::Validation(format!(
                    "Invalid Ethereum address hex: {}",
                    e
                )));
            }
        }
        "neo_n3" => {
            // Neo N3 addresses typically start with 'N' and are 34 characters long
            if !address.starts_with("N") || address.len() != 34 {
                return Err(Error::Validation("Invalid Neo N3 address format".into()));
            }

            // Ideally we would verify if it's a valid Neo address, but for now just check format
        }
        "solana" => {
            // Solana addresses are base58 encoded and typically 32-44 characters
            if address.len() < 32 || address.len() > 44 {
                return Err(Error::Validation("Invalid Solana address format".into()));
            }

            // Attempt to decode the base58 address
            if let Err(e) = bs58::decode(address).into_vec() {
                return Err(Error::Validation(format!("Invalid Solana address: {}", e)));
            }
        }
        _ => {
            return Err(Error::Validation(format!(
                "Unsupported blockchain type: {}",
                blockchain_type
            )));
        }
    }

    Ok(())
}
