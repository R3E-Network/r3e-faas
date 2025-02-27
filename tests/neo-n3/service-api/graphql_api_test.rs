// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App, HttpResponse, http::StatusCode};
    use r3e_api::routes::graphql;
    use r3e_api::graphql::schema::create_schema;
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
        
        // Create GraphQL schema
        let schema = web::Data::new(create_schema(db_service.clone()));
        
        test::init_service(
            App::new()
                .app_data(db_service.clone())
                .app_data(config.clone())
                .app_data(schema.clone())
                .service(
                    web::resource("/graphql")
                        .route(web::post().to(graphql::graphql_handler))
                        .route(web::get().to(graphql::graphql_playground))
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
    async fn test_graphql_playground() {
        // Create a mock database service
        let db_service = MockDbService::new();
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Send a GET request to the GraphQL playground endpoint
        let req = test::TestRequest::get()
            .uri("/graphql")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is OK (200)
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Verify that the response body contains HTML for the GraphQL playground
        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        
        assert!(body_str.contains("GraphQL Playground"));
    }
    
    #[actix_web::test]
    async fn test_graphql_query_functions() {
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
                    created_at: chrono::Utc::now(),
                })
            });
        
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
        
        // Create a GraphQL query to get functions
        let query = r#"
        {
            functions {
                id
                name
                code
                metadata {
                    runtime
                    triggerType
                    triggerConfig
                }
            }
        }
        "#;
        
        // Send a POST request to the GraphQL endpoint
        let req = test::TestRequest::post()
            .uri("/graphql")
            .header("Authorization", format!("Bearer {}", token))
            .set_json(json!({
                "query": query
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is OK (200)
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Verify the response body
        let body: Value = test::read_body_json(resp).await;
        
        // Check that the data field exists and contains functions
        assert!(body["data"].is_object());
        assert!(body["data"]["functions"].is_array());
        
        // Check that there are 2 functions
        let functions = body["data"]["functions"].as_array().unwrap();
        assert_eq!(functions.len(), 2);
        
        // Check the first function
        assert_eq!(functions[0]["id"], "func1");
        assert_eq!(functions[0]["name"], "Test Function 1");
        assert!(functions[0]["code"].is_string());
        assert_eq!(functions[0]["metadata"]["runtime"], "javascript");
        assert_eq!(functions[0]["metadata"]["triggerType"], "neo_block");
        
        // Check the second function
        assert_eq!(functions[1]["id"], "func2");
        assert_eq!(functions[1]["name"], "Test Function 2");
        assert!(functions[1]["code"].is_string());
        assert_eq!(functions[1]["metadata"]["runtime"], "javascript");
        assert_eq!(functions[1]["metadata"]["triggerType"], "neo_transaction");
    }
    
    #[actix_web::test]
    async fn test_graphql_query_function_by_id() {
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
                    created_at: chrono::Utc::now(),
                })
            });
        
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
        
        // Create a GraphQL query to get a function by ID
        let query = r#"
        {
            function(id: "func1") {
                id
                name
                code
                metadata {
                    runtime
                    triggerType
                    triggerConfig
                }
            }
        }
        "#;
        
        // Send a POST request to the GraphQL endpoint
        let req = test::TestRequest::post()
            .uri("/graphql")
            .header("Authorization", format!("Bearer {}", token))
            .set_json(json!({
                "query": query
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is OK (200)
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Verify the response body
        let body: Value = test::read_body_json(resp).await;
        
        // Check that the data field exists and contains the function
        assert!(body["data"].is_object());
        assert!(body["data"]["function"].is_object());
        
        // Check the function details
        let function = &body["data"]["function"];
        assert_eq!(function["id"], "func1");
        assert_eq!(function["name"], "Test Function 1");
        assert!(function["code"].is_string());
        assert_eq!(function["metadata"]["runtime"], "javascript");
        assert_eq!(function["metadata"]["triggerType"], "neo_block");
    }
    
    #[actix_web::test]
    async fn test_graphql_mutation_create_function() {
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
                    created_at: chrono::Utc::now(),
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
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                })
            });
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate a valid JWT token
        let token = generate_test_token("user123");
        
        // Create a GraphQL mutation to create a function
        let mutation = r#"
        mutation {
            createFunction(
                name: "New GraphQL Function",
                code: "function handler() { return 'Hello from GraphQL!'; }",
                metadata: {
                    runtime: "javascript",
                    triggerType: "neo_block",
                    triggerConfig: "{\"network\":\"testnet\"}"
                }
            ) {
                id
                name
                code
                metadata {
                    runtime
                    triggerType
                    triggerConfig
                }
            }
        }
        "#;
        
        // Send a POST request to the GraphQL endpoint
        let req = test::TestRequest::post()
            .uri("/graphql")
            .header("Authorization", format!("Bearer {}", token))
            .set_json(json!({
                "query": mutation
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is OK (200)
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Verify the response body
        let body: Value = test::read_body_json(resp).await;
        
        // Check that the data field exists and contains the created function
        assert!(body["data"].is_object());
        assert!(body["data"]["createFunction"].is_object());
        
        // Check the function details
        let function = &body["data"]["createFunction"];
        assert_eq!(function["id"], "new_func_id");
        assert_eq!(function["name"], "New GraphQL Function");
        assert!(function["code"].is_string());
        assert_eq!(function["metadata"]["runtime"], "javascript");
        assert_eq!(function["metadata"]["triggerType"], "neo_block");
    }
    
    #[actix_web::test]
    async fn test_graphql_mutation_update_function() {
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
                    created_at: chrono::Utc::now(),
                })
            });
        
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
        
        // Create a GraphQL mutation to update a function
        let mutation = r#"
        mutation {
            updateFunction(
                id: "func1",
                name: "Updated GraphQL Function",
                code: "function handler() { return 'Updated from GraphQL!'; }",
                metadata: {
                    runtime: "javascript",
                    triggerType: "neo_transaction",
                    triggerConfig: "{\"network\":\"testnet\",\"contract\":\"0x5678\"}"
                }
            ) {
                id
                name
                code
                metadata {
                    runtime
                    triggerType
                    triggerConfig
                }
            }
        }
        "#;
        
        // Send a POST request to the GraphQL endpoint
        let req = test::TestRequest::post()
            .uri("/graphql")
            .header("Authorization", format!("Bearer {}", token))
            .set_json(json!({
                "query": mutation
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is OK (200)
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Verify the response body
        let body: Value = test::read_body_json(resp).await;
        
        // Check that the data field exists and contains the updated function
        assert!(body["data"].is_object());
        assert!(body["data"]["updateFunction"].is_object());
        
        // Check the function details
        let function = &body["data"]["updateFunction"];
        assert_eq!(function["id"], "func1");
        assert_eq!(function["name"], "Updated GraphQL Function");
        assert!(function["code"].is_string());
        assert_eq!(function["metadata"]["runtime"], "javascript");
        assert_eq!(function["metadata"]["triggerType"], "neo_transaction");
    }
    
    #[actix_web::test]
    async fn test_graphql_mutation_delete_function() {
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
                    created_at: chrono::Utc::now(),
                })
            });
        
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
        
        // Create a GraphQL mutation to delete a function
        let mutation = r#"
        mutation {
            deleteFunction(id: "func1")
        }
        "#;
        
        // Send a POST request to the GraphQL endpoint
        let req = test::TestRequest::post()
            .uri("/graphql")
            .header("Authorization", format!("Bearer {}", token))
            .set_json(json!({
                "query": mutation
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is OK (200)
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Verify the response body
        let body: Value = test::read_body_json(resp).await;
        
        // Check that the data field exists and contains the result
        assert!(body["data"].is_object());
        assert!(body["data"]["deleteFunction"].is_boolean());
        assert_eq!(body["data"]["deleteFunction"], true);
    }
    
    #[actix_web::test]
    async fn test_graphql_query_services() {
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
                    created_at: chrono::Utc::now(),
                })
            });
        
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
        
        // Create a GraphQL query to get services
        let query = r#"
        {
            services {
                id
                name
                description
                metadata {
                    serviceType
                    config
                }
            }
        }
        "#;
        
        // Send a POST request to the GraphQL endpoint
        let req = test::TestRequest::post()
            .uri("/graphql")
            .header("Authorization", format!("Bearer {}", token))
            .set_json(json!({
                "query": query
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is OK (200)
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Verify the response body
        let body: Value = test::read_body_json(resp).await;
        
        // Check that the data field exists and contains services
        assert!(body["data"].is_object());
        assert!(body["data"]["services"].is_array());
        
        // Check that there are 2 services
        let services = body["data"]["services"].as_array().unwrap();
        assert_eq!(services.len(), 2);
        
        // Check the first service
        assert_eq!(services[0]["id"], "svc1");
        assert_eq!(services[0]["name"], "Test Service 1");
        assert_eq!(services[0]["description"], "A test service");
        assert_eq!(services[0]["metadata"]["serviceType"], "oracle");
        
        // Check the second service
        assert_eq!(services[1]["id"], "svc2");
        assert_eq!(services[1]["name"], "Test Service 2");
        assert_eq!(services[1]["description"], "Another test service");
        assert_eq!(services[1]["metadata"]["serviceType"], "tee");
    }
    
    #[actix_web::test]
    async fn test_graphql_unauthorized() {
        // Create a mock database service
        let db_service = MockDbService::new();
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Create a GraphQL query to get functions
        let query = r#"
        {
            functions {
                id
                name
            }
        }
        "#;
        
        // Send a POST request to the GraphQL endpoint without a token
        let req = test::TestRequest::post()
            .uri("/graphql")
            .set_json(json!({
                "query": query
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is OK (200) - GraphQL returns errors in the response body
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Verify the response body contains an error
        let body: Value = test::read_body_json(resp).await;
        
        // Check that the errors field exists
        assert!(body["errors"].is_array());
        
        // Check that the error message is about authentication
        let errors = body["errors"].as_array().unwrap();
        assert!(errors[0]["message"].as_str().unwrap().contains("authentication"));
    }
}
