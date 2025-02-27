/**
 * Mock Services Data for Neo N3 FaaS Platform GraphQL API
 * 
 * This file provides mock data and operations for services in the Neo N3 FaaS platform.
 */

// Mock services data
const services = [
  {
    id: 'service-1',
    name: 'neo-block-monitor',
    description: 'Monitors Neo N3 blockchain blocks and emits events',
    version: '1.0.0',
    dependencies: [
      { name: 'neo-sdk', version: '^1.0.0' },
      { name: 'axios', version: '^0.24.0' }
    ],
    permissions: {
      invoke: [
        { type: 'user', id: '*' }
      ],
      manage: [
        { type: 'user', id: 'owner' },
        { type: 'role', id: 'admin' }
      ]
    },
    resources: {
      memory: '128MB',
      timeout: '30s'
    },
    environment: {
      NEO_NETWORK: 'testnet',
      LOG_LEVEL: 'info'
    },
    storage: {
      enabled: true,
      retentionDays: 30
    },
    createdAt: '2025-02-15T10:00:00Z',
    updatedAt: '2025-02-15T10:00:00Z',
    ownerId: 'user-1'
  },
  {
    id: 'service-2',
    name: 'neo-oracle-service',
    description: 'Provides oracle services for Neo N3 smart contracts',
    version: '1.0.0',
    dependencies: [
      { name: 'neo-sdk', version: '^1.0.0' },
      { name: 'axios', version: '^0.24.0' },
      { name: 'crypto-js', version: '^4.1.1' }
    ],
    permissions: {
      invoke: [
        { type: 'user', id: '*' }
      ],
      manage: [
        { type: 'user', id: 'owner' },
        { type: 'role', id: 'admin' }
      ]
    },
    resources: {
      memory: '256MB',
      timeout: '60s'
    },
    environment: {
      NEO_NETWORK: 'testnet',
      LOG_LEVEL: 'info',
      ORACLE_API_KEY: '${ORACLE_API_KEY}'
    },
    storage: {
      enabled: true,
      retentionDays: 30
    },
    createdAt: '2025-02-16T10:00:00Z',
    updatedAt: '2025-02-16T10:00:00Z',
    ownerId: 'user-1'
  },
  {
    id: 'service-3',
    name: 'neo-tee-service',
    description: 'Provides TEE services for Neo N3 smart contracts',
    version: '1.0.0',
    dependencies: [
      { name: 'neo-sdk', version: '^1.0.0' },
      { name: 'axios', version: '^0.24.0' },
      { name: 'crypto-js', version: '^4.1.1' }
    ],
    permissions: {
      invoke: [
        { type: 'user', id: '*' }
      ],
      manage: [
        { type: 'user', id: 'owner' },
        { type: 'role', id: 'admin' }
      ]
    },
    resources: {
      memory: '512MB',
      timeout: '120s'
    },
    environment: {
      NEO_NETWORK: 'testnet',
      LOG_LEVEL: 'info',
      TEE_ATTESTATION_KEY: '${TEE_ATTESTATION_KEY}'
    },
    storage: {
      enabled: true,
      retentionDays: 30
    },
    createdAt: '2025-02-17T10:00:00Z',
    updatedAt: '2025-02-17T10:00:00Z',
    ownerId: 'user-2'
  }
];

// Get all services
function getServices() {
  return services;
}

// Get service by ID
function getServiceById(id) {
  const service = services.find(service => service.id === id);
  
  if (!service) {
    throw new Error(`Service not found: ${id}`);
  }
  
  return service;
}

// Get services by owner ID
function getServicesByOwnerId(ownerId) {
  return services.filter(service => service.ownerId === ownerId);
}

// Create a new service
function createService(input, user) {
  // Generate a new ID
  const id = `service-${services.length + 1}`;
  
  // Create timestamp
  const timestamp = new Date().toISOString();
  
  // Create new service
  const newService = {
    id,
    name: input.name,
    description: input.description || '',
    version: input.version,
    dependencies: input.dependencies || [],
    permissions: input.permissions || {
      invoke: [{ type: 'user', id: '*' }],
      manage: [{ type: 'user', id: 'owner' }]
    },
    resources: input.resources || {
      memory: '128MB',
      timeout: '30s'
    },
    environment: input.environment || {},
    storage: input.storage || {
      enabled: false
    },
    createdAt: timestamp,
    updatedAt: timestamp,
    ownerId: user.id
  };
  
  // Add to services
  services.push(newService);
  
  // Publish event
  const { pubsub, TOPICS } = require('../resolvers/subscription');
  pubsub.publish(TOPICS.SERVICE_CREATED, { serviceCreated: newService });
  
  return newService;
}

// Update an existing service
function updateService(id, input, user) {
  // Find service
  const serviceIndex = services.findIndex(service => service.id === id);
  
  if (serviceIndex === -1) {
    throw new Error(`Service not found: ${id}`);
  }
  
  // Check ownership
  const service = services[serviceIndex];
  
  if (service.ownerId !== user.id && !user.roles.includes('admin')) {
    throw new Error('Not authorized to update this service');
  }
  
  // Update service
  const updatedService = {
    ...service,
    name: input.name || service.name,
    description: input.description !== undefined ? input.description : service.description,
    version: input.version || service.version,
    dependencies: input.dependencies || service.dependencies,
    permissions: input.permissions || service.permissions,
    resources: input.resources || service.resources,
    environment: input.environment || service.environment,
    storage: input.storage || service.storage,
    updatedAt: new Date().toISOString()
  };
  
  // Update in services
  services[serviceIndex] = updatedService;
  
  // Publish event
  const { pubsub, TOPICS } = require('../resolvers/subscription');
  pubsub.publish(TOPICS.SERVICE_UPDATED, { serviceUpdated: updatedService });
  
  return updatedService;
}

// Delete a service
function deleteService(id, user) {
  // Find service
  const serviceIndex = services.findIndex(service => service.id === id);
  
  if (serviceIndex === -1) {
    throw new Error(`Service not found: ${id}`);
  }
  
  // Check ownership
  const service = services[serviceIndex];
  
  if (service.ownerId !== user.id && !user.roles.includes('admin')) {
    throw new Error('Not authorized to delete this service');
  }
  
  // Remove from services
  services.splice(serviceIndex, 1);
  
  // Publish event
  const { pubsub, TOPICS } = require('../resolvers/subscription');
  pubsub.publish(TOPICS.SERVICE_DELETED, { serviceDeleted: id });
  
  return true;
}

module.exports = {
  getServices,
  getServiceById,
  getServicesByOwnerId,
  createService,
  updateService,
  deleteService
};
