/**
 * Mock Logs Data for Neo N3 FaaS Platform GraphQL API
 * 
 * This file provides mock data and operations for function logs in the Neo N3 FaaS platform.
 */

// Mock logs data
const logs = [
  {
    id: 'log-1',
    functionId: 'function-1',
    executionId: 'execution-1',
    timestamp: '2025-02-15T12:00:00.100Z',
    level: 'info',
    message: 'Function started'
  },
  {
    id: 'log-2',
    functionId: 'function-1',
    executionId: 'execution-1',
    timestamp: '2025-02-15T12:00:00.500Z',
    level: 'info',
    message: 'Processing block 1000000'
  },
  {
    id: 'log-3',
    functionId: 'function-1',
    executionId: 'execution-1',
    timestamp: '2025-02-15T12:00:00.900Z',
    level: 'info',
    message: 'Function completed successfully'
  },
  {
    id: 'log-4',
    functionId: 'function-2',
    executionId: 'execution-2',
    timestamp: '2025-02-15T12:30:00.100Z',
    level: 'info',
    message: 'Function started'
  },
  {
    id: 'log-5',
    functionId: 'function-2',
    executionId: 'execution-2',
    timestamp: '2025-02-15T12:30:00.500Z',
    level: 'info',
    message: 'Processing transaction 0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890'
  },
  {
    id: 'log-6',
    functionId: 'function-2',
    executionId: 'execution-2',
    timestamp: '2025-02-15T12:30:00.900Z',
    level: 'info',
    message: 'Function completed successfully'
  },
  {
    id: 'log-7',
    functionId: 'function-3',
    executionId: 'execution-3',
    timestamp: '2025-02-16T12:00:00.100Z',
    level: 'info',
    message: 'Function started'
  },
  {
    id: 'log-8',
    functionId: 'function-3',
    executionId: 'execution-3',
    timestamp: '2025-02-16T12:00:00.300Z',
    level: 'info',
    message: 'Fetching price data for NEO'
  },
  {
    id: 'log-9',
    functionId: 'function-3',
    executionId: 'execution-3',
    timestamp: '2025-02-16T12:00:00.800Z',
    level: 'info',
    message: 'Function completed successfully'
  },
  {
    id: 'log-10',
    functionId: 'function-4',
    executionId: 'execution-4',
    timestamp: '2025-02-16T12:30:00.100Z',
    level: 'info',
    message: 'Function started'
  },
  {
    id: 'log-11',
    functionId: 'function-4',
    executionId: 'execution-4',
    timestamp: '2025-02-16T12:30:00.300Z',
    level: 'info',
    message: 'Generating random number between 1 and 100'
  },
  {
    id: 'log-12',
    functionId: 'function-4',
    executionId: 'execution-4',
    timestamp: '2025-02-16T12:30:00.800Z',
    level: 'info',
    message: 'Function completed successfully'
  },
  {
    id: 'log-13',
    functionId: 'function-5',
    executionId: 'execution-5',
    timestamp: '2025-02-17T12:00:00.100Z',
    level: 'info',
    message: 'Function started'
  },
  {
    id: 'log-14',
    functionId: 'function-5',
    executionId: 'execution-5',
    timestamp: '2025-02-17T12:00:00.300Z',
    level: 'info',
    message: 'Generating secp256r1 key pair in TEE'
  },
  {
    id: 'log-15',
    functionId: 'function-5',
    executionId: 'execution-5',
    timestamp: '2025-02-17T12:00:01.800Z',
    level: 'info',
    message: 'Function completed successfully'
  },
  {
    id: 'log-16',
    functionId: 'function-6',
    executionId: 'execution-6',
    timestamp: '2025-02-17T12:30:00.100Z',
    level: 'info',
    message: 'Function started'
  },
  {
    id: 'log-17',
    functionId: 'function-6',
    executionId: 'execution-6',
    timestamp: '2025-02-17T12:30:00.300Z',
    level: 'info',
    message: 'Attempting to encrypt data with key key-invalid'
  },
  {
    id: 'log-18',
    functionId: 'function-6',
    executionId: 'execution-6',
    timestamp: '2025-02-17T12:30:00.800Z',
    level: 'error',
    message: 'Key not found: key-invalid'
  }
];

// Get all logs
function getLogs(functionId, limit) {
  let filteredLogs = logs;
  
  // Filter by function ID if provided
  if (functionId) {
    filteredLogs = filteredLogs.filter(log => log.functionId === functionId);
  }
  
  // Sort by timestamp (newest first)
  filteredLogs = filteredLogs.sort((a, b) => new Date(b.timestamp) - new Date(a.timestamp));
  
  // Limit if provided
  if (limit) {
    filteredLogs = filteredLogs.slice(0, limit);
  }
  
  return filteredLogs;
}

// Get logs by function ID
function getLogsByFunctionId(functionId, limit) {
  return getLogs(functionId, limit);
}

// Add a log entry
function addLogEntry(functionId, executionId, level, message) {
  // Generate a new ID
  const id = `log-${logs.length + 1}`;
  
  // Create timestamp
  const timestamp = new Date().toISOString();
  
  // Create log entry
  const logEntry = {
    id,
    functionId,
    executionId,
    timestamp,
    level,
    message
  };
  
  // Add to logs
  logs.push(logEntry);
  
  // Publish event
  const { pubsub, TOPICS } = require('../resolvers/subscription');
  pubsub.publish(TOPICS.LOG_ENTRY_ADDED, { logEntryAdded: logEntry });
  
  // Also publish to function-specific topic
  pubsub.publish(`${TOPICS.LOG_ENTRY_ADDED}.${functionId}`, { logEntryAdded: logEntry });
  
  return logEntry;
}

module.exports = {
  getLogs,
  getLogsByFunctionId,
  addLogEntry
};
