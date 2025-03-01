// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::task::{Context, Poll};
use axum::http::{Request, Response, HeaderValue};
use futures::future::BoxFuture;
use tower::{Layer, Service};
use tracing::{debug, info};

/// Security headers layer
#[derive(Clone)]
pub struct SecurityHeadersLayer;

impl SecurityHeadersLayer {
    /// Create a new security headers layer
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for SecurityHeadersLayer {
    type Service = SecurityHeadersService<S>;

    fn layer(&self, service: S) -> Self::Service {
        SecurityHeadersService {
            inner: service,
        }
    }
}

/// Security headers service
#[derive(Clone)]
pub struct SecurityHeadersService<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for SecurityHeadersService<S>
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
        let mut inner = self.inner.clone();
        
        Box::pin(async move {
            let mut response = inner.call(request).await?;
            
            // Add security headers
            let headers = response.headers_mut();
            
            // Strict-Transport-Security (HSTS)
            headers.insert(
                "Strict-Transport-Security",
                HeaderValue::from_static("max-age=31536000; includeSubDomains; preload"),
            );
            
            // X-Content-Type-Options
            headers.insert(
                "X-Content-Type-Options",
                HeaderValue::from_static("nosniff"),
            );
            
            // X-Frame-Options
            headers.insert(
                "X-Frame-Options",
                HeaderValue::from_static("DENY"),
            );
            
            // Content-Security-Policy
            headers.insert(
                "Content-Security-Policy",
                HeaderValue::from_static("default-src 'self'; script-src 'self'; object-src 'none'; frame-ancestors 'none';"),
            );
            
            // X-XSS-Protection
            headers.insert(
                "X-XSS-Protection",
                HeaderValue::from_static("1; mode=block"),
            );
            
            // Referrer-Policy
            headers.insert(
                "Referrer-Policy",
                HeaderValue::from_static("strict-origin-when-cross-origin"),
            );
            
            // Permissions-Policy
            headers.insert(
                "Permissions-Policy",
                HeaderValue::from_static("geolocation=(), microphone=(), camera=()"),
            );
            
            // Cache-Control
            headers.insert(
                "Cache-Control",
                HeaderValue::from_static("no-store, max-age=0"),
            );
            
            // Clear-Site-Data (on logout endpoints)
            if request.uri().path().ends_with("/logout") {
                headers.insert(
                    "Clear-Site-Data",
                    HeaderValue::from_static("\"cache\", \"cookies\", \"storage\""),
                );
            }
            
            debug!("Added security headers to response");
            
            Ok(response)
        })
    }
}
