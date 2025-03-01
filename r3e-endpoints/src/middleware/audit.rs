// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::time::{Instant, SystemTime, UNIX_EPOCH};
use std::task::{Context, Poll};
use std::net::SocketAddr;

use axum::extract::ConnectInfo;
use axum::http::{Request, Response, Method};
use futures::future::BoxFuture;
use tower::{Layer, Service};
use tracing::{info, warn, error};

/// Audit log layer
#[derive(Clone)]
pub struct AuditLogLayer;

impl AuditLogLayer {
    /// Create a new audit log layer
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for AuditLogLayer {
    type Service = AuditLogService<S>;

    fn layer(&self, service: S) -> Self::Service {
        AuditLogService {
            inner: service,
        }
    }
}

/// Audit log service
#[derive(Clone)]
pub struct AuditLogService<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for AuditLogService<S>
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

        // Get request details
        let method = request.method().clone();
        let uri = request.uri().clone();
        let path = uri.path().to_string();
        
        // Get client IP
        let client_ip = request
            .extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .map(|connect_info| connect_info.0.ip().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        
        // Get timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Start timer
        let start = Instant::now();
        
        // Log security-sensitive operations
        let is_sensitive = is_security_sensitive(&method, &path);
        if is_sensitive {
            info!(
                target: "audit",
                "AUDIT: Request initiated - method={}, path={}, ip={}, timestamp={}",
                method, path, client_ip, timestamp
            );
        }
        
        // Call the inner service
        Box::pin(async move {
            let result = inner.call(request).await;
            
            // Calculate request duration
            let duration = start.elapsed().as_millis();
            
            match &result {
                Ok(response) => {
                    let status = response.status();
                    
                    if is_sensitive {
                        if status.is_success() {
                            info!(
                                target: "audit",
                                "AUDIT: Request completed - method={}, path={}, ip={}, status={}, duration={}ms, timestamp={}",
                                method, path, client_ip, status.as_u16(), duration, timestamp
                            );
                        } else {
                            warn!(
                                target: "audit",
                                "AUDIT: Request failed - method={}, path={}, ip={}, status={}, duration={}ms, timestamp={}",
                                method, path, client_ip, status.as_u16(), duration, timestamp
                            );
                        }
                    }
                }
                Err(_) => {
                    if is_sensitive {
                        error!(
                            target: "audit",
                            "AUDIT: Request error - method={}, path={}, ip={}, duration={}ms, timestamp={}",
                            method, path, client_ip, duration, timestamp
                        );
                    }
                }
            }
            
            result
        })
    }
}

/// Check if a request is security-sensitive
fn is_security_sensitive(method: &Method, path: &str) -> bool {
    // Authentication and authorization
    if path.starts_with("/auth/") || path.starts_with("/wallet/") {
        return true;
    }
    
    // Service invocation
    if path.contains("/invoke") && method == Method::POST {
        return true;
    }
    
    // Meta transactions
    if path.starts_with("/meta-tx/") && method == Method::POST {
        return true;
    }
    
    false
}
