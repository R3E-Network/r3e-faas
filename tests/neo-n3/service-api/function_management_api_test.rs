// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App, HttpResponse, http::StatusCode};
    use r3e_api::routes::functions;
    use r3e_api::models::function::{Function, FunctionMetadata};
    use r3e_api::models::user::User;
    use r3e_api::auth::{JwtClaims, generate_token};
    use r3e_api::config::Config;
    use serde_json::{json, Value};
    use std::sync::Arc;
    use mockall::predicate::*;
    use mockall::mock;
    use chrono::Utc;
    
    // Mock the database service
    mock! {
        DbService {}
        trait DbService {
            async fn get_user_by_id(&self, id: &str) -> Option<User>;
            async fn get_function(&self, id: &str) -> Option<Function>;
            async fn list_functions(&self, user_id: &str) -> Vec<Function>;
            async fn create_function(&self, function: Function) -> Result<Function, String>;
            async fn update_function(&self, function: Function) -> Result<Function, String>;
            async fn delete_function(&self, id: &str) -> Result<(), String>;
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
                    web::scope("/api/functions")
                        .service(functions::get_function)
                        .service(functions::list_functions)
                        .service(functions::create_function)
                        .service(functions::update_function)
                        .service(functions::delete_function)
                )
        )
    }
    
    // Helper function to generate a valid JWT token for testing
    fn generate_test_token(user_id: &str) -> String {
        let claims = JwtClaims {
            sub: user_id.to_string(),
            exp: (Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
        };
        
        generate_token(&claims, "test_secret").unwrap()
    }
    
    // Helper function to create a test function
    fn create_test_function(id: &str, user_id: &str, name: &str) -> Function {
        Function {
            id: id.to_string(),
            user_id: user_id.to_string(),
            name: name.to_string(),
            code: "function handler() { return 'Hello, World!'; }".to_string(),
            metadata: FunctionMetadata {
                runtime: "javascript".to_string(),
                trigger_type: "neo_block".to_string(),
                trigger_config: json!({ "network": "testnet" }),
            },
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    
    #[actix_web::test]
    async fn test_list_functions_empty() {
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
        
        // Set up the mock behavior for list_functions (empty list)
        db_service
            .expect_list_functions()
            .with(eq("user123"))
            .returning(|_| Vec::new());
        
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
        
        // Verify the response body is an empty array
        let body: Value = test::read_body_json(resp).await;
        assert!(body.is_array());
        assert_eq!(body.as_array().unwrap().len(), 0);
    }
    
    #[actix_web::test]
    async fn test_list_functions_with_results() {
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
        
        // Set up the mock behavior for list_functions
        db_service
            .expect_list_functions()
            .with(eq("user123"))
            .returning(|_| {
                vec![
                    create_test_function("func1", "user123", "Test Function 1"),
                    create_test_function("func2", "user123", "Test Function 2"),
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
    async fn test_get_function_success() {
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
        
        // Set up the mock behavior for get_function
        db_service
            .expect_get_function()
            .with(eq("func1"))
            .returning(|_| Some(create_test_function("func1", "user123", "Test Function 1")));
        
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
    async fn test_get_function_not_found() {
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
        
        // Set up the mock behavior for get_function (not found)
        db_service
            .expect_get_function()
            .with(eq("nonexistent"))
            .returning(|_| None);
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate a valid JWT token
        let token = generate_test_token("user123");
        
        // Send a GET request to the get function endpoint
        let req = test::TestRequest::get()
            .uri("/api/functions/nonexistent")
            .header("Authorization", format!("Bearer {}", token))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is Not Found (404)
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
    
    #[actix_web::test]
    async fn test_get_function_unauthorized() {
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
        
        // Set up the mock behavior for get_function
        db_service
            .expect_get_function()
            .with(eq("func1"))
            .returning(|_| Some(create_test_function("func1", "other_user", "Test Function 1")));
        
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
        
        // Verify that the response status is Forbidden (403)
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }
    
    #[actix_web::test]
    async fn test_create_function_success() {
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
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
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
    async fn test_create_function_validation_error() {
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
        let token = generate_test_token("user123");
        
        // Send a POST request to the create function endpoint with invalid data (missing name)
        let req = test::TestRequest::post()
            .uri("/api/functions")
            .header("Authorization", format!("Bearer {}", token))
            .set_json(json!({
                "code": "function handler() { return 'Hello, Neo N3!'; }",
                "metadata": {
                    "runtime": "javascript",
                    "trigger_type": "neo_block",
                    "trigger_config": { "network": "testnet" }
                }
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is Bad Request (400)
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
    
    #[actix_web::test]
    async fn test_update_function_success() {
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
        
        // Set up the mock behavior for get_function
        db_service
            .expect_get_function()
            .with(eq("func1"))
            .returning(|_| Some(create_test_function("func1", "user123", "Test Function 1")));
        
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
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
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
    async fn test_update_function_not_found() {
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
        
        // Set up the mock behavior for get_function (not found)
        db_service
            .expect_get_function()
            .with(eq("nonexistent"))
            .returning(|_| None);
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate a valid JWT token
        let token = generate_test_token("user123");
        
        // Send a PUT request to the update function endpoint
        let req = test::TestRequest::put()
            .uri("/api/functions/nonexistent")
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
        
        // Verify that the response status is Not Found (404)
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
    
    #[actix_web::test]
    async fn test_update_function_unauthorized() {
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
        
        // Set up the mock behavior for get_function
        db_service
            .expect_get_function()
            .with(eq("func1"))
            .returning(|_| Some(create_test_function("func1", "other_user", "Test Function 1")));
        
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
        
        // Verify that the response status is Forbidden (403)
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }
    
    #[actix_web::test]
    async fn test_delete_function_success() {
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
        
        // Set up the mock behavior for get_function
        db_service
            .expect_get_function()
            .with(eq("func1"))
            .returning(|_| Some(create_test_function("func1", "user123", "Test Function 1")));
        
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
    async fn test_delete_function_not_found() {
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
        
        // Set up the mock behavior for get_function (not found)
        db_service
            .expect_get_function()
            .with(eq("nonexistent"))
            .returning(|_| None);
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate a valid JWT token
        let token = generate_test_token("user123");
        
        // Send a DELETE request to the delete function endpoint
        let req = test::TestRequest::delete()
            .uri("/api/functions/nonexistent")
            .header("Authorization", format!("Bearer {}", token))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is Not Found (404)
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
    
    #[actix_web::test]
    async fn test_delete_function_unauthorized() {
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
        
        // Set up the mock behavior for get_function
        db_service
            .expect_get_function()
            .with(eq("func1"))
            .returning(|_| Some(create_test_function("func1", "other_user", "Test Function 1")));
        
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
        
        // Verify that the response status is Forbidden (403)
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }
}
