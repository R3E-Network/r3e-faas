// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::task::{Context, Poll};
use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use futures::future::BoxFuture;
use serde_json::json;
use tower::{Layer, Service};
use tracing::{debug, error, info, warn};
use validator::Validate;

/// Validation layer for request validation
#[derive(Clone)]
pub struct ValidationLayer;

impl ValidationLayer {
    /// Create a new validation layer
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for ValidationLayer {
    type Service = ValidationService<S>;

    fn layer(&self, service: S) -> Self::Service {
        ValidationService { inner: service }
    }
}

/// Validation service for request validation
#[derive(Clone)]
pub struct ValidationService<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for ValidationService<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let mut inner = self.inner.clone();
        
        Box::pin(async move {
            // Extract the request body
            let (parts, body) = request.into_parts();
            
            // Check if the request is a JSON request
            let content_type = parts.headers.get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");
                
            if content_type.contains("application/json") {
                // Read the body
                let bytes = hyper::body::to_bytes(body).await.unwrap_or_default();
                
                // Try to parse the body as JSON
                if let Ok(json_str) = std::str::from_utf8(&bytes) {
                    // Validate based on the request path
                    let path = parts.uri.path();
                    
                    if path.contains("/services") && path.contains("/invoke") {
                        // Validate service invocation request
                        if let Ok(request) = serde_json::from_str::<crate::types::ServiceInvocationRequest>(json_str) {
                            if let Err(errors) = request.validate() {
                                debug!("Validation failed for service invocation request: {:?}", errors);
                                let response = validation_error_response(errors);
                                return Ok(response);
                            }
                        }
                    } else if path.contains("/wallet/connect") {
                        // Validate wallet connection request
                        if let Ok(request) = serde_json::from_str::<crate::types::WalletConnectionRequest>(json_str) {
                            if let Err(errors) = request.validate() {
                                debug!("Validation failed for wallet connection request: {:?}", errors);
                                let response = validation_error_response(errors);
                                return Ok(response);
                            }
                        }
                    } else if path.contains("/wallet/sign") {
                        // Validate message signing request
                        if let Ok(request) = serde_json::from_str::<crate::types::MessageSigningRequest>(json_str) {
                            if let Err(errors) = request.validate() {
                                debug!("Validation failed for message signing request: {:?}", errors);
                                let response = validation_error_response(errors);
                                return Ok(response);
                            }
                        }
                    } else if path.contains("/meta-tx") {
                        // Validate meta transaction request
                        if let Ok(request) = serde_json::from_str::<crate::types::MetaTransactionRequest>(json_str) {
                            if let Err(errors) = request.validate() {
                                debug!("Validation failed for meta transaction request: {:?}", errors);
                                let response = validation_error_response(errors);
                                return Ok(response);
                            }
                        }
                    }
                    
                    // Recreate the body
                    let body = Body::from(bytes);
                    
                    // Recreate the request
                    let request = Request::from_parts(parts, body);
                    
                    // Pass the request to the inner service
                    inner.call(request).await
                } else {
                    // Invalid JSON
                    let response = json_error_response("Invalid JSON format");
                    Ok(response)
                }
            } else {
                // Not a JSON request, pass it through
                let request = Request::from_parts(parts, body);
                inner.call(request).await
            }
        })
    }
}

/// Create a validation error response
fn validation_error_response(errors: validator::ValidationErrors) -> Response {
    let error_map = errors
        .field_errors()
        .iter()
        .map(|(field, errors)| {
            let error_messages: Vec<String> = errors
                .iter()
                .map(|error| error.message.clone().unwrap_or_else(|| error.code.clone()))
                .collect();
            (*field, error_messages)
        })
        .collect::<serde_json::Map<_, _>>();

    (
        StatusCode::BAD_REQUEST,
        axum::Json(json!({
            "error": "Validation Error",
            "details": error_map
        })),
    )
        .into_response()
}

/// Create a JSON error response
fn json_error_response(message: &str) -> Response {
    (
        StatusCode::BAD_REQUEST,
        axum::Json(json!({
            "error": "Invalid Request",
            "message": message
        })),
    )
        .into_response()
}
