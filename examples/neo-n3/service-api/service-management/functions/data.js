/**
 * Data processing function for the Neo N3 Service Management Example
 * 
 * This function handles data processing for the service and demonstrates
 * how to implement data operations in a Neo N3 FaaS service.
 */

/**
 * Data processing handler function for the service
 * 
 * @param {Object} request - The request object containing information about the request
 * @param {Object} user - Information about the authenticated user (if any)
 * @param {Object} context - Context information about the execution environment
 * @returns {Object} Response object with status code and body
 */
async function handler(request, user, context) {
  try {
    // Log the request for debugging
    console.log('Received data processing request:', JSON.stringify(request, null, 2));
    
    // Extract request information
    const method = request.method || 'GET';
    const query = request.query || {};
    const body = request.body || {};
    
    // Check if this is a GET request (data retrieval)
    if (method === 'GET') {
      return handleDataRetrieval(query, user, context);
    }
    
    // Check if this is a POST request (data processing)
    if (method === 'POST') {
      return handleDataProcessing(body, user, context);
    }
    
    // If the method is not supported, return an error
    return {
      statusCode: 405,
      headers: {
        'Content-Type': 'application/json',
        'Allow': 'GET, POST'
      },
      body: {
        error: 'Method Not Allowed',
        message: `Method ${method} is not supported. Supported methods: GET, POST`
      }
    };
  } catch (error) {
    // Log the error
    console.error('Error processing data request:', error);
    
    // Return an error response
    return {
      statusCode: 500,
      headers: {
        'Content-Type': 'application/json'
      },
      body: {
        error: 'Internal Server Error',
        message: error.message
      }
    };
  }
}

/**
 * Handle data retrieval (GET requests)
 * 
 * @param {Object} query - The query parameters
 * @param {Object} user - The authenticated user (if any)
 * @param {Object} context - The execution context
 * @returns {Object} Response object with status code and body
 */
function handleDataRetrieval(query, user, context) {
  // Extract query parameters
  const { type, id } = query;
  
  // Check if the type parameter is provided
  if (!type) {
    return {
      statusCode: 400,
      headers: { 'Content-Type': 'application/json' },
      body: {
        error: 'Bad Request',
        message: 'The "type" query parameter is required'
      }
    };
  }
  
  // Get mock data based on type and id
  const data = getMockData(type, id);
  
  if (data) {
    return {
      statusCode: 200,
      headers: { 'Content-Type': 'application/json' },
      body: data
    };
  } else {
    return {
      statusCode: 400,
      headers: { 'Content-Type': 'application/json' },
      body: {
        error: 'Bad Request',
        message: `Unsupported data type: ${type}`
      }
    };
  }
}

/**
 * Handle data processing (POST requests)
 * 
 * @param {Object} body - The request body
 * @param {Object} user - The authenticated user (if any)
 * @param {Object} context - The execution context
 * @returns {Object} Response object with status code and body
 */
function handleDataProcessing(body, user, context) {
  // Extract operation from the request body
  const { operation, data } = body;
  
  // Check if the operation parameter is provided
  if (!operation) {
    return {
      statusCode: 400,
      headers: { 'Content-Type': 'application/json' },
      body: {
        error: 'Bad Request',
        message: 'The "operation" field is required in the request body'
      }
    };
  }
  
  // Check if the data parameter is provided
  if (!data) {
    return {
      statusCode: 400,
      headers: { 'Content-Type': 'application/json' },
      body: {
        error: 'Bad Request',
        message: 'The "data" field is required in the request body'
      }
    };
  }
  
  // Process data based on operation
  switch (operation) {
    case 'transform':
      return transformData(data);
    case 'validate':
      return validateData(data);
    case 'store':
      return storeData(data, user);
    case 'search':
      return searchData(data);
    default:
      return {
        statusCode: 400,
        headers: { 'Content-Type': 'application/json' },
        body: {
          error: 'Bad Request',
          message: `Unsupported operation: ${operation}`
        }
      };
  }
}

/**
 * Get mock data based on type and id
 * 
 * @param {string} type - The type of data to retrieve
 * @param {string} id - The ID of the data to retrieve (optional)
 * @returns {Object|null} The mock data or null if not found
 */
function getMockData(type, id) {
  // Mock data for different types
  const mockData = {
    services: [
      {
        id: 'service-1',
        name: 'Example Service 1',
        description: 'An example service for the Neo N3 FaaS platform',
        version: '1.0.0',
        functions: [
          { name: 'index', description: 'Main function for the service' },
          { name: 'auth', description: 'Authentication function for the service' },
          { name: 'data', description: 'Data processing function for the service' }
        ],
        created_at: '2023-01-01T00:00:00Z',
        updated_at: '2023-01-02T00:00:00Z'
      },
      {
        id: 'service-2',
        name: 'Example Service 2',
        description: 'Another example service for the Neo N3 FaaS platform',
        version: '1.0.0',
        functions: [
          { name: 'index', description: 'Main function for the service' },
          { name: 'process', description: 'Processing function for the service' }
        ],
        created_at: '2023-01-03T00:00:00Z',
        updated_at: '2023-01-04T00:00:00Z'
      }
    ],
    functions: [
      {
        id: 'function-1',
        name: 'Example Function 1',
        description: 'An example function for the Neo N3 FaaS platform',
        handler: 'function.js:handler',
        trigger: {
          type: 'request',
          config: {
            http: {
              path: '/example-function-1',
              methods: ['GET', 'POST']
            }
          }
        },
        created_at: '2023-01-01T00:00:00Z',
        updated_at: '2023-01-02T00:00:00Z'
      },
      {
        id: 'function-2',
        name: 'Example Function 2',
        description: 'Another example function for the Neo N3 FaaS platform',
        handler: 'function.js:handler',
        trigger: {
          type: 'event',
          config: {
            source: 'neo',
            event: 'block'
          }
        },
        created_at: '2023-01-03T00:00:00Z',
        updated_at: '2023-01-04T00:00:00Z'
      }
    ],
    users: [
      {
        id: 'user-1',
        username: 'admin',
        name: 'Admin User',
        roles: ['admin', 'user'],
        created_at: '2023-01-01T00:00:00Z'
      },
      {
        id: 'user-2',
        username: 'user',
        name: 'Regular User',
        roles: ['user'],
        created_at: '2023-01-02T00:00:00Z'
      }
    ],
    executions: [
      {
        id: 'execution-1',
        function_id: 'function-1',
        service_id: 'service-1',
        status: 'success',
        start_time: '2023-01-01T00:00:00Z',
        end_time: '2023-01-01T00:00:01Z',
        duration: 1000,
        logs: [
          { timestamp: '2023-01-01T00:00:00Z', level: 'info', message: 'Execution started' },
          { timestamp: '2023-01-01T00:00:01Z', level: 'info', message: 'Execution completed' }
        ]
      },
      {
        id: 'execution-2',
        function_id: 'function-2',
        service_id: 'service-2',
        status: 'error',
        start_time: '2023-01-02T00:00:00Z',
        end_time: '2023-01-02T00:00:01Z',
        duration: 1000,
        logs: [
          { timestamp: '2023-01-02T00:00:00Z', level: 'info', message: 'Execution started' },
          { timestamp: '2023-01-02T00:00:01Z', level: 'error', message: 'Execution failed: Error message' }
        ]
      }
    ],
    logs: [
      { timestamp: '2023-01-01T00:00:00Z', level: 'info', message: 'Service started' },
      { timestamp: '2023-01-01T00:00:01Z', level: 'info', message: 'Function executed' },
      { timestamp: '2023-01-01T00:00:02Z', level: 'error', message: 'Error occurred' },
      { timestamp: '2023-01-01T00:00:03Z', level: 'warn', message: 'Warning message' }
    ]
  };
  
  // If type is not supported, return null
  if (!mockData[type]) {
    return null;
  }
  
  // If id is provided, return the specific item
  if (id) {
    const item = mockData[type].find(item => item.id === id);
    return item ? { [type.slice(0, -1)]: item } : null;
  }
  
  // Otherwise, return all items of the specified type
  return { [type]: mockData[type] };
}

/**
 * Transform data
 * 
 * @param {Object} data - The data to transform
 * @returns {Object} Response object with status code and body
 */
function transformData(data) {
  try {
    // Example transformation: Convert all string values to uppercase
    const transformed = transformObject(data);
    
    return {
      statusCode: 200,
      headers: { 'Content-Type': 'application/json' },
      body: {
        message: 'Data transformed successfully',
        original: data,
        transformed
      }
    };
  } catch (error) {
    return {
      statusCode: 400,
      headers: { 'Content-Type': 'application/json' },
      body: {
        error: 'Bad Request',
        message: `Error transforming data: ${error.message}`
      }
    };
  }
}

/**
 * Transform an object by converting all string values to uppercase
 * 
 * @param {Object} obj - The object to transform
 * @returns {Object} The transformed object
 */
function transformObject(obj) {
  if (typeof obj !== 'object' || obj === null) {
    return obj;
  }
  
  if (Array.isArray(obj)) {
    return obj.map(item => transformObject(item));
  }
  
  const result = {};
  for (const key in obj) {
    if (Object.prototype.hasOwnProperty.call(obj, key)) {
      const value = obj[key];
      
      if (typeof value === 'string') {
        result[key] = value.toUpperCase();
      } else if (typeof value === 'object' && value !== null) {
        result[key] = transformObject(value);
      } else {
        result[key] = value;
      }
    }
  }
  
  return result;
}

/**
 * Validate data
 * 
 * @param {Object} data - The data to validate
 * @returns {Object} Response object with status code and body
 */
function validateData(data) {
  try {
    // Example validation: Check if required fields are present
    const validationResult = validateObject(data);
    
    if (validationResult.valid) {
      return {
        statusCode: 200,
        headers: { 'Content-Type': 'application/json' },
        body: {
          message: 'Data validation successful',
          data
        }
      };
    } else {
      return {
        statusCode: 400,
        headers: { 'Content-Type': 'application/json' },
        body: {
          error: 'Validation Error',
          message: 'Data validation failed',
          errors: validationResult.errors
        }
      };
    }
  } catch (error) {
    return {
      statusCode: 400,
      headers: { 'Content-Type': 'application/json' },
      body: {
        error: 'Bad Request',
        message: `Error validating data: ${error.message}`
      }
    };
  }
}

/**
 * Validate an object
 * 
 * @param {Object} obj - The object to validate
 * @returns {Object} The validation result
 */
function validateObject(obj) {
  const errors = [];
  
  // Check if the object is defined
  if (!obj) {
    errors.push('Data is required');
    return { valid: false, errors };
  }
  
  // Example validation rules based on object type
  if (obj.type === 'service') {
    // Validate service object
    if (!obj.name) {
      errors.push('Service name is required');
    }
    
    if (!obj.description) {
      errors.push('Service description is required');
    }
    
    if (!obj.version) {
      errors.push('Service version is required');
    }
    
    if (!obj.functions || !Array.isArray(obj.functions) || obj.functions.length === 0) {
      errors.push('Service must have at least one function');
    }
  } else if (obj.type === 'function') {
    // Validate function object
    if (!obj.name) {
      errors.push('Function name is required');
    }
    
    if (!obj.handler) {
      errors.push('Function handler is required');
    }
    
    if (!obj.trigger) {
      errors.push('Function trigger is required');
    }
  } else if (obj.type === 'user') {
    // Validate user object
    if (!obj.username) {
      errors.push('Username is required');
    }
    
    if (!obj.password) {
      errors.push('Password is required');
    }
  } else {
    // Unknown object type
    errors.push(`Unknown object type: ${obj.type}`);
  }
  
  return {
    valid: errors.length === 0,
    errors
  };
}

/**
 * Store data
 * 
 * @param {Object} data - The data to store
 * @param {Object} user - The authenticated user
 * @returns {Object} Response object with status code and body
 */
function storeData(data, user) {
  try {
    // In a real application, this would store the data in a database
    // For this example, we'll just return a success response
    
    // Check if the user is authenticated
    if (!user || !user.id) {
      return {
        statusCode: 401,
        headers: { 'Content-Type': 'application/json' },
        body: {
          error: 'Unauthorized',
          message: 'Authentication is required to store data'
        }
      };
    }
    
    // Generate a mock ID for the stored data
    const id = `data-${Date.now()}`;
    
    return {
      statusCode: 201,
      headers: { 'Content-Type': 'application/json' },
      body: {
        message: 'Data stored successfully',
        id,
        data,
        stored_by: user.id,
        stored_at: new Date().toISOString()
      }
    };
  } catch (error) {
    return {
      statusCode: 500,
      headers: { 'Content-Type': 'application/json' },
      body: {
        error: 'Internal Server Error',
        message: `Error storing data: ${error.message}`
      }
    };
  }
}

/**
 * Search data
 * 
 * @param {Object} query - The search query
 * @returns {Object} Response object with status code and body
 */
function searchData(query) {
  try {
    // Extract search parameters
    const { type, term } = query;
    
    // Check if the type parameter is provided
    if (!type) {
      return {
        statusCode: 400,
        headers: { 'Content-Type': 'application/json' },
        body: {
          error: 'Bad Request',
          message: 'The "type" field is required in the search query'
        }
      };
    }
    
    // Check if the term parameter is provided
    if (!term) {
      return {
        statusCode: 400,
        headers: { 'Content-Type': 'application/json' },
        body: {
          error: 'Bad Request',
          message: 'The "term" field is required in the search query'
        }
      };
    }
    
    // Get mock data for the specified type
    const data = getMockData(type);
    
    if (!data) {
      return {
        statusCode: 400,
        headers: { 'Content-Type': 'application/json' },
        body: {
          error: 'Bad Request',
          message: `Unsupported data type: ${type}`
        }
      };
    }
    
    // Search for items matching the term
    const results = searchItems(data[type], term);
    
    return {
      statusCode: 200,
      headers: { 'Content-Type': 'application/json' },
      body: {
        message: 'Search completed successfully',
        query,
        results_count: results.length,
        results
      }
    };
  } catch (error) {
    return {
      statusCode: 500,
      headers: { 'Content-Type': 'application/json' },
      body: {
        error: 'Internal Server Error',
        message: `Error searching data: ${error.message}`
      }
    };
  }
}

/**
 * Search for items matching a term
 * 
 * @param {Array} items - The items to search
 * @param {string} term - The search term
 * @returns {Array} The matching items
 */
function searchItems(items, term) {
  if (!items || !Array.isArray(items)) {
    return [];
  }
  
  const termLower = term.toLowerCase();
  
  return items.filter(item => {
    // Check if any string property contains the search term
    return Object.values(item).some(value => {
      if (typeof value === 'string') {
        return value.toLowerCase().includes(termLower);
      }
      return false;
    });
  });
}

// Export the handler function
module.exports = { handler };
