/**
 * Registration script for Neo N3 Oracle Rate Limiting Example
 * 
 * This script registers the oracle rate limiting service function with the Neo N3 FaaS platform.
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
          rate_limiting: config.oracle.rate_limiting
        }
      }
    };
    
    // Register the function
    console.log('Registering oracle rate limiting service function with Neo N3 FaaS platform...');
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
      
      // Display information about the rate limiting configuration
      console.log('\nOracle Rate Limiting Configuration:');
      
      console.log('Rate Limiting Strategies:');
      if (config.oracle.rate_limiting.strategies.fixed_window && config.oracle.rate_limiting.strategies.fixed_window.enabled) {
        console.log('- Fixed Window Rate Limiting (Enabled)');
        console.log('  Default Limits:');
        console.log(`  - Per Second: ${config.oracle.rate_limiting.strategies.fixed_window.default_limits.per_second}`);
        console.log(`  - Per Minute: ${config.oracle.rate_limiting.strategies.fixed_window.default_limits.per_minute}`);
        console.log(`  - Per Hour: ${config.oracle.rate_limiting.strategies.fixed_window.default_limits.per_hour}`);
        console.log(`  - Per Day: ${config.oracle.rate_limiting.strategies.fixed_window.default_limits.per_day}`);
      }
      
      if (config.oracle.rate_limiting.strategies.sliding_window && config.oracle.rate_limiting.strategies.sliding_window.enabled) {
        console.log('- Sliding Window Rate Limiting (Enabled)');
        console.log('  Default Windows:');
        config.oracle.rate_limiting.strategies.sliding_window.default_windows.forEach(window => {
          console.log(`  - ${window.window} seconds: ${window.max_requests} requests`);
        });
      }
      
      if (config.oracle.rate_limiting.strategies.token_bucket && config.oracle.rate_limiting.strategies.token_bucket.enabled) {
        console.log('- Token Bucket Rate Limiting (Enabled)');
        console.log('  Default Bucket:');
        console.log(`  - Capacity: ${config.oracle.rate_limiting.strategies.token_bucket.default_bucket.capacity}`);
        console.log(`  - Refill Rate: ${config.oracle.rate_limiting.strategies.token_bucket.default_bucket.refill_rate} tokens/second`);
      }
      
      if (config.oracle.rate_limiting.multi_level && config.oracle.rate_limiting.multi_level.enabled) {
        console.log('- Multi-Level Rate Limiting (Enabled)');
        console.log('  Levels:');
        config.oracle.rate_limiting.multi_level.levels.forEach(level => {
          console.log(`  - ${level.level} (Priority: ${level.priority})`);
        });
      }
      
      if (config.oracle.rate_limiting.adaptive && config.oracle.rate_limiting.adaptive.enabled) {
        console.log('- Adaptive Rate Limiting (Enabled)');
        console.log('  Thresholds:');
        console.log(`  - Low: ${config.oracle.rate_limiting.adaptive.thresholds.low}`);
        console.log(`  - Medium: ${config.oracle.rate_limiting.adaptive.thresholds.medium}`);
        console.log(`  - High: ${config.oracle.rate_limiting.adaptive.thresholds.high}`);
        console.log('  Adjustment Factors:');
        console.log(`  - Low: ${config.oracle.rate_limiting.adaptive.adjustment_factors.low}`);
        console.log(`  - Medium: ${config.oracle.rate_limiting.adaptive.adjustment_factors.medium}`);
        console.log(`  - High: ${config.oracle.rate_limiting.adaptive.adjustment_factors.high}`);
        console.log(`  - Critical: ${config.oracle.rate_limiting.adaptive.adjustment_factors.critical}`);
      }
      
      console.log('\nRate Limit Response Configuration:');
      console.log(`- Status Code: ${config.oracle.rate_limiting.response.status_code}`);
      console.log(`- Include Headers: ${config.oracle.rate_limiting.response.include_headers}`);
      console.log('- Headers:');
      console.log(`  - Limit: ${config.oracle.rate_limiting.response.headers.limit}`);
      console.log(`  - Remaining: ${config.oracle.rate_limiting.response.headers.remaining}`);
      console.log(`  - Reset: ${config.oracle.rate_limiting.response.headers.reset}`);
      console.log(`  - Retry After: ${config.oracle.rate_limiting.response.headers.retry_after}`);
      
      console.log('\nRate Limit Storage Configuration:');
      console.log(`- Type: ${config.oracle.rate_limiting.storage.type}`);
      console.log(`- Key Prefix: ${config.oracle.rate_limiting.storage.key_prefix}`);
      console.log(`- Expiration: ${config.oracle.rate_limiting.storage.expiration} seconds`);
      
      console.log('\nThe oracle rate limiting service function is now registered and will start providing rate limiting for oracle services.');
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
