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
                
                // TODO: Send callback if callback_url is provided
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
