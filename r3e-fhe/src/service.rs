// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Service implementation for the Fully Homomorphic Encryption service.

use crate::{
    scheme::{FheScheme, OpenFheScheme, TfheScheme},
    storage::{FheStorage, MemoryFheStorage, RocksDbFheStorage},
    FheCiphertext, FheCiphertextId, FheConfig, FheError, FheKeyPair, FheKeyPairId, FheParameters,
    FhePrivateKey, FhePrivateKeyId, FhePublicKey, FhePublicKeyId, FheResult, FheSchemeType,
    FheStorageType, HomomorphicOperation,
};
use log::{debug, info};
use serde_json::Value;
use std::{collections::HashMap, path::Path, sync::Arc};

/// Service for Fully Homomorphic Encryption operations.
#[derive(Debug)]
pub struct FheService {
    /// Configuration for the service.
    config: FheConfig,
    /// Schemes for different FHE types.
    schemes: HashMap<FheSchemeType, Arc<dyn FheScheme>>,
    /// Storage for FHE data.
    storage: Arc<dyn FheStorage>,
}

impl FheService {
    /// Create a new FHE service with the given configuration.
    pub async fn new(config: FheConfig) -> FheResult<Self> {
        // Initialize storage
        let storage: Arc<dyn FheStorage> = match config.storage.storage_type {
            FheStorageType::Memory => Arc::new(MemoryFheStorage::new()),
            FheStorageType::RocksDb => {
                let path = config.storage.rocksdb_path.as_ref().ok_or_else(|| {
                    FheError::ConfigurationError("RocksDB path not specified".into())
                })?;
                Arc::new(RocksDbFheStorage::new(path)?)
            }
        };

        // Initialize schemes
        let mut schemes = HashMap::new();

        // Add TFHE scheme if enabled
        if let Some(tfhe_config) = &config.schemes.tfhe {
            if tfhe_config.enabled {
                let scheme = TfheScheme::new(
                    tfhe_config.default_security_level,
                    tfhe_config.default_polynomial_modulus_degree,
                    tfhe_config.default_plaintext_modulus,
                );
                schemes.insert(FheSchemeType::Tfhe, Arc::new(scheme));
            }
        }

        // Add OpenFHE scheme if enabled
        if let Some(openfhe_config) = &config.schemes.openfhe {
            if openfhe_config.enabled {
                let scheme = OpenFheScheme::new(
                    openfhe_config.library_path.clone(),
                    openfhe_config.default_security_level,
                    openfhe_config.default_polynomial_modulus_degree,
                    openfhe_config.default_plaintext_modulus,
                );
                schemes.insert(FheSchemeType::OpenFhe, Arc::new(scheme));
            }
        }

        Ok(Self {
            config,
            schemes,
            storage,
        })
    }

    /// Register a scheme for an FHE type.
    pub fn register_scheme(&mut self, scheme: Arc<dyn FheScheme>) {
        let scheme_type = scheme.scheme_type();
        info!("Registering scheme for type: {}", scheme_type);
        self.schemes.insert(scheme_type, scheme);
    }

    /// Get a scheme for an FHE type.
    fn get_scheme(&self, scheme_type: FheSchemeType) -> FheResult<Arc<dyn FheScheme>> {
        self.schemes.get(&scheme_type).cloned().ok_or_else(|| {
            FheError::UnsupportedSchemeError(format!("Unsupported scheme: {}", scheme_type))
        })
    }

    /// Generate a key pair for FHE operations.
    pub async fn generate_key_pair(
        &self,
        scheme_type: FheSchemeType,
        params: &FheParameters,
    ) -> FheResult<FheKeyPairId> {
        info!("Generating key pair for scheme: {}", scheme_type);
        debug!("Parameters: {:?}", params);

        // Get the scheme for the type
        let scheme = self.get_scheme(scheme_type)?;

        // Generate the key pair
        let key_pair = scheme.generate_key_pair(params).await?;

        // Store the key pair and its components
        self.storage.store_key_pair(&key_pair).await?;
        self.storage.store_public_key(&key_pair.public_key).await?;
        self.storage
            .store_private_key(&key_pair.private_key)
            .await?;

        Ok(key_pair.id)
    }

    /// Encrypt data using a public key.
    pub async fn encrypt(
        &self,
        public_key_id: &FhePublicKeyId,
        plaintext: &[u8],
    ) -> FheResult<FheCiphertextId> {
        info!("Encrypting data with public key: {}", public_key_id);
        debug!("Plaintext size: {} bytes", plaintext.len());

        // Check plaintext size
        if plaintext.len() > self.config.service.max_plaintext_size_bytes {
            return Err(FheError::InvalidInputError(format!(
                "Plaintext size exceeds maximum allowed: {} > {}",
                plaintext.len(),
                self.config.service.max_plaintext_size_bytes
            )));
        }

        // Get the public key
        let public_key = self.storage.get_public_key(public_key_id).await?;

        // Get the scheme for the key's type
        let scheme = self.get_scheme(public_key.scheme_type)?;

        // Encrypt the data
        let ciphertext = scheme.encrypt(&public_key, plaintext).await?;

        // Check ciphertext size
        if ciphertext.ciphertext_data.len() > self.config.service.max_ciphertext_size_bytes {
            return Err(FheError::EncryptionError(format!(
                "Ciphertext size exceeds maximum allowed: {} > {}",
                ciphertext.ciphertext_data.len(),
                self.config.service.max_ciphertext_size_bytes
            )));
        }

        // Store the ciphertext
        self.storage.store_ciphertext(&ciphertext).await?;

        Ok(ciphertext.id)
    }

    /// Decrypt data using a private key.
    pub async fn decrypt(
        &self,
        private_key_id: &FhePrivateKeyId,
        ciphertext_id: &FheCiphertextId,
    ) -> FheResult<Vec<u8>> {
        info!("Decrypting data with private key: {}", private_key_id);
        debug!("Ciphertext ID: {}", ciphertext_id);

        // Get the private key
        let private_key = self.storage.get_private_key(private_key_id).await?;

        // Get the ciphertext
        let ciphertext = self.storage.get_ciphertext(ciphertext_id).await?;

        // Get the scheme for the key's type
        let scheme = self.get_scheme(private_key.scheme_type)?;

        // Decrypt the data
        scheme.decrypt(&private_key, &ciphertext).await
    }

    /// Add two ciphertexts homomorphically.
    pub async fn add(
        &self,
        ciphertext1_id: &FheCiphertextId,
        ciphertext2_id: &FheCiphertextId,
    ) -> FheResult<FheCiphertextId> {
        info!(
            "Adding ciphertexts: {} and {}",
            ciphertext1_id, ciphertext2_id
        );

        // Get the ciphertexts
        let ciphertext1 = self.storage.get_ciphertext(ciphertext1_id).await?;
        let ciphertext2 = self.storage.get_ciphertext(ciphertext2_id).await?;

        // Ensure both ciphertexts use the same scheme
        if ciphertext1.scheme_type != ciphertext2.scheme_type {
            return Err(FheError::InvalidInputError(format!(
                "Ciphertexts use different schemes: {} and {}",
                ciphertext1.scheme_type, ciphertext2.scheme_type
            )));
        }

        // Ensure both ciphertexts were encrypted with the same public key
        if ciphertext1.public_key_id != ciphertext2.public_key_id {
            return Err(FheError::InvalidInputError(
                "Ciphertexts must be encrypted with the same public key".into(),
            ));
        }

        // Get the scheme for the ciphertext's type
        let scheme = self.get_scheme(ciphertext1.scheme_type)?;

        // Ensure the scheme supports addition
        if !scheme
            .supported_operations()
            .contains(&HomomorphicOperation::Add)
        {
            return Err(FheError::UnsupportedSchemeError(format!(
                "Scheme {} does not support addition",
                ciphertext1.scheme_type
            )));
        }

        // Add the ciphertexts
        let result = scheme.add(&ciphertext1, &ciphertext2).await?;

        // Check result size
        if result.ciphertext_data.len() > self.config.service.max_ciphertext_size_bytes {
            return Err(FheError::HomomorphicOperationError(format!(
                "Result size exceeds maximum allowed: {} > {}",
                result.ciphertext_data.len(),
                self.config.service.max_ciphertext_size_bytes
            )));
        }

        // Store the result
        self.storage.store_ciphertext(&result).await?;

        Ok(result.id)
    }

    /// Subtract one ciphertext from another homomorphically.
    pub async fn subtract(
        &self,
        ciphertext1_id: &FheCiphertextId,
        ciphertext2_id: &FheCiphertextId,
    ) -> FheResult<FheCiphertextId> {
        info!(
            "Subtracting ciphertexts: {} and {}",
            ciphertext1_id, ciphertext2_id
        );

        // Get the ciphertexts
        let ciphertext1 = self.storage.get_ciphertext(ciphertext1_id).await?;
        let ciphertext2 = self.storage.get_ciphertext(ciphertext2_id).await?;

        // Ensure both ciphertexts use the same scheme
        if ciphertext1.scheme_type != ciphertext2.scheme_type {
            return Err(FheError::InvalidInputError(format!(
                "Ciphertexts use different schemes: {} and {}",
                ciphertext1.scheme_type, ciphertext2.scheme_type
            )));
        }

        // Ensure both ciphertexts were encrypted with the same public key
        if ciphertext1.public_key_id != ciphertext2.public_key_id {
            return Err(FheError::InvalidInputError(
                "Ciphertexts must be encrypted with the same public key".into(),
            ));
        }

        // Get the scheme for the ciphertext's type
        let scheme = self.get_scheme(ciphertext1.scheme_type)?;

        // Ensure the scheme supports subtraction
        if !scheme
            .supported_operations()
            .contains(&HomomorphicOperation::Subtract)
        {
            return Err(FheError::UnsupportedSchemeError(format!(
                "Scheme {} does not support subtraction",
                ciphertext1.scheme_type
            )));
        }

        // Subtract the ciphertexts
        let result = scheme.subtract(&ciphertext1, &ciphertext2).await?;

        // Check result size
        if result.ciphertext_data.len() > self.config.service.max_ciphertext_size_bytes {
            return Err(FheError::HomomorphicOperationError(format!(
                "Result size exceeds maximum allowed: {} > {}",
                result.ciphertext_data.len(),
                self.config.service.max_ciphertext_size_bytes
            )));
        }

        // Store the result
        self.storage.store_ciphertext(&result).await?;

        Ok(result.id)
    }

    /// Multiply two ciphertexts homomorphically.
    pub async fn multiply(
        &self,
        ciphertext1_id: &FheCiphertextId,
        ciphertext2_id: &FheCiphertextId,
    ) -> FheResult<FheCiphertextId> {
        info!(
            "Multiplying ciphertexts: {} and {}",
            ciphertext1_id, ciphertext2_id
        );

        // Get the ciphertexts
        let ciphertext1 = self.storage.get_ciphertext(ciphertext1_id).await?;
        let ciphertext2 = self.storage.get_ciphertext(ciphertext2_id).await?;

        // Ensure both ciphertexts use the same scheme
        if ciphertext1.scheme_type != ciphertext2.scheme_type {
            return Err(FheError::InvalidInputError(format!(
                "Ciphertexts use different schemes: {} and {}",
                ciphertext1.scheme_type, ciphertext2.scheme_type
            )));
        }

        // Ensure both ciphertexts were encrypted with the same public key
        if ciphertext1.public_key_id != ciphertext2.public_key_id {
            return Err(FheError::InvalidInputError(
                "Ciphertexts must be encrypted with the same public key".into(),
            ));
        }

        // Get the scheme for the ciphertext's type
        let scheme = self.get_scheme(ciphertext1.scheme_type)?;

        // Ensure the scheme supports multiplication
        if !scheme
            .supported_operations()
            .contains(&HomomorphicOperation::Multiply)
        {
            return Err(FheError::UnsupportedSchemeError(format!(
                "Scheme {} does not support multiplication",
                ciphertext1.scheme_type
            )));
        }

        // Multiply the ciphertexts
        let result = scheme.multiply(&ciphertext1, &ciphertext2).await?;

        // Check result size
        if result.ciphertext_data.len() > self.config.service.max_ciphertext_size_bytes {
            return Err(FheError::HomomorphicOperationError(format!(
                "Result size exceeds maximum allowed: {} > {}",
                result.ciphertext_data.len(),
                self.config.service.max_ciphertext_size_bytes
            )));
        }

        // Store the result
        self.storage.store_ciphertext(&result).await?;

        Ok(result.id)
    }

    /// Negate a ciphertext homomorphically.
    pub async fn negate(&self, ciphertext_id: &FheCiphertextId) -> FheResult<FheCiphertextId> {
        info!("Negating ciphertext: {}", ciphertext_id);

        // Get the ciphertext
        let ciphertext = self.storage.get_ciphertext(ciphertext_id).await?;

        // Get the scheme for the ciphertext's type
        let scheme = self.get_scheme(ciphertext.scheme_type)?;

        // Ensure the scheme supports negation
        if !scheme
            .supported_operations()
            .contains(&HomomorphicOperation::Negate)
        {
            return Err(FheError::UnsupportedSchemeError(format!(
                "Scheme {} does not support negation",
                ciphertext.scheme_type
            )));
        }

        // Negate the ciphertext
        let result = scheme.negate(&ciphertext).await?;

        // Check result size
        if result.ciphertext_data.len() > self.config.service.max_ciphertext_size_bytes {
            return Err(FheError::HomomorphicOperationError(format!(
                "Result size exceeds maximum allowed: {} > {}",
                result.ciphertext_data.len(),
                self.config.service.max_ciphertext_size_bytes
            )));
        }

        // Store the result
        self.storage.store_ciphertext(&result).await?;

        Ok(result.id)
    }

    /// Estimate the noise budget of a ciphertext.
    pub async fn estimate_noise_budget(
        &self,
        ciphertext_id: &FheCiphertextId,
    ) -> FheResult<Option<u32>> {
        info!("Estimating noise budget for ciphertext: {}", ciphertext_id);

        // Get the ciphertext
        let ciphertext = self.storage.get_ciphertext(ciphertext_id).await?;

        // Get the scheme for the ciphertext's type
        let scheme = self.get_scheme(ciphertext.scheme_type)?;

        // Estimate the noise budget
        scheme.estimate_noise_budget(&ciphertext).await
    }

    /// Get a key pair by ID.
    pub async fn get_key_pair(&self, id: &FheKeyPairId) -> FheResult<FheKeyPair> {
        self.storage.get_key_pair(id).await
    }

    /// List all key pairs.
    pub async fn list_key_pairs(&self) -> FheResult<Vec<FheKeyPair>> {
        self.storage.list_key_pairs().await
    }

    /// Delete a key pair by ID.
    pub async fn delete_key_pair(&self, id: &FheKeyPairId) -> FheResult<()> {
        info!("Deleting key pair: {}", id);

        // Get the key pair
        let key_pair = self.storage.get_key_pair(id).await?;

        // Delete the public and private keys
        self.storage
            .delete_public_key(&key_pair.public_key.id)
            .await?;
        self.storage
            .delete_private_key(&key_pair.private_key.id)
            .await?;

        // Delete the key pair
        self.storage.delete_key_pair(id).await
    }

    /// Get a public key by ID.
    pub async fn get_public_key(&self, id: &FhePublicKeyId) -> FheResult<FhePublicKey> {
        self.storage.get_public_key(id).await
    }

    /// List all public keys.
    pub async fn list_public_keys(&self) -> FheResult<Vec<FhePublicKey>> {
        self.storage.list_public_keys().await
    }

    /// Delete a public key by ID.
    pub async fn delete_public_key(&self, id: &FhePublicKeyId) -> FheResult<()> {
        info!("Deleting public key: {}", id);
        self.storage.delete_public_key(id).await
    }

    /// Get a private key by ID.
    pub async fn get_private_key(&self, id: &FhePrivateKeyId) -> FheResult<FhePrivateKey> {
        self.storage.get_private_key(id).await
    }

    /// List all private keys.
    pub async fn list_private_keys(&self) -> FheResult<Vec<FhePrivateKey>> {
        self.storage.list_private_keys().await
    }

    /// Delete a private key by ID.
    pub async fn delete_private_key(&self, id: &FhePrivateKeyId) -> FheResult<()> {
        info!("Deleting private key: {}", id);
        self.storage.delete_private_key(id).await
    }

    /// Get a ciphertext by ID.
    pub async fn get_ciphertext(&self, id: &FheCiphertextId) -> FheResult<FheCiphertext> {
        self.storage.get_ciphertext(id).await
    }

    /// List all ciphertexts.
    pub async fn list_ciphertexts(&self) -> FheResult<Vec<FheCiphertext>> {
        self.storage.list_ciphertexts().await
    }

    /// List ciphertexts by public key ID.
    pub async fn list_ciphertexts_by_public_key(
        &self,
        public_key_id: &FhePublicKeyId,
    ) -> FheResult<Vec<FheCiphertext>> {
        self.storage
            .list_ciphertexts_by_public_key(public_key_id)
            .await
    }

    /// Delete a ciphertext by ID.
    pub async fn delete_ciphertext(&self, id: &FheCiphertextId) -> FheResult<()> {
        info!("Deleting ciphertext: {}", id);
        self.storage.delete_ciphertext(id).await
    }

    /// Get information about all registered schemes.
    pub fn get_schemes_info(&self) -> Value {
        let mut schemes_info = serde_json::Map::new();
        for (scheme_type, scheme) in &self.schemes {
            schemes_info.insert(scheme_type.to_string(), scheme.get_info());
        }
        serde_json::Value::Object(schemes_info)
    }

    /// Get the default scheme type.
    pub fn get_default_scheme_type(&self) -> FheResult<FheSchemeType> {
        if let Some(default_scheme) = &self.config.service.default_scheme {
            match default_scheme.as_str() {
                "TFHE" => Ok(FheSchemeType::Tfhe),
                "OpenFHE" => Ok(FheSchemeType::OpenFhe),
                "SEAL" => Ok(FheSchemeType::Seal),
                "HElib" => Ok(FheSchemeType::Helib),
                "Lattigo" => Ok(FheSchemeType::Lattigo),
                _ => Err(FheError::ConfigurationError(format!(
                    "Unknown default scheme: {}",
                    default_scheme
                ))),
            }
        } else if !self.schemes.is_empty() {
            // If no default is specified, use the first available scheme
            Ok(*self.schemes.keys().next().unwrap())
        } else {
            Err(FheError::ConfigurationError(
                "No default scheme specified and no schemes available".into(),
            ))
        }
    }
}
