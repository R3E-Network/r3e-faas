// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

#[cfg(test)]
mod tests {
    use r3e_oracle::provider::random::{RandomNumberProvider, RandomData};
    use r3e_oracle::types::{RandomType, RandomRange, OracleError};
    use std::sync::Arc;
    use std::time::{Duration, SystemTime};
    use mockall::predicate::*;
    use mockall::mock;

    // Mock the RandomNumberProvider trait for testing
    mock! {
        RandomNumberProvider {}
        trait RandomNumberProvider {
            async fn generate_random(&self, random_type: RandomType, range: Option<RandomRange>) -> Result<RandomData, OracleError>;
            async fn verify_random(&self, random_data: &RandomData) -> Result<bool, OracleError>;
            async fn get_supported_random_types(&self) -> Result<Vec<RandomType>, OracleError>;
        }
    }

    // Helper function to create a mock random number provider
    fn create_mock_provider() -> MockRandomNumberProvider {
        let mut provider = MockRandomNumberProvider::new();
        
        // Set up default behavior for generate_random with Integer type
        provider.expect_generate_random()
            .with(eq(RandomType::Integer), eq(Some(RandomRange::Integer { min: 1, max: 100 })))
            .returning(|_, _| {
                Ok(RandomData {
                    value: "42".to_string(),
                    random_type: RandomType::Integer,
                    timestamp: SystemTime::now(),
                    source: "mock".to_string(),
                    proof: Some("mock_proof".to_string()),
                })
            });
        
        // Set up default behavior for generate_random with Float type
        provider.expect_generate_random()
            .with(eq(RandomType::Float), eq(Some(RandomRange::Float { min: 0.0, max: 1.0 })))
            .returning(|_, _| {
                Ok(RandomData {
                    value: "0.7531".to_string(),
                    random_type: RandomType::Float,
                    timestamp: SystemTime::now(),
                    source: "mock".to_string(),
                    proof: Some("mock_proof".to_string()),
                })
            });
        
        // Set up default behavior for generate_random with Bytes type
        provider.expect_generate_random()
            .with(eq(RandomType::Bytes), eq(Some(RandomRange::Bytes { length: 32 })))
            .returning(|_, _| {
                Ok(RandomData {
                    value: "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2".to_string(),
                    random_type: RandomType::Bytes,
                    timestamp: SystemTime::now(),
                    source: "mock".to_string(),
                    proof: Some("mock_proof".to_string()),
                })
            });
        
        // Set up default behavior for generate_random with UUID type
        provider.expect_generate_random()
            .with(eq(RandomType::Uuid), eq(None))
            .returning(|_, _| {
                Ok(RandomData {
                    value: "550e8400-e29b-41d4-a716-446655440000".to_string(),
                    random_type: RandomType::Uuid,
                    timestamp: SystemTime::now(),
                    source: "mock".to_string(),
                    proof: Some("mock_proof".to_string()),
                })
            });
        
        // Set up default behavior for generate_random with error
        provider.expect_generate_random()
            .with(eq(RandomType::Custom("INVALID".to_string())), eq(None))
            .returning(|_, _| {
                Err(OracleError::RandomTypeNotSupported("INVALID".to_string()))
            });
        
        // Set up default behavior for verify_random
        provider.expect_verify_random()
            .returning(|random_data| {
                // Verify that the random data has a proof
                if random_data.proof.is_some() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            });
        
        // Set up default behavior for get_supported_random_types
        provider.expect_get_supported_random_types()
            .returning(|| {
                Ok(vec![
                    RandomType::Integer,
                    RandomType::Float,
                    RandomType::Bytes,
                    RandomType::Uuid,
                ])
            });
        
        provider
    }

    #[tokio::test]
    async fn test_random_number_generate_integer() {
        // Create a mock provider
        let provider = create_mock_provider();
        
        // Generate a random integer
        let random_data = provider.generate_random(
            RandomType::Integer,
            Some(RandomRange::Integer { min: 1, max: 100 })
        ).await.unwrap();
        
        // Verify the random data
        assert_eq!(random_data.random_type, RandomType::Integer);
        assert_eq!(random_data.value, "42");
        assert_eq!(random_data.source, "mock");
        assert!(random_data.proof.is_some());
        
        // Verify that the random data can be verified
        let is_valid = provider.verify_random(&random_data).await.unwrap();
        assert!(is_valid);
    }

    #[tokio::test]
    async fn test_random_number_generate_float() {
        // Create a mock provider
        let provider = create_mock_provider();
        
        // Generate a random float
        let random_data = provider.generate_random(
            RandomType::Float,
            Some(RandomRange::Float { min: 0.0, max: 1.0 })
        ).await.unwrap();
        
        // Verify the random data
        assert_eq!(random_data.random_type, RandomType::Float);
        assert_eq!(random_data.value, "0.7531");
        assert_eq!(random_data.source, "mock");
        assert!(random_data.proof.is_some());
        
        // Verify that the random data can be verified
        let is_valid = provider.verify_random(&random_data).await.unwrap();
        assert!(is_valid);
    }

    #[tokio::test]
    async fn test_random_number_generate_bytes() {
        // Create a mock provider
        let provider = create_mock_provider();
        
        // Generate random bytes
        let random_data = provider.generate_random(
            RandomType::Bytes,
            Some(RandomRange::Bytes { length: 32 })
        ).await.unwrap();
        
        // Verify the random data
        assert_eq!(random_data.random_type, RandomType::Bytes);
        assert_eq!(random_data.value.len(), 64); // 32 bytes = 64 hex characters
        assert_eq!(random_data.source, "mock");
        assert!(random_data.proof.is_some());
        
        // Verify that the random data can be verified
        let is_valid = provider.verify_random(&random_data).await.unwrap();
        assert!(is_valid);
    }

    #[tokio::test]
    async fn test_random_number_generate_uuid() {
        // Create a mock provider
        let provider = create_mock_provider();
        
        // Generate a random UUID
        let random_data = provider.generate_random(
            RandomType::Uuid,
            None
        ).await.unwrap();
        
        // Verify the random data
        assert_eq!(random_data.random_type, RandomType::Uuid);
        assert_eq!(random_data.value, "550e8400-e29b-41d4-a716-446655440000");
        assert_eq!(random_data.source, "mock");
        assert!(random_data.proof.is_some());
        
        // Verify that the random data can be verified
        let is_valid = provider.verify_random(&random_data).await.unwrap();
        assert!(is_valid);
    }

    #[tokio::test]
    async fn test_random_number_generate_error() {
        // Create a mock provider
        let provider = create_mock_provider();
        
        // Try to generate a random number with an invalid type
        let result = provider.generate_random(
            RandomType::Custom("INVALID".to_string()),
            None
        ).await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(OracleError::RandomTypeNotSupported(random_type)) => {
                assert_eq!(random_type, "INVALID");
            }
            _ => panic!("Expected RandomTypeNotSupported error"),
        }
    }

    #[tokio::test]
    async fn test_random_number_verify_invalid() {
        // Create a mock provider
        let provider = create_mock_provider();
        
        // Create a random data without a proof
        let random_data = RandomData {
            value: "42".to_string(),
            random_type: RandomType::Integer,
            timestamp: SystemTime::now(),
            source: "mock".to_string(),
            proof: None, // No proof
        };
        
        // Verify that the random data is invalid
        let is_valid = provider.verify_random(&random_data).await.unwrap();
        assert!(!is_valid);
    }

    #[tokio::test]
    async fn test_random_number_get_supported_types() {
        // Create a mock provider
        let provider = create_mock_provider();
        
        // Get the supported random types
        let random_types = provider.get_supported_random_types().await.unwrap();
        
        // Verify the random types
        assert_eq!(random_types.len(), 4);
        assert!(random_types.contains(&RandomType::Integer));
        assert!(random_types.contains(&RandomType::Float));
        assert!(random_types.contains(&RandomType::Bytes));
        assert!(random_types.contains(&RandomType::Uuid));
    }

    // Test with multiple random number requests
    #[tokio::test]
    async fn test_random_number_multiple_requests() {
        // Create a custom mock provider for this test
        let mut provider = MockRandomNumberProvider::new();
        
        // Set up behavior for multiple random number requests
        provider.expect_generate_random()
            .times(3) // Expect 3 calls
            .returning(|random_type, range| {
                match random_type {
                    RandomType::Integer => Ok(RandomData {
                        value: "42".to_string(),
                        random_type: RandomType::Integer,
                        timestamp: SystemTime::now(),
                        source: "mock".to_string(),
                        proof: Some("mock_proof".to_string()),
                    }),
                    RandomType::Float => Ok(RandomData {
                        value: "0.7531".to_string(),
                        random_type: RandomType::Float,
                        timestamp: SystemTime::now(),
                        source: "mock".to_string(),
                        proof: Some("mock_proof".to_string()),
                    }),
                    RandomType::Bytes => Ok(RandomData {
                        value: "a1b2c3d4e5f6".to_string(),
                        random_type: RandomType::Bytes,
                        timestamp: SystemTime::now(),
                        source: "mock".to_string(),
                        proof: Some("mock_proof".to_string()),
                    }),
                    _ => Err(OracleError::RandomTypeNotSupported(format!("{:?}", random_type))),
                }
            });
        
        // Make multiple random number requests
        let integer_data = provider.generate_random(RandomType::Integer, None).await.unwrap();
        let float_data = provider.generate_random(RandomType::Float, None).await.unwrap();
        let bytes_data = provider.generate_random(RandomType::Bytes, None).await.unwrap();
        
        // Verify the random data
        assert_eq!(integer_data.random_type, RandomType::Integer);
        assert_eq!(integer_data.value, "42");
        
        assert_eq!(float_data.random_type, RandomType::Float);
        assert_eq!(float_data.value, "0.7531");
        
        assert_eq!(bytes_data.random_type, RandomType::Bytes);
        assert_eq!(bytes_data.value, "a1b2c3d4e5f6");
    }

    // Test error handling for different error types
    #[tokio::test]
    async fn test_random_number_error_handling() {
        // Create a custom mock provider for this test
        let mut provider = MockRandomNumberProvider::new();
        
        // Set up behavior for different error types
        provider.expect_generate_random()
            .with(eq(RandomType::Custom("NETWORK_ERROR".to_string())), eq(None))
            .returning(|_, _| {
                Err(OracleError::NetworkError("Failed to connect to random source".to_string()))
            });
        
        provider.expect_generate_random()
            .with(eq(RandomType::Custom("TIMEOUT".to_string())), eq(None))
            .returning(|_, _| {
                Err(OracleError::Timeout("Request timed out".to_string()))
            });
        
        provider.expect_generate_random()
            .with(eq(RandomType::Custom("RATE_LIMIT".to_string())), eq(None))
            .returning(|_, _| {
                Err(OracleError::RateLimitExceeded("Too many requests".to_string()))
            });
        
        // Test network error
        let result = provider.generate_random(RandomType::Custom("NETWORK_ERROR".to_string()), None).await;
        assert!(matches!(result, Err(OracleError::NetworkError(_))));
        
        // Test timeout error
        let result = provider.generate_random(RandomType::Custom("TIMEOUT".to_string()), None).await;
        assert!(matches!(result, Err(OracleError::Timeout(_))));
        
        // Test rate limit error
        let result = provider.generate_random(RandomType::Custom("RATE_LIMIT".to_string()), None).await;
        assert!(matches!(result, Err(OracleError::RateLimitExceeded(_))));
    }
}
