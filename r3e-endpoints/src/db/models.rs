/// Authentication challenge for wallet authentication
#[derive(Debug, Clone)]
pub struct AuthChallenge {
    /// Challenge ID
    pub id: String,
    
    /// Wallet address
    pub address: String,
    
    /// Blockchain type
    pub blockchain_type: String,
    
    /// Challenge message to sign
    pub message: String,
    
    /// Challenge expiration timestamp
    pub expires_at: u64,
    
    /// Challenge creation timestamp
    pub created_at: u64,
} 