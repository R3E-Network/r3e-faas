// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

#[cfg(test)]
mod tests {
    use r3e_oracle::service::{OracleService, OracleServiceConfig, OracleServiceRegistry};
    use r3e_oracle::types::{OracleError, OracleRequest, OracleResponse, OracleType};
    use r3e_oracle::provider::{OracleProvider, ProviderRegistry};
    use std::sync::Arc;
    use std::time::{Duration, SystemTime};
    use mockall::predicate::*;
    use mockall::mock;
    use async_trait::async_trait;
    use std::collections::HashMap;

    // Mock the OracleProvider trait for testing
    mock! {
        OracleProvider {}
        trait OracleProvider {
            fn get_type(&self) -> OracleType;
            async fn process_request(&self, request: OracleRequest) -> Result<OracleResponse, OracleError>;
            fn get_name(&self) -> &str;
            fn get_description(&self) -> &str;
        }
    }

    // Mock the OracleServiceRegistry trait for testing
    mock! {
        OracleServiceRegistry {}
        trait OracleServiceRegistry {
            async fn register_service(&self, service: OracleService) -> Result<String, OracleError>;
            async fn get_service(&self, service_id: &str) -> Result<OracleService, OracleError>;
            async fn list_services(&self) -> Result<Vec<OracleService>, OracleError>;
            async fn update_service(&self, service_id: &str, service: OracleService) -> Result<(), OracleError>;
            async fn delete_service(&self, service_id: &str) -> Result<(), OracleError>;
        }
    }

    // Mock the ProviderRegistry trait for testing
    mock! {
        ProviderRegistry {}
        trait ProviderRegistry {
            async fn register_provider(&self, provider: Arc<dyn OracleProvider>) -> Result<(), OracleError>;
            async fn get_provider(&self, provider_type: OracleType) -> Result<Arc<dyn OracleProvider>, OracleError>;
            async fn list_providers(&self) -> Result<Vec<OracleType>, OracleError>;
            async fn unregister_provider(&self, provider_type: OracleType) -> Result<(), OracleError>;
        }
    }

    // Helper function to create a mock provider
    fn create_mock_provider() -> MockOracleProvider {
        let mut provider = MockOracleProvider::new();
        
        // Set up default behavior for get_type
        provider.expect_get_type()
            .returning(|| {
                OracleType::Custom("weather".to_string())
            });
        
        // Set up default behavior for process_request
        provider.expect_process_request()
            .returning(|request| {
                match request.data.get("location") {
                    Some(location) if location == "New York" => {
                        let mut response_data = HashMap::new();
                        response_data.insert("temperature".to_string(), "72".to_string());
                        response_data.insert("conditions".to_string(), "sunny".to_string());
                        
                        Ok(OracleResponse {
                            request_id: request.id,
                            data: response_data,
                            timestamp: SystemTime::now(),
                            provider: "weather".to_string(),
                        })
                    },
                    Some(location) if location == "error_location" => {
                        Err(OracleError::InvalidRequest("Invalid location".to_string()))
                    },
                    _ => {
                        Err(OracleError::InvalidRequest("Location not provided".to_string()))
                    }
                }
            });
        
        // Set up default behavior for get_name
        provider.expect_get_name()
            .returning(|| {
                "Weather Oracle"
            });
        
        // Set up default behavior for get_description
        provider.expect_get_description()
            .returning(|| {
                "Provides weather information for a given location"
            });
        
        provider
    }

    // Helper function to create a mock service registry
    fn create_mock_service_registry() -> MockOracleServiceRegistry {
        let mut registry = MockOracleServiceRegistry::new();
        
        // Set up default behavior for register_service
        registry.expect_register_service()
            .returning(|service| {
                Ok("service123".to_string())
            });
        
        // Set up default behavior for get_service
        registry.expect_get_service()
            .with(eq("service123"))
            .returning(|_| {
                Ok(OracleService {
                    id: Some("service123".to_string()),
                    name: "Weather Service".to_string(),
                    description: "Provides weather information".to_string(),
                    provider_type: OracleType::Custom("weather".to_string()),
                    config: OracleServiceConfig {
                        rate_limit: Some(100),
                        cache_ttl: Some(300),
                        timeout: Some(5000),
                        custom_config: Some(HashMap::new()),
                    },
                    created_at: SystemTime::now(),
                    updated_at: SystemTime::now(),
                })
            });
        
        // Set up behavior for get_service with invalid service ID
        registry.expect_get_service()
            .with(eq("invalid_service"))
            .returning(|service_id| {
                Err(OracleError::ServiceNotFound(service_id.to_string()))
            });
        
        // Set up default behavior for list_services
        registry.expect_list_services()
            .returning(|| {
                Ok(vec![
                    OracleService {
                        id: Some("service123".to_string()),
                        name: "Weather Service".to_string(),
                        description: "Provides weather information".to_string(),
                        provider_type: OracleType::Custom("weather".to_string()),
                        config: OracleServiceConfig {
                            rate_limit: Some(100),
                            cache_ttl: Some(300),
                            timeout: Some(5000),
                            custom_config: Some(HashMap::new()),
                        },
                        created_at: SystemTime::now(),
                        updated_at: SystemTime::now(),
                    },
                    OracleService {
                        id: Some("service456".to_string()),
                        name: "Stock Price Service".to_string(),
                        description: "Provides stock price information".to_string(),
                        provider_type: OracleType::Custom("stock".to_string()),
                        config: OracleServiceConfig {
                            rate_limit: Some(50),
                            cache_ttl: Some(60),
                            timeout: Some(3000),
                            custom_config: Some(HashMap::new()),
                        },
                        created_at: SystemTime::now(),
                        updated_at: SystemTime::now(),
                    },
                ])
            });
        
        // Set up default behavior for update_service
        registry.expect_update_service()
            .with(eq("service123"), function(|service: &OracleService| {
                service.name == "Updated Weather Service"
            }))
            .returning(|_, _| {
                Ok(())
            });
        
        // Set up behavior for update_service with invalid service ID
        registry.expect_update_service()
            .with(eq("invalid_service"), any())
            .returning(|service_id, _| {
                Err(OracleError::ServiceNotFound(service_id.to_string()))
            });
        
        // Set up default behavior for delete_service
        registry.expect_delete_service()
            .with(eq("service123"))
            .returning(|_| {
                Ok(())
            });
        
        // Set up behavior for delete_service with invalid service ID
        registry.expect_delete_service()
            .with(eq("invalid_service"))
            .returning(|service_id| {
                Err(OracleError::ServiceNotFound(service_id.to_string()))
            });
        
        registry
    }

    // Helper function to create a mock provider registry
    fn create_mock_provider_registry() -> MockProviderRegistry {
        let mut registry = MockProviderRegistry::new();
        
        // Set up default behavior for register_provider
        registry.expect_register_provider()
            .returning(|_| {
                Ok(())
            });
        
        // Set up default behavior for get_provider
        registry.expect_get_provider()
            .with(eq(OracleType::Custom("weather".to_string())))
            .returning(|_| {
                let provider = create_mock_provider();
                Ok(Arc::new(provider) as Arc<dyn OracleProvider>)
            });
        
        // Set up behavior for get_provider with invalid provider type
        registry.expect_get_provider()
            .with(eq(OracleType::Custom("invalid_provider".to_string())))
            .returning(|provider_type| {
                Err(OracleError::ProviderNotFound(format!("{:?}", provider_type)))
            });
        
        // Set up default behavior for list_providers
        registry.expect_list_providers()
            .returning(|| {
                Ok(vec![
                    OracleType::Custom("weather".to_string()),
                    OracleType::Custom("stock".to_string()),
                ])
            });
        
        // Set up default behavior for unregister_provider
        registry.expect_unregister_provider()
            .with(eq(OracleType::Custom("weather".to_string())))
            .returning(|_| {
                Ok(())
            });
        
        // Set up behavior for unregister_provider with invalid provider type
        registry.expect_unregister_provider()
            .with(eq(OracleType::Custom("invalid_provider".to_string())))
            .returning(|provider_type| {
                Err(OracleError::ProviderNotFound(format!("{:?}", provider_type)))
            });
        
        registry
    }

    // Custom Oracle Provider implementation for testing
    struct CustomWeatherProvider {
        name: String,
        description: String,
    }

    impl CustomWeatherProvider {
        fn new(name: &str, description: &str) -> Self {
            CustomWeatherProvider {
                name: name.to_string(),
                description: description.to_string(),
            }
        }
    }

    #[async_trait]
    impl OracleProvider for CustomWeatherProvider {
        fn get_type(&self) -> OracleType {
            OracleType::Custom("weather".to_string())
        }

        async fn process_request(&self, request: OracleRequest) -> Result<OracleResponse, OracleError> {
            match request.data.get("location") {
                Some(location) if !location.is_empty() => {
                    let mut response_data = HashMap::new();
                    response_data.insert("temperature".to_string(), "68".to_string());
                    response_data.insert("conditions".to_string(), "partly cloudy".to_string());
                    
                    Ok(OracleResponse {
                        request_id: request.id,
                        data: response_data,
                        timestamp: SystemTime::now(),
                        provider: "custom_weather".to_string(),
                    })
                },
                _ => {
                    Err(OracleError::InvalidRequest("Location not provided".to_string()))
                }
            }
        }

        fn get_name(&self) -> &str {
            &self.name
        }

        fn get_description(&self) -> &str {
            &self.description
        }
    }

    #[tokio::test]
    async fn test_register_custom_provider() {
        // Create a mock provider registry
        let provider_registry = create_mock_provider_registry();
        
        // Create a custom provider
        let provider = create_mock_provider();
        
        // Register the provider
        let result = provider_registry.register_provider(Arc::new(provider)).await;
        
        // Verify that the registration was successful
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_custom_provider() {
        // Create a mock provider registry
        let provider_registry = create_mock_provider_registry();
        
        // Get the provider
        let provider = provider_registry.get_provider(OracleType::Custom("weather".to_string())).await.unwrap();
        
        // Verify the provider
        assert_eq!(provider.get_type(), OracleType::Custom("weather".to_string()));
        assert_eq!(provider.get_name(), "Weather Oracle");
        assert_eq!(provider.get_description(), "Provides weather information for a given location");
    }

    #[tokio::test]
    async fn test_get_invalid_provider() {
        // Create a mock provider registry
        let provider_registry = create_mock_provider_registry();
        
        // Try to get an invalid provider
        let result = provider_registry.get_provider(OracleType::Custom("invalid_provider".to_string())).await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(OracleError::ProviderNotFound(provider_type)) => {
                assert!(provider_type.contains("invalid_provider"));
            }
            _ => panic!("Expected ProviderNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_list_providers() {
        // Create a mock provider registry
        let provider_registry = create_mock_provider_registry();
        
        // List the providers
        let providers = provider_registry.list_providers().await.unwrap();
        
        // Verify the providers
        assert_eq!(providers.len(), 2);
        assert!(providers.contains(&OracleType::Custom("weather".to_string())));
        assert!(providers.contains(&OracleType::Custom("stock".to_string())));
    }

    #[tokio::test]
    async fn test_unregister_provider() {
        // Create a mock provider registry
        let provider_registry = create_mock_provider_registry();
        
        // Unregister the provider
        let result = provider_registry.unregister_provider(OracleType::Custom("weather".to_string())).await;
        
        // Verify that the unregistration was successful
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_unregister_invalid_provider() {
        // Create a mock provider registry
        let provider_registry = create_mock_provider_registry();
        
        // Try to unregister an invalid provider
        let result = provider_registry.unregister_provider(OracleType::Custom("invalid_provider".to_string())).await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(OracleError::ProviderNotFound(provider_type)) => {
                assert!(provider_type.contains("invalid_provider"));
            }
            _ => panic!("Expected ProviderNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_register_custom_service() {
        // Create a mock service registry
        let service_registry = create_mock_service_registry();
        
        // Create a custom service
        let service = OracleService {
            id: None, // ID will be assigned by the registry
            name: "Weather Service".to_string(),
            description: "Provides weather information".to_string(),
            provider_type: OracleType::Custom("weather".to_string()),
            config: OracleServiceConfig {
                rate_limit: Some(100),
                cache_ttl: Some(300),
                timeout: Some(5000),
                custom_config: Some(HashMap::new()),
            },
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        };
        
        // Register the service
        let service_id = service_registry.register_service(service).await.unwrap();
        
        // Verify the service ID
        assert_eq!(service_id, "service123");
    }

    #[tokio::test]
    async fn test_get_custom_service() {
        // Create a mock service registry
        let service_registry = create_mock_service_registry();
        
        // Get the service
        let service = service_registry.get_service("service123").await.unwrap();
        
        // Verify the service
        assert_eq!(service.id, Some("service123".to_string()));
        assert_eq!(service.name, "Weather Service");
        assert_eq!(service.description, "Provides weather information");
        assert_eq!(service.provider_type, OracleType::Custom("weather".to_string()));
        assert_eq!(service.config.rate_limit, Some(100));
        assert_eq!(service.config.cache_ttl, Some(300));
        assert_eq!(service.config.timeout, Some(5000));
    }

    #[tokio::test]
    async fn test_get_invalid_service() {
        // Create a mock service registry
        let service_registry = create_mock_service_registry();
        
        // Try to get an invalid service
        let result = service_registry.get_service("invalid_service").await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(OracleError::ServiceNotFound(service_id)) => {
                assert_eq!(service_id, "invalid_service");
            }
            _ => panic!("Expected ServiceNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_list_services() {
        // Create a mock service registry
        let service_registry = create_mock_service_registry();
        
        // List the services
        let services = service_registry.list_services().await.unwrap();
        
        // Verify the services
        assert_eq!(services.len(), 2);
        assert_eq!(services[0].id, Some("service123".to_string()));
        assert_eq!(services[0].name, "Weather Service");
        assert_eq!(services[1].id, Some("service456".to_string()));
        assert_eq!(services[1].name, "Stock Price Service");
    }

    #[tokio::test]
    async fn test_update_service() {
        // Create a mock service registry
        let service_registry = create_mock_service_registry();
        
        // Create an updated service
        let updated_service = OracleService {
            id: Some("service123".to_string()),
            name: "Updated Weather Service".to_string(),
            description: "Provides updated weather information".to_string(),
            provider_type: OracleType::Custom("weather".to_string()),
            config: OracleServiceConfig {
                rate_limit: Some(200),
                cache_ttl: Some(600),
                timeout: Some(10000),
                custom_config: Some(HashMap::new()),
            },
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        };
        
        // Update the service
        let result = service_registry.update_service("service123", updated_service).await;
        
        // Verify that the update was successful
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_invalid_service() {
        // Create a mock service registry
        let service_registry = create_mock_service_registry();
        
        // Create an updated service
        let updated_service = OracleService {
            id: Some("invalid_service".to_string()),
            name: "Updated Weather Service".to_string(),
            description: "Provides updated weather information".to_string(),
            provider_type: OracleType::Custom("weather".to_string()),
            config: OracleServiceConfig {
                rate_limit: Some(200),
                cache_ttl: Some(600),
                timeout: Some(10000),
                custom_config: Some(HashMap::new()),
            },
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        };
        
        // Try to update an invalid service
        let result = service_registry.update_service("invalid_service", updated_service).await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(OracleError::ServiceNotFound(service_id)) => {
                assert_eq!(service_id, "invalid_service");
            }
            _ => panic!("Expected ServiceNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_delete_service() {
        // Create a mock service registry
        let service_registry = create_mock_service_registry();
        
        // Delete the service
        let result = service_registry.delete_service("service123").await;
        
        // Verify that the deletion was successful
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_invalid_service() {
        // Create a mock service registry
        let service_registry = create_mock_service_registry();
        
        // Try to delete an invalid service
        let result = service_registry.delete_service("invalid_service").await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(OracleError::ServiceNotFound(service_id)) => {
                assert_eq!(service_id, "invalid_service");
            }
            _ => panic!("Expected ServiceNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_process_request_with_custom_provider() {
        // Create a mock provider
        let provider = create_mock_provider();
        
        // Create a request
        let mut request_data = HashMap::new();
        request_data.insert("location".to_string(), "New York".to_string());
        
        let request = OracleRequest {
            id: "request123".to_string(),
            data: request_data,
            timestamp: SystemTime::now(),
        };
        
        // Process the request
        let response = provider.process_request(request).await.unwrap();
        
        // Verify the response
        assert_eq!(response.request_id, "request123");
        assert_eq!(response.provider, "weather");
        assert_eq!(response.data.get("temperature"), Some(&"72".to_string()));
        assert_eq!(response.data.get("conditions"), Some(&"sunny".to_string()));
    }

    #[tokio::test]
    async fn test_process_request_with_error() {
        // Create a mock provider
        let provider = create_mock_provider();
        
        // Create a request with an error location
        let mut request_data = HashMap::new();
        request_data.insert("location".to_string(), "error_location".to_string());
        
        let request = OracleRequest {
            id: "request123".to_string(),
            data: request_data,
            timestamp: SystemTime::now(),
        };
        
        // Process the request
        let result = provider.process_request(request).await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(OracleError::InvalidRequest(message)) => {
                assert_eq!(message, "Invalid location");
            }
            _ => panic!("Expected InvalidRequest error"),
        }
    }

    #[tokio::test]
    async fn test_custom_provider_implementation() {
        // Create a custom provider
        let provider = CustomWeatherProvider::new(
            "Custom Weather Oracle",
            "Custom provider for weather information",
        );
        
        // Verify the provider
        assert_eq!(provider.get_type(), OracleType::Custom("weather".to_string()));
        assert_eq!(provider.get_name(), "Custom Weather Oracle");
        assert_eq!(provider.get_description(), "Custom provider for weather information");
        
        // Create a request
        let mut request_data = HashMap::new();
        request_data.insert("location".to_string(), "Boston".to_string());
        
        let request = OracleRequest {
            id: "request456".to_string(),
            data: request_data,
            timestamp: SystemTime::now(),
        };
        
        // Process the request
        let response = provider.process_request(request).await.unwrap();
        
        // Verify the response
        assert_eq!(response.request_id, "request456");
        assert_eq!(response.provider, "custom_weather");
        assert_eq!(response.data.get("temperature"), Some(&"68".to_string()));
        assert_eq!(response.data.get("conditions"), Some(&"partly cloudy".to_string()));
    }

    #[tokio::test]
    async fn test_custom_provider_error_handling() {
        // Create a custom provider
        let provider = CustomWeatherProvider::new(
            "Custom Weather Oracle",
            "Custom provider for weather information",
        );
        
        // Create a request without a location
        let request_data = HashMap::new();
        
        let request = OracleRequest {
            id: "request456".to_string(),
            data: request_data,
            timestamp: SystemTime::now(),
        };
        
        // Process the request
        let result = provider.process_request(request).await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(OracleError::InvalidRequest(message)) => {
                assert_eq!(message, "Location not provided");
            }
            _ => panic!("Expected InvalidRequest error"),
        }
    }

    // Test with a complete end-to-end flow
    #[tokio::test]
    async fn test_end_to_end_custom_oracle_service() {
        // Create a mock provider registry
        let provider_registry = create_mock_provider_registry();
        
        // Create a mock service registry
        let service_registry = create_mock_service_registry();
        
        // Create a custom provider
        let provider = create_mock_provider();
        
        // Register the provider
        provider_registry.register_provider(Arc::new(provider)).await.unwrap();
        
        // Create a custom service
        let service = OracleService {
            id: None, // ID will be assigned by the registry
            name: "Weather Service".to_string(),
            description: "Provides weather information".to_string(),
            provider_type: OracleType::Custom("weather".to_string()),
            config: OracleServiceConfig {
                rate_limit: Some(100),
                cache_ttl: Some(300),
                timeout: Some(5000),
                custom_config: Some(HashMap::new()),
            },
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        };
        
        // Register the service
        let service_id = service_registry.register_service(service).await.unwrap();
        
        // Get the service
        let service = service_registry.get_service(&service_id).await.unwrap();
        
        // Get the provider for the service
        let provider = provider_registry.get_provider(service.provider_type.clone()).await.unwrap();
        
        // Create a request
        let mut request_data = HashMap::new();
        request_data.insert("location".to_string(), "New York".to_string());
        
        let request = OracleRequest {
            id: "request789".to_string(),
            data: request_data,
            timestamp: SystemTime::now(),
        };
        
        // Process the request using the provider
        let response = provider.process_request(request).await.unwrap();
        
        // Verify the response
        assert_eq!(response.request_id, "request789");
        assert_eq!(response.provider, "weather");
        assert_eq!(response.data.get("temperature"), Some(&"72".to_string()));
        assert_eq!(response.data.get("conditions"), Some(&"sunny".to_string()));
    }
}
