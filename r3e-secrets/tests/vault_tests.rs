// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use r3e_secrets::{
    storage::{MemorySecretStorage, SecretStorage},
    vault::{SecretMetadata, SecretVault, VaultService},
    SecretError,
};
use std::sync::Arc;

#[tokio::test]
async fn test_store_and_get_secret() {
    // Create a memory storage
    let storage = Arc::new(MemorySecretStorage::new());

    // Create a vault
    let master_key = SecretVault::generate_master_key();
    let vault = SecretVault::new(storage, master_key);

    // Store a secret
    let user_id = "user1";
    let function_id = "function1";
    let name = "api_key";
    let value = b"secret_value";
    let description = Some("API key for service".to_string());
    let tags = vec!["api".to_string(), "service".to_string()];
    let expires_in = Some(3600); // 1 hour
    let rotation_period = Some(86400); // 1 day

    let secret_id = vault
        .store_secret(
            user_id,
            function_id,
            name,
            value,
            description.clone(),
            tags.clone(),
            expires_in,
            rotation_period,
        )
        .await
        .unwrap();

    // Get the secret
    let retrieved_value = vault.get_secret(user_id, function_id, &secret_id).await.unwrap();
    assert_eq!(retrieved_value, value);

    // Get the metadata
    let metadata = vault
        .get_secret_metadata(user_id, function_id, &secret_id)
        .await
        .unwrap();

    assert_eq!(metadata.name, name);
    assert_eq!(metadata.description, description);
    assert_eq!(metadata.tags, tags);
    assert_eq!(metadata.user_id, user_id);
    assert_eq!(metadata.function_id, function_id);
    assert!(metadata.expires_at > 0);
    assert_eq!(metadata.rotation_period, rotation_period.unwrap());
}

#[tokio::test]
async fn test_unauthorized_access() {
    // Create a memory storage
    let storage = Arc::new(MemorySecretStorage::new());

    // Create a vault
    let master_key = SecretVault::generate_master_key();
    let vault = SecretVault::new(storage, master_key);

    // Store a secret
    let user_id = "user1";
    let function_id = "function1";
    let name = "api_key";
    let value = b"secret_value";
    let description = Some("API key for service".to_string());
    let tags = vec!["api".to_string(), "service".to_string()];
    let expires_in = Some(3600); // 1 hour
    let rotation_period = Some(86400); // 1 day

    let secret_id = vault
        .store_secret(
            user_id,
            function_id,
            name,
            value,
            description.clone(),
            tags.clone(),
            expires_in,
            rotation_period,
        )
        .await
        .unwrap();

    // Try to access with wrong user ID
    let result = vault.get_secret("wrong_user", function_id, &secret_id).await;
    assert!(matches!(result, Err(SecretError::Unauthorized(_))));

    // Try to access with wrong function ID
    let result = vault.get_secret(user_id, "wrong_function", &secret_id).await;
    assert!(matches!(result, Err(SecretError::Unauthorized(_))));
}

#[tokio::test]
async fn test_secret_rotation() {
    // Create a memory storage
    let storage = Arc::new(MemorySecretStorage::new());

    // Create a vault
    let master_key = SecretVault::generate_master_key();
    let vault = SecretVault::new(storage, master_key);

    // Store a secret
    let user_id = "user1";
    let function_id = "function1";
    let name = "api_key";
    let value = b"secret_value";
    let description = Some("API key for service".to_string());
    let tags = vec!["api".to_string(), "service".to_string()];
    let expires_in = Some(3600); // 1 hour
    let rotation_period = Some(86400); // 1 day

    let secret_id = vault
        .store_secret(
            user_id,
            function_id,
            name,
            value,
            description.clone(),
            tags.clone(),
            expires_in,
            rotation_period,
        )
        .await
        .unwrap();

    // Rotate the secret
    let new_value = b"new_secret_value";
    vault
        .rotate_secret(user_id, function_id, &secret_id, new_value)
        .await
        .unwrap();

    // Get the rotated secret
    let retrieved_value = vault.get_secret(user_id, function_id, &secret_id).await.unwrap();
    assert_eq!(retrieved_value, new_value);

    // Get the metadata
    let metadata = vault
        .get_secret_metadata(user_id, function_id, &secret_id)
        .await
        .unwrap();

    assert_eq!(metadata.version, 2);
    assert_eq!(metadata.previous_versions.len(), 1);
}

#[tokio::test]
async fn test_secret_expiration() {
    // Create a memory storage
    let storage = Arc::new(MemorySecretStorage::new());

    // Create a vault
    let master_key = SecretVault::generate_master_key();
    let vault = SecretVault::new(storage, master_key);

    // Store a secret with immediate expiration
    let user_id = "user1";
    let function_id = "function1";
    let name = "api_key";
    let value = b"secret_value";
    let description = Some("API key for service".to_string());
    let tags = vec!["api".to_string(), "service".to_string()];
    let expires_in = Some(0); // Immediate expiration
    let rotation_period = Some(86400); // 1 day

    let secret_id = vault
        .store_secret(
            user_id,
            function_id,
            name,
            value,
            description.clone(),
            tags.clone(),
            expires_in,
            rotation_period,
        )
        .await
        .unwrap();

    // Try to get the expired secret
    let result = vault.get_secret(user_id, function_id, &secret_id).await;
    assert!(matches!(result, Err(SecretError::NotFound(_))));
}

#[tokio::test]
async fn test_master_key_rotation() {
    // Create a memory storage
    let storage = Arc::new(MemorySecretStorage::new());

    // Create a vault
    let master_key = SecretVault::generate_master_key();
    let mut vault = SecretVault::new(storage, master_key);

    // Store a secret
    let user_id = "user1";
    let function_id = "function1";
    let name = "api_key";
    let value = b"secret_value";
    let description = Some("API key for service".to_string());
    let tags = vec!["api".to_string(), "service".to_string()];
    let expires_in = Some(3600); // 1 hour
    let rotation_period = Some(86400); // 1 day

    let secret_id = vault
        .store_secret(
            user_id,
            function_id,
            name,
            value,
            description.clone(),
            tags.clone(),
            expires_in,
            rotation_period,
        )
        .await
        .unwrap();

    // Rotate the master key
    let new_master_key = SecretVault::generate_master_key();
    vault.rotate_master_key(new_master_key).await.unwrap();

    // Get the secret after key rotation
    let retrieved_value = vault.get_secret(user_id, function_id, &secret_id).await.unwrap();
    assert_eq!(retrieved_value, value);
}

#[tokio::test]
async fn test_list_secrets() {
    // Create a memory storage
    let storage = Arc::new(MemorySecretStorage::new());

    // Create a vault
    let master_key = SecretVault::generate_master_key();
    let vault = SecretVault::new(storage, master_key);

    // Store multiple secrets
    let user_id = "user1";
    let function_id = "function1";
    
    // Secret 1
    let name1 = "api_key";
    let value1 = b"secret_value1";
    let description1 = Some("API key for service".to_string());
    let tags1 = vec!["api".to_string(), "service".to_string()];
    let expires_in1 = Some(3600); // 1 hour
    let rotation_period1 = Some(86400); // 1 day

    vault
        .store_secret(
            user_id,
            function_id,
            name1,
            value1,
            description1.clone(),
            tags1.clone(),
            expires_in1,
            rotation_period1,
        )
        .await
        .unwrap();

    // Secret 2
    let name2 = "db_password";
    let value2 = b"secret_value2";
    let description2 = Some("Database password".to_string());
    let tags2 = vec!["db".to_string(), "password".to_string()];
    let expires_in2 = Some(7200); // 2 hours
    let rotation_period2 = Some(43200); // 12 hours

    vault
        .store_secret(
            user_id,
            function_id,
            name2,
            value2,
            description2.clone(),
            tags2.clone(),
            expires_in2,
            rotation_period2,
        )
        .await
        .unwrap();

    // List secrets
    let secrets = vault.list_secrets(user_id, function_id).await.unwrap();
    assert_eq!(secrets.len(), 2);

    // Check that both secrets are in the list
    let has_api_key = secrets.iter().any(|s| s.name == name1);
    let has_db_password = secrets.iter().any(|s| s.name == name2);
    assert!(has_api_key);
    assert!(has_db_password);
}
