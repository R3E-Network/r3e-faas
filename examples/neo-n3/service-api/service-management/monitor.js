/**
 * Service Monitoring Script for Neo N3 FaaS Platform
 * 
 * This script demonstrates how to monitor services in the Neo N3 FaaS platform.
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

// Get service logs
async function getServiceLogs(serviceId, options = {}) {
  try {
    const { limit = 100, level = null, startTime = null, endTime = null } = options;
    
    // Build query parameters
    const params = { limit };
    if (level) params.level = level;
    if (startTime) params.start_time = startTime;
    if (endTime) params.end_time = endTime;
    
    console.log('Getting logs for service with ID:', serviceId);
    const response = await axios.get(`${API_URL}/services/${serviceId}/logs`, {
      headers: { 'Authorization': `Bearer ${API_KEY}` },
      params
    });
    
    if (response.status === 200) {
      console.log('\nService logs retrieved successfully!');
      console.log('Total log entries:', response.data.length);
      
      response.data.forEach((log, index) => {
        const timestamp = new Date(log.timestamp).toISOString();
        const level = log.level.toUpperCase().padEnd(5);
        console.log(`${timestamp} [${level}] ${log.message}`);
      });
      
      return response.data;
    }
  } catch (error) {
    console.error('Error getting service logs:', error.message);
    return [];
  }
}

// Get service executions
async function getServiceExecutions(serviceId, options = {}) {
  try {
    const { limit = 100, status = null, startTime = null, endTime = null } = options;
    
    // Build query parameters
    const params = { limit };
    if (status) params.status = status;
    if (startTime) params.start_time = startTime;
    if (endTime) params.end_time = endTime;
    
    console.log('Getting executions for service with ID:', serviceId);
    const response = await axios.get(`${API_URL}/services/${serviceId}/executions`, {
      headers: { 'Authorization': `Bearer ${API_KEY}` },
      params
    });
    
    if (response.status === 200) {
      console.log('\nService executions retrieved successfully!');
      console.log('Total executions:', response.data.length);
      
      response.data.forEach((execution, index) => {
        const startTime = new Date(execution.start_time).toISOString();
        const endTime = execution.end_time ? new Date(execution.end_time).toISOString() : 'N/A';
        const duration = execution.duration ? `${execution.duration}ms` : 'N/A';
        const status = execution.status.toUpperCase();
        
        console.log(`\n${index + 1}. Execution ID: ${execution.id}`);
        console.log(`   Status: ${status}`);
        console.log(`   Started: ${startTime}`);
        console.log(`   Ended: ${endTime}`);
        console.log(`   Duration: ${duration}`);
        console.log(`   Function: ${execution.function_id}`);
      });
      
      return response.data;
    }
  } catch (error) {
    console.error('Error getting service executions:', error.message);
    return [];
  }
}

// Get execution logs
async function getExecutionLogs(serviceId, executionId) {
  try {
    console.log('Getting logs for execution with ID:', executionId);
    const response = await axios.get(`${API_URL}/services/${serviceId}/executions/${executionId}/logs`, {
      headers: { 'Authorization': `Bearer ${API_KEY}` }
    });
    
    if (response.status === 200) {
      console.log('\nExecution logs retrieved successfully!');
      console.log('Total log entries:', response.data.length);
      
      response.data.forEach((log, index) => {
        const timestamp = new Date(log.timestamp).toISOString();
        const level = log.level.toUpperCase().padEnd(5);
        console.log(`${timestamp} [${level}] ${log.message}`);
      });
      
      return response.data;
    }
  } catch (error) {
    console.error('Error getting execution logs:', error.message);
    return [];
  }
}

// Get service metrics
async function getServiceMetrics(serviceId, options = {}) {
  try {
    const { period = '1h', resolution = '1m' } = options;
    
    // Build query parameters
    const params = { period, resolution };
    
    console.log('Getting metrics for service with ID:', serviceId);
    const response = await axios.get(`${API_URL}/services/${serviceId}/metrics`, {
      headers: { 'Authorization': `Bearer ${API_KEY}` },
      params
    });
    
    if (response.status === 200) {
      console.log('\nService metrics retrieved successfully!');
      
      const metrics = response.data;
      
      console.log('\nExecution Metrics:');
      console.log(`Total Executions: ${metrics.executions.total}`);
      console.log(`Successful Executions: ${metrics.executions.success}`);
      console.log(`Failed Executions: ${metrics.executions.failed}`);
      console.log(`Average Duration: ${metrics.executions.avg_duration}ms`);
      
      console.log('\nResource Usage:');
      console.log(`Average Memory Usage: ${metrics.resources.memory.avg}MB`);
      console.log(`Peak Memory Usage: ${metrics.resources.memory.peak}MB`);
      console.log(`Average CPU Usage: ${metrics.resources.cpu.avg}%`);
      console.log(`Peak CPU Usage: ${metrics.resources.cpu.peak}%`);
      
      return metrics;
    }
  } catch (error) {
    console.error('Error getting service metrics:', error.message);
    return null;
  }
}

// Get service alerts
async function getServiceAlerts(serviceId) {
  try {
    console.log('Getting alerts for service with ID:', serviceId);
    const response = await axios.get(`${API_URL}/services/${serviceId}/alerts`, {
      headers: { 'Authorization': `Bearer ${API_KEY}` }
    });
    
    if (response.status === 200) {
      console.log('\nService alerts retrieved successfully!');
      console.log('Total alerts:', response.data.length);
      
      response.data.forEach((alert, index) => {
        const timestamp = new Date(alert.timestamp).toISOString();
        const severity = alert.severity.toUpperCase();
        
        console.log(`\n${index + 1}. Alert ID: ${alert.id}`);
        console.log(`   Severity: ${severity}`);
        console.log(`   Timestamp: ${timestamp}`);
        console.log(`   Message: ${alert.message}`);
      });
      
      return response.data;
    }
  } catch (error) {
    console.error('Error getting service alerts:', error.message);
    return [];
  }
}

// Watch service logs in real-time
async function watchServiceLogs(serviceId) {
  try {
    console.log('Watching logs for service with ID:', serviceId);
    console.log('Press Ctrl+C to stop watching logs');
    
    // Get initial logs
    const initialLogs = await getServiceLogs(serviceId, { limit: 10 });
    
    // Keep track of the latest log timestamp
    let latestTimestamp = initialLogs.length > 0 
      ? new Date(initialLogs[0].timestamp).getTime() 
      : Date.now();
    
    // Poll for new logs every 2 seconds
    const interval = setInterval(async () => {
      try {
        const newLogs = await axios.get(`${API_URL}/services/${serviceId}/logs`, {
          headers: { 'Authorization': `Bearer ${API_KEY}` },
          params: {
            limit: 100,
            start_time: new Date(latestTimestamp + 1).toISOString()
          }
        });
        
        if (newLogs.status === 200 && newLogs.data.length > 0) {
          newLogs.data.forEach(log => {
            const timestamp = new Date(log.timestamp).toISOString();
            const level = log.level.toUpperCase().padEnd(5);
            console.log(`${timestamp} [${level}] ${log.message}`);
            
            const logTimestamp = new Date(log.timestamp).getTime();
            if (logTimestamp > latestTimestamp) {
              latestTimestamp = logTimestamp;
            }
          });
        }
      } catch (error) {
        console.error('Error polling for new logs:', error.message);
      }
    }, 2000);
    
    // Handle process termination
    process.on('SIGINT', () => {
      clearInterval(interval);
      console.log('\nStopped watching logs');
      process.exit(0);
    });
  } catch (error) {
    console.error('Error watching service logs:', error.message);
    process.exit(1);
  }
}

// Main function
async function main() {
  const args = process.argv.slice(2);
  const command = args[0] || 'help';
  const serviceId = args[1] || readServiceId();
  
  if (command === 'logs') {
    // Get service logs
    const level = args[2];
    const limit = args[3] ? parseInt(args[3]) : 100;
    
    await getServiceLogs(serviceId, { level, limit });
  } else if (command === 'watch-logs') {
    // Watch service logs in real-time
    await watchServiceLogs(serviceId);
  } else if (command === 'executions') {
    // Get service executions
    const status = args[2];
    const limit = args[3] ? parseInt(args[3]) : 100;
    
    await getServiceExecutions(serviceId, { status, limit });
  } else if (command === 'execution-logs') {
    // Get execution logs
    const executionId = args[2];
    
    if (!executionId) {
      console.error('Error: Execution ID is required');
      process.exit(1);
    }
    
    await getExecutionLogs(serviceId, executionId);
  } else if (command === 'metrics') {
    // Get service metrics
    const period = args[2] || '1h';
    const resolution = args[3] || '1m';
    
    await getServiceMetrics(serviceId, { period, resolution });
  } else if (command === 'alerts') {
    // Get service alerts
    await getServiceAlerts(serviceId);
  } else if (command === 'dashboard') {
    // Display service dashboard
    console.log('=== Neo N3 FaaS Service Dashboard ===\n');
    
    // Get service details
    const serviceDetails = await getServiceDetails(serviceId);
    if (!serviceDetails) {
      console.error('Error: Could not retrieve service details');
      process.exit(1);
    }
    
    console.log('Service Information:');
    console.log(`Name: ${serviceDetails.name}`);
    console.log(`Description: ${serviceDetails.description}`);
    console.log(`Version: ${serviceDetails.version}`);
    console.log(`Functions: ${serviceDetails.functions.length}`);
    console.log('\n');
    
    // Get service metrics
    console.log('Service Metrics:');
    await getServiceMetrics(serviceId);
    console.log('\n');
    
    // Get recent executions
    console.log('Recent Executions:');
    await getServiceExecutions(serviceId, { limit: 5 });
    console.log('\n');
    
    // Get recent logs
    console.log('Recent Logs:');
    await getServiceLogs(serviceId, { limit: 10 });
    console.log('\n');
    
    // Get alerts
    console.log('Active Alerts:');
    await getServiceAlerts(serviceId);
    console.log('\n');
    
    console.log('=== Dashboard End ===');
  } else if (command === 'demo') {
    // Run a demo of service monitoring
    console.log('=== Neo N3 FaaS Service Monitoring Demo ===\n');
    
    // Step 1: Get service details
    console.log('Step 1: Getting service details...\n');
    const serviceDetails = await getServiceDetails(serviceId);
    console.log('\n');
    
    // Step 2: Get service logs
    console.log('Step 2: Getting service logs...\n');
    await getServiceLogs(serviceId, { limit: 5 });
    console.log('\n');
    
    // Step 3: Get service executions
    console.log('Step 3: Getting service executions...\n');
    const executions = await getServiceExecutions(serviceId, { limit: 5 });
    console.log('\n');
    
    // Step 4: Get service metrics
    console.log('Step 4: Getting service metrics...\n');
    await getServiceMetrics(serviceId);
    console.log('\n');
    
    // Step 5: Get execution logs (if there are executions)
    if (executions && executions.length > 0) {
      const executionId = executions[0].id;
      console.log(`Step 5: Getting logs for execution with ID: ${executionId}...\n`);
      await getExecutionLogs(serviceId, executionId);
      console.log('\n');
    }
    
    // Step 6: Get service alerts
    console.log('Step 6: Getting service alerts...\n');
    await getServiceAlerts(serviceId);
    console.log('\n');
    
    console.log('=== Demo completed successfully ===');
  } else if (command === 'help') {
    // Display help
    console.log('Neo N3 Service Monitoring Example');
    console.log('\nUsage:');
    console.log('  node monitor.js <command> <serviceId> [options]');
    console.log('\nCommands:');
    console.log('  logs <serviceId> [level] [limit]        Get service logs');
    console.log('  watch-logs <serviceId>                  Watch service logs in real-time');
    console.log('  executions <serviceId> [status] [limit] Get service executions');
    console.log('  execution-logs <serviceId> <executionId> Get execution logs');
    console.log('  metrics <serviceId> [period] [resolution] Get service metrics');
    console.log('  alerts <serviceId>                      Get service alerts');
    console.log('  dashboard <serviceId>                   Display service dashboard');
    console.log('  demo <serviceId>                        Run a demo of service monitoring');
    console.log('  help                                    Display this help message');
    console.log('\nExamples:');
    console.log('  node monitor.js logs service123 error 50');
    console.log('  node monitor.js watch-logs service123');
    console.log('  node monitor.js executions service123 failed 20');
    console.log('  node monitor.js metrics service123 24h 5m');
  } else {
    console.error('Unknown command:', command);
    console.error('Run "node monitor.js help" for usage information');
    process.exit(1);
  }
}

// Execute the main function
main().catch(error => {
  console.error('Unhandled error:', error);
  process.exit(1);
});
