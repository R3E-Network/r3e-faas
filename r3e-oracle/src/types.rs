// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use serde::{Deserialize, Serialize};

/// Price data for a specific asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceData {
    /// Asset symbol (e.g., "NEO", "GAS")
    pub symbol: String,
    
    /// Price in USD
    pub price_usd: f64,
    
    /// Price source
    pub source: String,
    
    /// Timestamp of the price data
    pub timestamp: u64,
}

/// Price request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceRequest {
    /// Asset symbol (e.g., "NEO", "GAS")
    pub symbol: String,
    
    /// Currency to convert to (default: "USD")
    #[serde(default = "default_currency")]
    pub currency: String,
    
    /// Preferred sources (optional)
    #[serde(default)]
    pub sources: Vec<String>,
}

fn default_currency() -> String {
    "USD".to_string()
}

/// Price response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceResponse {
    /// Asset symbol
    pub symbol: String,
    
    /// Currency
    pub currency: String,
    
    /// Price value
    pub price: f64,
    
    /// Price sources used
    pub sources: Vec<String>,
    
    /// Timestamp
    pub timestamp: u64,
}

/// Random number request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomRequest {
    /// Minimum value (inclusive)
    #[serde(default)]
    pub min: u64,
    
    /// Maximum value (inclusive)
    #[serde(default = "default_max")]
    pub max: u64,
    
    /// Number of random values to generate
    #[serde(default = "default_count")]
    pub count: u32,
    
    /// Random number generation method
    #[serde(default)]
    pub method: RandomMethod,
    
    /// Optional seed for deterministic generation
    pub seed: Option<String>,
}

fn default_max() -> u64 {
    u64::MAX
}

fn default_count() -> u32 {
    1
}

/// Random number generation method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RandomMethod {
    /// Cryptographically secure random number
    #[serde(rename = "secure")]
    Secure,
    
    /// Blockchain-based random number
    #[serde(rename = "blockchain")]
    Blockchain,
    
    /// Verifiable random function
    #[serde(rename = "vrf")]
    Vrf,
}

impl Default for RandomMethod {
    fn default() -> Self {
        RandomMethod::Secure
    }
}

/// Random number response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomResponse {
    /// Generated random values
    pub values: Vec<u64>,
    
    /// Generation method used
    pub method: RandomMethod,
    
    /// Proof of randomness (for verifiable methods)
    pub proof: Option<String>,
    
    /// Timestamp
    pub timestamp: u64,
}

/// Weather request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherRequest {
    /// Location (city name, coordinates, etc.)
    pub location: String,
    
    /// Weather data type
    #[serde(default)]
    pub data_type: WeatherDataType,
}

/// Weather data type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeatherDataType {
    /// Current weather
    #[serde(rename = "current")]
    Current,
    
    /// Weather forecast
    #[serde(rename = "forecast")]
    Forecast,
    
    /// Historical weather data
    #[serde(rename = "historical")]
    Historical,
}

impl Default for WeatherDataType {
    fn default() -> Self {
        WeatherDataType::Current
    }
}

/// Weather response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherResponse {
    /// Location
    pub location: String,
    
    /// Weather data
    pub data: serde_json::Value,
    
    /// Weather data source
    pub source: String,
    
    /// Timestamp
    pub timestamp: u64,
}

/// Sports request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SportsRequest {
    /// Sport type
    pub sport: String,
    
    /// League
    pub league: Option<String>,
    
    /// Team
    pub team: Option<String>,
    
    /// Data type
    #[serde(default)]
    pub data_type: SportsDataType,
}

/// Sports data type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SportsDataType {
    /// Scores
    #[serde(rename = "scores")]
    Scores,
    
    /// Standings
    #[serde(rename = "standings")]
    Standings,
    
    /// Schedule
    #[serde(rename = "schedule")]
    Schedule,
    
    /// Statistics
    #[serde(rename = "stats")]
    Stats,
}

impl Default for SportsDataType {
    fn default() -> Self {
        SportsDataType::Scores
    }
}

/// Sports response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SportsResponse {
    /// Sport type
    pub sport: String,
    
    /// League
    pub league: Option<String>,
    
    /// Team
    pub team: Option<String>,
    
    /// Sports data
    pub data: serde_json::Value,
    
    /// Data source
    pub source: String,
    
    /// Timestamp
    pub timestamp: u64,
}

/// Custom request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomRequest {
    /// Request type
    pub request_type: String,
    
    /// Request parameters
    pub params: serde_json::Value,
}

/// Custom response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomResponse {
    /// Response data
    pub data: serde_json::Value,
    
    /// Data source
    pub source: String,
    
    /// Timestamp
    pub timestamp: u64,
}
