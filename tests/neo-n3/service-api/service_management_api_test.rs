// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App, HttpResponse, http::StatusCode};
    use r3e_api::routes::services;
    use r3e_api::models::service::{Service, ServiceMetadata};
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
                .service(
                    web::scope("/api/services")
                        .service(services::get_service)
                        .service(services::list_services)
                        .service(services::create_service)
                        .service(services::update_service)
                        .service(services::delete_service)
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
    
    // Helper function to create a test service
    fn create_test_service(id: &str, user_id: &str, name: &str, service_type: &str) -> Service {
        Service {
            id: id.to_string(),
            user_id: user_id.to_string(),
            name: name.to_string(),
            description: format!("Description for {}", name),
            metadata: ServiceMetadata {
                service_type: service_type.to_string(),
                config: json!({ "provider": "test_provider" }),
            },
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    
    #[actix_web::test]
    async fn test_list_services_empty() {
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
        
        // Set up the mock behavior for list_services (empty list)
        db_service
            .expect_list_services()
            .with(eq("user123"))
            .returning(|_| Vec::new());
        
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
        
        // Verify the response body is an empty array
        let body: Value = test::read_body_json(resp).await;
        assert!(body.is_array());
        assert_eq!(body.as_array().unwrap().len(), 0);
    }
    
    #[actix_web::test]
    async fn test_list_services_with_results() {
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
        
        // Set up the mock behavior for list_services
        db_service
            .expect_list_services()
            .with(eq("user123"))
            .returning(|_| {
                vec![
                    create_test_service("svc1", "user123", "Test Oracle Service", "oracle"),
                    create_test_service("svc2", "user123", "Test TEE Service", "tee"),
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
        assert_eq!(body[0]["name"], "Test Oracle Service");
        assert_eq!(body[0]["metadata"]["service_type"], "oracle");
        assert_eq!(body[1]["id"], "svc2");
        assert_eq!(body[1]["name"], "Test TEE Service");
        assert_eq!(body[1]["metadata"]["service_type"], "tee");
    }
    
    #[actix_web::test]
    async fn test_get_service_success() {
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
        
        // Set up the mock behavior for get_service
        db_service
            .expect_get_service()
            .with(eq("svc1"))
            .returning(|_| Some(create_test_service("svc1", "user123", "Test Oracle Service", "oracle")));
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate a valid JWT token
        let token = generate_test_token("user123");
        
        // Send a GET request to the get service endpoint
        let req = test::TestRequest::get()
            .uri("/api/services/svc1")
            .header("Authorization", format!("Bearer {}", token))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is OK (200)
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Verify the response body
        let body: Value = test::read_body_json(resp).await;
        assert_eq!(body["id"], "svc1");
        assert_eq!(body["name"], "Test Oracle Service");
        assert_eq!(body["user_id"], "user123");
        assert_eq!(body["metadata"]["service_type"], "oracle");
    }
    
    #[actix_web::test]
    async fn test_get_service_not_found() {
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
        
        // Set up the mock behavior for get_service (not found)
        db_service
            .expect_get_service()
            .with(eq("nonexistent"))
            .returning(|_| None);
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate a valid JWT token
        let token = generate_test_token("user123");
        
        // Send a GET request to the get service endpoint
        let req = test::TestRequest::get()
            .uri("/api/services/nonexistent")
            .header("Authorization", format!("Bearer {}", token))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is Not Found (404)
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
    
    #[actix_web::test]
    async fn test_get_service_unauthorized() {
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
        
        // Set up the mock behavior for get_service
        db_service
            .expect_get_service()
            .with(eq("svc1"))
            .returning(|_| Some(create_test_service("svc1", "other_user", "Test Oracle Service", "oracle")));
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate a valid JWT token
        let token = generate_test_token("user123");
        
        // Send a GET request to the get service endpoint
        let req = test::TestRequest::get()
            .uri("/api/services/svc1")
            .header("Authorization", format!("Bearer {}", token))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is Forbidden (403)
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }
    
    #[actix_web::test]
    async fn test_create_service_success() {
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
        
        // Set up the mock behavior for create_service
        db_service
            .expect_create_service()
            .returning(|service| {
                Ok(Service {
                    id: "new_svc_id".to_string(),
                    user_id: service.user_id,
                    name: service.name,
                    description: service.description,
                    metadata: service.metadata,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                })
            });
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate a valid JWT token
        let token = generate_test_token("user123");
        
        // Send a POST request to the create service endpoint
        let req = test::TestRequest::post()
            .uri("/api/services")
            .header("Authorization", format!("Bearer {}", token))
            .set_json(json!({
                "name": "New Oracle Service",
                "description": "A new oracle service for testing",
                "metadata": {
                    "service_type": "oracle",
                    "config": { "provider": "price_feed" }
                }
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is Created (201)
        assert_eq!(resp.status(), StatusCode::CREATED);
        
        // Verify the response body
        let body: Value = test::read_body_json(resp).await;
        assert_eq!(body["id"], "new_svc_id");
        assert_eq!(body["name"], "New Oracle Service");
        assert_eq!(body["user_id"], "user123");
        assert_eq!(body["description"], "A new oracle service for testing");
        assert_eq!(body["metadata"]["service_type"], "oracle");
    }
    
    #[actix_web::test]
    async fn test_create_service_validation_error() {
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
        
        // Send a POST request to the create service endpoint with invalid data (missing name)
        let req = test::TestRequest::post()
            .uri("/api/services")
            .header("Authorization", format!("Bearer {}", token))
            .set_json(json!({
                "description": "A new oracle service for testing",
                "metadata": {
                    "service_type": "oracle",
                    "config": { "provider": "price_feed" }
                }
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is Bad Request (400)
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
    
    #[actix_web::test]
    async fn test_update_service_success() {
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
        
        // Set up the mock behavior for get_service
        db_service
            .expect_get_service()
            .with(eq("svc1"))
            .returning(|_| Some(create_test_service("svc1", "user123", "Test Oracle Service", "oracle")));
        
        // Set up the mock behavior for update_service
        db_service
            .expect_update_service()
            .returning(|service| {
                Ok(Service {
                    id: service.id,
                    user_id: service.user_id,
                    name: service.name,
                    description: service.description,
                    metadata: service.metadata,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                })
            });
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate a valid JWT token
        let token = generate_test_token("user123");
        
        // Send a PUT request to the update service endpoint
        let req = test::TestRequest::put()
            .uri("/api/services/svc1")
            .header("Authorization", format!("Bearer {}", token))
            .set_json(json!({
                "name": "Updated Oracle Service",
                "description": "An updated oracle service for testing",
                "metadata": {
                    "service_type": "oracle",
                    "config": { "provider": "random_number" }
                }
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is OK (200)
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Verify the response body
        let body: Value = test::read_body_json(resp).await;
        assert_eq!(body["id"], "svc1");
        assert_eq!(body["name"], "Updated Oracle Service");
        assert_eq!(body["user_id"], "user123");
        assert_eq!(body["description"], "An updated oracle service for testing");
        assert_eq!(body["metadata"]["service_type"], "oracle");
    }
    
    #[actix_web::test]
    async fn test_update_service_not_found() {
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
        
        // Set up the mock behavior for get_service (not found)
        db_service
            .expect_get_service()
            .with(eq("nonexistent"))
            .returning(|_| None);
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate a valid JWT token
        let token = generate_test_token("user123");
        
        // Send a PUT request to the update service endpoint
        let req = test::TestRequest::put()
            .uri("/api/services/nonexistent")
            .header("Authorization", format!("Bearer {}", token))
            .set_json(json!({
                "name": "Updated Oracle Service",
                "description": "An updated oracle service for testing",
                "metadata": {
                    "service_type": "oracle",
                    "config": { "provider": "random_number" }
                }
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is Not Found (404)
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
    
    #[actix_web::test]
    async fn test_update_service_unauthorized() {
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
        
        // Set up the mock behavior for get_service
        db_service
            .expect_get_service()
            .with(eq("svc1"))
            .returning(|_| Some(create_test_service("svc1", "other_user", "Test Oracle Service", "oracle")));
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate a valid JWT token
        let token = generate_test_token("user123");
        
        // Send a PUT request to the update service endpoint
        let req = test::TestRequest::put()
            .uri("/api/services/svc1")
            .header("Authorization", format!("Bearer {}", token))
            .set_json(json!({
                "name": "Updated Oracle Service",
                "description": "An updated oracle service for testing",
                "metadata": {
                    "service_type": "oracle",
                    "config": { "provider": "random_number" }
                }
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is Forbidden (403)
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }
    
    #[actix_web::test]
    async fn test_delete_service_success() {
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
        
        // Set up the mock behavior for get_service
        db_service
            .expect_get_service()
            .with(eq("svc1"))
            .returning(|_| Some(create_test_service("svc1", "user123", "Test Oracle Service", "oracle")));
        
        // Set up the mock behavior for delete_service
        db_service
            .expect_delete_service()
            .with(eq("svc1"))
            .returning(|_| Ok(()));
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate a valid JWT token
        let token = generate_test_token("user123");
        
        // Send a DELETE request to the delete service endpoint
        let req = test::TestRequest::delete()
            .uri("/api/services/svc1")
            .header("Authorization", format!("Bearer {}", token))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is No Content (204)
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    }
    
    #[actix_web::test]
    async fn test_delete_service_not_found() {
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
        
        // Set up the mock behavior for get_service (not found)
        db_service
            .expect_get_service()
            .with(eq("nonexistent"))
            .returning(|_| None);
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate a valid JWT token
        let token = generate_test_token("user123");
        
        // Send a DELETE request to the delete service endpoint
        let req = test::TestRequest::delete()
            .uri("/api/services/nonexistent")
            .header("Authorization", format!("Bearer {}", token))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is Not Found (404)
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
    
    #[actix_web::test]
    async fn test_delete_service_unauthorized() {
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
        
        // Set up the mock behavior for get_service
        db_service
            .expect_get_service()
            .with(eq("svc1"))
            .returning(|_| Some(create_test_service("svc1", "other_user", "Test Oracle Service", "oracle")));
        
        // Create the test app
        let app = create_test_app(db_service);
        
        // Generate a valid JWT token
        let token = generate_test_token("user123");
        
        // Send a DELETE request to the delete service endpoint
        let req = test::TestRequest::delete()
            .uri("/api/services/svc1")
            .header("Authorization", format!("Bearer {}", token))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is Forbidden (403)
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }
    
    #[actix_web::test]
    async fn test_service_type_validation() {
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
        
        // Send a POST request to the create service endpoint with invalid service type
        let req = test::TestRequest::post()
            .uri("/api/services")
            .header("Authorization", format!("Bearer {}", token))
            .set_json(json!({
                "name": "Invalid Service",
                "description": "A service with invalid type",
                "metadata": {
                    "service_type": "invalid_type",
                    "config": { "provider": "test_provider" }
                }
            }))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Verify that the response status is Bad Request (400)
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
