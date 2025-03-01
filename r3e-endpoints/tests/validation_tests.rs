// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use axum::{
    body::Body,
    extract::Request,
    http::{header, Method, StatusCode},
    response::Response,
    routing::post,
    Router,
};
use serde_json::json;
use tower::ServiceExt;
use uuid::Uuid;

use r3e_endpoints::{
    middleware::ValidationLayer,
    types::{BlockchainType, ServiceInvocationRequest, WalletConnectionRequest},
};

#[tokio::test]
async fn test_validation_middleware_valid_request() {
    // Create a simple router with the validation middleware
    let app = Router::new()
        .route(
            "/services/invoke",
            post(|| async { "Service invoked successfully" }),
        )
        .layer(ValidationLayer::new());

    // Create a valid service invocation request
    let valid_request = ServiceInvocationRequest {
        service_id: Uuid::new_v4(),
        function: "test_function".to_string(),
        params: json!({"param1": "value1", "param2": 42}),
        signature: Some(
            "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
        ),
        timestamp: chrono::Utc::now().timestamp() as u64,
    };

    // Create a test request
    let request = Request::builder()
        .uri("/services/invoke")
        .method(Method::POST)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&valid_request).unwrap()))
        .unwrap();

    // Process the request
    let response = app.oneshot(request).await.unwrap();

    // Check that the response is successful
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_validation_middleware_invalid_request() {
    // Create a simple router with the validation middleware
    let app = Router::new()
        .route(
            "/services/invoke",
            post(|| async { "Service invoked successfully" }),
        )
        .layer(ValidationLayer::new());

    // Create an invalid service invocation request (invalid function name)
    let invalid_request = ServiceInvocationRequest {
        service_id: Uuid::new_v4(),
        function: "".to_string(), // Empty function name (invalid)
        params: json!({"param1": "value1", "param2": 42}),
        signature: Some(
            "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
        ),
        timestamp: chrono::Utc::now().timestamp() as u64,
    };

    // Create a test request
    let request = Request::builder()
        .uri("/services/invoke")
        .method(Method::POST)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&invalid_request).unwrap()))
        .unwrap();

    // Process the request
    let response = app.oneshot(request).await.unwrap();

    // Check that the response is a validation error
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_validation_middleware_invalid_timestamp() {
    // Create a simple router with the validation middleware
    let app = Router::new()
        .route(
            "/services/invoke",
            post(|| async { "Service invoked successfully" }),
        )
        .layer(ValidationLayer::new());

    // Create an invalid service invocation request (timestamp too far in the future)
    let invalid_request = ServiceInvocationRequest {
        service_id: Uuid::new_v4(),
        function: "test_function".to_string(),
        params: json!({"param1": "value1", "param2": 42}),
        signature: Some(
            "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
        ),
        timestamp: chrono::Utc::now().timestamp() as u64 + 3600, // 1 hour in the future (invalid)
    };

    // Create a test request
    let request = Request::builder()
        .uri("/services/invoke")
        .method(Method::POST)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&invalid_request).unwrap()))
        .unwrap();

    // Process the request
    let response = app.oneshot(request).await.unwrap();

    // Check that the response is a validation error
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_validation_middleware_wallet_connection() {
    // Create a simple router with the validation middleware
    let app = Router::new()
        .route(
            "/wallet/connect",
            post(|| async { "Wallet connected successfully" }),
        )
        .layer(ValidationLayer::new());

    // Create a valid wallet connection request
    let valid_request = WalletConnectionRequest {
        blockchain_type: BlockchainType::NeoN3,
        address: "NZV3gXYAaRBggLYvbZSuBhqzANcLcnKKUu".to_string(),
        public_key: Some(
            "03b209fd4f53a7170ea4444e0cb0a6bb6a53c2bd016926989cf85f9b0fba17a70c".to_string(),
        ),
        signature_curve: r3e_endpoints::types::SignatureCurve::Secp256r1,
        signature: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
        message: "Connect wallet to R3E FaaS".to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
    };

    // Create a test request
    let request = Request::builder()
        .uri("/wallet/connect")
        .method(Method::POST)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&valid_request).unwrap()))
        .unwrap();

    // Process the request
    let response = app.oneshot(request).await.unwrap();

    // Check that the response is successful
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_validation_middleware_invalid_json() {
    // Create a simple router with the validation middleware
    let app = Router::new()
        .route(
            "/services/invoke",
            post(|| async { "Service invoked successfully" }),
        )
        .layer(ValidationLayer::new());

    // Create an invalid JSON request
    let invalid_json = r#"{"service_id": "not-a-uuid", "function": "test_function", "params": {}, "timestamp": 123456789}"#;

    // Create a test request
    let request = Request::builder()
        .uri("/services/invoke")
        .method(Method::POST)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(invalid_json))
        .unwrap();

    // Process the request
    let response = app.oneshot(request).await.unwrap();

    // Check that the response is a validation error
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
