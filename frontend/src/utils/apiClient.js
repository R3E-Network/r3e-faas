import axios from 'axios';
import { 
  getAuthToken, 
  refreshToken, 
  logout, 
  getCsrfToken
} from '../services/authService';

// Create base API client
const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:8080';

// Track ongoing refresh token process
let isRefreshing = false;
let refreshPromise = null;
let failedQueue = [];

// Process queue of failed requests
const processQueue = (error, token = null) => {
  failedQueue.forEach(promise => {
    if (error) {
      promise.reject(error);
    } else {
      promise.resolve(token);
    }
  });
  
  failedQueue = [];
};

// Create axios instance with custom configuration
const apiClient = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
  timeout: 30000, // 30 seconds
});

// Add request interceptor to include auth token and CSRF protection
apiClient.interceptors.request.use(
  (config) => {
    // Add auth token if available
    const token = getAuthToken();
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    
    // Add CSRF token to every request
    const csrfToken = getCsrfToken();
    if (csrfToken) {
      config.headers['X-CSRF-Token'] = csrfToken;
    }
    
    // Add browser fingerprint for additional security
    config.headers['X-Client-Version'] = process.env.REACT_APP_VERSION || '1.0.0';
    config.headers['X-Client-Platform'] = 'web';

    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// Add response interceptor to handle token refresh
apiClient.interceptors.response.use(
  (response) => {
    return response;
  },
  async (error) => {
    const originalRequest = error.config;
    
    // If the error is not 401 or the request already has _retry flag, reject
    if (!error.response || error.response.status !== 401 || originalRequest._retry) {
      return Promise.reject(error);
    }
    
    // Mark request as retry in progress
    originalRequest._retry = true;
    
    try {
      // Use a shared promise if refresh is already in progress
      if (isRefreshing) {
        return new Promise((resolve, reject) => {
          failedQueue.push({ resolve, reject });
        })
          .then(token => {
            originalRequest.headers.Authorization = `Bearer ${token}`;
            return apiClient(originalRequest);
          })
          .catch(err => Promise.reject(err));
      }
      
      isRefreshing = true;
      
      // Try to refresh the token
      const refreshResponse = await refreshToken();
      const newToken = refreshResponse.token;
      
      if (!newToken) {
        throw new Error('Failed to refresh token - no token returned');
      }
      
      isRefreshing = false;
      
      // Update authorization header for the original request
      originalRequest.headers.Authorization = `Bearer ${newToken}`;
      
      // Process any requests in the queue
      processQueue(null, newToken);
      
      // Retry the original request with the new token
      return apiClient(originalRequest);
    } catch (refreshError) {
      isRefreshing = false;
      
      // Process queue with error
      processQueue(refreshError);
      
      // If refresh fails, log out the user
      console.error('Token refresh failed:', refreshError);
      logout();
      
      // Redirect to login page if configured
      if (process.env.REACT_APP_AUTO_REDIRECT_TO_LOGIN !== 'false') {
        window.location.href = '/login';
      }
      
      return Promise.reject(refreshError);
    }
  }
);

// Helper methods for common API operations
export const api = {
  /**
   * Make a GET request
   * @param {string} url - API endpoint
   * @param {Object} params - Query parameters
   * @param {Object} config - Additional axios config
   * @returns {Promise} - Axios promise
   */
  get: (url, params = {}, config = {}) => {
    return apiClient.get(url, { ...config, params });
  },
  
  /**
   * Make a POST request
   * @param {string} url - API endpoint
   * @param {Object} data - Request body
   * @param {Object} config - Additional axios config
   * @returns {Promise} - Axios promise
   */
  post: (url, data = {}, config = {}) => {
    return apiClient.post(url, data, config);
  },
  
  /**
   * Make a PUT request
   * @param {string} url - API endpoint
   * @param {Object} data - Request body
   * @param {Object} config - Additional axios config
   * @returns {Promise} - Axios promise
   */
  put: (url, data = {}, config = {}) => {
    return apiClient.put(url, data, config);
  },
  
  /**
   * Make a DELETE request
   * @param {string} url - API endpoint
   * @param {Object} config - Additional axios config
   * @returns {Promise} - Axios promise
   */
  delete: (url, config = {}) => {
    return apiClient.delete(url, config);
  },
  
  /**
   * Make a PATCH request
   * @param {string} url - API endpoint
   * @param {Object} data - Request body
   * @param {Object} config - Additional axios config
   * @returns {Promise} - Axios promise
   */
  patch: (url, data = {}, config = {}) => {
    return apiClient.patch(url, data, config);
  },
  
  /**
   * Upload a file using multipart/form-data
   * @param {string} url - API endpoint
   * @param {FormData} formData - Form data with file(s)
   * @param {Object} config - Additional axios config
   * @returns {Promise} - Axios promise
   */
  uploadFile: (url, formData, config = {}) => {
    return apiClient.post(url, formData, {
      ...config,
      headers: {
        ...config.headers,
        'Content-Type': 'multipart/form-data'
      }
    });
  }
};

/**
 * Set up global error handling for API requests
 * @param {Function} errorHandler - Function to call when an error occurs
 */
export const setupErrorHandling = (errorHandler) => {
  apiClient.interceptors.response.use(
    response => response,
    error => {
      // Call the error handler if provided
      if (errorHandler && typeof errorHandler === 'function') {
        errorHandler(error);
      }
      
      return Promise.reject(error);
    }
  );
};

export default api; 