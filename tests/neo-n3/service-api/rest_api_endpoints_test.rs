// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App, HttpResponse, http::StatusCode};
    use r3e_api::routes::{health, auth, functions, services};
    use r3e_api::models::function::{Function, FunctionMetadata};
    use r3e_api::models::service::{Service, ServiceMetadata};
    use r3e_api::models::user::User;
    use r3e_api::auth::{JwtClaims, generate_token};
    use r3e_api::config::Config;
    use serde_json::{json, Value};
    use std::sync::Arc;
    use mockall::predicate::*;
    use mockall::mock;
    
    // Mock the database service
    mock! {
        DbService {}
        trait DbService {
            async fn get_user_by_id(&self, id: &str) -> Option<User>;
            async fn get_user_by_username(&self, username: &str) -> Option<User>;
            async fn create_user(&self, user: User) -> Result<User, String>;
            async fn get_function(&self, id: &str) -> Option<Function>;
            async fn list_functions(&self, user_id: &str) -> Vec<Function>;
            async fn create_function(&self, function: Function) -> Result<Function, String>;
            async fn update_function(&self, function: Function) -> Result<Function, String>;
            async fn delete_function(&self, id: &str) -> Result<(), String>;
            async fn get_service(&self, id: &str) -> Option<Service>;
            async fn list_services(&self, user_id: &str) -> Vec<Service>;
            async fn create_service(&self, service: Service) -> Result<Service, String>;
            async fn update_service(&self, service: Service) -> Result<Service, String>;
            async fn delete_service(&self, id: &str) -> Result<(), String>;
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
                .service(health::health_check)
                .service(
                    web::scope("/api")
                        .service(auth::login)
                        .service(auth::register)
                        .service(
                            web::scope("/functions")
                                .service(functions::get_function)
                                .service(functions::list_functions)
                                .service(functions::create_function)
                                .service(functions::update_function)
                                .service(functions::delete_function)
                        )
                        .service(
                            web::scope("/services")
                                .service(services::get_service)
                                .service(services::list_services)
                                .service(services::create_service)
                                .service(services::update_service)
                                .service(services::delete_service)
                        )
                )
        )
    }
    
    // Helper function to generate a valid JWT token for testing
    fn generate_test_token(user_id: &str) -> String {
        let claims = JwtClaims {
            sub: user_id.to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
        };
        
        generate_token(&claims, "test_secret").unwrap()
    }
    
    #[actix_web::test]
    async fn test_health_check_endpoint() {
        // Create a mock database service
        let db_service = MockDbService::new();
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Send a GET request to the health check endpoint
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is OK (200)
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Verify the response body
        let body: Value = test::read_body_json(resp).await;
        assert_eq!(body["status"], "ok");
    }
    
    #[actix_web::test]
    async fn test_login_endpoint_success() {
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
                    created_at: chrono::Utc::now(),
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
    }
    
    #[actix_web::test]
    async fn test_login_endpoint_invalid_credentials() {
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
                    created_at: chrono::Utc::now(),
                })
            });
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Send a POST request to the login endpoint with invalid password
        let req = test::TestRequest::post()
            .uri("/api/login")
            .set_json(json!({
                "username": "testuser",
                "password": "wrongpassword"
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is Unauthorized (401)
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
    
    #[actix_web::test]
    async fn test_register_endpoint_success() {
        // Create a mock database service
        let mut db_service = MockDbService::new();
        
        // Set up the mock behavior for get_user_by_username (user doesn't exist)
        db_service
            .expect_get_user_by_username()
            .with(eq("newuser"))
            .returning(|_| None);
        
        // Set up the mock behavior for create_user
        db_service
            .expect_create_user()
            .returning(|user| {
                Ok(User {
                    id: "new_user_id".to_string(),
                    username: user.username,
                    password_hash: user.password_hash,
                    created_at: chrono::Utc::now(),
                })
            });
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Send a POST request to the register endpoint
        let req = test::TestRequest::post()
            .uri("/api/register")
            .set_json(json!({
                "username": "newuser",
                "password": "newpassword123"
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is Created (201)
        assert_eq!(resp.status(), StatusCode::CREATED);
        
        // Verify the response body
        let body: Value = test::read_body_json(resp).await;
        assert_eq!(body["id"], "new_user_id");
        assert_eq!(body["username"], "newuser");
    }
    
    #[actix_web::test]
    async fn test_list_functions_endpoint() {
        // Create a mock database service
        let mut db_service = MockDbService::new();
        
        // Set up the mock behavior for list_functions
        db_service
            .expect_list_functions()
            .with(eq("user123"))
            .returning(|_| {
                vec![
                    Function {
                        id: "func1".to_string(),
                        user_id: "user123".to_string(),
                        name: "Test Function 1".to_string(),
                        code: "function handler() { return 'Hello, World!'; }".to_string(),
                        metadata: FunctionMetadata {
                            runtime: "javascript".to_string(),
                            trigger_type: "neo_block".to_string(),
                            trigger_config: json!({ "network": "testnet" }),
                        },
                        created_at: chrono::Utc::now(),
                        updated_at: chrono::Utc::now(),
                    },
                    Function {
                        id: "func2".to_string(),
                        user_id: "user123".to_string(),
                        name: "Test Function 2".to_string(),
                        code: "function handler() { return 'Hello, Neo!'; }".to_string(),
                        metadata: FunctionMetadata {
                            runtime: "javascript".to_string(),
                            trigger_type: "neo_transaction".to_string(),
                            trigger_config: json!({ "network": "testnet", "contract": "0x1234" }),
                        },
                        created_at: chrono::Utc::now(),
                        updated_at: chrono::Utc::now(),
                    },
                ]
            });
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate a valid JWT token
        let token = generate_test_token("user123");
        
        // Send a GET request to the list functions endpoint
        let req = test::TestRequest::get()
            .uri("/api/functions")
            .header("Authorization", format!("Bearer {}", token))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is OK (200)
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Verify the response body
        let body: Value = test::read_body_json(resp).await;
        assert!(body.is_array());
        assert_eq!(body.as_array().unwrap().len(), 2);
        assert_eq!(body[0]["id"], "func1");
        assert_eq!(body[1]["id"], "func2");
    }
    
    #[actix_web::test]
    async fn test_get_function_endpoint() {
        // Create a mock database service
        let mut db_service = MockDbService::new();
        
        // Set up the mock behavior for get_function
        db_service
            .expect_get_function()
            .with(eq("func1"))
            .returning(|_| {
                Some(Function {
                    id: "func1".to_string(),
                    user_id: "user123".to_string(),
                    name: "Test Function 1".to_string(),
                    code: "function handler() { return 'Hello, World!'; }".to_string(),
                    metadata: FunctionMetadata {
                        runtime: "javascript".to_string(),
                        trigger_type: "neo_block".to_string(),
                        trigger_config: json!({ "network": "testnet" }),
                    },
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                })
            });
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate a valid JWT token
        let token = generate_test_token("user123");
        
        // Send a GET request to the get function endpoint
        let req = test::TestRequest::get()
            .uri("/api/functions/func1")
            .header("Authorization", format!("Bearer {}", token))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is OK (200)
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Verify the response body
        let body: Value = test::read_body_json(resp).await;
        assert_eq!(body["id"], "func1");
        assert_eq!(body["name"], "Test Function 1");
        assert_eq!(body["user_id"], "user123");
    }
    
    #[actix_web::test]
    async fn test_create_function_endpoint() {
        // Create a mock database service
        let mut db_service = MockDbService::new();
        
        // Set up the mock behavior for create_function
        db_service
            .expect_create_function()
            .returning(|function| {
                Ok(Function {
                    id: "new_func_id".to_string(),
                    user_id: function.user_id,
                    name: function.name,
                    code: function.code,
                    metadata: function.metadata,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                })
            });
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate a valid JWT token
        let token = generate_test_token("user123");
        
        // Send a POST request to the create function endpoint
        let req = test::TestRequest::post()
            .uri("/api/functions")
            .header("Authorization", format!("Bearer {}", token))
            .set_json(json!({
                "name": "New Function",
                "code": "function handler() { return 'Hello, Neo N3!'; }",
                "metadata": {
                    "runtime": "javascript",
                    "trigger_type": "neo_block",
                    "trigger_config": { "network": "testnet" }
                }
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is Created (201)
        assert_eq!(resp.status(), StatusCode::CREATED);
        
        // Verify the response body
        let body: Value = test::read_body_json(resp).await;
        assert_eq!(body["id"], "new_func_id");
        assert_eq!(body["name"], "New Function");
        assert_eq!(body["user_id"], "user123");
    }
    
    #[actix_web::test]
    async fn test_update_function_endpoint() {
        // Create a mock database service
        let mut db_service = MockDbService::new();
        
        // Set up the mock behavior for get_function
        db_service
            .expect_get_function()
            .with(eq("func1"))
            .returning(|_| {
                Some(Function {
                    id: "func1".to_string(),
                    user_id: "user123".to_string(),
                    name: "Test Function 1".to_string(),
                    code: "function handler() { return 'Hello, World!'; }".to_string(),
                    metadata: FunctionMetadata {
                        runtime: "javascript".to_string(),
                        trigger_type: "neo_block".to_string(),
                        trigger_config: json!({ "network": "testnet" }),
                    },
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                })
            });
        
        // Set up the mock behavior for update_function
        db_service
            .expect_update_function()
            .returning(|function| {
                Ok(Function {
                    id: function.id,
                    user_id: function.user_id,
                    name: function.name,
                    code: function.code,
                    metadata: function.metadata,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                })
            });
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate a valid JWT token
        let token = generate_test_token("user123");
        
        // Send a PUT request to the update function endpoint
        let req = test::TestRequest::put()
            .uri("/api/functions/func1")
            .header("Authorization", format!("Bearer {}", token))
            .set_json(json!({
                "name": "Updated Function",
                "code": "function handler() { return 'Updated!'; }",
                "metadata": {
                    "runtime": "javascript",
                    "trigger_type": "neo_transaction",
                    "trigger_config": { "network": "testnet", "contract": "0x5678" }
                }
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is OK (200)
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Verify the response body
        let body: Value = test::read_body_json(resp).await;
        assert_eq!(body["id"], "func1");
        assert_eq!(body["name"], "Updated Function");
        assert_eq!(body["user_id"], "user123");
    }
    
    #[actix_web::test]
    async fn test_delete_function_endpoint() {
        // Create a mock database service
        let mut db_service = MockDbService::new();
        
        // Set up the mock behavior for get_function
        db_service
            .expect_get_function()
            .with(eq("func1"))
            .returning(|_| {
                Some(Function {
                    id: "func1".to_string(),
                    user_id: "user123".to_string(),
                    name: "Test Function 1".to_string(),
                    code: "function handler() { return 'Hello, World!'; }".to_string(),
                    metadata: FunctionMetadata {
                        runtime: "javascript".to_string(),
                        trigger_type: "neo_block".to_string(),
                        trigger_config: json!({ "network": "testnet" }),
                    },
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                })
            });
        
        // Set up the mock behavior for delete_function
        db_service
            .expect_delete_function()
            .with(eq("func1"))
            .returning(|_| Ok(()));
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate a valid JWT token
        let token = generate_test_token("user123");
        
        // Send a DELETE request to the delete function endpoint
        let req = test::TestRequest::delete()
            .uri("/api/functions/func1")
            .header("Authorization", format!("Bearer {}", token))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is No Content (204)
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    }
    
    #[actix_web::test]
    async fn test_list_services_endpoint() {
        // Create a mock database service
        let mut db_service = MockDbService::new();
        
        // Set up the mock behavior for list_services
        db_service
            .expect_list_services()
            .with(eq("user123"))
            .returning(|_| {
                vec![
                    Service {
                        id: "svc1".to_string(),
                        user_id: "user123".to_string(),
                        name: "Test Service 1".to_string(),
                        description: "A test service".to_string(),
                        metadata: ServiceMetadata {
                            service_type: "oracle".to_string(),
                            config: json!({ "provider": "price_feed" }),
                        },
                        created_at: chrono::Utc::now(),
                        updated_at: chrono::Utc::now(),
                    },
                    Service {
                        id: "svc2".to_string(),
                        user_id: "user123".to_string(),
                        name: "Test Service 2".to_string(),
                        description: "Another test service".to_string(),
                        metadata: ServiceMetadata {
                            service_type: "tee".to_string(),
                            config: json!({ "provider": "sgx" }),
                        },
                        created_at: chrono::Utc::now(),
                        updated_at: chrono::Utc::now(),
                    },
                ]
            });
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate a valid JWT token
        let token = generate_test_token("user123");
        
        // Send a GET request to the list services endpoint
        let req = test::TestRequest::get()
            .uri("/api/services")
            .header("Authorization", format!("Bearer {}", token))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is OK (200)
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Verify the response body
        let body: Value = test::read_body_json(resp).await;
        assert!(body.is_array());
        assert_eq!(body.as_array().unwrap().len(), 2);
        assert_eq!(body[0]["id"], "svc1");
        assert_eq!(body[1]["id"], "svc2");
    }
    
    #[actix_web::test]
    async fn test_unauthorized_access() {
        // Create a mock database service
        let db_service = MockDbService::new();
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Send a GET request to the list functions endpoint without a token
        let req = test::TestRequest::get()
            .uri("/api/functions")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is Unauthorized (401)
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
    
    #[actix_web::test]
    async fn test_invalid_token() {
        // Create a mock database service
        let db_service = MockDbService::new();
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Send a GET request to the list functions endpoint with an invalid token
        let req = test::TestRequest::get()
            .uri("/api/functions")
            .header("Authorization", "Bearer invalid.token.here")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is Unauthorized (401)
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}
