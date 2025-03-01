#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::EndpointService;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::post,
        Router,
    };
    use serde_json::json;
    use tower::ServiceExt;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_connect_wallet() {
        // Mock database client
        let db_client = Arc::new(crate::db::MockDatabaseClient::new());
        let service = Arc::new(EndpointService::new_with_db(db_client.clone()));

        // Build router
        let app = Router::new()
            .route("/connect", post(connect_wallet))
            .with_state(service);

        // Create request
        let request = Request::builder()
            .method("POST")
            .uri("/connect")
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::to_string(&json!({
                    "blockchain_type": "ethereum",
                    "address": "0x1234567890abcdef1234567890abcdef12345678"
                }))
                .unwrap(),
            ))
            .unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Check response
        assert_eq!(response.status(), StatusCode::OK);

        // Parse response
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let response: ConnectWalletResponse = serde_json::from_slice(&body).unwrap();

        // Check challenge
        assert!(!response.challenge.is_empty());
        assert!(!response.challenge_id.is_empty());
        assert!(response.expires_at > 0);
    }

    #[tokio::test]
    async fn test_wallet_authentication_flow() {
        // Mock database client that can store and retrieve challenges
        let mut db_client = crate::db::MockDatabaseClient::new();
        
        // Setup challenge storage
        let challenge_id = Uuid::new_v4().to_string();
        let wallet_address = "0x1234567890abcdef1234567890abcdef12345678".to_string();
        let blockchain_type = "ethereum".to_string();
        let message = "Sign this message for testing".to_string();
        let expires_at = (chrono::Utc::now().timestamp() as u64) + 300;
        
        // Mock challenge
        let challenge = crate::db::models::AuthChallenge {
            id: challenge_id.clone(),
            address: wallet_address.clone(),
            blockchain_type: blockchain_type.clone(),
            message: message.clone(),
            expires_at,
            created_at: chrono::Utc::now().timestamp() as u64,
        };
        
        // Setup mock methods
        db_client.expect_get_auth_challenge()
            .returning(move |id| {
                if id == challenge_id {
                    Ok(Some(challenge.clone()))
                } else {
                    Ok(None)
                }
            });
            
        db_client.expect_verify_signature()
            .returning(|_, _, _, _, _| Ok(true));
            
        db_client.expect_find_user_by_wallet_address()
            .returning(|_, _| Ok(None));
            
        db_client.expect_create_wallet_user()
            .returning(|_, _, _| Ok(()));
            
        db_client.expect_create_session()
            .returning(|_, _, _| Ok(()));
            
        db_client.expect_delete_auth_challenge()
            .returning(|_| Ok(()));
        
        let service = Arc::new(EndpointService::new_with_db(Arc::new(db_client)));

        // Build router
        let app = Router::new()
            .route("/authenticate", post(authenticate_wallet))
            .with_state(service);

        // Create request
        let request = Request::builder()
            .method("POST")
            .uri("/authenticate")
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::to_string(&json!({
                    "challenge_id": challenge_id,
                    "blockchain_type": blockchain_type,
                    "address": wallet_address,
                    "signature": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1b"
                }))
                .unwrap(),
            ))
            .unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Check response
        assert_eq!(response.status(), StatusCode::OK);

        // Parse response
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let response: AuthenticateWalletResponse = serde_json::from_slice(&body).unwrap();

        // Check authentication response
        assert!(!response.user_id.is_empty());
        assert_eq!(response.address, wallet_address);
        assert_eq!(response.blockchain_type, blockchain_type);
        assert!(!response.token.is_empty());
        assert!(response.expires_at > 0);
    }
} 