/**
 * Authentication service for wallet-based authentication
 */

import axios from 'axios';
import { v4 as uuidv4 } from 'uuid';

// API endpoints
const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:8080';
const CONNECT_ENDPOINT = `${API_BASE_URL}/auth/wallet/connect`;
const AUTHENTICATE_ENDPOINT = `${API_BASE_URL}/auth/wallet/authenticate`;
const REFRESH_ENDPOINT = `${API_BASE_URL}/auth/refresh`;

// Storage keys
const AUTH_TOKEN_KEY = 'authToken';
const USER_ID_KEY = 'userId';
const WALLET_ADDRESS_KEY = 'walletAddress';
const BLOCKCHAIN_TYPE_KEY = 'blockchainType';
const TOKEN_EXPIRY_KEY = 'tokenExpiry';
const CSRF_TOKEN_KEY = 'csrfToken';

/**
 * Generate a CSRF token for request protection
 * @returns {string} Generated CSRF token
 */
export const generateCsrfToken = () => {
  const token = uuidv4();
  sessionStorage.setItem(CSRF_TOKEN_KEY, token);
  return token;
};

/**
 * Get the current CSRF token
 * @returns {string} Current CSRF token
 */
export const getCsrfToken = () => {
  let token = sessionStorage.getItem(CSRF_TOKEN_KEY);
  
  // Generate a token if one doesn't exist
  if (!token) {
    token = generateCsrfToken();
  }
  
  return token;
};

/**
 * Request a challenge for wallet authentication
 * @param {string} address - Wallet address
 * @param {string} blockchainType - Blockchain type (ethereum, neo_n3, solana)
 * @returns {Promise<Object>} - Challenge information
 */
export const requestChallenge = async (address, blockchainType) => {
  try {
    // Add CSRF token
    const csrfToken = getCsrfToken();
    
    // Make the request
    const response = await axios.post(
      CONNECT_ENDPOINT, 
      {
        address,
        blockchain_type: blockchainType
      }, 
      {
        headers: {
          'X-CSRF-Token': csrfToken
        }
      }
    );
    
    return response.data;
  } catch (error) {
    console.error('Error requesting challenge:', error);
    const errorMsg = error.response?.data?.message || 'Failed to request challenge';
    throw new Error(errorMsg);
  }
};

/**
 * Verify a signed challenge for wallet authentication
 * @param {string} challengeId - Challenge ID from requestChallenge
 * @param {string} address - Wallet address
 * @param {string} blockchainType - Blockchain type (ethereum, neo_n3, solana)
 * @param {string} signature - Signed challenge
 * @param {string} signatureCurve - Signature curve (optional)
 * @returns {Promise<Object>} - Authentication information including JWT
 */
export const verifyChallenge = async (challengeId, address, blockchainType, signature, signatureCurve) => {
  try {
    // Validate inputs
    if (!challengeId || !address || !blockchainType || !signature) {
      throw new Error('Missing required parameters for challenge verification');
    }
    
    const payload = {
      challenge_id: challengeId,
      address,
      blockchain_type: blockchainType,
      signature
    };
    
    // Only add signature curve if provided
    if (signatureCurve) {
      payload.signature_curve = signatureCurve;
    }
    
    // Add CSRF token
    const csrfToken = getCsrfToken();
    
    const response = await axios.post(
      AUTHENTICATE_ENDPOINT, 
      payload,
      {
        headers: {
          'X-CSRF-Token': csrfToken
        }
      }
    );
    
    // Validate the response
    const authData = response.data;
    if (!authData.token || !authData.user_id || !authData.address) {
      throw new Error('Invalid authentication response from server');
    }
    
    // Store authentication info in sessionStorage
    sessionStorage.setItem(AUTH_TOKEN_KEY, authData.token);
    sessionStorage.setItem(USER_ID_KEY, authData.user_id);
    sessionStorage.setItem(WALLET_ADDRESS_KEY, authData.address);
    sessionStorage.setItem(BLOCKCHAIN_TYPE_KEY, authData.blockchain_type);
    sessionStorage.setItem(TOKEN_EXPIRY_KEY, authData.expires_at.toString());
    
    return authData;
  } catch (error) {
    console.error('Error verifying challenge:', error);
    const errorMsg = error.response?.data?.message || error.message || 'Failed to verify challenge';
    throw new Error(errorMsg);
  }
};

/**
 * Refresh the authentication token
 * @returns {Promise<Object>} - Refreshed authentication information
 */
export const refreshToken = async () => {
  try {
    // Get current token
    const currentToken = sessionStorage.getItem(AUTH_TOKEN_KEY);
    
    if (!currentToken) {
      throw new Error('No authentication token found');
    }
    
    // Add CSRF token
    const csrfToken = getCsrfToken();
    
    // Make the request
    const response = await axios.post(
      REFRESH_ENDPOINT, 
      {
        token: currentToken
      },
      {
        headers: {
          'X-CSRF-Token': csrfToken
        }
      }
    );
    
    // Update token in sessionStorage
    const authData = response.data;
    
    // Validate the response
    if (!authData.token || !authData.expires_at) {
      throw new Error('Invalid refresh response from server');
    }
    
    sessionStorage.setItem(AUTH_TOKEN_KEY, authData.token);
    sessionStorage.setItem(TOKEN_EXPIRY_KEY, authData.expires_at.toString());
    
    return authData;
  } catch (error) {
    console.error('Error refreshing token:', error);
    const errorMsg = error.response?.data?.message || error.message || 'Failed to refresh token';
    throw new Error(errorMsg);
  }
};

/**
 * Check if user is currently authenticated
 * @returns {boolean} - True if user is authenticated
 */
export const isAuthenticated = () => {
  const token = sessionStorage.getItem(AUTH_TOKEN_KEY);
  const expiry = sessionStorage.getItem(TOKEN_EXPIRY_KEY);
  
  if (!token || !expiry) {
    return false;
  }
  
  // Check if token is expired
  const now = Math.floor(Date.now() / 1000);
  return parseInt(expiry, 10) > now;
};

/**
 * Get the current authenticated user information
 * @returns {Object|null} - User information or null if not authenticated
 */
export const getCurrentUser = () => {
  if (!isAuthenticated()) {
    return null;
  }
  
  return {
    userId: sessionStorage.getItem(USER_ID_KEY),
    walletAddress: sessionStorage.getItem(WALLET_ADDRESS_KEY),
    blockchainType: sessionStorage.getItem(BLOCKCHAIN_TYPE_KEY)
  };
};

/**
 * Logout the current user
 */
export const logout = () => {
  sessionStorage.removeItem(AUTH_TOKEN_KEY);
  sessionStorage.removeItem(USER_ID_KEY);
  sessionStorage.removeItem(WALLET_ADDRESS_KEY);
  sessionStorage.removeItem(BLOCKCHAIN_TYPE_KEY);
  sessionStorage.removeItem(TOKEN_EXPIRY_KEY);
  
  // Don't remove CSRF token as it's still needed for non-authenticated requests
};

/**
 * Get the authentication token for API requests
 * @returns {string|null} - Auth token or null if not authenticated
 */
export const getAuthToken = () => {
  if (!isAuthenticated()) {
    return null;
  }
  
  return sessionStorage.getItem(AUTH_TOKEN_KEY);
};

/**
 * Automatically try to refresh the token if it's about to expire
 * Call this when setting up the application
 */
export const setupTokenRefresh = () => {
  // Check if token exists and if it's about to expire
  const expiry = sessionStorage.getItem(TOKEN_EXPIRY_KEY);
  
  if (!expiry) {
    return;
  }
  
  const now = Math.floor(Date.now() / 1000);
  const expiryTime = parseInt(expiry, 10);
  const timeUntilExpiry = expiryTime - now;
  
  // If token expires in less than 5 minutes, refresh it
  if (timeUntilExpiry > 0 && timeUntilExpiry < 300) {
    refreshToken().catch(error => {
      console.error('Auto token refresh failed:', error);
      // If refresh fails, log out
      logout();
    });
  }
  
  // Set up a refresh timer to check periodically
  // Refresh when token has 5 minutes left
  setInterval(() => {
    if (isAuthenticated()) {
      const currentExpiry = sessionStorage.getItem(TOKEN_EXPIRY_KEY);
      const nowTime = Math.floor(Date.now() / 1000);
      const currentExpiryTime = parseInt(currentExpiry, 10);
      const remainingTime = currentExpiryTime - nowTime;
      
      // If less than 5 minutes remaining, refresh
      if (remainingTime > 0 && remainingTime < 300) {
        refreshToken().catch(console.error);
      }
    }
  }, 60000); // Check every minute
}; 