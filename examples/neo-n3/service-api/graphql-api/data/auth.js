/**
 * Mock Authentication Data for Neo N3 FaaS Platform GraphQL API
 * 
 * This file provides mock data and operations for authentication in the Neo N3 FaaS platform.
 */

const jwt = require('jsonwebtoken');
const { getUserByUsername } = require('./users');

// Secret key for JWT
const JWT_SECRET = 'your-secret-key'; // In a real implementation, this would be an environment variable
const JWT_REFRESH_SECRET = 'your-refresh-secret-key'; // In a real implementation, this would be an environment variable

// Token expiration times
const TOKEN_EXPIRATION = '1h';
const REFRESH_TOKEN_EXPIRATION = '7d';

// Active refresh tokens
const refreshTokens = new Map();

// Login user
function login(username, password) {
  try {
    // Get user
    const user = getUserByUsername(username);
    
    // Check password
    if (user.password !== password) {
      throw new Error('Invalid password');
    }
    
    // Create tokens
    const token = createToken(user);
    const refreshToken = createRefreshToken(user);
    
    // Store refresh token
    refreshTokens.set(refreshToken, {
      userId: user.id,
      expiresAt: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000) // 7 days
    });
    
    // Remove password from user object
    const { password: _, ...userWithoutPassword } = user;
    
    // Return auth result
    return {
      token,
      refreshToken,
      user: userWithoutPassword,
      expiresIn: 3600 // 1 hour in seconds
    };
  } catch (error) {
    throw new Error(`Login failed: ${error.message}`);
  }
}

// Refresh token
function refreshToken(token) {
  try {
    // Check if refresh token exists
    if (!refreshTokens.has(token)) {
      throw new Error('Invalid refresh token');
    }
    
    // Get refresh token data
    const refreshTokenData = refreshTokens.get(token);
    
    // Check if refresh token has expired
    if (new Date() > refreshTokenData.expiresAt) {
      // Remove expired refresh token
      refreshTokens.delete(token);
      throw new Error('Refresh token has expired');
    }
    
    // Get user
    const { getUserById } = require('./users');
    const user = getUserById(refreshTokenData.userId);
    
    // Create new tokens
    const newToken = createToken(user);
    const newRefreshToken = createRefreshToken(user);
    
    // Remove old refresh token
    refreshTokens.delete(token);
    
    // Store new refresh token
    refreshTokens.set(newRefreshToken, {
      userId: user.id,
      expiresAt: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000) // 7 days
    });
    
    // Return auth result
    return {
      token: newToken,
      refreshToken: newRefreshToken,
      user,
      expiresIn: 3600 // 1 hour in seconds
    };
  } catch (error) {
    throw new Error(`Token refresh failed: ${error.message}`);
  }
}

// Logout user
function logout(user) {
  try {
    // Remove all refresh tokens for user
    for (const [token, data] of refreshTokens.entries()) {
      if (data.userId === user.id) {
        refreshTokens.delete(token);
      }
    }
    
    return true;
  } catch (error) {
    throw new Error(`Logout failed: ${error.message}`);
  }
}

// Create JWT token
function createToken(user) {
  // Create payload
  const payload = {
    id: user.id,
    username: user.username,
    email: user.email,
    roles: user.roles
  };
  
  // Sign token
  return jwt.sign(payload, JWT_SECRET, { expiresIn: TOKEN_EXPIRATION });
}

// Create refresh token
function createRefreshToken(user) {
  // Create payload
  const payload = {
    id: user.id,
    type: 'refresh'
  };
  
  // Sign token
  return jwt.sign(payload, JWT_REFRESH_SECRET, { expiresIn: REFRESH_TOKEN_EXPIRATION });
}

// Verify token
function verifyToken(token) {
  try {
    // Verify token
    const decoded = jwt.verify(token, JWT_SECRET);
    
    // Return decoded token
    return decoded;
  } catch (error) {
    throw new Error(`Token verification failed: ${error.message}`);
  }
}

module.exports = {
  login,
  refreshToken,
  logout,
  verifyToken
};
