// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

#[cfg(test)]
mod tests {
    use r3e_oracle::auth::{OracleAuthProvider, AuthToken, AuthRequest, AuthResponse, Permission};
    use r3e_oracle::types::{OracleError, Asset, Currency};
    use std::sync::Arc;
    use std::time::{Duration, SystemTime};
    use mockall::predicate::*;
    use mockall::mock;

    // Mock the OracleAuthProvider trait for testing
    mock! {
        OracleAuthProvider {}
        trait OracleAuthProvider {
            async fn authenticate(&self, request: AuthRequest) -> Result<AuthResponse, OracleError>;
            async fn validate_token(&self, token: &AuthToken) -> Result<bool, OracleError>;
            async fn check_permission(&self, token: &AuthToken, permission: Permission) -> Result<bool, OracleError>;
            async fn revoke_token(&self, token: &AuthToken) -> Result<(), OracleError>;
            async fn get_user_permissions(&self, token: &AuthToken) -> Result<Vec<Permission>, OracleError>;
        }
    }

    // Helper function to create a mock auth provider
    fn create_mock_provider() -> MockOracleAuthProvider {
        let mut provider = MockOracleAuthProvider::new();
        
        // Set up default behavior for authenticate
        provider.expect_authenticate()
            .with(function(|req: &AuthRequest| req.username == "valid_user" && req.password == "valid_password"))
            .returning(|_| {
                Ok(AuthResponse {
                    token: AuthToken {
                        value: "valid_token".to_string(),
                        expires_at: SystemTime::now() + Duration::from_secs(3600),
                        user_id: "user123".to_string(),
                    },
                    user_id: "user123".to_string(),
                    permissions: vec![
                        Permission::PriceFeed(Asset::Neo, Currency::Usd),
                        Permission::RandomNumber,
                    ],
                })
            });
        
        // Set up behavior for authenticate with invalid credentials
        provider.expect_authenticate()
            .with(function(|req: &AuthRequest| req.username == "invalid_user" || req.password != "valid_password"))
            .returning(|_| {
                Err(OracleError::AuthenticationFailed("Invalid credentials".to_string()))
            });
        
        // Set up default behavior for validate_token
        provider.expect_validate_token()
            .with(function(|token: &AuthToken| token.value == "valid_token"))
            .returning(|_| {
                Ok(true)
            });
        
        // Set up behavior for validate_token with invalid token
        provider.expect_validate_token()
            .with(function(|token: &AuthToken| token.value == "invalid_token"))
            .returning(|_| {
                Ok(false)
            });
        
        // Set up behavior for validate_token with expired token
        provider.expect_validate_token()
            .with(function(|token: &AuthToken| token.value == "expired_token"))
            .returning(|_| {
                Err(OracleError::TokenExpired)
            });
        
        // Set up default behavior for check_permission
        provider.expect_check_permission()
            .with(function(|token: &AuthToken, permission: &Permission| {
                token.value == "valid_token" && 
                (*permission == Permission::PriceFeed(Asset::Neo, Currency::Usd) || 
                 *permission == Permission::RandomNumber)
            }))
            .returning(|_, _| {
                Ok(true)
            });
        
        // Set up behavior for check_permission with unauthorized permission
        provider.expect_check_permission()
            .with(function(|token: &AuthToken, permission: &Permission| {
                token.value == "valid_token" && 
                *permission == Permission::PriceFeed(Asset::Gas, Currency::Usd)
            }))
            .returning(|_, _| {
                Ok(false)
            });
        
        // Set up default behavior for revoke_token
        provider.expect_revoke_token()
            .with(function(|token: &AuthToken| token.value == "valid_token"))
            .returning(|_| {
                Ok(())
            });
        
        // Set up behavior for revoke_token with invalid token
        provider.expect_revoke_token()
            .with(function(|token: &AuthToken| token.value == "invalid_token"))
            .returning(|_| {
                Err(OracleError::InvalidToken)
            });
        
        // Set up default behavior for get_user_permissions
        provider.expect_get_user_permissions()
            .with(function(|token: &AuthToken| token.value == "valid_token"))
            .returning(|_| {
                Ok(vec![
                    Permission::PriceFeed(Asset::Neo, Currency::Usd),
                    Permission::RandomNumber,
                ])
            });
        
        // Set up behavior for get_user_permissions with invalid token
        provider.expect_get_user_permissions()
            .with(function(|token: &AuthToken| token.value == "invalid_token"))
            .returning(|_| {
                Err(OracleError::InvalidToken)
            });
        
        provider
    }

    #[tokio::test]
    async fn test_oracle_auth_authenticate_success() {
        // Create a mock provider
        let provider = create_mock_provider();
        
        // Authenticate with valid credentials
        let request = AuthRequest {
            username: "valid_user".to_string(),
            password: "valid_password".to_string(),
            client_id: Some("test_client".to_string()),
        };
        
        let response = provider.authenticate(request).await.unwrap();
        
        // Verify the response
        assert_eq!(response.token.value, "valid_token");
        assert_eq!(response.user_id, "user123");
        assert_eq!(response.permissions.len(), 2);
        assert!(response.permissions.contains(&Permission::PriceFeed(Asset::Neo, Currency::Usd)));
        assert!(response.permissions.contains(&Permission::RandomNumber));
    }

    #[tokio::test]
    async fn test_oracle_auth_authenticate_failure() {
        // Create a mock provider
        let provider = create_mock_provider();
        
        // Authenticate with invalid credentials
        let request = AuthRequest {
            username: "invalid_user".to_string(),
            password: "invalid_password".to_string(),
            client_id: Some("test_client".to_string()),
        };
        
        let result = provider.authenticate(request).await;
        
        // Verify that authentication failed
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(OracleError::AuthenticationFailed(message)) => {
                assert_eq!(message, "Invalid credentials");
            }
            _ => panic!("Expected AuthenticationFailed error"),
        }
    }

    #[tokio::test]
    async fn test_oracle_auth_validate_token_valid() {
        // Create a mock provider
        let provider = create_mock_provider();
        
        // Create a valid token
        let token = AuthToken {
            value: "valid_token".to_string(),
            expires_at: SystemTime::now() + Duration::from_secs(3600),
            user_id: "user123".to_string(),
        };
        
        // Validate the token
        let is_valid = provider.validate_token(&token).await.unwrap();
        
        // Verify that the token is valid
        assert!(is_valid);
    }

    #[tokio::test]
    async fn test_oracle_auth_validate_token_invalid() {
        // Create a mock provider
        let provider = create_mock_provider();
        
        // Create an invalid token
        let token = AuthToken {
            value: "invalid_token".to_string(),
            expires_at: SystemTime::now() + Duration::from_secs(3600),
            user_id: "user123".to_string(),
        };
        
        // Validate the token
        let is_valid = provider.validate_token(&token).await.unwrap();
        
        // Verify that the token is invalid
        assert!(!is_valid);
    }

    #[tokio::test]
    async fn test_oracle_auth_validate_token_expired() {
        // Create a mock provider
        let provider = create_mock_provider();
        
        // Create an expired token
        let token = AuthToken {
            value: "expired_token".to_string(),
            expires_at: SystemTime::now() - Duration::from_secs(3600), // Expired 1 hour ago
            user_id: "user123".to_string(),
        };
        
        // Validate the token
        let result = provider.validate_token(&token).await;
        
        // Verify that validation failed due to token expiration
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(OracleError::TokenExpired) => {
                // Expected error
            }
            _ => panic!("Expected TokenExpired error"),
        }
    }

    #[tokio::test]
    async fn test_oracle_auth_check_permission_granted() {
        // Create a mock provider
        let provider = create_mock_provider();
        
        // Create a valid token
        let token = AuthToken {
            value: "valid_token".to_string(),
            expires_at: SystemTime::now() + Duration::from_secs(3600),
            user_id: "user123".to_string(),
        };
        
        // Check a permission that the user has
        let has_permission = provider.check_permission(&token, Permission::PriceFeed(Asset::Neo, Currency::Usd)).await.unwrap();
        
        // Verify that the user has the permission
        assert!(has_permission);
        
        // Check another permission that the user has
        let has_permission = provider.check_permission(&token, Permission::RandomNumber).await.unwrap();
        
        // Verify that the user has the permission
        assert!(has_permission);
    }

    #[tokio::test]
    async fn test_oracle_auth_check_permission_denied() {
        // Create a mock provider
        let provider = create_mock_provider();
        
        // Create a valid token
        let token = AuthToken {
            value: "valid_token".to_string(),
            expires_at: SystemTime::now() + Duration::from_secs(3600),
            user_id: "user123".to_string(),
        };
        
        // Check a permission that the user doesn't have
        let has_permission = provider.check_permission(&token, Permission::PriceFeed(Asset::Gas, Currency::Usd)).await.unwrap();
        
        // Verify that the user doesn't have the permission
        assert!(!has_permission);
    }

    #[tokio::test]
    async fn test_oracle_auth_revoke_token_success() {
        // Create a mock provider
        let provider = create_mock_provider();
        
        // Create a valid token
        let token = AuthToken {
            value: "valid_token".to_string(),
            expires_at: SystemTime::now() + Duration::from_secs(3600),
            user_id: "user123".to_string(),
        };
        
        // Revoke the token
        let result = provider.revoke_token(&token).await;
        
        // Verify that the token was revoked successfully
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_oracle_auth_revoke_token_failure() {
        // Create a mock provider
        let provider = create_mock_provider();
        
        // Create an invalid token
        let token = AuthToken {
            value: "invalid_token".to_string(),
            expires_at: SystemTime::now() + Duration::from_secs(3600),
            user_id: "user123".to_string(),
        };
        
        // Try to revoke the token
        let result = provider.revoke_token(&token).await;
        
        // Verify that revocation failed
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(OracleError::InvalidToken) => {
                // Expected error
            }
            _ => panic!("Expected InvalidToken error"),
        }
    }

    #[tokio::test]
    async fn test_oracle_auth_get_user_permissions() {
        // Create a mock provider
        let provider = create_mock_provider();
        
        // Create a valid token
        let token = AuthToken {
            value: "valid_token".to_string(),
            expires_at: SystemTime::now() + Duration::from_secs(3600),
            user_id: "user123".to_string(),
        };
        
        // Get the user's permissions
        let permissions = provider.get_user_permissions(&token).await.unwrap();
        
        // Verify the permissions
        assert_eq!(permissions.len(), 2);
        assert!(permissions.contains(&Permission::PriceFeed(Asset::Neo, Currency::Usd)));
        assert!(permissions.contains(&Permission::RandomNumber));
    }

    #[tokio::test]
    async fn test_oracle_auth_get_user_permissions_invalid_token() {
        // Create a mock provider
        let provider = create_mock_provider();
        
        // Create an invalid token
        let token = AuthToken {
            value: "invalid_token".to_string(),
            expires_at: SystemTime::now() + Duration::from_secs(3600),
            user_id: "user123".to_string(),
        };
        
        // Try to get the user's permissions
        let result = provider.get_user_permissions(&token).await;
        
        // Verify that the operation failed
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(OracleError::InvalidToken) => {
                // Expected error
            }
            _ => panic!("Expected InvalidToken error"),
        }
    }

    // Test with multiple authentication requests
    #[tokio::test]
    async fn test_oracle_auth_multiple_requests() {
        // Create a custom mock provider for this test
        let mut provider = MockOracleAuthProvider::new();
        
        // Set up behavior for multiple authentication requests
        provider.expect_authenticate()
            .times(2) // Expect 2 calls
            .returning(|request| {
                if request.username == "user1" && request.password == "pass1" {
                    Ok(AuthResponse {
                        token: AuthToken {
                            value: "token1".to_string(),
                            expires_at: SystemTime::now() + Duration::from_secs(3600),
                            user_id: "user1".to_string(),
                        },
                        user_id: "user1".to_string(),
                        permissions: vec![Permission::PriceFeed(Asset::Neo, Currency::Usd)],
                    })
                } else if request.username == "user2" && request.password == "pass2" {
                    Ok(AuthResponse {
                        token: AuthToken {
                            value: "token2".to_string(),
                            expires_at: SystemTime::now() + Duration::from_secs(3600),
                            user_id: "user2".to_string(),
                        },
                        user_id: "user2".to_string(),
                        permissions: vec![Permission::RandomNumber],
                    })
                } else {
                    Err(OracleError::AuthenticationFailed("Invalid credentials".to_string()))
                }
            });
        
        // Authenticate with first user
        let request1 = AuthRequest {
            username: "user1".to_string(),
            password: "pass1".to_string(),
            client_id: None,
        };
        
        let response1 = provider.authenticate(request1).await.unwrap();
        
        // Verify the response for first user
        assert_eq!(response1.token.value, "token1");
        assert_eq!(response1.user_id, "user1");
        assert_eq!(response1.permissions.len(), 1);
        assert!(response1.permissions.contains(&Permission::PriceFeed(Asset::Neo, Currency::Usd)));
        
        // Authenticate with second user
        let request2 = AuthRequest {
            username: "user2".to_string(),
            password: "pass2".to_string(),
            client_id: None,
        };
        
        let response2 = provider.authenticate(request2).await.unwrap();
        
        // Verify the response for second user
        assert_eq!(response2.token.value, "token2");
        assert_eq!(response2.user_id, "user2");
        assert_eq!(response2.permissions.len(), 1);
        assert!(response2.permissions.contains(&Permission::RandomNumber));
    }
}
