# Zero-Knowledge Computing Service

The Zero-Knowledge Computing Service in R3E FaaS provides a comprehensive platform for creating and verifying zero-knowledge proofs. This service enables developers to build privacy-preserving applications that can prove statements without revealing sensitive information.

## Overview

Zero-Knowledge proofs allow one party (the prover) to prove to another party (the verifier) that a statement is true without revealing any additional information beyond the validity of the statement itself. The R3E FaaS platform supports multiple ZK solutions, including ZoKrates, Bulletproofs, and Circom.

## Features

- **Multiple ZK Solutions**: Support for various ZK frameworks
  - ZoKrates: A toolbox for zkSNARKs on Ethereum
  - Bulletproofs: Short non-interactive zero-knowledge proofs without a trusted setup
  - Circom: A circuit compiler for zkSNARKs
  - Bellman: A Rust library for zkSNARK implementations
  - Arkworks: A comprehensive ecosystem for zkSNARK development

- **Circuit Compilation**: Compile ZK circuits from source code
- **Key Generation**: Generate proving and verification keys
- **Proof Generation**: Create ZK proofs with public and private inputs
- **Proof Verification**: Verify ZK proofs without revealing private inputs

## JavaScript API

The Zero-Knowledge Computing Service is accessible through the `zk` module in the R3E JavaScript API:

```javascript
import { zk } from "r3e";
```

### Circuit Types

The service supports multiple circuit types:

```javascript
// Available circuit types
zk.CircuitType.ZOKRATES    // ZoKrates circuits
zk.CircuitType.BULLETPROOFS // Bulletproofs circuits
zk.CircuitType.CIRCOM      // Circom circuits
zk.CircuitType.BELLMAN     // Bellman circuits
zk.CircuitType.ARKWORKS    // Arkworks circuits
```

### Compiling Circuits

Compile a ZK circuit from source code:

```javascript
const circuitSource = `
  def main(private field a, private field b, field c) -> bool:
    return a * b == c
`;

const circuitId = zk.compileCircuit(
  circuitSource,
  zk.CircuitType.ZOKRATES,
  "multiply"
);
```

### Generating Keys

Generate proving and verification keys for a circuit:

```javascript
const { provingKeyId, verificationKeyId } = zk.generateKeys(circuitId);
```

### Creating Proofs

Generate a zero-knowledge proof:

```javascript
const publicInputs = ["6"]; // Public inputs (visible to verifier)
const privateInputs = ["2", "3"]; // Private inputs (hidden from verifier)

const proofId = zk.generateProof(
  circuitId,
  provingKeyId,
  publicInputs,
  privateInputs
);
```

### Verifying Proofs

Verify a zero-knowledge proof:

```javascript
const isValid = zk.verifyProof(
  proofId,
  verificationKeyId,
  publicInputs
);

console.log(`Proof is valid: ${isValid}`);
```

## Use Cases

### Private Identity Verification

Verify identity attributes without revealing the actual data:

```javascript
// Circuit to verify age is above 18 without revealing actual age
const circuitSource = `
  def main(private field age, field min_age) -> bool:
    return age >= min_age
`;

// Compile circuit and generate keys
const circuitId = zk.compileCircuit(circuitSource, zk.CircuitType.ZOKRATES, "age_verification");
const { provingKeyId, verificationKeyId } = zk.generateKeys(circuitId);

// Generate proof (age is 25, minimum age is 18)
const publicInputs = ["18"]; // Minimum age (public)
const privateInputs = ["25"]; // Actual age (private)
const proofId = zk.generateProof(circuitId, provingKeyId, publicInputs, privateInputs);

// Verify proof
const isValid = zk.verifyProof(proofId, verificationKeyId, publicInputs);
console.log(`Age verification valid: ${isValid}`);
```

### Confidential Transactions

Prove transaction validity without revealing amounts:

```javascript
// Circuit to verify a transaction is valid without revealing amounts
const circuitSource = `
  def main(private field balance, private field amount, field remaining) -> bool:
    return balance - amount == remaining && amount > 0
`;

// Compile circuit and generate keys
const circuitId = zk.compileCircuit(circuitSource, zk.CircuitType.ZOKRATES, "transaction");
const { provingKeyId, verificationKeyId } = zk.generateKeys(circuitId);

// Generate proof (balance is 100, amount is 30, remaining is 70)
const publicInputs = ["70"]; // Remaining balance (public)
const privateInputs = ["100", "30"]; // Initial balance and amount (private)
const proofId = zk.generateProof(circuitId, provingKeyId, publicInputs, privateInputs);

// Verify proof
const isValid = zk.verifyProof(proofId, verificationKeyId, publicInputs);
console.log(`Transaction valid: ${isValid}`);
```

### Merkle Tree Membership

Prove membership in a Merkle tree without revealing the path:

```javascript
// Using Circom for Merkle tree membership proof
const circuitSource = `
  pragma circom 2.0.0;
  
  template MerkleTreeCheck(levels) {
    signal input leaf;
    signal input root;
    signal private input path_elements[levels];
    signal private input path_indices[levels];
    
    // Circuit implementation would go here
    
    signal output out;
    out <== 1;
  }
  
  component main = MerkleTreeCheck(10);
`;

// Compile circuit and generate keys
const circuitId = zk.compileCircuit(circuitSource, zk.CircuitType.CIRCOM, "merkle_proof");
const { provingKeyId, verificationKeyId } = zk.generateKeys(circuitId);

// Generate proof
const publicInputs = [
  "123456789", // leaf
  "987654321"  // root
];
const privateInputs = [
  "[111111, 222222, 333333, 444444, 555555, 666666, 777777, 888888, 999999, 000000]", // path_elements
  "[0, 1, 0, 1, 0, 1, 0, 1, 0, 1]" // path_indices
];
const proofId = zk.generateProof(circuitId, provingKeyId, publicInputs, privateInputs);

// Verify proof
const isValid = zk.verifyProof(proofId, verificationKeyId, publicInputs);
console.log(`Merkle proof valid: ${isValid}`);
```

## Performance Considerations

- **Circuit Complexity**: More complex circuits require more computational resources
- **Proof Generation**: Generating proofs is computationally intensive
- **Proof Verification**: Verification is typically much faster than proof generation
- **Storage**: Keys and proofs require storage space

## Security Considerations

- **Trusted Setup**: Some ZK systems require a trusted setup phase
- **Parameter Selection**: Proper parameter selection is crucial for security
- **Implementation**: Use the provided APIs rather than custom implementations
- **Key Management**: Securely store proving and verification keys

## Limitations

- **Computational Overhead**: ZK proofs can be computationally expensive
- **Circuit Size**: Complex circuits may have performance implications
- **Language Expressiveness**: ZK circuit languages have limitations compared to general-purpose languages

## Further Reading

- [ZoKrates Documentation](https://zokrates.github.io/)
- [Bulletproofs Paper](https://eprint.iacr.org/2017/1066.pdf)
- [Circom Documentation](https://docs.circom.io/)
- [Bellman GitHub Repository](https://github.com/zkcrypto/bellman)
- [Arkworks Documentation](https://arkworks.rs/)
