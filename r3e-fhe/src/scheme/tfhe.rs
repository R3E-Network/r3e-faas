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
        std::fs::write(&path, data)
            .map_err(|e| FheError::SchemeError(format!("Failed to write temporary file: {}", e)))?;
        Ok(path)
    }

    /// Generate TFHE parameters based on the provided FheParameters.
    fn generate_tfhe_params(&self, params: &FheParameters) -> FheResult<Vec<u8>> {
        // Generate TFHE parameters using the TFHE-rs library
        let security_level = params.security_level.unwrap_or(self.default_security_level);
        let polynomial_modulus_degree = params
            .polynomial_modulus_degree
            .unwrap_or(self.default_polynomial_modulus_degree);
        let plaintext_modulus = params
            .plaintext_modulus
            .unwrap_or(self.default_plaintext_modulus);

        // Create TFHE parameters
        let tfhe_params = tfhe::ConfigBuilder::default()
            .with_security_level(security_level)
            .with_polynomial_size(polynomial_modulus_degree as usize)
            .with_message_modulus(plaintext_modulus)
            .build()
            .map_err(|e| {
                FheError::SchemeError(format!("Failed to generate TFHE parameters: {}", e))
            })?;

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
        let tfhe_params = tfhe::Config::deserialize(&public_key[32..]).map_err(|e| {
            FheError::SchemeError(format!("Failed to deserialize TFHE parameters: {}", e))
        })?;

        let tfhe_public_key = tfhe::PublicKey::deserialize(&public_key[..32], &tfhe_params)
            .map_err(|e| {
                FheError::SchemeError(format!("Failed to deserialize TFHE public key: {}", e))
            })?;

        // Encrypt the plaintext using TFHE
        let ciphertext = tfhe_public_key
            .encrypt(plaintext)
            .map_err(|e| FheError::SchemeError(format!("Failed to encrypt plaintext: {}", e)))?;

        Ok(ciphertext)
    }

    /// Decrypt a ciphertext using TFHE.
    fn decrypt_tfhe(&self, private_key: &[u8], ciphertext: &[u8]) -> FheResult<Vec<u8>> {
        // Deserialize the private key and parameters
        let tfhe_params = tfhe::Config::deserialize(&private_key[32..]).map_err(|e| {
            FheError::SchemeError(format!("Failed to deserialize TFHE parameters: {}", e))
        })?;

        let tfhe_private_key = tfhe::PrivateKey::deserialize(&private_key[..32], &tfhe_params)
            .map_err(|e| {
                FheError::SchemeError(format!("Failed to deserialize TFHE private key: {}", e))
            })?;

        // Decrypt the ciphertext using TFHE
        let plaintext = tfhe_private_key
            .decrypt(ciphertext)
            .map_err(|e| FheError::SchemeError(format!("Failed to decrypt ciphertext: {}", e)))?;

        Ok(plaintext)
    }

    /// Add two ciphertexts using TFHE.
    fn add_tfhe(&self, ciphertext1: &[u8], ciphertext2: &[u8]) -> FheResult<Vec<u8>> {
        // Deserialize the ciphertexts and parameters
        let tfhe_params = tfhe::Config::deserialize(&ciphertext1[32..]).map_err(|e| {
            FheError::SchemeError(format!("Failed to deserialize TFHE parameters: {}", e))
        })?;

        let c1 = tfhe::Ciphertext::deserialize(ciphertext1, &tfhe_params).map_err(|e| {
            FheError::SchemeError(format!("Failed to deserialize first ciphertext: {}", e))
        })?;

        let c2 = tfhe::Ciphertext::deserialize(ciphertext2, &tfhe_params).map_err(|e| {
            FheError::SchemeError(format!("Failed to deserialize second ciphertext: {}", e))
        })?;

        // Add the ciphertexts using TFHE
        let result = c1
            .add(&c2)
            .map_err(|e| FheError::SchemeError(format!("Failed to add ciphertexts: {}", e)))?;

        // Serialize the result
        let result_data = result
            .serialize()
            .map_err(|e| FheError::SchemeError(format!("Failed to serialize result: {}", e)))?;

        Ok(result_data)
    }

    /// Subtract two ciphertexts using TFHE.
    fn subtract_tfhe(&self, ciphertext1: &[u8], ciphertext2: &[u8]) -> FheResult<Vec<u8>> {
        // Deserialize the ciphertexts and parameters
        let tfhe_params = tfhe::Config::deserialize(&ciphertext1[32..]).map_err(|e| {
            FheError::SchemeError(format!("Failed to deserialize TFHE parameters: {}", e))
        })?;

        let c1 = tfhe::Ciphertext::deserialize(ciphertext1, &tfhe_params).map_err(|e| {
            FheError::SchemeError(format!("Failed to deserialize first ciphertext: {}", e))
        })?;

        let c2 = tfhe::Ciphertext::deserialize(ciphertext2, &tfhe_params).map_err(|e| {
            FheError::SchemeError(format!("Failed to deserialize second ciphertext: {}", e))
        })?;

        // Subtract the ciphertexts using TFHE
        let result = c1
            .sub(&c2)
            .map_err(|e| FheError::SchemeError(format!("Failed to subtract ciphertexts: {}", e)))?;

        // Serialize the result
        let result_data = result
            .serialize()
            .map_err(|e| FheError::SchemeError(format!("Failed to serialize result: {}", e)))?;

        Ok(result_data)
    }

    /// Multiply two ciphertexts using TFHE.
    fn multiply_tfhe(&self, ciphertext1: &[u8], ciphertext2: &[u8]) -> FheResult<Vec<u8>> {
        use bincode::{deserialize, serialize};
        use tfhe::integer::RadixCiphertext;
        use tfhe::shortint::ServerKey;

        // Deserialize the ciphertexts
        let c1: RadixCiphertext = deserialize(ciphertext1).map_err(|e| {
            FheError::SerializationError(format!("Failed to deserialize ciphertext1: {}", e))
        })?;

        let c2: RadixCiphertext = deserialize(ciphertext2).map_err(|e| {
            FheError::SerializationError(format!("Failed to deserialize ciphertext2: {}", e))
        })?;

        // Extract server key parameters from the ciphertext
        // In a real implementation, the server key would be managed by a secure key manager
        // This is a simplified example that assumes the server key is available
        let server_key_path = self.get_temp_file_path("server_key.bin");
        let server_key: ServerKey = if server_key_path.exists() {
            let data = std::fs::read(&server_key_path)
                .map_err(|e| FheError::IoError(format!("Failed to read server key: {}", e)))?;

            deserialize(&data).map_err(|e| {
                FheError::SerializationError(format!("Failed to deserialize server key: {}", e))
            })?
        } else {
            return Err(FheError::InvalidOperationError(
                "Server key not found".into(),
            ));
        };

        // Multiply the ciphertexts
        let result = server_key.mul(&c1, &c2);

        // Serialize the result
        let result_data = serialize(&result).map_err(|e| {
            FheError::SerializationError(format!("Failed to serialize result: {}", e))
        })?;

        Ok(result_data)
    }

    /// Negate a ciphertext using TFHE.
    fn negate_tfhe(&self, ciphertext: &[u8]) -> FheResult<Vec<u8>> {
        use bincode::{deserialize, serialize};
        use tfhe::integer::RadixCiphertext;
        use tfhe::shortint::ServerKey;

        // Deserialize the ciphertext
        let c: RadixCiphertext = deserialize(ciphertext).map_err(|e| {
            FheError::SerializationError(format!("Failed to deserialize ciphertext: {}", e))
        })?;

        // Extract server key parameters
        let server_key_path = self.get_temp_file_path("server_key.bin");
        let server_key: ServerKey = if server_key_path.exists() {
            let data = std::fs::read(&server_key_path)
                .map_err(|e| FheError::IoError(format!("Failed to read server key: {}", e)))?;

            deserialize(&data).map_err(|e| {
                FheError::SerializationError(format!("Failed to deserialize server key: {}", e))
            })?
        } else {
            return Err(FheError::InvalidOperationError(
                "Server key not found".into(),
            ));
        };

        // Negate the ciphertext
        let result = server_key.neg(&c);

        // Serialize the result
        let result_data = serialize(&result).map_err(|e| {
            FheError::SerializationError(format!("Failed to serialize result: {}", e))
        })?;

        Ok(result_data)
    }

    /// Estimate the noise budget of a ciphertext.
    fn estimate_noise_budget_tfhe(&self, ciphertext: &[u8]) -> FheResult<Option<u32>> {
        use bincode::deserialize;
        use tfhe::integer::RadixCiphertext;
        use tfhe::shortint::ServerKey;

        // Deserialize the ciphertext
        let c: RadixCiphertext = deserialize(ciphertext).map_err(|e| {
            FheError::SerializationError(format!("Failed to deserialize ciphertext: {}", e))
        })?;

        // Extract server key parameters
        let server_key_path = self.get_temp_file_path("server_key.bin");
        let server_key: ServerKey = if server_key_path.exists() {
            let data = std::fs::read(&server_key_path)
                .map_err(|e| FheError::IoError(format!("Failed to read server key: {}", e)))?;

            deserialize(&data).map_err(|e| {
                FheError::SerializationError(format!("Failed to deserialize server key: {}", e))
            })?
        } else {
            return Err(FheError::InvalidOperationError(
                "Server key not found".into(),
            ));
        };

        // Calculate noise using the TFHE-rs library
        // In TFHE-rs, this would be done by checking the noise level of each block
        // For this example, we're using a simplified approach
        let mut total_noise = 0;
        let blocks = c.blocks();
        for block in blocks {
            let block_noise = server_key.measure_noise(block);
            total_noise += block_noise as u32;
        }

        // Return average noise level (scaled out of 100)
        let noise_budget = if blocks.len() > 0 {
            Some((total_noise / blocks.len() as u32) % 100)
        } else {
            Some(0)
        };

        Ok(noise_budget)
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
        use bincode::serialize;
        use tfhe::integer::gen_keys_radix;
        use tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2;

        // Generate TFHE parameters
        let tfhe_params = self.generate_tfhe_params(params)?;

        // Use the TFHE-rs library to generate key material
        // Number of bits to use for the integers
        let num_bits = params.key_size as usize;

        // Generate the client and server keys
        let (client_key, server_key) = gen_keys_radix(PARAM_MESSAGE_2_CARRY_2, num_bits);

        // Serialize the keys
        let public_key_data = serialize(&server_key).map_err(|e| {
            FheError::SerializationError(format!("Failed to serialize server key: {}", e))
        })?;

        let private_key_data = serialize(&client_key).map_err(|e| {
            FheError::SerializationError(format!("Failed to serialize client key: {}", e))
        })?;

        // Save the server key for future operations
        let server_key_path = self.get_temp_file_path("server_key.bin");
        std::fs::write(&server_key_path, &public_key_data)
            .map_err(|e| FheError::IoError(format!("Failed to write server key: {}", e)))?;

        // Append the parameters to the keys
        let mut public_key_with_params = public_key_data.clone();
        public_key_with_params.extend_from_slice(&tfhe_params);

        let mut private_key_with_params = private_key_data;
        private_key_with_params.extend_from_slice(&tfhe_params);

        let public_key = FhePublicKey {
            id: FhePublicKeyId::new(),
            scheme_type: FheSchemeType::Tfhe,
            key_data: public_key_with_params,
            created_at: Self::current_timestamp(),
        };

        let private_key = FhePrivateKey {
            id: FhePrivateKeyId::new(),
            scheme_type: FheSchemeType::Tfhe,
            key_data: private_key_with_params,
            created_at: Self::current_timestamp(),
        };

        let key_pair = FheKeyPair {
            id: FheKeyPairId::new(),
            scheme_type: FheSchemeType::Tfhe,
            public_key,
            private_key,
            parameters: params.clone(),
            created_at: Self::current_timestamp(),
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
        let result_data =
            self.add_tfhe(&ciphertext1.ciphertext_data, &ciphertext2.ciphertext_data)?;

        let timestamp = Self::current_timestamp();

        // Estimate the noise budget
        let noise_budget = self.estimate_noise_budget_tfhe(&result_data)?;

        let metadata = FheCiphertextMetadata {
            plaintext_size: ciphertext1.metadata.plaintext_size,
            ciphertext_size: result_data.len(),
            operation_count: ciphertext1.metadata.operation_count
                + ciphertext2.metadata.operation_count
                + 1,
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
        let result_data =
            self.subtract_tfhe(&ciphertext1.ciphertext_data, &ciphertext2.ciphertext_data)?;

        let timestamp = Self::current_timestamp();

        // Estimate the noise budget
        let noise_budget = self.estimate_noise_budget_tfhe(&result_data)?;

        let metadata = FheCiphertextMetadata {
            plaintext_size: ciphertext1.metadata.plaintext_size,
            ciphertext_size: result_data.len(),
            operation_count: ciphertext1.metadata.operation_count
                + ciphertext2.metadata.operation_count
                + 1,
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
        let result_data =
            self.multiply_tfhe(&ciphertext1.ciphertext_data, &ciphertext2.ciphertext_data)?;

        let timestamp = Self::current_timestamp();

        // Estimate the noise budget
        let noise_budget = self.estimate_noise_budget_tfhe(&result_data)?;

        let metadata = FheCiphertextMetadata {
            plaintext_size: ciphertext1.metadata.plaintext_size,
            ciphertext_size: result_data.len(),
            operation_count: ciphertext1.metadata.operation_count
                + ciphertext2.metadata.operation_count
                + 1,
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
