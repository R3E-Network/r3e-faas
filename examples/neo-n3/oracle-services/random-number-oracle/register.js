/**
 * Registration script for Neo N3 Random Number Oracle
 * 
 * This script registers the random number oracle function with the Neo N3 FaaS platform.
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
        trigger_config: config.trigger.config,
        oracle_config: config.oracle
      }
    };
    
    // Register the function
    console.log('Registering random number oracle function with Neo N3 FaaS platform...');
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
      
      // Display information about the random number oracle configuration
      console.log('\nRandom Number Oracle Configuration:');
      
      console.log('Supported Random Types:');
      config.oracle.types.forEach(type => {
        console.log(`- ${type.name} (Enabled: ${type.enabled})`);
      });
      
      console.log('\nEntropy Sources:');
      config.oracle.entropy.forEach(source => {
        console.log(`- ${source.name} (Enabled: ${source.enabled}, Weight: ${source.weight})`);
      });
      
      console.log('\nVerification Methods:');
      config.oracle.verification.forEach(method => {
        console.log(`- ${method.name} (Enabled: ${method.enabled})`);
      });
      
      console.log('\nTrigger Configuration:');
      if (config.trigger.config.schedule) {
        console.log(`Schedule: ${config.trigger.config.schedule.cron} (${config.trigger.config.schedule.timezone})`);
      }
      
      if (config.trigger.config.request && config.trigger.config.request.http && config.trigger.config.request.http.enabled) {
        console.log(`HTTP Endpoint: ${config.trigger.config.request.http.path}`);
        console.log(`HTTP Methods: ${config.trigger.config.request.http.methods.join(', ')}`);
      }
      
      if (config.trigger.config.neo_contract) {
        console.log(`Neo Contract: ${config.trigger.config.neo_contract.contract_hash}`);
        console.log(`Method Name: ${config.trigger.config.neo_contract.method_name}`);
      }
      
      console.log('\nThe random number oracle function is now registered and will start generating random numbers according to the configured schedule.');
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
