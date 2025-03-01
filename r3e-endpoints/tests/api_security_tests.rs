// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use r3e_endpoints::{
    app::create_app,
    config::Config,
    service::EndpointService,
};
use tower::ServiceExt;

#[tokio::test]
async fn test_authentication_required() {
    // Create a test service
    let config = Config::default();
    let service = Arc::new(EndpointService::new(config).await.unwrap());
    let app = create_app(service);

    // Test accessing a protected endpoint without authentication
    let response = app
        .oneshot(
            Request::builder()
                .uri("/services/list")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return 401 Unauthorized
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_rate_limiting() {
    // Create a test service with low rate limits for testing
    let mut config = Config::default();
    config.rate_limit_requests_per_minute = 5;
    
    let service = Arc::new(EndpointService::new(config).await.unwrap());
    let app = create_app(service);

    // Make multiple requests in quick succession
    let mut responses = Vec::new();
    for _ in 0..10 {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        
        responses.push(response.status());
    }

    // Some of the later requests should be rate limited (429 Too Many Requests)
    assert!(responses.contains(&StatusCode::TOO_MANY_REQUESTS));
}

#[tokio::test]
async fn test_input_validation() {
    // Create a test service
    let config = Config::default();
    let service = Arc::new(EndpointService::new(config).await.unwrap());
    let app = create_app(service);

    // Test with invalid JSON
    let response = app
        .oneshot(
            Request::builder()
                .uri("/services/deploy")
                .header("Content-Type", "application/json")
                .method("POST")
                .body(Body::from(r#"{"invalid_json":#"#))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return 400 Bad Request
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Test with missing required fields
    let response = app
        .oneshot(
            Request::builder()
                .uri("/services/deploy")
                .header("Content-Type", "application/json")
                .method("POST")
                .body(Body::from(r#"{"name":"test"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return 400 Bad Request or 422 Unprocessable Entity
    assert!(
        response.status() == StatusCode::BAD_REQUEST || 
        response.status() == StatusCode::UNPROCESSABLE_ENTITY
    );
}

#[tokio::test]
async fn test_cors_headers() {
    // Create a test service
    let config = Config::default();
    let service = Arc::new(EndpointService::new(config).await.unwrap());
    let app = create_app(service);

    // Test CORS preflight request
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .method("OPTIONS")
                .header("Origin", "https://example.com")
                .header("Access-Control-Request-Method", "GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return 200 OK with CORS headers
    assert_eq!(response.status(), StatusCode::OK);
    
    let headers = response.headers();
    assert!(headers.contains_key("access-control-allow-origin"));
    assert!(headers.contains_key("access-control-allow-methods"));
    assert!(headers.contains_key("access-control-allow-headers"));
}

#[tokio::test]
async fn test_content_security_policy() {
    // Create a test service
    let config = Config::default();
    let service = Arc::new(EndpointService::new(config).await.unwrap());
    let app = create_app(service);

    // Test that CSP headers are set
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return 200 OK with CSP headers
    assert_eq!(response.status(), StatusCode::OK);
    
    let headers = response.headers();
    // Check for Content-Security-Policy header
    // Note: This test might need adjustment based on actual CSP implementation
    assert!(
        headers.contains_key("content-security-policy") || 
        headers.contains_key("content-security-policy-report-only")
    );
}

#[tokio::test]
async fn test_xss_protection() {
    // Create a test service
    let config = Config::default();
    let service = Arc::new(EndpointService::new(config).await.unwrap());
    let app = create_app(service);

    // Test that XSS protection headers are set
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return 200 OK with XSS protection headers
    assert_eq!(response.status(), StatusCode::OK);
    
    let headers = response.headers();
    // Check for X-XSS-Protection header
    assert!(headers.contains_key("x-xss-protection"));
}
