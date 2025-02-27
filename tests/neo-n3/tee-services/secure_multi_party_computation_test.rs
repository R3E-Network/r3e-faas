// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

#[cfg(test)]
mod tests {
    use r3e_tee::provider::{MpcProvider, MpcSession, MpcParticipant, MpcError};
    use r3e_tee::types::{TeeType, SecurityLevel, ComputationProtocol};
    use std::time::{Duration, SystemTime};
    use mockall::predicate::*;
    use mockall::mock;

    // Mock the MpcProvider trait for testing
    mock! {
        MpcProvider {}
        trait MpcProvider {
            async fn create_session(&self, protocol: ComputationProtocol, participants: Vec<MpcParticipant>) -> Result<MpcSession, MpcError>;
            async fn join_session(&self, session_id: &str, participant_id: &str) -> Result<(), MpcError>;
            async fn submit_input(&self, session_id: &str, participant_id: &str, input: Vec<u8>) -> Result<(), MpcError>;
            async fn get_computation_status(&self, session_id: &str) -> Result<String, MpcError>;
            async fn get_computation_result(&self, session_id: &str, participant_id: &str) -> Result<Vec<u8>, MpcError>;
            async fn abort_session(&self, session_id: &str, participant_id: &str) -> Result<(), MpcError>;
            fn get_supported_protocols(&self) -> Vec<ComputationProtocol>;
            fn get_tee_type(&self) -> TeeType;
            fn get_security_level(&self) -> SecurityLevel;
        }
    }

    // Helper function to create a mock MPC provider
    fn create_mock_provider() -> MockMpcProvider {
        let mut provider = MockMpcProvider::new();
        
        // Set up default behavior for create_session
        provider.expect_create_session()
            .returning(|protocol, participants| {
                Ok(MpcSession {
                    session_id: "session123".to_string(),
                    protocol,
                    participants: participants.clone(),
                    status: "created".to_string(),
                    created_at: SystemTime::now(),
                    expires_at: SystemTime::now() + Duration::from_secs(3600),
                })
            });
        
        // Set up default behavior for join_session
        provider.expect_join_session()
            .returning(|_, _| Ok(()));
        
        // Set up default behavior for submit_input
        provider.expect_submit_input()
            .returning(|_, _, _| Ok(()));
        
        // Set up default behavior for get_computation_status
        provider.expect_get_computation_status()
            .returning(|_| Ok("computing".to_string()));
        
        // Set up default behavior for get_computation_result
        provider.expect_get_computation_result()
            .returning(|_, _| Ok(vec![1, 2, 3, 4, 5]));
        
        // Set up default behavior for abort_session
        provider.expect_abort_session()
            .returning(|_, _| Ok(()));
        
        // Set up default behavior for get_supported_protocols
        provider.expect_get_supported_protocols()
            .returning(|| vec![
                ComputationProtocol::SecretSharing,
                ComputationProtocol::SecureMultiPartyComputation,
                ComputationProtocol::FederatedLearning,
            ]);
        
        // Set up default behavior for get_tee_type
        provider.expect_get_tee_type()
            .returning(|| TeeType::IntelSgx);
        
        // Set up default behavior for get_security_level
        provider.expect_get_security_level()
            .returning(|| SecurityLevel::High);
        
        provider
    }

    #[tokio::test]
    async fn test_mpc_session_lifecycle() {
        // Create a mock MPC provider
        let mut provider = MockMpcProvider::new();
        
        // Create participants
        let participants = vec![
            MpcParticipant {
                id: "participant1".to_string(),
                name: "Alice".to_string(),
                public_key: vec![1, 2, 3, 4, 5],
            },
            MpcParticipant {
                id: "participant2".to_string(),
                name: "Bob".to_string(),
                public_key: vec![6, 7, 8, 9, 10],
            },
        ];
        
        // Set up behavior for create_session
        provider.expect_create_session()
            .with(eq(ComputationProtocol::SecretSharing), function(|p: &Vec<MpcParticipant>| p.len() == 2))
            .times(1)
            .returning(|protocol, participants| {
                Ok(MpcSession {
                    session_id: "session123".to_string(),
                    protocol,
                    participants: participants.clone(),
                    status: "created".to_string(),
                    created_at: SystemTime::now(),
                    expires_at: SystemTime::now() + Duration::from_secs(3600),
                })
            });
        
        // Set up behavior for join_session
        provider.expect_join_session()
            .with(eq("session123"), eq("participant1"))
            .times(1)
            .returning(|_, _| Ok(()));
        
        // Set up behavior for submit_input
        provider.expect_submit_input()
            .with(eq("session123"), eq("participant1"), any())
            .times(1)
            .returning(|_, _, _| Ok(()));
        
        // Set up behavior for get_computation_status
        provider.expect_get_computation_status()
            .with(eq("session123"))
            .times(1)
            .returning(|_| Ok("computing".to_string()));
        
        // Set up behavior for get_computation_result
        provider.expect_get_computation_result()
            .with(eq("session123"), eq("participant1"))
            .times(1)
            .returning(|_, _| Ok(vec![1, 2, 3, 4, 5]));
        
        // Set up behavior for abort_session
        provider.expect_abort_session()
            .with(eq("session123"), eq("participant1"))
            .times(1)
            .returning(|_, _| Ok(()));
        
        // Create a session
        let session = provider.create_session(ComputationProtocol::SecretSharing, participants).await.unwrap();
        
        // Verify the session
        assert_eq!(session.session_id, "session123");
        assert_eq!(session.protocol, ComputationProtocol::SecretSharing);
        assert_eq!(session.status, "created");
        
        // Join the session
        let result = provider.join_session("session123", "participant1").await;
        assert!(result.is_ok());
        
        // Submit input
        let input = vec![10, 20, 30, 40, 50];
        let result = provider.submit_input("session123", "participant1", input).await;
        assert!(result.is_ok());
        
        // Get computation status
        let status = provider.get_computation_status("session123").await.unwrap();
        assert_eq!(status, "computing");
        
        // Get computation result
        let result = provider.get_computation_result("session123", "participant1").await.unwrap();
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
        
        // Abort the session
        let result = provider.abort_session("session123", "participant1").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_session_with_different_protocols() {
        // Create a mock MPC provider
        let provider = create_mock_provider();
        
        // Create participants
        let participants = vec![
            MpcParticipant {
                id: "participant1".to_string(),
                name: "Alice".to_string(),
                public_key: vec![1, 2, 3, 4, 5],
            },
            MpcParticipant {
                id: "participant2".to_string(),
                name: "Bob".to_string(),
                public_key: vec![6, 7, 8, 9, 10],
            },
        ];
        
        // Create a session with SecretSharing protocol
        let session = provider.create_session(ComputationProtocol::SecretSharing, participants.clone()).await.unwrap();
        assert_eq!(session.protocol, ComputationProtocol::SecretSharing);
        
        // Create a session with SecureMultiPartyComputation protocol
        let session = provider.create_session(ComputationProtocol::SecureMultiPartyComputation, participants.clone()).await.unwrap();
        assert_eq!(session.protocol, ComputationProtocol::SecureMultiPartyComputation);
        
        // Create a session with FederatedLearning protocol
        let session = provider.create_session(ComputationProtocol::FederatedLearning, participants.clone()).await.unwrap();
        assert_eq!(session.protocol, ComputationProtocol::FederatedLearning);
    }

    #[tokio::test]
    async fn test_create_session_with_invalid_participants() {
        // Create a custom mock provider for this test
        let mut provider = MockMpcProvider::new();
        
        // Set up behavior for create_session with empty participants
        provider.expect_create_session()
            .with(any(), function(|p: &Vec<MpcParticipant>| p.is_empty()))
            .times(1)
            .returning(|_, _| {
                Err(MpcError::InvalidParticipants("No participants provided".to_string()))
            });
        
        // Set up behavior for create_session with single participant
        provider.expect_create_session()
            .with(any(), function(|p: &Vec<MpcParticipant>| p.len() == 1))
            .times(1)
            .returning(|_, _| {
                Err(MpcError::InvalidParticipants("At least 2 participants are required".to_string()))
            });
        
        // Try to create a session with empty participants
        let empty_participants: Vec<MpcParticipant> = vec![];
        let result = provider.create_session(ComputationProtocol::SecretSharing, empty_participants).await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        match result {
            Err(MpcError::InvalidParticipants(msg)) => {
                assert_eq!(msg, "No participants provided");
            }
            _ => panic!("Expected InvalidParticipants error"),
        }
        
        // Try to create a session with a single participant
        let single_participant = vec![
            MpcParticipant {
                id: "participant1".to_string(),
                name: "Alice".to_string(),
                public_key: vec![1, 2, 3, 4, 5],
            },
        ];
        let result = provider.create_session(ComputationProtocol::SecretSharing, single_participant).await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        match result {
            Err(MpcError::InvalidParticipants(msg)) => {
                assert_eq!(msg, "At least 2 participants are required");
            }
            _ => panic!("Expected InvalidParticipants error"),
        }
    }

    #[tokio::test]
    async fn test_join_session_with_non_existent_session() {
        // Create a custom mock provider for this test
        let mut provider = MockMpcProvider::new();
        
        // Set up behavior for join_session with non-existent session
        provider.expect_join_session()
            .with(eq("non_existent_session"), any())
            .times(1)
            .returning(|_, _| {
                Err(MpcError::SessionNotFound("Session not found".to_string()))
            });
        
        // Try to join a non-existent session
        let result = provider.join_session("non_existent_session", "participant1").await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        match result {
            Err(MpcError::SessionNotFound(msg)) => {
                assert_eq!(msg, "Session not found");
            }
            _ => panic!("Expected SessionNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_join_session_with_invalid_participant() {
        // Create a custom mock provider for this test
        let mut provider = MockMpcProvider::new();
        
        // Set up behavior for join_session with invalid participant
        provider.expect_join_session()
            .with(eq("session123"), eq("invalid_participant"))
            .times(1)
            .returning(|_, _| {
                Err(MpcError::InvalidParticipant("Participant not found in session".to_string()))
            });
        
        // Try to join a session with an invalid participant
        let result = provider.join_session("session123", "invalid_participant").await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        match result {
            Err(MpcError::InvalidParticipant(msg)) => {
                assert_eq!(msg, "Participant not found in session");
            }
            _ => panic!("Expected InvalidParticipant error"),
        }
    }

    #[tokio::test]
    async fn test_submit_input_with_non_existent_session() {
        // Create a custom mock provider for this test
        let mut provider = MockMpcProvider::new();
        
        // Set up behavior for submit_input with non-existent session
        provider.expect_submit_input()
            .with(eq("non_existent_session"), any(), any())
            .times(1)
            .returning(|_, _, _| {
                Err(MpcError::SessionNotFound("Session not found".to_string()))
            });
        
        // Try to submit input to a non-existent session
        let input = vec![10, 20, 30, 40, 50];
        let result = provider.submit_input("non_existent_session", "participant1", input).await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        match result {
            Err(MpcError::SessionNotFound(msg)) => {
                assert_eq!(msg, "Session not found");
            }
            _ => panic!("Expected SessionNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_get_computation_status_with_non_existent_session() {
        // Create a custom mock provider for this test
        let mut provider = MockMpcProvider::new();
        
        // Set up behavior for get_computation_status with non-existent session
        provider.expect_get_computation_status()
            .with(eq("non_existent_session"))
            .times(1)
            .returning(|_| {
                Err(MpcError::SessionNotFound("Session not found".to_string()))
            });
        
        // Try to get the status of a non-existent session
        let result = provider.get_computation_status("non_existent_session").await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        match result {
            Err(MpcError::SessionNotFound(msg)) => {
                assert_eq!(msg, "Session not found");
            }
            _ => panic!("Expected SessionNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_get_computation_result_with_non_existent_session() {
        // Create a custom mock provider for this test
        let mut provider = MockMpcProvider::new();
        
        // Set up behavior for get_computation_result with non-existent session
        provider.expect_get_computation_result()
            .with(eq("non_existent_session"), any())
            .times(1)
            .returning(|_, _| {
                Err(MpcError::SessionNotFound("Session not found".to_string()))
            });
        
        // Try to get the result of a non-existent session
        let result = provider.get_computation_result("non_existent_session", "participant1").await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        match result {
            Err(MpcError::SessionNotFound(msg)) => {
                assert_eq!(msg, "Session not found");
            }
            _ => panic!("Expected SessionNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_abort_session_with_non_existent_session() {
        // Create a custom mock provider for this test
        let mut provider = MockMpcProvider::new();
        
        // Set up behavior for abort_session with non-existent session
        provider.expect_abort_session()
            .with(eq("non_existent_session"), any())
            .times(1)
            .returning(|_, _| {
                Err(MpcError::SessionNotFound("Session not found".to_string()))
            });
        
        // Try to abort a non-existent session
        let result = provider.abort_session("non_existent_session", "participant1").await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        match result {
            Err(MpcError::SessionNotFound(msg)) => {
                assert_eq!(msg, "Session not found");
            }
            _ => panic!("Expected SessionNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_get_supported_protocols() {
        // Create a mock MPC provider
        let provider = create_mock_provider();
        
        // Get the supported protocols
        let protocols = provider.get_supported_protocols();
        
        // Verify the protocols
        assert_eq!(protocols.len(), 3);
        assert!(protocols.contains(&ComputationProtocol::SecretSharing));
        assert!(protocols.contains(&ComputationProtocol::SecureMultiPartyComputation));
        assert!(protocols.contains(&ComputationProtocol::FederatedLearning));
    }

    #[tokio::test]
    async fn test_get_tee_type() {
        // Create a mock MPC provider
        let provider = create_mock_provider();
        
        // Get the TEE type
        let tee_type = provider.get_tee_type();
        
        // Verify the TEE type
        assert_eq!(tee_type, TeeType::IntelSgx);
    }

    #[tokio::test]
    async fn test_get_security_level() {
        // Create a mock MPC provider
        let provider = create_mock_provider();
        
        // Get the security level
        let security_level = provider.get_security_level();
        
        // Verify the security level
        assert_eq!(security_level, SecurityLevel::High);
    }

    #[tokio::test]
    async fn test_custom_mpc_provider() {
        // Define a custom MPC provider
        struct CustomMpcProvider {
            sessions: std::collections::HashMap<String, MpcSession>,
            participant_inputs: std::collections::HashMap<(String, String), Vec<u8>>,
        }
        
        impl CustomMpcProvider {
            fn new() -> Self {
                Self {
                    sessions: std::collections::HashMap::new(),
                    participant_inputs: std::collections::HashMap::new(),
                }
            }
        }
        
        #[async_trait::async_trait]
        impl MpcProvider for CustomMpcProvider {
            async fn create_session(&self, protocol: ComputationProtocol, participants: Vec<MpcParticipant>) -> Result<MpcSession, MpcError> {
                if participants.is_empty() {
                    return Err(MpcError::InvalidParticipants("No participants provided".to_string()));
                }
                
                if participants.len() < 2 {
                    return Err(MpcError::InvalidParticipants("At least 2 participants are required".to_string()));
                }
                
                let session = MpcSession {
                    session_id: format!("session_{}", SystemTime::now().elapsed().unwrap().as_secs()),
                    protocol,
                    participants,
                    status: "created".to_string(),
                    created_at: SystemTime::now(),
                    expires_at: SystemTime::now() + Duration::from_secs(3600),
                };
                
                Ok(session)
            }
            
            async fn join_session(&self, session_id: &str, participant_id: &str) -> Result<(), MpcError> {
                if !self.sessions.contains_key(session_id) {
                    return Err(MpcError::SessionNotFound(format!("Session {} not found", session_id)));
                }
                
                let session = self.sessions.get(session_id).unwrap();
                
                if !session.participants.iter().any(|p| p.id == participant_id) {
                    return Err(MpcError::InvalidParticipant(format!("Participant {} not found in session", participant_id)));
                }
                
                Ok(())
            }
            
            async fn submit_input(&self, session_id: &str, participant_id: &str, input: Vec<u8>) -> Result<(), MpcError> {
                if !self.sessions.contains_key(session_id) {
                    return Err(MpcError::SessionNotFound(format!("Session {} not found", session_id)));
                }
                
                let session = self.sessions.get(session_id).unwrap();
                
                if !session.participants.iter().any(|p| p.id == participant_id) {
                    return Err(MpcError::InvalidParticipant(format!("Participant {} not found in session", participant_id)));
                }
                
                Ok(())
            }
            
            async fn get_computation_status(&self, session_id: &str) -> Result<String, MpcError> {
                if !self.sessions.contains_key(session_id) {
                    return Err(MpcError::SessionNotFound(format!("Session {} not found", session_id)));
                }
                
                let session = self.sessions.get(session_id).unwrap();
                
                Ok(session.status.clone())
            }
            
            async fn get_computation_result(&self, session_id: &str, participant_id: &str) -> Result<Vec<u8>, MpcError> {
                if !self.sessions.contains_key(session_id) {
                    return Err(MpcError::SessionNotFound(format!("Session {} not found", session_id)));
                }
                
                let session = self.sessions.get(session_id).unwrap();
                
                if !session.participants.iter().any(|p| p.id == participant_id) {
                    return Err(MpcError::InvalidParticipant(format!("Participant {} not found in session", participant_id)));
                }
                
                // In a real implementation, this would compute the result based on the inputs
                // For testing, we just return a dummy result
                Ok(vec![1, 2, 3, 4, 5])
            }
            
            async fn abort_session(&self, session_id: &str, participant_id: &str) -> Result<(), MpcError> {
                if !self.sessions.contains_key(session_id) {
                    return Err(MpcError::SessionNotFound(format!("Session {} not found", session_id)));
                }
                
                let session = self.sessions.get(session_id).unwrap();
                
                if !session.participants.iter().any(|p| p.id == participant_id) {
                    return Err(MpcError::InvalidParticipant(format!("Participant {} not found in session", participant_id)));
                }
                
                Ok(())
            }
            
            fn get_supported_protocols(&self) -> Vec<ComputationProtocol> {
                vec![
                    ComputationProtocol::SecretSharing,
                    ComputationProtocol::SecureMultiPartyComputation,
                ]
            }
            
            fn get_tee_type(&self) -> TeeType {
                TeeType::IntelSgx
            }
            
            fn get_security_level(&self) -> SecurityLevel {
                SecurityLevel::High
            }
        }
        
        // Create a custom MPC provider
        let provider = CustomMpcProvider::new();
        
        // Create participants
        let participants = vec![
            MpcParticipant {
                id: "participant1".to_string(),
                name: "Alice".to_string(),
                public_key: vec![1, 2, 3, 4, 5],
            },
            MpcParticipant {
                id: "participant2".to_string(),
                name: "Bob".to_string(),
                public_key: vec![6, 7, 8, 9, 10],
            },
        ];
        
        // Create a session
        let session = provider.create_session(ComputationProtocol::SecretSharing, participants).await.unwrap();
        
        // Verify the session
        assert_eq!(session.protocol, ComputationProtocol::SecretSharing);
        assert_eq!(session.status, "created");
        assert_eq!(session.participants.len(), 2);
        
        // Get the supported protocols
        let protocols = provider.get_supported_protocols();
        
        // Verify the protocols
        assert_eq!(protocols.len(), 2);
        assert!(protocols.contains(&ComputationProtocol::SecretSharing));
        assert!(protocols.contains(&ComputationProtocol::SecureMultiPartyComputation));
        
        // Get the TEE type
        let tee_type = provider.get_tee_type();
        
        // Verify the TEE type
        assert_eq!(tee_type, TeeType::IntelSgx);
        
        // Get the security level
        let security_level = provider.get_security_level();
        
        // Verify the security level
        assert_eq!(security_level, SecurityLevel::High);
    }

    #[tokio::test]
    async fn test_multiple_sessions() {
        // Create a mock MPC provider
        let provider = create_mock_provider();
        
        // Create participants for session 1
        let participants1 = vec![
            MpcParticipant {
                id: "participant1".to_string(),
                name: "Alice".to_string(),
                public_key: vec![1, 2, 3, 4, 5],
            },
            MpcParticipant {
                id: "participant2".to_string(),
                name: "Bob".to_string(),
                public_key: vec![6, 7, 8, 9, 10],
            },
        ];
        
        // Create participants for session 2
        let participants2 = vec![
            MpcParticipant {
                id: "participant3".to_string(),
                name: "Charlie".to_string(),
                public_key: vec![11, 12, 13, 14, 15],
            },
            MpcParticipant {
                id: "participant4".to_string(),
                name: "Dave".to_string(),
                public_key: vec![16, 17, 18, 19, 20],
            },
            MpcParticipant {
                id: "participant5".to_string(),
                name: "Eve".to_string(),
                public_key: vec![21, 22, 23, 24, 25],
            },
        ];
        
        // Create session 1
        let session1 = provider.create_session(ComputationProtocol::SecretSharing, participants1).await.unwrap();
        
        // Create session 2
        let session2 = provider.create_session(ComputationProtocol::SecureMultiPartyComputation, participants2).await.unwrap();
        
        // Verify session 1
        assert_eq!(session1.protocol, ComputationProtocol::SecretSharing);
        assert_eq!(session1.participants.len(), 2);
        
        // Verify session 2
        assert_eq!(session2.protocol, ComputationProtocol::SecureMultiPartyComputation);
        assert_eq!(session2.participants.len(), 3);
    }
}
