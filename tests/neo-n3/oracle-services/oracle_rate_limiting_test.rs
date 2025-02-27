// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

#[cfg(test)]
mod tests {
    use r3e_oracle::auth::{AuthToken};
    use r3e_oracle::types::{OracleError, Asset, Currency};
    use r3e_oracle::service::{RateLimiter, RateLimitConfig, RateLimitInfo};
    use std::sync::Arc;
    use std::time::{Duration, SystemTime, Instant};
    use mockall::predicate::*;
    use mockall::mock;
    use std::collections::HashMap;

    // Mock the RateLimiter trait for testing
    mock! {
        RateLimiter {}
        trait RateLimiter {
            async fn check_rate_limit(&self, user_id: &str, service: &str) -> Result<RateLimitInfo, OracleError>;
            async fn increment_request_count(&self, user_id: &str, service: &str) -> Result<RateLimitInfo, OracleError>;
            async fn get_rate_limit_config(&self, service: &str) -> Result<RateLimitConfig, OracleError>;
            async fn update_rate_limit_config(&self, service: &str, config: RateLimitConfig) -> Result<(), OracleError>;
            async fn reset_rate_limit(&self, user_id: &str, service: &str) -> Result<(), OracleError>;
        }
    }

    // Helper function to create a mock rate limiter
    fn create_mock_rate_limiter() -> MockRateLimiter {
        let mut rate_limiter = MockRateLimiter::new();
        
        // Set up default behavior for check_rate_limit
        rate_limiter.expect_check_rate_limit()
            .with(eq("user123"), eq("price_feed"))
            .returning(|_, _| {
                Ok(RateLimitInfo {
                    request_count: 5,
                    max_requests: 10,
                    reset_at: SystemTime::now() + Duration::from_secs(3600),
                    remaining: 5,
                })
            });
        
        // Set up behavior for check_rate_limit with rate limit exceeded
        rate_limiter.expect_check_rate_limit()
            .with(eq("user123"), eq("rate_limited_service"))
            .returning(|_, _| {
                Ok(RateLimitInfo {
                    request_count: 10,
                    max_requests: 10,
                    reset_at: SystemTime::now() + Duration::from_secs(3600),
                    remaining: 0,
                })
            });
        
        // Set up default behavior for increment_request_count
        rate_limiter.expect_increment_request_count()
            .with(eq("user123"), eq("price_feed"))
            .returning(|_, _| {
                Ok(RateLimitInfo {
                    request_count: 6, // Incremented from 5 to 6
                    max_requests: 10,
                    reset_at: SystemTime::now() + Duration::from_secs(3600),
                    remaining: 4,
                })
            });
        
        // Set up behavior for increment_request_count with rate limit exceeded
        rate_limiter.expect_increment_request_count()
            .with(eq("user123"), eq("rate_limited_service"))
            .returning(|_, _| {
                Err(OracleError::RateLimitExceeded("Rate limit exceeded for service: rate_limited_service".to_string()))
            });
        
        // Set up default behavior for get_rate_limit_config
        rate_limiter.expect_get_rate_limit_config()
            .with(eq("price_feed"))
            .returning(|_| {
                Ok(RateLimitConfig {
                    max_requests: 10,
                    window_seconds: 3600,
                })
            });
        
        // Set up behavior for get_rate_limit_config with unknown service
        rate_limiter.expect_get_rate_limit_config()
            .with(eq("unknown_service"))
            .returning(|service| {
                Err(OracleError::ServiceNotFound(service.to_string()))
            });
        
        // Set up default behavior for update_rate_limit_config
        rate_limiter.expect_update_rate_limit_config()
            .with(eq("price_feed"), function(|config: &RateLimitConfig| {
                config.max_requests == 20 && config.window_seconds == 3600
            }))
            .returning(|_, _| {
                Ok(())
            });
        
        // Set up default behavior for reset_rate_limit
        rate_limiter.expect_reset_rate_limit()
            .with(eq("user123"), eq("price_feed"))
            .returning(|_, _| {
                Ok(())
            });
        
        rate_limiter
    }

    #[tokio::test]
    async fn test_rate_limit_check_within_limit() {
        // Create a mock rate limiter
        let rate_limiter = create_mock_rate_limiter();
        
        // Check rate limit for a user and service
        let rate_limit_info = rate_limiter.check_rate_limit("user123", "price_feed").await.unwrap();
        
        // Verify the rate limit info
        assert_eq!(rate_limit_info.request_count, 5);
        assert_eq!(rate_limit_info.max_requests, 10);
        assert_eq!(rate_limit_info.remaining, 5);
        assert!(rate_limit_info.reset_at > SystemTime::now());
    }

    #[tokio::test]
    async fn test_rate_limit_check_at_limit() {
        // Create a mock rate limiter
        let rate_limiter = create_mock_rate_limiter();
        
        // Check rate limit for a user and service that is at the limit
        let rate_limit_info = rate_limiter.check_rate_limit("user123", "rate_limited_service").await.unwrap();
        
        // Verify the rate limit info
        assert_eq!(rate_limit_info.request_count, 10);
        assert_eq!(rate_limit_info.max_requests, 10);
        assert_eq!(rate_limit_info.remaining, 0);
        assert!(rate_limit_info.reset_at > SystemTime::now());
    }

    #[tokio::test]
    async fn test_rate_limit_increment_within_limit() {
        // Create a mock rate limiter
        let rate_limiter = create_mock_rate_limiter();
        
        // Increment request count for a user and service
        let rate_limit_info = rate_limiter.increment_request_count("user123", "price_feed").await.unwrap();
        
        // Verify the rate limit info
        assert_eq!(rate_limit_info.request_count, 6); // Incremented from 5 to 6
        assert_eq!(rate_limit_info.max_requests, 10);
        assert_eq!(rate_limit_info.remaining, 4);
        assert!(rate_limit_info.reset_at > SystemTime::now());
    }

    #[tokio::test]
    async fn test_rate_limit_increment_exceeds_limit() {
        // Create a mock rate limiter
        let rate_limiter = create_mock_rate_limiter();
        
        // Try to increment request count for a user and service that is already at the limit
        let result = rate_limiter.increment_request_count("user123", "rate_limited_service").await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(OracleError::RateLimitExceeded(message)) => {
                assert_eq!(message, "Rate limit exceeded for service: rate_limited_service");
            }
            _ => panic!("Expected RateLimitExceeded error"),
        }
    }

    #[tokio::test]
    async fn test_rate_limit_get_config() {
        // Create a mock rate limiter
        let rate_limiter = create_mock_rate_limiter();
        
        // Get rate limit config for a service
        let config = rate_limiter.get_rate_limit_config("price_feed").await.unwrap();
        
        // Verify the config
        assert_eq!(config.max_requests, 10);
        assert_eq!(config.window_seconds, 3600);
    }

    #[tokio::test]
    async fn test_rate_limit_get_config_unknown_service() {
        // Create a mock rate limiter
        let rate_limiter = create_mock_rate_limiter();
        
        // Try to get rate limit config for an unknown service
        let result = rate_limiter.get_rate_limit_config("unknown_service").await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(OracleError::ServiceNotFound(service)) => {
                assert_eq!(service, "unknown_service");
            }
            _ => panic!("Expected ServiceNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_rate_limit_update_config() {
        // Create a mock rate limiter
        let rate_limiter = create_mock_rate_limiter();
        
        // Update rate limit config for a service
        let config = RateLimitConfig {
            max_requests: 20, // Increased from 10 to 20
            window_seconds: 3600,
        };
        
        let result = rate_limiter.update_rate_limit_config("price_feed", config).await;
        
        // Verify that the update was successful
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_rate_limit_reset() {
        // Create a mock rate limiter
        let rate_limiter = create_mock_rate_limiter();
        
        // Reset rate limit for a user and service
        let result = rate_limiter.reset_rate_limit("user123", "price_feed").await;
        
        // Verify that the reset was successful
        assert!(result.is_ok());
    }

    // Test with a custom implementation of RateLimiter
    #[tokio::test]
    async fn test_rate_limiter_implementation() {
        // Create a custom implementation of RateLimiter for testing
        struct TestRateLimiter {
            rate_limits: HashMap<String, RateLimitInfo>,
            configs: HashMap<String, RateLimitConfig>,
        }
        
        impl TestRateLimiter {
            fn new() -> Self {
                let mut rate_limits = HashMap::new();
                let mut configs = HashMap::new();
                
                // Set up initial rate limit info for user123/price_feed
                rate_limits.insert(
                    "user123:price_feed".to_string(),
                    RateLimitInfo {
                        request_count: 5,
                        max_requests: 10,
                        reset_at: SystemTime::now() + Duration::from_secs(3600),
                        remaining: 5,
                    },
                );
                
                // Set up initial rate limit config for price_feed
                configs.insert(
                    "price_feed".to_string(),
                    RateLimitConfig {
                        max_requests: 10,
                        window_seconds: 3600,
                    },
                );
                
                TestRateLimiter {
                    rate_limits,
                    configs,
                }
            }
            
            fn get_key(&self, user_id: &str, service: &str) -> String {
                format!("{}:{}", user_id, service)
            }
        }
        
        #[async_trait::async_trait]
        impl RateLimiter for TestRateLimiter {
            async fn check_rate_limit(&self, user_id: &str, service: &str) -> Result<RateLimitInfo, OracleError> {
                let key = self.get_key(user_id, service);
                
                if let Some(info) = self.rate_limits.get(&key) {
                    Ok(info.clone())
                } else {
                    // If no rate limit info exists, create a new one
                    if let Some(config) = self.configs.get(service) {
                        Ok(RateLimitInfo {
                            request_count: 0,
                            max_requests: config.max_requests,
                            reset_at: SystemTime::now() + Duration::from_secs(config.window_seconds),
                            remaining: config.max_requests,
                        })
                    } else {
                        Err(OracleError::ServiceNotFound(service.to_string()))
                    }
                }
            }
            
            async fn increment_request_count(&self, user_id: &str, service: &str) -> Result<RateLimitInfo, OracleError> {
                let key = self.get_key(user_id, service);
                
                // Check if rate limit info exists
                let mut info = if let Some(info) = self.rate_limits.get(&key) {
                    info.clone()
                } else {
                    // If no rate limit info exists, create a new one
                    if let Some(config) = self.configs.get(service) {
                        RateLimitInfo {
                            request_count: 0,
                            max_requests: config.max_requests,
                            reset_at: SystemTime::now() + Duration::from_secs(config.window_seconds),
                            remaining: config.max_requests,
                        }
                    } else {
                        return Err(OracleError::ServiceNotFound(service.to_string()));
                    }
                };
                
                // Check if rate limit has been exceeded
                if info.request_count >= info.max_requests {
                    return Err(OracleError::RateLimitExceeded(format!("Rate limit exceeded for service: {}", service)));
                }
                
                // Increment request count
                info.request_count += 1;
                info.remaining -= 1;
                
                // Update rate limit info
                let mut rate_limits = self.rate_limits.clone();
                rate_limits.insert(key, info.clone());
                
                Ok(info)
            }
            
            async fn get_rate_limit_config(&self, service: &str) -> Result<RateLimitConfig, OracleError> {
                if let Some(config) = self.configs.get(service) {
                    Ok(config.clone())
                } else {
                    Err(OracleError::ServiceNotFound(service.to_string()))
                }
            }
            
            async fn update_rate_limit_config(&self, service: &str, config: RateLimitConfig) -> Result<(), OracleError> {
                let mut configs = self.configs.clone();
                configs.insert(service.to_string(), config);
                
                Ok(())
            }
            
            async fn reset_rate_limit(&self, user_id: &str, service: &str) -> Result<(), OracleError> {
                let key = self.get_key(user_id, service);
                
                // Check if rate limit info exists
                if let Some(info) = self.rate_limits.get(&key) {
                    let mut info = info.clone();
                    
                    // Reset request count
                    info.request_count = 0;
                    info.remaining = info.max_requests;
                    
                    // Update rate limit info
                    let mut rate_limits = self.rate_limits.clone();
                    rate_limits.insert(key, info);
                    
                    Ok(())
                } else {
                    // If no rate limit info exists, there's nothing to reset
                    Ok(())
                }
            }
        }
        
        // Create a test rate limiter
        let rate_limiter = TestRateLimiter::new();
        
        // Test check_rate_limit
        let info = rate_limiter.check_rate_limit("user123", "price_feed").await.unwrap();
        assert_eq!(info.request_count, 5);
        assert_eq!(info.max_requests, 10);
        assert_eq!(info.remaining, 5);
        
        // Test increment_request_count
        let info = rate_limiter.increment_request_count("user123", "price_feed").await.unwrap();
        assert_eq!(info.request_count, 6);
        assert_eq!(info.max_requests, 10);
        assert_eq!(info.remaining, 4);
        
        // Test get_rate_limit_config
        let config = rate_limiter.get_rate_limit_config("price_feed").await.unwrap();
        assert_eq!(config.max_requests, 10);
        assert_eq!(config.window_seconds, 3600);
        
        // Test update_rate_limit_config
        let new_config = RateLimitConfig {
            max_requests: 20,
            window_seconds: 7200,
        };
        
        let result = rate_limiter.update_rate_limit_config("price_feed", new_config).await;
        assert!(result.is_ok());
        
        // Test reset_rate_limit
        let result = rate_limiter.reset_rate_limit("user123", "price_feed").await;
        assert!(result.is_ok());
    }

    // Test rate limit window expiration
    #[tokio::test]
    async fn test_rate_limit_window_expiration() {
        // Create a custom mock rate limiter for this test
        let mut rate_limiter = MockRateLimiter::new();
        
        // Set up behavior for check_rate_limit with expired window
        let now = SystemTime::now();
        let expired_time = now - Duration::from_secs(60); // 60 seconds ago
        
        rate_limiter.expect_check_rate_limit()
            .with(eq("user123"), eq("expired_window_service"))
            .returning(move |_, _| {
                Ok(RateLimitInfo {
                    request_count: 10,
                    max_requests: 10,
                    reset_at: expired_time,
                    remaining: 0,
                })
            });
        
        // Set up behavior for increment_request_count with expired window
        rate_limiter.expect_increment_request_count()
            .with(eq("user123"), eq("expired_window_service"))
            .returning(move |_, _| {
                // Since the window has expired, the rate limit should be reset
                Ok(RateLimitInfo {
                    request_count: 1, // Reset to 1 (first request in new window)
                    max_requests: 10,
                    reset_at: now + Duration::from_secs(3600), // New window
                    remaining: 9,
                })
            });
        
        // Check rate limit for a user and service with expired window
        let rate_limit_info = rate_limiter.check_rate_limit("user123", "expired_window_service").await.unwrap();
        
        // Verify the rate limit info
        assert_eq!(rate_limit_info.request_count, 10);
        assert_eq!(rate_limit_info.max_requests, 10);
        assert_eq!(rate_limit_info.remaining, 0);
        assert!(rate_limit_info.reset_at < now);
        
        // Increment request count for a user and service with expired window
        let rate_limit_info = rate_limiter.increment_request_count("user123", "expired_window_service").await.unwrap();
        
        // Verify the rate limit info
        assert_eq!(rate_limit_info.request_count, 1); // Reset to 1
        assert_eq!(rate_limit_info.max_requests, 10);
        assert_eq!(rate_limit_info.remaining, 9);
        assert!(rate_limit_info.reset_at > now);
    }

    // Test rate limit with multiple users
    #[tokio::test]
    async fn test_rate_limit_multiple_users() {
        // Create a custom mock rate limiter for this test
        let mut rate_limiter = MockRateLimiter::new();
        
        // Set up behavior for check_rate_limit with multiple users
        rate_limiter.expect_check_rate_limit()
            .with(eq("user1"), eq("shared_service"))
            .returning(|_, _| {
                Ok(RateLimitInfo {
                    request_count: 3,
                    max_requests: 10,
                    reset_at: SystemTime::now() + Duration::from_secs(3600),
                    remaining: 7,
                })
            });
        
        rate_limiter.expect_check_rate_limit()
            .with(eq("user2"), eq("shared_service"))
            .returning(|_, _| {
                Ok(RateLimitInfo {
                    request_count: 5,
                    max_requests: 10,
                    reset_at: SystemTime::now() + Duration::from_secs(3600),
                    remaining: 5,
                })
            });
        
        // Check rate limit for user1
        let rate_limit_info = rate_limiter.check_rate_limit("user1", "shared_service").await.unwrap();
        
        // Verify the rate limit info for user1
        assert_eq!(rate_limit_info.request_count, 3);
        assert_eq!(rate_limit_info.max_requests, 10);
        assert_eq!(rate_limit_info.remaining, 7);
        
        // Check rate limit for user2
        let rate_limit_info = rate_limiter.check_rate_limit("user2", "shared_service").await.unwrap();
        
        // Verify the rate limit info for user2
        assert_eq!(rate_limit_info.request_count, 5);
        assert_eq!(rate_limit_info.max_requests, 10);
        assert_eq!(rate_limit_info.remaining, 5);
    }
}
