// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::task::{Context, Poll};
use std::time::Instant;

use axum::http::{Request, Response};
use futures::future::BoxFuture;
use tower::{Layer, Service};
use tracing::{info, warn};

use crate::auth::key_rotation::KeyRotationService;

/// API key rotation layer
#[derive(Clone)]
pub struct KeyRotationLayer {
    /// Key rotation service
    key_rotation_service: KeyRotationService,
}

impl KeyRotationLayer {
    /// Create a new key rotation layer
    pub fn new(key_rotation_service: KeyRotationService) -> Self {
        Self {
            key_rotation_service,
        }
    }
}

impl<S> Layer<S> for KeyRotationLayer {
    type Service = KeyRotationMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        KeyRotationMiddleware {
            inner: service,
            key_rotation_service: self.key_rotation_service.clone(),
        }
    }
}

/// API key rotation middleware
#[derive(Clone)]
pub struct KeyRotationMiddleware<S> {
    /// Inner service
    inner: S,
    
    /// Key rotation service
    key_rotation_service: KeyRotationService,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for KeyRotationMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<ReqBody>) -> Self::Future {
        // Clone the service
        let mut inner = self.inner.clone();
        let key_rotation_service = self.key_rotation_service.clone();
        
        // Extract API key from request
        let api_key_id = request
            .headers()
            .get("X-API-Key-ID")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
            
        let user_id = request
            .headers()
            .get("X-User-ID")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        
        // Start timer
        let start = Instant::now();
        
        // Call the inner service
        Box::pin(async move {
            let result = inner.call(request).await;
            
            // Check if API key needs rotation
            if let (Some(key_id), Some(user_id)) = (api_key_id, user_id) {
                match key_rotation_service.needs_rotation(&key_id) {
                    Ok(true) => {
                        // Rotate the key
                        match key_rotation_service.rotate_key(&key_id, &user_id).await {
                            Ok((new_key_value, new_api_key)) => {
                                info!(
                                    "Rotated API key: old_id={}, new_id={}, user_id={}",
                                    key_id, new_api_key.id, new_api_key.user_id
                                );
                                
                                // Add the new key to the response headers
                                if let Ok(ref mut response) = result {
                                    response.headers_mut().insert(
                                        "X-New-API-Key-ID",
                                        new_api_key.id.parse().unwrap(),
                                    );
                                    response.headers_mut().insert(
                                        "X-New-API-Key",
                                        new_key_value.parse().unwrap(),
                                    );
                                }
                            }
                            Err(err) => {
                                warn!("Failed to rotate API key {}: {}", key_id, err);
                            }
                        }
                    }
                    Ok(false) => {
                        // Key doesn't need rotation
                    }
                    Err(err) => {
                        warn!("Failed to check if API key {} needs rotation: {}", key_id, err);
                    }
                }
            }
            
            result
        })
    }
}
