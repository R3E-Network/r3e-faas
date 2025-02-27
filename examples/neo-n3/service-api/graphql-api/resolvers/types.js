/**
 * GraphQL Type Resolvers for Neo N3 FaaS Platform
 * 
 * This file contains the resolvers for GraphQL types in the Neo N3 FaaS platform.
 */

// Type resolvers
const Types = {
  // Service type resolvers
  Service: {
    functions: async (parent, _, { user }) => {
      // Get functions for the service
      const { getFunctionsByServiceId } = require('../data/functions');
      return getFunctionsByServiceId(parent.id);
    },
    
    dependencies: async (parent, _, { user }) => {
      // Return dependencies from parent
      return parent.dependencies || [];
    },
    
    permissions: async (parent, _, { user }) => {
      // Return permissions from parent
      return parent.permissions || {
        invoke: [],
        manage: []
      };
    },
    
    resources: async (parent, _, { user }) => {
      // Return resources from parent
      return parent.resources || {
        memory: '128MB',
        timeout: '30s'
      };
    },
    
    environment: async (parent, _, { user }) => {
      // Return environment variables from parent
      if (!parent.environment) {
        return [];
      }
      
      // Convert environment object to array of name/value pairs
      return Object.entries(parent.environment).map(([name, value]) => ({
        name,
        value
      }));
    },
    
    storage: async (parent, _, { user }) => {
      // Return storage from parent
      return parent.storage || {
        enabled: false
      };
    },
    
    owner: async (parent, _, { user }) => {
      // Get owner for the service
      if (!parent.ownerId) {
        return null;
      }
      
      const { getUserById } = require('../data/users');
      return getUserById(parent.ownerId);
    }
  },
  
  // Function type resolvers
  Function: {
    service: async (parent, _, { user }) => {
      // Get service for the function
      const { getServiceById } = require('../data/services');
      return getServiceById(parent.serviceId);
    },
    
    executions: async (parent, { limit }, { user }) => {
      // Get executions for the function
      const { getExecutionsByFunctionId } = require('../data/executions');
      return getExecutionsByFunctionId(parent.id, limit);
    },
    
    logs: async (parent, { limit }, { user }) => {
      // Get logs for the function
      const { getLogsByFunctionId } = require('../data/logs');
      return getLogsByFunctionId(parent.id, limit);
    },
    
    metrics: async (parent, _, { user }) => {
      // Get metrics for the function
      const { getMetricsByFunctionId } = require('../data/metrics');
      return getMetricsByFunctionId(parent.id);
    }
  },
  
  // Trigger type resolvers
  Trigger: {
    __resolveType(obj) {
      // Resolve the type of trigger
      if (obj.type === 'http') {
        return 'HttpTrigger';
      } else if (obj.type === 'event') {
        return 'EventTrigger';
      } else if (obj.type === 'schedule') {
        return 'ScheduleTrigger';
      }
      
      return null;
    }
  },
  
  // HttpTrigger type resolvers
  HttpTrigger: {
    type: (parent) => parent.type,
    path: (parent) => parent.path,
    methods: (parent) => parent.methods,
    cors: (parent) => parent.cors
  },
  
  // EventTrigger type resolvers
  EventTrigger: {
    type: (parent) => parent.type,
    source: (parent) => parent.source,
    event: (parent) => parent.event,
    filter: (parent) => parent.filter
  },
  
  // ScheduleTrigger type resolvers
  ScheduleTrigger: {
    type: (parent) => parent.type,
    cron: (parent) => parent.cron,
    timezone: (parent) => parent.timezone
  },
  
  // Execution type resolvers
  Execution: {
    function: async (parent, _, { user }) => {
      // Get function for the execution
      const { getFunctionById } = require('../data/functions');
      return getFunctionById(parent.functionId);
    }
  },
  
  // LogEntry type resolvers
  LogEntry: {
    function: async (parent, _, { user }) => {
      // Get function for the log entry
      const { getFunctionById } = require('../data/functions');
      return getFunctionById(parent.functionId);
    },
    
    execution: async (parent, _, { user }) => {
      // Get execution for the log entry
      if (!parent.executionId) {
        return null;
      }
      
      const { getExecutionById } = require('../data/executions');
      return getExecutionById(parent.executionId);
    }
  },
  
  // User type resolvers
  User: {
    services: async (parent, _, { user }) => {
      // Get services for the user
      const { getServicesByOwnerId } = require('../data/services');
      return getServicesByOwnerId(parent.id);
    }
  },
  
  // Metrics type resolvers
  Metrics: {
    invocations: (parent) => parent.invocations,
    resources: (parent) => parent.resources
  }
};

module.exports = Types;
