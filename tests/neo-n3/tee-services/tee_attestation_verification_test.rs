// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

#[cfg(test)]
mod tests {
    use r3e_tee::attestation::{AttestationVerifier, AttestationReport, AttestationResult, AttestationError};
    use r3e_tee::types::{TeeType, PlatformConfiguration, SecurityLevel};
    use std::sync::Arc;
    use std::time::{Duration, SystemTime};
    use mockall::predicate::*;
    use mockall::mock;

    // Mock the AttestationVerifier trait for testing
    mock! {
        AttestationVerifier {}
        trait AttestationVerifier {
            fn get_supported_tee_types(&self) -> Vec<TeeType>;
            async fn verify_attestation(&self, report: &AttestationReport) -> Result<AttestationResult, AttestationError>;
            async fn validate_platform_configuration(&self, config: &PlatformConfiguration) -> Result<SecurityLevel, AttestationError>;
            fn get_name(&self) -> &str;
        }
    }

    // Helper function to create a mock attestation verifier
    fn create_mock_verifier() -> MockAttestationVerifier {
        let mut verifier = MockAttestationVerifier::new();
        
        // Set up default behavior for get_supported_tee_types
        verifier.expect_get_supported_tee_types()
            .returning(|| {
                vec![TeeType::IntelSgx, TeeType::AmdSev]
            });
        
        // Set up default behavior for verify_attestation
        verifier.expect_verify_attestation()
            .with(function(|report: &AttestationReport| {
                report.tee_type == TeeType::IntelSgx && report.quote.len() > 0
            }))
            .returning(|report| {
                Ok(AttestationResult {
                    is_valid: true,
                    security_level: SecurityLevel::High,
                    platform_info: Some(PlatformConfiguration {
                        tee_type: report.tee_type.clone(),
                        version: "2.0".to_string(),
                        security_version: 5,
                        attributes: vec!["encrypted_memory".to_string(), "secure_boot".to_string()],
                    }),
                    timestamp: SystemTime::now(),
                })
            });
        
        // Set up behavior for verify_attestation with invalid report
        verifier.expect_verify_attestation()
            .with(function(|report: &AttestationReport| {
                report.tee_type == TeeType::IntelSgx && report.quote.is_empty()
            }))
            .returning(|_| {
                Err(AttestationError::InvalidQuote("Empty quote".to_string()))
            });
        
        // Set up behavior for verify_attestation with unsupported TEE type
        verifier.expect_verify_attestation()
            .with(function(|report: &AttestationReport| {
                report.tee_type == TeeType::ArmTrustZone
            }))
            .returning(|report| {
                Err(AttestationError::UnsupportedTeeType(format!("{:?}", report.tee_type)))
            });
        
        // Set up default behavior for validate_platform_configuration
        verifier.expect_validate_platform_configuration()
            .with(function(|config: &PlatformConfiguration| {
                config.tee_type == TeeType::IntelSgx && config.security_version >= 5
            }))
            .returning(|_| {
                Ok(SecurityLevel::High)
            });
        
        // Set up behavior for validate_platform_configuration with outdated security version
        verifier.expect_validate_platform_configuration()
            .with(function(|config: &PlatformConfiguration| {
                config.tee_type == TeeType::IntelSgx && config.security_version < 5
            }))
            .returning(|_| {
                Ok(SecurityLevel::Medium)
            });
        
        // Set up behavior for validate_platform_configuration with unsupported TEE type
        verifier.expect_validate_platform_configuration()
            .with(function(|config: &PlatformConfiguration| {
                config.tee_type == TeeType::ArmTrustZone
            }))
            .returning(|config| {
                Err(AttestationError::UnsupportedTeeType(format!("{:?}", config.tee_type)))
            });
        
        // Set up default behavior for get_name
        verifier.expect_get_name()
            .returning(|| {
                "Intel SGX Attestation Verifier"
            });
        
        verifier
    }

    #[tokio::test]
    async fn test_get_supported_tee_types() {
        // Create a mock verifier
        let verifier = create_mock_verifier();
        
        // Get the supported TEE types
        let tee_types = verifier.get_supported_tee_types();
        
        // Verify the TEE types
        assert_eq!(tee_types.len(), 2);
        assert!(tee_types.contains(&TeeType::IntelSgx));
        assert!(tee_types.contains(&TeeType::AmdSev));
    }

    #[tokio::test]
    async fn test_verify_valid_attestation() {
        // Create a mock verifier
        let verifier = create_mock_verifier();
        
        // Create a valid attestation report
        let report = AttestationReport {
            tee_type: TeeType::IntelSgx,
            quote: vec![1, 2, 3, 4, 5], // Non-empty quote
            nonce: vec![6, 7, 8, 9, 10],
            timestamp: SystemTime::now(),
            additional_data: None,
        };
        
        // Verify the attestation
        let result = verifier.verify_attestation(&report).await.unwrap();
        
        // Verify the result
        assert!(result.is_valid);
        assert_eq!(result.security_level, SecurityLevel::High);
        assert!(result.platform_info.is_some());
        
        // Verify the platform info
        let platform_info = result.platform_info.unwrap();
        assert_eq!(platform_info.tee_type, TeeType::IntelSgx);
        assert_eq!(platform_info.version, "2.0");
        assert_eq!(platform_info.security_version, 5);
        assert_eq!(platform_info.attributes.len(), 2);
        assert!(platform_info.attributes.contains(&"encrypted_memory".to_string()));
        assert!(platform_info.attributes.contains(&"secure_boot".to_string()));
    }

    #[tokio::test]
    async fn test_verify_invalid_attestation() {
        // Create a mock verifier
        let verifier = create_mock_verifier();
        
        // Create an invalid attestation report (empty quote)
        let report = AttestationReport {
            tee_type: TeeType::IntelSgx,
            quote: vec![], // Empty quote
            nonce: vec![6, 7, 8, 9, 10],
            timestamp: SystemTime::now(),
            additional_data: None,
        };
        
        // Verify the attestation
        let result = verifier.verify_attestation(&report).await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(AttestationError::InvalidQuote(message)) => {
                assert_eq!(message, "Empty quote");
            }
            _ => panic!("Expected InvalidQuote error"),
        }
    }

    #[tokio::test]
    async fn test_verify_unsupported_tee_type() {
        // Create a mock verifier
        let verifier = create_mock_verifier();
        
        // Create an attestation report with unsupported TEE type
        let report = AttestationReport {
            tee_type: TeeType::ArmTrustZone,
            quote: vec![1, 2, 3, 4, 5],
            nonce: vec![6, 7, 8, 9, 10],
            timestamp: SystemTime::now(),
            additional_data: None,
        };
        
        // Verify the attestation
        let result = verifier.verify_attestation(&report).await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(AttestationError::UnsupportedTeeType(tee_type)) => {
                assert!(tee_type.contains("ArmTrustZone"));
            }
            _ => panic!("Expected UnsupportedTeeType error"),
        }
    }

    #[tokio::test]
    async fn test_validate_high_security_platform() {
        // Create a mock verifier
        let verifier = create_mock_verifier();
        
        // Create a platform configuration with high security
        let config = PlatformConfiguration {
            tee_type: TeeType::IntelSgx,
            version: "2.0".to_string(),
            security_version: 5,
            attributes: vec!["encrypted_memory".to_string(), "secure_boot".to_string()],
        };
        
        // Validate the platform configuration
        let security_level = verifier.validate_platform_configuration(&config).await.unwrap();
        
        // Verify the security level
        assert_eq!(security_level, SecurityLevel::High);
    }

    #[tokio::test]
    async fn test_validate_medium_security_platform() {
        // Create a mock verifier
        let verifier = create_mock_verifier();
        
        // Create a platform configuration with medium security (outdated security version)
        let config = PlatformConfiguration {
            tee_type: TeeType::IntelSgx,
            version: "2.0".to_string(),
            security_version: 4, // Less than 5
            attributes: vec!["encrypted_memory".to_string()],
        };
        
        // Validate the platform configuration
        let security_level = verifier.validate_platform_configuration(&config).await.unwrap();
        
        // Verify the security level
        assert_eq!(security_level, SecurityLevel::Medium);
    }

    #[tokio::test]
    async fn test_validate_unsupported_platform() {
        // Create a mock verifier
        let verifier = create_mock_verifier();
        
        // Create a platform configuration with unsupported TEE type
        let config = PlatformConfiguration {
            tee_type: TeeType::ArmTrustZone,
            version: "1.0".to_string(),
            security_version: 3,
            attributes: vec![],
        };
        
        // Validate the platform configuration
        let result = verifier.validate_platform_configuration(&config).await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        
        // Verify the error type
        match result {
            Err(AttestationError::UnsupportedTeeType(tee_type)) => {
                assert!(tee_type.contains("ArmTrustZone"));
            }
            _ => panic!("Expected UnsupportedTeeType error"),
        }
    }

    #[tokio::test]
    async fn test_get_verifier_name() {
        // Create a mock verifier
        let verifier = create_mock_verifier();
        
        // Get the verifier name
        let name = verifier.get_name();
        
        // Verify the name
        assert_eq!(name, "Intel SGX Attestation Verifier");
    }

    // Test with a custom implementation of the AttestationVerifier trait
    struct CustomAttestationVerifier {
        name: String,
        supported_tee_types: Vec<TeeType>,
    }

    impl CustomAttestationVerifier {
        fn new(name: &str, supported_tee_types: Vec<TeeType>) -> Self {
            CustomAttestationVerifier {
                name: name.to_string(),
                supported_tee_types,
            }
        }
    }

    impl AttestationVerifier for CustomAttestationVerifier {
        fn get_supported_tee_types(&self) -> Vec<TeeType> {
            self.supported_tee_types.clone()
        }

        async fn verify_attestation(&self, report: &AttestationReport) -> Result<AttestationResult, AttestationError> {
            // Check if the TEE type is supported
            if !self.supported_tee_types.contains(&report.tee_type) {
                return Err(AttestationError::UnsupportedTeeType(format!("{:?}", report.tee_type)));
            }

            // Check if the quote is valid (non-empty)
            if report.quote.is_empty() {
                return Err(AttestationError::InvalidQuote("Empty quote".to_string()));
            }

            // Check if the nonce is valid (non-empty)
            if report.nonce.is_empty() {
                return Err(AttestationError::InvalidNonce("Empty nonce".to_string()));
            }

            // Check if the timestamp is valid (not too old)
            let now = SystemTime::now();
            let max_age = Duration::from_secs(3600); // 1 hour
            if let Ok(age) = now.duration_since(report.timestamp) {
                if age > max_age {
                    return Err(AttestationError::ExpiredAttestation("Attestation report is too old".to_string()));
                }
            }

            // Return a valid attestation result
            Ok(AttestationResult {
                is_valid: true,
                security_level: SecurityLevel::High,
                platform_info: Some(PlatformConfiguration {
                    tee_type: report.tee_type.clone(),
                    version: "1.0".to_string(),
                    security_version: 1,
                    attributes: vec!["custom_attribute".to_string()],
                }),
                timestamp: SystemTime::now(),
            })
        }

        async fn validate_platform_configuration(&self, config: &PlatformConfiguration) -> Result<SecurityLevel, AttestationError> {
            // Check if the TEE type is supported
            if !self.supported_tee_types.contains(&config.tee_type) {
                return Err(AttestationError::UnsupportedTeeType(format!("{:?}", config.tee_type)));
            }

            // Determine security level based on security version
            let security_level = match config.security_version {
                0..=2 => SecurityLevel::Low,
                3..=4 => SecurityLevel::Medium,
                _ => SecurityLevel::High,
            };

            Ok(security_level)
        }

        fn get_name(&self) -> &str {
            &self.name
        }
    }

    #[tokio::test]
    async fn test_custom_verifier_implementation() {
        // Create a custom verifier
        let verifier = CustomAttestationVerifier::new(
            "Custom Attestation Verifier",
            vec![TeeType::IntelSgx, TeeType::AmdSev],
        );
        
        // Verify the verifier
        assert_eq!(verifier.get_name(), "Custom Attestation Verifier");
        assert_eq!(verifier.get_supported_tee_types().len(), 2);
        assert!(verifier.get_supported_tee_types().contains(&TeeType::IntelSgx));
        assert!(verifier.get_supported_tee_types().contains(&TeeType::AmdSev));
        
        // Create a valid attestation report
        let report = AttestationReport {
            tee_type: TeeType::IntelSgx,
            quote: vec![1, 2, 3, 4, 5],
            nonce: vec![6, 7, 8, 9, 10],
            timestamp: SystemTime::now(),
            additional_data: None,
        };
        
        // Verify the attestation
        let result = verifier.verify_attestation(&report).await.unwrap();
        
        // Verify the result
        assert!(result.is_valid);
        assert_eq!(result.security_level, SecurityLevel::High);
        assert!(result.platform_info.is_some());
        
        // Verify the platform info
        let platform_info = result.platform_info.unwrap();
        assert_eq!(platform_info.tee_type, TeeType::IntelSgx);
        assert_eq!(platform_info.version, "1.0");
        assert_eq!(platform_info.security_version, 1);
        assert_eq!(platform_info.attributes.len(), 1);
        assert!(platform_info.attributes.contains(&"custom_attribute".to_string()));
    }

    #[tokio::test]
    async fn test_custom_verifier_error_handling() {
        // Create a custom verifier
        let verifier = CustomAttestationVerifier::new(
            "Custom Attestation Verifier",
            vec![TeeType::IntelSgx, TeeType::AmdSev],
        );
        
        // Test with unsupported TEE type
        let report1 = AttestationReport {
            tee_type: TeeType::ArmTrustZone,
            quote: vec![1, 2, 3, 4, 5],
            nonce: vec![6, 7, 8, 9, 10],
            timestamp: SystemTime::now(),
            additional_data: None,
        };
        
        let result1 = verifier.verify_attestation(&report1).await;
        assert!(matches!(result1, Err(AttestationError::UnsupportedTeeType(_))));
        
        // Test with empty quote
        let report2 = AttestationReport {
            tee_type: TeeType::IntelSgx,
            quote: vec![],
            nonce: vec![6, 7, 8, 9, 10],
            timestamp: SystemTime::now(),
            additional_data: None,
        };
        
        let result2 = verifier.verify_attestation(&report2).await;
        assert!(matches!(result2, Err(AttestationError::InvalidQuote(_))));
        
        // Test with empty nonce
        let report3 = AttestationReport {
            tee_type: TeeType::IntelSgx,
            quote: vec![1, 2, 3, 4, 5],
            nonce: vec![],
            timestamp: SystemTime::now(),
            additional_data: None,
        };
        
        let result3 = verifier.verify_attestation(&report3).await;
        assert!(matches!(result3, Err(AttestationError::InvalidNonce(_))));
        
        // Test with expired timestamp
        let expired_time = SystemTime::now() - Duration::from_secs(7200); // 2 hours ago
        let report4 = AttestationReport {
            tee_type: TeeType::IntelSgx,
            quote: vec![1, 2, 3, 4, 5],
            nonce: vec![6, 7, 8, 9, 10],
            timestamp: expired_time,
            additional_data: None,
        };
        
        let result4 = verifier.verify_attestation(&report4).await;
        assert!(matches!(result4, Err(AttestationError::ExpiredAttestation(_))));
    }

    #[tokio::test]
    async fn test_custom_verifier_platform_validation() {
        // Create a custom verifier
        let verifier = CustomAttestationVerifier::new(
            "Custom Attestation Verifier",
            vec![TeeType::IntelSgx, TeeType::AmdSev],
        );
        
        // Test with low security level
        let config1 = PlatformConfiguration {
            tee_type: TeeType::IntelSgx,
            version: "1.0".to_string(),
            security_version: 1,
            attributes: vec![],
        };
        
        let result1 = verifier.validate_platform_configuration(&config1).await.unwrap();
        assert_eq!(result1, SecurityLevel::Low);
        
        // Test with medium security level
        let config2 = PlatformConfiguration {
            tee_type: TeeType::IntelSgx,
            version: "1.0".to_string(),
            security_version: 3,
            attributes: vec![],
        };
        
        let result2 = verifier.validate_platform_configuration(&config2).await.unwrap();
        assert_eq!(result2, SecurityLevel::Medium);
        
        // Test with high security level
        let config3 = PlatformConfiguration {
            tee_type: TeeType::IntelSgx,
            version: "1.0".to_string(),
            security_version: 5,
            attributes: vec![],
        };
        
        let result3 = verifier.validate_platform_configuration(&config3).await.unwrap();
        assert_eq!(result3, SecurityLevel::High);
        
        // Test with unsupported TEE type
        let config4 = PlatformConfiguration {
            tee_type: TeeType::ArmTrustZone,
            version: "1.0".to_string(),
            security_version: 5,
            attributes: vec![],
        };
        
        let result4 = verifier.validate_platform_configuration(&config4).await;
        assert!(matches!(result4, Err(AttestationError::UnsupportedTeeType(_))));
    }
}
