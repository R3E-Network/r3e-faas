# TEE Service Example for Neo N3 FaaS Platform

This example demonstrates how to create, deploy, and use Trusted Execution Environment (TEE) services on the Neo N3 FaaS platform.

## Prerequisites

Before you begin, ensure you have the following:

- Neo N3 FaaS CLI installed (`npm install -g r3e-faas-cli`)
- A Neo N3 FaaS account
- Basic knowledge of JavaScript and Neo N3 blockchain
- Understanding of TEE concepts (optional but recommended)

## Introduction to TEE Services

Trusted Execution Environment (TEE) services provide a secure and isolated execution environment for sensitive code and data. TEEs ensure that the code and data are protected from unauthorized access and tampering, even from privileged users or the operating system.

The Neo N3 FaaS platform supports multiple TEE technologies:

- Intel SGX (Software Guard Extensions)
- AMD SEV (Secure Encrypted Virtualization)
- ARM TrustZone

TEE services are ideal for applications that require:

- Secure key management
- Confidential computing
- Privacy-preserving data processing
- Secure multi-party computation
- Verifiable computation

## Project Setup

1. Create a new project directory:

```bash
mkdir neo-tee-example
cd neo-tee-example
```

2. Initialize a new Neo N3 FaaS project:

```bash
r3e-faas-cli init
```

This command creates the following files:

```
neo-tee-example/
├── functions/
│   └── hello.js
├── r3e.yaml
└── package.json
```

## Using TEE Services

### Secure Key Management

Create a new file `functions/secure-key-management.js` with the following code:

```javascript
/**
 * A function that demonstrates secure key management using TEE
 * 
 * @param {Object} event - The event object
 * @param {Object} context - The context object
 * @returns {Object} - The response object
 */
export default async function(event, context) {
  // Log the event for debugging
  console.log('Event received:', event);
  
  // Get the operation from the event parameters or use a default
  const operation = event.params?.operation || 'generate';
  
  // Get the key ID from the event parameters (if provided)
  const keyId = event.params?.keyId;
  
  // Get the key type from the event parameters or use a default
  const keyType = event.params?.keyType || 'EC_SECP256R1';
  
  // Get the TEE service
  const tee = context.tee;
  
  if (!tee) {
    return {
      error: 'TEE service not available'
    };
  }
  
  // Execute the operation in the TEE
  try {
    // Execute the operation in the TEE
    const result = await tee.execute(async (secureContext) => {
      // Get the key management API
      const keyManager = secureContext.keyManager;
      
      // Perform the requested operation
      switch (operation) {
        case 'generate':
          // Generate a new key pair
          const newKeyPair = await keyManager.generateKeyPair(keyType);
          
          // Return only the public key and key ID
          return {
            keyId: newKeyPair.keyId,
            publicKey: newKeyPair.publicKey,
            message: `Generated new ${keyType} key pair`
          };
        
        case 'get':
          // Validate key ID
          if (!keyId) {
            throw new Error('Key ID is required for get operation');
          }
          
          // Get the public key for the specified key ID
          const publicKey = await keyManager.getPublicKey(keyId);
          
          return {
            keyId,
            publicKey,
            message: `Retrieved public key for key ID ${keyId}`
          };
        
        case 'sign':
          // Validate key ID
          if (!keyId) {
            throw new Error('Key ID is required for sign operation');
          }
          
          // Get the data to sign from the event parameters or use a default
          const data = event.params?.data || 'Hello, Neo N3 FaaS!';
          
          // Sign the data with the specified key
          const signature = await keyManager.sign(keyId, data);
          
          return {
            keyId,
            data,
            signature,
            message: `Signed data with key ID ${keyId}`
          };
        
        case 'verify':
          // Validate key ID
          if (!keyId) {
            throw new Error('Key ID is required for verify operation');
          }
          
          // Get the data and signature from the event parameters
          const verifyData = event.params?.data;
          const verifySignature = event.params?.signature;
          
          if (!verifyData || !verifySignature) {
            throw new Error('Data and signature are required for verify operation');
          }
          
          // Verify the signature
          const isValid = await keyManager.verify(keyId, verifyData, verifySignature);
          
          return {
            keyId,
            data: verifyData,
            signature: verifySignature,
            isValid,
            message: isValid ? 'Signature is valid' : 'Signature is invalid'
          };
        
        default:
          throw new Error(`Unsupported operation: ${operation}`);
      }
    });
    
    // Return the result
    return {
      operation,
      ...result,
      timestamp: new Date().toISOString()
    };
  } catch (error) {
    console.error(`Error performing ${operation} operation:`, error);
    return {
      error: `Failed to perform ${operation} operation: ${error.message}`
    };
  }
}
```

This function:
1. Receives an event object with parameters for the key management operation
2. Executes the operation in the TEE using the `tee.execute()` method
3. Performs key generation, retrieval, signing, or verification based on the operation parameter
4. Returns the result of the operation

### Confidential Computing

Create a new file `functions/confidential-computing.js` with the following code:

```javascript
/**
 * A function that demonstrates confidential computing using TEE
 * 
 * @param {Object} event - The event object
 * @param {Object} context - The context object
 * @returns {Object} - The response object
 */
export default async function(event, context) {
  // Log the event for debugging
  console.log('Event received:', event);
  
  // Get the operation from the event parameters or use a default
  const operation = event.params?.operation || 'process';
  
  // Get the data from the event parameters or use a default
  const data = event.params?.data || { value: 'sensitive data' };
  
  // Get the TEE service
  const tee = context.tee;
  
  if (!tee) {
    return {
      error: 'TEE service not available'
    };
  }
  
  // Execute the operation in the TEE
  try {
    // Execute the operation in the TEE
    const result = await tee.execute(async (secureContext) => {
      // Perform the requested operation
      switch (operation) {
        case 'process':
          // Process the data in the TEE
          console.log('Processing data in TEE:', data);
          
          // Simulate data processing
          const processedData = {
            ...data,
            processed: true,
            processingTime: new Date().toISOString()
          };
          
          return {
            result: processedData,
            message: 'Data processed successfully in TEE'
          };
        
        case 'encrypt':
          // Encrypt the data in the TEE
          console.log('Encrypting data in TEE:', data);
          
          // Get the encryption API
          const encryption = secureContext.encryption;
          
          // Generate a new encryption key or use an existing one
          const encryptionKeyId = event.params?.keyId || await encryption.generateKey('AES_256');
          
          // Encrypt the data
          const encryptedData = await encryption.encrypt(encryptionKeyId, JSON.stringify(data));
          
          return {
            keyId: encryptionKeyId,
            encryptedData,
            message: 'Data encrypted successfully in TEE'
          };
        
        case 'decrypt':
          // Decrypt the data in the TEE
          console.log('Decrypting data in TEE');
          
          // Get the encryption API
          const decryption = secureContext.encryption;
          
          // Get the key ID and encrypted data from the event parameters
          const decryptionKeyId = event.params?.keyId;
          const encryptedDataToDecrypt = event.params?.encryptedData;
          
          if (!decryptionKeyId || !encryptedDataToDecrypt) {
            throw new Error('Key ID and encrypted data are required for decrypt operation');
          }
          
          // Decrypt the data
          const decryptedDataString = await decryption.decrypt(decryptionKeyId, encryptedDataToDecrypt);
          const decryptedData = JSON.parse(decryptedDataString);
          
          return {
            keyId: decryptionKeyId,
            decryptedData,
            message: 'Data decrypted successfully in TEE'
          };
        
        default:
          throw new Error(`Unsupported operation: ${operation}`);
      }
    });
    
    // Return the result
    return {
      operation,
      ...result,
      timestamp: new Date().toISOString()
    };
  } catch (error) {
    console.error(`Error performing ${operation} operation:`, error);
    return {
      error: `Failed to perform ${operation} operation: ${error.message}`
    };
  }
}
```

This function:
1. Receives an event object with parameters for the confidential computing operation
2. Executes the operation in the TEE using the `tee.execute()` method
3. Performs data processing, encryption, or decryption based on the operation parameter
4. Returns the result of the operation

### Secure Multi-Party Computation

Create a new file `functions/secure-mpc.js` with the following code:

```javascript
/**
 * A function that demonstrates secure multi-party computation using TEE
 * 
 * @param {Object} event - The event object
 * @param {Object} context - The context object
 * @returns {Object} - The response object
 */
export default async function(event, context) {
  // Log the event for debugging
  console.log('Event received:', event);
  
  // Get the operation from the event parameters or use a default
  const operation = event.params?.operation || 'compute';
  
  // Get the TEE service
  const tee = context.tee;
  
  if (!tee) {
    return {
      error: 'TEE service not available'
    };
  }
  
  // Execute the operation in the TEE
  try {
    // Execute the operation in the TEE
    const result = await tee.execute(async (secureContext) => {
      // Get the MPC API
      const mpc = secureContext.mpc;
      
      // Perform the requested operation
      switch (operation) {
        case 'compute':
          // Get the computation ID from the event parameters or create a new one
          const computationId = event.params?.computationId || await mpc.createComputation('SECURE_SUM');
          
          // Get the participant ID and input from the event parameters
          const participantId = event.params?.participantId;
          const input = event.params?.input;
          
          if (!participantId || input === undefined) {
            throw new Error('Participant ID and input are required for compute operation');
          }
          
          // Add the participant's input to the computation
          await mpc.addInput(computationId, participantId, input);
          
          // Check if all participants have provided their inputs
          const isReady = await mpc.isComputationReady(computationId);
          
          if (isReady) {
            // Compute the result
            const computationResult = await mpc.computeResult(computationId);
            
            return {
              computationId,
              result: computationResult,
              status: 'completed',
              message: 'Computation completed successfully'
            };
          } else {
            // Return the computation status
            const status = await mpc.getComputationStatus(computationId);
            
            return {
              computationId,
              status: 'waiting',
              participantsSubmitted: status.participantsSubmitted,
              participantsRequired: status.participantsRequired,
              message: 'Waiting for all participants to provide inputs'
            };
          }
        
        case 'status':
          // Get the computation ID from the event parameters
          const statusComputationId = event.params?.computationId;
          
          if (!statusComputationId) {
            throw new Error('Computation ID is required for status operation');
          }
          
          // Get the computation status
          const computationStatus = await mpc.getComputationStatus(statusComputationId);
          
          return {
            computationId: statusComputationId,
            status: computationStatus.isReady ? 'ready' : 'waiting',
            participantsSubmitted: computationStatus.participantsSubmitted,
            participantsRequired: computationStatus.participantsRequired,
            message: computationStatus.isReady ? 'Computation is ready to be executed' : 'Waiting for all participants to provide inputs'
          };
        
        case 'result':
          // Get the computation ID from the event parameters
          const resultComputationId = event.params?.computationId;
          
          if (!resultComputationId) {
            throw new Error('Computation ID is required for result operation');
          }
          
          // Check if the computation is ready
          const resultIsReady = await mpc.isComputationReady(resultComputationId);
          
          if (!resultIsReady) {
            const resultStatus = await mpc.getComputationStatus(resultComputationId);
            
            return {
              computationId: resultComputationId,
              status: 'waiting',
              participantsSubmitted: resultStatus.participantsSubmitted,
              participantsRequired: resultStatus.participantsRequired,
              message: 'Waiting for all participants to provide inputs'
            };
          }
          
          // Compute the result
          const result = await mpc.computeResult(resultComputationId);
          
          return {
            computationId: resultComputationId,
            result,
            status: 'completed',
            message: 'Computation completed successfully'
          };
        
        default:
          throw new Error(`Unsupported operation: ${operation}`);
      }
    });
    
    // Return the result
    return {
      operation,
      ...result,
      timestamp: new Date().toISOString()
    };
  } catch (error) {
    console.error(`Error performing ${operation} operation:`, error);
    return {
      error: `Failed to perform ${operation} operation: ${error.message}`
    };
  }
}
```

This function:
1. Receives an event object with parameters for the secure multi-party computation operation
2. Executes the operation in the TEE using the `tee.execute()` method
3. Performs computation setup, status checking, or result retrieval based on the operation parameter
4. Returns the result of the operation

## Configuration

Open the `r3e.yaml` file and update it with the following configuration:

```yaml
project:
  name: neo-tee-example
  version: 0.1.0

functions:
  secure-key-management:
    handler: functions/secure-key-management.js
    runtime: javascript
    trigger:
      type: http
      path: /secure-key-management
      method: post
    environment:
      NODE_ENV: production
    tee:
      provider: sgx
      attestation: required
  
  confidential-computing:
    handler: functions/confidential-computing.js
    runtime: javascript
    trigger:
      type: http
      path: /confidential-computing
      method: post
    environment:
      NODE_ENV: production
    tee:
      provider: sgx
      attestation: required
  
  secure-mpc:
    handler: functions/secure-mpc.js
    runtime: javascript
    trigger:
      type: http
      path: /secure-mpc
      method: post
    environment:
      NODE_ENV: production
    tee:
      provider: sgx
      attestation: required
```

This configuration:
1. Sets the project name and version
2. Defines three functions: secure-key-management, confidential-computing, and secure-mpc
3. Specifies the handler file path for each function
4. Sets the runtime to JavaScript for each function
5. Configures HTTP triggers for each function
6. Sets the `NODE_ENV` environment variable to `production` for each function
7. Configures TEE settings for each function, including the provider (SGX) and attestation requirement

## Local Testing

Before deploying, test the functions locally:

### Test Secure Key Management Function

```bash
# Generate a new key pair
r3e-faas-cli invoke-local --function secure-key-management --params '{"operation": "generate", "keyType": "EC_SECP256R1"}'

# Get the public key for a key ID
r3e-faas-cli invoke-local --function secure-key-management --params '{"operation": "get", "keyId": "key-id-from-generate-operation"}'

# Sign data with a key
r3e-faas-cli invoke-local --function secure-key-management --params '{"operation": "sign", "keyId": "key-id-from-generate-operation", "data": "Hello, Neo N3 FaaS!"}'

# Verify a signature
r3e-faas-cli invoke-local --function secure-key-management --params '{"operation": "verify", "keyId": "key-id-from-generate-operation", "data": "Hello, Neo N3 FaaS!", "signature": "signature-from-sign-operation"}'
```

### Test Confidential Computing Function

```bash
# Process data in the TEE
r3e-faas-cli invoke-local --function confidential-computing --params '{"operation": "process", "data": {"value": "sensitive data", "userId": "user123"}}'

# Encrypt data in the TEE
r3e-faas-cli invoke-local --function confidential-computing --params '{"operation": "encrypt", "data": {"value": "sensitive data", "userId": "user123"}}'

# Decrypt data in the TEE
r3e-faas-cli invoke-local --function confidential-computing --params '{"operation": "decrypt", "keyId": "key-id-from-encrypt-operation", "encryptedData": "encrypted-data-from-encrypt-operation"}'
```

### Test Secure Multi-Party Computation Function

```bash
# Create a new computation and add the first participant's input
r3e-faas-cli invoke-local --function secure-mpc --params '{"operation": "compute", "participantId": "participant1", "input": 10}'

# Add the second participant's input to the computation
r3e-faas-cli invoke-local --function secure-mpc --params '{"operation": "compute", "computationId": "computation-id-from-previous-operation", "participantId": "participant2", "input": 20}'

# Add the third participant's input to the computation
r3e-faas-cli invoke-local --function secure-mpc --params '{"operation": "compute", "computationId": "computation-id-from-previous-operation", "participantId": "participant3", "input": 30}'

# Check the status of the computation
r3e-faas-cli invoke-local --function secure-mpc --params '{"operation": "status", "computationId": "computation-id-from-previous-operation"}'

# Get the result of the computation
r3e-faas-cli invoke-local --function secure-mpc --params '{"operation": "result", "computationId": "computation-id-from-previous-operation"}'
```

## Deployment

Deploy the functions to the Neo N3 FaaS platform:

```bash
r3e-faas-cli deploy
```

This command:
1. Packages the function code
2. Uploads it to the Neo N3 FaaS platform
3. Creates the necessary resources
4. Configures the HTTP endpoints and TEE settings

After successful deployment, you should see output similar to:

```
Deploying project: neo-tee-example
Deploying function: secure-key-management... Done!
Deploying function: confidential-computing... Done!
Deploying function: secure-mpc... Done!
Function URLs:
- secure-key-management: https://faas.example.com/functions/secure-key-management
- confidential-computing: https://faas.example.com/functions/confidential-computing
- secure-mpc: https://faas.example.com/functions/secure-mpc
```

## Invoking the Functions

### Using curl

```bash
# Generate a new key pair
curl -X POST https://faas.example.com/functions/secure-key-management \
  -H "Content-Type: application/json" \
  -d '{"operation": "generate", "keyType": "EC_SECP256R1"}'

# Process data in the TEE
curl -X POST https://faas.example.com/functions/confidential-computing \
  -H "Content-Type: application/json" \
  -d '{"operation": "process", "data": {"value": "sensitive data", "userId": "user123"}}'

# Create a new computation and add the first participant's input
curl -X POST https://faas.example.com/functions/secure-mpc \
  -H "Content-Type: application/json" \
  -d '{"operation": "compute", "participantId": "participant1", "input": 10}'
```

### Using the CLI

```bash
# Generate a new key pair
r3e-faas-cli invoke --function secure-key-management --params '{"operation": "generate", "keyType": "EC_SECP256R1"}'

# Process data in the TEE
r3e-faas-cli invoke --function confidential-computing --params '{"operation": "process", "data": {"value": "sensitive data", "userId": "user123"}}'

# Create a new computation and add the first participant's input
r3e-faas-cli invoke --function secure-mpc --params '{"operation": "compute", "participantId": "participant1", "input": 10}'
```

## Advanced TEE Features

### Attestation Verification

The Neo N3 FaaS platform supports attestation verification for TEE services. Attestation is the process of verifying that the TEE is genuine and has not been tampered with.

To verify the attestation of a TEE service, you can use the `tee.verifyAttestation()` method:

```javascript
// Verify the attestation of a TEE service
const attestationResult = await context.tee.verifyAttestation();

console.log('Attestation result:', attestationResult);
```

### Secure Storage

TEE services also provide secure storage for sensitive data. The secure storage is encrypted and can only be accessed from within the TEE.

To use secure storage, you can use the `secureContext.storage` API:

```javascript
// Store data in secure storage
await secureContext.storage.set('key', 'value');

// Retrieve data from secure storage
const value = await secureContext.storage.get('key');

console.log('Value from secure storage:', value);
```

### Secure Random Number Generation

TEE services provide secure random number generation that is resistant to side-channel attacks.

To generate secure random numbers, you can use the `secureContext.random` API:

```javascript
// Generate a secure random number
const randomNumber = await secureContext.random.getRandomNumber(1, 100);

console.log('Secure random number:', randomNumber);

// Generate secure random bytes
const randomBytes = await secureContext.random.getRandomBytes(32);

console.log('Secure random bytes:', randomBytes);
```

## Integration with Neo N3 Blockchain

TEE services can be integrated with the Neo N3 blockchain to provide secure and verifiable computation for smart contracts.

### Verifiable Computation

Create a new file `functions/verifiable-computation.js` with the following code:

```javascript
/**
 * A function that demonstrates verifiable computation using TEE and Neo N3 blockchain
 * 
 * @param {Object} event - The event object
 * @param {Object} context - The context object
 * @returns {Object} - The response object
 */
export default async function(event, context) {
  // Log the event for debugging
  console.log('Event received:', event);
  
  // Get the operation from the event parameters or use a default
  const operation = event.params?.operation || 'compute';
  
  // Get the TEE service
  const tee = context.tee;
  
  if (!tee) {
    return {
      error: 'TEE service not available'
    };
  }
  
  // Get the Neo N3 service
  const neo = context.neo;
  
  if (!neo) {
    return {
      error: 'Neo N3 service not available'
    };
  }
  
  // Execute the operation in the TEE
  try {
    // Execute the operation in the TEE
    const result = await tee.execute(async (secureContext) => {
      // Perform the requested operation
      switch (operation) {
        case 'compute':
          // Get the computation parameters from the event
          const input = event.params?.input;
          const contractHash = event.params?.contractHash;
          const method = event.params?.method;
          
          if (!input || !contractHash || !method) {
            throw new Error('Input, contract hash, and method are required for compute operation');
          }
          
          // Perform the computation in the TEE
          console.log(`Computing ${method} in TEE with input:`, input);
          
          // Get the Neo N3 API in the TEE
          const neoInTee = secureContext.neo;
          
          // Get the contract instance
          const contract = await neoInTee.getContract(contractHash);
          
          // Call the contract method
          const computationResult = await contract.call(method, [input]);
          
          // Generate a proof of computation
          const proof = await secureContext.attestation.generateProof({
            input,
            contractHash,
            method,
            result: computationResult
          });
          
          return {
            input,
            contractHash,
            method,
            result: computationResult,
            proof,
            message: 'Computation completed successfully with proof'
          };
        
        case 'verify':
          // Get the verification parameters from the event
          const verifyInput = event.params?.input;
          const verifyResult = event.params?.result;
          const verifyProof = event.params?.proof;
          const verifyContractHash = event.params?.contractHash;
          const verifyMethod = event.params?.method;
          
          if (!verifyInput || !verifyResult || !verifyProof || !verifyContractHash || !verifyMethod) {
            throw new Error('Input, result, proof, contract hash, and method are required for verify operation');
          }
          
          // Verify the proof
          const isValid = await secureContext.attestation.verifyProof({
            input: verifyInput,
            result: verifyResult,
            proof: verifyProof,
            contractHash: verifyContractHash,
            method: verifyMethod
          });
          
          return {
            input: verifyInput,
            result: verifyResult,
            contractHash: verifyContractHash,
            method: verifyMethod,
            isValid,
            message: isValid ? 'Proof is valid' : 'Proof is invalid'
          };
        
        default:
          throw new Error(`Unsupported operation: ${operation}`);
      }
    });
    
    // Return the result
    return {
      operation,
      ...result,
      timestamp: new Date().toISOString()
    };
  } catch (error) {
    console.error(`Error performing ${operation} operation:`, error);
    return {
      error: `Failed to perform ${operation} operation: ${error.message}`
    };
  }
}
```

This function:
1. Receives an event object with parameters for the verifiable computation operation
2. Executes the operation in the TEE using the `tee.execute()` method
3. Performs computation or verification based on the operation parameter
4. Interacts with the Neo N3 blockchain from within the TEE
5. Generates or verifies proofs of computation
6. Returns the result of the operation

Update the `r3e.yaml` file to include the verifiable-computation function:

```yaml
functions:
  # ... existing functions ...
  
  verifiable-computation:
    handler: functions/verifiable-computation.js
    runtime: javascript
    trigger:
      type: http
      path: /verifiable-computation
      method: post
    environment:
      NODE_ENV: production
    tee:
      provider: sgx
      attestation: required
```

## Monitoring

View the function logs:

```bash
r3e-faas-cli logs --function secure-key-management
```

To follow the logs in real-time:

```bash
r3e-faas-cli logs --function secure-key-management --follow
```

## Cleaning Up

To remove the functions:

```bash
r3e-faas-cli remove
```

## Next Steps

Now that you've created, deployed, and used TEE services on the Neo N3 FaaS platform, you can:

1. Create more complex TEE services for specific security needs
2. Integrate TEE services with Neo N3 smart contracts
3. Implement secure multi-party computation protocols
4. Create privacy-preserving applications

For more examples, see:
- [Service API Example](./service-api.md)

For more information, see the [Documentation](../README.md).
