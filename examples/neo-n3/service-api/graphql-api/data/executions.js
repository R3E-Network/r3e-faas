/**
 * Mock Executions Data for Neo N3 FaaS Platform GraphQL API
 * 
 * This file provides mock data and operations for function executions in the Neo N3 FaaS platform.
 */

// Mock executions data
const executions = [
  {
    id: 'execution-1',
    functionId: 'function-1',
    status: 'success',
    startTime: '2025-02-15T12:00:00Z',
    endTime: '2025-02-15T12:00:01Z',
    duration: 1000,
    request: JSON.stringify({
      event: 'block',
      data: {
        blockHeight: 1000000,
        blockHash: '0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef',
        timestamp: 1708000000000
      }
    }),
    response: JSON.stringify({
      status: 'success',
      message: 'Block processed successfully'
    }),
    error: null
  },
  {
    id: 'execution-2',
    functionId: 'function-2',
    status: 'success',
    startTime: '2025-02-15T12:30:00Z',
    endTime: '2025-02-15T12:30:01Z',
    duration: 1000,
    request: JSON.stringify({
      event: 'transaction',
      data: {
        txHash: '0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890',
        blockHeight: 1000001,
        timestamp: 1708001000000
      }
    }),
    response: JSON.stringify({
      status: 'success',
      message: 'Transaction processed successfully'
    }),
    error: null
  },
  {
    id: 'execution-3',
    functionId: 'function-3',
    status: 'success',
    startTime: '2025-02-16T12:00:00Z',
    endTime: '2025-02-16T12:00:01Z',
    duration: 1000,
    request: JSON.stringify({
      method: 'GET',
      path: '/oracle/price',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer token123'
      },
      body: null
    }),
    response: JSON.stringify({
      status: 'success',
      data: {
        symbol: 'NEO',
        price: 42.5,
        timestamp: 1708100000000
      }
    }),
    error: null
  },
  {
    id: 'execution-4',
    functionId: 'function-4',
    status: 'success',
    startTime: '2025-02-16T12:30:00Z',
    endTime: '2025-02-16T12:30:01Z',
    duration: 1000,
    request: JSON.stringify({
      method: 'POST',
      path: '/oracle/random',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer token123'
      },
      body: {
        min: 1,
        max: 100
      }
    }),
    response: JSON.stringify({
      status: 'success',
      data: {
        random: 42,
        timestamp: 1708101000000
      }
    }),
    error: null
  },
  {
    id: 'execution-5',
    functionId: 'function-5',
    status: 'success',
    startTime: '2025-02-17T12:00:00Z',
    endTime: '2025-02-17T12:00:02Z',
    duration: 2000,
    request: JSON.stringify({
      method: 'POST',
      path: '/tee/keys',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer token123'
      },
      body: {
        operation: 'generate',
        keyType: 'secp256r1'
      }
    }),
    response: JSON.stringify({
      status: 'success',
      data: {
        keyId: 'key-123',
        publicKey: '0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef',
        timestamp: 1708200000000
      }
    }),
    error: null
  },
  {
    id: 'execution-6',
    functionId: 'function-6',
    status: 'error',
    startTime: '2025-02-17T12:30:00Z',
    endTime: '2025-02-17T12:30:01Z',
    duration: 1000,
    request: JSON.stringify({
      method: 'POST',
      path: '/tee/compute',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer token123'
      },
      body: {
        operation: 'encrypt',
        data: 'Hello, world!',
        keyId: 'key-invalid'
      }
    }),
    response: null,
    error: 'Key not found: key-invalid'
  }
];

// Get all executions
function getExecutions(limit) {
  if (limit) {
    return executions.slice(0, limit);
  }
  
  return executions;
}

// Get execution by ID
function getExecutionById(id) {
  const execution = executions.find(execution => execution.id === id);
  
  if (!execution) {
    throw new Error(`Execution not found: ${id}`);
  }
  
  return execution;
}

// Get executions by function ID
function getExecutionsByFunctionId(functionId, limit) {
  const functionExecutions = executions.filter(execution => execution.functionId === functionId);
  
  if (limit) {
    return functionExecutions.slice(0, limit);
  }
  
  return functionExecutions;
}

// Invoke a function
function invokeFunction(id, input, user) {
  // Check if function exists
  const { getFunctionById } = require('./functions');
  const func = getFunctionById(id);
  
  // Generate a new ID
  const executionId = `execution-${executions.length + 1}`;
  
  // Create timestamp
  const startTime = new Date().toISOString();
  
  // Create request
  const request = JSON.stringify({
    method: input.method || 'POST',
    path: input.path || '/',
    headers: input.headers ? JSON.parse(input.headers) : {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${user.id}`
    },
    body: input.body ? JSON.parse(input.body) : null
  });
  
  // Simulate function execution
  let status, response, error, duration;
  
  try {
    // Simulate successful execution
    if (Math.random() > 0.1) { // 90% success rate
      status = 'success';
      
      // Generate response based on function type
      if (func.trigger.type === 'http') {
        if (func.name.includes('price')) {
          response = JSON.stringify({
            status: 'success',
            data: {
              symbol: 'NEO',
              price: Math.random() * 100,
              timestamp: Date.now()
            }
          });
        } else if (func.name.includes('random')) {
          response = JSON.stringify({
            status: 'success',
            data: {
              random: Math.floor(Math.random() * 100),
              timestamp: Date.now()
            }
          });
        } else if (func.name.includes('key')) {
          response = JSON.stringify({
            status: 'success',
            data: {
              keyId: `key-${Math.floor(Math.random() * 1000)}`,
              publicKey: `0x${Math.random().toString(16).substring(2)}${Math.random().toString(16).substring(2)}`,
              timestamp: Date.now()
            }
          });
        } else {
          response = JSON.stringify({
            status: 'success',
            message: 'Function executed successfully'
          });
        }
      } else {
        response = JSON.stringify({
          status: 'success',
          message: 'Function executed successfully'
        });
      }
      
      error = null;
    } else {
      // Simulate error
      status = 'error';
      response = null;
      error = 'Function execution failed';
    }
    
    // Simulate duration
    duration = Math.floor(Math.random() * 1000) + 500;
  } catch (err) {
    status = 'error';
    response = null;
    error = err.message;
    duration = Math.floor(Math.random() * 1000) + 500;
  }
  
  // Calculate end time
  const endTimeDate = new Date(new Date(startTime).getTime() + duration);
  const endTime = endTimeDate.toISOString();
  
  // Create execution
  const execution = {
    id: executionId,
    functionId: id,
    status,
    startTime,
    endTime,
    duration,
    request,
    response,
    error
  };
  
  // Add to executions
  executions.push(execution);
  
  // Publish event
  const { pubsub, TOPICS } = require('../resolvers/subscription');
  pubsub.publish(TOPICS.FUNCTION_INVOKED, { functionInvoked: execution });
  
  // Create invocation result
  const invocationResult = {
    execution,
    statusCode: status === 'success' ? 200 : 500,
    headers: JSON.stringify({
      'Content-Type': 'application/json'
    }),
    body: response
  };
  
  return invocationResult;
}

module.exports = {
  getExecutions,
  getExecutionById,
  getExecutionsByFunctionId,
  invokeFunction
};
