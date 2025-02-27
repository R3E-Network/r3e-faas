/**
 * Update script for Function Management Example
 * 
 * This script demonstrates how to update an existing function in the Neo N3 FaaS platform.
 */

const fs = require('fs');
const path = require('path');
const axios = require('axios');
const yaml = require('js-yaml');

// Configuration
const API_URL = process.env.R3E_API_URL || 'http://localhost:8080/api';
const API_KEY = process.env.R3E_API_KEY;

// Check if API key is provided
if (!API_KEY) {
  console.error('Error: R3E_API_KEY environment variable is required');
  console.error('Please set it using: export R3E_API_KEY=your_api_key');
  process.exit(1);
}

// Function to read and parse the configuration file
function readConfig() {
  try {
    const configPath = path.join(__dirname, 'config.yaml');
    const configContent = fs.readFileSync(configPath, 'utf8');
    return yaml.load(configContent);
  } catch (error) {
    console.error('Error reading config file:', error.message);
    process.exit(1);
  }
}

// Function to read the function code
function readFunctionCode() {
  try {
    const functionPath = path.join(__dirname, 'function.js');
    return fs.readFileSync(functionPath, 'utf8');
  } catch (error) {
    console.error('Error reading function code:', error.message);
    process.exit(1);
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

// Function to update a function
async function updateFunction(functionId, payload) {
  try {
    console.log('Updating function with ID:', functionId);
    console.log('Update payload:', JSON.stringify(payload, null, 2));
    
    // Update the function
    const response = await axios.put(`${API_URL}/functions/${functionId}`, payload, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 200) {
      console.log('\nFunction updated successfully!');
      console.log('Function ID:', functionId);
      console.log('Function Name:', response.data.name);
      console.log('Function Version:', response.data.metadata.version);
      
      return response.data;
    } else {
      console.error('Error updating function:', response.data);
      return null;
    }
  } catch (error) {
    console.error('Error updating function:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return null;
  }
}

// Function to update function description
async function updateFunctionDescription(functionId, description) {
  try {
    // Get the function details
    const functionDetails = await getFunctionDetails(functionId);
    
    if (!functionDetails) {
      console.error('Error: Function not found');
      process.exit(1);
    }
    
    // Prepare the update payload
    const payload = {
      name: functionDetails.name,
      description: description,
      code: functionDetails.code,
      version: functionDetails.metadata.version,
      metadata: functionDetails.metadata
    };
    
    // Update the function
    return await updateFunction(functionId, payload);
  } catch (error) {
    console.error('Error updating function description:', error.message);
    return null;
  }
}

// Function to update function code
async function updateFunctionCode(functionId, code) {
  try {
    // Get the function details
    const functionDetails = await getFunctionDetails(functionId);
    
    if (!functionDetails) {
      console.error('Error: Function not found');
      process.exit(1);
    }
    
    // Prepare the update payload
    const payload = {
      name: functionDetails.name,
      description: functionDetails.description,
      code: code,
      version: functionDetails.metadata.version,
      metadata: functionDetails.metadata
    };
    
    // Update the function
    return await updateFunction(functionId, payload);
  } catch (error) {
    console.error('Error updating function code:', error.message);
    return null;
  }
}

// Function to update function configuration
async function updateFunctionConfig(functionId, config) {
  try {
    // Get the function details
    const functionDetails = await getFunctionDetails(functionId);
    
    if (!functionDetails) {
      console.error('Error: Function not found');
      process.exit(1);
    }
    
    // Prepare the update payload
    const payload = {
      name: config.name || functionDetails.name,
      description: config.description || functionDetails.description,
      code: functionDetails.code,
      version: config.version || functionDetails.metadata.version,
      metadata: {
        runtime: config.runtime || functionDetails.metadata.runtime,
        handler: config.handler || functionDetails.metadata.handler,
        trigger_type: config.trigger?.type || functionDetails.metadata.trigger_type,
        trigger_config: config.trigger?.config || functionDetails.metadata.trigger_config,
        permissions: config.permissions || functionDetails.metadata.permissions,
        resources: config.resources || functionDetails.metadata.resources,
        environment: config.environment || functionDetails.metadata.environment,
        storage: config.storage || functionDetails.metadata.storage
      }
    };
    
    // Update the function
    return await updateFunction(functionId, payload);
  } catch (error) {
    console.error('Error updating function configuration:', error.message);
    return null;
  }
}

// Function to update function environment variables
async function updateFunctionEnvironment(functionId, environment) {
  try {
    // Get the function details
    const functionDetails = await getFunctionDetails(functionId);
    
    if (!functionDetails) {
      console.error('Error: Function not found');
      process.exit(1);
    }
    
    // Prepare the update payload
    const payload = {
      name: functionDetails.name,
      description: functionDetails.description,
      code: functionDetails.code,
      version: functionDetails.metadata.version,
      metadata: {
        ...functionDetails.metadata,
        environment: {
          ...functionDetails.metadata.environment,
          ...environment
        }
      }
    };
    
    // Update the function
    return await updateFunction(functionId, payload);
  } catch (error) {
    console.error('Error updating function environment variables:', error.message);
    return null;
  }
}

// Function to update function permissions
async function updateFunctionPermissions(functionId, permissions) {
  try {
    // Get the function details
    const functionDetails = await getFunctionDetails(functionId);
    
    if (!functionDetails) {
      console.error('Error: Function not found');
      process.exit(1);
    }
    
    // Prepare the update payload
    const payload = {
      name: functionDetails.name,
      description: functionDetails.description,
      code: functionDetails.code,
      version: functionDetails.metadata.version,
      metadata: {
        ...functionDetails.metadata,
        permissions: permissions
      }
    };
    
    // Update the function
    return await updateFunction(functionId, payload);
  } catch (error) {
    console.error('Error updating function permissions:', error.message);
    return null;
  }
}

// Main function
async function main() {
  // Parse command line arguments
  const args = process.argv.slice(2);
  const command = args[0] || 'help';
  
  if (command === 'description') {
    // Update function description
    const functionId = args[1];
    const description = args[2];
    
    if (!functionId || !description) {
      console.error('Error: Function ID and description are required');
      console.error('Usage: node update.js description <functionId> <description>');
      process.exit(1);
    }
    
    await updateFunctionDescription(functionId, description);
  } else if (command === 'code') {
    // Update function code
    const functionId = args[1];
    const codeFile = args[2] || 'function.js';
    
    if (!functionId) {
      console.error('Error: Function ID is required');
      console.error('Usage: node update.js code <functionId> [codeFile]');
      process.exit(1);
    }
    
    // Read the function code
    const code = fs.readFileSync(codeFile, 'utf8');
    
    await updateFunctionCode(functionId, code);
  } else if (command === 'config') {
    // Update function configuration
    const functionId = args[1];
    const configFile = args[2] || 'config.yaml';
    
    if (!functionId) {
      console.error('Error: Function ID is required');
      console.error('Usage: node update.js config <functionId> [configFile]');
      process.exit(1);
    }
    
    // Read the configuration file
    const config = yaml.load(fs.readFileSync(configFile, 'utf8'));
    
    await updateFunctionConfig(functionId, config);
  } else if (command === 'environment') {
    // Update function environment variables
    const functionId = args[1];
    
    if (!functionId) {
      console.error('Error: Function ID is required');
      console.error('Usage: node update.js environment <functionId> <key=value> [key=value...]');
      process.exit(1);
    }
    
    // Parse environment variables
    const environment = {};
    for (let i = 2; i < args.length; i++) {
      const [key, value] = args[i].split('=');
      if (key && value) {
        environment[key] = value;
      }
    }
    
    if (Object.keys(environment).length === 0) {
      console.error('Error: At least one environment variable is required');
      console.error('Usage: node update.js environment <functionId> <key=value> [key=value...]');
      process.exit(1);
    }
    
    await updateFunctionEnvironment(functionId, environment);
  } else if (command === 'permissions') {
    // Update function permissions
    const functionId = args[1];
    const permissionsFile = args[2];
    
    if (!functionId || !permissionsFile) {
      console.error('Error: Function ID and permissions file are required');
      console.error('Usage: node update.js permissions <functionId> <permissionsFile>');
      process.exit(1);
    }
    
    // Read the permissions file
    const permissions = JSON.parse(fs.readFileSync(permissionsFile, 'utf8'));
    
    await updateFunctionPermissions(functionId, permissions);
  } else if (command === 'full') {
    // Full function update
    const functionId = args[1];
    const configFile = args[2] || 'config.yaml';
    const codeFile = args[3] || 'function.js';
    
    if (!functionId) {
      console.error('Error: Function ID is required');
      console.error('Usage: node update.js full <functionId> [configFile] [codeFile]');
      process.exit(1);
    }
    
    // Read the configuration file
    const config = yaml.load(fs.readFileSync(configFile, 'utf8'));
    
    // Read the function code
    const code = fs.readFileSync(codeFile, 'utf8');
    
    // Prepare the update payload
    const payload = {
      name: config.name,
      description: config.description,
      code: code,
      version: config.version,
      metadata: {
        runtime: config.runtime,
        handler: config.handler,
        trigger_type: config.trigger.type,
        trigger_config: config.trigger.config,
        permissions: config.permissions,
        resources: config.resources,
        environment: config.environment,
        storage: config.storage
      }
    };
    
    await updateFunction(functionId, payload);
  } else if (command === 'help') {
    // Display help
    console.log('Neo N3 Function Update Example');
    console.log('\nUsage:');
    console.log('  node update.js <command> [options]');
    console.log('\nCommands:');
    console.log('  description <functionId> <description>   Update function description');
    console.log('  code <functionId> [codeFile]             Update function code');
    console.log('  config <functionId> [configFile]         Update function configuration');
    console.log('  environment <functionId> <key=value>     Update function environment variables');
    console.log('  permissions <functionId> <permissionsFile> Update function permissions');
    console.log('  full <functionId> [configFile] [codeFile] Full function update');
    console.log('  help                                     Display this help message');
  } else {
    console.error('Unknown command:', command);
    console.error('Run "node update.js help" for usage information');
    process.exit(1);
  }
}

// Execute the main function
main().catch(error => {
  console.error('Unhandled error:', error);
  process.exit(1);
});
