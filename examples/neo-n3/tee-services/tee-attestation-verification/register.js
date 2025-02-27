/**
 * Registration script for Neo N3 TEE Attestation Verification Example
 * 
 * This script registers the TEE attestation verification service function with the Neo N3 FaaS platform.
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

// Function to read the library code
function readLibraryCode() {
  try {
    const libDir = path.join(__dirname, 'lib');
    const libFiles = fs.readdirSync(libDir).filter(file => file.endsWith('.js'));
    
    const libraries = {};
    for (const file of libFiles) {
      const filePath = path.join(libDir, file);
      const content = fs.readFileSync(filePath, 'utf8');
      libraries[file] = content;
    }
    
    return libraries;
  } catch (error) {
    console.error('Error reading library code:', error.message);
    return {}; // Return empty object if lib directory doesn't exist
  }
}

// Function to register the function with the FaaS platform
async function registerFunction() {
  try {
    // Read configuration and function code
    const config = readConfig();
    const code = readFunctionCode();
    const libraries = readLibraryCode();
    
    // Prepare the registration payload
    const payload = {
      name: config.name,
      description: config.description,
      code: code,
      libraries: libraries,
      metadata: {
        runtime: config.runtime,
        trigger_type: config.trigger.type,
        trigger_config: config.trigger.config,
        tee: {
          attestation_verification: config.tee.attestation_verification
        }
      }
    };
    
    // Register the function
    console.log('Registering TEE attestation verification service function with Neo N3 FaaS platform...');
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
      
      // Display information about the TEE configuration
      console.log('\nTEE Configuration:');
      
      console.log('Supported TEE Types:');
      config.tee.attestation_verification.provider.types.forEach(type => {
        if (type.enabled) {
          console.log(`- ${type.name}: ${type.description}`);
        }
      });
      
      console.log('\nAttestation Protocols:');
      
      if (config.tee.attestation_verification.protocols.sgx_epid.enabled) {
        console.log('- SGX EPID Protocol:');
        config.tee.attestation_verification.protocols.sgx_epid.steps.forEach(step => {
          if (step.enabled) {
            console.log(`  - ${step.name}: ${step.description}`);
          }
        });
      }
      
      if (config.tee.attestation_verification.protocols.sgx_dcap.enabled) {
        console.log('- SGX DCAP Protocol:');
        config.tee.attestation_verification.protocols.sgx_dcap.steps.forEach(step => {
          if (step.enabled) {
            console.log(`  - ${step.name}: ${step.description}`);
          }
        });
      }
      
      if (config.tee.attestation_verification.protocols.sev.enabled) {
        console.log('- AMD SEV Protocol:');
        config.tee.attestation_verification.protocols.sev.steps.forEach(step => {
          if (step.enabled) {
            console.log(`  - ${step.name}: ${step.description}`);
          }
        });
      }
      
      if (config.tee.attestation_verification.protocols.trustzone.enabled) {
        console.log('- ARM TrustZone Protocol:');
        config.tee.attestation_verification.protocols.trustzone.steps.forEach(step => {
          if (step.enabled) {
            console.log(`  - ${step.name}: ${step.description}`);
          }
        });
      }
      
      console.log('\nUse Cases:');
      
      if (config.tee.attestation_verification.use_cases.remote_attestation.enabled) {
        console.log('- Remote Attestation:');
        config.tee.attestation_verification.use_cases.remote_attestation.types.forEach(type => {
          if (type.enabled) {
            console.log(`  - ${type.name}: ${type.description}`);
          }
        });
      }
      
      if (config.tee.attestation_verification.use_cases.secure_channel.enabled) {
        console.log('- Secure Channel Establishment:');
        config.tee.attestation_verification.use_cases.secure_channel.types.forEach(type => {
          if (type.enabled) {
            console.log(`  - ${type.name}: ${type.description}`);
          }
        });
      }
      
      if (config.tee.attestation_verification.use_cases.tcb_verification.enabled) {
        console.log('- TCB Verification:');
        config.tee.attestation_verification.use_cases.tcb_verification.types.forEach(type => {
          if (type.enabled) {
            console.log(`  - ${type.name}: ${type.description}`);
          }
        });
      }
      
      console.log('\nThe TEE attestation verification service function is now registered and will start providing attestation verification services.');
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
