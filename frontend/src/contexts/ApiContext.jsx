import React, { createContext, useContext, useState } from 'react';
import axios from 'axios';

// Create context
const ApiContext = createContext(null);

// API base URL
const API_BASE_URL = 'http://localhost:3000';

// Create API client
const apiClient = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Provider component
export const ApiProvider = ({ children }) => {
  // State
  const [token, setToken] = useState(localStorage.getItem('token') || null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  
  // Set token
  const setAuthToken = (newToken) => {
    if (newToken) {
      localStorage.setItem('token', newToken);
      apiClient.defaults.headers.common['Authorization'] = `Bearer ${newToken}`;
    } else {
      localStorage.removeItem('token');
      delete apiClient.defaults.headers.common['Authorization'];
    }
    
    setToken(newToken);
  };
  
  // Initialize token from localStorage
  if (token) {
    apiClient.defaults.headers.common['Authorization'] = `Bearer ${token}`;
  }
  
  // Wallet connection
  const connectWallet = async (request) => {
    try {
      setLoading(true);
      setError(null);
      
      const response = await apiClient.post('/wallet/connect', request);
      
      // Set token
      setAuthToken(response.data.token);
      
      return response.data;
    } catch (error) {
      setError(error.response?.data?.message || error.message);
      throw error;
    } finally {
      setLoading(false);
    }
  };
  
  // Sign message
  const signMessage = async (request) => {
    try {
      setLoading(true);
      setError(null);
      
      const response = await apiClient.post('/wallet/sign', request);
      
      return response.data;
    } catch (error) {
      setError(error.response?.data?.message || error.message);
      throw error;
    } finally {
      setLoading(false);
    }
  };
  
  // Verify signature
  const verifySignature = async (request) => {
    try {
      setLoading(true);
      setError(null);
      
      const response = await apiClient.post('/wallet/verify', request);
      
      return response.data;
    } catch (error) {
      setError(error.response?.data?.message || error.message);
      throw error;
    } finally {
      setLoading(false);
    }
  };
  
  // Submit meta transaction
  const submitMetaTx = async (request) => {
    try {
      setLoading(true);
      setError(null);
      
      const response = await apiClient.post('/meta-tx/submit', request);
      
      return response.data;
    } catch (error) {
      setError(error.response?.data?.message || error.message);
      throw error;
    } finally {
      setLoading(false);
    }
  };
  
  // Get meta transaction status
  const getMetaTxStatus = async (id) => {
    try {
      setLoading(true);
      setError(null);
      
      const response = await apiClient.get(`/meta-tx/status/${id}`);
      
      return response.data;
    } catch (error) {
      setError(error.response?.data?.message || error.message);
      throw error;
    } finally {
      setLoading(false);
    }
  };
  
  // Get meta transaction
  const getMetaTx = async (id) => {
    try {
      setLoading(true);
      setError(null);
      
      const response = await apiClient.get(`/meta-tx/transaction/${id}`);
      
      return response.data;
    } catch (error) {
      setError(error.response?.data?.message || error.message);
      throw error;
    } finally {
      setLoading(false);
    }
  };
  
  // Get next nonce
  const getNextNonce = async (address) => {
    try {
      setLoading(true);
      setError(null);
      
      const response = await apiClient.get(`/meta-tx/nonce/${address}`);
      
      return response.data;
    } catch (error) {
      setError(error.response?.data?.message || error.message);
      throw error;
    } finally {
      setLoading(false);
    }
  };
  
  // List services
  const listServices = async () => {
    try {
      setLoading(true);
      setError(null);
      
      const response = await apiClient.get('/services');
      
      return response.data;
    } catch (error) {
      setError(error.response?.data?.message || error.message);
      throw error;
    } finally {
      setLoading(false);
    }
  };
  
  // Get service
  const getService = async (id) => {
    try {
      setLoading(true);
      setError(null);
      
      const response = await apiClient.get(`/services/${id}`);
      
      return response.data;
    } catch (error) {
      setError(error.response?.data?.message || error.message);
      throw error;
    } finally {
      setLoading(false);
    }
  };
  
  // Invoke service
  const invokeService = async (id, request) => {
    try {
      setLoading(true);
      setError(null);
      
      const response = await apiClient.post(`/services/${id}/invoke`, request);
      
      return response.data;
    } catch (error) {
      setError(error.response?.data?.message || error.message);
      throw error;
    } finally {
      setLoading(false);
    }
  };
  
  // Logout
  const logout = () => {
    setAuthToken(null);
  };
  
  // Value to provide
  const value = {
    token,
    loading,
    error,
    setAuthToken,
    connectWallet,
    signMessage,
    verifySignature,
    submitMetaTx,
    getMetaTxStatus,
    getMetaTx,
    getNextNonce,
    listServices,
    getService,
    invokeService,
    logout,
  };
  
  return (
    <ApiContext.Provider value={value}>
      {children}
    </ApiContext.Provider>
  );
};

// Hook to use API context
export const useApi = () => {
  const context = useContext(ApiContext);
  
  if (!context) {
    throw new Error('useApi must be used within an ApiProvider');
  }
  
  return context;
};
