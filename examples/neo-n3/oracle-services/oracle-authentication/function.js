/**
 * Neo N3 Oracle Authentication Example
 * 
 * This function demonstrates how to implement secure authentication for oracle services
 * in the Neo N3 FaaS platform. It includes multiple authentication methods, role-based
 * access control, and access monitoring and auditing.
 */

// Import the Neo and Oracle modules from the r3e runtime
import { neo } from 'r3e';
import { oracle } from 'r3e';
import { runlog } from 'r3e';
import { crypto } from 'r3e';

// Authentication methods
const AUTH_METHODS = {
  API_KEY: 'api_key',
  JWT: 'jwt',
  BLOCKCHAIN: 'blockchain'
};

// Permission types
const PERMISSIONS = {
  READ_ALL: 'read_all',
  WRITE_ALL: 'write_all',
  READ_PRICE_DATA: 'read_price_data',
  READ_WEATHER_DATA: 'read_weather_data',
  READ_SPORTS_DATA: 'read_sports_data'
};

/**
 * Main handler function for the oracle authentication service
 */
export async function handler(event, context) {
  try {
    runlog.info('Oracle Authentication Service function triggered');
    
    // Get configuration from context
    const config = context.config.oracle.auth;
    const rbacConfig = context.config.oracle.rbac;
    const auditingConfig = context.config.oracle.auditing;
    
    // Parse request
    const request = parseRequest(event);
    
    // Authenticate the request
    const authResult = await authenticateRequest(request, config);
    
    // If authentication failed, return error
    if (!authResult.success) {
      // Log failed authentication attempt
      if (auditingConfig.enabled && auditingConfig.log_failure) {
        await logAccessAttempt({
          userId: authResult.userId || 'unknown',
          resource: request.resource,
          action: request.action,
          method: authResult.method,
          success: false,
          reason: authResult.error,
          timestamp: Date.now()
        }, context);
      }
      
      // Check for suspicious activity
      if (auditingConfig.enabled && auditingConfig.alert_on_suspicious) {
        await checkSuspiciousActivity(authResult.userId || request.ip, auditingConfig, context);
      }
      
      return {
        status: 'error',
        error: 'Authentication failed',
        message: authResult.error
      };
    }
    
    // Check permissions
    const permissionResult = await checkPermissions(authResult.userId, request.resource, request.action, rbacConfig);
    
    // If permission check failed, return error
    if (!permissionResult.success) {
      // Log failed permission attempt
      if (auditingConfig.enabled && auditingConfig.log_failure) {
        await logAccessAttempt({
          userId: authResult.userId,
          resource: request.resource,
          action: request.action,
          method: authResult.method,
          success: false,
          reason: permissionResult.error,
          timestamp: Date.now()
        }, context);
      }
      
      return {
        status: 'error',
        error: 'Permission denied',
        message: permissionResult.error
      };
    }
    
    // Log successful access
    if (auditingConfig.enabled && auditingConfig.log_success) {
      await logAccessAttempt({
        userId: authResult.userId,
        resource: request.resource,
        action: request.action,
        method: authResult.method,
        success: true,
        timestamp: Date.now()
      }, context);
    }
    
    // Process the request
    const result = await processRequest(request, authResult.userId);
    
    // Return the result
    return {
      status: 'success',
      userId: authResult.userId,
      method: authResult.method,
      timestamp: Date.now(),
      data: result
    };
    
  } catch (error) {
    // Log any errors
    runlog.error('Error in oracle authentication service:', error);
    
    // Return error information
    return {
      status: 'error',
      message: `Error in oracle authentication service: ${error.message}`,
      error: error.stack
    };
  }
}

/**
 * Parse the request from the event
 */
function parseRequest(event) {
  // Default request
  const defaultRequest = {
    resource: 'oracle',
    action: 'read',
    data: {},
    headers: {},
    ip: '0.0.0.0'
  };
  
  // If event is HTTP request
  if (event.type === 'http') {
    return {
      resource: event.path.split('/')[2] || 'oracle',
      action: event.method === 'GET' ? 'read' : 'write',
      data: event.body || {},
      headers: event.headers || {},
      ip: event.ip || '0.0.0.0'
    };
  }
  
  // If event is direct invocation
  return { ...defaultRequest, ...event.data };
}

/**
 * Authenticate the request using the configured methods
 */
async function authenticateRequest(request, config) {
  // Try each enabled authentication method
  
  // API Key Authentication
  if (config.api_key && config.api_key.enabled) {
    const apiKeyResult = await authenticateWithApiKey(request, config.api_key);
    if (apiKeyResult.success) {
      return { ...apiKeyResult, method: AUTH_METHODS.API_KEY };
    }
  }
  
  // JWT Authentication
  if (config.jwt && config.jwt.enabled) {
    const jwtResult = await authenticateWithJwt(request, config.jwt);
    if (jwtResult.success) {
      return { ...jwtResult, method: AUTH_METHODS.JWT };
    }
  }
  
  // Blockchain Authentication
  if (config.blockchain && config.blockchain.enabled) {
    const blockchainResult = await authenticateWithBlockchain(request, config.blockchain);
    if (blockchainResult.success) {
      return { ...blockchainResult, method: AUTH_METHODS.BLOCKCHAIN };
    }
  }
  
  // If all authentication methods failed, return error
  return {
    success: false,
    error: 'Authentication failed for all configured methods'
  };
}

/**
 * Authenticate with API Key
 */
async function authenticateWithApiKey(request, config) {
  try {
    // Get API key from request headers
    const apiKey = request.headers[config.header.toLowerCase()];
    
    // If no API key provided, return error
    if (!apiKey) {
      return {
        success: false,
        error: `API key not provided in header: ${config.header}`
      };
    }
    
    // In a real implementation, this would validate the API key against a database
    // For this example, we'll use a mock validation
    const isValid = await mockValidateApiKey(apiKey);
    
    if (!isValid) {
      return {
        success: false,
        error: 'Invalid API key'
      };
    }
    
    // Get user ID from API key
    const userId = await getUserIdFromApiKey(apiKey);
    
    return {
      success: true,
      userId: userId
    };
  } catch (error) {
    return {
      success: false,
      error: `API key authentication error: ${error.message}`
    };
  }
}

/**
 * Authenticate with JWT token
 */
async function authenticateWithJwt(request, config) {
  try {
    // Get JWT token from request headers
    const authHeader = request.headers[config.header.toLowerCase()];
    
    // If no auth header provided, return error
    if (!authHeader) {
      return {
        success: false,
        error: `JWT token not provided in header: ${config.header}`
      };
    }
    
    // Extract token from header
    const token = extractTokenFromHeader(authHeader, config.header_format);
    
    // If no token extracted, return error
    if (!token) {
      return {
        success: false,
        error: 'JWT token not found in authorization header'
      };
    }
    
    // In a real implementation, this would verify the JWT token
    // For this example, we'll use a mock verification
    const decoded = await mockVerifyJwtToken(token, config);
    
    if (!decoded) {
      return {
        success: false,
        error: 'Invalid JWT token'
      };
    }
    
    return {
      success: true,
      userId: decoded.sub
    };
  } catch (error) {
    return {
      success: false,
      error: `JWT authentication error: ${error.message}`
    };
  }
}

/**
 * Authenticate with blockchain signature
 */
async function authenticateWithBlockchain(request, config) {
  try {
    // Get signature, public key, and timestamp from request headers
    const signature = request.headers[config.signature_header.toLowerCase()];
    const publicKey = request.headers[config.public_key_header.toLowerCase()];
    const timestamp = request.headers[config.timestamp_header.toLowerCase()];
    
    // If any required header is missing, return error
    if (!signature || !publicKey || !timestamp) {
      return {
        success: false,
        error: 'Missing required blockchain authentication headers'
      };
    }
    
    // Check timestamp to prevent replay attacks
    const timestampAge = Date.now() - parseInt(timestamp);
    if (timestampAge > config.max_timestamp_age * 1000) {
      return {
        success: false,
        error: 'Timestamp too old, possible replay attack'
      };
    }
    
    // Create message to verify
    const message = createMessageToVerify(request, timestamp);
    
    // In a real implementation, this would verify the signature using Neo cryptography
    // For this example, we'll use a mock verification
    const isValid = await mockVerifyBlockchainSignature(message, signature, publicKey);
    
    if (!isValid) {
      return {
        success: false,
        error: 'Invalid blockchain signature'
      };
    }
    
    // Get user ID from public key
    const userId = await getUserIdFromPublicKey(publicKey);
    
    return {
      success: true,
      userId: userId
    };
  } catch (error) {
    return {
      success: false,
      error: `Blockchain authentication error: ${error.message}`
    };
  }
}

/**
 * Check if the user has the required permissions
 */
async function checkPermissions(userId, resource, action, rbacConfig) {
  try {
    // If RBAC is not enabled, allow all access
    if (!rbacConfig || !rbacConfig.enabled) {
      return {
        success: true
      };
    }
    
    // Get user roles
    const userRoles = await getUserRoles(userId, rbacConfig);
    
    // If no roles found, deny access
    if (!userRoles || userRoles.length === 0) {
      return {
        success: false,
        error: 'User has no assigned roles'
      };
    }
    
    // Get required permission for the resource and action
    const requiredPermission = getRequiredPermission(resource, action);
    
    // Check if any of the user's roles has the required permission
    const hasPermission = await checkRolePermissions(userRoles, requiredPermission, rbacConfig);
    
    if (!hasPermission) {
      return {
        success: false,
        error: `User does not have the required permission: ${requiredPermission}`
      };
    }
    
    return {
      success: true
    };
  } catch (error) {
    return {
      success: false,
      error: `Permission check error: ${error.message}`
    };
  }
}

/**
 * Process the authenticated request
 */
async function processRequest(request, userId) {
  // In a real implementation, this would process the request based on the resource and action
  // For this example, we'll return mock data
  
  switch (request.resource) {
    case 'price':
      return {
        prices: [
          { symbol: 'NEO', price: 42.5 },
          { symbol: 'GAS', price: 12.3 },
          { symbol: 'BTC', price: 50000 }
        ]
      };
    
    case 'weather':
      return {
        location: 'New York',
        temperature: 22.5,
        condition: 'partly cloudy'
      };
    
    case 'sports':
      return {
        games: [
          { home: 'Lakers', away: 'Warriors', score: '105-98' },
          { home: 'Celtics', away: 'Nets', score: '112-104' }
        ]
      };
    
    default:
      return {
        message: `Authenticated request processed for user: ${userId}`
      };
  }
}

/**
 * Log access attempt
 */
async function logAccessAttempt(accessData, context) {
  try {
    // Create a unique key for this access log
    const key = `access:${Date.now()}:${Math.random().toString(36).substring(2, 15)}`;
    
    // Store the access log
    await context.store.set(key, JSON.stringify(accessData));
    
    // Update the access log index
    const indexKey = 'access:index';
    const indexJson = await context.store.get(indexKey) || '[]';
    const index = JSON.parse(indexJson);
    
    // Add the new key to the index
    index.push(key);
    
    // Keep only the last 1000 entries
    if (index.length > 1000) {
      index.shift();
    }
    
    // Save the updated index
    await context.store.set(indexKey, JSON.stringify(index));
    
    runlog.info('Access attempt logged:', accessData);
  } catch (error) {
    runlog.error('Error logging access attempt:', error);
  }
}

/**
 * Check for suspicious activity
 */
async function checkSuspiciousActivity(userId, auditingConfig, context) {
  try {
    // Get recent access logs for this user
    const recentLogs = await getRecentAccessLogs(userId, context);
    
    // Count failed attempts
    const failedAttempts = recentLogs.filter(log => 
      !log.success && 
      (Date.now() - log.timestamp) < auditingConfig.suspicious_rules.failed_attempts_window * 1000
    );
    
    // If too many failed attempts, alert
    if (failedAttempts.length >= auditingConfig.suspicious_rules.max_failed_attempts) {
      await alertSuspiciousActivity({
        userId: userId,
        type: 'too_many_failed_attempts',
        count: failedAttempts.length,
        window: auditingConfig.suspicious_rules.failed_attempts_window,
        timestamp: Date.now()
      }, context);
    }
    
    // Count requests per minute
    const requestsLastMinute = recentLogs.filter(log => 
      (Date.now() - log.timestamp) < 60000
    );
    
    // If too many requests per minute, alert
    if (requestsLastMinute.length >= auditingConfig.suspicious_rules.max_requests_per_minute) {
      await alertSuspiciousActivity({
        userId: userId,
        type: 'too_many_requests',
        count: requestsLastMinute.length,
        window: 60,
        timestamp: Date.now()
      }, context);
    }
  } catch (error) {
    runlog.error('Error checking suspicious activity:', error);
  }
}

/**
 * Alert suspicious activity
 */
async function alertSuspiciousActivity(alertData, context) {
  try {
    // Create a unique key for this alert
    const key = `alert:${Date.now()}:${Math.random().toString(36).substring(2, 15)}`;
    
    // Store the alert
    await context.store.set(key, JSON.stringify(alertData));
    
    // Update the alert index
    const indexKey = 'alert:index';
    const indexJson = await context.store.get(indexKey) || '[]';
    const index = JSON.parse(indexJson);
    
    // Add the new key to the index
    index.push(key);
    
    // Keep only the last 100 entries
    if (index.length > 100) {
      index.shift();
    }
    
    // Save the updated index
    await context.store.set(indexKey, JSON.stringify(index));
    
    runlog.warn('Suspicious activity detected:', alertData);
  } catch (error) {
    runlog.error('Error alerting suspicious activity:', error);
  }
}

/**
 * Get recent access logs for a user
 */
async function getRecentAccessLogs(userId, context) {
  try {
    // Get access log index
    const indexKey = 'access:index';
    const indexJson = await context.store.get(indexKey) || '[]';
    const index = JSON.parse(indexJson);
    
    // Get recent logs
    const logs = [];
    
    for (const key of index) {
      const logJson = await context.store.get(key);
      if (logJson) {
        const log = JSON.parse(logJson);
        if (log.userId === userId) {
          logs.push(log);
        }
      }
    }
    
    return logs;
  } catch (error) {
    runlog.error('Error getting recent access logs:', error);
    return [];
  }
}

/**
 * Extract token from authorization header
 */
function extractTokenFromHeader(header, format) {
  // If format is 'Bearer {token}'
  if (format === 'Bearer {token}') {
    const parts = header.split(' ');
    if (parts.length === 2 && parts[0] === 'Bearer') {
      return parts[1];
    }
  }
  
  // If format is just the token
  if (format === '{token}') {
    return header;
  }
  
  return null;
}

/**
 * Create message to verify for blockchain authentication
 */
function createMessageToVerify(request, timestamp) {
  // Create a message that includes the request details and timestamp
  const message = {
    resource: request.resource,
    action: request.action,
    timestamp: timestamp,
    data: request.data
  };
  
  return JSON.stringify(message);
}

/**
 * Get required permission for a resource and action
 */
function getRequiredPermission(resource, action) {
  // If action is read
  if (action === 'read') {
    switch (resource) {
      case 'price':
        return PERMISSIONS.READ_PRICE_DATA;
      case 'weather':
        return PERMISSIONS.READ_WEATHER_DATA;
      case 'sports':
        return PERMISSIONS.READ_SPORTS_DATA;
      default:
        return PERMISSIONS.READ_ALL;
    }
  }
  
  // If action is write
  if (action === 'write') {
    return PERMISSIONS.WRITE_ALL;
  }
  
  // Default permission
  return PERMISSIONS.READ_ALL;
}

/**
 * Get user roles from RBAC configuration
 */
async function getUserRoles(userId, rbacConfig) {
  // Check user role assignments
  const userConfig = rbacConfig.users.find(user => user.id === userId);
  
  if (userConfig) {
    return userConfig.roles;
  }
  
  // Check contract role assignments (if userId is a contract hash)
  const contractConfig = rbacConfig.contracts.find(contract => contract.hash === userId);
  
  if (contractConfig) {
    return contractConfig.roles;
  }
  
  return [];
}

/**
 * Check if any of the user's roles has the required permission
 */
async function checkRolePermissions(userRoles, requiredPermission, rbacConfig) {
  // Get role definitions
  const roleDefs = rbacConfig.roles;
  
  // Check each role
  for (const roleName of userRoles) {
    const roleDef = roleDefs.find(role => role.name === roleName);
    
    if (roleDef && roleDef.permissions.includes(requiredPermission)) {
      return true;
    }
    
    // Special case: if role has READ_ALL, it can read any resource
    if (requiredPermission.startsWith('read_') && 
        roleDef && 
        roleDef.permissions.includes(PERMISSIONS.READ_ALL)) {
      return true;
    }
    
    // Special case: if role has WRITE_ALL, it can write any resource
    if (requiredPermission.startsWith('write_') && 
        roleDef && 
        roleDef.permissions.includes(PERMISSIONS.WRITE_ALL)) {
      return true;
    }
  }
  
  return false;
}

/**
 * Mock API key validation
 */
async function mockValidateApiKey(apiKey) {
  // In a real implementation, this would validate the API key against a database
  // For this example, we'll use a simple validation
  return apiKey === 'valid-api-key-123' || 
         apiKey === 'valid-api-key-456' || 
         apiKey === 'valid-api-key-789';
}

/**
 * Get user ID from API key
 */
async function getUserIdFromApiKey(apiKey) {
  // In a real implementation, this would look up the user ID from the API key
  // For this example, we'll use a simple mapping
  switch (apiKey) {
    case 'valid-api-key-123':
      return 'user1';
    case 'valid-api-key-456':
      return 'user2';
    case 'valid-api-key-789':
      return 'user3';
    default:
      return 'unknown';
  }
}

/**
 * Mock JWT token verification
 */
async function mockVerifyJwtToken(token, config) {
  // In a real implementation, this would verify the JWT token
  // For this example, we'll use a simple verification
  
  // Check if token is one of our mock tokens
  if (token === 'valid-jwt-token-user1') {
    return {
      sub: 'user1',
      iss: config.issuer,
      aud: config.audience,
      exp: Math.floor(Date.now() / 1000) + config.expiration
    };
  }
  
  if (token === 'valid-jwt-token-user2') {
    return {
      sub: 'user2',
      iss: config.issuer,
      aud: config.audience,
      exp: Math.floor(Date.now() / 1000) + config.expiration
    };
  }
  
  if (token === 'valid-jwt-token-user3') {
    return {
      sub: 'user3',
      iss: config.issuer,
      aud: config.audience,
      exp: Math.floor(Date.now() / 1000) + config.expiration
    };
  }
  
  return null;
}

/**
 * Mock blockchain signature verification
 */
async function mockVerifyBlockchainSignature(message, signature, publicKey) {
  // In a real implementation, this would verify the signature using Neo cryptography
  // For this example, we'll use a simple verification
  
  // Check if signature and public key match our mock values
  return (
    (signature === 'valid-signature-1' && publicKey === 'valid-public-key-1') ||
    (signature === 'valid-signature-2' && publicKey === 'valid-public-key-2') ||
    (signature === 'valid-signature-3' && publicKey === 'valid-public-key-3')
  );
}

/**
 * Get user ID from public key
 */
async function getUserIdFromPublicKey(publicKey) {
  // In a real implementation, this would look up the user ID from the public key
  // For this example, we'll use a simple mapping
  switch (publicKey) {
    case 'valid-public-key-1':
      return 'user1';
    case 'valid-public-key-2':
      return 'user2';
    case 'valid-public-key-3':
      return 'user3';
    default:
      return 'unknown';
  }
}
