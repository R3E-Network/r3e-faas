/**
 * Data processing function for the Neo N3 Service Registration Example
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
  
  // Mock data for different types
  const mockData = {
    user: getMockUserData(id),
    transaction: getMockTransactionData(id),
    block: getMockBlockData(id),
    contract: getMockContractData(id),
    asset: getMockAssetData(id)
  };
  
  // Return data based on type
  if (mockData[type]) {
    return {
      statusCode: 200,
      headers: { 'Content-Type': 'application/json' },
      body: mockData[type]
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
      return handleTransformOperation(data);
    case 'validate':
      return handleValidateOperation(data);
    case 'analyze':
      return handleAnalyzeOperation(data);
    case 'store':
      return handleStoreOperation(data, user, context);
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
 * Handle transform operation
 * 
 * @param {Object} data - The data to transform
 * @returns {Object} Response object with status code and body
 */
function handleTransformOperation(data) {
  // In a real application, you would perform actual data transformation
  // For this example, we'll just return a mock transformation
  
  let transformed = { ...data };
  
  // Perform some mock transformations
  if (typeof data === 'object') {
    // Convert all string values to uppercase
    Object.keys(data).forEach(key => {
      if (typeof data[key] === 'string') {
        transformed[key] = data[key].toUpperCase();
      }
    });
    
    // Add a timestamp
    transformed.transformedAt = new Date().toISOString();
  }
  
  return {
    statusCode: 200,
    headers: { 'Content-Type': 'application/json' },
    body: {
      operation: 'transform',
      original: data,
      transformed
    }
  };
}

/**
 * Handle validate operation
 * 
 * @param {Object} data - The data to validate
 * @returns {Object} Response object with status code and body
 */
function handleValidateOperation(data) {
  // In a real application, you would perform actual data validation
  // For this example, we'll just return a mock validation result
  
  const validationResults = [];
  
  // Perform some mock validations
  if (typeof data === 'object') {
    // Check for required fields
    const requiredFields = ['id', 'name'];
    requiredFields.forEach(field => {
      if (!data[field]) {
        validationResults.push({
          field,
          valid: false,
          message: `Field "${field}" is required`
        });
      } else {
        validationResults.push({
          field,
          valid: true,
          message: `Field "${field}" is valid`
        });
      }
    });
    
    // Check for field types
    if (data.id && typeof data.id !== 'string') {
      validationResults.push({
        field: 'id',
        valid: false,
        message: 'Field "id" must be a string'
      });
    }
    
    if (data.age && typeof data.age !== 'number') {
      validationResults.push({
        field: 'age',
        valid: false,
        message: 'Field "age" must be a number'
      });
    }
  }
  
  // Determine overall validity
  const isValid = validationResults.every(result => result.valid);
  
  return {
    statusCode: 200,
    headers: { 'Content-Type': 'application/json' },
    body: {
      operation: 'validate',
      data,
      isValid,
      validationResults
    }
  };
}

/**
 * Handle analyze operation
 * 
 * @param {Object} data - The data to analyze
 * @returns {Object} Response object with status code and body
 */
function handleAnalyzeOperation(data) {
  // In a real application, you would perform actual data analysis
  // For this example, we'll just return a mock analysis result
  
  const analysis = {
    type: typeof data,
    summary: {}
  };
  
  // Perform some mock analysis
  if (typeof data === 'object') {
    // Count the number of fields
    analysis.summary.fieldCount = Object.keys(data).length;
    
    // Count the number of each type of field
    const fieldTypes = {};
    Object.values(data).forEach(value => {
      const type = typeof value;
      fieldTypes[type] = (fieldTypes[type] || 0) + 1;
    });
    analysis.summary.fieldTypes = fieldTypes;
    
    // Calculate the average string length
    const stringFields = Object.values(data).filter(value => typeof value === 'string');
    if (stringFields.length > 0) {
      const totalLength = stringFields.reduce((sum, str) => sum + str.length, 0);
      analysis.summary.averageStringLength = totalLength / stringFields.length;
    }
    
    // Calculate the average number value
    const numberFields = Object.values(data).filter(value => typeof value === 'number');
    if (numberFields.length > 0) {
      const total = numberFields.reduce((sum, num) => sum + num, 0);
      analysis.summary.averageNumberValue = total / numberFields.length;
    }
  }
  
  return {
    statusCode: 200,
    headers: { 'Content-Type': 'application/json' },
    body: {
      operation: 'analyze',
      data,
      analysis
    }
  };
}

/**
 * Handle store operation
 * 
 * @param {Object} data - The data to store
 * @param {Object} user - The authenticated user (if any)
 * @param {Object} context - The execution context
 * @returns {Object} Response object with status code and body
 */
function handleStoreOperation(data, user, context) {
  // In a real application, you would store the data in a database
  // For this example, we'll just return a mock storage result
  
  // Generate a mock ID for the stored data
  const id = generateMockId();
  
  // Add metadata to the stored data
  const storedData = {
    ...data,
    id,
    createdAt: new Date().toISOString(),
    createdBy: user?.id || 'anonymous'
  };
  
  return {
    statusCode: 200,
    headers: { 'Content-Type': 'application/json' },
    body: {
      operation: 'store',
      id,
      data: storedData,
      message: 'Data stored successfully'
    }
  };
}

/**
 * Generate a mock ID
 * 
 * @returns {string} A mock ID
 */
function generateMockId() {
  return 'id_' + Math.random().toString(36).substring(2, 15);
}

/**
 * Get mock user data
 * 
 * @param {string} id - The user ID
 * @returns {Object} Mock user data
 */
function getMockUserData(id) {
  const mockUsers = {
    '1': { id: '1', username: 'admin', name: 'Admin User', roles: ['admin', 'user'] },
    '2': { id: '2', username: 'user', name: 'Regular User', roles: ['user'] }
  };
  
  return id && mockUsers[id] ? mockUsers[id] : mockUsers['1'];
}

/**
 * Get mock transaction data
 * 
 * @param {string} id - The transaction ID
 * @returns {Object} Mock transaction data
 */
function getMockTransactionData(id) {
  const mockTransactions = {
    'tx123': { txid: 'tx123', sender: 'addr1', receiver: 'addr2', amount: 100, asset: 'NEO', timestamp: Date.now() - 3600000 },
    'tx456': { txid: 'tx456', sender: 'addr3', receiver: 'addr1', amount: 50, asset: 'GAS', timestamp: Date.now() - 7200000 }
  };
  
  return id && mockTransactions[id] ? mockTransactions[id] : mockTransactions['tx123'];
}

/**
 * Get mock block data
 * 
 * @param {string} id - The block height or hash
 * @returns {Object} Mock block data
 */
function getMockBlockData(id) {
  const mockBlocks = {
    '12345': { height: 12345, hash: 'block123', transactions: 10, timestamp: Date.now() - 3600000, size: 1024 },
    '12346': { height: 12346, hash: 'block456', transactions: 5, timestamp: Date.now() - 1800000, size: 512 }
  };
  
  return id && mockBlocks[id] ? mockBlocks[id] : mockBlocks['12345'];
}

/**
 * Get mock contract data
 * 
 * @param {string} id - The contract hash
 * @returns {Object} Mock contract data
 */
function getMockContractData(id) {
  const mockContracts = {
    'contract123': { hash: 'contract123', name: 'ExampleToken', standard: 'NEP-17', owner: 'addr1', totalSupply: 1000000 },
    'contract456': { hash: 'contract456', name: 'ExampleNFT', standard: 'NEP-11', owner: 'addr2', totalSupply: 100 }
  };
  
  return id && mockContracts[id] ? mockContracts[id] : mockContracts['contract123'];
}

/**
 * Get mock asset data
 * 
 * @param {string} id - The asset ID
 * @returns {Object} Mock asset data
 */
function getMockAssetData(id) {
  const mockAssets = {
    'neo': { id: 'neo', name: 'NEO', symbol: 'NEO', decimals: 0, totalSupply: 100000000 },
    'gas': { id: 'gas', name: 'GAS', symbol: 'GAS', decimals: 8, totalSupply: 100000000 }
  };
  
  return id && mockAssets[id] ? mockAssets[id] : mockAssets['neo'];
}

// Export the handler function
module.exports = { handler };
