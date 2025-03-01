// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! User repository implementation using RocksDB

use crate::rocksdb::{AsyncRocksDbClient, DbRepository, DbResult, RocksDbConfig};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Column family name for users
pub const CF_USERS: &str = "users";

/// User entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User ID
    pub id: String,

    /// User name
    pub username: String,

    /// User email
    pub email: String,

    /// Hashed password
    pub password_hash: String,

    /// User roles
    pub roles: Vec<String>,

    /// User is active
    pub active: bool,

    /// Created at timestamp (millis since epoch)
    pub created_at: u64,

    /// Updated at timestamp (millis since epoch)
    pub updated_at: u64,
}

/// User repository
pub struct UserRepository {
    db: Arc<AsyncRocksDbClient>,
}

impl UserRepository {
    /// Create a new user repository
    pub fn new(db: Arc<AsyncRocksDbClient>) -> Self {
        Self { db }
    }

    /// Initialize the repository
    pub fn from_config(config: RocksDbConfig) -> Self {
        // Make sure users column family is configured
        let mut config = config.clone();
        if !config.column_families.iter().any(|cf| cf.name == CF_USERS) {
            config
                .column_families
                .push(crate::rocksdb::ColumnFamilyConfig {
                    name: CF_USERS.to_string(),
                    prefix_extractor: None,
                    block_size: 4096,
                    block_cache_size: 8 * 1024 * 1024,
                    bloom_filter_bits: 10,
                    cache_index_and_filter_blocks: true,
                });
        }

        let db = Arc::new(AsyncRocksDbClient::new(config));
        Self::new(db)
    }

    /// Find user by username
    pub async fn find_by_username(&self, username: String) -> DbResult<Option<User>> {
        // In a real implementation, we would use a secondary index
        // For simplicity, we'll scan all users
        let inner = self.db.inner.clone();
        let cf_name = CF_USERS.to_string();

        tokio::task::spawn_blocking(move || {
            let users = inner.iter_cf::<User>(&cf_name, rocksdb::IteratorMode::Start)?;
            let user = users
                .into_iter()
                .filter_map(|(_, user)| {
                    if user.username == username {
                        Some(user)
                    } else {
                        None
                    }
                })
                .next();

            Ok(user)
        })
        .await
        .map_err(|e| crate::rocksdb::DbError::TransactionFailed(e.to_string()))?
    }

    /// Find users by email
    pub async fn find_by_email(&self, email: String) -> DbResult<Option<User>> {
        // Similar to find_by_username, we'll scan all users
        let inner = self.db.inner.clone();
        let cf_name = CF_USERS.to_string();
        let email = email.to_lowercase(); // Normalize email for case-insensitive comparison

        tokio::task::spawn_blocking(move || {
            let users = inner.iter_cf::<User>(&cf_name, rocksdb::IteratorMode::Start)?;
            let user = users
                .into_iter()
                .filter_map(|(_, user)| {
                    if user.email.to_lowercase() == email {
                        Some(user)
                    } else {
                        None
                    }
                })
                .next();

            Ok(user)
        })
        .await
        .map_err(|e| crate::rocksdb::DbError::TransactionFailed(e.to_string()))?
    }

    /// Find users by role
    pub async fn find_by_role(&self, role: String) -> DbResult<Vec<User>> {
        let inner = self.db.inner.clone();
        let cf_name = CF_USERS.to_string();

        tokio::task::spawn_blocking(move || {
            let users_data = inner.iter_cf::<User>(&cf_name, rocksdb::IteratorMode::Start)?;
            let users: Vec<User> = users_data
                .into_iter()
                .filter_map(|(_, user)| {
                    if user.roles.contains(&role) {
                        Some(user)
                    } else {
                        None
                    }
                })
                .collect();

            Ok(users)
        })
        .await
        .map_err(|e| crate::rocksdb::DbError::TransactionFailed(e.to_string()))?
    }

    /// Find active users
    pub async fn find_active(&self) -> DbResult<Vec<User>> {
        let inner = self.db.inner.clone();
        let cf_name = CF_USERS.to_string();

        tokio::task::spawn_blocking(move || {
            let users_data = inner.iter_cf::<User>(&cf_name, rocksdb::IteratorMode::Start)?;
            let users: Vec<User> = users_data
                .into_iter()
                .filter_map(|(_, user)| if user.active { Some(user) } else { None })
                .collect();

            Ok(users)
        })
        .await
        .map_err(|e| crate::rocksdb::DbError::TransactionFailed(e.to_string()))?
    }
}

// Implement the DbRepository trait using the macro
crate::rocksdb::impl_db_repository!(UserRepository, User, String, CF_USERS, |user: &User| user
    .id
    .clone().into());
