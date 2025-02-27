/**
 * Registration script for Neo N3 TEE Integration Example
 * 
 * This script registers the TEE integration service function with the Neo N3 FaaS platform.
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
          neo_integration: config.tee.neo_integration
        }
      }
    };
    
    // Register the function
    console.log('Registering TEE integration service function with Neo N3 FaaS platform...');
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
      
      // Display information about the TEE integration configuration
      console.log('\nTEE Integration Configuration:');
      
      console.log('Supported TEE Types:');
      config.tee.neo_integration.provider.types.forEach(type => {
        if (type.enabled) {
          console.log(`- ${type.name}: ${type.description}`);
        }
      });
      
      console.log('\nIntegration Features:');
      
      if (config.tee.neo_integration.features.secure_contract_execution.enabled) {
        console.log('- Secure Smart Contract Execution:');
        config.tee.neo_integration.features.secure_contract_execution.modes.forEach(mode => {
          if (mode.enabled) {
            console.log(`  - ${mode.name}: ${mode.description}`);
          }
        });
      }
      
      if (config.tee.neo_integration.features.verifiable_computation.enabled) {
        console.log('- Verifiable Off-Chain Computation:');
        config.tee.neo_integration.features.verifiable_computation.types.forEach(type => {
          if (type.enabled) {
            console.log(`  - ${type.name}: ${type.description}`);
          }
        });
      }
      
      if (config.tee.neo_integration.features.secure_key_management.enabled) {
        console.log('- Secure Key Management:');
        config.tee.neo_integration.features.secure_key_management.types.forEach(type => {
          if (type.enabled) {
            console.log(`  - ${type.name}: ${type.description}`);
          }
        });
      }
      
      if (config.tee.neo_integration.features.tee_oracle.enabled) {
        console.log('- TEE-Based Oracle:');
        config.tee.neo_integration.features.tee_oracle.data_types.forEach(type => {
          if (type.enabled) {
            console.log(`  - ${type.name}: ${type.description}`);
          }
        });
      }
      
      if (config.tee.neo_integration.features.blockchain_attestation.enabled) {
        console.log('- Blockchain-Based Attestation:');
        config.tee.neo_integration.features.blockchain_attestation.types.forEach(type => {
          if (type.enabled) {
            console.log(`  - ${type.name}: ${type.description}`);
          }
        });
      }
      
      console.log('\nNeo N3 Smart Contract Integration:');
      
      if (config.tee.neo_integration.neo.contracts.verifier.hash) {
        console.log(`- TEE Verifier Contract: ${config.tee.neo_integration.neo.contracts.verifier.hash}`);
        config.tee.neo_integration.neo.contracts.verifier.operations.forEach(op => {
          if (op.enabled) {
            console.log(`  - ${op.name}`);
          }
        });
      }
      
      if (config.tee.neo_integration.neo.contracts.oracle.hash) {
        console.log(`- TEE Oracle Contract: ${config.tee.neo_integration.neo.contracts.oracle.hash}`);
        config.tee.neo_integration.neo.contracts.oracle.operations.forEach(op => {
          if (op.enabled) {
            console.log(`  - ${op.name}`);
          }
        });
      }
      
      if (config.tee.neo_integration.neo.contracts.computation.hash) {
        console.log(`- TEE Computation Verifier Contract: ${config.tee.neo_integration.neo.contracts.computation.hash}`);
        config.tee.neo_integration.neo.contracts.computation.operations.forEach(op => {
          if (op.enabled) {
            console.log(`  - ${op.name}`);
          }
        });
      }
      
      console.log('\nThe TEE integration service function is now registered and will start providing integration services between TEEs and Neo N3 blockchain applications.');
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
