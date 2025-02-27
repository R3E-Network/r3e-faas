/**
 * Version management script for Function Management Example
 * 
 * This script demonstrates how to create and manage function versions in the Neo N3 FaaS platform.
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

// Function to get function details
async function getFunctionDetails(functionId) {
  try {
    console.log('Getting details for function with ID:', functionId);
    
    // Get the function details
    const response = await axios.get(`${API_URL}/functions/${functionId}`, {
      headers: {
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 200) {
      console.log('\nFunction details retrieved successfully!');
      console.log('Function Name:', response.data.name);
      console.log('Function Description:', response.data.description);
      console.log('Function Version:', response.data.metadata.version);
      
      return response.data;
    } else {
      console.error('Error getting function details:', response.data);
      return null;
    }
  } catch (error) {
    console.error('Error getting function details:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return null;
  }
}

// Function to create a function version
async function createFunctionVersion(functionId, payload) {
  try {
    console.log('Creating new version for function with ID:', functionId);
    console.log('Version payload:', JSON.stringify(payload, null, 2));
    
    // Create the function version
    const response = await axios.post(`${API_URL}/functions/${functionId}/versions`, payload, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 201) {
      console.log('\nFunction version created successfully!');
      console.log('Function ID:', functionId);
      console.log('Function Name:', response.data.name);
      console.log('Function Version:', response.data.metadata.version);
      
      return response.data;
    } else {
      console.error('Error creating function version:', response.data);
      return null;
    }
  } catch (error) {
    console.error('Error creating function version:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return null;
  }
}

// Function to list function versions
async function listFunctionVersions(functionId) {
  try {
    console.log('Listing versions for function with ID:', functionId);
    
    // List the function versions
    const response = await axios.get(`${API_URL}/functions/${functionId}/versions`, {
      headers: {
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 200) {
      console.log('\nFunction versions retrieved successfully!');
      console.log('Total versions:', response.data.length);
      
      if (response.data.length > 0) {
        console.log('\nVersions:');
        response.data.forEach((version, index) => {
          console.log(`\n${index + 1}. Version: ${version.metadata.version}`);
          console.log(`   Created: ${version.created_at}`);
          console.log(`   Active: ${version.active ? 'Yes' : 'No'}`);
        });
      } else {
        console.log('\nNo versions found.');
      }
      
      return response.data;
    } else {
      console.error('Error listing function versions:', response.data);
      return [];
    }
  } catch (error) {
    console.error('Error listing function versions:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return [];
  }
}

// Function to get function version details
async function getFunctionVersionDetails(functionId, versionId) {
  try {
    console.log('Getting details for function version with ID:', versionId);
    
    // Get the function version details
    const response = await axios.get(`${API_URL}/functions/${functionId}/versions/${versionId}`, {
      headers: {
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 200) {
      console.log('\nFunction version details retrieved successfully!');
      console.log('Function Name:', response.data.name);
      console.log('Function Description:', response.data.description);
      console.log('Function Version:', response.data.metadata.version);
      console.log('Created At:', response.data.created_at);
      console.log('Active:', response.data.active ? 'Yes' : 'No');
      
      return response.data;
    } else {
      console.error('Error getting function version details:', response.data);
      return null;
    }
  } catch (error) {
    console.error('Error getting function version details:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return null;
  }
}

// Function to activate a function version
async function activateFunctionVersion(functionId, versionId) {
  try {
    console.log('Activating version with ID:', versionId, 'for function with ID:', functionId);
    
    // Activate the function version
    const response = await axios.post(`${API_URL}/functions/${functionId}/versions/${versionId}/activate`, {}, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 200) {
      console.log('\nFunction version activated successfully!');
      console.log('Function ID:', functionId);
      console.log('Version ID:', versionId);
      
      return true;
    } else {
      console.error('Error activating function version:', response.data);
      return false;
    }
  } catch (error) {
    console.error('Error activating function version:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return false;
  }
}

// Function to delete a function version
async function deleteFunctionVersion(functionId, versionId) {
  try {
    console.log('Deleting version with ID:', versionId, 'for function with ID:', functionId);
    
    // Delete the function version
    const response = await axios.delete(`${API_URL}/functions/${functionId}/versions/${versionId}`, {
      headers: {
        'Authorization': `Bearer ${API_KEY}`
      }
    });
    
    // Check the response
    if (response.status === 204) {
      console.log('\nFunction version deleted successfully!');
      return true;
    } else {
      console.error('Error deleting function version:', response.data);
      return false;
    }
  } catch (error) {
    console.error('Error deleting function version:');
    if (error.response) {
      console.error('Status:', error.response.status);
      console.error('Data:', error.response.data);
    } else {
      console.error(error.message);
    }
    return false;
  }
}

// Function to create a new version with incremented version number
async function createIncrementedVersion(functionId, type = 'patch') {
  try {
    // Get the function details
    const functionDetails = await getFunctionDetails(functionId);
    
    if (!functionDetails) {
      console.error('Error: Function not found');
      process.exit(1);
    }
    
    // Parse the current version
    const currentVersion = functionDetails.metadata.version;
    const [major, minor, patch] = currentVersion.split('.').map(Number);
    
    // Calculate the new version based on the type
    let newVersion;
    if (type === 'major') {
      newVersion = `${major + 1}.0.0`;
    } else if (type === 'minor') {
      newVersion = `${major}.${minor + 1}.0`;
    } else {
      // Default to patch
      newVersion = `${major}.${minor}.${patch + 1}`;
    }
    
    console.log('Current version:', currentVersion);
    console.log('New version:', newVersion);
    
    // Read the configuration file
    const config = readConfig();
    
    // Read the function code
    const code = readFunctionCode();
    
    // Prepare the version payload
    const payload = {
      name: functionDetails.name,
      description: functionDetails.description,
      code: code,
      version: newVersion,
      metadata: {
        ...functionDetails.metadata,
        version: newVersion
      }
    };
    
    // Create the function version
    return await createFunctionVersion(functionId, payload);
  } catch (error) {
    console.error('Error creating incremented version:', error.message);
    return null;
  }
}

// Function to compare two versions
async function compareVersions(functionId, version1Id, version2Id) {
  try {
    // Get the details of both versions
    const version1 = await getFunctionVersionDetails(functionId, version1Id);
    const version2 = await getFunctionVersionDetails(functionId, version2Id);
    
    if (!version1 || !version2) {
      console.error('Error: One or both versions not found');
      process.exit(1);
    }
    
    console.log('\nComparing versions:');
    console.log(`Version 1: ${version1.metadata.version} (ID: ${version1Id})`);
    console.log(`Version 2: ${version2.metadata.version} (ID: ${version2Id})`);
    
    // Compare basic metadata
    console.log('\nMetadata Comparison:');
    console.log('Name:', version1.name === version2.name ? 'Same' : 'Different');
    console.log('Description:', version1.description === version2.description ? 'Same' : 'Different');
    console.log('Runtime:', version1.metadata.runtime === version2.metadata.runtime ? 'Same' : 'Different');
    console.log('Handler:', version1.metadata.handler === version2.metadata.handler ? 'Same' : 'Different');
    
    // Compare trigger configuration
    console.log('\nTrigger Configuration:');
    const triggerType1 = version1.metadata.trigger_type;
    const triggerType2 = version2.metadata.trigger_type;
    console.log('Trigger Type:', triggerType1 === triggerType2 ? 'Same' : 'Different');
    
    // Compare code (simplified)
    console.log('\nCode Comparison:');
    if (version1.code === version2.code) {
      console.log('Code: Same');
    } else {
      console.log('Code: Different');
      
      // Calculate a simple diff (this is a very basic implementation)
      const lines1 = version1.code.split('\n');
      const lines2 = version2.code.split('\n');
      
      console.log('\nSimple Diff:');
      const maxLines = Math.min(10, Math.max(lines1.length, lines2.length));
      
      for (let i = 0; i < maxLines; i++) {
        if (i < lines1.length && i < lines2.length) {
          if (lines1[i] !== lines2[i]) {
            console.log(`Line ${i + 1}:`);
            console.log(`< ${lines1[i]}`);
            console.log(`> ${lines2[i]}`);
          }
        } else if (i < lines1.length) {
          console.log(`Line ${i + 1}:`);
          console.log(`< ${lines1[i]}`);
          console.log(`> (none)`);
        } else if (i < lines2.length) {
          console.log(`Line ${i + 1}:`);
          console.log(`< (none)`);
          console.log(`> ${lines2[i]}`);
        }
      }
      
      if (Math.max(lines1.length, lines2.length) > maxLines) {
        console.log('... (diff truncated)');
      }
    }
    
    return { version1, version2 };
  } catch (error) {
    console.error('Error comparing versions:', error.message);
    return null;
  }
}

// Main function
async function main() {
  // Parse command line arguments
  const args = process.argv.slice(2);
  const command = args[0] || 'help';
  
  if (command === 'list') {
    // List function versions
    const functionId = args[1];
    
    if (!functionId) {
      console.error('Error: Function ID is required');
      console.error('Usage: node version.js list <functionId>');
      process.exit(1);
    }
    
    await listFunctionVersions(functionId);
  } else if (command === 'get') {
    // Get function version details
    const functionId = args[1];
    const versionId = args[2];
    
    if (!functionId || !versionId) {
      console.error('Error: Function ID and Version ID are required');
      console.error('Usage: node version.js get <functionId> <versionId>');
      process.exit(1);
    }
    
    await getFunctionVersionDetails(functionId, versionId);
  } else if (command === 'create') {
    // Create a function version
    const functionId = args[1];
    const versionType = args[2] || 'patch';
    
    if (!functionId) {
      console.error('Error: Function ID is required');
      console.error('Usage: node version.js create <functionId> [major|minor|patch]');
      process.exit(1);
    }
    
    if (!['major', 'minor', 'patch'].includes(versionType)) {
      console.error('Error: Version type must be one of: major, minor, patch');
      process.exit(1);
    }
    
    await createIncrementedVersion(functionId, versionType);
  } else if (command === 'activate') {
    // Activate a function version
    const functionId = args[1];
    const versionId = args[2];
    
    if (!functionId || !versionId) {
      console.error('Error: Function ID and Version ID are required');
      console.error('Usage: node version.js activate <functionId> <versionId>');
      process.exit(1);
    }
    
    const activated = await activateFunctionVersion(functionId, versionId);
    
    if (activated) {
      console.log('Function version activated successfully');
    } else {
      console.error('Failed to activate function version');
    }
  } else if (command === 'delete') {
    // Delete a function version
    const functionId = args[1];
    const versionId = args[2];
    
    if (!functionId || !versionId) {
      console.error('Error: Function ID and Version ID are required');
      console.error('Usage: node version.js delete <functionId> <versionId>');
      process.exit(1);
    }
    
    // Ask for confirmation
    const readline = require('readline').createInterface({
      input: process.stdin,
      output: process.stdout
    });
    
    readline.question(`Are you sure you want to delete version with ID ${versionId} for function with ID ${functionId}? (y/n) `, async (answer) => {
      if (answer.toLowerCase() === 'y') {
        const deleted = await deleteFunctionVersion(functionId, versionId);
        
        if (deleted) {
          console.log('Function version deleted successfully');
        } else {
          console.error('Failed to delete function version');
        }
      } else {
        console.log('Function version deletion cancelled');
      }
      
      readline.close();
    });
  } else if (command === 'compare') {
    // Compare two function versions
    const functionId = args[1];
    const version1Id = args[2];
    const version2Id = args[3];
    
    if (!functionId || !version1Id || !version2Id) {
      console.error('Error: Function ID and two Version IDs are required');
      console.error('Usage: node version.js compare <functionId> <version1Id> <version2Id>');
      process.exit(1);
    }
    
    await compareVersions(functionId, version1Id, version2Id);
  } else if (command === 'help') {
    // Display help
    console.log('Neo N3 Function Version Management Example');
    console.log('\nUsage:');
    console.log('  node version.js <command> [options]');
    console.log('\nCommands:');
    console.log('  list <functionId>                      List all versions of a function');
    console.log('  get <functionId> <versionId>           Get details of a function version');
    console.log('  create <functionId> [major|minor|patch] Create a new version of a function');
    console.log('  activate <functionId> <versionId>      Activate a function version');
    console.log('  delete <functionId> <versionId>        Delete a function version');
    console.log('  compare <functionId> <v1Id> <v2Id>     Compare two function versions');
    console.log('  help                                   Display this help message');
  } else {
    console.error('Unknown command:', command);
    console.error('Run "node version.js help" for usage information');
    process.exit(1);
  }
}

// Execute the main function
if (command !== 'delete') {
  main().catch(error => {
    console.error('Unhandled error:', error);
    process.exit(1);
  });
}
