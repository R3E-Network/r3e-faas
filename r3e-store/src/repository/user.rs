// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! User repository implementation

use crate::rocksdb::{AsyncRocksDbClient, DbError, DbResult, repository_impl};
use serde::{Deserialize, Serialize};

/// Column family name for users
pub const CF_USERS: &str = "users";
/// Column family name for user-by-username
pub const CF_USERNAMES: &str = "usernames";
/// Column family name for user-by-email
pub const CF_EMAILS: &str = "emails";

/// User entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User ID
    pub id: String,

    /// User name
    pub username: Option<String>,

    /// User email
    pub email: Option<String>,

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

/// User error enum
#[derive(Debug)]
pub enum UserError {
    /// User not found
    NotFound(String),
    
    /// User already exists
    AlreadyExists(String),
    
    /// Username already exists
    UsernameAlreadyExists(String),
    
    /// Username taken
    UsernameTaken(String),
    
    /// Email already exists
    EmailAlreadyExists(String),
    
    /// Email taken
    EmailTaken(String),
    
    /// Database error
    DbError(DbError),
    
    /// Other error
    Other(String),
}

impl From<DbError> for UserError {
    fn from(err: DbError) -> Self {
        UserError::DbError(err)
    }
}

impl From<UserError> for DbError {
    fn from(error: UserError) -> Self {
        match error {
            UserError::AlreadyExists(msg) => DbError::Other(format!("User already exists: {}", msg)),
            UserError::NotFound(msg) => DbError::Other(format!("User not found: {}", msg)),
            UserError::DbError(e) => e,
            UserError::Other(msg) => DbError::Other(format!("User error: {}", msg)),
            UserError::UsernameTaken(msg) => DbError::Other(format!("Username taken: {}", msg)),
            UserError::EmailTaken(msg) => DbError::Other(format!("Email taken: {}", msg)),
            UserError::UsernameAlreadyExists(msg) => DbError::Other(format!("Username already exists: {}", msg)),
            UserError::EmailAlreadyExists(msg) => DbError::Other(format!("Email already exists: {}", msg)),
        }
    }
}

/// User repository implementation
pub struct UserRepository {
    db: AsyncRocksDbClient,
}

impl UserRepository {
    /// Create a new user repository
    pub fn new(db: AsyncRocksDbClient) -> Self {
        Self { db }
    }

    /// Get the user column family name
    fn cf_name() -> String {
        "user".to_string()
    }

    /// Create a new user (uses to_owned() to avoid borrowing issues)
    pub async fn create(&self, user: User) -> Result<(), UserError> {
        // Validate username and email uniqueness
        if let Some(username) = &user.username {
            if self.exists_username(username).await? {
                return Err(UserError::UsernameAlreadyExists(username.clone()));
            }
        }

        if let Some(email) = &user.email {
            if self.exists_email(email).await? {
                return Err(UserError::EmailAlreadyExists(email.clone()));
            }
        }

        // Save the user with full ownership
        let user_id = user.id.clone();
        self.db.put_cf(CF_USERS, user_id.clone(), user.clone()).await?;

        // Create username index if provided
        if let Some(username) = &user.username {
            let username_str = username.clone();
            self.db.put_cf(CF_USERNAMES, username_str, user_id.clone()).await?;
        }

        // Create email index if provided
        if let Some(email) = &user.email {
            let email_str = email.clone();
            self.db.put_cf(CF_EMAILS, email_str, user_id.clone()).await?;
        }

        Ok(())
    }

    /// Get a user by ID
    pub async fn find_by_id(&self, id: &str) -> Result<Option<User>, UserError> {
        // Convert to owned string
        let id_owned = id.to_string();
        
        // Get the user from the DB
        let user = self.db.get_cf::<_, User>(CF_USERS, id_owned).await?;
        
        Ok(user)
    }

    /// Find a user by username
    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, UserError> {
        // Get the user ID from the username index
        match self.db.get_cf::<_, String>(CF_USERNAMES, username.to_string()).await? {
            Some(user_id) => self.find_by_id(&user_id).await,
            None => Ok(None),
        }
    }

    /// Find a user by email
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
        // Get the user ID from the email index
        match self.db.get_cf::<_, String>(CF_EMAILS, email.to_string()).await? {
            Some(user_id) => self.find_by_id(&user_id).await,
            None => Ok(None),
        }
    }

    /// Update a user
    pub async fn update(&self, user: User) -> Result<(), UserError> {
        // Get the current user to compare values
        let current_user = self.find_by_id(&user.id).await?;
        if let Some(current) = current_user {
            let user_id = user.id.clone();
            
            // Check if the username is changed and is already taken
            if current.username != user.username {
                if let Some(username) = &user.username {
                    if self.exists_username(username).await? {
                        return Err(UserError::UsernameAlreadyExists(username.clone()));
                    }
                }
            }
            
            // Check if the email is changed and is already taken
            if current.email != user.email {
                if let Some(email) = &user.email {
                    if self.exists_email(email).await? {
                        return Err(UserError::EmailAlreadyExists(email.clone()));
                    }
                }
            }
            
            // Update the user
            self.db.put_cf(CF_USERS, user_id, user).await?;
            
            Ok(())
        } else {
            Err(UserError::NotFound(format!(
                "User with id {} does not exist",
                user.id
            )))
        }
    }

    /// Delete a user
    pub async fn delete(&self, id: &str) -> DbResult<()> {
        // Get the user to remove indexes
        let user = self.find_by_id(id).await?;
        
        if let Some(user) = user {
            // Remove username and email indexes
            if let Some(username) = &user.username {
                self.db
                    .delete_cf(CF_USERNAMES, format!("username:{}", username))
                    .await?;
            }
            
            if let Some(email) = &user.email {
                self.db
                    .delete_cf(CF_EMAILS, format!("email:{}", email))
                    .await?;
            }
            
            // Remove the user
            self.db.delete_cf(CF_USERS, id.to_string()).await?;
        }
        
        Ok(())
    }

    /// Get all users
    async fn get_all(&self) -> DbResult<Vec<User>> {
        let results: Vec<(String, User)> = self.db.collect_cf(CF_USERS).await?;
        let users = results.into_iter().map(|(_, user)| user).collect();
        Ok(users)
    }

    /// Check if a username already exists
    pub async fn exists_username(&self, username: &str) -> Result<bool, UserError> {
        let username_owned = username.to_string();
        let db_result = self.db.exists_cf(CF_USERNAMES, username_owned).await?;
        Ok(db_result)
    }

    /// Check if an email already exists
    pub async fn exists_email(&self, email: &str) -> Result<bool, UserError> {
        let email_owned = email.to_string();
        let db_result = self.db.exists_cf(CF_EMAILS, email_owned).await?;
        Ok(db_result)
    }
}

// Implement the DbRepository trait using the macro
repository_impl!(
    UserRepository,
    AsyncRocksDbClient,
    User,
    |user: &User| user.id.to_string()
);
