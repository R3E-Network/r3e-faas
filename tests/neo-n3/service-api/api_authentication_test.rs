// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App, HttpResponse, http::StatusCode};
    use r3e_api::routes::{health, auth, functions};
    use r3e_api::models::user::User;
    use r3e_api::auth::{JwtClaims, generate_token, verify_token};
    use r3e_api::config::Config;
    use serde_json::{json, Value};
    use std::sync::Arc;
    use mockall::predicate::*;
    use mockall::mock;
    use chrono::{Duration, Utc};
    
    // Mock the database service
    mock! {
        DbService {}
        trait DbService {
            async fn get_user_by_id(&self, id: &str) -> Option<User>;
            async fn get_user_by_username(&self, username: &str) -> Option<User>;
            async fn create_user(&self, user: User) -> Result<User, String>;
        }
    }
    
    // Helper function to create a test app with mocked database service
    fn create_test_app(db_service: MockDbService) -> impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    > {
        let db_service = web::Data::new(db_service);
        let config = web::Data::new(Config {
            jwt_secret: "test_secret".to_string(),
            jwt_expiration: 3600,
            api_port: 8080,
            database_url: "test_db_url".to_string(),
        });
        
        test::init_service(
            App::new()
                .app_data(db_service.clone())
                .app_data(config.clone())
                .service(
                    web::scope("/api")
                        .service(auth::login)
                        .service(auth::register)
                        .service(
                            web::scope("/functions")
                                .service(functions::list_functions)
                        )
                )
        )
    }
    
    #[actix_web::test]
    async fn test_token_generation_and_verification() {
        // Create a user ID for the token
        let user_id = "user123";
        
        // Create a JWT secret
        let jwt_secret = "test_secret";
        
        // Create JWT claims
        let claims = JwtClaims {
            sub: user_id.to_string(),
            exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
        };
        
        // Generate a token
        let token = generate_token(&claims, jwt_secret).unwrap();
        
        // Verify that the token is not empty
        assert!(!token.is_empty());
        
        // Verify the token
        let verified_claims = verify_token(&token, jwt_secret).unwrap();
        
        // Verify that the claims match
        assert_eq!(verified_claims.sub, user_id);
    }
    
    #[actix_web::test]
    async fn test_expired_token_verification() {
        // Create a user ID for the token
        let user_id = "user123";
        
        // Create a JWT secret
        let jwt_secret = "test_secret";
        
        // Create JWT claims with an expired token (1 hour in the past)
        let claims = JwtClaims {
            sub: user_id.to_string(),
            exp: (Utc::now() - Duration::hours(1)).timestamp() as usize,
        };
        
        // Generate a token
        let token = generate_token(&claims, jwt_secret).unwrap();
        
        // Verify that the token is not empty
        assert!(!token.is_empty());
        
        // Verify the token (should fail because it's expired)
        let result = verify_token(&token, jwt_secret);
        
        // Verify that the verification failed
        assert!(result.is_err());
        
        // Verify that the error message is about expiration
        let error = result.unwrap_err();
        assert!(error.to_string().contains("expired"));
    }
    
    #[actix_web::test]
    async fn test_invalid_token_verification() {
        // Create a JWT secret
        let jwt_secret = "test_secret";
        
        // Create an invalid token
        let token = "invalid.token.here";
        
        // Verify the token (should fail because it's invalid)
        let result = verify_token(token, jwt_secret);
        
        // Verify that the verification failed
        assert!(result.is_err());
    }
    
    #[actix_web::test]
    async fn test_wrong_secret_verification() {
        // Create a user ID for the token
        let user_id = "user123";
        
        // Create a JWT secret
        let jwt_secret = "test_secret";
        
        // Create JWT claims
        let claims = JwtClaims {
            sub: user_id.to_string(),
            exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
        };
        
        // Generate a token
        let token = generate_token(&claims, jwt_secret).unwrap();
        
        // Verify that the token is not empty
        assert!(!token.is_empty());
        
        // Verify the token with a wrong secret
        let wrong_secret = "wrong_secret";
        let result = verify_token(&token, wrong_secret);
        
        // Verify that the verification failed
        assert!(result.is_err());
    }
    
    #[actix_web::test]
    async fn test_login_endpoint_returns_token() {
        // Create a mock database service
        let mut db_service = MockDbService::new();
        
        // Set up the mock behavior for get_user_by_username
        db_service
            .expect_get_user_by_username()
            .with(eq("testuser"))
            .returning(|_| {
                Some(User {
                    id: "user123".to_string(),
                    username: "testuser".to_string(),
                    password_hash: "$2b$12$szAJOBBqLPIwRQo1WVCH8OXwBJP.rvmJCPcALCO9wJCdSvPKfgmOy".to_string(), // Hash for "password123"
                    created_at: Utc::now(),
                })
            });
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Send a POST request to the login endpoint
        let req = test::TestRequest::post()
            .uri("/api/login")
            .set_json(json!({
                "username": "testuser",
                "password": "password123"
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is OK (200)
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Verify the response body contains a token
        let body: Value = test::read_body_json(resp).await;
        assert!(body["token"].is_string());
        
        // Get the token from the response
        let token = body["token"].as_str().unwrap();
        
        // Verify the token
        let claims = verify_token(token, "test_secret").unwrap();
        
        // Verify that the claims match
        assert_eq!(claims.sub, "user123");
    }
    
    #[actix_web::test]
    async fn test_protected_endpoint_with_valid_token() {
        // Create a mock database service
        let mut db_service = MockDbService::new();
        
        // Set up the mock behavior for get_user_by_id
        db_service
            .expect_get_user_by_id()
            .with(eq("user123"))
            .returning(|_| {
                Some(User {
                    id: "user123".to_string(),
                    username: "testuser".to_string(),
                    password_hash: "hash".to_string(),
                    created_at: Utc::now(),
                })
            });
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate a valid JWT token
        let claims = JwtClaims {
            sub: "user123".to_string(),
            exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
        };
        
        let token = generate_token(&claims, "test_secret").unwrap();
        
        // Send a GET request to a protected endpoint with the token
        let req = test::TestRequest::get()
            .uri("/api/functions")
            .header("Authorization", format!("Bearer {}", token))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is OK (200)
        assert_eq!(resp.status(), StatusCode::OK);
    }
    
    #[actix_web::test]
    async fn test_protected_endpoint_with_invalid_token() {
        // Create a mock database service
        let db_service = MockDbService::new();
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Send a GET request to a protected endpoint with an invalid token
        let req = test::TestRequest::get()
            .uri("/api/functions")
            .header("Authorization", "Bearer invalid.token.here")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is Unauthorized (401)
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
    
    #[actix_web::test]
    async fn test_protected_endpoint_with_expired_token() {
        // Create a mock database service
        let db_service = MockDbService::new();
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate an expired JWT token
        let claims = JwtClaims {
            sub: "user123".to_string(),
            exp: (Utc::now() - Duration::hours(1)).timestamp() as usize,
        };
        
        let token = generate_token(&claims, "test_secret").unwrap();
        
        // Send a GET request to a protected endpoint with the expired token
        let req = test::TestRequest::get()
            .uri("/api/functions")
            .header("Authorization", format!("Bearer {}", token))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is Unauthorized (401)
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
    
    #[actix_web::test]
    async fn test_protected_endpoint_without_token() {
        // Create a mock database service
        let db_service = MockDbService::new();
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Send a GET request to a protected endpoint without a token
        let req = test::TestRequest::get()
            .uri("/api/functions")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is Unauthorized (401)
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
    
    #[actix_web::test]
    async fn test_token_with_wrong_user_id() {
        // Create a mock database service
        let mut db_service = MockDbService::new();
        
        // Set up the mock behavior for get_user_by_id (user doesn't exist)
        db_service
            .expect_get_user_by_id()
            .with(eq("nonexistent_user"))
            .returning(|_| None);
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate a JWT token with a non-existent user ID
        let claims = JwtClaims {
            sub: "nonexistent_user".to_string(),
            exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
        };
        
        let token = generate_token(&claims, "test_secret").unwrap();
        
        // Send a GET request to a protected endpoint with the token
        let req = test::TestRequest::get()
            .uri("/api/functions")
            .header("Authorization", format!("Bearer {}", token))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is Unauthorized (401)
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}
