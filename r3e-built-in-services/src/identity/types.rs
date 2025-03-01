// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IdentityError {
    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Verification error: {0}")]
    Verification(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

/// Supported DID methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DidMethod {
    /// Neo N3 DID method
    Neo,

    /// Ethereum DID method
    Ethereum,

    /// Web DID method
    Web,

    /// Key DID method
    Key,
}

/// Identity credential
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityCredential {
    /// Credential ID
    pub id: String,

    /// Credential type
    pub credential_type: Vec<String>,

    /// Issuer DID
    pub issuer: String,

    /// Subject DID
    pub subject: String,

    /// Issuance date
    pub issuance_date: String,

    /// Expiration date (optional)
    pub expiration_date: Option<String>,

    /// Credential status
    pub status: CredentialStatus,

    /// Credential claims
    pub claims: serde_json::Value,

    /// Proof
    pub proof: CredentialProof,
}

/// Credential status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialStatus {
    /// Status ID
    pub id: String,

    /// Status type
    pub status_type: String,

    /// Status purpose
    pub purpose: Option<String>,
}

/// Credential proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialProof {
    /// Proof type
    pub proof_type: String,

    /// Creation date
    pub created: String,

    /// Verification method
    pub verification_method: String,

    /// Proof purpose
    pub proof_purpose: String,

    /// Proof value
    pub proof_value: String,
}

/// Identity verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerification {
    /// Verification ID
    pub id: String,

    /// Verification type
    pub verification_type: VerificationType,

    /// User DID
    pub did: String,

    /// Verification status
    pub status: VerificationStatus,

    /// Verification data
    pub data: serde_json::Value,

    /// Creation timestamp
    pub created_at: u64,

    /// Expiration timestamp
    pub expires_at: Option<u64>,
}

/// Verification type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationType {
    /// Email verification
    Email,

    /// Phone verification
    Phone,

    /// Government ID verification
    GovernmentId,

    /// Social media verification
    SocialMedia,

    /// Biometric verification
    Biometric,
}

/// Verification status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationStatus {
    /// Pending verification
    Pending,

    /// Verification in progress
    InProgress,

    /// Verification completed
    Completed,

    /// Verification failed
    Failed,

    /// Verification expired
    Expired,
}

/// Identity profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityProfile {
    /// Decentralized identifier
    pub did: String,

    /// DID method
    pub method: DidMethod,

    /// DID document
    pub document: serde_json::Value,

    /// Profile metadata
    pub metadata: HashMap<String, String>,

    /// Authentication methods
    pub auth_methods: Vec<AuthMethod>,

    /// Recovery methods
    pub recovery_methods: Vec<RecoveryMethod>,

    /// Verification methods
    pub verification_methods: Vec<String>,

    /// Creation timestamp
    pub created_at: u64,

    /// Last updated timestamp
    pub updated_at: u64,
}

/// Authentication method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthMethod {
    /// Authentication method ID
    pub id: String,

    /// Authentication type
    pub auth_type: AuthType,

    /// Authentication data
    pub data: serde_json::Value,

    /// Is this method enabled?
    pub enabled: bool,

    /// Last used timestamp
    pub last_used: Option<u64>,
}

/// Authentication type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthType {
    /// Public key authentication
    PublicKey,

    /// Password authentication
    Password,

    /// OAuth authentication
    OAuth,

    /// WebAuthn authentication
    WebAuthn,

    /// One-time password
    OTP,
}

/// Recovery method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryMethod {
    /// Recovery method ID
    pub id: String,

    /// Recovery type
    pub recovery_type: RecoveryType,

    /// Recovery data
    pub data: serde_json::Value,

    /// Is this method enabled?
    pub enabled: bool,
}

/// Recovery type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryType {
    /// Backup phrase
    BackupPhrase,

    /// Social recovery
    Social,

    /// Email recovery
    Email,

    /// Phone recovery
    Phone,

    /// Hardware device recovery
    HardwareDevice,
}

/// Authentication request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRequest {
    /// Request ID
    pub id: String,

    /// DID to authenticate
    pub did: String,

    /// Authentication method ID
    pub auth_method_id: String,

    /// Authentication data
    pub auth_data: serde_json::Value,

    /// Challenge
    pub challenge: String,

    /// Creation timestamp
    pub created_at: u64,

    /// Expiration timestamp
    pub expires_at: u64,
}

/// Authentication response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    /// Request ID
    pub request_id: String,

    /// Authentication successful?
    pub success: bool,

    /// Authentication token (if successful)
    pub token: Option<String>,

    /// Error message (if unsuccessful)
    pub error: Option<String>,

    /// Token expiration timestamp
    pub expires_at: Option<u64>,
}
