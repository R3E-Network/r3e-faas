/**
 * Registration script for Function Registration Example
 * 
 * This script demonstrates how to register a JavaScript function with the Neo N3 FaaS platform.
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

// Function to register the function with the FaaS platform
async function registerFunction() {
  try {
    // Read configuration and function code
    const config = readConfig();
    const code = readFunctionCode();
    
    // Prepare the registration payload
    const payload = {
      name: config.name,
      description: config.description,
      code: code,
      metadata: {
        runtime: config.runtime,
        handler: config.handler,
        version: config.version,
        trigger_type: config.trigger.type,
        trigger_config: config.trigger.config,
        permissions: config.permissions,
        resources: config.resources,
        environment: config.environment,
        storage: config.storage
      }
    };
    
    console.log('Registering function with Neo N3 FaaS platform...');
    console.log('Function Name:', config.name);
    console.log('Function Description:', config.description);
    console.log('Function Version:', config.version);
    console.log('Function Runtime:', config.runtime);
    console.log('Function Handler:', config.handler);
    console.log('Trigger Type:', config.trigger.type);
    
    // Register the function
    const response = await axios.post(`${API_URL}/functions`, payload, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 201) {
      console.log('\nFunction registered successfully!');
      console.log('Function ID:', response.data.id);
      console.log('Function URL:', `${API_URL}/functions/${response.data.id}`);
      
      // Save the function ID to a file for later use
      fs.writeFileSync(path.join(__dirname, '.function-id'), response.data.id);
      
      console.log('\nFunction ID saved to .function-id file');
      console.log('You can now use the client.js script to invoke the function');
      
      // Display information about the function
      console.log('\nFunction Details:');
      console.log('Trigger Type:', config.trigger.type);
      
      if (config.trigger.type === 'request') {
        console.log('HTTP Endpoint:', `${API_URL}${config.trigger.config.http.path}`);
        console.log('HTTP Methods:', config.trigger.config.http.methods.join(', '));
      } else if (config.trigger.type === 'blockchain') {
        console.log('Blockchain Events:');
        config.trigger.config.neo.events.forEach(event => {
          if (event.enabled) {
            console.log(`- ${event.type}`);
            if (event.type === 'notification' && event.contract_hash) {
              console.log(`  Contract Hash: ${event.contract_hash}`);
            }
          }
        });
      } else if (config.trigger.type === 'schedule') {
        console.log('Schedule:', config.trigger.config.cron);
      }
      
      console.log('\nPermissions:');
      if (config.permissions && config.permissions.invoke) {
        console.log('Invoke Permissions:');
        config.permissions.invoke.forEach(permission => {
          console.log(`- ${permission.type}: ${permission.id}`);
        });
      }
      
      if (config.permissions && config.permissions.manage) {
        console.log('Manage Permissions:');
        config.permissions.manage.forEach(permission => {
          console.log(`- ${permission.type}: ${permission.id}`);
        });
      }
      
      console.log('\nResource Limits:');
      if (config.resources) {
        console.log('Memory:', config.resources.memory);
        console.log('Timeout:', config.resources.timeout);
      }
      
      console.log('\nThe function is now registered and ready to be invoked!');
    } else {
      console.error('Error registering function:', response.data);
    }
  } catch (error) {
    console.error('Error registering function:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    process.exit(1);
  }
}

// Function to update an existing function
async function updateFunction(functionId) {
  try {
    // Read configuration and function code
    const config = readConfig();
    const code = readFunctionCode();
    
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
    
    console.log('Updating function on Neo N3 FaaS platform...');
    console.log('Function ID:', functionId);
    console.log('Function Name:', config.name);
    console.log('Function Version:', config.version);
    
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
      console.log('Function URL:', `${API_URL}/functions/${functionId}`);
      
      console.log('\nThe function has been updated and is ready to be invoked!');
    } else {
      console.error('Error updating function:', response.data);
    }
  } catch (error) {
    console.error('Error updating function:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    process.exit(1);
  }
}

// Main function
async function main() {
  // Check if function ID file exists
  const functionIdPath = path.join(__dirname, '.function-id');
  
  if (fs.existsSync(functionIdPath)) {
    // Function already registered, update it
    const functionId = fs.readFileSync(functionIdPath, 'utf8').trim();
    console.log('Found existing function ID:', functionId);
    
    // Ask for confirmation
    const readline = require('readline').createInterface({
      input: process.stdin,
      output: process.stdout
    });
    
    readline.question('Do you want to update this function? (y/n) ', async (answer) => {
      if (answer.toLowerCase() === 'y') {
        await updateFunction(functionId);
      } else {
        console.log('Function update cancelled');
      }
      
      readline.close();
    });
  } else {
    // Register a new function
    await registerFunction();
  }
}

// Execute the main function
main();
