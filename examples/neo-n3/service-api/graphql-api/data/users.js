/**
 * Mock Users Data for Neo N3 FaaS Platform GraphQL API
 * 
 * This file provides mock data and operations for users in the Neo N3 FaaS platform.
 */

// Mock users data
const users = [
  {
    id: 'user-1',
    username: 'alice',
    email: 'alice@example.com',
    password: 'password123', // In a real implementation, this would be hashed
    roles: ['user', 'admin'],
    createdAt: '2025-02-01T10:00:00Z',
    updatedAt: '2025-02-01T10:00:00Z'
  },
  {
    id: 'user-2',
    username: 'bob',
    email: 'bob@example.com',
    password: 'password456', // In a real implementation, this would be hashed
    roles: ['user'],
    createdAt: '2025-02-02T10:00:00Z',
    updatedAt: '2025-02-02T10:00:00Z'
  },
  {
    id: 'user-3',
    username: 'charlie',
    email: 'charlie@example.com',
    password: 'password789', // In a real implementation, this would be hashed
    roles: ['user'],
    createdAt: '2025-02-03T10:00:00Z',
    updatedAt: '2025-02-03T10:00:00Z'
  }
];

// Get all users
function getUsers() {
  return users.map(user => {
    // Remove password from user object
    const { password, ...userWithoutPassword } = user;
    return userWithoutPassword;
  });
}

// Get user by ID
function getUserById(id) {
  const user = users.find(user => user.id === id);
  
  if (!user) {
    throw new Error(`User not found: ${id}`);
  }
  
  // Remove password from user object
  const { password, ...userWithoutPassword } = user;
  return userWithoutPassword;
}

// Get user by username
function getUserByUsername(username) {
  const user = users.find(user => user.username === username);
  
  if (!user) {
    throw new Error(`User not found: ${username}`);
  }
  
  return user;
}

// Get current user
function getCurrentUser(id) {
  return getUserById(id);
}

// Create a new user
function createUser(input) {
  // Check if username already exists
  const existingUser = users.find(user => user.username === input.username);
  
  if (existingUser) {
    throw new Error(`Username already exists: ${input.username}`);
  }
  
  // Generate a new ID
  const id = `user-${users.length + 1}`;
  
  // Create timestamp
  const timestamp = new Date().toISOString();
  
  // Create new user
  const newUser = {
    id,
    username: input.username,
    email: input.email,
    password: input.password, // In a real implementation, this would be hashed
    roles: input.roles || ['user'],
    createdAt: timestamp,
    updatedAt: timestamp
  };
  
  // Add to users
  users.push(newUser);
  
  // Remove password from user object
  const { password, ...userWithoutPassword } = newUser;
  return userWithoutPassword;
}

// Update an existing user
function updateUser(id, input) {
  // Find user
  const userIndex = users.findIndex(user => user.id === id);
  
  if (userIndex === -1) {
    throw new Error(`User not found: ${id}`);
  }
  
  // Get user
  const user = users[userIndex];
  
  // Update user
  const updatedUser = {
    ...user,
    username: input.username || user.username,
    email: input.email || user.email,
    password: input.password || user.password, // In a real implementation, this would be hashed
    roles: input.roles || user.roles,
    updatedAt: new Date().toISOString()
  };
  
  // Update in users
  users[userIndex] = updatedUser;
  
  // Remove password from user object
  const { password, ...userWithoutPassword } = updatedUser;
  return userWithoutPassword;
}

// Delete a user
function deleteUser(id) {
  // Find user
  const userIndex = users.findIndex(user => user.id === id);
  
  if (userIndex === -1) {
    throw new Error(`User not found: ${id}`);
  }
  
  // Remove from users
  users.splice(userIndex, 1);
  
  return true;
}

module.exports = {
  getUsers,
  getUserById,
  getUserByUsername,
  getCurrentUser,
  createUser,
  updateUser,
  deleteUser
};
