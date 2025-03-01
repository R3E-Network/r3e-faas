// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use serde::{Deserialize, Serialize};
use std::env;

use crate::error::Error;

/// Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Server port
    pub port: u16,

    /// Database URL
    pub database_url: String,

    /// JWT secret
    pub jwt_secret: String,

    /// JWT expiration (in seconds)
    pub jwt_expiration: u64,

    /// Neo N3 RPC URL
    pub neo_rpc_url: String,

    /// Ethereum RPC URL
    pub eth_rpc_url: String,

    /// Relayer wallet private key
    pub relayer_private_key: String,
    
    /// Rate limit (requests per minute)
    pub rate_limit_requests_per_minute: u32,
}

impl Config {
    /// Create a new configuration from environment variables
    pub fn from_env() -> Result<Self, Error> {
        // Load .env file if it exists
        dotenv::dotenv().ok();

        // Get the port
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .map_err(|e| Error::Configuration(format!("Invalid port: {}", e)))?;

        // Get the database URL
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| Error::Configuration("DATABASE_URL is not set".to_string()))?;

        // Get the JWT secret
        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| Error::Configuration("JWT_SECRET is not set".to_string()))?;

        // Get the JWT expiration
        let jwt_expiration = env::var("JWT_EXPIRATION")
            .unwrap_or_else(|_| "86400".to_string())
            .parse::<u64>()
            .map_err(|e| Error::Configuration(format!("Invalid JWT expiration: {}", e)))?;

        // Get the Neo N3 RPC URL
        let neo_rpc_url =
            env::var("NEO_RPC_URL").unwrap_or_else(|_| "https://rpc.neo.org:443".to_string());

        // Get the Ethereum RPC URL
        let eth_rpc_url = env::var("ETH_RPC_URL")
            .unwrap_or_else(|_| "https://mainnet.infura.io/v3/your-api-key".to_string());

        // Get the relayer wallet private key
        let relayer_private_key = env::var("RELAYER_PRIVATE_KEY")
            .map_err(|_| Error::Configuration("RELAYER_PRIVATE_KEY is not set".to_string()))?;

        // Get the rate limit (requests per minute)
        let rate_limit_requests_per_minute = env::var("RATE_LIMIT_REQUESTS_PER_MINUTE")
            .unwrap_or_else(|_| "60".to_string())
            .parse::<u32>()
            .map_err(|e| Error::Configuration(format!("Invalid rate limit: {}", e)))?;

        Ok(Self {
            port,
            database_url,
            jwt_secret,
            jwt_expiration,
            neo_rpc_url,
            eth_rpc_url,
            relayer_private_key,
            rate_limit_requests_per_minute,
        })
    }
}
