// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Example showing how to use RocksDB for storage

use log::{info, LevelFilter};
use r3e_store::{
    rocksdb::{ColumnFamilyConfig, RocksDbConfig},
    BlockchainType, Service, ServiceRepository, ServiceType, User, UserRepository,
};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    info!("Starting RocksDB example");

    // Create RocksDB configuration
    let config = RocksDbConfig {
        path: "./data/rockdb_example".to_string(),
        create_if_missing: true,
        create_missing_column_families: true,
        increase_parallelism: num_cpus::get() as i32,
        optimize_point_lookup: true,
        optimize_level_style_compaction: true,
        write_buffer_size: 64 * 1024 * 1024, // 64MB
        max_write_buffer_number: 3,
        min_write_buffer_number_to_merge: 1,
        column_families: vec![
            ColumnFamilyConfig {
                name: r3e_store::CF_USERS.to_string(),
                prefix_extractor: None,
                block_size: 4096,
                block_cache_size: 8 * 1024 * 1024,
                bloom_filter_bits: 10,
                cache_index_and_filter_blocks: true,
            },
            ColumnFamilyConfig {
                name: r3e_store::CF_SERVICES.to_string(),
                prefix_extractor: None,
                block_size: 4096,
                block_cache_size: 8 * 1024 * 1024,
                bloom_filter_bits: 10,
                cache_index_and_filter_blocks: true,
            },
        ],
    };

    // Initialize repositories
    let user_repo = UserRepository::from_config(config.clone());
    let service_repo = ServiceRepository::from_config(config);

    // Current timestamp
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;

    // Create a test user
    let user = User {
        id: Uuid::new_v4().to_string(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: "$argon2id$v=19$m=16,t=2,p=1$c29tZXNhbHQ$u1DxL15DIrtHhpJQsz4hQA".to_string(),
        roles: vec!["user".to_string(), "developer".to_string()],
        active: true,
        created_at: now,
        updated_at: now,
    };

    // Save the user
    info!("Saving user: {}", user.username);
    user_repo.save(&user).await?;

    // Create a test service
    let service = Service {
        id: Uuid::new_v4().to_string(),
        name: "Test FHE Service".to_string(),
        description: "A test FHE service for the example".to_string(),
        owner_id: user.id.clone(),
        enabled: true,
        service_type: ServiceType::FullyHomomorphicEncryption,
        endpoint: "https://fhe.example.com/api".to_string(),
        contract_address: None,
        blockchain_type: None,
        metadata: serde_json::json!({
            "version": "1.0.0",
            "schema": "https://schema.example.com/fhe-service.json",
        }),
        created_at: now,
        updated_at: now,
    };

    // Save the service
    info!("Saving service: {}", service.name);
    service_repo.save(&service).await?;

    // Create another service (blockchain)
    let blockchain_service = Service {
        id: Uuid::new_v4().to_string(),
        name: "Test Neo Service".to_string(),
        description: "A test Neo blockchain service for the example".to_string(),
        owner_id: user.id.clone(),
        enabled: true,
        service_type: ServiceType::Blockchain,
        endpoint: "https://neo.example.com/api".to_string(),
        contract_address: Some("0x1234567890abcdef1234567890abcdef12345678".to_string()),
        blockchain_type: Some(BlockchainType::Neo),
        metadata: serde_json::json!({
            "version": "1.0.0",
            "schema": "https://schema.example.com/neo-service.json",
        }),
        created_at: now,
        updated_at: now,
    };

    // Save the blockchain service
    info!("Saving blockchain service: {}", blockchain_service.name);
    service_repo.save(&blockchain_service).await?;

    // Retrieve the user
    let retrieved_user = user_repo.get_by_id(&user.id).await?;
    match retrieved_user {
        Some(u) => info!("Retrieved user: {} ({})", u.username, u.email),
        None => info!("User not found"),
    }

    // Find user by username
    let user_by_username = user_repo.find_by_username(&user.username).await?;
    match user_by_username {
        Some(u) => info!("Found user by username: {}", u.id),
        None => info!("User not found by username"),
    }

    // Find services by owner
    let services = service_repo.find_by_owner(&user.id).await?;
    info!("Found {} services owned by user", services.len());
    for service in &services {
        info!("  - {}: {}", service.id, service.name);
    }

    // Find services by type
    let fhe_services = service_repo
        .find_by_type(&ServiceType::FullyHomomorphicEncryption)
        .await?;
    info!("Found {} FHE services", fhe_services.len());

    // Find blockchain services
    let neo_services = service_repo
        .find_by_blockchain(&BlockchainType::Neo)
        .await?;
    info!("Found {} Neo blockchain services", neo_services.len());

    // Delete the services
    for service in services {
        info!("Deleting service: {}", service.id);
        service_repo.delete(&service.id).await?;
    }

    // Delete the user
    info!("Deleting user: {}", user.id);
    user_repo.delete(&user.id).await?;

    info!("RocksDB example completed successfully");
    Ok(())
}
