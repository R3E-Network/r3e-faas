/**
 * Mock Functions Data for Neo N3 FaaS Platform GraphQL API
 * 
 * This file provides mock data and operations for functions in the Neo N3 FaaS platform.
 */

// Mock functions data
const functions = [
  {
    id: 'function-1',
    name: 'block-monitor',
    description: 'Monitors Neo N3 blockchain blocks',
    handler: 'block-monitor.js:handler',
    trigger: {
      type: 'event',
      source: 'neo',
      event: 'block',
      filter: null
    },
    serviceId: 'service-1',
    createdAt: '2025-02-15T10:00:00Z',
    updatedAt: '2025-02-15T10:00:00Z'
  },
  {
    id: 'function-2',
    name: 'transaction-monitor',
    description: 'Monitors Neo N3 blockchain transactions',
    handler: 'transaction-monitor.js:handler',
    trigger: {
      type: 'event',
      source: 'neo',
      event: 'transaction',
      filter: 'type=InvocationTransaction'
    },
    serviceId: 'service-1',
    createdAt: '2025-02-15T10:30:00Z',
    updatedAt: '2025-02-15T10:30:00Z'
  },
  {
    id: 'function-3',
    name: 'price-oracle',
    description: 'Provides price data for Neo N3 smart contracts',
    handler: 'price-oracle.js:handler',
    trigger: {
      type: 'http',
      path: '/oracle/price',
      methods: ['GET', 'POST'],
      cors: {
        enabled: true,
        allowedOrigins: ['*'],
        allowedMethods: ['GET', 'POST', 'OPTIONS'],
        allowedHeaders: ['Content-Type', 'Authorization']
      }
    },
    serviceId: 'service-2',
    createdAt: '2025-02-16T10:00:00Z',
    updatedAt: '2025-02-16T10:00:00Z'
  },
  {
    id: 'function-4',
    name: 'random-oracle',
    description: 'Provides random numbers for Neo N3 smart contracts',
    handler: 'random-oracle.js:handler',
    trigger: {
      type: 'http',
      path: '/oracle/random',
      methods: ['GET', 'POST'],
      cors: {
        enabled: true,
        allowedOrigins: ['*'],
        allowedMethods: ['GET', 'POST', 'OPTIONS'],
        allowedHeaders: ['Content-Type', 'Authorization']
      }
    },
    serviceId: 'service-2',
    createdAt: '2025-02-16T10:30:00Z',
    updatedAt: '2025-02-16T10:30:00Z'
  },
  {
    id: 'function-5',
    name: 'secure-key-manager',
    description: 'Manages secure keys in TEE for Neo N3 smart contracts',
    handler: 'secure-key-manager.js:handler',
    trigger: {
      type: 'http',
      path: '/tee/keys',
      methods: ['GET', 'POST', 'PUT', 'DELETE'],
      cors: {
        enabled: true,
        allowedOrigins: ['*'],
        allowedMethods: ['GET', 'POST', 'PUT', 'DELETE', 'OPTIONS'],
        allowedHeaders: ['Content-Type', 'Authorization']
      }
    },
    serviceId: 'service-3',
    createdAt: '2025-02-17T10:00:00Z',
    updatedAt: '2025-02-17T10:00:00Z'
  },
  {
    id: 'function-6',
    name: 'confidential-compute',
    description: 'Performs confidential computing in TEE for Neo N3 smart contracts',
    handler: 'confidential-compute.js:handler',
    trigger: {
      type: 'http',
      path: '/tee/compute',
      methods: ['POST'],
      cors: {
        enabled: true,
        allowedOrigins: ['*'],
        allowedMethods: ['POST', 'OPTIONS'],
        allowedHeaders: ['Content-Type', 'Authorization']
      }
    },
    serviceId: 'service-3',
    createdAt: '2025-02-17T10:30:00Z',
    updatedAt: '2025-02-17T10:30:00Z'
  }
];

// Get all functions
function getFunctions() {
  return functions;
}

// Get function by ID
function getFunctionById(id) {
  const func = functions.find(func => func.id === id);
  
  if (!func) {
    throw new Error(`Function not found: ${id}`);
  }
  
  return func;
}

// Get functions by service ID
function getFunctionsByServiceId(serviceId) {
  return functions.filter(func => func.serviceId === serviceId);
}

// Create a new function
function createFunction(serviceId, input, user) {
  // Check if service exists
  const { getServiceById } = require('./services');
  const service = getServiceById(serviceId);
  
  // Check ownership
  if (service.ownerId !== user.id && !user.roles.includes('admin')) {
    throw new Error('Not authorized to create functions for this service');
  }
  
  // Generate a new ID
  const id = `function-${functions.length + 1}`;
  
  // Create timestamp
  const timestamp = new Date().toISOString();
  
  // Create new function
  const newFunction = {
    id,
    name: input.name,
    description: input.description || '',
    handler: input.handler,
    trigger: input.trigger,
    serviceId,
    createdAt: timestamp,
    updatedAt: timestamp
  };
  
  // Add to functions
  functions.push(newFunction);
  
  // Publish event
  const { pubsub, TOPICS } = require('../resolvers/subscription');
  pubsub.publish(TOPICS.FUNCTION_CREATED, { functionCreated: newFunction });
  
  return newFunction;
}

// Update an existing function
function updateFunction(id, input, user) {
  // Find function
  const functionIndex = functions.findIndex(func => func.id === id);
  
  if (functionIndex === -1) {
    throw new Error(`Function not found: ${id}`);
  }
  
  // Get function
  const func = functions[functionIndex];
  
  // Check ownership
  const { getServiceById } = require('./services');
  const service = getServiceById(func.serviceId);
  
  if (service.ownerId !== user.id && !user.roles.includes('admin')) {
    throw new Error('Not authorized to update this function');
  }
  
  // Update function
  const updatedFunction = {
    ...func,
    name: input.name || func.name,
    description: input.description !== undefined ? input.description : func.description,
    handler: input.handler || func.handler,
    trigger: input.trigger || func.trigger,
    updatedAt: new Date().toISOString()
  };
  
  // Update in functions
  functions[functionIndex] = updatedFunction;
  
  // Publish event
  const { pubsub, TOPICS } = require('../resolvers/subscription');
  pubsub.publish(TOPICS.FUNCTION_UPDATED, { functionUpdated: updatedFunction });
  
  return updatedFunction;
}

// Delete a function
function deleteFunction(id, user) {
  // Find function
  const functionIndex = functions.findIndex(func => func.id === id);
  
  if (functionIndex === -1) {
    throw new Error(`Function not found: ${id}`);
  }
  
  // Get function
  const func = functions[functionIndex];
  
  // Check ownership
  const { getServiceById } = require('./services');
  const service = getServiceById(func.serviceId);
  
  if (service.ownerId !== user.id && !user.roles.includes('admin')) {
    throw new Error('Not authorized to delete this function');
  }
  
  // Remove from functions
  functions.splice(functionIndex, 1);
  
  // Publish event
  const { pubsub, TOPICS } = require('../resolvers/subscription');
  pubsub.publish(TOPICS.FUNCTION_DELETED, { functionDeleted: id });
  
  return true;
}

module.exports = {
  getFunctions,
  getFunctionById,
  getFunctionsByServiceId,
  createFunction,
  updateFunction,
  deleteFunction
};
