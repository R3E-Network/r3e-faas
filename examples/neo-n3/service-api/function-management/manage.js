/**
 * Management script for Function Management Example
 * 
 * This script demonstrates how to manage functions in the Neo N3 FaaS platform.
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
      } else {
        console.log('\nNo functions found.');
      }
      
      return response.data;
    } else {
      console.error('Error listing functions:', response.data);
      return [];
    }
  } catch (error) {
    console.error('Error listing functions:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return [];
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
      
      console.log('\nPermissions:');
      if (response.data.metadata.permissions && response.data.metadata.permissions.invoke) {
        console.log('Invoke Permissions:');
        response.data.metadata.permissions.invoke.forEach(permission => {
          console.log(`- ${permission.type}: ${permission.id}`);
        });
      }
      
      if (response.data.metadata.permissions && response.data.metadata.permissions.manage) {
        console.log('Manage Permissions:');
        response.data.metadata.permissions.manage.forEach(permission => {
          console.log(`- ${permission.type}: ${permission.id}`);
        });
      }
      
      console.log('\nResource Limits:');
      if (response.data.metadata.resources) {
        console.log('Memory:', response.data.metadata.resources.memory);
        console.log('Timeout:', response.data.metadata.resources.timeout);
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

// Function to update a function
async function updateFunction(functionId, payload) {
  try {
    console.log('Updating function with ID:', functionId);
    
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

// Function to delete a function
async function deleteFunction(functionId) {
  try {
    console.log('Deleting function with ID:', functionId);
    
    // Delete the function
    const response = await axios.delete(`${API_URL}/functions/${functionId}`, {
      headers: {
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 204) {
      console.log('\nFunction deleted successfully!');
      return true;
    } else {
      console.error('Error deleting function:', response.data);
      return false;
    }
  } catch (error) {
    console.error('Error deleting function:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return false;
  }
}

// Function to get function logs
async function getFunctionLogs(functionId) {
  try {
    console.log('Getting logs for function with ID:', functionId);
    
    // Get the function logs
    const response = await axios.get(`${API_URL}/functions/${functionId}/logs`, {
      headers: {
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 200) {
      console.log('\nFunction logs retrieved successfully!');
      console.log('Total logs:', response.data.length);
      
      if (response.data.length > 0) {
        console.log('\nLogs:');
        response.data.forEach((log, index) => {
          console.log(`[${log.timestamp}] ${log.level}: ${log.message}`);
        });
      } else {
        console.log('\nNo logs found.');
      }
      
      return response.data;
    } else {
      console.error('Error getting function logs:', response.data);
      return [];
    }
  } catch (error) {
    console.error('Error getting function logs:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return [];
  }
}

// Function to get function executions
async function getFunctionExecutions(functionId) {
  try {
    console.log('Getting executions for function with ID:', functionId);
    
    // Get the function executions
    const response = await axios.get(`${API_URL}/functions/${functionId}/executions`, {
      headers: {
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 200) {
      console.log('\nFunction executions retrieved successfully!');
      console.log('Total executions:', response.data.length);
      
      if (response.data.length > 0) {
        console.log('\nExecutions:');
        response.data.forEach((execution, index) => {
          console.log(`\n${index + 1}. Execution ID: ${execution.id}`);
          console.log(`   Status: ${execution.status}`);
          console.log(`   Started: ${execution.start_time}`);
          console.log(`   Duration: ${execution.duration}ms`);
          console.log(`   Trigger: ${execution.trigger_type}`);
        });
      } else {
        console.log('\nNo executions found.');
      }
      
      return response.data;
    } else {
      console.error('Error getting function executions:', response.data);
      return [];
    }
  } catch (error) {
    console.error('Error getting function executions:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return [];
  }
}

// Function to create a function version
async function createFunctionVersion(functionId, payload) {
  try {
    console.log('Creating new version for function with ID:', functionId);
    
    // Create the function version
    const response = await axios.post(`${API_URL}/functions/${functionId}/versions`, payload, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 201) {
      console.log('\nFunction version created successfully!');
      console.log('Function ID:', functionId);
      console.log('Function Name:', response.data.name);
      console.log('Function Version:', response.data.metadata.version);
      
      return response.data;
    } else {
      console.error('Error creating function version:', response.data);
      return null;
    }
  } catch (error) {
    console.error('Error creating function version:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return null;
  }
}

// Function to list function versions
async function listFunctionVersions(functionId) {
  try {
    console.log('Listing versions for function with ID:', functionId);
    
    // List the function versions
    const response = await axios.get(`${API_URL}/functions/${functionId}/versions`, {
      headers: {
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 200) {
      console.log('\nFunction versions retrieved successfully!');
      console.log('Total versions:', response.data.length);
      
      if (response.data.length > 0) {
        console.log('\nVersions:');
        response.data.forEach((version, index) => {
          console.log(`\n${index + 1}. Version: ${version.metadata.version}`);
          console.log(`   Created: ${version.created_at}`);
          console.log(`   Active: ${version.active ? 'Yes' : 'No'}`);
        });
      } else {
        console.log('\nNo versions found.');
      }
      
      return response.data;
    } else {
      console.error('Error listing function versions:', response.data);
      return [];
    }
  } catch (error) {
    console.error('Error listing function versions:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return [];
  }
}

// Function to activate a function version
async function activateFunctionVersion(functionId, versionId) {
  try {
    console.log('Activating version with ID:', versionId, 'for function with ID:', functionId);
    
    // Activate the function version
    const response = await axios.post(`${API_URL}/functions/${functionId}/versions/${versionId}/activate`, {}, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 200) {
      console.log('\nFunction version activated successfully!');
      console.log('Function ID:', functionId);
      console.log('Version ID:', versionId);
      
      return true;
    } else {
      console.error('Error activating function version:', response.data);
      return false;
    }
  } catch (error) {
    console.error('Error activating function version:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return false;
  }
}

// Main function
async function main() {
  // Parse command line arguments
  const args = process.argv.slice(2);
  const command = args[0] || 'help';
  
  if (command === 'list') {
    // List all functions
    await listFunctions();
  } else if (command === 'get') {
    // Get function details
    const functionId = args[1];
    
    if (!functionId) {
      console.error('Error: Function ID is required');
      console.error('Usage: node manage.js get <functionId>');
      process.exit(1);
    }
    
    await getFunctionDetails(functionId);
  } else if (command === 'update') {
    // Update a function
    const functionId = args[1];
    const configFile = args[2] || 'config.yaml';
    const functionFile = args[3] || 'function.js';
    
    if (!functionId) {
      console.error('Error: Function ID is required');
      console.error('Usage: node manage.js update <functionId> [configFile] [functionFile]');
      process.exit(1);
    }
    
    // Read the function details
    const functionDetails = await getFunctionDetails(functionId);
    
    if (!functionDetails) {
      console.error('Error: Function not found');
      process.exit(1);
    }
    
    // Read the configuration file
    const config = yaml.load(fs.readFileSync(configFile, 'utf8'));
    
    // Read the function code
    const code = fs.readFileSync(functionFile, 'utf8');
    
    // Prepare the update payload
    const payload = {
      name: config.name || functionDetails.name,
      description: config.description || functionDetails.description,
      code: code,
      version: config.version || functionDetails.metadata.version,
      metadata: {
        runtime: config.runtime || functionDetails.metadata.runtime,
        handler: config.handler || functionDetails.metadata.handler,
        trigger_type: config.trigger.type || functionDetails.metadata.trigger_type,
        trigger_config: config.trigger.config || functionDetails.metadata.trigger_config,
        permissions: config.permissions || functionDetails.metadata.permissions,
        resources: config.resources || functionDetails.metadata.resources,
        environment: config.environment || functionDetails.metadata.environment,
        storage: config.storage || functionDetails.metadata.storage
      }
    };
    
    // Update the function
    await updateFunction(functionId, payload);
  } else if (command === 'delete') {
    // Delete a function
    const functionId = args[1];
    
    if (!functionId) {
      console.error('Error: Function ID is required');
      console.error('Usage: node manage.js delete <functionId>');
      process.exit(1);
    }
    
    // Ask for confirmation
    const readline = require('readline').createInterface({
      input: process.stdin,
      output: process.stdout
    });
    
    readline.question(`Are you sure you want to delete function with ID ${functionId}? (y/n) `, async (answer) => {
      if (answer.toLowerCase() === 'y') {
        const deleted = await deleteFunction(functionId);
        
        if (deleted) {
          console.log('Function deleted successfully');
        } else {
          console.error('Failed to delete function');
        }
      } else {
        console.log('Function deletion cancelled');
      }
      
      readline.close();
    });
  } else if (command === 'logs') {
    // Get function logs
    const functionId = args[1];
    
    if (!functionId) {
      console.error('Error: Function ID is required');
      console.error('Usage: node manage.js logs <functionId>');
      process.exit(1);
    }
    
    await getFunctionLogs(functionId);
  } else if (command === 'executions') {
    // Get function executions
    const functionId = args[1];
    
    if (!functionId) {
      console.error('Error: Function ID is required');
      console.error('Usage: node manage.js executions <functionId>');
      process.exit(1);
    }
    
    await getFunctionExecutions(functionId);
  } else if (command === 'versions') {
    // List function versions
    const functionId = args[1];
    
    if (!functionId) {
      console.error('Error: Function ID is required');
      console.error('Usage: node manage.js versions <functionId>');
      process.exit(1);
    }
    
    await listFunctionVersions(functionId);
  } else if (command === 'create-version') {
    // Create a function version
    const functionId = args[1];
    const configFile = args[2] || 'config.yaml';
    const functionFile = args[3] || 'function.js';
    
    if (!functionId) {
      console.error('Error: Function ID is required');
      console.error('Usage: node manage.js create-version <functionId> [configFile] [functionFile]');
      process.exit(1);
    }
    
    // Read the function details
    const functionDetails = await getFunctionDetails(functionId);
    
    if (!functionDetails) {
      console.error('Error: Function not found');
      process.exit(1);
    }
    
    // Read the configuration file
    const config = yaml.load(fs.readFileSync(configFile, 'utf8'));
    
    // Read the function code
    const code = fs.readFileSync(functionFile, 'utf8');
    
    // Prepare the version payload
    const payload = {
      name: config.name || functionDetails.name,
      description: config.description || functionDetails.description,
      code: code,
      version: config.version || functionDetails.metadata.version,
      metadata: {
        runtime: config.runtime || functionDetails.metadata.runtime,
        handler: config.handler || functionDetails.metadata.handler,
        trigger_type: config.trigger.type || functionDetails.metadata.trigger_type,
        trigger_config: config.trigger.config || functionDetails.metadata.trigger_config,
        permissions: config.permissions || functionDetails.metadata.permissions,
        resources: config.resources || functionDetails.metadata.resources,
        environment: config.environment || functionDetails.metadata.environment,
        storage: config.storage || functionDetails.metadata.storage
      }
    };
    
    // Create the function version
    await createFunctionVersion(functionId, payload);
  } else if (command === 'activate-version') {
    // Activate a function version
    const functionId = args[1];
    const versionId = args[2];
    
    if (!functionId || !versionId) {
      console.error('Error: Function ID and Version ID are required');
      console.error('Usage: node manage.js activate-version <functionId> <versionId>');
      process.exit(1);
    }
    
    const activated = await activateFunctionVersion(functionId, versionId);
    
    if (activated) {
      console.log('Function version activated successfully');
    } else {
      console.error('Failed to activate function version');
    }
  } else if (command === 'help') {
    // Display help
    console.log('Neo N3 Function Management Example');
    console.log('\nUsage:');
    console.log('  node manage.js <command> [options]');
    console.log('\nCommands:');
    console.log('  list                                  List all functions');
    console.log('  get <functionId>                      Get function details');
    console.log('  update <functionId> [config] [func]   Update a function');
    console.log('  delete <functionId>                   Delete a function');
    console.log('  logs <functionId>                     Get function logs');
    console.log('  executions <functionId>               Get function executions');
    console.log('  versions <functionId>                 List function versions');
    console.log('  create-version <functionId> [config] [func]  Create a function version');
    console.log('  activate-version <functionId> <versionId>    Activate a function version');
    console.log('  help                                  Display this help message');
  } else {
    console.error('Unknown command:', command);
    console.error('Run "node manage.js help" for usage information');
    process.exit(1);
  }
}

// Execute the main function
if (command !== 'delete') {
  main().catch(error => {
    console.error('Unhandled error:', error);
    process.exit(1);
  });
}
