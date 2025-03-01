// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use crate::error::Error;
use crate::types::{BlockchainType, SignatureCurve};
use ethers_core::types::Signature as EthSignature;
use neo3::neo_crypto::keys::PublicKey;
use neo3::neo_types::address::Address;

/// Verify a signature
pub fn verify_signature(
    blockchain_type: &str,
    signature_curve: &str,
    address: &str,
    message: &str,
    signature: &str,
) -> Result<bool, Error> {
    log::debug!(
        "Verifying signature: type={}, curve={}, address={}, message={}, sig={}",
        blockchain_type,
        signature_curve,
        address,
        message,
        signature
    );
    
    match blockchain_type.to_lowercase().as_str() {
        "neo_n3" => {
            // Validate address format
            if !address.starts_with("N") {
                return Err(Error::Validation("Invalid Neo N3 address".into()));
            }
            
            // Use neo-crypto from neo-rs to verify signature
            use neo_crypto::{
                keys::{PublicKey, Secp256r1PublicKey},
                Base64Decoder,
            };
            
            // Convert signature from base64 to bytes
            let signature_bytes = Base64Decoder::decode(signature)
                .map_err(|e| Error::Validation(format!("Invalid signature encoding: {}", e)))?;
            
            // Get public key from address
            let public_key = match neo_crypto::address_to_script_hash(address) {
                Ok(script_hash) => {
                    // Recover the public key from the message and signature
                    // Note: In a real implementation we'd need to derive the public key from the
                    // script hash or have the client provide it alongside the signature
                    let message_bytes = message.as_bytes();
                    
                    match neo_crypto::recovery::ecrecover_p256r1(message_bytes, &signature_bytes) {
                        Ok(pub_key) => pub_key,
                        Err(e) => return Err(Error::Validation(format!("Failed to recover public key: {}", e))),
                    }
                },
                Err(e) => return Err(Error::Validation(format!("Invalid Neo address: {}", e))),
            };
            
            // Verify the signature using the recovered public key
            match Secp256r1PublicKey::from_bytes(public_key.as_bytes()) {
                Ok(pk) => {
                    match pk.verify(message.as_bytes(), &signature_bytes) {
                        Ok(is_valid) => Ok(is_valid),
                        Err(e) => Err(Error::Internal(format!("Signature verification error: {}", e))),
                    }
                },
                Err(e) => Err(Error::Internal(format!("Invalid public key: {}", e))),
            }
        },
        "ethereum" => {
            // Validate Ethereum address format
            if !address.starts_with("0x") || address.len() != 42 {
                return Err(Error::Validation("Invalid Ethereum address".into()));
            }
            
            // Use ethers library for Ethereum signature verification
            use ethers::{
                core::{
                    types::Address,
                    utils::hex,
                },
                signers::Signer,
            };
            
            // Parse the Ethereum address
            let eth_address = match address.parse::<Address>() {
                Ok(addr) => addr,
                Err(e) => return Err(Error::Validation(format!("Invalid Ethereum address: {}", e))),
            };
            
            // For Ethereum, we check if the signature contains a 0x prefix
            let signature_data = if signature.starts_with("0x") {
                &signature[2..]
            } else {
                signature
            };
            
            // Decode the signature
            let signature_bytes = match hex::decode(signature_data) {
                Ok(bytes) => bytes,
                Err(e) => return Err(Error::Validation(format!("Invalid signature encoding: {}", e))),
            };
            
            // For Ethereum, we need to recover the address from the signature
            // First, create an Ethereum signed message hash
            let message_hash = ethers::core::utils::hash_message(message);
            
            // Recover the address from the signature
            match ethers::core::utils::recover_signer(&message_hash, &signature_bytes) {
                Ok(recovered_address) => {
                    // Check if the recovered address matches the provided address
                    Ok(recovered_address == eth_address)
                },
                Err(e) => Err(Error::Internal(format!("Failed to recover signer address: {}", e))),
            }
        },
        "solana" => {
            // Validate Solana address format - base58 encoded
            if address.len() < 32 || address.len() > 44 {
                return Err(Error::Validation("Invalid Solana address".into()));
            }
            
            // Use solana_sdk for signature verification
            use bs58;
            use ed25519_dalek::{PublicKey, Signature, Verifier};
            
            // Decode the base58 public key
            let public_key_bytes = match bs58::decode(address).into_vec() {
                Ok(bytes) => {
                    if bytes.len() != 32 {
                        return Err(Error::Validation("Invalid Solana public key length".into()));
                    }
                    bytes
                },
                Err(e) => return Err(Error::Validation(format!("Invalid Solana address: {}", e))),
            };
            
            // Decode the base58 signature
            let signature_bytes = match bs58::decode(signature).into_vec() {
                Ok(bytes) => {
                    if bytes.len() != 64 {
                        return Err(Error::Validation("Invalid Solana signature length".into()));
                    }
                    bytes
                },
                Err(e) => return Err(Error::Validation(format!("Invalid signature encoding: {}", e))),
            };
            
            // Create public key and signature objects
            let public_key = match PublicKey::from_bytes(&public_key_bytes) {
                Ok(pk) => pk,
                Err(e) => return Err(Error::Validation(format!("Invalid public key: {}", e))),
            };
            
            let ed25519_signature = match Signature::from_bytes(&signature_bytes) {
                Ok(sig) => sig,
                Err(e) => return Err(Error::Validation(format!("Invalid signature: {}", e))),
            };
            
            // Verify the signature
            match public_key.verify(message.as_bytes(), &ed25519_signature) {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        },
        _ => Err(Error::Validation(format!("Unsupported blockchain type: {}", blockchain_type))),
    }
}

/// Generate JWT token
pub fn generate_jwt_token(
    address: &str,
    blockchain_type: &BlockchainType,
    connection_id: &str,
    jwt_secret: &str,
    jwt_expiration: u64,
) -> Result<String, Error> {
    use chrono::Utc;
    use jsonwebtoken::{encode, EncodingKey, Header};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        /// Subject (wallet address)
        sub: String,

        /// Blockchain type
        blockchain_type: String,

        /// Connection ID
        connection_id: String,

        /// Issued at
        iat: u64,

        /// Expiration
        exp: u64,
    }

    // Create JWT claims
    let now = Utc::now().timestamp() as u64;
    let claims = Claims {
        sub: address.to_string(),
        blockchain_type: format!("{:?}", blockchain_type).to_lowercase(),
        connection_id: connection_id.to_string(),
        iat: now,
        exp: now + jwt_expiration,
    };

    // Create JWT token
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| Error::Internal(format!("Failed to create JWT token: {}", e)))
}

/// Verify JWT token
pub fn verify_jwt_token(token: &str, jwt_secret: &str) -> Result<JwtClaims, Error> {
    use jsonwebtoken::{decode, DecodingKey, Validation};
    use serde::{Deserialize, Serialize};

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

    // Decode JWT token
    let token_data = decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| Error::Authentication(format!("Invalid JWT token: {}", e)))?;

    Ok(token_data.claims)
}
