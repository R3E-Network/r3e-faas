# Fully Homomorphic Encryption Service

The Fully Homomorphic Encryption (FHE) Service in R3E FaaS enables computation on encrypted data without decryption. This powerful cryptographic technique allows developers to build privacy-preserving applications that can process sensitive data while maintaining confidentiality.

## Overview

Fully Homomorphic Encryption allows computations to be performed on encrypted data, producing an encrypted result that, when decrypted, matches the result of the operations as if they had been performed on the plaintext. The R3E FaaS platform supports multiple FHE schemes, including TFHE, OpenFHE, SEAL, HElib, and Lattigo.

## Features

- **Multiple FHE Schemes**: Support for various FHE libraries
  - TFHE: Fast Fully Homomorphic Encryption over the Torus
  - OpenFHE: Open-source FHE library
  - SEAL: Simple Encrypted Arithmetic Library
  - HElib: Homomorphic Encryption library
  - Lattigo: Lattice-based cryptographic library

- **Key Management**: Generate and manage FHE key pairs
- **Homomorphic Operations**: Perform addition, subtraction, multiplication on encrypted data
- **Noise Budget Management**: Track and manage noise budget for operations
- **Secure Computation**: Compute on encrypted data without decryption

## JavaScript API

The Fully Homomorphic Encryption Service is accessible through the `fhe` module in the R3E JavaScript API:

```javascript
import { fhe } from "r3e";
```

### Scheme Types

The service supports multiple FHE schemes:

```javascript
// Available scheme types
fhe.SchemeType.TFHE     // TFHE scheme
fhe.SchemeType.OPENFHE  // OpenFHE scheme
fhe.SchemeType.SEAL     // SEAL scheme
fhe.SchemeType.HELIB    // HElib scheme
fhe.SchemeType.LATTIGO  // Lattigo scheme
```

### Generating Keys

Generate a key pair for FHE operations:

```javascript
const keyPairId = fhe.generateKeys(fhe.SchemeType.TFHE, {
  securityLevel: 128,
  polynomialModulusDegree: 4096,
  plaintextModulus: 1024
});

// For demonstration purposes, we assume we have access to the public and private keys
const publicKeyId = `${keyPairId}_public`;
const privateKeyId = `${keyPairId}_private`;
```

### Encrypting Data

Encrypt data using a public key:

```javascript
const plaintext = "42"; // We'll encrypt the number 42
const ciphertextId = fhe.encrypt(publicKeyId, plaintext);
```

### Homomorphic Operations

Perform operations on encrypted data:

```javascript
// Encrypt two values
const ciphertext1Id = fhe.encrypt(publicKeyId, "42");
const ciphertext2Id = fhe.encrypt(publicKeyId, "8");

// Addition
const addResultId = fhe.add(ciphertext1Id, ciphertext2Id);

// Subtraction
const subtractResultId = fhe.subtract(ciphertext1Id, ciphertext2Id);

// Multiplication
const multiplyResultId = fhe.multiply(ciphertext1Id, ciphertext2Id);

// Negation
const negateResultId = fhe.negate(ciphertext1Id);
```

### Decrypting Results

Decrypt the results using a private key:

```javascript
const addResult = fhe.decrypt(privateKeyId, addResultId, true);
console.log(`Addition result: ${addResult}`); // Should be 50 (42 + 8)

const multiplyResult = fhe.decrypt(privateKeyId, multiplyResultId, true);
console.log(`Multiplication result: ${multiplyResult}`); // Should be 336 (42 * 8)
```

### Noise Budget Management

Check the noise budget of a ciphertext:

```javascript
const noiseBudget = fhe.estimateNoiseBudget(multiplyResultId);
console.log(`Noise budget after multiplication: ${noiseBudget}`);
```

## Use Cases

### Private Information Retrieval

Retrieve data from a database without revealing which record is being accessed:

```javascript
// Generate keys
const keyPairId = fhe.generateKeys(fhe.SchemeType.TFHE);
const publicKeyId = `${keyPairId}_public`;
const privateKeyId = `${keyPairId}_private`;

// Simulate a database
const database = [
  "Record 1: User data",
  "Record 2: Financial information",
  "Record 3: Medical history",
  "Record 4: Personal preferences"
];

// Encrypt the query index (we want to retrieve Record 3)
const queryIndex = "2"; // 0-based index for Record 3
const encryptedQueryId = fhe.encrypt(publicKeyId, queryIndex);

// Server processes the encrypted query without knowing which record is requested
// This is a simplified example - in practice, more complex FHE operations would be used
const resultIds = [];
for (let i = 0; i < database.length; i++) {
  // Create a selector and process each entry
  // In a real implementation, this would use homomorphic operations
  const resultId = `result_${i}_${encryptedQueryId}`;
  resultIds.push(resultId);
}

// Combine results
let combinedResultId = resultIds[0];
for (let i = 1; i < resultIds.length; i++) {
  combinedResultId = fhe.add(combinedResultId, resultIds[i]);
}

// Decrypt the result
const retrievedRecord = fhe.decrypt(privateKeyId, combinedResultId, true);
console.log(`Retrieved record: ${retrievedRecord}`);
```

### Secure Multi-Party Computation

Compute the average of salaries without revealing individual salaries:

```javascript
// Party A generates keys
const keyPairIdA = fhe.generateKeys(fhe.SchemeType.TFHE);
const publicKeyIdA = `${keyPairIdA}_public`;
const privateKeyIdA = `${keyPairIdA}_private`;

// Party A encrypts their salary
const salaryA = "120000"; // $120,000
const encryptedSalaryAId = fhe.encrypt(publicKeyIdA, salaryA);

// Party B encrypts their salary using Party A's public key
const salaryB = "95000"; // $95,000
const encryptedSalaryBId = fhe.encrypt(publicKeyIdA, salaryB);

// Compute the sum of salaries homomorphically
const sumSalariesId = fhe.add(encryptedSalaryAId, encryptedSalaryBId);

// Party A decrypts the sum
const sumSalaries = fhe.decrypt(privateKeyIdA, sumSalariesId, true);

// Calculate average
const averageSalary = parseInt(sumSalaries) / 2;
console.log(`Average salary: ${averageSalary}`); // $107,500
```

### Encrypted Machine Learning

Perform machine learning on encrypted data:

```javascript
// Generate keys
const keyPairId = fhe.generateKeys(fhe.SchemeType.TFHE);
const publicKeyId = `${keyPairId}_public`;
const privateKeyId = `${keyPairId}_private`;

// Encrypt training data
const encryptedFeatures = [];
const features = [
  [1.2, 3.4, 2.1],
  [2.3, 1.8, 0.9],
  [3.1, 2.2, 1.5]
];

for (const feature of features) {
  const encryptedFeature = feature.map(value => 
    fhe.encrypt(publicKeyId, value.toString())
  );
  encryptedFeatures.push(encryptedFeature);
}

// Encrypt model weights
const weights = [0.5, 0.3, 0.2];
const encryptedWeights = weights.map(weight => 
  fhe.encrypt(publicKeyId, weight.toString())
);

// Compute predictions homomorphically
const encryptedPredictions = [];
for (const encryptedFeature of encryptedFeatures) {
  // Compute weighted sum
  let encryptedSum = fhe.encrypt(publicKeyId, "0");
  for (let i = 0; i < encryptedFeature.length; i++) {
    const encryptedProduct = fhe.multiply(encryptedFeature[i], encryptedWeights[i]);
    encryptedSum = fhe.add(encryptedSum, encryptedProduct);
  }
  encryptedPredictions.push(encryptedSum);
}

// Decrypt predictions
const predictions = encryptedPredictions.map(encryptedPrediction => 
  fhe.decrypt(privateKeyId, encryptedPrediction, true)
);
console.log("Predictions:", predictions);
```

## Performance Considerations

- **Scheme Selection**: Different schemes have different performance characteristics
- **Parameter Selection**: Parameters affect security, performance, and noise growth
- **Operation Complexity**: Homomorphic operations have varying computational costs
- **Noise Growth**: Noise grows with each operation, limiting the number of operations

## Security Considerations

- **Key Management**: Securely store private keys
- **Parameter Selection**: Proper parameter selection is crucial for security
- **Implementation**: Use the provided APIs rather than custom implementations
- **Side-Channel Attacks**: Be aware of potential side-channel vulnerabilities

## Limitations

- **Performance Overhead**: FHE operations are computationally expensive
- **Noise Growth**: Noise accumulates with operations, limiting computation depth
- **Operation Support**: Not all operations are efficiently supported in FHE
- **Parameter Selection**: Balancing security, performance, and functionality

## Further Reading

- [TFHE Library](https://tfhe.github.io/tfhe/)
- [OpenFHE Library](https://openfhe.org/)
- [Microsoft SEAL](https://github.com/microsoft/SEAL)
- [HElib](https://github.com/homenc/HElib)
- [Lattigo](https://github.com/tuneinsight/lattigo)
