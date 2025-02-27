/**
 * Registration script for Neo N3 Multi-Event Handler
 * 
 * This script registers the multi-event handler function with the Neo N3 FaaS platform.
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
        trigger_type: config.trigger.type,
        trigger_config: config.trigger.config
      }
    };
    
    // Register the function
    console.log('Registering multi-event handler function with Neo N3 FaaS platform...');
    const response = await axios.post(`${API_URL}/functions`, payload, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 201) {
      console.log('Function registered successfully!');
      console.log('Function ID:', response.data.id);
      console.log('Function URL:', `${API_URL}/functions/${response.data.id}`);
      
      // Display information about the event types
      console.log('\nEvent Type Configuration:');
      
      console.log('Event Types:');
      config.trigger.config.event_types.forEach(type => {
        console.log(`- ${type}`);
      });
      
      if (config.trigger.config.block) {
        console.log('\nBlock Event Configuration:');
        if (config.trigger.config.block.min_index) {
          console.log(`Min Index: ${config.trigger.config.block.min_index}`);
        }
        if (config.trigger.config.block.max_index) {
          console.log(`Max Index: ${config.trigger.config.block.max_index}`);
        }
      }
      
      if (config.trigger.config.transaction) {
        console.log('\nTransaction Event Configuration:');
        if (config.trigger.config.transaction.types) {
          console.log('Transaction Types:');
          config.trigger.config.transaction.types.forEach(type => {
            console.log(`- ${type}`);
          });
        }
        if (config.trigger.config.transaction.contract_hashes) {
          console.log('Contract Hashes:');
          config.trigger.config.transaction.contract_hashes.forEach(hash => {
            console.log(`- ${hash}`);
          });
        }
      }
      
      if (config.trigger.config.notification) {
        console.log('\nNotification Event Configuration:');
        if (config.trigger.config.notification.contract_hashes) {
          console.log('Contract Hashes:');
          config.trigger.config.notification.contract_hashes.forEach(hash => {
            console.log(`- ${hash}`);
          });
        }
        if (config.trigger.config.notification.event_names) {
          console.log('Event Names:');
          config.trigger.config.notification.event_names.forEach(name => {
            console.log(`- ${name}`);
          });
        }
      }
      
      if (config.trigger.config.coordination && config.trigger.config.coordination.enabled) {
        console.log('\nCoordination Configuration:');
        console.log(`Max Wait Time: ${config.trigger.config.coordination.max_wait_time} seconds`);
        console.log(`Min Batch Size: ${config.trigger.config.coordination.min_batch_size} events`);
      }
      
      console.log('\nThe function will be triggered when events matching these criteria occur on the Neo N3 blockchain.');
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

// Execute the registration
registerFunction();
