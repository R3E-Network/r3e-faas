/**
 * Service Management Client for Neo N3 FaaS Platform
 * 
 * This script demonstrates how to interact with services in the Neo N3 FaaS platform.
 */

const axios = require('axios');
const fs = require('fs');
const path = require('path');
const readline = require('readline');

// Configuration
const API_URL = process.env.R3E_API_URL || 'http://localhost:8080/api';
const API_KEY = process.env.R3E_API_KEY;

// Check if API key is provided
if (!API_KEY) {
  console.error('Error: R3E_API_KEY environment variable is required');
  console.error('Please set it using: export R3E_API_KEY=your_api_key');
  process.exit(1);
}

// Helper function to read service ID
function readServiceId() {
  try {
    const serviceIdPath = path.join(__dirname, '.service-id');
    if (!fs.existsSync(serviceIdPath)) {
      console.error('Error: Service ID file not found');
      process.exit(1);
    }
    return fs.readFileSync(serviceIdPath, 'utf8').trim();
  } catch (error) {
    console.error('Error reading service ID:', error.message);
    process.exit(1);
  }
}

// Create readline interface for user input
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

// Helper function to prompt user for input
function prompt(question) {
  return new Promise((resolve) => {
    rl.question(question, (answer) => {
      resolve(answer);
    });
  });
}

// Get service details
async function getServiceDetails(serviceId) {
  try {
    console.log('Getting details for service with ID:', serviceId);
    const response = await axios.get(`${API_URL}/services/${serviceId}`, {
      headers: { 'Authorization': `Bearer ${API_KEY}` }
    });
    
    if (response.status === 200) {
      console.log('\nService details retrieved successfully!');
      return response.data;
    }
  } catch (error) {
    console.error('Error getting service details:', error.message);
    return null;
  }
}

// List service functions
async function listServiceFunctions(serviceId) {
  try {
    // Get service details
    const serviceDetails = await getServiceDetails(serviceId);
    if (!serviceDetails) {
      console.error('Error: Could not retrieve service details');
      return [];
    }
    
    // Extract functions
    const functions = serviceDetails.functions || [];
    
    console.log('\nService Functions:');
    functions.forEach((func, index) => {
      console.log(`\n${index + 1}. ${func.name}`);
      console.log(`   Description: ${func.description}`);
      console.log(`   Handler: ${func.handler}`);
      console.log(`   Trigger: ${func.trigger.type}`);
      
      if (func.trigger.type === 'request') {
        console.log(`   HTTP Path: ${func.trigger.config.http.path}`);
        console.log(`   HTTP Methods: ${func.trigger.config.http.methods.join(', ')}`);
      }
    });
    
    return functions;
  } catch (error) {
    console.error('Error listing service functions:', error.message);
    return [];
  }
}

// Invoke service function
async function invokeServiceFunction(serviceId, functionName, payload) {
  try {
    console.log(`Invoking function "${functionName}" in service with ID: ${serviceId}`);
    const response = await axios.post(`${API_URL}/services/${serviceId}/functions/${functionName}/invoke`, payload, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    if (response.status === 200) {
      console.log('\nFunction invoked successfully!');
      console.log('Response:', JSON.stringify(response.data, null, 2));
      return response.data;
    }
  } catch (error) {
    console.error('Error invoking function:', error.message);
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    }
    return null;
  }
}

// Get function logs
async function getFunctionLogs(serviceId, functionName, limit = 10) {
  try {
    console.log(`Getting logs for function "${functionName}" in service with ID: ${serviceId}`);
    const response = await axios.get(`${API_URL}/services/${serviceId}/functions/${functionName}/logs`, {
      headers: { 'Authorization': `Bearer ${API_KEY}` },
      params: { limit }
    });
    
    if (response.status === 200) {
      console.log('\nFunction logs retrieved successfully!');
      console.log('Total log entries:', response.data.length);
      
      response.data.forEach((log, index) => {
        const timestamp = new Date(log.timestamp).toISOString();
        const level = log.level.toUpperCase().padEnd(5);
        console.log(`${timestamp} [${level}] ${log.message}`);
      });
      
      return response.data;
    }
  } catch (error) {
    console.error('Error getting function logs:', error.message);
    return [];
  }
}

// Get function metrics
async function getFunctionMetrics(serviceId, functionName, period = '1h') {
  try {
    console.log(`Getting metrics for function "${functionName}" in service with ID: ${serviceId}`);
    const response = await axios.get(`${API_URL}/services/${serviceId}/functions/${functionName}/metrics`, {
      headers: { 'Authorization': `Bearer ${API_KEY}` },
      params: { period }
    });
    
    if (response.status === 200) {
      console.log('\nFunction metrics retrieved successfully!');
      
      const metrics = response.data;
      
      console.log('\nExecution Metrics:');
      console.log(`Total Invocations: ${metrics.invocations.total}`);
      console.log(`Successful Invocations: ${metrics.invocations.success}`);
      console.log(`Failed Invocations: ${metrics.invocations.failed}`);
      console.log(`Average Duration: ${metrics.invocations.avg_duration}ms`);
      
      console.log('\nResource Usage:');
      console.log(`Average Memory Usage: ${metrics.resources.memory.avg}MB`);
      console.log(`Peak Memory Usage: ${metrics.resources.memory.peak}MB`);
      console.log(`Average CPU Usage: ${metrics.resources.cpu.avg}%`);
      console.log(`Peak CPU Usage: ${metrics.resources.cpu.peak}%`);
      
      return metrics;
    }
  } catch (error) {
    console.error('Error getting function metrics:', error.message);
    return null;
  }
}

// Interactive client
async function interactiveClient(serviceId) {
  try {
    // Get service details
    const serviceDetails = await getServiceDetails(serviceId);
    if (!serviceDetails) {
      console.error('Error: Could not retrieve service details');
      process.exit(1);
    }
    
    console.log('\n=== Neo N3 FaaS Service Client ===');
    console.log(`Service: ${serviceDetails.name} (ID: ${serviceId})`);
    console.log(`Description: ${serviceDetails.description}`);
    console.log(`Version: ${serviceDetails.version}`);
    console.log('===============================\n');
    
    // List functions
    const functions = await listServiceFunctions(serviceId);
    
    if (functions.length === 0) {
      console.error('Error: No functions found in the service');
      process.exit(1);
    }
    
    // Prompt user to select a function
    const functionIndex = await prompt(`\nSelect a function to invoke (1-${functions.length}): `);
    const selectedFunction = functions[parseInt(functionIndex) - 1];
    
    if (!selectedFunction) {
      console.error('Error: Invalid function selection');
      process.exit(1);
    }
    
    console.log(`\nSelected function: ${selectedFunction.name}`);
    console.log(`Description: ${selectedFunction.description}`);
    
    // Prompt user for payload
    console.log('\nEnter payload (JSON format):');
    const payloadStr = await prompt('');
    
    let payload;
    try {
      payload = JSON.parse(payloadStr);
    } catch (error) {
      console.error('Error parsing payload:', error.message);
      process.exit(1);
    }
    
    // Invoke the function
    await invokeServiceFunction(serviceId, selectedFunction.name, payload);
    
    // Ask if user wants to see logs
    const showLogs = await prompt('\nDo you want to see function logs? (y/n): ');
    if (showLogs.toLowerCase() === 'y' || showLogs.toLowerCase() === 'yes') {
      await getFunctionLogs(serviceId, selectedFunction.name);
    }
    
    // Ask if user wants to see metrics
    const showMetrics = await prompt('\nDo you want to see function metrics? (y/n): ');
    if (showMetrics.toLowerCase() === 'y' || showMetrics.toLowerCase() === 'yes') {
      await getFunctionMetrics(serviceId, selectedFunction.name);
    }
    
    console.log('\n=== Client session completed ===');
  } catch (error) {
    console.error('Error in interactive client:', error.message);
  } finally {
    rl.close();
  }
}

// Main function
async function main() {
  const args = process.argv.slice(2);
  const command = args[0] || 'interactive';
  const serviceId = args[1] || readServiceId();
  
  if (command === 'list-functions') {
    // List service functions
    await listServiceFunctions(serviceId);
    rl.close();
  } else if (command === 'invoke') {
    // Invoke service function
    const functionName = args[2];
    const payloadFile = args[3];
    
    if (!functionName) {
      console.error('Error: Function name is required');
      console.error('Usage: node client.js invoke <serviceId> <functionName> [payloadFile]');
      process.exit(1);
    }
    
    let payload = {};
    
    if (payloadFile) {
      try {
        const payloadStr = fs.readFileSync(payloadFile, 'utf8');
        payload = JSON.parse(payloadStr);
      } catch (error) {
        console.error(`Error reading payload file: ${error.message}`);
        process.exit(1);
      }
    }
    
    await invokeServiceFunction(serviceId, functionName, payload);
    rl.close();
  } else if (command === 'logs') {
    // Get function logs
    const functionName = args[2];
    const limit = args[3] ? parseInt(args[3]) : 10;
    
    if (!functionName) {
      console.error('Error: Function name is required');
      console.error('Usage: node client.js logs <serviceId> <functionName> [limit]');
      process.exit(1);
    }
    
    await getFunctionLogs(serviceId, functionName, limit);
    rl.close();
  } else if (command === 'metrics') {
    // Get function metrics
    const functionName = args[2];
    const period = args[3] || '1h';
    
    if (!functionName) {
      console.error('Error: Function name is required');
      console.error('Usage: node client.js metrics <serviceId> <functionName> [period]');
      process.exit(1);
    }
    
    await getFunctionMetrics(serviceId, functionName, period);
    rl.close();
  } else if (command === 'interactive') {
    // Interactive client
    await interactiveClient(serviceId);
  } else if (command === 'demo') {
    // Run a demo of service client
    console.log('=== Neo N3 FaaS Service Client Demo ===\n');
    
    // Step 1: Get service details
    console.log('Step 1: Getting service details...\n');
    const serviceDetails = await getServiceDetails(serviceId);
    console.log('\n');
    
    if (!serviceDetails) {
      console.error('Error: Could not retrieve service details');
      process.exit(1);
    }
    
    // Step 2: List service functions
    console.log('Step 2: Listing service functions...\n');
    const functions = await listServiceFunctions(serviceId);
    console.log('\n');
    
    if (functions.length === 0) {
      console.error('Error: No functions found in the service');
      process.exit(1);
    }
    
    // Step 3: Invoke the index function
    const indexFunction = functions.find(f => f.name === 'index') || functions[0];
    console.log(`Step 3: Invoking function "${indexFunction.name}"...\n`);
    
    const payload = {
      name: 'Demo User',
      action: 'test',
      timestamp: new Date().toISOString()
    };
    
    await invokeServiceFunction(serviceId, indexFunction.name, payload);
    console.log('\n');
    
    // Step 4: Get function logs
    console.log(`Step 4: Getting logs for function "${indexFunction.name}"...\n`);
    await getFunctionLogs(serviceId, indexFunction.name);
    console.log('\n');
    
    // Step 5: Get function metrics
    console.log(`Step 5: Getting metrics for function "${indexFunction.name}"...\n`);
    await getFunctionMetrics(serviceId, indexFunction.name);
    console.log('\n');
    
    console.log('=== Demo completed successfully ===');
    rl.close();
  } else if (command === 'help') {
    // Display help
    console.log('Neo N3 Service Client Example');
    console.log('\nUsage:');
    console.log('  node client.js <command> <serviceId> [options]');
    console.log('\nCommands:');
    console.log('  list-functions <serviceId>                List service functions');
    console.log('  invoke <serviceId> <functionName> [payloadFile] Invoke a service function');
    console.log('  logs <serviceId> <functionName> [limit]   Get function logs');
    console.log('  metrics <serviceId> <functionName> [period] Get function metrics');
    console.log('  interactive <serviceId>                   Start interactive client');
    console.log('  demo <serviceId>                          Run a demo of service client');
    console.log('  help                                      Display this help message');
    console.log('\nExamples:');
    console.log('  node client.js list-functions service123');
    console.log('  node client.js invoke service123 index payload.json');
    console.log('  node client.js logs service123 index 20');
    console.log('  node client.js metrics service123 index 24h');
    rl.close();
  } else {
    console.error('Unknown command:', command);
    console.error('Run "node client.js help" for usage information');
    rl.close();
    process.exit(1);
  }
}

// Execute the main function
main().catch(error => {
  console.error('Unhandled error:', error);
  rl.close();
  process.exit(1);
});
