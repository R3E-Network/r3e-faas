/**
 * Authentication function for the Neo N3 Service Management Example
 * 
 * This function handles authentication for the service and demonstrates
 * how to implement authentication in a Neo N3 FaaS service.
 */

const crypto = require('crypto');
const jwt = require('jsonwebtoken');

// Mock user database (in a real application, this would be stored in a database)
const users = {
  admin: {
    id: '1',
    username: 'admin',
    password: hashPassword('admin123'),
    name: 'Admin User',
    roles: ['admin', 'user']
  },
  user: {
    id: '2',
    username: 'user',
    password: hashPassword('user123'),
    name: 'Regular User',
    roles: ['user']
  }
};

// Mock token database (in a real application, this would be stored in a database)
const tokens = {};

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
    
    // Check if this is a POST request
    if (method !== 'POST') {
      return {
        statusCode: 405,
        headers: {
          'Content-Type': 'application/json',
          'Allow': 'POST'
        },
        body: {
          error: 'Method Not Allowed',
          message: `Method ${method} is not supported. Supported methods: POST`
        }
      };
    }
    
    // Extract action from the request body
    const { action } = body;
    
    // Check if the action parameter is provided
    if (!action) {
      return {
        statusCode: 400,
        headers: { 'Content-Type': 'application/json' },
        body: {
          error: 'Bad Request',
          message: 'The "action" field is required in the request body'
        }
      };
    }
    
    // Handle different authentication actions
    switch (action) {
      case 'login':
        return handleLogin(body, context);
      case 'verify':
        return handleVerify(body, context);
      case 'refresh':
        return handleRefresh(body, context);
      case 'logout':
        return handleLogout(body, context);
      default:
        return {
          statusCode: 400,
          headers: { 'Content-Type': 'application/json' },
          body: {
            error: 'Bad Request',
            message: `Unsupported action: ${action}`
          }
        };
    }
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
 * Handle login action
 * 
 * @param {Object} body - The request body
 * @param {Object} context - The execution context
 * @returns {Object} Response object with status code and body
 */
function handleLogin(body, context) {
  // Extract username and password from the request body
  const { username, password } = body;
  
  // Check if username and password are provided
  if (!username || !password) {
    return {
      statusCode: 400,
      headers: { 'Content-Type': 'application/json' },
      body: {
        error: 'Bad Request',
        message: 'Username and password are required'
      }
    };
  }
  
  // Check if the user exists
  const user = users[username];
  if (!user) {
    return {
      statusCode: 401,
      headers: { 'Content-Type': 'application/json' },
      body: {
        error: 'Unauthorized',
        message: 'Invalid username or password'
      }
    };
  }
  
  // Check if the password is correct
  if (!verifyPassword(password, user.password)) {
    return {
      statusCode: 401,
      headers: { 'Content-Type': 'application/json' },
      body: {
        error: 'Unauthorized',
        message: 'Invalid username or password'
      }
    };
  }
  
  // Generate tokens
  const tokens = generateTokens(user, context);
  
  // Store the refresh token
  storeRefreshToken(tokens.refreshToken, user.id);
  
  // Return the tokens and user information
  return {
    statusCode: 200,
    headers: { 'Content-Type': 'application/json' },
    body: {
      message: 'Login successful',
      user: {
        id: user.id,
        username: user.username,
        name: user.name,
        roles: user.roles
      },
      tokens
    }
  };
}

/**
 * Handle verify action
 * 
 * @param {Object} body - The request body
 * @param {Object} context - The execution context
 * @returns {Object} Response object with status code and body
 */
function handleVerify(body, context) {
  // Extract token from the request body
  const { token } = body;
  
  // Check if token is provided
  if (!token) {
    return {
      statusCode: 400,
      headers: { 'Content-Type': 'application/json' },
      body: {
        error: 'Bad Request',
        message: 'Token is required'
      }
    };
  }
  
  try {
    // Verify the token
    const decoded = jwt.verify(token, getJwtSecret(context));
    
    // Check if the token is an access token
    if (decoded.type !== 'access') {
      return {
        statusCode: 401,
        headers: { 'Content-Type': 'application/json' },
        body: {
          error: 'Unauthorized',
          message: 'Invalid token type'
        }
      };
    }
    
    // Get the user from the decoded token
    const user = users[decoded.username];
    if (!user) {
      return {
        statusCode: 401,
        headers: { 'Content-Type': 'application/json' },
        body: {
          error: 'Unauthorized',
          message: 'User not found'
        }
      };
    }
    
    // Return the user information
    return {
      statusCode: 200,
      headers: { 'Content-Type': 'application/json' },
      body: {
        message: 'Token is valid',
        user: {
          id: user.id,
          username: user.username,
          roles: user.roles
        },
        expiresAt: new Date(decoded.exp * 1000).toISOString()
      }
    };
  } catch (error) {
    // Token verification failed
    return {
      statusCode: 401,
      headers: { 'Content-Type': 'application/json' },
      body: {
        error: 'Unauthorized',
        message: 'Invalid token: ' + error.message
      }
    };
  }
}

/**
 * Handle refresh action
 * 
 * @param {Object} body - The request body
 * @param {Object} context - The execution context
 * @returns {Object} Response object with status code and body
 */
function handleRefresh(body, context) {
  // Extract refresh token from the request body
  const { refreshToken } = body;
  
  // Check if refresh token is provided
  if (!refreshToken) {
    return {
      statusCode: 400,
      headers: { 'Content-Type': 'application/json' },
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
    if (decoded.type !== 'refresh') {
      return {
        statusCode: 401,
        headers: { 'Content-Type': 'application/json' },
        body: {
          error: 'Unauthorized',
          message: 'Invalid token type'
        }
      };
    }
    
    // Check if the refresh token is in the database
    if (!isRefreshTokenValid(refreshToken, decoded.sub)) {
      return {
        statusCode: 401,
        headers: { 'Content-Type': 'application/json' },
        body: {
          error: 'Unauthorized',
          message: 'Invalid refresh token'
        }
      };
    }
    
    // Get the user from the decoded token
    const user = Object.values(users).find(u => u.id === decoded.sub);
    if (!user) {
      return {
        statusCode: 401,
        headers: { 'Content-Type': 'application/json' },
        body: {
          error: 'Unauthorized',
          message: 'User not found'
        }
      };
    }
    
    // Remove the old refresh token
    removeRefreshToken(refreshToken);
    
    // Generate new tokens
    const newTokens = generateTokens(user, context);
    
    // Store the new refresh token
    storeRefreshToken(newTokens.refreshToken, user.id);
    
    // Return the new tokens
    return {
      statusCode: 200,
      headers: { 'Content-Type': 'application/json' },
      body: {
        message: 'Token refreshed successfully',
        tokens: newTokens
      }
    };
  } catch (error) {
    // Token verification failed
    return {
      statusCode: 401,
      headers: { 'Content-Type': 'application/json' },
      body: {
        error: 'Unauthorized',
        message: 'Invalid refresh token: ' + error.message
      }
    };
  }
}

/**
 * Handle logout action
 * 
 * @param {Object} body - The request body
 * @param {Object} context - The execution context
 * @returns {Object} Response object with status code and body
 */
function handleLogout(body, context) {
  // Extract refresh token from the request body
  const { refreshToken } = body;
  
  // If refresh token is provided, remove it from the database
  if (refreshToken) {
    removeRefreshToken(refreshToken);
  }
  
  // Return success response
  return {
    statusCode: 200,
    headers: { 'Content-Type': 'application/json' },
    body: {
      message: 'Logout successful'
    }
  };
}

/**
 * Generate access and refresh tokens for a user
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
      roles: user.roles,
      type: 'access'
    },
    jwtSecret,
    { expiresIn: '1h' }
  );
  
  // Generate refresh token
  const refreshToken = jwt.sign(
    {
      sub: user.id,
      type: 'refresh'
    },
    jwtSecret,
    { expiresIn: '7d' }
  );
  
  return {
    accessToken,
    refreshToken,
    expiresIn: 3600 // 1 hour in seconds
  };
}

/**
 * Store a refresh token in the database
 * 
 * @param {string} token - The refresh token
 * @param {string} userId - The user ID
 */
function storeRefreshToken(token, userId) {
  // In a real application, this would store the token in a database
  // For this example, we'll store it in memory
  tokens[token] = {
    userId,
    createdAt: Date.now()
  };
}

/**
 * Check if a refresh token is valid
 * 
 * @param {string} token - The refresh token
 * @param {string} userId - The user ID
 * @returns {boolean} True if the token is valid, false otherwise
 */
function isRefreshTokenValid(token, userId) {
  // In a real application, this would check the token in a database
  // For this example, we'll check it in memory
  return tokens[token] && tokens[token].userId === userId;
}

/**
 * Remove a refresh token from the database
 * 
 * @param {string} token - The refresh token
 */
function removeRefreshToken(token) {
  // In a real application, this would remove the token from a database
  // For this example, we'll remove it from memory
  delete tokens[token];
}

/**
 * Get the JWT secret from the context
 * 
 * @param {Object} context - The execution context
 * @returns {string} The JWT secret
 */
function getJwtSecret(context) {
  // In a real application, this would be stored securely
  // For this example, we'll use a hardcoded secret or from environment
  return context?.env?.JWT_SECRET || 'your-jwt-secret-key';
}

/**
 * Hash a password
 * 
 * @param {string} password - The password to hash
 * @returns {string} The hashed password
 */
function hashPassword(password) {
  // In a real application, you would use a proper password hashing library
  // For this example, we'll use a simple hash
  return crypto.createHash('sha256').update(password).digest('hex');
}

/**
 * Verify a password
 * 
 * @param {string} password - The password to verify
 * @param {string} hash - The password hash
 * @returns {boolean} True if the password is correct, false otherwise
 */
function verifyPassword(password, hash) {
  // In a real application, you would use a proper password verification
  // For this example, we'll use a simple hash comparison
  return hashPassword(password) === hash;
}

// Export the handler function
module.exports = { handler };
