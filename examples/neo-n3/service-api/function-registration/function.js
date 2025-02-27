/**
 * Example function for Neo N3 FaaS platform
 * 
 * This function demonstrates a simple HTTP endpoint that can be registered
 * with the Neo N3 FaaS platform. It returns a greeting message and information
 * about the request.
 */

/**
 * Main handler function for the FaaS platform
 * 
 * @param {Object} request - The request object containing information about the request
 * @param {Object} user - Information about the authenticated user (if any)
 * @param {Object} context - Context information about the execution environment
 * @returns {Object} Response object with status code and body
 */
async function handler(request, user, context) {
  try {
    // Log the request for debugging
    console.log('Received request:', JSON.stringify(request, null, 2));
    
    // Extract request information
    const method = request.method || 'GET';
    const path = request.path || '/';
    const query = request.query || {};
    const headers = request.headers || {};
    const body = request.body || {};
    
    // Extract user information (if authenticated)
    const userId = user?.id || 'anonymous';
    const userRoles = user?.roles || [];
    
    // Extract context information
    const functionId = context?.function?.id || 'unknown';
    const functionName = context?.function?.name || 'example-function';
    const functionVersion = context?.function?.version || '1.0.0';
    const executionId = context?.execution?.id || 'unknown';
    const timestamp = context?.execution?.timestamp || Date.now();
    
    // Create a response based on the request
    let message = `Hello from Neo N3 FaaS platform!`;
    
    // Customize the message based on the request
    if (body.name) {
      message = `Hello, ${body.name}! Welcome to the Neo N3 FaaS platform!`;
    }
    
    // Create the response object
    const response = {
      statusCode: 200,
      headers: {
        'Content-Type': 'application/json',
        'X-Function-Id': functionId,
        'X-Execution-Id': executionId
      },
      body: {
        message,
        request: {
          method,
          path,
          query,
          headers: sanitizeHeaders(headers),
          body: sanitizeBody(body)
        },
        user: {
          id: userId,
          roles: userRoles
        },
        function: {
          id: functionId,
          name: functionName,
          version: functionVersion
        },
        execution: {
          id: executionId,
          timestamp,
          date: new Date(timestamp).toISOString()
        }
      }
    };
    
    // Log the response for debugging
    console.log('Sending response:', JSON.stringify(response, null, 2));
    
    return response;
  } catch (error) {
    // Log the error
    console.error('Error processing request:', error);
    
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
 * Sanitize request headers to remove sensitive information
 * 
 * @param {Object} headers - The request headers
 * @returns {Object} Sanitized headers
 */
function sanitizeHeaders(headers) {
  const sanitized = { ...headers };
  
  // Remove sensitive headers
  const sensitiveHeaders = [
    'authorization',
    'cookie',
    'x-api-key',
    'x-auth-token'
  ];
  
  sensitiveHeaders.forEach(header => {
    if (sanitized[header]) {
      sanitized[header] = '[REDACTED]';
    }
  });
  
  return sanitized;
}

/**
 * Sanitize request body to remove sensitive information
 * 
 * @param {Object} body - The request body
 * @returns {Object} Sanitized body
 */
function sanitizeBody(body) {
  const sanitized = { ...body };
  
  // Remove sensitive fields
  const sensitiveFields = [
    'password',
    'token',
    'secret',
    'key',
    'apiKey',
    'api_key',
    'privateKey',
    'private_key'
  ];
  
  sensitiveFields.forEach(field => {
    if (sanitized[field]) {
      sanitized[field] = '[REDACTED]';
    }
  });
  
  return sanitized;
}

// Export the handler function
module.exports = { handler };
