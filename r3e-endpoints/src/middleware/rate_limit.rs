// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use std::time::Duration;
use std::task::{Context, Poll};
use std::net::SocketAddr;

use axum::extract::ConnectInfo;
use axum::http::{Request, Response, StatusCode};
use axum::response::IntoResponse;
use futures::future::BoxFuture;
use governor::{Quota, RateLimiter, clock::{DefaultClock, Clock}};
use governor::middleware::NoOpMiddleware;
use governor::state::{NotKeyed, InMemoryState};
use tower::{Layer, Service};

use std::num::NonZeroU32;
use std::collections::HashMap;
use std::sync::RwLock;

/// Rate limiter type
pub type GlobalRateLimiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>;
pub type KeyedRateLimiter = RateLimiter<String, InMemoryState, DefaultClock, NoOpMiddleware>;

/// Rate limiter storage
#[derive(Clone)]
pub struct RateLimiterStore {
    /// Global rate limiter
    global: Arc<GlobalRateLimiter>,
    /// Per-IP rate limiters
    ip_limiters: Arc<RwLock<HashMap<String, Arc<KeyedRateLimiter>>>>,
    /// Per-user rate limiters
    user_limiters: Arc<RwLock<HashMap<String, Arc<KeyedRateLimiter>>>>,
    /// Requests per minute
    requests_per_minute: u32,
}

impl RateLimiterStore {
    /// Create a new rate limiter store
    pub fn new(requests_per_minute: u32) -> Self {
        let rpm = NonZeroU32::new(requests_per_minute).unwrap_or(NonZeroU32::new(60).unwrap());
        
        Self {
            global: Arc::new(RateLimiter::direct(Quota::per_minute(rpm))),
            ip_limiters: Arc::new(RwLock::new(HashMap::new())),
            user_limiters: Arc::new(RwLock::new(HashMap::new())),
            requests_per_minute,
        }
    }

    /// Get or create an IP rate limiter
    pub fn get_ip_limiter(&self, ip: &str) -> Arc<KeyedRateLimiter> {
        let read_guard = self.ip_limiters.read().unwrap();
        
        if let Some(limiter) = read_guard.get(ip) {
            return limiter.clone();
        }
        
        drop(read_guard);
        
        let rpm = NonZeroU32::new(self.requests_per_minute).unwrap_or(NonZeroU32::new(60).unwrap());
        let limiter = Arc::new(RateLimiter::keyed(Quota::per_minute(rpm)));
        
        let mut write_guard = self.ip_limiters.write().unwrap();
        write_guard.insert(ip.to_string(), limiter.clone());
        
        limiter
    }

    /// Get or create a user rate limiter
    pub fn get_user_limiter(&self, user_id: &str) -> Arc<KeyedRateLimiter> {
        let read_guard = self.user_limiters.read().unwrap();
        
        if let Some(limiter) = read_guard.get(user_id) {
            return limiter.clone();
        }
        
        drop(read_guard);
        
        let rpm = NonZeroU32::new(self.requests_per_minute).unwrap_or(NonZeroU32::new(60).unwrap());
        let limiter = Arc::new(RateLimiter::keyed(Quota::per_minute(rpm)));
        
        let mut write_guard = self.user_limiters.write().unwrap();
        write_guard.insert(user_id.to_string(), limiter.clone());
        
        limiter
    }
}

/// Rate limit layer
#[derive(Clone)]
pub struct RateLimitLayer {
    store: RateLimiterStore,
}

impl RateLimitLayer {
    /// Create a new rate limit layer
    pub fn new(requests_per_minute: u32) -> Self {
        Self {
            store: RateLimiterStore::new(requests_per_minute),
        }
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitService<S>;

    fn layer(&self, service: S) -> Self::Service {
        RateLimitService {
            inner: service,
            store: self.store.clone(),
        }
    }
}

/// Rate limit service
#[derive(Clone)]
pub struct RateLimitService<S> {
    inner: S,
    store: RateLimiterStore,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for RateLimitService<S>
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
        // Clone the service and store
        let mut inner = self.inner.clone();
        let store = self.store.clone();

        // Check global rate limit
        if let Err(_) = store.global.check() {
            return Box::pin(async move {
                let response = (
                    StatusCode::TOO_MANY_REQUESTS,
                    "Rate limit exceeded. Please try again later.",
                )
                    .into_response()
                    .map(|body| body.into_body());

                Ok(response)
            });
        }

        // Get client IP
        let client_ip = request
            .extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .map(|connect_info| connect_info.0.ip().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        // Check IP-based rate limit
        let ip_limiter = store.get_ip_limiter(&client_ip);
        if let Err(_) = ip_limiter.check_key(&client_ip) {
            return Box::pin(async move {
                let response = (
                    StatusCode::TOO_MANY_REQUESTS,
                    "IP-based rate limit exceeded. Please try again later.",
                )
                    .into_response()
                    .map(|body| body.into_body());

                Ok(response)
            });
        }

        // Call the inner service
        Box::pin(async move {
            let response = inner.call(request).await?;
            Ok(response)
        })
    }
}
