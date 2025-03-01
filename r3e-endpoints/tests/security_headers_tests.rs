// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
    routing::get,
    Router,
};
use tower::ServiceExt;

use r3e_endpoints::middleware::SecurityHeadersLayer;

#[tokio::test]
async fn test_security_headers_middleware() {
    // Create a simple router with the security headers middleware
    let app = Router::new()
        .route("/test", get(|| async { "Hello, World!" }))
        .layer(SecurityHeadersLayer::new());

    // Create a test request
    let request = Request::builder()
        .uri("/test")
        .body(Body::empty())
        .unwrap();

    // Process the request
    let response = app.oneshot(request).await.unwrap();

    // Check that the response has the expected security headers
    assert_eq!(response.status(), StatusCode::OK);
    
    // Get the headers
    let headers = response.headers();
    
    // Check for Strict-Transport-Security header
    assert_eq!(
        headers.get("Strict-Transport-Security").unwrap(),
        "max-age=31536000; includeSubDomains; preload"
    );
    
    // Check for X-Content-Type-Options header
    assert_eq!(
        headers.get("X-Content-Type-Options").unwrap(),
        "nosniff"
    );
    
    // Check for X-Frame-Options header
    assert_eq!(
        headers.get("X-Frame-Options").unwrap(),
        "DENY"
    );
    
    // Check for Content-Security-Policy header
    assert_eq!(
        headers.get("Content-Security-Policy").unwrap(),
        "default-src 'self'; script-src 'self'; object-src 'none'; frame-ancestors 'none';"
    );
    
    // Check for X-XSS-Protection header
    assert_eq!(
        headers.get("X-XSS-Protection").unwrap(),
        "1; mode=block"
    );
    
    // Check for Referrer-Policy header
    assert_eq!(
        headers.get("Referrer-Policy").unwrap(),
        "strict-origin-when-cross-origin"
    );
    
    // Check for Permissions-Policy header
    assert_eq!(
        headers.get("Permissions-Policy").unwrap(),
        "geolocation=(), microphone=(), camera=()"
    );
    
    // Check for Cache-Control header
    assert_eq!(
        headers.get("Cache-Control").unwrap(),
        "no-store, max-age=0"
    );
}

#[tokio::test]
async fn test_clear_site_data_on_logout() {
    // Create a simple router with the security headers middleware
    let app = Router::new()
        .route("/logout", get(|| async { "Logged out" }))
        .layer(SecurityHeadersLayer::new());

    // Create a test request to the logout endpoint
    let request = Request::builder()
        .uri("/logout")
        .body(Body::empty())
        .unwrap();

    // Process the request
    let response = app.oneshot(request).await.unwrap();

    // Check that the response has the Clear-Site-Data header
    assert_eq!(response.status(), StatusCode::OK);
    
    // Get the headers
    let headers = response.headers();
    
    // Check for Clear-Site-Data header
    assert_eq!(
        headers.get("Clear-Site-Data").unwrap(),
        "\"cache\", \"cookies\", \"storage\""
    );
}
