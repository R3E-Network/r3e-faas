/**
 * GraphQL Mutation Resolvers for Neo N3 FaaS Platform
 * 
 * This file contains the resolvers for GraphQL mutations in the Neo N3 FaaS platform.
 */

const { createService, updateService, deleteService } = require('../data/services');
const { createFunction, updateFunction, deleteFunction } = require('../data/functions');
const { invokeFunction } = require('../data/executions');
const { login, refreshToken, logout } = require('../data/auth');

// Mutation resolvers
const Mutation = {
  // Services
  createService: async (_, { input }, { user }) => {
    // Check authentication
    if (!user) {
      throw new Error('Authentication required');
    }
    
    return createService(input, user);
  },
  
  updateService: async (_, { id, input }, { user }) => {
    // Check authentication
    if (!user) {
      throw new Error('Authentication required');
    }
    
    return updateService(id, input, user);
  },
  
  deleteService: async (_, { id }, { user }) => {
    // Check authentication
    if (!user) {
      throw new Error('Authentication required');
    }
    
    return deleteService(id, user);
  },
  
  // Functions
  createFunction: async (_, { serviceId, input }, { user }) => {
    // Check authentication
    if (!user) {
      throw new Error('Authentication required');
    }
    
    return createFunction(serviceId, input, user);
  },
  
  updateFunction: async (_, { id, input }, { user }) => {
    // Check authentication
    if (!user) {
      throw new Error('Authentication required');
    }
    
    return updateFunction(id, input, user);
  },
  
  deleteFunction: async (_, { id }, { user }) => {
    // Check authentication
    if (!user) {
      throw new Error('Authentication required');
    }
    
    return deleteFunction(id, user);
  },
  
  // Invocations
  invokeFunction: async (_, { id, input }, { user }) => {
    // Check authentication
    if (!user) {
      throw new Error('Authentication required');
    }
    
    return invokeFunction(id, input, user);
  },
  
  // Authentication
  login: async (_, { username, password }) => {
    return login(username, password);
  },
  
  refreshToken: async (_, { token }) => {
    return refreshToken(token);
  },
  
  logout: async (_, __, { user }) => {
    // Check authentication
    if (!user) {
      throw new Error('Authentication required');
    }
    
    return logout(user);
  }
};

module.exports = Mutation;
