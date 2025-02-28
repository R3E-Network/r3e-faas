// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! OpenFHE scheme implementation for the Fully Homomorphic Encryption service.

use crate::{
    FheCiphertext, FheCiphertextId, FheCiphertextMetadata, FheError, FheKeyPair, FheKeyPairId,
    FheParameters, FhePrivateKey, FhePrivateKeyId, FhePublicKey, FhePublicKeyId, FheResult,
    FheSchemeType, HomomorphicOperation,
};
use async_trait::async_trait;
use log::{debug, info, warn};
use rand::Rng;
use serde_json::Value;
use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use tempfile::TempDir;

use super::FheScheme;

/// OpenFHE scheme implementation for Fully Homomorphic Encryption operations.
#[derive(Debug)]
pub struct OpenFheScheme {
    /// Path to the OpenFHE library.
    pub library_path: Option<PathBuf>,
    /// Default security level in bits.
    pub default_security_level: u32,
    /// Default polynomial modulus degree.
    pub default_polynomial_modulus_degree: u32,
    /// Default plaintext modulus.
    pub default_plaintext_modulus: u32,
    /// Temporary directory for OpenFHE operations.
    pub temp_dir: TempDir,
}

impl OpenFheScheme {
    /// Create a new OpenFHE scheme.
    pub fn new(
        library_path: Option<PathBuf>,
        default_security_level: u32,
        default_polynomial_modulus_degree: u32,
        default_plaintext_modulus: u32,
    ) -> Self {
        // Create a temporary directory for OpenFHE operations
        let temp_dir = TempDir::new().expect("Failed to create temporary directory for OpenFHE operations");
        debug!("Created temporary directory for OpenFHE operations: {:?}", temp_dir.path());
        
        Self {
            library_path,
            default_security_level,
            default_polynomial_modulus_degree,
            default_plaintext_modulus,
            temp_dir,
        }
    }

    /// Get the current timestamp.
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
    
    /// Get a temporary file path in the temporary directory.
    fn get_temp_file_path(&self, prefix: &str, suffix: &str) -> PathBuf {
        self.temp_dir.path().join(format!("{}_{}{}", prefix, Self::current_timestamp(), suffix))
    }
    
    /// Write data to a temporary file and return the file path.
    fn write_temp_file(&self, prefix: &str, suffix: &str, data: &[u8]) -> FheResult<PathBuf> {
        let file_path = self.get_temp_file_path(prefix, suffix);
        fs::write(&file_path, data).map_err(|e| {
            FheError::IoError(format!("Failed to write temporary file: {}", e))
        })?;
        Ok(file_path)
    }
    
    /// Generate OpenFHE parameters based on the provided FHE parameters.
    fn generate_openfhe_params(&self, params: &FheParameters) -> FheResult<Vec<u8>> {
        // In a real implementation, we would use the OpenFHE library to generate parameters
        // For now, we'll create a placeholder
        
        let security_level = params.security_level.unwrap_or(self.default_security_level);
        let polynomial_modulus_degree = params.polynomial_modulus_degree.unwrap_or(self.default_polynomial_modulus_degree);
        let plaintext_modulus = params.plaintext_modulus.unwrap_or(self.default_plaintext_modulus);
        
        // Generate random parameters as a placeholder
        let mut rng = rand::thread_rng();
        let mut params_data = Vec::with_capacity(128);
        
        // Add metadata
        params_data.extend_from_slice(&security_level.to_le_bytes());
        params_data.extend_from_slice(&polynomial_modulus_degree.to_le_bytes());
        params_data.extend_from_slice(&plaintext_modulus.to_le_bytes());
        
        // Add random data to simulate actual parameters
        for _ in 0..100 {
            params_data.push(rng.gen());
        }
        
        Ok(params_data)
    }
    
    /// Encrypt data using OpenFHE.
    fn encrypt_openfhe(&self, public_key_data: &[u8], plaintext: &[u8]) -> FheResult<Vec<u8>> {
        // In a real implementation, we would use the OpenFHE library to encrypt the data
        // For now, we'll create a placeholder
        
        if public_key_data.len() < 4 {
            return Err(FheError::InvalidInputError("Invalid public key data".into()));
        }
        
        let mut rng = rand::thread_rng();
        let mut ciphertext_data = Vec::with_capacity(plaintext.len() * 2 + 16);
        
        // Add metadata
        ciphertext_data.extend_from_slice(&(plaintext.len() as u32).to_le_bytes());
        
        // Add a simple XOR-based encryption as a placeholder
        // In a real implementation, this would be replaced with actual OpenFHE encryption
        for (i, &byte) in plaintext.iter().enumerate() {
            let key_byte = public_key_data[i % public_key_data.len()];
            let random_byte: u8 = rng.gen();
            ciphertext_data.push(byte ^ key_byte ^ random_byte);
            ciphertext_data.push(random_byte);
        }
        
        Ok(ciphertext_data)
    }
    
    /// Decrypt data using OpenFHE.
    fn decrypt_openfhe(&self, private_key_data: &[u8], ciphertext_data: &[u8]) -> FheResult<Vec<u8>> {
        // In a real implementation, we would use the OpenFHE library to decrypt the data
        // For now, we'll create a placeholder
        
        if private_key_data.len() < 4 || ciphertext_data.len() < 4 {
            return Err(FheError::InvalidInputError("Invalid key or ciphertext data".into()));
        }
        
        // Extract metadata
        let plaintext_len = u32::from_le_bytes([
            ciphertext_data[0],
            ciphertext_data[1],
            ciphertext_data[2],
            ciphertext_data[3],
        ]) as usize;
        
        if plaintext_len * 2 + 4 > ciphertext_data.len() {
            return Err(FheError::InvalidInputError("Invalid ciphertext data".into()));
        }
        
        let mut plaintext = Vec::with_capacity(plaintext_len);
        
        // Perform a simple XOR-based decryption as a placeholder
        // In a real implementation, this would be replaced with actual OpenFHE decryption
        for i in 0..plaintext_len {
            let ciphertext_index = 4 + i * 2;
            let encrypted_byte = ciphertext_data[ciphertext_index];
            let random_byte = ciphertext_data[ciphertext_index + 1];
            let key_byte = private_key_data[i % private_key_data.len()];
            plaintext.push(encrypted_byte ^ key_byte ^ random_byte);
        }
        
        Ok(plaintext)
    }
    
    /// Add two ciphertexts using OpenFHE.
    fn add_openfhe(&self, ciphertext1: &[u8], ciphertext2: &[u8]) -> FheResult<Vec<u8>> {
        // In a real implementation, we would use the OpenFHE library to add the ciphertexts
        // For now, we'll create a placeholder
        
        if ciphertext1.len() < 4 || ciphertext2.len() < 4 {
            return Err(FheError::InvalidInputError("Invalid ciphertext data".into()));
        }
        
        // Extract metadata
        let plaintext_len1 = u32::from_le_bytes([
            ciphertext1[0],
            ciphertext1[1],
            ciphertext1[2],
            ciphertext1[3],
        ]) as usize;
        
        let plaintext_len2 = u32::from_le_bytes([
            ciphertext2[0],
            ciphertext2[1],
            ciphertext2[2],
            ciphertext2[3],
        ]) as usize;
        
        // Use the larger plaintext length
        let plaintext_len = plaintext_len1.max(plaintext_len2);
        
        let mut result_data = Vec::with_capacity(plaintext_len * 2 + 4);
        
        // Add metadata
        result_data.extend_from_slice(&(plaintext_len as u32).to_le_bytes());
        
        // Perform a simple operation as a placeholder
        // In a real implementation, this would be replaced with actual OpenFHE addition
        for i in 0..plaintext_len {
            let mut byte1 = 0;
            let mut random_byte1 = 0;
            
            if i < plaintext_len1 {
                let index1 = 4 + i * 2;
                if index1 + 1 < ciphertext1.len() {
                    byte1 = ciphertext1[index1];
                    random_byte1 = ciphertext1[index1 + 1];
                }
            }
            
            let mut byte2 = 0;
            let mut random_byte2 = 0;
            
            if i < plaintext_len2 {
                let index2 = 4 + i * 2;
                if index2 + 1 < ciphertext2.len() {
                    byte2 = ciphertext2[index2];
                    random_byte2 = ciphertext2[index2 + 1];
                }
            }
            
            // Simulate homomorphic addition
            let result_byte = byte1 ^ byte2;
            let result_random_byte = random_byte1 ^ random_byte2;
            
            result_data.push(result_byte);
            result_data.push(result_random_byte);
        }
        
        Ok(result_data)
    }
    
    /// Subtract two ciphertexts using OpenFHE.
    fn subtract_openfhe(&self, ciphertext1: &[u8], ciphertext2: &[u8]) -> FheResult<Vec<u8>> {
        // In a real implementation, we would use the OpenFHE library to subtract the ciphertexts
        // For now, we'll create a placeholder that is similar to addition
        // In many FHE schemes, subtraction is similar to addition with a negated second operand
        
        if ciphertext1.len() < 4 || ciphertext2.len() < 4 {
            return Err(FheError::InvalidInputError("Invalid ciphertext data".into()));
        }
        
        // Extract metadata
        let plaintext_len1 = u32::from_le_bytes([
            ciphertext1[0],
            ciphertext1[1],
            ciphertext1[2],
            ciphertext1[3],
        ]) as usize;
        
        let plaintext_len2 = u32::from_le_bytes([
            ciphertext2[0],
            ciphertext2[1],
            ciphertext2[2],
            ciphertext2[3],
        ]) as usize;
        
        // Use the larger plaintext length
        let plaintext_len = plaintext_len1.max(plaintext_len2);
        
        let mut result_data = Vec::with_capacity(plaintext_len * 2 + 4);
        
        // Add metadata
        result_data.extend_from_slice(&(plaintext_len as u32).to_le_bytes());
        
        // Perform a simple operation as a placeholder
        // In a real implementation, this would be replaced with actual OpenFHE subtraction
        for i in 0..plaintext_len {
            let mut byte1 = 0;
            let mut random_byte1 = 0;
            
            if i < plaintext_len1 {
                let index1 = 4 + i * 2;
                if index1 + 1 < ciphertext1.len() {
                    byte1 = ciphertext1[index1];
                    random_byte1 = ciphertext1[index1 + 1];
                }
            }
            
            let mut byte2 = 0;
            let mut random_byte2 = 0;
            
            if i < plaintext_len2 {
                let index2 = 4 + i * 2;
                if index2 + 1 < ciphertext2.len() {
                    byte2 = ciphertext2[index2];
                    random_byte2 = ciphertext2[index2 + 1];
                }
            }
            
            // Simulate homomorphic subtraction (using XOR as a placeholder)
            let result_byte = byte1 ^ byte2 ^ 0xFF; // Invert byte2 to simulate subtraction
            let result_random_byte = random_byte1 ^ random_byte2;
            
            result_data.push(result_byte);
            result_data.push(result_random_byte);
        }
        
        Ok(result_data)
    }
    
    /// Multiply two ciphertexts using OpenFHE.
    fn multiply_openfhe(&self, ciphertext1: &[u8], ciphertext2: &[u8]) -> FheResult<Vec<u8>> {
        // In a real implementation, we would use the OpenFHE library to multiply the ciphertexts
        // For now, we'll create a placeholder
        
        if ciphertext1.len() < 4 || ciphertext2.len() < 4 {
            return Err(FheError::InvalidInputError("Invalid ciphertext data".into()));
        }
        
        // Extract metadata
        let plaintext_len1 = u32::from_le_bytes([
            ciphertext1[0],
            ciphertext1[1],
            ciphertext1[2],
            ciphertext1[3],
        ]) as usize;
        
        let plaintext_len2 = u32::from_le_bytes([
            ciphertext2[0],
            ciphertext2[1],
            ciphertext2[2],
            ciphertext2[3],
        ]) as usize;
        
        // Use the larger plaintext length
        let plaintext_len = plaintext_len1.max(plaintext_len2);
        
        let mut result_data = Vec::with_capacity(plaintext_len * 2 + 4);
        
        // Add metadata
        result_data.extend_from_slice(&(plaintext_len as u32).to_le_bytes());
        
        // Perform a simple operation as a placeholder
        // In a real implementation, this would be replaced with actual OpenFHE multiplication
        for i in 0..plaintext_len {
            let mut byte1 = 0;
            let mut random_byte1 = 0;
            
            if i < plaintext_len1 {
                let index1 = 4 + i * 2;
                if index1 + 1 < ciphertext1.len() {
                    byte1 = ciphertext1[index1];
                    random_byte1 = ciphertext1[index1 + 1];
                }
            }
            
            let mut byte2 = 0;
            let mut random_byte2 = 0;
            
            if i < plaintext_len2 {
                let index2 = 4 + i * 2;
                if index2 + 1 < ciphertext2.len() {
                    byte2 = ciphertext2[index2];
                    random_byte2 = ciphertext2[index2 + 1];
                }
            }
            
            // Simulate homomorphic multiplication (using a different operation than addition)
            let result_byte = byte1 & byte2; // Use AND as a placeholder for multiplication
            let result_random_byte = random_byte1 | random_byte2; // Use OR for random bytes
            
            result_data.push(result_byte);
            result_data.push(result_random_byte);
        }
        
        Ok(result_data)
    }
    
    /// Negate a ciphertext using OpenFHE.
    fn negate_openfhe(&self, ciphertext: &[u8]) -> FheResult<Vec<u8>> {
        // In a real implementation, we would use the OpenFHE library to negate the ciphertext
        // For now, we'll create a placeholder
        
        if ciphertext.len() < 4 {
            return Err(FheError::InvalidInputError("Invalid ciphertext data".into()));
        }
        
        // Extract metadata
        let plaintext_len = u32::from_le_bytes([
            ciphertext[0],
            ciphertext[1],
            ciphertext[2],
            ciphertext[3],
        ]) as usize;
        
        let mut result_data = Vec::with_capacity(plaintext_len * 2 + 4);
        
        // Add metadata
        result_data.extend_from_slice(&(plaintext_len as u32).to_le_bytes());
        
        // Perform a simple operation as a placeholder
        // In a real implementation, this would be replaced with actual OpenFHE negation
        for i in 0..plaintext_len {
            let index = 4 + i * 2;
            if index + 1 < ciphertext.len() {
                let byte = ciphertext[index];
                let random_byte = ciphertext[index + 1];
                
                // Simulate homomorphic negation (using NOT as a placeholder)
                let result_byte = !byte;
                
                result_data.push(result_byte);
                result_data.push(random_byte);
            }
        }
        
        Ok(result_data)
    }
    
    /// Estimate the noise budget of a ciphertext using OpenFHE.
    fn estimate_noise_budget_openfhe(&self, ciphertext: &[u8]) -> FheResult<Option<u32>> {
        // In a real implementation, we would use the OpenFHE library to estimate the noise budget
        // For now, we'll create a placeholder
        
        if ciphertext.len() < 4 {
            return Err(FheError::InvalidInputError("Invalid ciphertext data".into()));
        }
        
        // Count the number of set bits in the ciphertext as a simple noise estimation
        let mut bit_count = 0;
        for &byte in ciphertext.iter().skip(4) {
            bit_count += byte.count_ones();
        }
        
        // Calculate a noise budget based on the bit count
        // In a real implementation, this would be replaced with actual OpenFHE noise estimation
        let max_noise = 128 * ciphertext.len() as u32;
        let noise = bit_count.min(max_noise);
        let noise_budget = max_noise.saturating_sub(noise);
        
        Ok(Some(noise_budget))
    }
}

#[async_trait]
impl FheScheme for OpenFheScheme {
    fn name(&self) -> &str {
        "OpenFHE"
    }

    fn scheme_type(&self) -> FheSchemeType {
        FheSchemeType::OpenFhe
    }

    async fn generate_key_pair(&self, params: &FheParameters) -> FheResult<FheKeyPair> {
        info!("Generating key pair with OpenFHE scheme");
        debug!("Parameters: {:?}", params);

        // Generate OpenFHE parameters
        let params_data = self.generate_openfhe_params(params)?;
        
        let timestamp = Self::current_timestamp();

        // In a real implementation, we would use the OpenFHE library to generate keys
        // For now, we'll create placeholder keys using the parameters
        let mut rng = rand::thread_rng();
        
        // Generate public key
        let mut public_key_data = Vec::with_capacity(128);
        public_key_data.extend_from_slice(&params_data[0..12]); // Use part of the parameters
        for _ in 0..116 {
            public_key_data.push(rng.gen());
        }
        
        // Generate private key
        let mut private_key_data = Vec::with_capacity(128);
        private_key_data.extend_from_slice(&params_data[12..24]); // Use another part of the parameters
        for _ in 0..116 {
            private_key_data.push(rng.gen());
        }

        let public_key = FhePublicKey {
            id: FhePublicKeyId::new(),
            scheme_type: FheSchemeType::OpenFhe,
            key_data: public_key_data,
            created_at: timestamp,
        };

        let private_key = FhePrivateKey {
            id: FhePrivateKeyId::new(),
            scheme_type: FheSchemeType::OpenFhe,
            key_data: private_key_data,
            created_at: timestamp,
        };

        let key_pair = FheKeyPair {
            id: FheKeyPairId::new(),
            scheme_type: FheSchemeType::OpenFhe,
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
        info!("Encrypting data with OpenFHE scheme");
        debug!("Plaintext size: {} bytes", plaintext.len());

        // Encrypt the data using OpenFHE
        let ciphertext_data = self.encrypt_openfhe(&public_key.key_data, plaintext)?;
        
        let timestamp = Self::current_timestamp();

        // Estimate the noise budget
        let noise_budget = self.estimate_noise_budget_openfhe(&ciphertext_data)?;

        let metadata = FheCiphertextMetadata {
            plaintext_size: plaintext.len(),
            ciphertext_size: ciphertext_data.len(),
            operation_count: 0,
            noise_budget,
            properties: serde_json::json!({
                "scheme": "OpenFHE",
                "version": env!("CARGO_PKG_VERSION"),
                "security_level": self.default_security_level,
                "polynomial_modulus_degree": self.default_polynomial_modulus_degree,
                "plaintext_modulus": self.default_plaintext_modulus,
            }),
        };

        let ciphertext = FheCiphertext {
            id: FheCiphertextId::new(),
            scheme_type: FheSchemeType::OpenFhe,
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
        info!("Decrypting data with OpenFHE scheme");
        debug!("Ciphertext ID: {}", ciphertext.id);

        // Ensure the ciphertext uses the OpenFHE scheme
        if ciphertext.scheme_type != FheSchemeType::OpenFhe {
            return Err(FheError::UnsupportedSchemeError(
                "Ciphertext must use the OpenFHE scheme".into(),
            ));
        }

        // Decrypt the data using OpenFHE
        let plaintext = self.decrypt_openfhe(&private_key.key_data, &ciphertext.ciphertext_data)?;
        
        Ok(plaintext)
    }

    async fn add(
        &self,
        ciphertext1: &FheCiphertext,
        ciphertext2: &FheCiphertext,
    ) -> FheResult<FheCiphertext> {
        info!("Adding ciphertexts with OpenFHE scheme");
        debug!("Ciphertext IDs: {} and {}", ciphertext1.id, ciphertext2.id);

        // Ensure both ciphertexts use the same scheme
        if ciphertext1.scheme_type != FheSchemeType::OpenFhe
            || ciphertext2.scheme_type != FheSchemeType::OpenFhe
        {
            return Err(FheError::UnsupportedSchemeError(
                "Both ciphertexts must use the OpenFHE scheme".into(),
            ));
        }

        // Ensure both ciphertexts were encrypted with the same public key
        if ciphertext1.public_key_id != ciphertext2.public_key_id {
            return Err(FheError::InvalidInputError(
                "Both ciphertexts must be encrypted with the same public key".into(),
            ));
        }

        // Add the ciphertexts using OpenFHE
        let result_data = self.add_openfhe(&ciphertext1.ciphertext_data, &ciphertext2.ciphertext_data)?;
        
        let timestamp = Self::current_timestamp();

        // Estimate the noise budget
        let noise_budget = self.estimate_noise_budget_openfhe(&result_data)?;

        let metadata = FheCiphertextMetadata {
            plaintext_size: ciphertext1.metadata.plaintext_size,
            ciphertext_size: result_data.len(),
            operation_count: ciphertext1.metadata.operation_count + ciphertext2.metadata.operation_count + 1,
            noise_budget,
            properties: serde_json::json!({
                "scheme": "OpenFHE",
                "version": env!("CARGO_PKG_VERSION"),
                "operation": "add",
                "security_level": self.default_security_level,
                "polynomial_modulus_degree": self.default_polynomial_modulus_degree,
                "plaintext_modulus": self.default_plaintext_modulus,
            }),
        };

        let result = FheCiphertext {
            id: FheCiphertextId::new(),
            scheme_type: FheSchemeType::OpenFhe,
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
        info!("Subtracting ciphertexts with OpenFHE scheme");
        debug!("Ciphertext IDs: {} and {}", ciphertext1.id, ciphertext2.id);

        // Ensure both ciphertexts use the same scheme
        if ciphertext1.scheme_type != FheSchemeType::OpenFhe
            || ciphertext2.scheme_type != FheSchemeType::OpenFhe
        {
            return Err(FheError::UnsupportedSchemeError(
                "Both ciphertexts must use the OpenFHE scheme".into(),
            ));
        }

        // Ensure both ciphertexts were encrypted with the same public key
        if ciphertext1.public_key_id != ciphertext2.public_key_id {
            return Err(FheError::InvalidInputError(
                "Both ciphertexts must be encrypted with the same public key".into(),
            ));
        }

        // Subtract the ciphertexts using OpenFHE
        let result_data = self.subtract_openfhe(&ciphertext1.ciphertext_data, &ciphertext2.ciphertext_data)?;
        
        let timestamp = Self::current_timestamp();

        // Estimate the noise budget
        let noise_budget = self.estimate_noise_budget_openfhe(&result_data)?;

        let metadata = FheCiphertextMetadata {
            plaintext_size: ciphertext1.metadata.plaintext_size,
            ciphertext_size: result_data.len(),
            operation_count: ciphertext1.metadata.operation_count + ciphertext2.metadata.operation_count + 1,
            noise_budget,
            properties: serde_json::json!({
                "scheme": "OpenFHE",
                "version": env!("CARGO_PKG_VERSION"),
                "operation": "subtract",
                "security_level": self.default_security_level,
                "polynomial_modulus_degree": self.default_polynomial_modulus_degree,
                "plaintext_modulus": self.default_plaintext_modulus,
            }),
        };

        let result = FheCiphertext {
            id: FheCiphertextId::new(),
            scheme_type: FheSchemeType::OpenFhe,
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
        info!("Multiplying ciphertexts with OpenFHE scheme");
        debug!("Ciphertext IDs: {} and {}", ciphertext1.id, ciphertext2.id);

        // Ensure both ciphertexts use the same scheme
        if ciphertext1.scheme_type != FheSchemeType::OpenFhe
            || ciphertext2.scheme_type != FheSchemeType::OpenFhe
        {
            return Err(FheError::UnsupportedSchemeError(
                "Both ciphertexts must use the OpenFHE scheme".into(),
            ));
        }

        // Ensure both ciphertexts were encrypted with the same public key
        if ciphertext1.public_key_id != ciphertext2.public_key_id {
            return Err(FheError::InvalidInputError(
                "Both ciphertexts must be encrypted with the same public key".into(),
            ));
        }

        // Multiply the ciphertexts using OpenFHE
        let result_data = self.multiply_openfhe(&ciphertext1.ciphertext_data, &ciphertext2.ciphertext_data)?;
        
        let timestamp = Self::current_timestamp();

        // Estimate the noise budget
        let noise_budget = self.estimate_noise_budget_openfhe(&result_data)?;

        let metadata = FheCiphertextMetadata {
            plaintext_size: ciphertext1.metadata.plaintext_size,
            ciphertext_size: result_data.len(),
            operation_count: ciphertext1.metadata.operation_count + ciphertext2.metadata.operation_count + 1,
            noise_budget,
            properties: serde_json::json!({
                "scheme": "OpenFHE",
                "version": env!("CARGO_PKG_VERSION"),
                "operation": "multiply",
                "security_level": self.default_security_level,
                "polynomial_modulus_degree": self.default_polynomial_modulus_degree,
                "plaintext_modulus": self.default_plaintext_modulus,
            }),
        };

        let result = FheCiphertext {
            id: FheCiphertextId::new(),
            scheme_type: FheSchemeType::OpenFhe,
            public_key_id: ciphertext1.public_key_id.clone(),
            ciphertext_data: result_data,
            created_at: timestamp,
            metadata,
        };

        Ok(result)
    }

    async fn negate(&self, ciphertext: &FheCiphertext) -> FheResult<FheCiphertext> {
        info!("Negating ciphertext with OpenFHE scheme");
        debug!("Ciphertext ID: {}", ciphertext.id);

        // Ensure the ciphertext uses the OpenFHE scheme
        if ciphertext.scheme_type != FheSchemeType::OpenFhe {
            return Err(FheError::UnsupportedSchemeError(
                "Ciphertext must use the OpenFHE scheme".into(),
            ));
        }

        // Negate the ciphertext using OpenFHE
        let result_data = self.negate_openfhe(&ciphertext.ciphertext_data)?;
        
        let timestamp = Self::current_timestamp();

        // Estimate the noise budget
        let noise_budget = self.estimate_noise_budget_openfhe(&result_data)?;

        let metadata = FheCiphertextMetadata {
            plaintext_size: ciphertext.metadata.plaintext_size,
            ciphertext_size: result_data.len(),
            operation_count: ciphertext.metadata.operation_count + 1,
            noise_budget,
            properties: serde_json::json!({
                "scheme": "OpenFHE",
                "version": env!("CARGO_PKG_VERSION"),
                "operation": "negate",
                "security_level": self.default_security_level,
                "polynomial_modulus_degree": self.default_polynomial_modulus_degree,
                "plaintext_modulus": self.default_plaintext_modulus,
            }),
        };

        let result = FheCiphertext {
            id: FheCiphertextId::new(),
            scheme_type: FheSchemeType::OpenFhe,
            public_key_id: ciphertext.public_key_id.clone(),
            ciphertext_data: result_data,
            created_at: timestamp,
            metadata,
        };

        Ok(result)
    }

    async fn estimate_noise_budget(&self, ciphertext: &FheCiphertext) -> FheResult<Option<u32>> {
        info!("Estimating noise budget with OpenFHE scheme");
        debug!("Ciphertext ID: {}", ciphertext.id);

        // Ensure the ciphertext uses the OpenFHE scheme
        if ciphertext.scheme_type != FheSchemeType::OpenFhe {
            return Err(FheError::UnsupportedSchemeError(
                "Ciphertext must use the OpenFHE scheme".into(),
            ));
        }

        // Estimate the noise budget using OpenFHE
        let noise_budget = self.estimate_noise_budget_openfhe(&ciphertext.ciphertext_data)?;
        
        Ok(noise_budget)
    }

    fn supported_operations(&self) -> Vec<crate::HomomorphicOperation> {
        vec![
            HomomorphicOperation::Add,
            HomomorphicOperation::Subtract,
            HomomorphicOperation::Multiply,
            HomomorphicOperation::Negate,
            HomomorphicOperation::Rotate,
        ]
    }

    fn get_info(&self) -> Value {
        serde_json::json!({
            "name": self.name(),
            "scheme_type": self.scheme_type().to_string(),
            "library_path": self.library_path,
            "default_security_level": self.default_security_level,
            "default_polynomial_modulus_degree": self.default_polynomial_modulus_degree,
            "default_plaintext_modulus": self.default_plaintext_modulus,
            "supported_operations": self.supported_operations().iter().map(|op| op.to_string()).collect::<Vec<String>>(),
            "version": env!("CARGO_PKG_VERSION"),
        })
    }
}
