/**
 * Authentication Library for Neo N3 Oracle Authentication Example
 * 
 * This module provides functions for implementing different authentication methods
 * for oracle services in the Neo N3 FaaS platform.
 */

// Import required modules
const crypto = require('crypto');
const jwt = require('jsonwebtoken');

/**
 * API Key Authentication
 */
class ApiKeyAuth {
  /**
   * Create a new API key authentication instance
   * @param {Object} config - Configuration for API key authentication
   */
  constructor(config) {
    this.config = config;
    this.apiKeys = new Map();
    
    // Initialize with some mock API keys
    this.apiKeys.set('valid-api-key-123', {
      userId: 'user1',
      created: Date.now(),
      expires: Date.now() + (config.rotation_days * 86400000)
    });
    
    this.apiKeys.set('valid-api-key-456', {
      userId: 'user2',
      created: Date.now(),
      expires: Date.now() + (config.rotation_days * 86400000)
    });
    
    this.apiKeys.set('valid-api-key-789', {
      userId: 'user3',
      created: Date.now(),
      expires: Date.now() + (config.rotation_days * 86400000)
    });
  }
  
  /**
   * Validate an API key
   * @param {string} apiKey - The API key to validate
   * @returns {Object|null} - The user ID if valid, null otherwise
   */
  validate(apiKey) {
    // Check if the API key exists
    if (!this.apiKeys.has(apiKey)) {
      return null;
    }
    
    // Get the API key data
    const keyData = this.apiKeys.get(apiKey);
    
    // Check if the API key has expired
    if (keyData.expires && keyData.expires < Date.now()) {
      return null;
    }
    
    // Return the user ID
    return {
      userId: keyData.userId,
      method: 'api_key'
    };
  }
  
  /**
   * Generate a new API key
   * @param {string} userId - The user ID to generate an API key for
   * @returns {string} - The generated API key
   */
  generate(userId) {
    // Generate a random API key
    const apiKey = crypto.randomBytes(32).toString('hex');
    
    // Store the API key
    this.apiKeys.set(apiKey, {
      userId: userId,
      created: Date.now(),
      expires: this.config.rotation_days ? Date.now() + (this.config.rotation_days * 86400000) : null
    });
    
    // Return the API key
    return apiKey;
  }
  
  /**
   * Revoke an API key
   * @param {string} apiKey - The API key to revoke
   * @returns {boolean} - True if the API key was revoked, false otherwise
   */
  revoke(apiKey) {
    // Check if the API key exists
    if (!this.apiKeys.has(apiKey)) {
      return false;
    }
    
    // Delete the API key
    this.apiKeys.delete(apiKey);
    
    // Return success
    return true;
  }
}

/**
 * JWT Authentication
 */
class JwtAuth {
  /**
   * Create a new JWT authentication instance
   * @param {Object} config - Configuration for JWT authentication
   */
  constructor(config) {
    this.config = config;
  }
  
  /**
   * Validate a JWT token
   * @param {string} token - The JWT token to validate
   * @returns {Object|null} - The user ID if valid, null otherwise
   */
  validate(token) {
    try {
      // Verify the JWT token
      const decoded = jwt.verify(token, this.config.secret, {
        issuer: this.config.issuer,
        audience: this.config.audience
      });
      
      // Return the user ID
      return {
        userId: decoded.sub,
        method: 'jwt'
      };
    } catch (error) {
      // If the token is invalid, return null
      return null;
    }
  }
  
  /**
   * Generate a new JWT token
   * @param {string} userId - The user ID to generate a token for
   * @returns {string} - The generated JWT token
   */
  generate(userId) {
    // Create the JWT payload
    const payload = {
      sub: userId,
      iat: Math.floor(Date.now() / 1000),
      exp: Math.floor(Date.now() / 1000) + this.config.expiration,
      iss: this.config.issuer,
      aud: this.config.audience
    };
    
    // Sign the JWT token
    const token = jwt.sign(payload, this.config.secret);
    
    // Return the token
    return token;
  }
}

/**
 * Blockchain Authentication
 */
class BlockchainAuth {
  /**
   * Create a new blockchain authentication instance
   * @param {Object} config - Configuration for blockchain authentication
   */
  constructor(config) {
    this.config = config;
    this.publicKeys = new Map();
    
    // Initialize with some mock public keys
    this.publicKeys.set('valid-public-key-1', 'user1');
    this.publicKeys.set('valid-public-key-2', 'user2');
    this.publicKeys.set('valid-public-key-3', 'user3');
  }
  
  /**
   * Validate a blockchain signature
   * @param {string} message - The message that was signed
   * @param {string} signature - The signature to validate
   * @param {string} publicKey - The public key of the signer
   * @param {number} timestamp - The timestamp of the message
   * @returns {Object|null} - The user ID if valid, null otherwise
   */
  validate(message, signature, publicKey, timestamp) {
    // Check if the timestamp is valid
    if (timestamp && (Date.now() - timestamp) > (this.config.max_timestamp_age * 1000)) {
      return null;
    }
    
    // Check if the public key is registered
    if (!this.publicKeys.has(publicKey)) {
      return null;
    }
    
    // In a real implementation, this would verify the signature using Neo cryptography
    // For this example, we'll use a simple validation
    const isValid = (
      (signature === 'valid-signature-1' && publicKey === 'valid-public-key-1') ||
      (signature === 'valid-signature-2' && publicKey === 'valid-public-key-2') ||
      (signature === 'valid-signature-3' && publicKey === 'valid-public-key-3')
    );
    
    if (!isValid) {
      return null;
    }
    
    // Return the user ID
    return {
      userId: this.publicKeys.get(publicKey),
      method: 'blockchain'
    };
  }
  
  /**
   * Register a public key
   * @param {string} publicKey - The public key to register
   * @param {string} userId - The user ID to associate with the public key
   * @returns {boolean} - True if the public key was registered, false otherwise
   */
  register(publicKey, userId) {
    // Store the public key
    this.publicKeys.set(publicKey, userId);
    
    // Return success
    return true;
  }
  
  /**
   * Unregister a public key
   * @param {string} publicKey - The public key to unregister
   * @returns {boolean} - True if the public key was unregistered, false otherwise
   */
  unregister(publicKey) {
    // Check if the public key exists
    if (!this.publicKeys.has(publicKey)) {
      return false;
    }
    
    // Delete the public key
    this.publicKeys.delete(publicKey);
    
    // Return success
    return true;
  }
}

/**
 * Role-Based Access Control
 */
class RbacAuth {
  /**
   * Create a new RBAC instance
   * @param {Object} config - Configuration for RBAC
   */
  constructor(config) {
    this.config = config;
    this.roles = new Map();
    this.userRoles = new Map();
    
    // Initialize roles
    if (config.roles) {
      for (const role of config.roles) {
        this.roles.set(role.name, role.permissions);
      }
    }
    
    // Initialize user roles
    if (config.users) {
      for (const user of config.users) {
        this.userRoles.set(user.id, user.roles);
      }
    }
    
    // Initialize contract roles
    if (config.contracts) {
      for (const contract of config.contracts) {
        this.userRoles.set(contract.hash, contract.roles);
      }
    }
  }
  
  /**
   * Check if a user has a permission
   * @param {string} userId - The user ID to check
   * @param {string} permission - The permission to check
   * @returns {boolean} - True if the user has the permission, false otherwise
   */
  hasPermission(userId, permission) {
    // Check if RBAC is enabled
    if (!this.config.enabled) {
      return true;
    }
    
    // Check if the user has any roles
    if (!this.userRoles.has(userId)) {
      return false;
    }
    
    // Get the user's roles
    const userRoles = this.userRoles.get(userId);
    
    // Check each role
    for (const roleName of userRoles) {
      // Check if the role exists
      if (!this.roles.has(roleName)) {
        continue;
      }
      
      // Get the role's permissions
      const permissions = this.roles.get(roleName);
      
      // Check if the role has the permission
      if (permissions.includes(permission)) {
        return true;
      }
      
      // Special case: if role has 'read_all', it can read any resource
      if (permission.startsWith('read_') && permissions.includes('read_all')) {
        return true;
      }
      
      // Special case: if role has 'write_all', it can write any resource
      if (permission.startsWith('write_') && permissions.includes('write_all')) {
        return true;
      }
    }
    
    // If no role has the permission, return false
    return false;
  }
  
  /**
   * Get the roles for a user
   * @param {string} userId - The user ID to get roles for
   * @returns {Array} - The user's roles
   */
  getRoles(userId) {
    // Check if the user has any roles
    if (!this.userRoles.has(userId)) {
      return [];
    }
    
    // Return the user's roles
    return this.userRoles.get(userId);
  }
  
  /**
   * Assign a role to a user
   * @param {string} userId - The user ID to assign the role to
   * @param {string} roleName - The role to assign
   * @returns {boolean} - True if the role was assigned, false otherwise
   */
  assignRole(userId, roleName) {
    // Check if the role exists
    if (!this.roles.has(roleName)) {
      return false;
    }
    
    // Get the user's roles
    let userRoles = [];
    if (this.userRoles.has(userId)) {
      userRoles = this.userRoles.get(userId);
    }
    
    // Check if the user already has the role
    if (userRoles.includes(roleName)) {
      return true;
    }
    
    // Add the role
    userRoles.push(roleName);
    
    // Update the user's roles
    this.userRoles.set(userId, userRoles);
    
    // Return success
    return true;
  }
  
  /**
   * Revoke a role from a user
   * @param {string} userId - The user ID to revoke the role from
   * @param {string} roleName - The role to revoke
   * @returns {boolean} - True if the role was revoked, false otherwise
   */
  revokeRole(userId, roleName) {
    // Check if the user has any roles
    if (!this.userRoles.has(userId)) {
      return false;
    }
    
    // Get the user's roles
    let userRoles = this.userRoles.get(userId);
    
    // Check if the user has the role
    const index = userRoles.indexOf(roleName);
    if (index === -1) {
      return false;
    }
    
    // Remove the role
    userRoles.splice(index, 1);
    
    // Update the user's roles
    this.userRoles.set(userId, userRoles);
    
    // Return success
    return true;
  }
}

// Export the authentication classes
module.exports = {
  ApiKeyAuth,
  JwtAuth,
  BlockchainAuth,
  RbacAuth
};
