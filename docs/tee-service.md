# Trusted Execution Environment (TEE) Service

The R3E FaaS platform provides a comprehensive Trusted Execution Environment (TEE) Service that enables secure computation in isolated environments with hardware-based security guarantees.

## Overview

Trusted Execution Environments (TEEs) are secure areas within a processor that ensure the confidentiality and integrity of code and data loaded inside them. The R3E FaaS TEE Service leverages these secure environments to provide isolated execution for sensitive computations.

The R3E FaaS TEE Service supports multiple TEE providers:

- **Intel SGX**: Intel Software Guard Extensions
- **AWS Nitro Enclaves**: AWS's isolated compute environments

## Features

- **Secure Execution**: Run code in isolated environments with hardware-based security
- **Attestation**: Verify the authenticity and integrity of TEE environments
- **Key Management**: Securely manage cryptographic keys within TEEs
- **Secure Storage**: Store sensitive data with encryption
- **Confidential Computing**: Process sensitive data without exposing it to the host system
- **JavaScript API**: Easy-to-use JavaScript API for TEE operations

## JavaScript API

The TEE Service is accessible through the JavaScript API:

```javascript
// Access the TEE service
const tee = r3e.tee;
```

### Secure Execution

```javascript
// Execute a function in a TEE
const result = await tee.execute({
  code: `
    function secureComputation(input) {
      // Perform secure computation
      return input * 2;
    }
    
    secureComputation(params.value);
  `,
  params: { value: 42 },
  provider: tee.ProviderType.SGX
});

console.log(`Secure execution result: ${result}`); // 84
```

### Attestation

```javascript
// Generate an attestation report
const attestationReport = await tee.generateAttestationReport({
  provider: tee.ProviderType.SGX,
  data: "Custom data to include in the report"
});

// Verify an attestation report
const isValid = await tee.verifyAttestationReport(attestationReport);
console.log(`Attestation report is valid: ${isValid}`);
```

### Key Management

```javascript
// Generate a key pair within the TEE
const keyPairId = await tee.generateKeyPair({
  algorithm: "RSA",
  keySize: 2048,
  purpose: "signing"
});

// Sign data using a key in the TEE
const signature = await tee.sign({
  keyId: keyPairId,
  data: "Data to sign",
  algorithm: "RSA-SHA256"
});

// Verify a signature using a key in the TEE
const isValid = await tee.verify({
  keyId: keyPairId,
  data: "Data to sign",
  signature: signature,
  algorithm: "RSA-SHA256"
});
```

### Secure Storage

```javascript
// Store data securely in the TEE
await tee.secureStore({
  key: "sensitive-data",
  value: "Secret information",
  encryption: "AES-GCM"
});

// Retrieve data from secure storage
const data = await tee.secureRetrieve({
  key: "sensitive-data",
  encryption: "AES-GCM"
});

console.log(`Retrieved data: ${data}`); // Secret information
```

### Confidential Computing

```javascript
// Perform confidential computing on sensitive data
const result = await tee.confidentialCompute({
  code: `
    function processData(data) {
      // Process sensitive data
      let sum = 0;
      for (const value of data) {
        sum += value;
      }
      return sum / data.length;
    }
    
    processData(params.sensitiveData);
  `,
  params: {
    sensitiveData: [10, 20, 30, 40, 50]
  },
  provider: tee.ProviderType.NITRO
});

console.log(`Confidential computing result: ${result}`); // 30
```

## Provider-Specific Features

### Intel SGX

```javascript
// Get SGX-specific options
const sgxOptions = await tee.getProviderOptions(tee.ProviderType.SGX);

// Execute with SGX-specific options
const result = await tee.execute({
  code: `
    function secureComputation(input) {
      return input * 2;
    }
    
    secureComputation(params.value);
  `,
  params: { value: 42 },
  provider: tee.ProviderType.SGX,
  options: {
    enclaveMode: "hardware",
    heapSize: "128MB",
    stackSize: "16MB"
  }
});
```

### AWS Nitro Enclaves

```javascript
// Get Nitro-specific options
const nitroOptions = await tee.getProviderOptions(tee.ProviderType.NITRO);

// Execute with Nitro-specific options
const result = await tee.execute({
  code: `
    function secureComputation(input) {
      return input * 2;
    }
    
    secureComputation(params.value);
  `,
  params: { value: 42 },
  provider: tee.ProviderType.NITRO,
  options: {
    memoryMiB: 2048,
    cpuCount: 2,
    enclaveImageId: "custom-image-id"
  }
});
```

## Use Cases

### Secure Key Management

TEEs can be used for secure key management:

```javascript
// Generate a key pair within the TEE
const keyPairId = await tee.generateKeyPair({
  algorithm: "ECDSA",
  curve: "secp256k1",
  purpose: "signing"
});

// Export the public key
const publicKey = await tee.exportPublicKey(keyPairId);

// Sign a blockchain transaction within the TEE
const transaction = {
  from: "0x1234567890abcdef1234567890abcdef12345678",
  to: "0xabcdef1234567890abcdef1234567890abcdef12",
  value: "1000000000000000000", // 1 ETH
  gas: "21000",
  gasPrice: "20000000000" // 20 Gwei
};

const signature = await tee.sign({
  keyId: keyPairId,
  data: JSON.stringify(transaction),
  algorithm: "ECDSA-secp256k1"
});

// The private key never leaves the TEE
console.log(`Transaction signature: ${signature}`);
```

### Confidential Smart Contract Execution

TEEs can be used for confidential smart contract execution:

```javascript
// Define a confidential smart contract
const contractCode = `
  function confidentialContract(params) {
    // Confidential smart contract logic
    const { bidAmount, minBid, secretKey } = params;
    
    // Verify the bid meets the minimum
    if (bidAmount < minBid) {
      return { success: false, message: "Bid too low" };
    }
    
    // Process the bid securely
    const bidHash = crypto.createHash("sha256")
      .update(bidAmount.toString() + secretKey)
      .digest("hex");
    
    return {
      success: true,
      bidHash: bidHash
    };
  }
  
  confidentialContract(params);
`;

// Execute the confidential contract in a TEE
const result = await tee.execute({
  code: contractCode,
  params: {
    bidAmount: 1000,
    minBid: 500,
    secretKey: "very-secret-key-that-never-leaves-tee"
  },
  provider: tee.ProviderType.SGX
});

console.log(`Contract execution result: ${JSON.stringify(result)}`);
// { success: true, bidHash: "..." }
```

### Secure Multi-Party Computation

TEEs can be used for secure multi-party computation:

```javascript
// Define a secure multi-party computation
const mpcCode = `
  function secureMultiPartyComputation(params) {
    const { partyData, computationType } = params;
    
    // Process data from multiple parties securely
    let result;
    
    switch (computationType) {
      case "sum":
        result = partyData.reduce((sum, data) => sum + data, 0);
        break;
      case "average":
        result = partyData.reduce((sum, data) => sum + data, 0) / partyData.length;
        break;
      case "max":
        result = Math.max(...partyData);
        break;
      case "min":
        result = Math.min(...partyData);
        break;
      default:
        throw new Error("Unsupported computation type");
    }
    
    return { result };
  }
  
  secureMultiPartyComputation(params);
`;

// Execute the secure multi-party computation in a TEE
const result = await tee.execute({
  code: mpcCode,
  params: {
    partyData: [10, 20, 30, 40, 50], // Data from multiple parties
    computationType: "average"
  },
  provider: tee.ProviderType.NITRO
});

console.log(`MPC result: ${JSON.stringify(result)}`);
// { result: 30 }
```

### Privacy-Preserving Analytics

TEEs can be used for privacy-preserving analytics:

```javascript
// Define privacy-preserving analytics
const analyticsCode = `
  function privacyPreservingAnalytics(params) {
    const { userData, analyticsType } = params;
    
    // Process sensitive user data securely
    const results = {};
    
    if (analyticsType.includes("demographics")) {
      // Compute demographic statistics
      const ages = userData.map(user => user.age);
      results.averageAge = ages.reduce((sum, age) => sum + age, 0) / ages.length;
      
      // Count gender distribution without revealing individual data
      const genderCounts = {};
      userData.forEach(user => {
        genderCounts[user.gender] = (genderCounts[user.gender] || 0) + 1;
      });
      results.genderDistribution = genderCounts;
    }
    
    if (analyticsType.includes("financial")) {
      // Compute financial statistics
      const incomes = userData.map(user => user.income);
      results.averageIncome = incomes.reduce((sum, income) => sum + income, 0) / incomes.length;
      results.medianIncome = [...incomes].sort((a, b) => a - b)[Math.floor(incomes.length / 2)];
    }
    
    // Return aggregated results only
    return results;
  }
  
  privacyPreservingAnalytics(params);
`;

// Execute the privacy-preserving analytics in a TEE
const result = await tee.execute({
  code: analyticsCode,
  params: {
    userData: [
      { id: "user1", age: 25, gender: "female", income: 75000 },
      { id: "user2", age: 32, gender: "male", income: 85000 },
      { id: "user3", age: 45, gender: "female", income: 95000 },
      { id: "user4", age: 28, gender: "non-binary", income: 65000 },
      { id: "user5", age: 38, gender: "male", income: 105000 }
    ],
    analyticsType: ["demographics", "financial"]
  },
  provider: tee.ProviderType.SGX
});

console.log(`Analytics result: ${JSON.stringify(result)}`);
// { averageAge: 33.6, genderDistribution: { female: 2, male: 2, "non-binary": 1 }, averageIncome: 85000, medianIncome: 85000 }
```

## Configuration

The TEE Service can be configured using environment variables or configuration files:

### Environment Variables

- `R3E_FAAS__TEE__PROVIDER`: Default TEE provider (sgx, nitro)
- `R3E_FAAS__TEE__ATTESTATION_URL`: URL for attestation service
- `R3E_FAAS__TEE__ATTESTATION_TIMEOUT`: Timeout for attestation in seconds
- `R3E_FAAS__TEE__KEY_STORAGE_PATH`: Path for key storage
- `R3E_FAAS__TEE__MAX_MEMORY`: Maximum memory for TEE execution

### Configuration File

Configuration can also be provided in a YAML file:

```yaml
tee:
  provider: sgx
  attestation_url: https://attestation.example.com
  attestation_timeout: 30
  key_storage_path: /data/tee/keys
  max_memory: 1024
```

## Error Handling

The TEE Service provides detailed error messages for various failure scenarios:

```javascript
try {
  const result = await tee.execute({
    code: "function secureComputation() { return 42; }",
    provider: tee.ProviderType.SGX
  });
} catch (error) {
  console.error(`Error executing in TEE: ${error.message}`);
  
  if (error.code === "ATTESTATION_FAILED") {
    console.error(`Attestation failed: ${error.details}`);
  } else if (error.code === "EXECUTION_FAILED") {
    console.error(`Execution failed: ${error.details}`);
  } else if (error.code === "PROVIDER_ERROR") {
    console.error(`Provider error: ${error.details}`);
  }
}
```

## Security Considerations

- **Attestation**: Always verify attestation reports to ensure the TEE is genuine
- **Code Review**: Review code executed in TEEs to ensure it doesn't leak sensitive data
- **Memory Management**: Be aware of memory limitations and potential side-channel attacks
- **Key Management**: Properly manage keys generated within TEEs
- **Provider Security**: Different providers have different security properties

## Performance Considerations

- **Initialization Time**: TEEs take time to initialize
- **Memory Limitations**: TEEs have limited memory
- **CPU Limitations**: TEEs may have limited CPU resources
- **I/O Performance**: I/O operations from within TEEs may be slower
- **Provider Selection**: Different providers have different performance characteristics

## Examples

See the [examples](../examples/) directory for more examples of using the TEE Service:

- [Secure Key Management](../examples/neo-n3/tee-services/secure-key-management/)
- [Confidential Computing](../examples/neo-n3/tee-services/confidential-computing/)
- [TEE-Neo Integration](../examples/neo-n3/tee-services/tee-neo-integration/)
