/**
 * Client script for Function Registration Example
 * 
 * This script demonstrates how to invoke a registered function on the Neo N3 FaaS platform.
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

// Function to read the function ID from the .function-id file
function readFunctionId() {
  try {
    const functionIdPath = path.join(__dirname, '.function-id');
    
    if (!fs.existsSync(functionIdPath)) {
      console.error('Error: Function ID file not found');
      console.error('Please register the function first using register.js');
      process.exit(1);
    }
    
    return fs.readFileSync(functionIdPath, 'utf8').trim();
  } catch (error) {
    console.error('Error reading function ID:', error.message);
    process.exit(1);
  }
}

// Function to invoke the registered function
async function invokeFunction(functionId, data) {
  try {
    console.log('Invoking function with ID:', functionId);
    console.log('Request data:', JSON.stringify(data, null, 2));
    
    // Invoke the function
    const response = await axios.post(`${API_URL}/functions/${functionId}/invoke`, {
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

// Function to get function details
async function getFunctionDetails(functionId) {
  try {
    console.log('Getting details for function with ID:', functionId);
    
    // Get the function details
    const response = await axios.get(`${API_URL}/functions/${functionId}`, {
      headers: {
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 200) {
      console.log('\nFunction details retrieved successfully!');
      console.log('Function Name:', response.data.name);
      console.log('Function Description:', response.data.description);
      console.log('Function Version:', response.data.metadata.version);
      console.log('Function Runtime:', response.data.metadata.runtime);
      console.log('Function Handler:', response.data.metadata.handler);
      console.log('Trigger Type:', response.data.metadata.trigger_type);
      
      if (response.data.metadata.trigger_type === 'request') {
        console.log('HTTP Endpoint:', `${API_URL}${response.data.metadata.trigger_config.http.path}`);
        console.log('HTTP Methods:', response.data.metadata.trigger_config.http.methods.join(', '));
      }
      
      return response.data;
    } else {
      console.error('Error getting function details:', response.data);
      return null;
    }
  } catch (error) {
    console.error('Error getting function details:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return null;
  }
}

// Function to list all functions
async function listFunctions() {
  try {
    console.log('Listing all functions...');
    
    // List all functions
    const response = await axios.get(`${API_URL}/functions`, {
      headers: {
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 200) {
      console.log('\nFunctions retrieved successfully!');
      console.log('Total functions:', response.data.length);
      
      if (response.data.length > 0) {
        console.log('\nFunctions:');
        response.data.forEach((func, index) => {
          console.log(`\n${index + 1}. ${func.name} (ID: ${func.id})`);
          console.log(`   Description: ${func.description}`);
          console.log(`   Version: ${func.metadata.version}`);
          console.log(`   Runtime: ${func.metadata.runtime}`);
          console.log(`   Trigger Type: ${func.metadata.trigger_type}`);
        });
      }
      
      return response.data;
    } else {
      console.error('Error listing functions:', response.data);
      return null;
    }
  } catch (error) {
    console.error('Error listing functions:');
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
  const command = args[0] || 'invoke';
  
  if (command === 'list') {
    // List all functions
    await listFunctions();
  } else if (command === 'details') {
    // Get function details
    const functionId = args[1] || readFunctionId();
    await getFunctionDetails(functionId);
  } else if (command === 'invoke') {
    // Invoke the function
    const functionId = args[1] || readFunctionId();
    
    // Prepare the data to send to the function
    const data = {
      name: args[2] || 'World'
    };
    
    await invokeFunction(functionId, data);
  } else {
    console.error('Unknown command:', command);
    console.error('Available commands: invoke, details, list');
    console.error('Usage:');
    console.error('  node client.js invoke [functionId] [name]');
    console.error('  node client.js details [functionId]');
    console.error('  node client.js list');
    process.exit(1);
  }
}

// Execute the main function
main().catch(error => {
  console.error('Unhandled error:', error);
  process.exit(1);
});
