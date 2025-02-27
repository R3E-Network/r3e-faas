/**
 * Service Update Script for Neo N3 FaaS Platform
 * 
 * This script demonstrates how to update services in the Neo N3 FaaS platform.
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

// Update service description
async function updateServiceDescription(serviceId, newDescription) {
  // Get current service details
  const serviceDetails = await getServiceDetails(serviceId);
  if (!serviceDetails) {
    console.error('Error: Could not retrieve service details');
    process.exit(1);
  }
  
  // Create update payload with new description
  const updatePayload = {
    ...serviceDetails,
    description: newDescription
  };
  
  // Update the service
  return await updateService(serviceId, updatePayload);
}

// Update service version
async function updateServiceVersion(serviceId, newVersion) {
  // Get current service details
  const serviceDetails = await getServiceDetails(serviceId);
  if (!serviceDetails) {
    console.error('Error: Could not retrieve service details');
    process.exit(1);
  }
  
  // Create update payload with new version
  const updatePayload = {
    ...serviceDetails,
    version: newVersion
  };
  
  // Update the service
  return await updateService(serviceId, updatePayload);
}

// Add function to service
async function addFunctionToService(serviceId, functionData) {
  // Get current service details
  const serviceDetails = await getServiceDetails(serviceId);
  if (!serviceDetails) {
    console.error('Error: Could not retrieve service details');
    process.exit(1);
  }
  
  // Check if the function already exists
  const existingFunction = serviceDetails.functions.find(f => f.name === functionData.name);
  if (existingFunction) {
    console.error(`Error: Function "${functionData.name}" already exists in the service`);
    process.exit(1);
  }
  
  // Create update payload with new function
  const updatePayload = {
    ...serviceDetails,
    functions: [...serviceDetails.functions, functionData]
  };
  
  // Update the service
  return await updateService(serviceId, updatePayload);
}

// Update function in service
async function updateFunctionInService(serviceId, functionName, functionData) {
  // Get current service details
  const serviceDetails = await getServiceDetails(serviceId);
  if (!serviceDetails) {
    console.error('Error: Could not retrieve service details');
    process.exit(1);
  }
  
  // Check if the function exists
  const existingFunctionIndex = serviceDetails.functions.findIndex(f => f.name === functionName);
  if (existingFunctionIndex === -1) {
    console.error(`Error: Function "${functionName}" not found in the service`);
    process.exit(1);
  }
  
  // Create updated functions array
  const updatedFunctions = [...serviceDetails.functions];
  updatedFunctions[existingFunctionIndex] = functionData;
  
  // Create update payload with updated function
  const updatePayload = {
    ...serviceDetails,
    functions: updatedFunctions
  };
  
  // Update the service
  return await updateService(serviceId, updatePayload);
}

// Remove function from service
async function removeFunctionFromService(serviceId, functionName) {
  // Get current service details
  const serviceDetails = await getServiceDetails(serviceId);
  if (!serviceDetails) {
    console.error('Error: Could not retrieve service details');
    process.exit(1);
  }
  
  // Check if the function exists
  const existingFunction = serviceDetails.functions.find(f => f.name === functionName);
  if (!existingFunction) {
    console.error(`Error: Function "${functionName}" not found in the service`);
    process.exit(1);
  }
  
  // Create update payload with function removed
  const updatePayload = {
    ...serviceDetails,
    functions: serviceDetails.functions.filter(f => f.name !== functionName)
  };
  
  // Update the service
  return await updateService(serviceId, updatePayload);
}

// Add dependency to service
async function addDependencyToService(serviceId, dependencyData) {
  // Get current service details
  const serviceDetails = await getServiceDetails(serviceId);
  if (!serviceDetails) {
    console.error('Error: Could not retrieve service details');
    process.exit(1);
  }
  
  // Check if the dependency already exists
  const existingDependency = serviceDetails.dependencies.find(d => d.name === dependencyData.name);
  if (existingDependency) {
    console.error(`Error: Dependency "${dependencyData.name}" already exists in the service`);
    process.exit(1);
  }
  
  // Create update payload with new dependency
  const updatePayload = {
    ...serviceDetails,
    dependencies: [...serviceDetails.dependencies, dependencyData]
  };
  
  // Update the service
  return await updateService(serviceId, updatePayload);
}

// Remove dependency from service
async function removeDependencyFromService(serviceId, dependencyName) {
  // Get current service details
  const serviceDetails = await getServiceDetails(serviceId);
  if (!serviceDetails) {
    console.error('Error: Could not retrieve service details');
    process.exit(1);
  }
  
  // Check if the dependency exists
  const existingDependency = serviceDetails.dependencies.find(d => d.name === dependencyName);
  if (!existingDependency) {
    console.error(`Error: Dependency "${dependencyName}" not found in the service`);
    process.exit(1);
  }
  
  // Create update payload with dependency removed
  const updatePayload = {
    ...serviceDetails,
    dependencies: serviceDetails.dependencies.filter(d => d.name !== dependencyName)
  };
  
  // Update the service
  return await updateService(serviceId, updatePayload);
}

// Update service permissions
async function updateServicePermissions(serviceId, permissionsData) {
  // Get current service details
  const serviceDetails = await getServiceDetails(serviceId);
  if (!serviceDetails) {
    console.error('Error: Could not retrieve service details');
    process.exit(1);
  }
  
  // Create update payload with new permissions
  const updatePayload = {
    ...serviceDetails,
    permissions: permissionsData
  };
  
  // Update the service
  return await updateService(serviceId, updatePayload);
}

// Main function
async function main() {
  const args = process.argv.slice(2);
  const updateType = args[0] || 'help';
  const serviceId = args[1] || readServiceId();
  
  if (updateType === 'description') {
    // Update service description
    const newDescription = args[2] || 'Updated service description';
    await updateServiceDescription(serviceId, newDescription);
  } else if (updateType === 'version') {
    // Update service version
    const newVersion = args[2] || incrementVersion(await getServiceDetails(serviceId).then(details => details.version));
    await updateServiceVersion(serviceId, newVersion);
  } else if (updateType === 'add-function') {
    // Add function to service
    const functionName = args[2];
    
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
  } else if (updateType === 'update-function') {
    // Update function in service
    const functionName = args[2];
    
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
      ...existingFunction,
      description: `Updated ${existingFunction.name} function`
    };
    
    await updateFunctionInService(serviceId, functionName, functionData);
  } else if (updateType === 'remove-function') {
    // Remove function from service
    const functionName = args[2];
    
    if (!functionName) {
      console.error('Error: Function name is required');
      process.exit(1);
    }
    
    await removeFunctionFromService(serviceId, functionName);
  } else if (updateType === 'add-dependency') {
    // Add dependency to service
    const dependencyName = args[2];
    const dependencyVersion = args[3] || '^1.0.0';
    
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
  } else if (updateType === 'remove-dependency') {
    // Remove dependency from service
    const dependencyName = args[2];
    
    if (!dependencyName) {
      console.error('Error: Dependency name is required');
      process.exit(1);
    }
    
    await removeDependencyFromService(serviceId, dependencyName);
  } else if (updateType === 'permissions') {
    // Update service permissions
    
    // Create new permissions data
    const permissionsData = {
      invoke: [
        { type: "user", id: "*" },
        { type: "role", id: "admin" }
      ],
      manage: [
        { type: "user", id: "owner" },
        { type: "role", id: "admin" }
      ]
    };
    
    await updateServicePermissions(serviceId, permissionsData);
  } else if (updateType === 'help') {
    // Display help
    console.log('Neo N3 Service Update Example');
    console.log('\nUsage:');
    console.log('  node update.js <update-type> <serviceId> [options]');
    console.log('\nUpdate Types:');
    console.log('  description <serviceId> <newDescription>  Update service description');
    console.log('  version <serviceId> <newVersion>          Update service version');
    console.log('  add-function <serviceId> <functionName>   Add a function to the service');
    console.log('  update-function <serviceId> <functionName> Update a function in the service');
    console.log('  remove-function <serviceId> <functionName> Remove a function from the service');
    console.log('  add-dependency <serviceId> <name> <version> Add a dependency to the service');
    console.log('  remove-dependency <serviceId> <name>      Remove a dependency from the service');
    console.log('  permissions <serviceId>                   Update service permissions');
    console.log('  help                                      Display this help message');
    console.log('\nExamples:');
    console.log('  node update.js description service123 "New description"');
    console.log('  node update.js version service123 1.1.0');
    console.log('  node update.js add-function service123 newFunction');
  } else {
    console.error('Unknown update type:', updateType);
    console.error('Run "node update.js help" for usage information');
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
