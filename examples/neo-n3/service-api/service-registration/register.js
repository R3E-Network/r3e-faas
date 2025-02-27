/**
 * Service Registration Script for Neo N3 FaaS Platform
 * 
 * This script demonstrates how to register a service with the Neo N3 FaaS platform.
 * A service in the Neo N3 FaaS platform is a collection of related functions that
 * work together to provide a specific capability or feature.
 */

const fs = require('fs');
const path = require('path');
const axios = require('axios');
const yaml = require('js-yaml');

// Configuration
const API_URL = process.env.R3E_API_URL || 'http://localhost:8080/api';
const API_KEY = process.env.R3E_API_KEY;
const CONFIG_FILE = path.join(__dirname, 'config.yaml');
const SERVICE_ID_FILE = path.join(__dirname, '.service-id');

// Check if API key is provided
if (!API_KEY) {
  console.error('Error: R3E_API_KEY environment variable is required');
  console.error('Please set it using: export R3E_API_KEY=your_api_key');
  process.exit(1);
}

// Function to read the configuration file
function readConfig() {
  try {
    console.log('Reading configuration from:', CONFIG_FILE);
    const configYaml = fs.readFileSync(CONFIG_FILE, 'utf8');
    return yaml.load(configYaml);
  } catch (error) {
    console.error('Error reading configuration file:', error.message);
    process.exit(1);
  }
}

// Function to read function files
function readFunctionFiles(config) {
  try {
    console.log('Reading function files...');
    
    // Process each function
    const functions = config.functions.map(func => {
      // Extract handler file path and function name
      const [handlerFile, handlerFunction] = func.handler.split(':');
      const filePath = path.join(__dirname, handlerFile);
      
      console.log(`Reading function file: ${filePath}`);
      
      // Read the function file
      const code = fs.readFileSync(filePath, 'utf8');
      
      // Return the function with code
      return {
        ...func,
        code
      };
    });
    
    return functions;
  } catch (error) {
    console.error('Error reading function files:', error.message);
    process.exit(1);
  }
}

// Function to register the service
async function registerService(config, functions) {
  try {
    console.log('Registering service with the Neo N3 FaaS platform...');
    
    // Prepare the service registration payload
    const payload = {
      name: config.name,
      description: config.description,
      version: config.version,
      functions,
      dependencies: config.dependencies,
      permissions: config.permissions,
      resources: config.resources,
      environment: config.environment,
      storage: config.storage
    };
    
    // Register the service
    const response = await axios.post(`${API_URL}/services`, payload, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 201) {
      console.log('\nService registered successfully!');
      console.log('Service ID:', response.data.id);
      console.log('Service Name:', response.data.name);
      console.log('Service Description:', response.data.description);
      console.log('Service Version:', response.data.version);
      console.log('Number of Functions:', response.data.functions.length);
      
      // Save the service ID to a file
      fs.writeFileSync(SERVICE_ID_FILE, response.data.id);
      console.log(`\nService ID saved to ${SERVICE_ID_FILE}`);
      
      // Print function endpoints
      console.log('\nFunction Endpoints:');
      response.data.functions.forEach(func => {
        if (func.trigger && func.trigger.type === 'request' && func.trigger.config.http) {
          const endpoint = `${API_URL}${func.trigger.config.http.path}`;
          console.log(`- ${func.name}: ${endpoint}`);
        }
      });
      
      console.log('\nYou can now use the client.js script to interact with the service:');
      console.log('node client.js invoke index');
      
      return response.data;
    } else {
      console.error('Error registering service:', response.data);
      return null;
    }
  } catch (error) {
    console.error('Error registering service:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return null;
  }
}

// Function to update an existing service
async function updateService(serviceId, config, functions) {
  try {
    console.log(`Updating service with ID: ${serviceId}`);
    
    // Prepare the service update payload
    const payload = {
      name: config.name,
      description: config.description,
      version: config.version,
      functions,
      dependencies: config.dependencies,
      permissions: config.permissions,
      resources: config.resources,
      environment: config.environment,
      storage: config.storage
    };
    
    // Update the service
    const response = await axios.put(`${API_URL}/services/${serviceId}`, payload, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 200) {
      console.log('\nService updated successfully!');
      console.log('Service ID:', response.data.id);
      console.log('Service Name:', response.data.name);
      console.log('Service Description:', response.data.description);
      console.log('Service Version:', response.data.version);
      console.log('Number of Functions:', response.data.functions.length);
      
      // Print function endpoints
      console.log('\nFunction Endpoints:');
      response.data.functions.forEach(func => {
        if (func.trigger && func.trigger.type === 'request' && func.trigger.config.http) {
          const endpoint = `${API_URL}${func.trigger.config.http.path}`;
          console.log(`- ${func.name}: ${endpoint}`);
        }
      });
      
      return response.data;
    } else {
      console.error('Error updating service:', response.data);
      return null;
    }
  } catch (error) {
    console.error('Error updating service:');
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
    console.log(`Getting details for service with ID: ${serviceId}`);
    
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

// Function to delete a service
async function deleteService(serviceId) {
  try {
    console.log(`Deleting service with ID: ${serviceId}`);
    
    // Delete the service
    const response = await axios.delete(`${API_URL}/services/${serviceId}`, {
      headers: {
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 204) {
      console.log('\nService deleted successfully!');
      
      // Remove the service ID file
      if (fs.existsSync(SERVICE_ID_FILE)) {
        fs.unlinkSync(SERVICE_ID_FILE);
        console.log(`Service ID file ${SERVICE_ID_FILE} removed`);
      }
      
      return true;
    } else {
      console.error('Error deleting service:', response.data);
      return false;
    }
  } catch (error) {
    console.error('Error deleting service:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return false;
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

// Main function
async function main() {
  // Parse command line arguments
  const args = process.argv.slice(2);
  const command = args[0] || 'register';
  
  // Read the configuration
  const config = readConfig();
  
  if (command === 'register') {
    // Register a new service
    const functions = readFunctionFiles(config);
    await registerService(config, functions);
  } else if (command === 'update') {
    // Update an existing service
    const serviceId = args[1] || readServiceId();
    const functions = readFunctionFiles(config);
    await updateService(serviceId, config, functions);
  } else if (command === 'get') {
    // Get service details
    const serviceId = args[1] || readServiceId();
    await getServiceDetails(serviceId);
  } else if (command === 'delete') {
    // Delete a service
    const serviceId = args[1] || readServiceId();
    await deleteService(serviceId);
  } else if (command === 'list') {
    // List all services
    await listServices();
  } else {
    console.error('Unknown command:', command);
    console.error('Available commands: register, update, get, delete, list');
    console.error('Usage:');
    console.error('  node register.js register');
    console.error('  node register.js update [serviceId]');
    console.error('  node register.js get [serviceId]');
    console.error('  node register.js delete [serviceId]');
    console.error('  node register.js list');
    process.exit(1);
  }
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

// Execute the main function
main().catch(error => {
  console.error('Unhandled error:', error);
  process.exit(1);
});
