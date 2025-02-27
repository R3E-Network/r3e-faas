/**
 * GraphQL API Client for Neo N3 FaaS Platform
 * 
 * This script demonstrates how to interact with the GraphQL API in the Neo N3 FaaS platform.
 */

const axios = require('axios');
const fs = require('fs');
const path = require('path');
const readline = require('readline');

// Configuration
const API_URL = process.env.R3E_API_URL || 'http://localhost:8080/api/graphql';
const API_KEY = process.env.R3E_API_KEY;

// Check if API key is provided
if (!API_KEY) {
  console.error('Error: R3E_API_KEY environment variable is required');
  console.error('Please set it using: export R3E_API_KEY=your_api_key');
  process.exit(1);
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

// Execute GraphQL query
async function executeQuery(query, variables = {}) {
  try {
    const response = await axios.post(API_URL, {
      query,
      variables
    }, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    if (response.data.errors) {
      console.error('GraphQL Errors:');
      response.data.errors.forEach((error, index) => {
        console.error(`${index + 1}. ${error.message}`);
        if (error.locations) {
          console.error(`   Location: Line ${error.locations[0].line}, Column ${error.locations[0].column}`);
        }
        if (error.path) {
          console.error(`   Path: ${error.path.join('.')}`);
        }
      });
      return null;
    }
    
    return response.data.data;
  } catch (error) {
    console.error('Error executing GraphQL query:', error.message);
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    }
    return null;
  }
}

// Get all services
async function getServices() {
  const query = `
    query GetServices {
      services {
        id
        name
        description
        version
        functions {
          id
          name
        }
      }
    }
  `;
  
  console.log('Fetching all services...');
  const data = await executeQuery(query);
  
  if (data && data.services) {
    console.log('\nServices:');
    data.services.forEach((service, index) => {
      console.log(`\n${index + 1}. ${service.name} (ID: ${service.id})`);
      console.log(`   Description: ${service.description || 'No description'}`);
      console.log(`   Version: ${service.version}`);
      console.log(`   Functions: ${service.functions.length}`);
      
      if (service.functions.length > 0) {
        console.log('   Functions:');
        service.functions.forEach((func, funcIndex) => {
          console.log(`     ${funcIndex + 1}. ${func.name} (ID: ${func.id})`);
        });
      }
    });
    
    return data.services;
  }
  
  return [];
}

// Get service details
async function getServiceDetails(serviceId) {
  const query = `
    query GetService($id: ID!) {
      service(id: $id) {
        id
        name
        description
        version
        functions {
          id
          name
          description
          handler
          trigger {
            type
            ... on HttpTrigger {
              path
              methods
            }
            ... on EventTrigger {
              source
              event
            }
            ... on ScheduleTrigger {
              cron
              timezone
            }
          }
        }
        dependencies {
          name
          version
        }
        permissions {
          invoke {
            type
            id
          }
          manage {
            type
            id
          }
        }
        resources {
          memory
          timeout
        }
        environment {
          name
          value
        }
        storage {
          enabled
          retentionDays
        }
        createdAt
        updatedAt
        owner {
          id
          username
        }
      }
    }
  `;
  
  const variables = { id: serviceId };
  
  console.log(`Fetching details for service with ID: ${serviceId}...`);
  const data = await executeQuery(query, variables);
  
  if (data && data.service) {
    console.log('\nService Details:');
    console.log(`Name: ${data.service.name}`);
    console.log(`Description: ${data.service.description || 'No description'}`);
    console.log(`Version: ${data.service.version}`);
    console.log(`Created: ${new Date(data.service.createdAt).toLocaleString()}`);
    console.log(`Updated: ${new Date(data.service.updatedAt).toLocaleString()}`);
    console.log(`Owner: ${data.service.owner ? data.service.owner.username : 'Unknown'}`);
    
    console.log('\nFunctions:');
    data.service.functions.forEach((func, index) => {
      console.log(`\n${index + 1}. ${func.name} (ID: ${func.id})`);
      console.log(`   Description: ${func.description || 'No description'}`);
      console.log(`   Handler: ${func.handler}`);
      console.log(`   Trigger Type: ${func.trigger.type}`);
      
      if (func.trigger.type === 'http') {
        console.log(`   HTTP Path: ${func.trigger.path}`);
        console.log(`   HTTP Methods: ${func.trigger.methods.join(', ')}`);
      } else if (func.trigger.type === 'event') {
        console.log(`   Event Source: ${func.trigger.source}`);
        console.log(`   Event Type: ${func.trigger.event}`);
      } else if (func.trigger.type === 'schedule') {
        console.log(`   Cron: ${func.trigger.cron}`);
        console.log(`   Timezone: ${func.trigger.timezone || 'UTC'}`);
      }
    });
    
    console.log('\nDependencies:');
    if (data.service.dependencies.length === 0) {
      console.log('   No dependencies');
    } else {
      data.service.dependencies.forEach((dep, index) => {
        console.log(`   ${index + 1}. ${dep.name} (${dep.version})`);
      });
    }
    
    console.log('\nPermissions:');
    console.log('   Invoke:');
    data.service.permissions.invoke.forEach((perm, index) => {
      console.log(`     ${index + 1}. ${perm.type}: ${perm.id}`);
    });
    console.log('   Manage:');
    data.service.permissions.manage.forEach((perm, index) => {
      console.log(`     ${index + 1}. ${perm.type}: ${perm.id}`);
    });
    
    console.log('\nResources:');
    console.log(`   Memory: ${data.service.resources.memory}`);
    console.log(`   Timeout: ${data.service.resources.timeout}`);
    
    console.log('\nEnvironment Variables:');
    if (data.service.environment.length === 0) {
      console.log('   No environment variables');
    } else {
      data.service.environment.forEach((env, index) => {
        console.log(`   ${index + 1}. ${env.name}: ${env.value}`);
      });
    }
    
    console.log('\nStorage:');
    console.log(`   Enabled: ${data.service.storage.enabled}`);
    if (data.service.storage.enabled) {
      console.log(`   Retention Days: ${data.service.storage.retentionDays || 'Unlimited'}`);
    }
    
    return data.service;
  }
  
  return null;
}

// Get function details
async function getFunctionDetails(functionId) {
  const query = `
    query GetFunction($id: ID!) {
      function(id: $id) {
        id
        name
        description
        handler
        trigger {
          type
          ... on HttpTrigger {
            path
            methods
          }
          ... on EventTrigger {
            source
            event
          }
          ... on ScheduleTrigger {
            cron
            timezone
          }
        }
        service {
          id
          name
        }
        executions(limit: 5) {
          id
          status
          startTime
          endTime
          duration
        }
        logs(limit: 5) {
          timestamp
          level
          message
        }
        metrics {
          invocations {
            total
            success
            failed
            avgDuration
          }
          resources {
            memory {
              avg
              peak
            }
            cpu {
              avg
              peak
            }
          }
        }
        createdAt
        updatedAt
      }
    }
  `;
  
  const variables = { id: functionId };
  
  console.log(`Fetching details for function with ID: ${functionId}...`);
  const data = await executeQuery(query, variables);
  
  if (data && data.function) {
    console.log('\nFunction Details:');
    console.log(`Name: ${data.function.name}`);
    console.log(`Description: ${data.function.description || 'No description'}`);
    console.log(`Handler: ${data.function.handler}`);
    console.log(`Service: ${data.function.service.name} (ID: ${data.function.service.id})`);
    console.log(`Created: ${new Date(data.function.createdAt).toLocaleString()}`);
    console.log(`Updated: ${new Date(data.function.updatedAt).toLocaleString()}`);
    
    console.log('\nTrigger:');
    console.log(`   Type: ${data.function.trigger.type}`);
    
    if (data.function.trigger.type === 'http') {
      console.log(`   HTTP Path: ${data.function.trigger.path}`);
      console.log(`   HTTP Methods: ${data.function.trigger.methods.join(', ')}`);
    } else if (data.function.trigger.type === 'event') {
      console.log(`   Event Source: ${data.function.trigger.source}`);
      console.log(`   Event Type: ${data.function.trigger.event}`);
    } else if (data.function.trigger.type === 'schedule') {
      console.log(`   Cron: ${data.function.trigger.cron}`);
      console.log(`   Timezone: ${data.function.trigger.timezone || 'UTC'}`);
    }
    
    console.log('\nRecent Executions:');
    if (data.function.executions.length === 0) {
      console.log('   No recent executions');
    } else {
      data.function.executions.forEach((exec, index) => {
        console.log(`   ${index + 1}. ID: ${exec.id}`);
        console.log(`      Status: ${exec.status}`);
        console.log(`      Started: ${new Date(exec.startTime).toLocaleString()}`);
        if (exec.endTime) {
          console.log(`      Ended: ${new Date(exec.endTime).toLocaleString()}`);
        }
        if (exec.duration) {
          console.log(`      Duration: ${exec.duration}ms`);
        }
      });
    }
    
    console.log('\nRecent Logs:');
    if (data.function.logs.length === 0) {
      console.log('   No recent logs');
    } else {
      data.function.logs.forEach((log, index) => {
        const timestamp = new Date(log.timestamp).toLocaleString();
        console.log(`   ${timestamp} [${log.level.toUpperCase()}] ${log.message}`);
      });
    }
    
    console.log('\nMetrics:');
    const metrics = data.function.metrics;
    console.log('   Invocations:');
    console.log(`      Total: ${metrics.invocations.total}`);
    console.log(`      Success: ${metrics.invocations.success}`);
    console.log(`      Failed: ${metrics.invocations.failed}`);
    console.log(`      Average Duration: ${metrics.invocations.avgDuration}ms`);
    
    console.log('   Resources:');
    console.log(`      Memory: Avg ${metrics.resources.memory.avg}MB, Peak ${metrics.resources.memory.peak}MB`);
    console.log(`      CPU: Avg ${metrics.resources.cpu.avg}%, Peak ${metrics.resources.cpu.peak}%`);
    
    return data.function;
  }
  
  return null;
}

// Create a new service
async function createService(input) {
  const query = `
    mutation CreateService($input: CreateServiceInput!) {
      createService(input: $input) {
        id
        name
        description
        version
      }
    }
  `;
  
  const variables = { input };
  
  console.log('Creating new service...');
  const data = await executeQuery(query, variables);
  
  if (data && data.createService) {
    console.log('\nService created successfully!');
    console.log(`ID: ${data.createService.id}`);
    console.log(`Name: ${data.createService.name}`);
    console.log(`Description: ${data.createService.description || 'No description'}`);
    console.log(`Version: ${data.createService.version}`);
    
    return data.createService;
  }
  
  return null;
}

// Update an existing service
async function updateService(id, input) {
  const query = `
    mutation UpdateService($id: ID!, $input: UpdateServiceInput!) {
      updateService(id: $id, input: $input) {
        id
        name
        description
        version
      }
    }
  `;
  
  const variables = { id, input };
  
  console.log(`Updating service with ID: ${id}...`);
  const data = await executeQuery(query, variables);
  
  if (data && data.updateService) {
    console.log('\nService updated successfully!');
    console.log(`ID: ${data.updateService.id}`);
    console.log(`Name: ${data.updateService.name}`);
    console.log(`Description: ${data.updateService.description || 'No description'}`);
    console.log(`Version: ${data.updateService.version}`);
    
    return data.updateService;
  }
  
  return null;
}

// Delete a service
async function deleteService(id) {
  const query = `
    mutation DeleteService($id: ID!) {
      deleteService(id: $id)
    }
  `;
  
  const variables = { id };
  
  console.log(`Deleting service with ID: ${id}...`);
  const data = await executeQuery(query, variables);
  
  if (data && data.deleteService === true) {
    console.log('\nService deleted successfully!');
    return true;
  }
  
  return false;
}

// Invoke a function
async function invokeFunction(id, input) {
  const query = `
    mutation InvokeFunction($id: ID!, $input: InvokeFunctionInput!) {
      invokeFunction(id: $id, input: $input) {
        execution {
          id
          status
          startTime
          endTime
          duration
        }
        statusCode
        headers
        body
      }
    }
  `;
  
  const variables = { id, input };
  
  console.log(`Invoking function with ID: ${id}...`);
  const data = await executeQuery(query, variables);
  
  if (data && data.invokeFunction) {
    console.log('\nFunction invoked successfully!');
    console.log(`Execution ID: ${data.invokeFunction.execution.id}`);
    console.log(`Status: ${data.invokeFunction.execution.status}`);
    console.log(`Status Code: ${data.invokeFunction.statusCode}`);
    
    if (data.invokeFunction.headers) {
      console.log('\nResponse Headers:');
      const headers = JSON.parse(data.invokeFunction.headers);
      Object.entries(headers).forEach(([key, value]) => {
        console.log(`   ${key}: ${value}`);
      });
    }
    
    if (data.invokeFunction.body) {
      console.log('\nResponse Body:');
      try {
        const body = JSON.parse(data.invokeFunction.body);
        console.log(JSON.stringify(body, null, 2));
      } catch (error) {
        console.log(data.invokeFunction.body);
      }
    }
    
    return data.invokeFunction;
  }
  
  return null;
}

// Get user information
async function getUserInfo() {
  const query = `
    query GetUserInfo {
      me {
        id
        username
        email
        roles
        services {
          id
          name
        }
        createdAt
        updatedAt
      }
    }
  `;
  
  console.log('Fetching user information...');
  const data = await executeQuery(query);
  
  if (data && data.me) {
    console.log('\nUser Information:');
    console.log(`ID: ${data.me.id}`);
    console.log(`Username: ${data.me.username}`);
    console.log(`Email: ${data.me.email}`);
    console.log(`Roles: ${data.me.roles.join(', ')}`);
    console.log(`Created: ${new Date(data.me.createdAt).toLocaleString()}`);
    console.log(`Updated: ${new Date(data.me.updatedAt).toLocaleString()}`);
    
    console.log('\nServices:');
    if (data.me.services.length === 0) {
      console.log('   No services');
    } else {
      data.me.services.forEach((service, index) => {
        console.log(`   ${index + 1}. ${service.name} (ID: ${service.id})`);
      });
    }
    
    return data.me;
  }
  
  return null;
}

// Interactive client
async function interactiveClient() {
  try {
    console.log('\n=== Neo N3 FaaS GraphQL API Client ===\n');
    
    // Get user information
    const user = await getUserInfo();
    
    if (!user) {
      console.error('Error: Could not retrieve user information');
      process.exit(1);
    }
    
    while (true) {
      console.log('\nAvailable Actions:');
      console.log('1. List Services');
      console.log('2. Get Service Details');
      console.log('3. Get Function Details');
      console.log('4. Create Service');
      console.log('5. Update Service');
      console.log('6. Delete Service');
      console.log('7. Invoke Function');
      console.log('8. Exit');
      
      const action = await prompt('\nSelect an action (1-8): ');
      
      if (action === '1') {
        // List services
        await getServices();
      } else if (action === '2') {
        // Get service details
        const serviceId = await prompt('Enter service ID: ');
        await getServiceDetails(serviceId);
      } else if (action === '3') {
        // Get function details
        const functionId = await prompt('Enter function ID: ');
        await getFunctionDetails(functionId);
      } else if (action === '4') {
        // Create service
        console.log('\nCreating a new service:');
        
        const name = await prompt('Enter service name: ');
        const description = await prompt('Enter service description (optional): ');
        const version = await prompt('Enter service version: ');
        
        const input = {
          name,
          description: description || undefined,
          version,
          functions: []
        };
        
        const addFunction = await prompt('Do you want to add a function? (y/n): ');
        
        if (addFunction.toLowerCase() === 'y' || addFunction.toLowerCase() === 'yes') {
          const functionName = await prompt('Enter function name: ');
          const functionDescription = await prompt('Enter function description (optional): ');
          const handler = await prompt('Enter function handler (e.g., index.js:handler): ');
          const triggerType = await prompt('Enter trigger type (http, event, schedule): ');
          
          const functionInput = {
            name: functionName,
            description: functionDescription || undefined,
            handler,
            trigger: {
              type: triggerType
            }
          };
          
          if (triggerType === 'http') {
            const path = await prompt('Enter HTTP path: ');
            const methods = await prompt('Enter HTTP methods (comma-separated): ');
            
            functionInput.trigger.http = {
              path,
              methods: methods.split(',').map(m => m.trim())
            };
          } else if (triggerType === 'event') {
            const source = await prompt('Enter event source: ');
            const event = await prompt('Enter event type: ');
            
            functionInput.trigger.event = {
              source,
              event
            };
          } else if (triggerType === 'schedule') {
            const cron = await prompt('Enter cron expression: ');
            const timezone = await prompt('Enter timezone (optional): ');
            
            functionInput.trigger.schedule = {
              cron,
              timezone: timezone || undefined
            };
          }
          
          input.functions.push(functionInput);
        }
        
        await createService(input);
      } else if (action === '5') {
        // Update service
        const serviceId = await prompt('Enter service ID: ');
        
        // Get current service details
        const service = await getServiceDetails(serviceId);
        
        if (!service) {
          console.error('Error: Could not retrieve service details');
          continue;
        }
        
        console.log('\nUpdating service:');
        
        const name = await prompt(`Enter new service name (${service.name}): `);
        const description = await prompt(`Enter new service description (${service.description || 'No description'}): `);
        const version = await prompt(`Enter new service version (${service.version}): `);
        
        const input = {
          name: name || undefined,
          description: description || undefined,
          version: version || undefined
        };
        
        await updateService(serviceId, input);
      } else if (action === '6') {
        // Delete service
        const serviceId = await prompt('Enter service ID: ');
        const confirm = await prompt(`Are you sure you want to delete service with ID ${serviceId}? (y/n): `);
        
        if (confirm.toLowerCase() === 'y' || confirm.toLowerCase() === 'yes') {
          await deleteService(serviceId);
        }
      } else if (action === '7') {
        // Invoke function
        const functionId = await prompt('Enter function ID: ');
        
        console.log('\nInvoking function:');
        
        const method = await prompt('Enter HTTP method (GET, POST, etc.): ');
        const path = await prompt('Enter path (optional): ');
        
        console.log('Enter headers (JSON format, optional):');
        const headersStr = await prompt('');
        
        console.log('Enter body (JSON format, optional):');
        const bodyStr = await prompt('');
        
        const input = {
          method,
          path: path || undefined,
          headers: headersStr || undefined,
          body: bodyStr || undefined
        };
        
        await invokeFunction(functionId, input);
      } else if (action === '8') {
        // Exit
        console.log('\nExiting...');
        break;
      } else {
        console.error('Invalid action. Please try again.');
      }
    }
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
  
  if (command === 'services') {
    // List services
    await getServices();
  } else if (command === 'service') {
    // Get service details
    const serviceId = args[1];
    
    if (!serviceId) {
      console.error('Error: Service ID is required');
      console.error('Usage: node client.js service <serviceId>');
      process.exit(1);
    }
    
    await getServiceDetails(serviceId);
  } else if (command === 'function') {
    // Get function details
    const functionId = args[1];
    
    if (!functionId) {
      console.error('Error: Function ID is required');
      console.error('Usage: node client.js function <functionId>');
      process.exit(1);
    }
    
    await getFunctionDetails(functionId);
  } else if (command === 'invoke') {
    // Invoke function
    const functionId = args[1];
    const payloadFile = args[2];
    
    if (!functionId) {
      console.error('Error: Function ID is required');
      console.error('Usage: node client.js invoke <functionId> [payloadFile]');
      process.exit(1);
    }
    
    let payload = {
      method: 'GET'
    };
    
    if (payloadFile) {
      try {
        const payloadStr = fs.readFileSync(payloadFile, 'utf8');
        payload = JSON.parse(payloadStr);
      } catch (error) {
        console.error(`Error reading payload file: ${error.message}`);
        process.exit(1);
      }
    }
    
    await invokeFunction(functionId, payload);
  } else if (command === 'create') {
    // Create service
    const configFile = args[1];
    
    if (!configFile) {
      console.error('Error: Config file is required');
      console.error('Usage: node client.js create <configFile>');
      process.exit(1);
    }
    
    try {
      const configStr = fs.readFileSync(configFile, 'utf8');
      const config = JSON.parse(configStr);
      
      await createService(config);
    } catch (error) {
      console.error(`Error reading config file: ${error.message}`);
      process.exit(1);
    }
  } else if (command === 'update') {
    // Update service
    const serviceId = args[1];
    const configFile = args[2];
    
    if (!serviceId || !configFile) {
      console.error('Error: Service ID and config file are required');
      console.error('Usage: node client.js update <serviceId> <configFile>');
      process.exit(1);
    }
    
    try {
      const configStr = fs.readFileSync(configFile, 'utf8');
      const config = JSON.parse(configStr);
      
      await updateService(serviceId, config);
    } catch (error) {
      console.error(`Error reading config file: ${error.message}`);
      process.exit(1);
    }
  } else if (command === 'delete') {
    // Delete service
    const serviceId = args[1];
    
    if (!serviceId) {
      console.error('Error: Service ID is required');
      console.error('Usage: node client.js delete <serviceId>');
      process.exit(1);
    }
    
    await deleteService(serviceId);
  } else if (command === 'user') {
    // Get user information
    await getUserInfo();
  } else if (command === 'interactive') {
    // Interactive client
    await interactiveClient();
  } else if (command === 'help') {
    // Display help
    console.log('Neo N3 GraphQL API Client Example');
    console.log('\nUsage:');
    console.log('  node client.js <command> [options]');
    console.log('\nCommands:');
    console.log('  services                        List all services');
    console.log('  service <serviceId>             Get service details');
    console.log('  function <functionId>           Get function details');
    console.log('  invoke <functionId> [payloadFile] Invoke a function');
    console.log('  create <configFile>             Create a new service');
    console.log('  update <serviceId> <configFile> Update an existing service');
    console.log('  delete <serviceId>              Delete a service');
    console.log('  user                            Get user information');
    console.log('  interactive                     Start interactive client');
    console.log('  help                            Display this help message');
    console.log('\nExamples:');
    console.log('  node client.js services');
    console.log('  node client.js service service123');
    console.log('  node client.js function function123');
    console.log('  node client.js invoke function123 payload.json');
    console.log('  node client.js create service-config.json');
    console.log('  node client.js update service123 service-update.json');
    console.log('  node client.js delete service123');
  } else {
    console.error('Unknown command:', command);
    console.error('Run "node client.js help" for usage information');
    process.exit(1);
  }
  
  // Close readline interface if it's open
  if (rl.close) {
    rl.close();
  }
}

// Execute the main function
main().catch(error => {
  console.error('Unhandled error:', error);
  process.exit(1);
});
