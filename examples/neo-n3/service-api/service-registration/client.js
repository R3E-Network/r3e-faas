/**
 * Client script for Service Registration Example
 * 
 * This script demonstrates how to interact with a registered service on the Neo N3 FaaS platform.
 */

const fs = require('fs');
const path = require('path');
const axios = require('axios');

// Configuration
const API_URL = process.env.R3E_API_URL || 'http://localhost:8080/api';
const API_KEY = process.env.R3E_API_KEY;

// Check if API key is provided
if (!API_KEY) {
  console.error('Error: R3E_API_KEY environment variable is required');
  console.error('Please set it using: export R3E_API_KEY=your_api_key');
  process.exit(1);
}

// Function to read the service ID from the .service-id file
function readServiceId() {
  try {
    const serviceIdPath = path.join(__dirname, '.service-id');
    
    if (!fs.existsSync(serviceIdPath)) {
      console.error('Error: Service ID file not found');
      console.error('Please register the service first using register.js register');
      process.exit(1);
    }
    
    return fs.readFileSync(serviceIdPath, 'utf8').trim();
  } catch (error) {
    console.error('Error reading service ID:', error.message);
    process.exit(1);
  }
}

// Function to invoke a service function
async function invokeServiceFunction(serviceId, functionName, data) {
  try {
    console.log(`Invoking function "${functionName}" of service with ID: ${serviceId}`);
    console.log('Request data:', JSON.stringify(data, null, 2));
    
    // Invoke the function
    const response = await axios.post(`${API_URL}/services/${serviceId}/invoke`, {
      function: functionName,
      data
    }, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 200) {
      console.log('\nFunction invoked successfully!');
      console.log('Response status code:', response.data.statusCode);
      
      if (response.data.headers) {
        console.log('\nResponse headers:');
        Object.entries(response.data.headers).forEach(([key, value]) => {
          console.log(`${key}: ${value}`);
        });
      }
      
      console.log('\nResponse body:');
      console.log(JSON.stringify(response.data.body, null, 2));
      
      return response.data;
    } else {
      console.error('Error invoking function:', response.data);
      return null;
    }
  } catch (error) {
    console.error('Error invoking function:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return null;
  }
}

// Function to get service details
async function getServiceDetails(serviceId) {
  try {
    console.log('Getting details for service with ID:', serviceId);
    
    // Get the service details
    const response = await axios.get(`${API_URL}/services/${serviceId}`, {
      headers: {
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 200) {
      console.log('\nService details retrieved successfully!');
      console.log('Service ID:', response.data.id);
      console.log('Service Name:', response.data.name);
      console.log('Service Description:', response.data.description);
      console.log('Service Version:', response.data.version);
      console.log('Number of Functions:', response.data.functions.length);
      
      // Print function details
      console.log('\nFunctions:');
      response.data.functions.forEach((func, index) => {
        console.log(`\n${index + 1}. ${func.name}`);
        console.log(`   Description: ${func.description}`);
        console.log(`   Handler: ${func.handler}`);
        console.log(`   Trigger Type: ${func.trigger.type}`);
        
        if (func.trigger.type === 'request' && func.trigger.config.http) {
          const endpoint = `${API_URL}${func.trigger.config.http.path}`;
          console.log(`   HTTP Endpoint: ${endpoint}`);
          console.log(`   HTTP Methods: ${func.trigger.config.http.methods.join(', ')}`);
        }
      });
      
      return response.data;
    } else {
      console.error('Error getting service details:', response.data);
      return null;
    }
  } catch (error) {
    console.error('Error getting service details:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return null;
  }
}

// Function to list all services
async function listServices() {
  try {
    console.log('Listing all services...');
    
    // List all services
    const response = await axios.get(`${API_URL}/services`, {
      headers: {
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 200) {
      console.log('\nServices retrieved successfully!');
      console.log('Total services:', response.data.length);
      
      if (response.data.length > 0) {
        console.log('\nServices:');
        response.data.forEach((service, index) => {
          console.log(`\n${index + 1}. ${service.name} (ID: ${service.id})`);
          console.log(`   Description: ${service.description}`);
          console.log(`   Version: ${service.version}`);
          console.log(`   Functions: ${service.functions.length}`);
        });
      }
      
      return response.data;
    } else {
      console.error('Error listing services:', response.data);
      return [];
    }
  } catch (error) {
    console.error('Error listing services:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return [];
  }
}

// Function to authenticate with the service
async function authenticateWithService(serviceId, username, password) {
  try {
    console.log(`Authenticating with service (ID: ${serviceId})...`);
    
    // Prepare the authentication data
    const authData = {
      action: 'login',
      username,
      password
    };
    
    // Invoke the auth function
    const response = await invokeServiceFunction(serviceId, 'auth', authData);
    
    if (response && response.statusCode === 200) {
      console.log('\nAuthentication successful!');
      console.log('User ID:', response.body.user.id);
      console.log('Username:', response.body.user.username);
      console.log('Name:', response.body.user.name);
      console.log('Roles:', response.body.user.roles.join(', '));
      console.log('\nAccess Token:', response.body.tokens.accessToken);
      console.log('Refresh Token:', response.body.tokens.refreshToken);
      console.log('Expires In:', response.body.tokens.expiresIn, 'seconds');
      
      // Save the tokens to a file
      const tokensPath = path.join(__dirname, '.tokens.json');
      fs.writeFileSync(tokensPath, JSON.stringify(response.body.tokens, null, 2));
      console.log(`\nTokens saved to ${tokensPath}`);
      
      return response.body.tokens;
    } else {
      console.error('Authentication failed');
      return null;
    }
  } catch (error) {
    console.error('Error authenticating with service:', error.message);
    return null;
  }
}

// Function to verify a token
async function verifyToken(serviceId, token) {
  try {
    console.log(`Verifying token with service (ID: ${serviceId})...`);
    
    // Prepare the verification data
    const verifyData = {
      action: 'verify',
      token
    };
    
    // Invoke the auth function
    const response = await invokeServiceFunction(serviceId, 'auth', verifyData);
    
    if (response && response.statusCode === 200) {
      console.log('\nToken verification successful!');
      console.log('User ID:', response.body.user.id);
      console.log('Username:', response.body.user.username);
      console.log('Roles:', response.body.user.roles.join(', '));
      console.log('Expires At:', response.body.expiresAt);
      
      return true;
    } else {
      console.error('Token verification failed');
      return false;
    }
  } catch (error) {
    console.error('Error verifying token:', error.message);
    return false;
  }
}

// Function to refresh a token
async function refreshToken(serviceId, refreshToken) {
  try {
    console.log(`Refreshing token with service (ID: ${serviceId})...`);
    
    // Prepare the refresh data
    const refreshData = {
      action: 'refresh',
      refreshToken
    };
    
    // Invoke the auth function
    const response = await invokeServiceFunction(serviceId, 'auth', refreshData);
    
    if (response && response.statusCode === 200) {
      console.log('\nToken refresh successful!');
      console.log('New Access Token:', response.body.tokens.accessToken);
      console.log('New Refresh Token:', response.body.tokens.refreshToken);
      console.log('Expires In:', response.body.tokens.expiresIn, 'seconds');
      
      // Update the tokens file
      const tokensPath = path.join(__dirname, '.tokens.json');
      fs.writeFileSync(tokensPath, JSON.stringify(response.body.tokens, null, 2));
      console.log(`\nTokens updated in ${tokensPath}`);
      
      return response.body.tokens;
    } else {
      console.error('Token refresh failed');
      return null;
    }
  } catch (error) {
    console.error('Error refreshing token:', error.message);
    return null;
  }
}

// Function to logout
async function logout(serviceId) {
  try {
    console.log(`Logging out from service (ID: ${serviceId})...`);
    
    // Prepare the logout data
    const logoutData = {
      action: 'logout'
    };
    
    // Invoke the auth function
    const response = await invokeServiceFunction(serviceId, 'auth', logoutData);
    
    if (response && response.statusCode === 200) {
      console.log('\nLogout successful!');
      
      // Remove the tokens file
      const tokensPath = path.join(__dirname, '.tokens.json');
      if (fs.existsSync(tokensPath)) {
        fs.unlinkSync(tokensPath);
        console.log(`Tokens file ${tokensPath} removed`);
      }
      
      return true;
    } else {
      console.error('Logout failed');
      return false;
    }
  } catch (error) {
    console.error('Error logging out:', error.message);
    return false;
  }
}

// Function to process data
async function processData(serviceId, operation, data) {
  try {
    console.log(`Processing data with service (ID: ${serviceId})...`);
    console.log('Operation:', operation);
    console.log('Data:', JSON.stringify(data, null, 2));
    
    // Prepare the data processing request
    const processData = {
      operation,
      data
    };
    
    // Invoke the data function
    const response = await invokeServiceFunction(serviceId, 'data', processData);
    
    if (response && response.statusCode === 200) {
      console.log('\nData processing successful!');
      console.log('Operation:', response.body.operation);
      
      if (operation === 'transform') {
        console.log('\nOriginal Data:');
        console.log(JSON.stringify(response.body.original, null, 2));
        console.log('\nTransformed Data:');
        console.log(JSON.stringify(response.body.transformed, null, 2));
      } else if (operation === 'validate') {
        console.log('\nValidation Result:', response.body.isValid ? 'Valid' : 'Invalid');
        console.log('\nValidation Details:');
        response.body.validationResults.forEach((result, index) => {
          console.log(`${index + 1}. Field: ${result.field}`);
          console.log(`   Valid: ${result.valid}`);
          console.log(`   Message: ${result.message}`);
        });
      } else if (operation === 'analyze') {
        console.log('\nAnalysis Results:');
        console.log(JSON.stringify(response.body.analysis, null, 2));
      } else if (operation === 'store') {
        console.log('\nStored Data ID:', response.body.id);
        console.log('Stored Data:');
        console.log(JSON.stringify(response.body.data, null, 2));
      }
      
      return response.body;
    } else {
      console.error('Data processing failed');
      return null;
    }
  } catch (error) {
    console.error('Error processing data:', error.message);
    return null;
  }
}

// Function to retrieve data
async function retrieveData(serviceId, type, id) {
  try {
    console.log(`Retrieving data from service (ID: ${serviceId})...`);
    console.log('Data Type:', type);
    if (id) console.log('Data ID:', id);
    
    // Build the query parameters
    const queryParams = new URLSearchParams();
    queryParams.append('type', type);
    if (id) queryParams.append('id', id);
    
    // Invoke the data function with GET method
    const response = await axios.get(`${API_URL}/services/${serviceId}/functions/data?${queryParams.toString()}`, {
      headers: {
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    if (response.status === 200) {
      console.log('\nData retrieval successful!');
      console.log('Retrieved Data:');
      console.log(JSON.stringify(response.data, null, 2));
      
      return response.data;
    } else {
      console.error('Data retrieval failed');
      return null;
    }
  } catch (error) {
    console.error('Error retrieving data:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return null;
  }
}

// Main function
async function main() {
  // Parse command line arguments
  const args = process.argv.slice(2);
  const command = args[0] || 'help';
  
  if (command === 'invoke') {
    // Invoke a service function
    const serviceId = args[1] || readServiceId();
    const functionName = args[2];
    
    if (!functionName) {
      console.error('Error: Function name is required');
      console.error('Usage: node client.js invoke <serviceId> <functionName> [data]');
      process.exit(1);
    }
    
    // Prepare the data to send to the function
    let data = {};
    if (args[3]) {
      try {
        data = JSON.parse(args[3]);
      } catch (error) {
        // If not valid JSON, treat it as a simple string value
        data = { name: args[3] };
      }
    }
    
    await invokeServiceFunction(serviceId, functionName, data);
  } else if (command === 'details') {
    // Get service details
    const serviceId = args[1] || readServiceId();
    await getServiceDetails(serviceId);
  } else if (command === 'list') {
    // List all services
    await listServices();
  } else if (command === 'auth') {
    // Authenticate with the service
    const serviceId = args[1] || readServiceId();
    const username = args[2];
    const password = args[3];
    
    if (!username || !password) {
      console.error('Error: Username and password are required');
      console.error('Usage: node client.js auth <serviceId> <username> <password>');
      process.exit(1);
    }
    
    await authenticateWithService(serviceId, username, password);
  } else if (command === 'verify') {
    // Verify a token
    const serviceId = args[1] || readServiceId();
    const token = args[2];
    
    if (!token) {
      console.error('Error: Token is required');
      console.error('Usage: node client.js verify <serviceId> <token>');
      process.exit(1);
    }
    
    await verifyToken(serviceId, token);
  } else if (command === 'refresh') {
    // Refresh a token
    const serviceId = args[1] || readServiceId();
    const refreshToken = args[2];
    
    if (!refreshToken) {
      // Try to read the refresh token from the tokens file
      const tokensPath = path.join(__dirname, '.tokens.json');
      if (fs.existsSync(tokensPath)) {
        const tokens = JSON.parse(fs.readFileSync(tokensPath, 'utf8'));
        if (tokens.refreshToken) {
          await refreshToken(serviceId, tokens.refreshToken);
          return;
        }
      }
      
      console.error('Error: Refresh token is required');
      console.error('Usage: node client.js refresh <serviceId> <refreshToken>');
      process.exit(1);
    }
    
    await refreshToken(serviceId, refreshToken);
  } else if (command === 'logout') {
    // Logout
    const serviceId = args[1] || readServiceId();
    await logout(serviceId);
  } else if (command === 'process') {
    // Process data
    const serviceId = args[1] || readServiceId();
    const operation = args[2];
    
    if (!operation) {
      console.error('Error: Operation is required');
      console.error('Usage: node client.js process <serviceId> <operation> <data>');
      console.error('Supported operations: transform, validate, analyze, store');
      process.exit(1);
    }
    
    // Prepare the data to process
    let data = {};
    if (args[3]) {
      try {
        data = JSON.parse(args[3]);
      } catch (error) {
        console.error('Error: Data must be valid JSON');
        console.error('Example: node client.js process service123 transform \'{"name":"John","age":30}\'');
        process.exit(1);
      }
    }
    
    await processData(serviceId, operation, data);
  } else if (command === 'retrieve') {
    // Retrieve data
    const serviceId = args[1] || readServiceId();
    const type = args[2];
    const id = args[3];
    
    if (!type) {
      console.error('Error: Data type is required');
      console.error('Usage: node client.js retrieve <serviceId> <type> [id]');
      console.error('Supported types: user, transaction, block, contract, asset');
      process.exit(1);
    }
    
    await retrieveData(serviceId, type, id);
  } else if (command === 'demo') {
    // Run a full demo
    const serviceId = args[1] || readServiceId();
    
    console.log('=== Neo N3 FaaS Service Demo ===\n');
    
    // Step 1: Get service details
    console.log('Step 1: Getting service details...\n');
    const serviceDetails = await getServiceDetails(serviceId);
    console.log('\n');
    
    // Step 2: Authenticate with the service
    console.log('Step 2: Authenticating with the service...\n');
    const tokens = await authenticateWithService(serviceId, 'admin', 'admin123');
    console.log('\n');
    
    // Step 3: Verify the token
    console.log('Step 3: Verifying the token...\n');
    await verifyToken(serviceId, tokens.accessToken);
    console.log('\n');
    
    // Step 4: Process data - Transform
    console.log('Step 4: Processing data (transform)...\n');
    await processData(serviceId, 'transform', { name: 'john doe', email: 'john@example.com', age: 30 });
    console.log('\n');
    
    // Step 5: Process data - Validate
    console.log('Step 5: Processing data (validate)...\n');
    await processData(serviceId, 'validate', { id: '123', name: 'John Doe', email: 'john@example.com' });
    console.log('\n');
    
    // Step 6: Process data - Analyze
    console.log('Step 6: Processing data (analyze)...\n');
    await processData(serviceId, 'analyze', { id: '123', name: 'John Doe', email: 'john@example.com', age: 30, active: true });
    console.log('\n');
    
    // Step 7: Process data - Store
    console.log('Step 7: Processing data (store)...\n');
    await processData(serviceId, 'store', { name: 'John Doe', email: 'john@example.com', age: 30 });
    console.log('\n');
    
    // Step 8: Retrieve data
    console.log('Step 8: Retrieving data...\n');
    await retrieveData(serviceId, 'user', '1');
    console.log('\n');
    
    // Step 9: Refresh the token
    console.log('Step 9: Refreshing the token...\n');
    await refreshToken(serviceId, tokens.refreshToken);
    console.log('\n');
    
    // Step 10: Logout
    console.log('Step 10: Logging out...\n');
    await logout(serviceId);
    console.log('\n');
    
    console.log('=== Demo completed successfully ===');
  } else if (command === 'help') {
    // Display help
    console.log('Neo N3 Service Client Example');
    console.log('\nUsage:');
    console.log('  node client.js <command> [options]');
    console.log('\nCommands:');
    console.log('  invoke <serviceId> <functionName> [data]   Invoke a service function');
    console.log('  details <serviceId>                        Get service details');
    console.log('  list                                       List all services');
    console.log('  auth <serviceId> <username> <password>     Authenticate with the service');
    console.log('  verify <serviceId> <token>                 Verify a token');
    console.log('  refresh <serviceId> [refreshToken]         Refresh a token');
    console.log('  logout <serviceId>                         Logout from the service');
    console.log('  process <serviceId> <operation> <data>     Process data');
    console.log('    Supported operations: transform, validate, analyze, store');
    console.log('  retrieve <serviceId> <type> [id]           Retrieve data');
    console.log('    Supported types: user, transaction, block, contract, asset');
    console.log('  demo <serviceId>                           Run a full demo');
    console.log('  help                                       Display this help message');
    console.log('\nExamples:');
    console.log('  node client.js invoke service123 index');
    console.log('  node client.js auth service123 admin admin123');
    console.log('  node client.js process service123 transform \'{"name":"John","age":30}\'');
    console.log('  node client.js retrieve service123 user 1');
    console.log('  node client.js demo service123');
  } else {
    console.error('Unknown command:', command);
    console.error('Run "node client.js help" for usage information');
    process.exit(1);
  }
}

// Execute the main function
main().catch(error => {
  console.error('Unhandled error:', error);
  process.exit(1);
});
