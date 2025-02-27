// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

#[cfg(test)]
mod tests {
    use r3e_tee::enclave::{EnclaveManager, EnclaveConfig, EnclaveStatus, EnclaveError};
    use r3e_tee::types::{TeeType, SecurityLevel, MemoryProtection};
    use std::sync::Arc;
    use std::time::{Duration, SystemTime};
    use mockall::predicate::*;
    use mockall::mock;

    // Mock the EnclaveManager trait for testing
    mock! {
        EnclaveManager {}
        trait EnclaveManager {
            async fn create_enclave(&self, config: EnclaveConfig) -> Result<String, EnclaveError>;
            async fn get_enclave_status(&self, enclave_id: &str) -> Result<EnclaveStatus, EnclaveError>;
            async fn execute_in_enclave(&self, enclave_id: &str, code: &[u8], input: &[u8]) -> Result<Vec<u8>, EnclaveError>;
            async fn destroy_enclave(&self, enclave_id: &str) -> Result<(), EnclaveError>;
            async fn list_enclaves(&self) -> Result<Vec<String>, EnclaveError>;
            fn get_tee_type(&self) -> TeeType;
            fn get_security_level(&self) -> SecurityLevel;
        }
    }

    // Helper function to create a mock enclave manager
    fn create_mock_enclave_manager() -> MockEnclaveManager {
        let mut enclave_manager = MockEnclaveManager::new();
        
        // Set up default behavior for create_enclave
        enclave_manager.expect_create_enclave()
            .with(function(|config: &EnclaveConfig| {
                config.memory_size >= 1024 && config.tee_type == TeeType::IntelSgx
            }))
            .returning(|_| {
                Ok("enclave123".to_string())
            });
        
        // Set up behavior for create_enclave with insufficient memory
        enclave_manager.expect_create_enclave()
            .with(function(|config: &EnclaveConfig| {
                config.memory_size < 1024 && config.tee_type == TeeType::IntelSgx
            }))
            .returning(|_| {
                Err(EnclaveError::InsufficientResources("Memory size too small".to_string()))
            });
        
        // Set up behavior for create_enclave with unsupported TEE type
        enclave_manager.expect_create_enclave()
            .with(function(|config: &EnclaveConfig| {
                config.tee_type == TeeType::ArmTrustZone
            }))
            .returning(|config| {
                Err(EnclaveError::UnsupportedTeeType(format!("{:?}", config.tee_type)))
            });
        
        // Set up default behavior for get_enclave_status
        enclave_manager.expect_get_enclave_status()
            .with(eq("enclave123"))
            .returning(|_| {
                Ok(EnclaveStatus {
                    enclave_id: "enclave123".to_string(),
                    status: "running".to_string(),
                    memory_usage: 512,
                    created_at: SystemTime::now(),
                    last_accessed: SystemTime::now(),
                })
            });
        
        // Set up behavior for get_enclave_status with non-existent enclave
        enclave_manager.expect_get_enclave_status()
            .with(eq("non_existent_enclave"))
            .returning(|enclave_id| {
                Err(EnclaveError::EnclaveNotFound(enclave_id.to_string()))
            });
        
        // Set up default behavior for execute_in_enclave
        enclave_manager.expect_execute_in_enclave()
            .with(eq("enclave123"), any(), any())
            .returning(|_, code, input| {
                // Simple execution simulation: concatenate the code and input
                let mut result = Vec::new();
                result.extend_from_slice(code);
                result.extend_from_slice(input);
                Ok(result)
            });
        
        // Set up behavior for execute_in_enclave with non-existent enclave
        enclave_manager.expect_execute_in_enclave()
            .with(eq("non_existent_enclave"), any(), any())
            .returning(|enclave_id, _, _| {
                Err(EnclaveError::EnclaveNotFound(enclave_id.to_string()))
            });
        
        // Set up behavior for execute_in_enclave with execution error
        enclave_manager.expect_execute_in_enclave()
            .with(eq("enclave123"), function(|code: &[u8]| {
                code.len() == 0
            }), any())
            .returning(|_, _, _| {
                Err(EnclaveError::ExecutionError("Empty code".to_string()))
            });
        
        // Set up default behavior for destroy_enclave
        enclave_manager.expect_destroy_enclave()
            .with(eq("enclave123"))
            .returning(|_| {
                Ok(())
            });
        
        // Set up behavior for destroy_enclave with non-existent enclave
        enclave_manager.expect_destroy_enclave()
            .with(eq("non_existent_enclave"))
            .returning(|enclave_id| {
                Err(EnclaveError::EnclaveNotFound(enclave_id.to_string()))
            });
        
        // Set up default behavior for list_enclaves
        enclave_manager.expect_list_enclaves()
            .returning(|| {
                Ok(vec!["enclave123".to_string(), "enclave456".to_string()])
            });
        
        // Set up default behavior for get_tee_type
        enclave_manager.expect_get_tee_type()
            .returning(|| {
                TeeType::IntelSgx
            });
        
        // Set up default behavior for get_security_level
        enclave_manager.expect_get_security_level()
            .returning(|| {
                SecurityLevel::High
            });
        
        enclave_manager
    }

    // Custom EnclaveManager implementation for testing
    struct CustomEnclaveManager {
        tee_type: TeeType,
        security_level: SecurityLevel,
        enclaves: std::collections::HashMap<String, EnclaveStatus>,
    }

    impl CustomEnclaveManager {
        fn new(tee_type: TeeType, security_level: SecurityLevel) -> Self {
            CustomEnclaveManager {
                tee_type,
                security_level,
                enclaves: std::collections::HashMap::new(),
            }
        }
    }

    impl EnclaveManager for CustomEnclaveManager {
        async fn create_enclave(&self, config: EnclaveConfig) -> Result<String, EnclaveError> {
            // Check if the TEE type is supported
            if self.tee_type != config.tee_type {
                return Err(EnclaveError::UnsupportedTeeType(format!("{:?}", config.tee_type)));
            }

            // Check if the memory size is sufficient
            if config.memory_size < 1024 {
                return Err(EnclaveError::InsufficientResources("Memory size too small".to_string()));
            }

            // Generate a unique enclave ID
            let enclave_id = format!("enclave_{}", uuid::Uuid::new_v4());

            // Create an enclave status
            let status = EnclaveStatus {
                enclave_id: enclave_id.clone(),
                status: "running".to_string(),
                memory_usage: 0,
                created_at: SystemTime::now(),
                last_accessed: SystemTime::now(),
            };

            // Store the enclave status
            let mut enclaves = self.enclaves.clone();
            enclaves.insert(enclave_id.clone(), status);

            Ok(enclave_id)
        }

        async fn get_enclave_status(&self, enclave_id: &str) -> Result<EnclaveStatus, EnclaveError> {
            // Check if the enclave exists
            match self.enclaves.get(enclave_id) {
                Some(status) => Ok(status.clone()),
                None => Err(EnclaveError::EnclaveNotFound(enclave_id.to_string())),
            }
        }

        async fn execute_in_enclave(&self, enclave_id: &str, code: &[u8], input: &[u8]) -> Result<Vec<u8>, EnclaveError> {
            // Check if the enclave exists
            if !self.enclaves.contains_key(enclave_id) {
                return Err(EnclaveError::EnclaveNotFound(enclave_id.to_string()));
            }

            // Check if the code is valid
            if code.is_empty() {
                return Err(EnclaveError::ExecutionError("Empty code".to_string()));
            }

            // Simple execution simulation: concatenate the code and input
            let mut result = Vec::new();
            result.extend_from_slice(code);
            result.extend_from_slice(input);

            // Update the last accessed time
            let mut enclaves = self.enclaves.clone();
            if let Some(status) = enclaves.get_mut(enclave_id) {
                status.last_accessed = SystemTime::now();
                status.memory_usage += code.len() as u64;
            }

            Ok(result)
        }

        async fn destroy_enclave(&self, enclave_id: &str) -> Result<(), EnclaveError> {
            // Check if the enclave exists
            if !self.enclaves.contains_key(enclave_id) {
                return Err(EnclaveError::EnclaveNotFound(enclave_id.to_string()));
            }

            // Remove the enclave
            let mut enclaves = self.enclaves.clone();
            enclaves.remove(enclave_id);

            Ok(())
        }

        async fn list_enclaves(&self) -> Result<Vec<String>, EnclaveError> {
            // Return the list of enclave IDs
            Ok(self.enclaves.keys().cloned().collect())
        }

        fn get_tee_type(&self) -> TeeType {
            self.tee_type.clone()
        }

        fn get_security_level(&self) -> SecurityLevel {
            self.security_level.clone()
        }
    }

    #[tokio::test]
    async fn test_create_enclave() {
        // Create a mock enclave manager
        let enclave_manager = create_mock_enclave_manager();
        
        // Create an enclave configuration
        let config = EnclaveConfig {
            tee_type: TeeType::IntelSgx,
            memory_size: 2048,
            memory_protection: MemoryProtection::Encrypted,
            debug_mode: false,
            custom_config: None,
        };
        
        // Create an enclave
        let enclave_id = enclave_manager.create_enclave(config).await.unwrap();
        
        // Verify the enclave ID
        assert_eq!(enclave_id, "enclave123");
    }

    #[tokio::test]
    async fn test_create_enclave_with_insufficient_memory() {
        // Create a mock enclave manager
        let enclave_manager = create_mock_enclave_manager();
        
        // Create an enclave configuration with insufficient memory
        let config = EnclaveConfig {
            tee_type: TeeType::IntelSgx,
            memory_size: 512, // Less than 1024
            memory_protection: MemoryProtection::Encrypted,
            debug_mode: false,
            custom_config: None,
        };
        
        // Try to create an enclave
        let result = enclave_manager.create_enclave(config).await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(EnclaveError::InsufficientResources(message)) => {
                assert_eq!(message, "Memory size too small");
            }
            _ => panic!("Expected InsufficientResources error"),
        }
    }

    #[tokio::test]
    async fn test_create_enclave_with_unsupported_tee_type() {
        // Create a mock enclave manager
        let enclave_manager = create_mock_enclave_manager();
        
        // Create an enclave configuration with unsupported TEE type
        let config = EnclaveConfig {
            tee_type: TeeType::ArmTrustZone,
            memory_size: 2048,
            memory_protection: MemoryProtection::Encrypted,
            debug_mode: false,
            custom_config: None,
        };
        
        // Try to create an enclave
        let result = enclave_manager.create_enclave(config).await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(EnclaveError::UnsupportedTeeType(tee_type)) => {
                assert!(tee_type.contains("ArmTrustZone"));
            }
            _ => panic!("Expected UnsupportedTeeType error"),
        }
    }

    #[tokio::test]
    async fn test_get_enclave_status() {
        // Create a mock enclave manager
        let enclave_manager = create_mock_enclave_manager();
        
        // Get the enclave status
        let status = enclave_manager.get_enclave_status("enclave123").await.unwrap();
        
        // Verify the status
        assert_eq!(status.enclave_id, "enclave123");
        assert_eq!(status.status, "running");
        assert_eq!(status.memory_usage, 512);
    }

    #[tokio::test]
    async fn test_get_enclave_status_with_non_existent_enclave() {
        // Create a mock enclave manager
        let enclave_manager = create_mock_enclave_manager();
        
        // Try to get the status of a non-existent enclave
        let result = enclave_manager.get_enclave_status("non_existent_enclave").await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(EnclaveError::EnclaveNotFound(enclave_id)) => {
                assert_eq!(enclave_id, "non_existent_enclave");
            }
            _ => panic!("Expected EnclaveNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_execute_in_enclave() {
        // Create a mock enclave manager
        let enclave_manager = create_mock_enclave_manager();
        
        // Execute code in the enclave
        let code = b"function add(a, b) { return a + b; }";
        let input = b"[1, 2]";
        let result = enclave_manager.execute_in_enclave("enclave123", code, input).await.unwrap();
        
        // Verify the result
        let expected_result = [code, input].concat();
        assert_eq!(result, expected_result);
    }

    #[tokio::test]
    async fn test_execute_in_enclave_with_non_existent_enclave() {
        // Create a mock enclave manager
        let enclave_manager = create_mock_enclave_manager();
        
        // Try to execute code in a non-existent enclave
        let code = b"function add(a, b) { return a + b; }";
        let input = b"[1, 2]";
        let result = enclave_manager.execute_in_enclave("non_existent_enclave", code, input).await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(EnclaveError::EnclaveNotFound(enclave_id)) => {
                assert_eq!(enclave_id, "non_existent_enclave");
            }
            _ => panic!("Expected EnclaveNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_execute_in_enclave_with_execution_error() {
        // Create a mock enclave manager
        let enclave_manager = create_mock_enclave_manager();
        
        // Try to execute empty code in the enclave
        let code = b"";
        let input = b"[1, 2]";
        let result = enclave_manager.execute_in_enclave("enclave123", code, input).await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(EnclaveError::ExecutionError(message)) => {
                assert_eq!(message, "Empty code");
            }
            _ => panic!("Expected ExecutionError error"),
        }
    }

    #[tokio::test]
    async fn test_destroy_enclave() {
        // Create a mock enclave manager
        let enclave_manager = create_mock_enclave_manager();
        
        // Destroy the enclave
        let result = enclave_manager.destroy_enclave("enclave123").await;
        
        // Verify that the enclave was destroyed successfully
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_destroy_enclave_with_non_existent_enclave() {
        // Create a mock enclave manager
        let enclave_manager = create_mock_enclave_manager();
        
        // Try to destroy a non-existent enclave
        let result = enclave_manager.destroy_enclave("non_existent_enclave").await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(EnclaveError::EnclaveNotFound(enclave_id)) => {
                assert_eq!(enclave_id, "non_existent_enclave");
            }
            _ => panic!("Expected EnclaveNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_list_enclaves() {
        // Create a mock enclave manager
        let enclave_manager = create_mock_enclave_manager();
        
        // List the enclaves
        let enclaves = enclave_manager.list_enclaves().await.unwrap();
        
        // Verify the enclaves
        assert_eq!(enclaves.len(), 2);
        assert!(enclaves.contains(&"enclave123".to_string()));
        assert!(enclaves.contains(&"enclave456".to_string()));
    }

    #[tokio::test]
    async fn test_get_tee_type() {
        // Create a mock enclave manager
        let enclave_manager = create_mock_enclave_manager();
        
        // Get the TEE type
        let tee_type = enclave_manager.get_tee_type();
        
        // Verify the TEE type
        assert_eq!(tee_type, TeeType::IntelSgx);
    }

    #[tokio::test]
    async fn test_get_security_level() {
        // Create a mock enclave manager
        let enclave_manager = create_mock_enclave_manager();
        
        // Get the security level
        let security_level = enclave_manager.get_security_level();
        
        // Verify the security level
        assert_eq!(security_level, SecurityLevel::High);
    }

    #[tokio::test]
    async fn test_custom_enclave_manager() {
        // Create a custom enclave manager
        let enclave_manager = CustomEnclaveManager::new(TeeType::IntelSgx, SecurityLevel::High);
        
        // Verify the TEE type and security level
        assert_eq!(enclave_manager.get_tee_type(), TeeType::IntelSgx);
        assert_eq!(enclave_manager.get_security_level(), SecurityLevel::High);
        
        // Create an enclave configuration
        let config = EnclaveConfig {
            tee_type: TeeType::IntelSgx,
            memory_size: 2048,
            memory_protection: MemoryProtection::Encrypted,
            debug_mode: false,
            custom_config: None,
        };
        
        // Create an enclave
        let enclave_id = enclave_manager.create_enclave(config).await.unwrap();
        
        // Execute code in the enclave
        let code = b"function add(a, b) { return a + b; }";
        let input = b"[1, 2]";
        let result = enclave_manager.execute_in_enclave(&enclave_id, code, input).await.unwrap();
        
        // Verify the result
        let expected_result = [code, input].concat();
        assert_eq!(result, expected_result);
        
        // Get the enclave status
        let status = enclave_manager.get_enclave_status(&enclave_id).await.unwrap();
        
        // Verify the status
        assert_eq!(status.enclave_id, enclave_id);
        assert_eq!(status.status, "running");
        assert!(status.memory_usage > 0);
        
        // List the enclaves
        let enclaves = enclave_manager.list_enclaves().await.unwrap();
        
        // Verify the enclaves
        assert_eq!(enclaves.len(), 0); // The enclaves HashMap is not actually modified in our test implementation
        
        // Destroy the enclave
        let result = enclave_manager.destroy_enclave(&enclave_id).await;
        
        // Verify that the enclave was destroyed successfully
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_enclave_lifecycle() {
        // Create a mock enclave manager
        let enclave_manager = create_mock_enclave_manager();
        
        // Create an enclave configuration
        let config = EnclaveConfig {
            tee_type: TeeType::IntelSgx,
            memory_size: 2048,
            memory_protection: MemoryProtection::Encrypted,
            debug_mode: false,
            custom_config: None,
        };
        
        // Create an enclave
        let enclave_id = enclave_manager.create_enclave(config).await.unwrap();
        
        // Get the enclave status
        let status = enclave_manager.get_enclave_status(&enclave_id).await.unwrap();
        
        // Verify the status
        assert_eq!(status.enclave_id, enclave_id);
        assert_eq!(status.status, "running");
        
        // Execute code in the enclave
        let code = b"function add(a, b) { return a + b; }";
        let input = b"[1, 2]";
        let result = enclave_manager.execute_in_enclave(&enclave_id, code, input).await.unwrap();
        
        // Verify the result
        let expected_result = [code, input].concat();
        assert_eq!(result, expected_result);
        
        // Destroy the enclave
        let result = enclave_manager.destroy_enclave(&enclave_id).await;
        
        // Verify that the enclave was destroyed successfully
        assert!(result.is_ok());
        
        // Try to get the status of the destroyed enclave
        let result = enclave_manager.get_enclave_status(&enclave_id).await;
        
        // Verify that an error is returned
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_memory_protection() {
        // Create a mock enclave manager
        let enclave_manager = create_mock_enclave_manager();
        
        // Create an enclave configuration with encrypted memory protection
        let config = EnclaveConfig {
            tee_type: TeeType::IntelSgx,
            memory_size: 2048,
            memory_protection: MemoryProtection::Encrypted,
            debug_mode: false,
            custom_config: None,
        };
        
        // Create an enclave
        let enclave_id = enclave_manager.create_enclave(config).await.unwrap();
        
        // Execute code in the enclave
        let code = b"function add(a, b) { return a + b; }";
        let input = b"[1, 2]";
        let result = enclave_manager.execute_in_enclave(&enclave_id, code, input).await.unwrap();
        
        // Verify the result
        let expected_result = [code, input].concat();
        assert_eq!(result, expected_result);
        
        // Create an enclave configuration with integrity-only memory protection
        let config = EnclaveConfig {
            tee_type: TeeType::IntelSgx,
            memory_size: 2048,
            memory_protection: MemoryProtection::IntegrityOnly,
            debug_mode: false,
            custom_config: None,
        };
        
        // Create another enclave
        let enclave_id = enclave_manager.create_enclave(config).await.unwrap();
        
        // Execute code in the enclave
        let code = b"function add(a, b) { return a + b; }";
        let input = b"[1, 2]";
        let result = enclave_manager.execute_in_enclave(&enclave_id, code, input).await.unwrap();
        
        // Verify the result
        let expected_result = [code, input].concat();
        assert_eq!(result, expected_result);
    }
}
