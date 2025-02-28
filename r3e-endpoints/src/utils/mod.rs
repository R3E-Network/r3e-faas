// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use crate::error::Error;
use crate::types::{BlockchainType, SignatureCurve};
use ethers_core::types::Signature as EthSignature;
use NeoRust::neo_crypto::keys::PublicKey;
use NeoRust::neo_types::address::Address;

/// Verify signature
pub fn verify_signature(
    blockchain_type: &BlockchainType,
    signature_curve: &SignatureCurve,
    address: &str,
    message: &str,
    signature: &str,
) -> Result<bool, Error> {
    match (blockchain_type, signature_curve) {
        (BlockchainType::NeoN3, SignatureCurve::Secp256r1) => {
            // Verify Neo N3 signature using secp256r1 curve
            // In a real implementation, this would use the NeoRust SDK to verify the signature
            // For this example, we'll assume it's valid if the address is valid
            match Address::from_str(address) {
                Ok(_) => Ok(true),
                Err(e) => Err(Error::Validation(format!("Invalid Neo N3 address: {}", e))),
            }
        },
        (BlockchainType::Ethereum, SignatureCurve::Secp256k1) => {
            // Verify Ethereum signature using secp256k1 curve
            // In a real implementation, this would use the ethers crate to verify the signature
            // For this example, we'll assume it's valid if the signature is valid
            match EthSignature::from_str(signature) {
                Ok(_) => Ok(true),
                Err(e) => Err(Error::Validation(format!("Invalid Ethereum signature: {}", e))),
            }
        },
        _ => {
            // Invalid combination
            Err(Error::Validation(format!(
                "Invalid blockchain type and signature curve combination: {:?}, {:?}",
                blockchain_type, signature_curve
            )))
        }
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
pub fn verify_jwt_token(
    token: &str,
    jwt_secret: &str,
) -> Result<JwtClaims, Error> {
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
