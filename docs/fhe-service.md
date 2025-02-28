# Fully Homomorphic Encryption Service

The R3E FaaS platform provides a comprehensive Fully Homomorphic Encryption (FHE) Service that enables computation on encrypted data without decryption.

## Overview

Fully Homomorphic Encryption (FHE) is a form of encryption that allows computations to be performed directly on encrypted data without requiring decryption first. This enables secure processing of sensitive data while maintaining privacy and confidentiality.

The R3E FaaS FHE Service supports multiple FHE schemes:

- **TFHE**: A fast fully homomorphic encryption library over the torus
- **OpenFHE**: An open-source FHE library with multiple schemes

## Features

- **Multiple FHE Schemes**: Choose from various FHE implementations based on your needs
- **Key Generation**: Generate encryption and evaluation keys
- **Encryption/Decryption**: Encrypt and decrypt data
- **Homomorphic Operations**: Perform operations on encrypted data
  - Addition
  - Subtraction
  - Multiplication
  - Comparison
  - Boolean operations
- **Secure Storage**: Store keys and encrypted data securely
- **JavaScript API**: Easy-to-use JavaScript API for FHE operations

## JavaScript API

The FHE Service is accessible through the JavaScript API:

```javascript
// Access the FHE service
const fhe = r3e.fhe;
```

### Key Generation

```javascript
// Generate FHE keys
const keyPairId = await fhe.generateKeys(fhe.SchemeType.TFHE);
const publicKeyId = `${keyPairId}_public`;
const privateKeyId = `${keyPairId}_private`;

// Generate keys with specific parameters
const keyPairId = await fhe.generateKeys(fhe.SchemeType.TFHE, {
  securityLevel: "128",
  polyModulusDegree: 4096,
  plainModulus: 1024
});
```

### Encryption and Decryption

```javascript
// Encrypt data
const ciphertext1Id = await fhe.encrypt(publicKeyId, "42");
const ciphertext2Id = await fhe.encrypt(publicKeyId, "8");

// Decrypt data
const decryptedValue = await fhe.decrypt(privateKeyId, ciphertext1Id);
console.log(`Decrypted value: ${decryptedValue}`); // 42
```

### Homomorphic Operations

```javascript
// Addition
const addResultId = await fhe.add(ciphertext1Id, ciphertext2Id);
const addResult = await fhe.decrypt(privateKeyId, addResultId);
console.log(`Addition result: ${addResult}`); // 50

// Subtraction
const subResultId = await fhe.subtract(ciphertext1Id, ciphertext2Id);
const subResult = await fhe.decrypt(privateKeyId, subResultId);
console.log(`Subtraction result: ${subResult}`); // 34

// Multiplication
const multiplyResultId = await fhe.multiply(ciphertext1Id, ciphertext2Id);
const multiplyResult = await fhe.decrypt(privateKeyId, multiplyResultId);
console.log(`Multiplication result: ${multiplyResult}`); // 336

// Comparison
const isGreaterThanId = await fhe.greaterThan(ciphertext1Id, ciphertext2Id);
const isGreaterThan = await fhe.decrypt(privateKeyId, isGreaterThanId);
console.log(`Is greater than: ${isGreaterThan}`); // 1 (true)

// Boolean operations
const andResultId = await fhe.and(ciphertext1Id, ciphertext2Id);
const andResult = await fhe.decrypt(privateKeyId, andResultId);
console.log(`AND result: ${andResult}`);

const orResultId = await fhe.or(ciphertext1Id, ciphertext2Id);
const orResult = await fhe.decrypt(privateKeyId, orResultId);
console.log(`OR result: ${orResult}`);

const notResultId = await fhe.not(ciphertext1Id);
const notResult = await fhe.decrypt(privateKeyId, notResultId);
console.log(`NOT result: ${notResult}`);
```

### Key Management

```javascript
// List available keys
const keys = await fhe.listKeys();

// Get key details
const keyDetails = await fhe.getKey(publicKeyId);

// Delete keys
await fhe.deleteKey(publicKeyId);
await fhe.deleteKey(privateKeyId);
```

### Ciphertext Management

```javascript
// List available ciphertexts
const ciphertexts = await fhe.listCiphertexts();

// Get ciphertext details
const ciphertextDetails = await fhe.getCiphertext(ciphertext1Id);

// Delete ciphertext
await fhe.deleteCiphertext(ciphertext1Id);
```

## Scheme-Specific Features

### TFHE

```javascript
// Get TFHE-specific options
const tfheOptions = await fhe.getSchemeOptions(fhe.SchemeType.TFHE);

// Generate TFHE keys with specific options
const keyPairId = await fhe.generateKeys(fhe.SchemeType.TFHE, {
  securityLevel: "128",
  bootstrapKey: true,
  keySwitch: true
});

// Perform TFHE-specific operations
const bootstrappedId = await fhe.bootstrap(ciphertext1Id);
```

### OpenFHE

```javascript
// Get OpenFHE-specific options
const openFheOptions = await fhe.getSchemeOptions(fhe.SchemeType.OPENFHE);

// Generate OpenFHE keys with specific options
const keyPairId = await fhe.generateKeys(fhe.SchemeType.OPENFHE, {
  scheme: "CKKS",
  ringDim: 8192,
  mulDepth: 3,
  scalingModSize: 50,
  batchSize: 4096
});

// Perform OpenFHE-specific operations
const rotatedId = await fhe.rotate(ciphertext1Id, 1);
```

## Use Cases

### Private Data Analysis

FHE can be used to analyze sensitive data without exposing the raw data:

```javascript
// Generate keys
const keyPairId = await fhe.generateKeys(fhe.SchemeType.TFHE);
const publicKeyId = `${keyPairId}_public`;
const privateKeyId = `${keyPairId}_private`;

// Encrypt sensitive data
const salaries = [50000, 75000, 60000, 90000, 65000];
const encryptedSalaries = [];

for (const salary of salaries) {
  const encryptedSalary = await fhe.encrypt(publicKeyId, salary.toString());
  encryptedSalaries.push(encryptedSalary);
}

// Compute average salary on encrypted data
let sumId = await fhe.encrypt(publicKeyId, "0");
for (const encryptedSalary of encryptedSalaries) {
  sumId = await fhe.add(sumId, encryptedSalary);
}

// Create encrypted divisor
const countId = await fhe.encrypt(publicKeyId, salaries.length.toString());

// Compute average (this is simplified; real FHE division is more complex)
const averageId = await fhe.divide(sumId, countId);

// Decrypt the result
const average = await fhe.decrypt(privateKeyId, averageId);
console.log(`Average salary: ${average}`);
```

### Secure Multi-party Computation

FHE can be used for secure multi-party computation:

```javascript
// Party A generates keys
const keyPairIdA = await fhe.generateKeys(fhe.SchemeType.TFHE);
const publicKeyIdA = `${keyPairIdA}_public`;
const privateKeyIdA = `${keyPairIdA}_private`;

// Party B generates keys
const keyPairIdB = await fhe.generateKeys(fhe.SchemeType.TFHE);
const publicKeyIdB = `${keyPairIdB}_public`;
const privateKeyIdB = `${keyPairIdB}_private`;

// Party A encrypts their data
const dataA = 42;
const encryptedDataA = await fhe.encrypt(publicKeyIdA, dataA.toString());

// Party B encrypts their data
const dataB = 8;
const encryptedDataB = await fhe.encrypt(publicKeyIdB, dataB.toString());

// Party C (the compute party) performs computation on encrypted data
// First, re-encrypt Party B's data under Party A's public key
const reencryptedDataB = await fhe.reencrypt(encryptedDataB, publicKeyIdB, publicKeyIdA);

// Now compute on data encrypted under the same key
const resultId = await fhe.add(encryptedDataA, reencryptedDataB);

// Party A decrypts the result
const result = await fhe.decrypt(privateKeyIdA, resultId);
console.log(`Computation result: ${result}`); // 50
```

### Private Machine Learning

FHE can be used for privacy-preserving machine learning:

```javascript
// Generate keys
const keyPairId = await fhe.generateKeys(fhe.SchemeType.OPENFHE, {
  scheme: "CKKS",
  ringDim: 16384,
  mulDepth: 10,
  scalingModSize: 50,
  batchSize: 8192
});
const publicKeyId = `${keyPairId}_public`;
const privateKeyId = `${keyPairId}_private`;

// Encrypt training data
const encryptedFeatures = [];
for (const feature of trainingFeatures) {
  const encryptedFeature = await fhe.encrypt(publicKeyId, JSON.stringify(feature));
  encryptedFeatures.push(encryptedFeature);
}

// Encrypt model weights
const encryptedWeights = [];
for (const weight of modelWeights) {
  const encryptedWeight = await fhe.encrypt(publicKeyId, weight.toString());
  encryptedWeights.push(encryptedWeight);
}

// Perform private inference
const encryptedPredictions = [];
for (const encryptedFeature of encryptedFeatures) {
  // Compute dot product of feature and weights
  let dotProductId = await fhe.encrypt(publicKeyId, "0");
  for (let i = 0; i < encryptedWeights.length; i++) {
    const featureValue = JSON.parse(await fhe.decrypt(privateKeyId, encryptedFeature))[i];
    const featureValueId = await fhe.encrypt(publicKeyId, featureValue.toString());
    const productId = await fhe.multiply(featureValueId, encryptedWeights[i]);
    dotProductId = await fhe.add(dotProductId, productId);
  }
  
  // Apply activation function (simplified)
  const predictionId = await fhe.sigmoid(dotProductId);
  encryptedPredictions.push(predictionId);
}

// Decrypt predictions
const predictions = [];
for (const encryptedPrediction of encryptedPredictions) {
  const prediction = await fhe.decrypt(privateKeyId, encryptedPrediction);
  predictions.push(parseFloat(prediction));
}
console.log(`Predictions: ${predictions}`);
```

## Configuration

The FHE Service can be configured using environment variables or configuration files:

### Environment Variables

- `R3E_FAAS__FHE__SCHEME`: Default FHE scheme (tfhe, openfhe)
- `R3E_FAAS__FHE__STORAGE_TYPE`: Storage type for FHE data (memory, rocksdb)
- `R3E_FAAS__FHE__STORAGE_PATH`: Storage path for RocksDB
- `R3E_FAAS__FHE__MAX_CIPHERTEXT_SIZE`: Maximum ciphertext size in bytes
- `R3E_FAAS__FHE__TIMEOUT`: Timeout for FHE operations in seconds

### Configuration File

Configuration can also be provided in a YAML file:

```yaml
fhe:
  scheme: tfhe
  storage_type: rocksdb
  storage_path: /data/fhe
  max_ciphertext_size: 10485760 # 10 MB
  timeout: 300 # 5 minutes
```

## Error Handling

The FHE Service provides detailed error messages for various failure scenarios:

```javascript
try {
  const keyPairId = await fhe.generateKeys(fhe.SchemeType.TFHE);
} catch (error) {
  console.error(`Error generating keys: ${error.message}`);
  
  if (error.code === "KEY_GENERATION_ERROR") {
    console.error(`Key generation error: ${error.details}`);
  } else if (error.code === "INVALID_SCHEME_TYPE") {
    console.error(`Invalid scheme type: ${error.details}`);
  } else if (error.code === "SCHEME_ERROR") {
    console.error(`Scheme error: ${error.details}`);
  }
}
```

## Performance Considerations

- **Computational Intensity**: FHE operations are computationally intensive
- **Memory Usage**: FHE requires significant memory for key generation and operations
- **Operation Depth**: Complex operations may require bootstrapping or key switching
- **Scheme Selection**: Different schemes have different performance characteristics
- **Parameter Selection**: Parameter selection affects security, performance, and functionality

## Security Considerations

- **Key Management**: Encryption and evaluation keys must be managed securely
- **Parameter Selection**: Parameter selection affects security level
- **Noise Growth**: Operations on ciphertexts increase noise, which may affect decryption
- **Side-Channel Attacks**: Implementations should be resistant to side-channel attacks
- **Bootstrapping**: Bootstrapping may be required for deep circuits

## Limitations

- **Performance**: FHE operations are significantly slower than plaintext operations
- **Memory Usage**: FHE requires significant memory for key generation and operations
- **Operation Support**: Not all operations are efficiently supported in all schemes
- **Parameter Selection**: Parameter selection is complex and affects security, performance, and functionality
- **Division**: Division is not directly supported in most FHE schemes and requires approximation

## Examples

See the [examples](../examples/) directory for more examples of using the FHE Service:

- [FHE Computing Example](../examples/fhe_computing.js)
