/// Store an authentication challenge for wallet connection
pub async fn store_auth_challenge(
    &self,
    challenge_id: &str,
    address: &str,
    blockchain_type: &str,
    message: &str,
    expires_at: u64,
) -> Result<(), String> {
    // Get database connection
    let conn = self.pool.get().await
        .map_err(|e| format!("Failed to get database connection: {}", e))?;
    
    // Insert the challenge
    conn.execute(
        "INSERT INTO auth_challenges (id, address, blockchain_type, message, expires_at, created_at) 
         VALUES ($1, $2, $3, $4, $5, $6)",
        &[
            &challenge_id,
            &address,
            &blockchain_type,
            &message,
            &(expires_at as i64),
            &(Utc::now().timestamp() as i64),
        ],
    )
    .await
    .map_err(|e| format!("Failed to store auth challenge: {}", e))?;
    
    Ok(())
}

/// Get an authentication challenge
pub async fn get_auth_challenge(&self, challenge_id: &str) -> Result<Option<AuthChallenge>, String> {
    // Get database connection
    let conn = self.pool.get().await
        .map_err(|e| format!("Failed to get database connection: {}", e))?;
    
    // Get the challenge
    let row = conn.query_opt(
        "SELECT id, address, blockchain_type, message, expires_at, created_at
         FROM auth_challenges
         WHERE id = $1",
        &[&challenge_id],
    )
    .await
    .map_err(|e| format!("Failed to get auth challenge: {}", e))?;
    
    // Parse the row
    match row {
        Some(row) => {
            let challenge = AuthChallenge {
                id: row.get(0),
                address: row.get(1),
                blockchain_type: row.get(2),
                message: row.get(3),
                expires_at: row.get::<_, i64>(4) as u64,
                created_at: row.get::<_, i64>(5) as u64,
            };
            
            Ok(Some(challenge))
        },
        None => Ok(None),
    }
}

/// Delete an authentication challenge
pub async fn delete_auth_challenge(&self, challenge_id: &str) -> Result<(), String> {
    // Get database connection
    let conn = self.pool.get().await
        .map_err(|e| format!("Failed to get database connection: {}", e))?;
    
    // Delete the challenge
    conn.execute(
        "DELETE FROM auth_challenges WHERE id = $1",
        &[&challenge_id],
    )
    .await
    .map_err(|e| format!("Failed to delete auth challenge: {}", e))?;
    
    Ok(())
}

/// Find a user by wallet address
pub async fn find_user_by_wallet_address(
    &self,
    blockchain_type: &str,
    address: &str,
) -> Result<Option<User>, String> {
    // Get database connection
    let conn = self.pool.get().await
        .map_err(|e| format!("Failed to get database connection: {}", e))?;
    
    // Get the user
    let row = conn.query_opt(
        "SELECT id, username, password_hash, email, blockchain_type, created_at, updated_at
         FROM users
         WHERE wallet_address = $1 AND blockchain_type = $2",
        &[&address, &blockchain_type],
    )
    .await
    .map_err(|e| format!("Failed to find user by wallet address: {}", e))?;
    
    // Parse the row
    match row {
        Some(row) => {
            let user = User {
                id: row.get(0),
                username: row.get(1),
                password_hash: row.get(2),
                email: row.get(3),
                blockchain_type: parse_blockchain_type(row.get::<_, String>(4).as_str()),
                created_at: row.get::<_, i64>(5) as u64,
                updated_at: row.get::<_, i64>(6) as u64,
            };
            
            Ok(Some(user))
        },
        None => Ok(None),
    }
}

/// Create a new user with wallet authentication
pub async fn create_wallet_user(
    &self,
    user_id: &str,
    wallet_address: &str,
    blockchain_type: &str,
) -> Result<(), String> {
    // Get database connection
    let conn = self.pool.get().await
        .map_err(|e| format!("Failed to get database connection: {}", e))?;
    
    // Generate a random username based on the address
    let username = format!("user_{}", &wallet_address[0..8]);
    
    // Current timestamp
    let now = Utc::now().timestamp() as i64;
    
    // Insert the user
    conn.execute(
        "INSERT INTO users (id, username, password_hash, email, blockchain_type, wallet_address, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        &[
            &user_id,
            &username,
            &"", // No password for wallet users
            &"", // No email for wallet users
            &blockchain_type,
            &wallet_address,
            &now,
            &now,
        ],
    )
    .await
    .map_err(|e| format!("Failed to create user: {}", e))?;
    
    Ok(())
} 