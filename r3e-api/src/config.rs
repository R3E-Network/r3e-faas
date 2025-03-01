// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use serde::{Deserialize, Serialize};
use std::env;

/// API service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Server port
    pub port: u16,

    /// Database URL
    pub database_url: String,

    /// JWT secret
    pub jwt_secret: String,

    /// JWT expiration in seconds
    pub jwt_expiration: u64,

    /// Neo N3 RPC URL
    pub neo_rpc_url: String,

    /// Oracle service URL
    pub oracle_service_url: Option<String>,

    /// TEE service URL
    pub tee_service_url: Option<String>,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        Self {
            port: env::var("API_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),

            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgres://postgres:postgres@localhost:5432/r3e_faas".to_string()
            }),

            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "r3e_faas_secret".to_string()),

            jwt_expiration: env::var("JWT_EXPIRATION")
                .unwrap_or_else(|_| "86400".to_string())
                .parse()
                .unwrap_or(86400),

            neo_rpc_url: env::var("NEO_RPC_URL")
                .unwrap_or_else(|_| "http://localhost:10332".to_string()),

            oracle_service_url: env::var("ORACLE_SERVICE_URL").ok(),

            tee_service_url: env::var("TEE_SERVICE_URL").ok(),
        }
    }
}
