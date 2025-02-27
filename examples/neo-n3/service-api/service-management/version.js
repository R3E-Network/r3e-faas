/**
 * Service Versioning Script for Neo N3 FaaS Platform
 * 
 * This script demonstrates how to manage service versions in the Neo N3 FaaS platform.
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

// Get service version details
async function getServiceVersionDetails(serviceId, versionId) {
  try {
    console.log('Getting details for service version with ID:', versionId);
    const response = await axios.get(`${API_URL}/services/${serviceId}/versions/${versionId}`, {
      headers: { 'Authorization': `Bearer ${API_KEY}` }
    });
    
    if (response.status === 200) {
      console.log('\nService version details retrieved successfully!');
      return response.data;
    }
  } catch (error) {
    console.error('Error getting service version details:', error.message);
    return null;
  }
}

// Activate service version
async function activateServiceVersion(serviceId, versionId) {
  try {
    console.log('Activating service version with ID:', versionId);
    const response = await axios.post(`${API_URL}/services/${serviceId}/versions/${versionId}/activate`, {}, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    if (response.status === 200) {
      console.log('\nService version activated successfully!');
      return response.data;
    }
  } catch (error) {
    console.error('Error activating service version:', error.message);
    return null;
  }
}

// Delete service version
async function deleteServiceVersion(serviceId, versionId) {
  try {
    console.log('Deleting service version with ID:', versionId);
    const response = await axios.delete(`${API_URL}/services/${serviceId}/versions/${versionId}`, {
      headers: { 'Authorization': `Bearer ${API_KEY}` }
    });
    
    if (response.status === 204) {
      console.log('\nService version deleted successfully!');
      return true;
    }
  } catch (error) {
    console.error('Error deleting service version:', error.message);
    return false;
  }
}

// Create a new major version
async function createMajorVersion(serviceId) {
  // Get current service details
  const serviceDetails = await getServiceDetails(serviceId);
  if (!serviceDetails) {
    console.error('Error: Could not retrieve service details');
    process.exit(1);
  }
  
  // Parse current version
  const currentVersion = serviceDetails.version;
  const newVersion = incrementMajorVersion(currentVersion);
  
  // Create version payload
  const versionPayload = {
    ...serviceDetails,
    version: newVersion
  };
  
  // Create the new version
  return await createServiceVersion(serviceId, versionPayload);
}

// Create a new minor version
async function createMinorVersion(serviceId) {
  // Get current service details
  const serviceDetails = await getServiceDetails(serviceId);
  if (!serviceDetails) {
    console.error('Error: Could not retrieve service details');
    process.exit(1);
  }
  
  // Parse current version
  const currentVersion = serviceDetails.version;
  const newVersion = incrementMinorVersion(currentVersion);
  
  // Create version payload
  const versionPayload = {
    ...serviceDetails,
    version: newVersion
  };
  
  // Create the new version
  return await createServiceVersion(serviceId, versionPayload);
}

// Create a new patch version
async function createPatchVersion(serviceId) {
  // Get current service details
  const serviceDetails = await getServiceDetails(serviceId);
  if (!serviceDetails) {
    console.error('Error: Could not retrieve service details');
    process.exit(1);
  }
  
  // Parse current version
  const currentVersion = serviceDetails.version;
  const newVersion = incrementPatchVersion(currentVersion);
  
  // Create version payload
  const versionPayload = {
    ...serviceDetails,
    version: newVersion
  };
  
  // Create the new version
  return await createServiceVersion(serviceId, versionPayload);
}

// Helper function to increment major version
function incrementMajorVersion(version) {
  const parts = version.split('.');
  if (parts.length !== 3) {
    return '1.0.0';
  }
  
  const major = parseInt(parts[0]);
  
  // Increment major version, reset minor and patch
  return `${major + 1}.0.0`;
}

// Helper function to increment minor version
function incrementMinorVersion(version) {
  const parts = version.split('.');
  if (parts.length !== 3) {
    return '0.1.0';
  }
  
  const major = parseInt(parts[0]);
  const minor = parseInt(parts[1]);
  
  // Increment minor version, reset patch
  return `${major}.${minor + 1}.0`;
}

// Helper function to increment patch version
function incrementPatchVersion(version) {
  const parts = version.split('.');
  if (parts.length !== 3) {
    return '0.0.1';
  }
  
  const major = parseInt(parts[0]);
  const minor = parseInt(parts[1]);
  const patch = parseInt(parts[2]);
  
  // Increment patch version
  return `${major}.${minor}.${patch + 1}`;
}

// Main function
async function main() {
  const args = process.argv.slice(2);
  const command = args[0] || 'help';
  const serviceId = args[1] || readServiceId();
  
  if (command === 'list') {
    // List service versions
    await listServiceVersions(serviceId);
  } else if (command === 'create') {
    // Create a new version
    const versionType = args[2] || 'patch';
    
    if (versionType === 'major') {
      await createMajorVersion(serviceId);
    } else if (versionType === 'minor') {
      await createMinorVersion(serviceId);
    } else if (versionType === 'patch') {
      await createPatchVersion(serviceId);
    } else {
      console.error('Unknown version type:', versionType);
      console.error('Supported types: major, minor, patch');
      process.exit(1);
    }
  } else if (command === 'get') {
    // Get service version details
    const versionId = args[2];
    
    if (!versionId) {
      console.error('Error: Version ID is required');
      process.exit(1);
    }
    
    const versionDetails = await getServiceVersionDetails(serviceId, versionId);
    
    if (versionDetails) {
      console.log('\nService Version Details:');
      console.log('ID:', versionDetails.id);
      console.log('Version:', versionDetails.version);
      console.log('Created At:', versionDetails.created_at);
      console.log('Active:', versionDetails.active ? 'Yes' : 'No');
      console.log('Functions:', versionDetails.functions.length);
    }
  } else if (command === 'activate') {
    // Activate service version
    const versionId = args[2];
    
    if (!versionId) {
      console.error('Error: Version ID is required');
      process.exit(1);
    }
    
    await activateServiceVersion(serviceId, versionId);
  } else if (command === 'delete') {
    // Delete service version
    const versionId = args[2];
    
    if (!versionId) {
      console.error('Error: Version ID is required');
      process.exit(1);
    }
    
    // Confirm deletion
    console.log(`Are you sure you want to delete service version with ID: ${versionId}? (y/n)`);
    process.stdin.once('data', async (data) => {
      const answer = data.toString().trim().toLowerCase();
      if (answer === 'y' || answer === 'yes') {
        await deleteServiceVersion(serviceId, versionId);
      } else {
        console.log('Service version deletion cancelled');
      }
      process.exit(0);
    });
  } else if (command === 'demo') {
    // Run a demo of service versioning
    console.log('=== Neo N3 FaaS Service Versioning Demo ===\n');
    
    // Step 1: List all versions
    console.log('Step 1: Listing all service versions...\n');
    const versions = await listServiceVersions(serviceId);
    console.log('\n');
    
    // Step 2: Create a new patch version
    console.log('Step 2: Creating a new patch version...\n');
    const patchVersion = await createPatchVersion(serviceId);
    console.log('\n');
    
    // Step 3: Create a new minor version
    console.log('Step 3: Creating a new minor version...\n');
    const minorVersion = await createMinorVersion(serviceId);
    console.log('\n');
    
    // Step 4: List all versions again
    console.log('Step 4: Listing all service versions again...\n');
    await listServiceVersions(serviceId);
    console.log('\n');
    
    // Step 5: Get details of the minor version
    if (minorVersion) {
      console.log(`Step 5: Getting details of the minor version (ID: ${minorVersion.id})...\n`);
      await getServiceVersionDetails(serviceId, minorVersion.id);
      console.log('\n');
      
      // Step 6: Activate the minor version
      console.log(`Step 6: Activating the minor version (ID: ${minorVersion.id})...\n`);
      await activateServiceVersion(serviceId, minorVersion.id);
      console.log('\n');
    }
    
    console.log('=== Demo completed successfully ===');
  } else if (command === 'help') {
    // Display help
    console.log('Neo N3 Service Versioning Example');
    console.log('\nUsage:');
    console.log('  node version.js <command> <serviceId> [options]');
    console.log('\nCommands:');
    console.log('  list <serviceId>                    List all service versions');
    console.log('  create <serviceId> <type>           Create a new service version (type: major, minor, patch)');
    console.log('  get <serviceId> <versionId>         Get service version details');
    console.log('  activate <serviceId> <versionId>    Activate a service version');
    console.log('  delete <serviceId> <versionId>      Delete a service version');
    console.log('  demo <serviceId>                    Run a demo of service versioning');
    console.log('  help                                Display this help message');
    console.log('\nExamples:');
    console.log('  node version.js list service123');
    console.log('  node version.js create service123 minor');
    console.log('  node version.js activate service123 version456');
  } else {
    console.error('Unknown command:', command);
    console.error('Run "node version.js help" for usage information');
    process.exit(1);
  }
}

// Execute the main function
main().catch(error => {
  console.error('Unhandled error:', error);
  process.exit(1);
});
