// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

use crate::{
    OracleError, OracleProvider, OracleRequest, OracleRequestStatus, OracleRequestType, OracleResponse, OracleService,
};
use crate::auth::AuthService;
use crate::provider::ProviderRegistry;

/// Oracle service implementation
pub struct OracleServiceImpl {
    /// Provider registry
    provider_registry: Arc<ProviderRegistry>,
    
    /// Authentication service
    auth_service: Arc<AuthService>,
    
    /// Request storage
    requests: Arc<RwLock<HashMap<String, OracleRequest>>>,
    
    /// Response storage
    responses: Arc<RwLock<HashMap<String, OracleResponse>>>,
    
    /// Request channel
    request_tx: mpsc::Sender<OracleRequest>,
    
    /// Request channel receiver
    request_rx: Arc<RwLock<Option<mpsc::Receiver<OracleRequest>>>>,
}

impl OracleServiceImpl {
    /// Create a new Oracle service
    pub fn new(
        provider_registry: Arc<ProviderRegistry>,
        auth_service: Arc<AuthService>,
    ) -> Self {
        let (request_tx, request_rx) = mpsc::channel(100);
        
        Self {
            provider_registry,
            auth_service,
            requests: Arc::new(RwLock::new(HashMap::new())),
            responses: Arc::new(RwLock::new(HashMap::new())),
            request_tx,
            request_rx: Arc::new(RwLock::new(Some(request_rx))),
        }
    }
    
    /// Send callback to the specified URL
    async fn send_callback(callback_url: &str, response: &OracleResponse) -> Result<(), OracleError> {
        // Create a reqwest client
        let client = reqwest::Client::new();
        
        // Serialize the response to JSON
        let response_json = serde_json::to_string(response)
            .map_err(|e| OracleError::Internal(format!("Failed to serialize response: {}", e)))?;
        
        // Send the callback
        let result = client.post(callback_url)
            .header("Content-Type", "application/json")
            .body(response_json)
            .send()
            .await
            .map_err(|e| OracleError::Network(format!("Failed to send callback: {}", e)))?;
        
        // Check the status code
        if !result.status().is_success() {
            return Err(OracleError::Network(format!(
                "Callback failed with status code: {}",
                result.status()
            )));
        }
        
        Ok(())
    }
    
    /// Send response to blockchain gateway
    async fn send_to_blockchain_gateway(
        blockchain_info: &crate::types::BlockchainInfo,
        response: &OracleResponse,
    ) -> Result<(), OracleError> {
        // Create a gateway client
        let gateway = crate::gateway::BlockchainGateway::new(&blockchain_info.chain);
        
        // Send the response to the blockchain
        gateway.send_oracle_response(blockchain_info, response).await
            .map_err(|e| OracleError::Network(format!("Failed to send response to blockchain: {}", e)))
    }
    
    /// Update price feed on blockchain
    pub async fn update_price_feed(&self, price_data: &crate::types::PriceData) -> Result<String, OracleError> {
        // Check if price data has an index
        if price_data.index.is_none() {
            return Err(OracleError::Validation("Price data has no index".to_string()));
        }
        
        // Create a blockchain gateway service for Neo
        let client = reqwest::Client::new();
        let client_arc = Arc::new(client);
        
        // Create a price index registry
        let index_registry = Arc::new(crate::registry::PriceIndexRegistry::new());
        
        // Initialize default mappings
        index_registry.initialize_defaults().await;
        
        // Create a Neo blockchain gateway service
        let gateway_service = crate::gateway::NeoBlockchainGatewayService::new(
            client_arc,
            "NeoOracleWallet".to_string(), // This should be configurable
            "0x1234567890abcdef1234567890abcdef12345678".to_string(), // This should be configurable
            index_registry,
        );
        
        // Update price data on blockchain
        gateway_service.update_price_data(price_data).await
    }
    
    /// Start the Oracle service
    pub async fn start(&self) -> Result<(), OracleError> {
        let mut rx = self.request_rx.write().await.take()
            .ok_or_else(|| OracleError::Internal("Request receiver already taken".to_string()))?;
        
        let provider_registry = Arc::clone(&self.provider_registry);
        let requests = Arc::clone(&self.requests);
        let responses = Arc::clone(&self.responses);
        
        // Spawn a task to process requests
        tokio::spawn(async move {
            while let Some(request) = rx.recv().await {
                // Update request status
                {
                    let mut requests_lock = requests.write().await;
                    if let Some(req) = requests_lock.get_mut(&request.id) {
                        req.status = OracleRequestStatus::Processing;
                    }
                }
                
                // Process the request
                let result = provider_registry.process_request(&request).await;
                
                // Update request status and store response
                {
                    let mut requests_lock = requests.write().await;
                    if let Some(req) = requests_lock.get_mut(&request.id) {
                        req.status = match &result {
                            Ok(_) => OracleRequestStatus::Completed,
                            Err(_) => OracleRequestStatus::Failed,
                        };
                    }
                }
                
                match result {
                    Ok(response) => {
                        responses.write().await.insert(request.id.clone(), response);
                    }
                    Err(err) => {
                        log::error!("Failed to process request {}: {}", request.id, err);
                        
                        // Create an error response
                        let error_response = OracleResponse {
                            request_id: request.id.clone(),
                            data: "".to_string(),
                            status_code: 500,
                            timestamp: SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_secs(),
                            error: Some(err.to_string()),
                        };
                        
                        responses.write().await.insert(request.id.clone(), error_response);
                    }
                }
                
                // Send callback if callback_url is provided
                if let Some(callback_url) = &request.callback_url {
                    let response_clone = match result {
                        Ok(ref response) => response.clone(),
                        Err(ref err) => {
                            // Create an error response for the callback
                            OracleResponse {
                                request_id: request.id.clone(),
                                data: "".to_string(),
                                status_code: 500,
                                timestamp: SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap_or_default()
                                    .as_secs(),
                                error: Some(err.to_string()),
                            }
                        }
                    };
                    
                    // Send the callback asynchronously
                    let callback_url = callback_url.clone();
                    tokio::spawn(async move {
                        match Self::send_callback(&callback_url, &response_clone).await {
                            Ok(_) => {
                                log::info!("Callback sent successfully to {}", callback_url);
                            }
                            Err(err) => {
                                log::error!("Failed to send callback to {}: {}", callback_url, err);
                            }
                        }
                    });
                }
                
                // If this is a blockchain request, send the response to the blockchain gateway
                if let OracleRequestType::Blockchain(blockchain_info) = &request.request_type {
                    let response_clone = match result {
                        Ok(ref response) => response.clone(),
                        Err(ref err) => {
                            // Create an error response for the blockchain
                            OracleResponse {
                                request_id: request.id.clone(),
                                data: "".to_string(),
                                status_code: 500,
                                timestamp: SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap_or_default()
                                    .as_secs(),
                                error: Some(err.to_string()),
                            }
                        }
                    };
                    
                    // Send the response to the blockchain gateway asynchronously
                    let blockchain_info = blockchain_info.clone();
                    tokio::spawn(async move {
                        match Self::send_to_blockchain_gateway(&blockchain_info, &response_clone).await {
                            Ok(_) => {
                                log::info!("Response sent to blockchain gateway for chain {}", blockchain_info.chain);
                            }
                            Err(err) => {
                                log::error!("Failed to send response to blockchain gateway for chain {}: {}", 
                                           blockchain_info.chain, err);
                            }
                        }
                    });
                }
            }
        });
        
        Ok(())
    }
    
    /// Convert a Neo Oracle response to the internal format
    pub fn convert_neo_oracle_response(
        neo_response: &r3e_event::source::NeoOracleResponse,
    ) -> OracleResponse {
        OracleResponse {
            request_id: neo_response.id.to_string(),
            data: neo_response.result.clone(),
            status_code: match neo_response.code {
                r3e_event::source::NeoOracleCode::Success => 200,
                r3e_event::source::NeoOracleCode::ProtocolNotSupported => 400,
                r3e_event::source::NeoOracleCode::ConsensusUnreachable => 503,
                r3e_event::source::NeoOracleCode::NotFound => 404,
                r3e_event::source::NeoOracleCode::Timeout => 408,
                r3e_event::source::NeoOracleCode::Forbidden => 403,
                r3e_event::source::NeoOracleCode::ResponseTooLarge => 413,
                r3e_event::source::NeoOracleCode::InsufficientFunds => 402,
                r3e_event::source::NeoOracleCode::ContentTypeNotSupported => 415,
                r3e_event::source::NeoOracleCode::Error => 500,
            },
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            error: if neo_response.code != r3e_event::source::NeoOracleCode::Success {
                Some(format!("Neo Oracle error: {:?}", neo_response.code))
            } else {
                None
            },
        }
    }
    
    /// Convert an internal Oracle response to Neo format
    pub fn convert_to_neo_oracle_response(
        response: &OracleResponse,
    ) -> r3e_event::source::NeoOracleResponse {
        r3e_event::source::NeoOracleResponse {
            id: response.request_id.parse().unwrap_or(0),
            code: match response.status_code {
                200 => r3e_event::source::NeoOracleCode::Success,
                400 => r3e_event::source::NeoOracleCode::ProtocolNotSupported,
                503 => r3e_event::source::NeoOracleCode::ConsensusUnreachable,
                404 => r3e_event::source::NeoOracleCode::NotFound,
                408 => r3e_event::source::NeoOracleCode::Timeout,
                403 => r3e_event::source::NeoOracleCode::Forbidden,
                413 => r3e_event::source::NeoOracleCode::ResponseTooLarge,
                402 => r3e_event::source::NeoOracleCode::InsufficientFunds,
                415 => r3e_event::source::NeoOracleCode::ContentTypeNotSupported,
                _ => r3e_event::source::NeoOracleCode::Error,
            },
            result: response.data.clone(),
        }
    }
}

#[async_trait::async_trait]
impl OracleService for OracleServiceImpl {
    async fn submit_request(&self, request: OracleRequest) -> Result<String, OracleError> {
        // Store the request
        self.requests.write().await.insert(request.id.clone(), request.clone());
        
        // Send the request to the processing queue
        self.request_tx.send(request.clone()).await
            .map_err(|e| OracleError::Internal(format!("Failed to send request: {}", e)))?;
        
        Ok(request.id)
    }
    
    async fn get_request_status(&self, request_id: &str) -> Result<OracleRequestStatus, OracleError> {
        // Get the request
        let request = self.requests.read().await.get(request_id).cloned()
            .ok_or_else(|| OracleError::Validation(format!("Request not found: {}", request_id)))?;
        
        Ok(request.status)
    }
    
    async fn get_response(&self, request_id: &str) -> Result<OracleResponse, OracleError> {
        // Get the response
        let response = self.responses.read().await.get(request_id).cloned()
            .ok_or_else(|| OracleError::Validation(format!("Response not found: {}", request_id)))?;
        
        Ok(response)
    }
    
    async fn cancel_request(&self, request_id: &str) -> Result<bool, OracleError> {
        // Get the request
        let mut requests = self.requests.write().await;
        let request = requests.get_mut(request_id)
            .ok_or_else(|| OracleError::Validation(format!("Request not found: {}", request_id)))?;
        
        // Check if the request can be canceled
        if request.status != OracleRequestStatus::Pending && request.status != OracleRequestStatus::Processing {
            return Err(OracleError::Validation(format!(
                "Request cannot be canceled: {:?}",
                request.status
            )));
        }
        
        // Update request status
        request.status = OracleRequestStatus::Failed;
        
        // Create an error response
        let error_response = OracleResponse {
            request_id: request_id.to_string(),
            data: "".to_string(),
            status_code: 499, // Client Closed Request
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            error: Some("Request canceled".to_string()),
        };
        
        // Store the response
        self.responses.write().await.insert(request_id.to_string(), error_response);
        
        Ok(true)
    }
}

/// Create a new Oracle request
pub fn create_oracle_request(
    request_type: OracleRequestType,
    data: String,
    callback_url: Option<String>,
    requester_id: String,
) -> OracleRequest {
    let id = Uuid::new_v4().to_string();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    OracleRequest {
        id,
        request_type,
        data,
        callback_url,
        requester_id,
        timestamp,
        status: OracleRequestStatus::Pending,
    }
}
