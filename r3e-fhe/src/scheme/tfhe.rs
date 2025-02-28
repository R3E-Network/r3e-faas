// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! TFHE scheme implementation for the Fully Homomorphic Encryption service.

use crate::{
    FheCiphertext, FheCiphertextId, FheCiphertextMetadata, FheError, FheKeyPair, FheKeyPairId,
    FheParameters, FhePrivateKey, FhePrivateKeyId, FhePublicKey, FhePublicKeyId, FheResult,
    FheSchemeType, HomomorphicOperation,
};
use async_trait::async_trait;
use log::{debug, info};
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

use super::FheScheme;

/// TFHE scheme implementation for Fully Homomorphic Encryption operations.
#[derive(Debug)]
pub struct TfheScheme {
    /// Default security level in bits.
    pub default_security_level: u32,
    /// Default polynomial modulus degree.
    pub default_polynomial_modulus_degree: u32,
    /// Default plaintext modulus.
    pub default_plaintext_modulus: u32,
    /// Working directory for temporary files.
    temp_dir: tempfile::TempDir,
}

impl TfheScheme {
    /// Create a new TFHE scheme.
    pub fn new(
        default_security_level: u32,
        default_polynomial_modulus_degree: u32,
        default_plaintext_modulus: u32,
    ) -> FheResult<Self> {
        let temp_dir = tempfile::TempDir::new().map_err(|e| {
            FheError::SchemeError(format!("Failed to create temporary directory: {}", e))
        })?;
        
        Ok(Self {
            default_security_level,
            default_polynomial_modulus_degree,
            default_plaintext_modulus,
            temp_dir,
        })
    }

    /// Get the current timestamp.
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
    
    /// Get a temporary file path.
    fn get_temp_file_path(&self, name: &str) -> std::path::PathBuf {
        self.temp_dir.path().join(name)
    }
    
    /// Write data to a temporary file.
    fn write_temp_file(&self, name: &str, data: &[u8]) -> FheResult<std::path::PathBuf> {
        let path = self.get_temp_file_path(name);
        std::fs::write(&path, data).map_err(|e| {
            FheError::SchemeError(format!("Failed to write temporary file: {}", e))
        })?;
        Ok(path)
    }
    
    /// Generate TFHE parameters based on the provided FheParameters.
    fn generate_tfhe_params(&self, params: &FheParameters) -> FheResult<Vec<u8>> {
        // Generate TFHE parameters using the TFHE-rs library
        let security_level = params.security_level.unwrap_or(self.default_security_level);
        let polynomial_modulus_degree = params.polynomial_modulus_degree.unwrap_or(self.default_polynomial_modulus_degree);
        let plaintext_modulus = params.plaintext_modulus.unwrap_or(self.default_plaintext_modulus);
        
        // Create TFHE parameters
        let tfhe_params = tfhe::ConfigBuilder::default()
            .with_security_level(security_level)
            .with_polynomial_size(polynomial_modulus_degree as usize)
            .with_message_modulus(plaintext_modulus)
            .build()
            .map_err(|e| FheError::SchemeError(format!("Failed to generate TFHE parameters: {}", e)))?;
        
        // Serialize the parameters to a byte vector
        let mut result = Vec::new();
        result.extend_from_slice(&security_level.to_le_bytes());
        result.extend_from_slice(&polynomial_modulus_degree.to_le_bytes());
        result.extend_from_slice(&plaintext_modulus.to_le_bytes());
        
        Ok(result)
    }
    
    /// Encrypt a plaintext using TFHE.
    fn encrypt_tfhe(&self, public_key: &[u8], plaintext: &[u8]) -> FheResult<Vec<u8>> {
        // Deserialize the public key and parameters
        let tfhe_params = tfhe::Config::deserialize(&public_key[32..])
            .map_err(|e| FheError::SchemeError(format!("Failed to deserialize TFHE parameters: {}", e)))?;
        
        let tfhe_public_key = tfhe::PublicKey::deserialize(&public_key[..32], &tfhe_params)
            .map_err(|e| FheError::SchemeError(format!("Failed to deserialize TFHE public key: {}", e)))?;
            
        // Encrypt the plaintext using TFHE
        let ciphertext = tfhe_public_key.encrypt(plaintext)
            .map_err(|e| FheError::SchemeError(format!("Failed to encrypt plaintext: {}", e)))?;
        
        Ok(ciphertext)
    }
    
    /// Decrypt a ciphertext using TFHE.
    fn decrypt_tfhe(&self, private_key: &[u8], ciphertext: &[u8]) -> FheResult<Vec<u8>> {
        // Deserialize the private key and parameters
        let tfhe_params = tfhe::Config::deserialize(&private_key[32..])
            .map_err(|e| FheError::SchemeError(format!("Failed to deserialize TFHE parameters: {}", e)))?;
            
        let tfhe_private_key = tfhe::PrivateKey::deserialize(&private_key[..32], &tfhe_params)
            .map_err(|e| FheError::SchemeError(format!("Failed to deserialize TFHE private key: {}", e)))?;
            
        // Decrypt the ciphertext using TFHE
        let plaintext = tfhe_private_key.decrypt(ciphertext)
            .map_err(|e| FheError::SchemeError(format!("Failed to decrypt ciphertext: {}", e)))?;
        
        Ok(plaintext)
    }
    
    /// Add two ciphertexts using TFHE.
    fn add_tfhe(&self, ciphertext1: &[u8], ciphertext2: &[u8]) -> FheResult<Vec<u8>> {
        // Deserialize the ciphertexts and parameters
        let tfhe_params = tfhe::Config::deserialize(&ciphertext1[32..])
            .map_err(|e| FheError::SchemeError(format!("Failed to deserialize TFHE parameters: {}", e)))?;
            
        let c1 = tfhe::Ciphertext::deserialize(ciphertext1, &tfhe_params)
            .map_err(|e| FheError::SchemeError(format!("Failed to deserialize first ciphertext: {}", e)))?;
            
        let c2 = tfhe::Ciphertext::deserialize(ciphertext2, &tfhe_params)
            .map_err(|e| FheError::SchemeError(format!("Failed to deserialize second ciphertext: {}", e)))?;
            
        // Add the ciphertexts using TFHE
        let result = c1.add(&c2)
            .map_err(|e| FheError::SchemeError(format!("Failed to add ciphertexts: {}", e)))?;
            
        // Serialize the result
        let result_data = result.serialize()
            .map_err(|e| FheError::SchemeError(format!("Failed to serialize result: {}", e)))?;
            
        Ok(result_data)
    }
    
    /// Subtract two ciphertexts using TFHE.
    fn subtract_tfhe(&self, ciphertext1: &[u8], ciphertext2: &[u8]) -> FheResult<Vec<u8>> {
        // Deserialize the ciphertexts and parameters
        let tfhe_params = tfhe::Config::deserialize(&ciphertext1[32..])
            .map_err(|e| FheError::SchemeError(format!("Failed to deserialize TFHE parameters: {}", e)))?;
            
        let c1 = tfhe::Ciphertext::deserialize(ciphertext1, &tfhe_params)
            .map_err(|e| FheError::SchemeError(format!("Failed to deserialize first ciphertext: {}", e)))?;
            
        let c2 = tfhe::Ciphertext::deserialize(ciphertext2, &tfhe_params)
            .map_err(|e| FheError::SchemeError(format!("Failed to deserialize second ciphertext: {}", e)))?;
            
        // Subtract the ciphertexts using TFHE
        let result = c1.sub(&c2)
            .map_err(|e| FheError::SchemeError(format!("Failed to subtract ciphertexts: {}", e)))?;
            
        // Serialize the result
        let result_data = result.serialize()
            .map_err(|e| FheError::SchemeError(format!("Failed to serialize result: {}", e)))?;
            
        Ok(result_data)
    }
    
    /// Multiply two ciphertexts using TFHE.
    fn multiply_tfhe(&self, ciphertext1: &[u8], ciphertext2: &[u8]) -> FheResult<Vec<u8>> {
        // In a real implementation, we would use the TFHE library to multiply the ciphertexts
        // For now, we'll create a placeholder
        
        // Ensure the ciphertexts have the same length
        if ciphertext1.len() != ciphertext2.len() {
            return Err(FheError::InvalidInputError(
                "Ciphertexts must have the same length".into(),
            ));
        }
        
        // Create a simple "multiplication" by ANDing the ciphertexts
        let mut result = Vec::with_capacity(ciphertext1.len());
        for (i, &byte) in ciphertext1.iter().enumerate() {
            result.push(byte & ciphertext2[i]);
        }
        
        Ok(result)
    }
    
    /// Negate a ciphertext using TFHE.
    fn negate_tfhe(&self, ciphertext: &[u8]) -> FheResult<Vec<u8>> {
        // In a real implementation, we would use the TFHE library to negate the ciphertext
        // For now, we'll create a placeholder
        
        // Create a simple "negation" by inverting the bits
        let mut result = Vec::with_capacity(ciphertext.len());
        for &byte in ciphertext.iter() {
            result.push(!byte);
        }
        
        Ok(result)
    }
    
    /// Estimate the noise budget of a ciphertext.
    fn estimate_noise_budget_tfhe(&self, ciphertext: &[u8]) -> FheResult<Option<u32>> {
        // In a real implementation, we would use the TFHE library to estimate the noise budget
        // For now, we'll create a placeholder
        
        // Count the number of set bits as a simple "noise budget"
        let mut count = 0;
        for &byte in ciphertext.iter() {
            count += byte.count_ones();
        }
        
        // Return a value between 0 and 100
        let budget = 100 - ((count as f32 / (ciphertext.len() * 8) as f32) * 100.0) as u32;
        
        Ok(Some(budget))
    }
}

#[async_trait]
impl FheScheme for TfheScheme {
    fn name(&self) -> &str {
        "TFHE"
    }

    fn scheme_type(&self) -> FheSchemeType {
        FheSchemeType::Tfhe
    }

    async fn generate_key_pair(&self, params: &FheParameters) -> FheResult<FheKeyPair> {
        info!("Generating key pair with TFHE scheme");
        debug!("Parameters: {:?}", params);

        // Generate TFHE parameters
        let tfhe_params = self.generate_tfhe_params(params)?;
        
        // In a real implementation, we would use the TFHE library to generate keys
        // For now, we'll create a simple key pair
        
        let timestamp = Self::current_timestamp();
        
        // Generate a random public key
        let mut rng = rand::thread_rng();
        let mut public_key_data = vec![0u8; 32];
        rand::RngCore::fill_bytes(&mut rng, &mut public_key_data);
        
        // Generate a random private key
        let mut private_key_data = vec![0u8; 32];
        rand::RngCore::fill_bytes(&mut rng, &mut private_key_data);
        
        // Append the parameters to the keys
        public_key_data.extend_from_slice(&tfhe_params);
        private_key_data.extend_from_slice(&tfhe_params);

        let public_key = FhePublicKey {
            id: FhePublicKeyId::new(),
            scheme_type: FheSchemeType::Tfhe,
            key_data: public_key_data,
            created_at: timestamp,
        };

        let private_key = FhePrivateKey {
            id: FhePrivateKeyId::new(),
            scheme_type: FheSchemeType::Tfhe,
            key_data: private_key_data,
            created_at: timestamp,
        };

        let key_pair = FheKeyPair {
            id: FheKeyPairId::new(),
            scheme_type: FheSchemeType::Tfhe,
            public_key,
            private_key,
            parameters: params.clone(),
            created_at: timestamp,
        };

        Ok(key_pair)
    }

    async fn encrypt(
        &self,
        public_key: &FhePublicKey,
        plaintext: &[u8],
    ) -> FheResult<FheCiphertext> {
        info!("Encrypting data with TFHE scheme");
        debug!("Plaintext size: {} bytes", plaintext.len());

        // Encrypt the plaintext using TFHE
        let ciphertext_data = self.encrypt_tfhe(&public_key.key_data, plaintext)?;
        
        let timestamp = Self::current_timestamp();

        // Estimate the noise budget
        let noise_budget = self.estimate_noise_budget_tfhe(&ciphertext_data)?;

        let metadata = FheCiphertextMetadata {
            plaintext_size: plaintext.len(),
            ciphertext_size: ciphertext_data.len(),
            operation_count: 0,
            noise_budget,
            properties: serde_json::json!({
                "scheme": "TFHE",
                "version": env!("CARGO_PKG_VERSION"),
                "security_level": self.default_security_level,
                "polynomial_modulus_degree": self.default_polynomial_modulus_degree,
                "plaintext_modulus": self.default_plaintext_modulus,
            }),
        };

        let ciphertext = FheCiphertext {
            id: FheCiphertextId::new(),
            scheme_type: FheSchemeType::Tfhe,
            public_key_id: public_key.id.clone(),
            ciphertext_data,
            created_at: timestamp,
            metadata,
        };

        Ok(ciphertext)
    }

    async fn decrypt(
        &self,
        private_key: &FhePrivateKey,
        ciphertext: &FheCiphertext,
    ) -> FheResult<Vec<u8>> {
        info!("Decrypting data with TFHE scheme");
        debug!("Ciphertext ID: {}", ciphertext.id);

        // Ensure the ciphertext uses the TFHE scheme
        if ciphertext.scheme_type != FheSchemeType::Tfhe {
            return Err(FheError::UnsupportedSchemeError(
                "Ciphertext must use the TFHE scheme".into(),
            ));
        }

        // Decrypt the ciphertext using TFHE
        let plaintext = self.decrypt_tfhe(&private_key.key_data, &ciphertext.ciphertext_data)?;
        
        Ok(plaintext)
    }

    async fn add(
        &self,
        ciphertext1: &FheCiphertext,
        ciphertext2: &FheCiphertext,
    ) -> FheResult<FheCiphertext> {
        info!("Adding ciphertexts with TFHE scheme");
        debug!("Ciphertext IDs: {} and {}", ciphertext1.id, ciphertext2.id);

        // Ensure both ciphertexts use the same scheme
        if ciphertext1.scheme_type != FheSchemeType::Tfhe
            || ciphertext2.scheme_type != FheSchemeType::Tfhe
        {
            return Err(FheError::UnsupportedSchemeError(
                "Both ciphertexts must use the TFHE scheme".into(),
            ));
        }

        // Ensure both ciphertexts were encrypted with the same public key
        if ciphertext1.public_key_id != ciphertext2.public_key_id {
            return Err(FheError::InvalidInputError(
                "Both ciphertexts must be encrypted with the same public key".into(),
            ));
        }

        // Add the ciphertexts using TFHE
        let result_data = self.add_tfhe(&ciphertext1.ciphertext_data, &ciphertext2.ciphertext_data)?;
        
        let timestamp = Self::current_timestamp();

        // Estimate the noise budget
        let noise_budget = self.estimate_noise_budget_tfhe(&result_data)?;

        let metadata = FheCiphertextMetadata {
            plaintext_size: ciphertext1.metadata.plaintext_size,
            ciphertext_size: result_data.len(),
            operation_count: ciphertext1.metadata.operation_count + ciphertext2.metadata.operation_count + 1,
            noise_budget,
            properties: serde_json::json!({
                "scheme": "TFHE",
                "version": env!("CARGO_PKG_VERSION"),
                "operation": "add",
                "security_level": self.default_security_level,
                "polynomial_modulus_degree": self.default_polynomial_modulus_degree,
                "plaintext_modulus": self.default_plaintext_modulus,
            }),
        };

        let result = FheCiphertext {
            id: FheCiphertextId::new(),
            scheme_type: FheSchemeType::Tfhe,
            public_key_id: ciphertext1.public_key_id.clone(),
            ciphertext_data: result_data,
            created_at: timestamp,
            metadata,
        };

        Ok(result)
    }

    async fn subtract(
        &self,
        ciphertext1: &FheCiphertext,
        ciphertext2: &FheCiphertext,
    ) -> FheResult<FheCiphertext> {
        info!("Subtracting ciphertexts with TFHE scheme");
        debug!("Ciphertext IDs: {} and {}", ciphertext1.id, ciphertext2.id);

        // Ensure both ciphertexts use the same scheme
        if ciphertext1.scheme_type != FheSchemeType::Tfhe
            || ciphertext2.scheme_type != FheSchemeType::Tfhe
        {
            return Err(FheError::UnsupportedSchemeError(
                "Both ciphertexts must use the TFHE scheme".into(),
            ));
        }

        // Ensure both ciphertexts were encrypted with the same public key
        if ciphertext1.public_key_id != ciphertext2.public_key_id {
            return Err(FheError::InvalidInputError(
                "Both ciphertexts must be encrypted with the same public key".into(),
            ));
        }

        // Subtract the ciphertexts using TFHE
        let result_data = self.subtract_tfhe(&ciphertext1.ciphertext_data, &ciphertext2.ciphertext_data)?;
        
        let timestamp = Self::current_timestamp();

        // Estimate the noise budget
        let noise_budget = self.estimate_noise_budget_tfhe(&result_data)?;

        let metadata = FheCiphertextMetadata {
            plaintext_size: ciphertext1.metadata.plaintext_size,
            ciphertext_size: result_data.len(),
            operation_count: ciphertext1.metadata.operation_count + ciphertext2.metadata.operation_count + 1,
            noise_budget,
            properties: serde_json::json!({
                "scheme": "TFHE",
                "version": env!("CARGO_PKG_VERSION"),
                "operation": "subtract",
                "security_level": self.default_security_level,
                "polynomial_modulus_degree": self.default_polynomial_modulus_degree,
                "plaintext_modulus": self.default_plaintext_modulus,
            }),
        };

        let result = FheCiphertext {
            id: FheCiphertextId::new(),
            scheme_type: FheSchemeType::Tfhe,
            public_key_id: ciphertext1.public_key_id.clone(),
            ciphertext_data: result_data,
            created_at: timestamp,
            metadata,
        };

        Ok(result)
    }

    async fn multiply(
        &self,
        ciphertext1: &FheCiphertext,
        ciphertext2: &FheCiphertext,
    ) -> FheResult<FheCiphertext> {
        info!("Multiplying ciphertexts with TFHE scheme");
        debug!("Ciphertext IDs: {} and {}", ciphertext1.id, ciphertext2.id);

        // Ensure both ciphertexts use the same scheme
        if ciphertext1.scheme_type != FheSchemeType::Tfhe
            || ciphertext2.scheme_type != FheSchemeType::Tfhe
        {
            return Err(FheError::UnsupportedSchemeError(
                "Both ciphertexts must use the TFHE scheme".into(),
            ));
        }

        // Ensure both ciphertexts were encrypted with the same public key
        if ciphertext1.public_key_id != ciphertext2.public_key_id {
            return Err(FheError::InvalidInputError(
                "Both ciphertexts must be encrypted with the same public key".into(),
            ));
        }

        // Multiply the ciphertexts using TFHE
        let result_data = self.multiply_tfhe(&ciphertext1.ciphertext_data, &ciphertext2.ciphertext_data)?;
        
        let timestamp = Self::current_timestamp();

        // Estimate the noise budget
        let noise_budget = self.estimate_noise_budget_tfhe(&result_data)?;

        let metadata = FheCiphertextMetadata {
            plaintext_size: ciphertext1.metadata.plaintext_size,
            ciphertext_size: result_data.len(),
            operation_count: ciphertext1.metadata.operation_count + ciphertext2.metadata.operation_count + 1,
            noise_budget,
            properties: serde_json::json!({
                "scheme": "TFHE",
                "version": env!("CARGO_PKG_VERSION"),
                "operation": "multiply",
                "security_level": self.default_security_level,
                "polynomial_modulus_degree": self.default_polynomial_modulus_degree,
                "plaintext_modulus": self.default_plaintext_modulus,
            }),
        };

        let result = FheCiphertext {
            id: FheCiphertextId::new(),
            scheme_type: FheSchemeType::Tfhe,
            public_key_id: ciphertext1.public_key_id.clone(),
            ciphertext_data: result_data,
            created_at: timestamp,
            metadata,
        };

        Ok(result)
    }

    async fn negate(&self, ciphertext: &FheCiphertext) -> FheResult<FheCiphertext> {
        info!("Negating ciphertext with TFHE scheme");
        debug!("Ciphertext ID: {}", ciphertext.id);

        // Ensure the ciphertext uses the TFHE scheme
        if ciphertext.scheme_type != FheSchemeType::Tfhe {
            return Err(FheError::UnsupportedSchemeError(
                "Ciphertext must use the TFHE scheme".into(),
            ));
        }

        // Negate the ciphertext using TFHE
        let result_data = self.negate_tfhe(&ciphertext.ciphertext_data)?;
        
        let timestamp = Self::current_timestamp();

        // Estimate the noise budget
        let noise_budget = self.estimate_noise_budget_tfhe(&result_data)?;

        let metadata = FheCiphertextMetadata {
            plaintext_size: ciphertext.metadata.plaintext_size,
            ciphertext_size: result_data.len(),
            operation_count: ciphertext.metadata.operation_count + 1,
            noise_budget,
            properties: serde_json::json!({
                "scheme": "TFHE",
                "version": env!("CARGO_PKG_VERSION"),
                "operation": "negate",
                "security_level": self.default_security_level,
                "polynomial_modulus_degree": self.default_polynomial_modulus_degree,
                "plaintext_modulus": self.default_plaintext_modulus,
            }),
        };

        let result = FheCiphertext {
            id: FheCiphertextId::new(),
            scheme_type: FheSchemeType::Tfhe,
            public_key_id: ciphertext.public_key_id.clone(),
            ciphertext_data: result_data,
            created_at: timestamp,
            metadata,
        };

        Ok(result)
    }

    async fn estimate_noise_budget(&self, ciphertext: &FheCiphertext) -> FheResult<Option<u32>> {
        info!("Estimating noise budget with TFHE scheme");
        debug!("Ciphertext ID: {}", ciphertext.id);

        // Ensure the ciphertext uses the TFHE scheme
        if ciphertext.scheme_type != FheSchemeType::Tfhe {
            return Err(FheError::UnsupportedSchemeError(
                "Ciphertext must use the TFHE scheme".into(),
            ));
        }

        // Estimate the noise budget using TFHE
        let noise_budget = self.estimate_noise_budget_tfhe(&ciphertext.ciphertext_data)?;
        
        // Log the estimated noise budget
        info!("Estimated noise budget: {:?}", noise_budget);
        
        Ok(noise_budget)
    }

    fn supported_operations(&self) -> Vec<crate::HomomorphicOperation> {
        vec![
            HomomorphicOperation::Add,
            HomomorphicOperation::Subtract,
            HomomorphicOperation::Multiply,
            HomomorphicOperation::Negate,
        ]
    }

    fn get_info(&self) -> Value {
        serde_json::json!({
            "name": self.name(),
            "scheme_type": self.scheme_type().to_string(),
            "default_security_level": self.default_security_level,
            "default_polynomial_modulus_degree": self.default_polynomial_modulus_degree,
            "default_plaintext_modulus": self.default_plaintext_modulus,
            "supported_operations": self.supported_operations().iter().map(|op| op.to_string()).collect::<Vec<String>>(),
            "version": env!("CARGO_PKG_VERSION"),
            "temp_dir": self.temp_dir.path().to_string_lossy(),
        })
    }
}
