/**
 * GraphQL Subscription Resolvers for Neo N3 FaaS Platform
 * 
 * This file contains the resolvers for GraphQL subscriptions in the Neo N3 FaaS platform.
 */

const { PubSub } = require('graphql-subscriptions');

// Create PubSub instance
const pubsub = new PubSub();

// Subscription topics
const TOPICS = {
  SERVICE_CREATED: 'SERVICE_CREATED',
  SERVICE_UPDATED: 'SERVICE_UPDATED',
  SERVICE_DELETED: 'SERVICE_DELETED',
  FUNCTION_CREATED: 'FUNCTION_CREATED',
  FUNCTION_UPDATED: 'FUNCTION_UPDATED',
  FUNCTION_DELETED: 'FUNCTION_DELETED',
  FUNCTION_INVOKED: 'FUNCTION_INVOKED',
  LOG_ENTRY_ADDED: 'LOG_ENTRY_ADDED'
};

// Subscription resolvers
const Subscription = {
  // Services
  serviceCreated: {
    subscribe: () => pubsub.asyncIterator([TOPICS.SERVICE_CREATED])
  },
  
  serviceUpdated: {
    subscribe: () => pubsub.asyncIterator([TOPICS.SERVICE_UPDATED])
  },
  
  serviceDeleted: {
    subscribe: () => pubsub.asyncIterator([TOPICS.SERVICE_DELETED])
  },
  
  // Functions
  functionCreated: {
    subscribe: () => pubsub.asyncIterator([TOPICS.FUNCTION_CREATED])
  },
  
  functionUpdated: {
    subscribe: () => pubsub.asyncIterator([TOPICS.FUNCTION_UPDATED])
  },
  
  functionDeleted: {
    subscribe: () => pubsub.asyncIterator([TOPICS.FUNCTION_DELETED])
  },
  
  // Executions
  functionInvoked: {
    subscribe: () => pubsub.asyncIterator([TOPICS.FUNCTION_INVOKED])
  },
  
  // Logs
  logEntryAdded: {
    subscribe: (_, { functionId }) => {
      // If functionId is provided, filter logs by function
      if (functionId) {
        return pubsub.asyncIterator([`${TOPICS.LOG_ENTRY_ADDED}.${functionId}`]);
      }
      
      // Otherwise, subscribe to all logs
      return pubsub.asyncIterator([TOPICS.LOG_ENTRY_ADDED]);
    }
  }
};

// Export subscription resolvers and pubsub instance
module.exports = Subscription;

// Export pubsub instance and topics for use in other resolvers
module.exports.pubsub = pubsub;
module.exports.TOPICS = TOPICS;
