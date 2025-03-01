-- Add wallet_address column to users table
ALTER TABLE users ADD COLUMN IF NOT EXISTS wallet_address VARCHAR(255);

-- Create auth_challenges table for wallet authentication
CREATE TABLE IF NOT EXISTS auth_challenges (
    id VARCHAR(255) PRIMARY KEY,
    address VARCHAR(255) NOT NULL,
    blockchain_type VARCHAR(50) NOT NULL,
    message TEXT NOT NULL,
    expires_at BIGINT NOT NULL,
    created_at BIGINT NOT NULL
);

-- Create index on address for faster lookups
CREATE INDEX IF NOT EXISTS idx_auth_challenges_address ON auth_challenges(address);

-- Add index on wallet_address for faster lookups
CREATE INDEX IF NOT EXISTS idx_users_wallet_address ON users(wallet_address, blockchain_type); 