/**
 * Service Management Script for Neo N3 FaaS Platform
 * 
 * This script demonstrates how to manage services in the Neo N3 FaaS platform.
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

// Helper function to read config
function readConfig() {
  try {
    const configPath = path.join(__dirname, 'config.yaml');
    return yaml.load(fs.readFileSync(configPath, 'utf8'));
  } catch (error) {
    console.error('Error reading configuration file:', error.message);
    process.exit(1);
  }
}

// List all services
async function listServices() {
  try {
    console.log('Listing all services...');
    const response = await axios.get(`${API_URL}/services`, {
      headers: { 'Authorization': `Bearer ${API_KEY}` }
    });
    
    if (response.status === 200) {
      console.log('\nServices retrieved successfully!');
      console.log('Total services:', response.data.length);
      
      response.data.forEach((service, index) => {
        console.log(`\n${index + 1}. ${service.name} (ID: ${service.id})`);
        console.log(`   Description: ${service.description}`);
        console.log(`   Version: ${service.version}`);
      });
      
      return response.data;
    }
  } catch (error) {
    console.error('Error listing services:', error.message);
    return [];
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
      console.log('Service ID:', response.data.id);
      console.log('Service Name:', response.data.name);
      console.log('Service Description:', response.data.description);
      console.log('Service Version:', response.data.version);
      
      return response.data;
    }
  } catch (error) {
    console.error('Error getting service details:', error.message);
    return null;
  }
}

// Update service
async function updateService(serviceId, updatePayload) {
  try {
    console.log('Updating service with ID:', serviceId);
    const response = await axios.put(`${API_URL}/services/${serviceId}`, updatePayload, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    if (response.status === 200) {
      console.log('\nService updated successfully!');
      return response.data;
    }
  } catch (error) {
    console.error('Error updating service:', error.message);
    return null;
  }
}

// Delete service
async function deleteService(serviceId) {
  try {
    console.log('Deleting service with ID:', serviceId);
    const response = await axios.delete(`${API_URL}/services/${serviceId}`, {
      headers: { 'Authorization': `Bearer ${API_KEY}` }
    });
    
    if (response.status === 204) {
      console.log('\nService deleted successfully!');
      return true;
    }
  } catch (error) {
    console.error('Error deleting service:', error.message);
    return false;
  }
}

// Create service version
async function createServiceVersion(serviceId, versionPayload) {
  try {
    console.log('Creating new version for service with ID:', serviceId);
    const response = await axios.post(`${API_URL}/services/${serviceId}/versions`, versionPayload, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    if (response.status === 201) {
      console.log('\nService version created successfully!');
      console.log('New Version ID:', response.data.id);
      return response.data;
    }
  } catch (error) {
    console.error('Error creating service version:', error.message);
    return null;
  }
}

// List service versions
async function listServiceVersions(serviceId) {
  try {
    console.log('Listing versions for service with ID:', serviceId);
    const response = await axios.get(`${API_URL}/services/${serviceId}/versions`, {
      headers: { 'Authorization': `Bearer ${API_KEY}` }
    });
    
    if (response.status === 200) {
      console.log('\nService versions retrieved successfully!');
      console.log('Total versions:', response.data.length);
      
      response.data.forEach((version, index) => {
        console.log(`\n${index + 1}. Version: ${version.version} (ID: ${version.id})`);
        console.log(`   Created: ${version.created_at}`);
      });
      
      return response.data;
    }
  } catch (error) {
    console.error('Error listing service versions:', error.message);
    return [];
  }
}

// Get service logs
async function getServiceLogs(serviceId) {
  try {
    console.log('Getting logs for service with ID:', serviceId);
    const response = await axios.get(`${API_URL}/services/${serviceId}/logs`, {
      headers: { 'Authorization': `Bearer ${API_KEY}` }
    });
    
    if (response.status === 200) {
      console.log('\nService logs retrieved successfully!');
      console.log('Total log entries:', response.data.length);
      
      response.data.forEach((log, index) => {
        console.log(`\n${index + 1}. [${log.timestamp}] ${log.level}: ${log.message}`);
      });
      
      return response.data;
    }
  } catch (error) {
    console.error('Error getting service logs:', error.message);
    return [];
  }
}

// Get service executions
async function getServiceExecutions(serviceId) {
  try {
    console.log('Getting executions for service with ID:', serviceId);
    const response = await axios.get(`${API_URL}/services/${serviceId}/executions`, {
      headers: { 'Authorization': `Bearer ${API_KEY}` }
    });
    
    if (response.status === 200) {
      console.log('\nService executions retrieved successfully!');
      console.log('Total executions:', response.data.length);
      
      response.data.forEach((execution, index) => {
        console.log(`\n${index + 1}. Execution ID: ${execution.id}`);
        console.log(`   Status: ${execution.status}`);
        console.log(`   Started: ${execution.start_time}`);
        console.log(`   Duration: ${execution.duration}ms`);
      });
      
      return response.data;
    }
  } catch (error) {
    console.error('Error getting service executions:', error.message);
    return [];
  }
}

// Add function to service
async function addFunctionToService(serviceId, functionData) {
  try {
    console.log('Adding function to service with ID:', serviceId);
    const response = await axios.post(`${API_URL}/services/${serviceId}/functions`, functionData, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    if (response.status === 200) {
      console.log('\nFunction added to service successfully!');
      return response.data;
    }
  } catch (error) {
    console.error('Error adding function to service:', error.message);
    return null;
  }
}

// Update function in service
async function updateFunctionInService(serviceId, functionName, functionData) {
  try {
    console.log(`Updating function "${functionName}" in service with ID: ${serviceId}`);
    const response = await axios.put(`${API_URL}/services/${serviceId}/functions/${functionName}`, functionData, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    if (response.status === 200) {
      console.log('\nFunction updated successfully!');
      return response.data;
    }
  } catch (error) {
    console.error('Error updating function in service:', error.message);
    return null;
  }
}

// Remove function from service
async function removeFunctionFromService(serviceId, functionName) {
  try {
    console.log(`Removing function "${functionName}" from service with ID: ${serviceId}`);
    const response = await axios.delete(`${API_URL}/services/${serviceId}/functions/${functionName}`, {
      headers: { 'Authorization': `Bearer ${API_KEY}` }
    });
    
    if (response.status === 204) {
      console.log('\nFunction removed successfully!');
      return true;
    }
  } catch (error) {
    console.error('Error removing function from service:', error.message);
    return false;
  }
}

// Add dependency to service
async function addDependencyToService(serviceId, dependencyData) {
  try {
    console.log('Adding dependency to service with ID:', serviceId);
    const response = await axios.post(`${API_URL}/services/${serviceId}/dependencies`, dependencyData, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    if (response.status === 200) {
      console.log('\nDependency added to service successfully!');
      return response.data;
    }
  } catch (error) {
    console.error('Error adding dependency to service:', error.message);
    return null;
  }
}

// Remove dependency from service
async function removeDependencyFromService(serviceId, dependencyName) {
  try {
    console.log(`Removing dependency "${dependencyName}" from service with ID: ${serviceId}`);
    const response = await axios.delete(`${API_URL}/services/${serviceId}/dependencies/${dependencyName}`, {
      headers: { 'Authorization': `Bearer ${API_KEY}` }
    });
    
    if (response.status === 204) {
      console.log('\nDependency removed successfully!');
      return true;
    }
  } catch (error) {
    console.error('Error removing dependency from service:', error.message);
    return false;
  }
}

// Update service permissions
async function updateServicePermissions(serviceId, permissionsData) {
  try {
    console.log('Updating permissions for service with ID:', serviceId);
    const response = await axios.put(`${API_URL}/services/${serviceId}/permissions`, permissionsData, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    if (response.status === 200) {
      console.log('\nService permissions updated successfully!');
      return response.data;
    }
  } catch (error) {
    console.error('Error updating service permissions:', error.message);
    return null;
  }
}

// Main function
async function main() {
  const args = process.argv.slice(2);
  const command = args[0] || 'help';
  
  if (command === 'list') {
    // List all services
    await listServices();
  } else if (command === 'get') {
    // Get service details
    const serviceId = args[1] || readServiceId();
    await getServiceDetails(serviceId);
  } else if (command === 'update') {
    // Update service
    const serviceId = args[1] || readServiceId();
    
    // Get current service details
    const serviceDetails = await getServiceDetails(serviceId);
    if (!serviceDetails) {
      console.error('Error: Could not retrieve service details');
      process.exit(1);
    }
    
    // Read updated config
    const config = readConfig();
    
    // Create update payload
    const updatePayload = {
      name: config.name || serviceDetails.name,
      description: config.description || serviceDetails.description,
      version: config.version || serviceDetails.version,
      functions: config.functions || serviceDetails.functions,
      dependencies: config.dependencies || serviceDetails.dependencies,
      permissions: config.permissions || serviceDetails.permissions,
      resources: config.resources || serviceDetails.resources,
      environment: config.environment || serviceDetails.environment
    };
    
    // Update the service
    await updateService(serviceId, updatePayload);
  } else if (command === 'delete') {
    // Delete service
    const serviceId = args[1] || readServiceId();
    
    // Confirm deletion
    console.log(`Are you sure you want to delete service with ID: ${serviceId}? (y/n)`);
    process.stdin.once('data', async (data) => {
      const answer = data.toString().trim().toLowerCase();
      if (answer === 'y' || answer === 'yes') {
        await deleteService(serviceId);
        
        // Remove the service ID file if it exists
        const serviceIdPath = path.join(__dirname, '.service-id');
        if (fs.existsSync(serviceIdPath)) {
          fs.unlinkSync(serviceIdPath);
          console.log(`Service ID file ${serviceIdPath} removed`);
        }
      } else {
        console.log('Service deletion cancelled');
      }
      process.exit(0);
    });
  } else if (command === 'version') {
    // Service versioning
    const subCommand = args[1] || 'list';
    const serviceId = args[2] || readServiceId();
    
    if (subCommand === 'list') {
      // List service versions
      await listServiceVersions(serviceId);
    } else if (subCommand === 'create') {
      // Create a new service version
      
      // Get current service details
      const serviceDetails = await getServiceDetails(serviceId);
      if (!serviceDetails) {
        console.error('Error: Could not retrieve service details');
        process.exit(1);
      }
      
      // Read new version from config or increment current version
      const config = readConfig();
      const currentVersion = serviceDetails.version;
      const newVersion = config.version || incrementVersion(currentVersion);
      
      // Create version payload
      const versionPayload = {
        name: serviceDetails.name,
        description: serviceDetails.description,
        version: newVersion,
        functions: serviceDetails.functions,
        dependencies: serviceDetails.dependencies,
        permissions: serviceDetails.permissions,
        resources: serviceDetails.resources,
        environment: serviceDetails.environment
      };
      
      // Create the new version
      await createServiceVersion(serviceId, versionPayload);
    } else {
      console.error('Unknown version subcommand:', subCommand);
      console.error('Supported subcommands: list, create');
      process.exit(1);
    }
  } else if (command === 'logs') {
    // Get service logs
    const serviceId = args[1] || readServiceId();
    await getServiceLogs(serviceId);
  } else if (command === 'executions') {
    // Get service executions
    const serviceId = args[1] || readServiceId();
    await getServiceExecutions(serviceId);
  } else if (command === 'function') {
    // Function management
    const subCommand = args[1];
    const serviceId = args[2] || readServiceId();
    
    if (!subCommand) {
      console.error('Error: Function subcommand is required');
      console.error('Supported subcommands: add, update, remove');
      process.exit(1);
    }
    
    if (subCommand === 'add') {
      // Add a function to the service
      const functionName = args[3];
      
      if (!functionName) {
        console.error('Error: Function name is required');
        process.exit(1);
      }
      
      // Create function data
      const functionData = {
        name: functionName,
        description: `Function ${functionName} for the service`,
        handler: `functions/${functionName}.js:handler`,
        trigger: {
          type: "request",
          config: {
            http: {
              path: `/example-managed-service/${functionName}`,
              methods: ["GET", "POST"]
            }
          }
        }
      };
      
      await addFunctionToService(serviceId, functionData);
    } else if (subCommand === 'update') {
      // Update a function in the service
      const functionName = args[3];
      
      if (!functionName) {
        console.error('Error: Function name is required');
        process.exit(1);
      }
      
      // Get service details to get current function
      const serviceDetails = await getServiceDetails(serviceId);
      if (!serviceDetails) {
        console.error('Error: Could not retrieve service details');
        process.exit(1);
      }
      
      // Find the function to update
      const existingFunction = serviceDetails.functions.find(f => f.name === functionName);
      if (!existingFunction) {
        console.error(`Error: Function "${functionName}" not found in service`);
        process.exit(1);
      }
      
      // Create updated function data
      const functionData = {
        name: existingFunction.name,
        description: `Updated ${existingFunction.name} function`,
        handler: existingFunction.handler,
        trigger: existingFunction.trigger
      };
      
      await updateFunctionInService(serviceId, functionName, functionData);
    } else if (subCommand === 'remove') {
      // Remove a function from the service
      const functionName = args[3];
      
      if (!functionName) {
        console.error('Error: Function name is required');
        process.exit(1);
      }
      
      await removeFunctionFromService(serviceId, functionName);
    } else {
      console.error('Unknown function subcommand:', subCommand);
      console.error('Supported subcommands: add, update, remove');
      process.exit(1);
    }
  } else if (command === 'dependency') {
    // Dependency management
    const subCommand = args[1];
    const serviceId = args[2] || readServiceId();
    
    if (!subCommand) {
      console.error('Error: Dependency subcommand is required');
      console.error('Supported subcommands: add, remove');
      process.exit(1);
    }
    
    if (subCommand === 'add') {
      // Add a dependency to the service
      const dependencyName = args[3];
      const dependencyVersion = args[4] || '^1.0.0';
      
      if (!dependencyName) {
        console.error('Error: Dependency name is required');
        process.exit(1);
      }
      
      // Create dependency data
      const dependencyData = {
        name: dependencyName,
        version: dependencyVersion
      };
      
      await addDependencyToService(serviceId, dependencyData);
    } else if (subCommand === 'remove') {
      // Remove a dependency from the service
      const dependencyName = args[3];
      
      if (!dependencyName) {
        console.error('Error: Dependency name is required');
        process.exit(1);
      }
      
      await removeDependencyFromService(serviceId, dependencyName);
    } else {
      console.error('Unknown dependency subcommand:', subCommand);
      console.error('Supported subcommands: add, remove');
      process.exit(1);
    }
  } else if (command === 'permissions') {
    // Update service permissions
    const serviceId = args[1] || readServiceId();
    
    // Get current service details
    const serviceDetails = await getServiceDetails(serviceId);
    if (!serviceDetails) {
      console.error('Error: Could not retrieve service details');
      process.exit(1);
    }
    
    // Read updated permissions from config
    const config = readConfig();
    
    // Create permissions payload
    const permissionsData = config.permissions || serviceDetails.permissions;
    
    await updateServicePermissions(serviceId, permissionsData);
  } else if (command === 'demo') {
    // Run a demo of service management
    console.log('=== Neo N3 FaaS Service Management Demo ===\n');
    
    // Step 1: List all services
    console.log('Step 1: Listing all services...\n');
    const services = await listServices();
    console.log('\n');
    
    if (services.length === 0) {
      console.log('No services found. Please register a service first.');
      process.exit(0);
    }
    
    // Use the first service for the demo
    const serviceId = services[0].id;
    
    // Step 2: Get service details
    console.log(`Step 2: Getting details for service with ID: ${serviceId}...\n`);
    const serviceDetails = await getServiceDetails(serviceId);
    console.log('\n');
    
    // Step 3: Update service description
    console.log('Step 3: Updating service description...\n');
    const updatePayload = {
      ...serviceDetails,
      description: `${serviceDetails.description} (Updated by demo)`
    };
    await updateService(serviceId, updatePayload);
    console.log('\n');
    
    // Step 4: Create a new version
    console.log('Step 4: Creating a new version...\n');
    const newVersion = incrementVersion(serviceDetails.version);
    const versionPayload = {
      ...serviceDetails,
      version: newVersion
    };
    await createServiceVersion(serviceId, versionPayload);
    console.log('\n');
    
    // Step 5: List service versions
    console.log('Step 5: Listing service versions...\n');
    await listServiceVersions(serviceId);
    console.log('\n');
    
    // Step 6: Add a new function
    console.log('Step 6: Adding a new function...\n');
    const newFunction = {
      name: "demo-function",
      description: "Demo function added by the management script",
      handler: "functions/demo-function.js:handler",
      trigger: {
        type: "request",
        config: {
          http: {
            path: "/example-managed-service/demo-function",
            methods: ["GET", "POST"]
          }
        }
      }
    };
    await addFunctionToService(serviceId, newFunction);
    console.log('\n');
    
    // Step 7: Update the function
    console.log('Step 7: Updating the function...\n');
    const updatedFunction = {
      ...newFunction,
      description: "Updated demo function"
    };
    await updateFunctionInService(serviceId, "demo-function", updatedFunction);
    console.log('\n');
    
    // Step 8: Add a new dependency
    console.log('Step 8: Adding a new dependency...\n');
    const newDependency = {
      name: "demo-dependency",
      version: "^1.0.0"
    };
    await addDependencyToService(serviceId, newDependency);
    console.log('\n');
    
    // Step 9: Update permissions
    console.log('Step 9: Updating permissions...\n');
    const newPermissions = {
      invoke: [
        { type: "user", id: "*" },
        { type: "role", id: "admin" }
      ],
      manage: [
        { type: "user", id: "owner" },
        { type: "role", id: "admin" }
      ]
    };
    await updateServicePermissions(serviceId, newPermissions);
    console.log('\n');
    
    // Step 10: Get service logs
    console.log('Step 10: Getting service logs...\n');
    await getServiceLogs(serviceId);
    console.log('\n');
    
    // Step 11: Get service executions
    console.log('Step 11: Getting service executions...\n');
    await getServiceExecutions(serviceId);
    console.log('\n');
    
    // Step 12: Clean up (remove the added function and dependency)
    console.log('Step 12: Cleaning up...\n');
    await removeFunctionFromService(serviceId, "demo-function");
    await removeDependencyFromService(serviceId, "demo-dependency");
    
    // Restore original service description
    const restorePayload = {
      ...serviceDetails,
      description: serviceDetails.description
    };
    await updateService(serviceId, restorePayload);
    console.log('\n');
    
    console.log('=== Demo completed successfully ===');
  } else if (command === 'help') {
    // Display help
    console.log('Neo N3 Service Management Example');
    console.log('\nUsage:');
    console.log('  node manage.js <command> [options]');
    console.log('\nCommands:');
    console.log('  list                                     List all services');
    console.log('  get <serviceId>                          Get service details');
    console.log('  update <serviceId>                       Update service');
    console.log('  delete <serviceId>                       Delete service');
    console.log('  version list <serviceId>                 List service versions');
    console.log('  version create <serviceId>               Create a new service version');
    console.log('  logs <serviceId>                         Get service logs');
    console.log('  executions <serviceId>                   Get service executions');
    console.log('  function add <serviceId> <functionName>  Add a function to the service');
    console.log('  function update <serviceId> <functionName> Update a function in the service');
    console.log('  function remove <serviceId> <functionName> Remove a function from the service');
    console.log('  dependency add <serviceId> <name> <version> Add a dependency to the service');
    console.log('  dependency remove <serviceId> <name>     Remove a dependency from the service');
    console.log('  permissions <serviceId>                  Update service permissions');
    console.log('  demo                                     Run a demo of service management');
    console.log('  help                                     Display this help message');
    console.log('\nExamples:');
    console.log('  node manage.js list');
    console.log('  node manage.js get service123');
    console.log('  node manage.js update service123');
    console.log('  node manage.js function add service123 newFunction');
    console.log('  node manage.js demo');
  } else {
    console.error('Unknown command:', command);
    console.error('Run "node manage.js help" for usage information');
    process.exit(1);
  }
}

// Helper function to increment version
function incrementVersion(version) {
  const parts = version.split('.');
  if (parts.length !== 3) {
    return '1.0.0';
  }
  
  const major = parseInt(parts[0]);
  const minor = parseInt(parts[1]);
  const patch = parseInt(parts[2]);
  
  // Increment patch version
  return `${major}.${minor}.${patch + 1}`;
}

// Execute the main function
main().catch(error => {
  console.error('Unhandled error:', error);
  process.exit(1);
});
