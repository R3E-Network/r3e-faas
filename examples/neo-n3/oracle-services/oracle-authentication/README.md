# Neo N3 Oracle Authentication Example

This example demonstrates how to implement secure authentication for oracle services in the Neo N3 FaaS platform. Oracle authentication ensures that only authorized users and smart contracts can access oracle data, preventing unauthorized usage and potential abuse.

## Overview

The oracle authentication example shows how to:

1. Implement multiple authentication methods for oracle services
2. Secure oracle endpoints with API keys, JWT tokens, and blockchain-based authentication
3. Integrate with Neo N3 smart contracts using secure authentication
4. Implement role-based access control for oracle services
5. Monitor and audit oracle service access

## Files

- `function.js`: The JavaScript function that implements the oracle authentication service
- `config.yaml`: Configuration file for the oracle authentication service
- `register.js`: Script to register the function with the FaaS platform
- `smart-contract/`: Directory containing a sample Neo N3 smart contract that uses authenticated oracle services
- `lib/`: Directory containing authentication libraries and utilities

## Prerequisites

- Neo N3 FaaS platform installed and running
- Access to a Neo N3 node (testnet or mainnet)
- Node.js installed for running the registration script
- Neo N3 smart contract development environment (optional, for testing the smart contract)

## Setup

1. Configure your Neo N3 node connection and authentication settings in `config.yaml`
2. Register the function using the registration script:

```bash
node register.js
```

3. (Optional) Deploy the sample smart contract to interact with the authenticated oracle service

## How It Works

The oracle authentication service implements several authentication methods to secure oracle data access. The service can be configured to use one or more authentication methods based on security requirements.

### Authentication Methods

The example demonstrates several authentication methods:

#### 1. API Key Authentication

The simplest form of authentication, where clients include an API key in their requests. The service validates the API key against a stored list of valid keys.

```javascript
// Example API key authentication
const apiKey = request.headers['x-api-key'];
if (!isValidApiKey(apiKey)) {
  return { error: 'Invalid API key' };
}
```

#### 2. JWT Token Authentication

A more secure method using JSON Web Tokens (JWT) for authentication. Clients obtain a JWT token through a separate authentication process and include it in their requests.

```javascript
// Example JWT authentication
const token = request.headers['authorization'].split(' ')[1];
const decoded = verifyJwtToken(token);
if (!decoded) {
  return { error: 'Invalid JWT token' };
}
```

#### 3. Blockchain-Based Authentication

The most secure method, using Neo N3 blockchain for authentication. Clients sign their requests with their private key, and the service verifies the signature using the client's public key.

```javascript
// Example blockchain-based authentication
const signature = request.headers['x-signature'];
const message = request.body.message;
const publicKey = request.headers['x-public-key'];
if (!verifySignature(message, signature, publicKey)) {
  return { error: 'Invalid signature' };
}
```

### Role-Based Access Control

The example also demonstrates role-based access control (RBAC) for oracle services. Different users or smart contracts can be assigned different roles, each with specific access permissions.

```javascript
// Example role-based access control
const userRole = getUserRole(userId);
if (!hasPermission(userRole, 'read_price_data')) {
  return { error: 'Insufficient permissions' };
}
```

### Access Monitoring and Auditing

The example includes access monitoring and auditing capabilities to track oracle service usage and detect potential security issues.

```javascript
// Example access logging
logAccess({
  userId: userId,
  resource: resource,
  action: action,
  timestamp: Date.now(),
  success: success
});
```

## Customization

You can customize this example by:

- Adding your own authentication methods
- Modifying the role-based access control system
- Implementing additional security measures
- Extending the auditing and monitoring capabilities

## Additional Resources

- [Neo N3 Oracle Services Documentation](../../docs/neo-n3/components/oracle-services.md)
- [Neo N3 FaaS Platform Documentation](../../docs/neo-n3/README.md)
- [JavaScript Function Development Guide](../../docs/neo-n3/guides/function-development.md)
- [Neo N3 Smart Contract Development Guide](https://docs.neo.org/docs/en-us/develop/write/basics.html)
