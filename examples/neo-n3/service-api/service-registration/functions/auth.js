/**
 * Authentication function for the Neo N3 Service Registration Example
 * 
 * This function handles user authentication for the service and demonstrates
 * how to implement authentication in a Neo N3 FaaS service.
 */

const jwt = require('jsonwebtoken');
const crypto = require('crypto');

/**
 * Authentication handler function for the service
 * 
 * @param {Object} request - The request object containing information about the request
 * @param {Object} user - Information about the authenticated user (if any)
 * @param {Object} context - Context information about the execution environment
 * @returns {Object} Response object with status code and body
 */
async function handler(request, user, context) {
  try {
    // Log the request for debugging
    console.log('Received authentication request:', JSON.stringify(request, null, 2));
    
    // Extract request information
    const method = request.method || 'POST';
    const body = request.body || {};
    
    // Check if this is a login request
    if (method === 'POST' && body.action === 'login') {
      return handleLogin(body, context);
    }
    
    // Check if this is a token verification request
    if (method === 'POST' && body.action === 'verify') {
      return handleVerify(body, context);
    }
    
    // Check if this is a token refresh request
    if (method === 'POST' && body.action === 'refresh') {
      return handleRefresh(body, context);
    }
    
    // Check if this is a logout request
    if (method === 'POST' && body.action === 'logout') {
      return handleLogout(body, context);
    }
    
    // If no valid action is provided, return an error
    return {
      statusCode: 400,
      headers: {
        'Content-Type': 'application/json'
      },
      body: {
        error: 'Bad Request',
        message: 'Invalid action. Supported actions: login, verify, refresh, logout'
      }
    };
  } catch (error) {
    // Log the error
    console.error('Error processing authentication request:', error);
    
    // Return an error response
    return {
      statusCode: 500,
      headers: {
        'Content-Type': 'application/json'
      },
      body: {
        error: 'Internal Server Error',
        message: error.message
      }
    };
  }
}

/**
 * Handle user login
 * 
 * @param {Object} body - The request body
 * @param {Object} context - The execution context
 * @returns {Object} Response object with status code and body
 */
function handleLogin(body, context) {
  // Extract credentials from the request body
  const { username, password } = body;
  
  // Validate credentials
  if (!username || !password) {
    return {
      statusCode: 400,
      headers: {
        'Content-Type': 'application/json'
      },
      body: {
        error: 'Bad Request',
        message: 'Username and password are required'
      }
    };
  }
  
  // In a real application, you would validate the credentials against a database
  // For this example, we'll use a simple mock user database
  const user = mockValidateUser(username, password);
  
  if (!user) {
    return {
      statusCode: 401,
      headers: {
        'Content-Type': 'application/json'
      },
      body: {
        error: 'Unauthorized',
        message: 'Invalid username or password'
      }
    };
  }
  
  // Generate JWT tokens
  const { accessToken, refreshToken } = generateTokens(user, context);
  
  // Return the tokens
  return {
    statusCode: 200,
    headers: {
      'Content-Type': 'application/json'
    },
    body: {
      message: 'Login successful',
      user: {
        id: user.id,
        username: user.username,
        name: user.name,
        roles: user.roles
      },
      tokens: {
        accessToken,
        refreshToken,
        expiresIn: 3600 // 1 hour
      }
    }
  };
}

/**
 * Handle token verification
 * 
 * @param {Object} body - The request body
 * @param {Object} context - The execution context
 * @returns {Object} Response object with status code and body
 */
function handleVerify(body, context) {
  // Extract token from the request body
  const { token } = body;
  
  if (!token) {
    return {
      statusCode: 400,
      headers: {
        'Content-Type': 'application/json'
      },
      body: {
        error: 'Bad Request',
        message: 'Token is required'
      }
    };
  }
  
  try {
    // Verify the token
    const decoded = jwt.verify(token, getJwtSecret(context));
    
    // Return the decoded token
    return {
      statusCode: 200,
      headers: {
        'Content-Type': 'application/json'
      },
      body: {
        message: 'Token is valid',
        user: {
          id: decoded.sub,
          username: decoded.username,
          roles: decoded.roles
        },
        expiresAt: new Date(decoded.exp * 1000).toISOString()
      }
    };
  } catch (error) {
    // If the token is invalid, return an error
    return {
      statusCode: 401,
      headers: {
        'Content-Type': 'application/json'
      },
      body: {
        error: 'Unauthorized',
        message: 'Invalid token: ' + error.message
      }
    };
  }
}

/**
 * Handle token refresh
 * 
 * @param {Object} body - The request body
 * @param {Object} context - The execution context
 * @returns {Object} Response object with status code and body
 */
function handleRefresh(body, context) {
  // Extract refresh token from the request body
  const { refreshToken } = body;
  
  if (!refreshToken) {
    return {
      statusCode: 400,
      headers: {
        'Content-Type': 'application/json'
      },
      body: {
        error: 'Bad Request',
        message: 'Refresh token is required'
      }
    };
  }
  
  try {
    // Verify the refresh token
    const decoded = jwt.verify(refreshToken, getJwtSecret(context));
    
    // Check if the token is a refresh token
    if (!decoded.type || decoded.type !== 'refresh') {
      return {
        statusCode: 401,
        headers: {
          'Content-Type': 'application/json'
        },
        body: {
          error: 'Unauthorized',
          message: 'Invalid refresh token'
        }
      };
    }
    
    // Get the user from the decoded token
    const user = {
      id: decoded.sub,
      username: decoded.username,
      name: decoded.name,
      roles: decoded.roles
    };
    
    // Generate new tokens
    const tokens = generateTokens(user, context);
    
    // Return the new tokens
    return {
      statusCode: 200,
      headers: {
        'Content-Type': 'application/json'
      },
      body: {
        message: 'Token refreshed successfully',
        tokens: {
          accessToken: tokens.accessToken,
          refreshToken: tokens.refreshToken,
          expiresIn: 3600 // 1 hour
        }
      }
    };
  } catch (error) {
    // If the token is invalid, return an error
    return {
      statusCode: 401,
      headers: {
        'Content-Type': 'application/json'
      },
      body: {
        error: 'Unauthorized',
        message: 'Invalid refresh token: ' + error.message
      }
    };
  }
}

/**
 * Handle user logout
 * 
 * @param {Object} body - The request body
 * @param {Object} context - The execution context
 * @returns {Object} Response object with status code and body
 */
function handleLogout(body, context) {
  // In a real application, you would invalidate the token in a token blacklist
  // For this example, we'll just return a success response
  
  return {
    statusCode: 200,
    headers: {
      'Content-Type': 'application/json'
    },
    body: {
      message: 'Logout successful'
    }
  };
}

/**
 * Generate JWT tokens for a user
 * 
 * @param {Object} user - The user object
 * @param {Object} context - The execution context
 * @returns {Object} Access and refresh tokens
 */
function generateTokens(user, context) {
  const jwtSecret = getJwtSecret(context);
  
  // Generate access token
  const accessToken = jwt.sign(
    {
      sub: user.id,
      username: user.username,
      name: user.name,
      roles: user.roles,
      type: 'access'
    },
    jwtSecret,
    {
      expiresIn: '1h' // 1 hour
    }
  );
  
  // Generate refresh token
  const refreshToken = jwt.sign(
    {
      sub: user.id,
      username: user.username,
      name: user.name,
      roles: user.roles,
      type: 'refresh'
    },
    jwtSecret,
    {
      expiresIn: '7d' // 7 days
    }
  );
  
  return { accessToken, refreshToken };
}

/**
 * Get the JWT secret from the context
 * 
 * @param {Object} context - The execution context
 * @returns {string} The JWT secret
 */
function getJwtSecret(context) {
  // In a real application, you would get the JWT secret from a secure source
  // For this example, we'll use a mock secret
  return context?.env?.JWT_SECRET || 'example-jwt-secret-for-neo-n3-faas-service';
}

/**
 * Mock function to validate user credentials
 * 
 * @param {string} username - The username
 * @param {string} password - The password
 * @returns {Object|null} The user object if valid, null otherwise
 */
function mockValidateUser(username, password) {
  // In a real application, you would validate the credentials against a database
  // For this example, we'll use a simple mock user database
  const mockUsers = [
    {
      id: '1',
      username: 'admin',
      password: hashPassword('admin123'),
      name: 'Admin User',
      roles: ['admin', 'user']
    },
    {
      id: '2',
      username: 'user',
      password: hashPassword('user123'),
      name: 'Regular User',
      roles: ['user']
    }
  ];
  
  // Find the user by username
  const user = mockUsers.find(u => u.username === username);
  
  // If user not found or password doesn't match, return null
  if (!user || !verifyPassword(password, user.password)) {
    return null;
  }
  
  // Return the user without the password
  const { password: _, ...userWithoutPassword } = user;
  return userWithoutPassword;
}

/**
 * Hash a password
 * 
 * @param {string} password - The password to hash
 * @returns {string} The hashed password
 */
function hashPassword(password) {
  // In a real application, you would use a proper password hashing algorithm
  // For this example, we'll use a simple hash
  return crypto.createHash('sha256').update(password).digest('hex');
}

/**
 * Verify a password against a hash
 * 
 * @param {string} password - The password to verify
 * @param {string} hash - The hash to verify against
 * @returns {boolean} True if the password matches the hash, false otherwise
 */
function verifyPassword(password, hash) {
  // In a real application, you would use a proper password verification
  // For this example, we'll use a simple hash comparison
  return hashPassword(password) === hash;
}

// Export the handler function
module.exports = { handler };
