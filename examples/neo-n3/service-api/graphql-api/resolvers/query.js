/**
 * GraphQL Query Resolvers for Neo N3 FaaS Platform
 * 
 * This file contains the resolvers for GraphQL queries in the Neo N3 FaaS platform.
 */

const { getServices, getServiceById } = require('../data/services');
const { getFunctions, getFunctionById } = require('../data/functions');
const { getExecutions, getExecutionById } = require('../data/executions');
const { getLogs } = require('../data/logs');
const { getUsers, getUserById, getCurrentUser } = require('../data/users');

// Query resolvers
const Query = {
  // Services
  services: async (_, __, { user }) => {
    // Check authentication
    if (!user) {
      throw new Error('Authentication required');
    }
    
    return getServices();
  },
  
  service: async (_, { id }, { user }) => {
    // Check authentication
    if (!user) {
      throw new Error('Authentication required');
    }
    
    return getServiceById(id);
  },
  
  // Functions
  functions: async (_, __, { user }) => {
    // Check authentication
    if (!user) {
      throw new Error('Authentication required');
    }
    
    return getFunctions();
  },
  
  function: async (_, { id }, { user }) => {
    // Check authentication
    if (!user) {
      throw new Error('Authentication required');
    }
    
    return getFunctionById(id);
  },
  
  // Executions
  executions: async (_, { limit }, { user }) => {
    // Check authentication
    if (!user) {
      throw new Error('Authentication required');
    }
    
    return getExecutions(limit);
  },
  
  execution: async (_, { id }, { user }) => {
    // Check authentication
    if (!user) {
      throw new Error('Authentication required');
    }
    
    return getExecutionById(id);
  },
  
  // Logs
  logs: async (_, { functionId, limit }, { user }) => {
    // Check authentication
    if (!user) {
      throw new Error('Authentication required');
    }
    
    return getLogs(functionId, limit);
  },
  
  // Users
  me: async (_, __, { user }) => {
    // Check authentication
    if (!user) {
      throw new Error('Authentication required');
    }
    
    return getCurrentUser(user.id);
  },
  
  users: async (_, __, { user }) => {
    // Check authentication
    if (!user) {
      throw new Error('Authentication required');
    }
    
    // Check if user has admin role
    if (!user.roles.includes('admin')) {
      throw new Error('Admin role required');
    }
    
    return getUsers();
  },
  
  user: async (_, { id }, { user }) => {
    // Check authentication
    if (!user) {
      throw new Error('Authentication required');
    }
    
    // Check if user has admin role or is requesting their own data
    if (!user.roles.includes('admin') && user.id !== id) {
      throw new Error('Admin role required');
    }
    
    return getUserById(id);
  }
};

module.exports = Query;
