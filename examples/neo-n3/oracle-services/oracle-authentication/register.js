/**
 * Registration script for Neo N3 Oracle Authentication Example
 * 
 * This script registers the oracle authentication service function with the Neo N3 FaaS platform.
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
        oracle: {
          auth: config.oracle.auth,
          rbac: config.oracle.rbac,
          auditing: config.oracle.auditing
        }
      }
    };
    
    // Register the function
    console.log('Registering oracle authentication service function with Neo N3 FaaS platform...');
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
      
      // Display information about the authentication configuration
      console.log('\nOracle Authentication Configuration:');
      
      console.log('Authentication Methods:');
      if (config.oracle.auth.api_key && config.oracle.auth.api_key.enabled) {
        console.log('- API Key Authentication (Enabled)');
        console.log(`  Header: ${config.oracle.auth.api_key.header}`);
        console.log(`  Storage: ${config.oracle.auth.api_key.storage}`);
        console.log(`  Rotation: ${config.oracle.auth.api_key.rotation_days} days`);
      }
      
      if (config.oracle.auth.jwt && config.oracle.auth.jwt.enabled) {
        console.log('- JWT Authentication (Enabled)');
        console.log(`  Header: ${config.oracle.auth.jwt.header}`);
        console.log(`  Format: ${config.oracle.auth.jwt.header_format}`);
        console.log(`  Issuer: ${config.oracle.auth.jwt.issuer}`);
        console.log(`  Expiration: ${config.oracle.auth.jwt.expiration} seconds`);
      }
      
      if (config.oracle.auth.blockchain && config.oracle.auth.blockchain.enabled) {
        console.log('- Blockchain Authentication (Enabled)');
        console.log(`  Network: ${config.oracle.auth.blockchain.network}`);
        console.log(`  Signature Header: ${config.oracle.auth.blockchain.signature_header}`);
        console.log(`  Public Key Header: ${config.oracle.auth.blockchain.public_key_header}`);
      }
      
      console.log('\nRole-Based Access Control:');
      if (config.oracle.rbac && config.oracle.rbac.enabled) {
        console.log('- RBAC Enabled');
        console.log('  Roles:');
        config.oracle.rbac.roles.forEach(role => {
          console.log(`  - ${role.name}: ${role.permissions.join(', ')}`);
        });
      } else {
        console.log('- RBAC Disabled');
      }
      
      console.log('\nAccess Monitoring and Auditing:');
      if (config.oracle.auditing && config.oracle.auditing.enabled) {
        console.log('- Auditing Enabled');
        console.log(`  Log Level: ${config.oracle.auditing.log_level}`);
        console.log(`  Storage: ${config.oracle.auditing.storage}`);
        console.log(`  Retention: ${config.oracle.auditing.retention_days} days`);
        
        if (config.oracle.auditing.alert_on_suspicious) {
          console.log('  Suspicious Activity Detection Enabled');
          console.log(`  Max Failed Attempts: ${config.oracle.auditing.suspicious_rules.max_failed_attempts}`);
          console.log(`  Failed Attempts Window: ${config.oracle.auditing.suspicious_rules.failed_attempts_window} seconds`);
          console.log(`  Max Requests Per Minute: ${config.oracle.auditing.suspicious_rules.max_requests_per_minute}`);
        }
      } else {
        console.log('- Auditing Disabled');
      }
      
      console.log('\nThe oracle authentication service function is now registered and will start providing secure authentication for oracle services.');
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
