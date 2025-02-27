/**
 * Key Management Library for Neo N3 TEE Services
 * 
 * This library provides secure key management functions using Trusted Execution Environments (TEEs).
 */

// Import the TEE module from the r3e runtime
import { tee } from 'r3e';

/**
 * Key Manager class for handling cryptographic keys within a TEE
 */
export class KeyManager {
  /**
   * Initialize the key manager
   * @param {Object} config - Configuration options
   */
  constructor(config = {}) {
    this.config = {
      provider: config.provider || 'sgx',
      keyTypes: config.keyTypes || {
        asymmetric: ['secp256r1', 'secp256k1', 'ed25519'],
        symmetric: ['aes-256-gcm', 'chacha20-poly1305']
      },
      policies: config.policies || {
        rotation: {
          enabled: true,
          period: 90, // days
          overlap: 7  // days
        },
        usage: {
          maxOperations: 10000,
          maxAge: 365 // days
        },
        export: {
          allowed: false,
          publicAllowed: true
        }
      }
    };
    
    this.initialized = false;
  }
  
  /**
   * Initialize the TEE environment
   * @returns {Promise<boolean>} - True if initialization was successful
   */
  async initialize() {
    if (this.initialized) {
      return true;
    }
    
    try {
      // Verify the TEE environment through remote attestation
      const attestationResult = await this.verifyTEE();
      
      if (!attestationResult.verified) {
        throw new Error(`TEE attestation failed: ${attestationResult.reason}`);
      }
      
      this.initialized = true;
      return true;
    } catch (error) {
      console.error('Failed to initialize TEE environment:', error);
      throw error;
    }
  }
  
  /**
   * Verify the TEE environment through remote attestation
   * @returns {Promise<Object>} - Attestation result
   */
  async verifyTEE() {
    try {
      // Generate attestation report
      const report = await tee.attestation.generateReport();
      
      // Verify the report
      const verificationResult = await tee.attestation.verifyReport(report);
      
      return {
        verified: verificationResult.verified,
        report: report,
        details: verificationResult
      };
    } catch (error) {
      console.error('TEE attestation failed:', error);
      return {
        verified: false,
        reason: error.message,
        details: error
      };
    }
  }
  
  /**
   * Generate a new key pair within the TEE
   * @param {Object} options - Key generation options
   * @returns {Promise<Object>} - Key metadata
   */
  async generateKeyPair(options = {}) {
    await this.ensureInitialized();
    
    const algorithm = options.algorithm || 'secp256r1';
    const extractable = options.extractable !== undefined ? options.extractable : false;
    const usages = options.usages || ['sign', 'verify'];
    const userId = options.userId;
    const policy = options.policy || 'default';
    
    try {
      // Validate the algorithm
      if (!this.config.keyTypes.asymmetric.includes(algorithm)) {
        throw new Error(`Unsupported algorithm: ${algorithm}`);
      }
      
      // Generate the key pair within the TEE
      const keyPair = await tee.crypto.generateKeyPair({
        name: this.mapAlgorithmToWebCrypto(algorithm),
        extractable: extractable,
        usages: usages
      });
      
      // Store the private key in the TEE's secure storage
      const keyId = await tee.storage.storeKey(keyPair.privateKey);
      
      // Export the public key (if allowed)
      let publicKey = null;
      if (this.config.policies.export.publicAllowed) {
        publicKey = await tee.crypto.exportKey('spki', keyPair.publicKey);
      }
      
      // Create key metadata
      const metadata = {
        id: keyId,
        type: 'asymmetric',
        algorithm: algorithm,
        created: Date.now(),
        owner: userId,
        usageCount: 0,
        publicKey: publicKey ? this.bufferToHex(publicKey) : null,
        policies: this.getPolicyForKey(policy, algorithm)
      };
      
      // Store the metadata
      await tee.storage.storeMetadata(keyId, metadata);
      
      // Schedule key rotation if enabled
      if (metadata.policies.rotation.enabled) {
        this.scheduleKeyRotation(keyId, metadata);
      }
      
      return {
        keyId: keyId,
        algorithm: algorithm,
        publicKey: publicKey ? this.bufferToHex(publicKey) : null,
        created: metadata.created
      };
    } catch (error) {
      console.error('Failed to generate key pair:', error);
      throw error;
    }
  }
  
  /**
   * Generate a new symmetric key within the TEE
   * @param {Object} options - Key generation options
   * @returns {Promise<Object>} - Key metadata
   */
  async generateSymmetricKey(options = {}) {
    await this.ensureInitialized();
    
    const algorithm = options.algorithm || 'aes-256-gcm';
    const extractable = options.extractable !== undefined ? options.extractable : false;
    const usages = options.usages || ['encrypt', 'decrypt'];
    const userId = options.userId;
    const policy = options.policy || 'default';
    
    try {
      // Validate the algorithm
      if (!this.config.keyTypes.symmetric.includes(algorithm)) {
        throw new Error(`Unsupported algorithm: ${algorithm}`);
      }
      
      // Generate the symmetric key within the TEE
      const key = await tee.crypto.generateKey({
        name: this.mapAlgorithmToWebCrypto(algorithm),
        length: this.getKeyLength(algorithm),
        extractable: extractable,
        usages: usages
      });
      
      // Store the key in the TEE's secure storage
      const keyId = await tee.storage.storeKey(key);
      
      // Create key metadata
      const metadata = {
        id: keyId,
        type: 'symmetric',
        algorithm: algorithm,
        created: Date.now(),
        owner: userId,
        usageCount: 0,
        policies: this.getPolicyForKey(policy, algorithm)
      };
      
      // Store the metadata
      await tee.storage.storeMetadata(keyId, metadata);
      
      // Schedule key rotation if enabled
      if (metadata.policies.rotation.enabled) {
        this.scheduleKeyRotation(keyId, metadata);
      }
      
      return {
        keyId: keyId,
        algorithm: algorithm,
        created: metadata.created
      };
    } catch (error) {
      console.error('Failed to generate symmetric key:', error);
      throw error;
    }
  }
  
  /**
   * Sign data using a key stored in the TEE
   * @param {Object} options - Signing options
   * @returns {Promise<Object>} - Signature result
   */
  async sign(options = {}) {
    await this.ensureInitialized();
    
    const keyId = options.keyId;
    const data = options.data;
    const algorithm = options.algorithm || 'ECDSA';
    const userId = options.userId;
    
    try {
      // Check access to the key
      await this.checkAccess(userId, keyId, 'sign');
      
      // Get the key metadata
      const metadata = await tee.storage.getMetadata(keyId);
      
      // Prepare the data for signing
      const dataBuffer = typeof data === 'string' ? new TextEncoder().encode(data) : data;
      
      // Sign the data within the TEE
      const signature = await tee.crypto.sign(
        { name: algorithm, hash: 'SHA-256' },
        keyId,
        dataBuffer
      );
      
      // Update usage count
      metadata.usageCount += 1;
      await tee.storage.storeMetadata(keyId, metadata);
      
      // Log the operation
      await this.logOperation('sign', keyId, userId);
      
      return {
        signature: this.bufferToHex(signature),
        algorithm: algorithm,
        keyId: keyId
      };
    } catch (error) {
      console.error('Failed to sign data:', error);
      throw error;
    }
  }
  
  /**
   * Verify a signature using a key stored in the TEE
   * @param {Object} options - Verification options
   * @returns {Promise<boolean>} - True if the signature is valid
   */
  async verify(options = {}) {
    await this.ensureInitialized();
    
    const keyId = options.keyId;
    const data = options.data;
    const signature = options.signature;
    const algorithm = options.algorithm || 'ECDSA';
    const userId = options.userId;
    
    try {
      // Check access to the key
      await this.checkAccess(userId, keyId, 'verify');
      
      // Get the key metadata
      const metadata = await tee.storage.getMetadata(keyId);
      
      // Prepare the data and signature for verification
      const dataBuffer = typeof data === 'string' ? new TextEncoder().encode(data) : data;
      const signatureBuffer = typeof signature === 'string' ? this.hexToBuffer(signature) : signature;
      
      // Verify the signature within the TEE
      const isValid = await tee.crypto.verify(
        { name: algorithm, hash: 'SHA-256' },
        keyId,
        signatureBuffer,
        dataBuffer
      );
      
      // Update usage count
      metadata.usageCount += 1;
      await tee.storage.storeMetadata(keyId, metadata);
      
      // Log the operation
      await this.logOperation('verify', keyId, userId);
      
      return isValid;
    } catch (error) {
      console.error('Failed to verify signature:', error);
      throw error;
    }
  }
  
  /**
   * Encrypt data using a key stored in the TEE
   * @param {Object} options - Encryption options
   * @returns {Promise<Object>} - Encrypted data
   */
  async encrypt(options = {}) {
    await this.ensureInitialized();
    
    const keyId = options.keyId;
    const data = options.data;
    const algorithm = options.algorithm || 'AES-GCM';
    const userId = options.userId;
    
    try {
      // Check access to the key
      await this.checkAccess(userId, keyId, 'encrypt');
      
      // Get the key metadata
      const metadata = await tee.storage.getMetadata(keyId);
      
      // Prepare the data for encryption
      const dataBuffer = typeof data === 'string' ? new TextEncoder().encode(data) : data;
      
      // Generate IV for AES-GCM
      const iv = crypto.getRandomValues(new Uint8Array(12));
      
      // Encrypt the data within the TEE
      const encryptedData = await tee.crypto.encrypt(
        { name: algorithm, iv: iv },
        keyId,
        dataBuffer
      );
      
      // Update usage count
      metadata.usageCount += 1;
      await tee.storage.storeMetadata(keyId, metadata);
      
      // Log the operation
      await this.logOperation('encrypt', keyId, userId);
      
      return {
        ciphertext: this.bufferToHex(encryptedData),
        iv: this.bufferToHex(iv),
        algorithm: algorithm,
        keyId: keyId
      };
    } catch (error) {
      console.error('Failed to encrypt data:', error);
      throw error;
    }
  }
  
  /**
   * Decrypt data using a key stored in the TEE
   * @param {Object} options - Decryption options
   * @returns {Promise<Object>} - Decrypted data
   */
  async decrypt(options = {}) {
    await this.ensureInitialized();
    
    const keyId = options.keyId;
    const ciphertext = options.ciphertext;
    const iv = options.iv;
    const algorithm = options.algorithm || 'AES-GCM';
    const userId = options.userId;
    
    try {
      // Check access to the key
      await this.checkAccess(userId, keyId, 'decrypt');
      
      // Get the key metadata
      const metadata = await tee.storage.getMetadata(keyId);
      
      // Prepare the data for decryption
      const ciphertextBuffer = typeof ciphertext === 'string' ? this.hexToBuffer(ciphertext) : ciphertext;
      const ivBuffer = typeof iv === 'string' ? this.hexToBuffer(iv) : iv;
      
      // Decrypt the data within the TEE
      const decryptedData = await tee.crypto.decrypt(
        { name: algorithm, iv: ivBuffer },
        keyId,
        ciphertextBuffer
      );
      
      // Update usage count
      metadata.usageCount += 1;
      await tee.storage.storeMetadata(keyId, metadata);
      
      // Log the operation
      await this.logOperation('decrypt', keyId, userId);
      
      return {
        plaintext: new TextDecoder().decode(decryptedData),
        keyId: keyId
      };
    } catch (error) {
      console.error('Failed to decrypt data:', error);
      throw error;
    }
  }
  
  /**
   * Rotate a key stored in the TEE
   * @param {Object} options - Key rotation options
   * @returns {Promise<Object>} - New key metadata
   */
  async rotateKey(options = {}) {
    await this.ensureInitialized();
    
    const keyId = options.keyId;
    const userId = options.userId;
    
    try {
      // Check access to the key
      await this.checkAccess(userId, keyId, 'rotate');
      
      // Get the key metadata
      const metadata = await tee.storage.getMetadata(keyId);
      
      // Generate a new key with the same parameters
      let newKeyMetadata;
      if (metadata.type === 'asymmetric') {
        newKeyMetadata = await this.generateKeyPair({
          algorithm: metadata.algorithm,
          extractable: false,
          usages: ['sign', 'verify'],
          userId: metadata.owner,
          policy: 'default'
        });
      } else {
        newKeyMetadata = await this.generateSymmetricKey({
          algorithm: metadata.algorithm,
          extractable: false,
          usages: ['encrypt', 'decrypt'],
          userId: metadata.owner,
          policy: 'default'
        });
      }
      
      // Update the old key metadata to mark it as rotated
      metadata.rotated = true;
      metadata.rotatedTo = newKeyMetadata.keyId;
      metadata.rotatedAt = Date.now();
      
      // Calculate expiration date based on overlap period
      const overlapPeriod = metadata.policies.rotation.overlap * 24 * 60 * 60 * 1000; // Convert days to milliseconds
      metadata.expiresAt = metadata.rotatedAt + overlapPeriod;
      
      await tee.storage.storeMetadata(keyId, metadata);
      
      // Log the operation
      await this.logOperation('rotate', keyId, userId, { newKeyId: newKeyMetadata.keyId });
      
      return {
        oldKeyId: keyId,
        newKeyId: newKeyMetadata.keyId,
        algorithm: metadata.algorithm,
        rotatedAt: metadata.rotatedAt,
        expiresAt: metadata.expiresAt
      };
    } catch (error) {
      console.error('Failed to rotate key:', error);
      throw error;
    }
  }
  
  /**
   * Delete a key stored in the TEE
   * @param {Object} options - Key deletion options
   * @returns {Promise<boolean>} - True if the key was deleted
   */
  async deleteKey(options = {}) {
    await this.ensureInitialized();
    
    const keyId = options.keyId;
    const userId = options.userId;
    const force = options.force || false;
    
    try {
      // Check access to the key
      await this.checkAccess(userId, keyId, 'delete');
      
      // Get the key metadata
      const metadata = await tee.storage.getMetadata(keyId);
      
      // Check if the key is still in use
      if (!force && metadata.rotated && Date.now() < metadata.expiresAt) {
        throw new Error(`Key is still in overlap period and cannot be deleted until ${new Date(metadata.expiresAt)}`);
      }
      
      // Delete the key from the TEE's secure storage
      await tee.storage.deleteKey(keyId);
      
      // Delete the metadata
      await tee.storage.deleteMetadata(keyId);
      
      // Log the operation
      await this.logOperation('delete', keyId, userId);
      
      return true;
    } catch (error) {
      console.error('Failed to delete key:', error);
      throw error;
    }
  }
  
  /**
   * List keys stored in the TEE
   * @param {Object} options - Key listing options
   * @returns {Promise<Array>} - List of key metadata
   */
  async listKeys(options = {}) {
    await this.ensureInitialized();
    
    const userId = options.userId;
    const includeRotated = options.includeRotated || false;
    const includeExpired = options.includeExpired || false;
    
    try {
      // Get all key IDs
      const keyIds = await tee.storage.listKeys();
      
      // Filter keys based on user access and options
      const keys = [];
      for (const keyId of keyIds) {
        try {
          // Get the key metadata
          const metadata = await tee.storage.getMetadata(keyId);
          
          // Check if the user has access to the key
          if (metadata.owner !== userId && !metadata.authorizedUsers?.includes(userId)) {
            continue;
          }
          
          // Check if the key is rotated
          if (!includeRotated && metadata.rotated) {
            continue;
          }
          
          // Check if the key is expired
          if (!includeExpired && metadata.expiresAt && Date.now() > metadata.expiresAt) {
            continue;
          }
          
          // Add the key to the list
          keys.push({
            id: keyId,
            type: metadata.type,
            algorithm: metadata.algorithm,
            created: metadata.created,
            owner: metadata.owner,
            usageCount: metadata.usageCount,
            rotated: metadata.rotated || false,
            rotatedTo: metadata.rotatedTo,
            rotatedAt: metadata.rotatedAt,
            expiresAt: metadata.expiresAt,
            publicKey: metadata.publicKey
          });
        } catch (error) {
          console.warn(`Failed to get metadata for key ${keyId}:`, error);
        }
      }
      
      return keys;
    } catch (error) {
      console.error('Failed to list keys:', error);
      throw error;
    }
  }
  
  /**
   * Check if a user has access to a key for a specific operation
   * @param {string} userId - User ID
   * @param {string} keyId - Key ID
   * @param {string} operation - Operation to check
   * @returns {Promise<boolean>} - True if the user has access
   */
  async checkAccess(userId, keyId, operation) {
    try {
      // Get the key metadata
      const metadata = await tee.storage.getMetadata(keyId);
      
      // Check if the key exists
      if (!metadata) {
        throw new Error(`Key ${keyId} not found`);
      }
      
      // Check if the user is authorized
      if (metadata.owner !== userId && !metadata.authorizedUsers?.includes(userId)) {
        throw new Error(`User ${userId} is not authorized to access key ${keyId}`);
      }
      
      // Check if the key is expired
      if (metadata.expiresAt && Date.now() > metadata.expiresAt) {
        throw new Error(`Key ${keyId} has expired`);
      }
      
      // Check if the key has reached its maximum usage
      if (metadata.usageCount >= metadata.policies.usage.maxOperations) {
        throw new Error(`Key ${keyId} has reached its maximum usage limit`);
      }
      
      // Check if the key has reached its maximum age
      const maxAge = metadata.policies.usage.maxAge * 24 * 60 * 60 * 1000; // Convert days to milliseconds
      if (Date.now() - metadata.created > maxAge) {
        throw new Error(`Key ${keyId} has reached its maximum age`);
      }
      
      return true;
    } catch (error) {
      console.error('Access check failed:', error);
      throw error;
    }
  }
  
  /**
   * Schedule key rotation based on policy
   * @param {string} keyId - Key ID
   * @param {Object} metadata - Key metadata
   */
  async scheduleKeyRotation(keyId, metadata) {
    try {
      // Calculate rotation time based on policy
      const rotationPeriod = metadata.policies.rotation.period * 24 * 60 * 60 * 1000; // Convert days to milliseconds
      const rotationTime = metadata.created + rotationPeriod;
      
      // Schedule the rotation
      console.log(`Scheduling key rotation for ${keyId} at ${new Date(rotationTime)}`);
      
      // In a real implementation, this would use a scheduler
      // For this example, we'll just log the scheduled rotation
    } catch (error) {
      console.error('Failed to schedule key rotation:', error);
    }
  }
  
  /**
   * Log a key operation
   * @param {string} operation - Operation type
   * @param {string} keyId - Key ID
   * @param {string} userId - User ID
   * @param {Object} details - Additional details
   */
  async logOperation(operation, keyId, userId, details = {}) {
    try {
      // Create the log entry
      const logEntry = {
        timestamp: Date.now(),
        operation: operation,
        keyId: keyId,
        userId: userId,
        details: details
      };
      
      // In a real implementation, this would store the log entry
      // For this example, we'll just log the operation
      console.log(`Key operation: ${operation} on key ${keyId} by user ${userId}`);
    } catch (error) {
      console.error('Failed to log operation:', error);
    }
  }
  
  /**
   * Get the policy for a key based on its type
   * @param {string} policyName - Policy name
   * @param {string} algorithm - Key algorithm
   * @returns {Object} - Key policy
   */
  getPolicyForKey(policyName, algorithm) {
    // Check if there's a custom policy for this algorithm
    const customPolicy = this.config.policies.custom?.find(policy => 
      policy.name === policyName && policy.keyTypes.includes(algorithm)
    );
    
    // Return the custom policy if found, otherwise return the default policy
    return customPolicy || this.config.policies.default;
  }
  
  /**
   * Map algorithm names to Web Crypto API algorithm names
   * @param {string} algorithm - Algorithm name
   * @returns {string} - Web Crypto API algorithm name
   */
  mapAlgorithmToWebCrypto(algorithm) {
    const mapping = {
      'secp256r1': 'ECDSA',
      'secp256k1': 'ECDSA',
      'ed25519': 'Ed25519',
      'rsa-2048': 'RSASSA-PKCS1-v1_5',
      'rsa-4096': 'RSASSA-PKCS1-v1_5',
      'aes-128-gcm': 'AES-GCM',
      'aes-256-gcm': 'AES-GCM',
      'chacha20-poly1305': 'ChaCha20-Poly1305'
    };
    
    return mapping[algorithm] || algorithm;
  }
  
  /**
   * Get the key length for a symmetric algorithm
   * @param {string} algorithm - Algorithm name
   * @returns {number} - Key length in bits
   */
  getKeyLength(algorithm) {
    const lengths = {
      'aes-128-gcm': 128,
      'aes-256-gcm': 256,
      'chacha20-poly1305': 256
    };
    
    return lengths[algorithm] || 256;
  }
  
  /**
   * Ensure that the TEE environment is initialized
   */
  async ensureInitialized() {
    if (!this.initialized) {
      await this.initialize();
    }
  }
  
  /**
   * Convert a buffer to a hexadecimal string
   * @param {ArrayBuffer} buffer - Buffer to convert
   * @returns {string} - Hexadecimal string
   */
  bufferToHex(buffer) {
    return Array.from(new Uint8Array(buffer))
      .map(b => b.toString(16).padStart(2, '0'))
      .join('');
  }
  
  /**
   * Convert a hexadecimal string to a buffer
   * @param {string} hex - Hexadecimal string
   * @returns {ArrayBuffer} - Buffer
   */
  hexToBuffer(hex) {
    const bytes = new Uint8Array(hex.length / 2);
    for (let i = 0; i < hex.length; i += 2) {
      bytes[i / 2] = parseInt(hex.substr(i, 2), 16);
    }
    return bytes.buffer;
  }
}

// Export the KeyManager class
export default KeyManager;
