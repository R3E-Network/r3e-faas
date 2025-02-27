/**
 * Main function for the Neo N3 Service Registration Example
 * 
 * This function serves as the main entry point for the service and demonstrates
 * how to create a service with multiple functions in the Neo N3 FaaS platform.
 */

/**
 * Main handler function for the service
 * 
 * @param {Object} request - The request object containing information about the request
 * @param {Object} user - Information about the authenticated user (if any)
 * @param {Object} context - Context information about the execution environment
 * @returns {Object} Response object with status code and body
 */
async function handler(request, user, context) {
  try {
    // Log the request for debugging
    console.log('Received request in index function:', JSON.stringify(request, null, 2));
    
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
    const serviceId = context?.service?.id || 'unknown';
    const serviceName = context?.service?.name || 'example-service';
    const serviceVersion = context?.service?.version || '1.0.0';
    const functionId = context?.function?.id || 'unknown';
    const functionName = context?.function?.name || 'index';
    const executionId = context?.execution?.id || 'unknown';
    const timestamp = context?.execution?.timestamp || Date.now();
    
    // Create a response based on the request
    let message = `Welcome to the Neo N3 FaaS Service Example!`;
    
    // Customize the message based on the request
    if (body.name) {
      message = `Hello, ${body.name}! Welcome to the Neo N3 FaaS Service Example!`;
    }
    
    // Create the response object
    const response = {
      statusCode: 200,
      headers: {
        'Content-Type': 'application/json',
        'X-Service-Id': serviceId,
        'X-Function-Id': functionId,
        'X-Execution-Id': executionId
      },
      body: {
        message,
        service: {
          id: serviceId,
          name: serviceName,
          version: serviceVersion
        },
        function: {
          id: functionId,
          name: functionName
        },
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
        execution: {
          id: executionId,
          timestamp,
          date: new Date(timestamp).toISOString()
        },
        links: {
          auth: `${getBaseUrl(request)}/example-service/auth`,
          data: `${getBaseUrl(request)}/example-service/data`
        }
      }
    };
    
    // Log the response for debugging
    console.log('Sending response from index function:', JSON.stringify(response, null, 2));
    
    return response;
  } catch (error) {
    // Log the error
    console.error('Error processing request in index function:', error);
    
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

/**
 * Get the base URL from the request
 * 
 * @param {Object} request - The request object
 * @returns {string} The base URL
 */
function getBaseUrl(request) {
  const protocol = request.headers['x-forwarded-proto'] || 'http';
  const host = request.headers['host'] || 'localhost:8080';
  return `${protocol}://${host}`;
}

// Export the handler function
module.exports = { handler };
