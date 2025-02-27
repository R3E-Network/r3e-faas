/**
 * Monitoring script for Function Management Example
 * 
 * This script demonstrates how to monitor function execution and logs in the Neo N3 FaaS platform.
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

// Function to get function logs
async function getFunctionLogs(functionId, options = {}) {
  try {
    console.log('Getting logs for function with ID:', functionId);
    
    // Prepare query parameters
    const params = {};
    if (options.limit) params.limit = options.limit;
    if (options.startTime) params.start_time = options.startTime;
    if (options.endTime) params.end_time = options.endTime;
    if (options.level) params.level = options.level;
    
    // Get the function logs
    const response = await axios.get(`${API_URL}/functions/${functionId}/logs`, {
      params,
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
        response.data.forEach((log) => {
          const timestamp = new Date(log.timestamp).toISOString();
          console.log(`[${timestamp}] ${log.level.toUpperCase()}: ${log.message}`);
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
async function getFunctionExecutions(functionId, options = {}) {
  try {
    console.log('Getting executions for function with ID:', functionId);
    
    // Prepare query parameters
    const params = {};
    if (options.limit) params.limit = options.limit;
    if (options.startTime) params.start_time = options.startTime;
    if (options.endTime) params.end_time = options.endTime;
    if (options.status) params.status = options.status;
    
    // Get the function executions
    const response = await axios.get(`${API_URL}/functions/${functionId}/executions`, {
      params,
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
          console.log(`   Started: ${new Date(execution.start_time).toISOString()}`);
          console.log(`   Duration: ${execution.duration}ms`);
          console.log(`   Trigger: ${execution.trigger_type}`);
          
          if (execution.status === 'failed' && execution.error) {
            console.log(`   Error: ${execution.error.message}`);
          }
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

// Function to get execution details
async function getExecutionDetails(functionId, executionId) {
  try {
    console.log('Getting details for execution with ID:', executionId);
    
    // Get the execution details
    const response = await axios.get(`${API_URL}/functions/${functionId}/executions/${executionId}`, {
      headers: {
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 200) {
      console.log('\nExecution details retrieved successfully!');
      console.log('Execution ID:', response.data.id);
      console.log('Status:', response.data.status);
      console.log('Started:', new Date(response.data.start_time).toISOString());
      console.log('Duration:', response.data.duration, 'ms');
      console.log('Trigger:', response.data.trigger_type);
      
      if (response.data.request) {
        console.log('\nRequest:');
        console.log('Method:', response.data.request.method);
        console.log('Path:', response.data.request.path);
        console.log('Query:', JSON.stringify(response.data.request.query, null, 2));
        console.log('Headers:', JSON.stringify(response.data.request.headers, null, 2));
        console.log('Body:', JSON.stringify(response.data.request.body, null, 2));
      }
      
      if (response.data.response) {
        console.log('\nResponse:');
        console.log('Status Code:', response.data.response.statusCode);
        console.log('Headers:', JSON.stringify(response.data.response.headers, null, 2));
        console.log('Body:', JSON.stringify(response.data.response.body, null, 2));
      }
      
      if (response.data.status === 'failed' && response.data.error) {
        console.log('\nError:');
        console.log('Message:', response.data.error.message);
        console.log('Stack:', response.data.error.stack);
      }
      
      return response.data;
    } else {
      console.error('Error getting execution details:', response.data);
      return null;
    }
  } catch (error) {
    console.error('Error getting execution details:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return null;
  }
}

// Function to get function metrics
async function getFunctionMetrics(functionId, options = {}) {
  try {
    console.log('Getting metrics for function with ID:', functionId);
    
    // Prepare query parameters
    const params = {};
    if (options.startTime) params.start_time = options.startTime;
    if (options.endTime) params.end_time = options.endTime;
    if (options.interval) params.interval = options.interval;
    
    // Get the function metrics
    const response = await axios.get(`${API_URL}/functions/${functionId}/metrics`, {
      params,
      headers: {
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 200) {
      console.log('\nFunction metrics retrieved successfully!');
      
      // Display invocation metrics
      if (response.data.invocations) {
        console.log('\nInvocation Metrics:');
        console.log('Total Invocations:', response.data.invocations.total);
        console.log('Successful Invocations:', response.data.invocations.successful);
        console.log('Failed Invocations:', response.data.invocations.failed);
        console.log('Success Rate:', `${response.data.invocations.success_rate}%`);
      }
      
      // Display performance metrics
      if (response.data.performance) {
        console.log('\nPerformance Metrics:');
        console.log('Average Duration:', response.data.performance.avg_duration, 'ms');
        console.log('Min Duration:', response.data.performance.min_duration, 'ms');
        console.log('Max Duration:', response.data.performance.max_duration, 'ms');
        console.log('P95 Duration:', response.data.performance.p95_duration, 'ms');
        console.log('P99 Duration:', response.data.performance.p99_duration, 'ms');
      }
      
      // Display resource metrics
      if (response.data.resources) {
        console.log('\nResource Metrics:');
        console.log('Average Memory Usage:', response.data.resources.avg_memory, 'MB');
        console.log('Max Memory Usage:', response.data.resources.max_memory, 'MB');
        console.log('Average CPU Usage:', response.data.resources.avg_cpu, '%');
        console.log('Max CPU Usage:', response.data.resources.max_cpu, '%');
      }
      
      // Display error metrics
      if (response.data.errors) {
        console.log('\nError Metrics:');
        console.log('Total Errors:', response.data.errors.total);
        
        if (response.data.errors.types && response.data.errors.types.length > 0) {
          console.log('\nError Types:');
          response.data.errors.types.forEach((errorType, index) => {
            console.log(`${index + 1}. ${errorType.name}: ${errorType.count} (${errorType.percentage}%)`);
          });
        }
      }
      
      return response.data;
    } else {
      console.error('Error getting function metrics:', response.data);
      return null;
    }
  } catch (error) {
    console.error('Error getting function metrics:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return null;
  }
}

// Function to watch function logs in real-time
async function watchFunctionLogs(functionId) {
  try {
    console.log('Watching logs for function with ID:', functionId);
    console.log('Press Ctrl+C to stop watching logs');
    
    // Get the initial logs
    let lastTimestamp = new Date().toISOString();
    await getFunctionLogs(functionId, { startTime: lastTimestamp });
    
    // Set up polling interval
    const pollInterval = setInterval(async () => {
      const logs = await getFunctionLogs(functionId, { startTime: lastTimestamp });
      
      if (logs.length > 0) {
        // Update the last timestamp
        lastTimestamp = new Date(logs[logs.length - 1].timestamp).toISOString();
      }
    }, 5000); // Poll every 5 seconds
    
    // Handle process termination
    process.on('SIGINT', () => {
      clearInterval(pollInterval);
      console.log('\nStopped watching logs');
      process.exit(0);
    });
  } catch (error) {
    console.error('Error watching function logs:', error.message);
    process.exit(1);
  }
}

// Function to watch function executions in real-time
async function watchFunctionExecutions(functionId) {
  try {
    console.log('Watching executions for function with ID:', functionId);
    console.log('Press Ctrl+C to stop watching executions');
    
    // Get the initial executions
    let lastTimestamp = new Date().toISOString();
    await getFunctionExecutions(functionId, { startTime: lastTimestamp });
    
    // Set up polling interval
    const pollInterval = setInterval(async () => {
      const executions = await getFunctionExecutions(functionId, { startTime: lastTimestamp });
      
      if (executions.length > 0) {
        // Update the last timestamp
        lastTimestamp = new Date(executions[executions.length - 1].start_time).toISOString();
      }
    }, 5000); // Poll every 5 seconds
    
    // Handle process termination
    process.on('SIGINT', () => {
      clearInterval(pollInterval);
      console.log('\nStopped watching executions');
      process.exit(0);
    });
  } catch (error) {
    console.error('Error watching function executions:', error.message);
    process.exit(1);
  }
}

// Main function
async function main() {
  // Parse command line arguments
  const args = process.argv.slice(2);
  const command = args[0] || 'help';
  
  if (command === 'logs') {
    // Get function logs
    const functionId = args[1];
    const options = {};
    
    if (!functionId) {
      console.error('Error: Function ID is required');
      console.error('Usage: node monitor.js logs <functionId> [options]');
      process.exit(1);
    }
    
    // Parse options
    for (let i = 2; i < args.length; i++) {
      const [key, value] = args[i].split('=');
      if (key === 'limit') options.limit = parseInt(value);
      if (key === 'level') options.level = value;
      if (key === 'start') options.startTime = value;
      if (key === 'end') options.endTime = value;
    }
    
    await getFunctionLogs(functionId, options);
  } else if (command === 'executions') {
    // Get function executions
    const functionId = args[1];
    const options = {};
    
    if (!functionId) {
      console.error('Error: Function ID is required');
      console.error('Usage: node monitor.js executions <functionId> [options]');
      process.exit(1);
    }
    
    // Parse options
    for (let i = 2; i < args.length; i++) {
      const [key, value] = args[i].split('=');
      if (key === 'limit') options.limit = parseInt(value);
      if (key === 'status') options.status = value;
      if (key === 'start') options.startTime = value;
      if (key === 'end') options.endTime = value;
    }
    
    await getFunctionExecutions(functionId, options);
  } else if (command === 'execution') {
    // Get execution details
    const functionId = args[1];
    const executionId = args[2];
    
    if (!functionId || !executionId) {
      console.error('Error: Function ID and Execution ID are required');
      console.error('Usage: node monitor.js execution <functionId> <executionId>');
      process.exit(1);
    }
    
    await getExecutionDetails(functionId, executionId);
  } else if (command === 'metrics') {
    // Get function metrics
    const functionId = args[1];
    const options = {};
    
    if (!functionId) {
      console.error('Error: Function ID is required');
      console.error('Usage: node monitor.js metrics <functionId> [options]');
      process.exit(1);
    }
    
    // Parse options
    for (let i = 2; i < args.length; i++) {
      const [key, value] = args[i].split('=');
      if (key === 'start') options.startTime = value;
      if (key === 'end') options.endTime = value;
      if (key === 'interval') options.interval = value;
    }
    
    await getFunctionMetrics(functionId, options);
  } else if (command === 'watch-logs') {
    // Watch function logs in real-time
    const functionId = args[1];
    
    if (!functionId) {
      console.error('Error: Function ID is required');
      console.error('Usage: node monitor.js watch-logs <functionId>');
      process.exit(1);
    }
    
    await watchFunctionLogs(functionId);
  } else if (command === 'watch-executions') {
    // Watch function executions in real-time
    const functionId = args[1];
    
    if (!functionId) {
      console.error('Error: Function ID is required');
      console.error('Usage: node monitor.js watch-executions <functionId>');
      process.exit(1);
    }
    
    await watchFunctionExecutions(functionId);
  } else if (command === 'help') {
    // Display help
    console.log('Neo N3 Function Monitoring Example');
    console.log('\nUsage:');
    console.log('  node monitor.js <command> [options]');
    console.log('\nCommands:');
    console.log('  logs <functionId> [options]           Get function logs');
    console.log('    Options:');
    console.log('      limit=<number>                    Limit the number of logs returned');
    console.log('      level=<debug|info|warn|error>     Filter logs by level');
    console.log('      start=<ISO date>                  Start time for logs');
    console.log('      end=<ISO date>                    End time for logs');
    console.log('  executions <functionId> [options]     Get function executions');
    console.log('    Options:');
    console.log('      limit=<number>                    Limit the number of executions returned');
    console.log('      status=<success|failed|pending>   Filter executions by status');
    console.log('      start=<ISO date>                  Start time for executions');
    console.log('      end=<ISO date>                    End time for executions');
    console.log('  execution <functionId> <executionId>  Get execution details');
    console.log('  metrics <functionId> [options]        Get function metrics');
    console.log('    Options:');
    console.log('      start=<ISO date>                  Start time for metrics');
    console.log('      end=<ISO date>                    End time for metrics');
    console.log('      interval=<hour|day|week|month>    Interval for metrics aggregation');
    console.log('  watch-logs <functionId>               Watch function logs in real-time');
    console.log('  watch-executions <functionId>         Watch function executions in real-time');
    console.log('  help                                  Display this help message');
  } else {
    console.error('Unknown command:', command);
    console.error('Run "node monitor.js help" for usage information');
    process.exit(1);
  }
}

// Execute the main function
if (!['watch-logs', 'watch-executions'].includes(process.argv[2])) {
  main().catch(error => {
    console.error('Unhandled error:', error);
    process.exit(1);
  });
}
