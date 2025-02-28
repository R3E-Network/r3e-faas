# Zero-Knowledge Computing Service

The R3E FaaS platform provides a comprehensive Zero-Knowledge (ZK) Computing Service that enables privacy-preserving computations using various ZK-SNARK implementations.

## Overview

Zero-Knowledge proofs allow one party (the prover) to prove to another party (the verifier) that a statement is true without revealing any additional information beyond the validity of the statement itself. This is particularly useful for blockchain applications where privacy and confidentiality are important.

The R3E FaaS ZK Computing Service supports multiple ZK-SNARK implementations:

- **ZoKrates**: A toolbox for zkSNARKs on Ethereum
- **Bulletproofs**: A non-trusted-setup zero-knowledge proof system
- **Circom/SnarkJS**: A circuit compiler and proof generator
- **Bellman**: A Rust library for zkSNARK implementations
- **Arkworks**: A Rust ecosystem for zkSNARK development

## Features

- **Multiple ZK Providers**: Choose from various ZK implementations based on your needs
- **Circuit Compilation**: Compile ZK circuits from source code
- **Key Generation**: Generate proving and verification keys
- **Proof Generation**: Create ZK proofs with public and private inputs
- **Proof Verification**: Verify ZK proofs without revealing private inputs
- **Secure Storage**: Store circuits, keys, and proofs securely
- **JavaScript API**: Easy-to-use JavaScript API for ZK operations

## JavaScript API

The ZK Computing Service is accessible through the JavaScript API:

```javascript
// Access the ZK service
const zk = r3e.zk;
```

### Circuit Compilation

```javascript
// Compile a ZoKrates circuit
const circuitSource = `
  def main(private field a, private field b, field c) -> bool:
    return a * b == c
`;
const circuitId = await zk.compileCircuit(circuitSource, zk.CircuitType.ZOKRATES, "multiply");

// Compile a Circom circuit
const circomCircuitSource = `
  pragma circom 2.0.0;
  
  template Multiplier() {
    signal input a;
    signal input b;
    signal output c;
    
    c <== a * b;
  }
  
  component main = Multiplier();
`;
const circomCircuitId = await zk.compileCircuit(circomCircuitSource, zk.CircuitType.CIRCOM, "multiplier");
```

### Key Generation

```javascript
// Generate keys for a circuit
const { provingKeyId, verificationKeyId } = await zk.generateKeys(circuitId);
```

### Proof Generation

```javascript
// Generate a proof with ZoKrates
const publicInputs = ["6"]; // The public value c = 6
const privateInputs = ["2", "3"]; // The private values a = 2, b = 3
const proofId = await zk.generateProof(circuitId, provingKeyId, publicInputs, privateInputs);

// Generate a proof with Circom/SnarkJS
const circomPublicInputs = { c: "6" };
const circomPrivateInputs = { a: "2", b: "3" };
const circomProofId = await zk.generateProof(circomCircuitId, circomProvingKeyId, circomPublicInputs, circomPrivateInputs);
```

### Proof Verification

```javascript
// Verify a proof
const isValid = await zk.verifyProof(proofId, verificationKeyId, publicInputs);
console.log(`Proof is valid: ${isValid}`);
```

### Range Proofs with Bulletproofs

```javascript
// Create a range proof
const commitment = await zk.createRangeProof({
  value: 42,
  min: 0,
  max: 100,
  blindingFactor: "random" // or provide a specific blinding factor
});

// Verify a range proof
const isValid = await zk.verifyRangeProof(commitment.proof, commitment.commitment, 0, 100);
```

### Circuit Management

```javascript
// List available circuits
const circuits = await zk.listCircuits();

// Get circuit details
const circuit = await zk.getCircuit(circuitId);

// Delete a circuit
await zk.deleteCircuit(circuitId);
```

## Provider-Specific Features

### ZoKrates

```javascript
// Get ZoKrates-specific options
const zokratesOptions = await zk.getProviderOptions(zk.ProviderType.ZOKRATES);

// Compile with ZoKrates-specific options
const circuitId = await zk.compileCircuit(circuitSource, zk.CircuitType.ZOKRATES, "multiply", {
  curve: "bn128",
  scheme: "g16"
});
```

### Circom/SnarkJS

```javascript
// Get Circom-specific options
const circomOptions = await zk.getProviderOptions(zk.ProviderType.CIRCOM);

// Compile with Circom-specific options
const circuitId = await zk.compileCircuit(circomCircuitSource, zk.CircuitType.CIRCOM, "multiplier", {
  prime: "bn128",
  protocol: "groth16"
});
```

### Bulletproofs

```javascript
// Create a Bulletproofs multiparty range proof
const multipartyCommitment = await zk.createMultipartyRangeProof({
  parties: 3,
  values: [10, 20, 30],
  min: 0,
  max: 100
});

// Verify a multiparty range proof
const isValid = await zk.verifyMultipartyRangeProof(multipartyCommitment.proof, multipartyCommitment.commitments, 0, 100);
```

### Bellman

```javascript
// Use Bellman for a custom circuit
const bellmanCircuitId = await zk.compileCircuit(bellmanCircuitSource, zk.CircuitType.BELLMAN, "custom");
```

### Arkworks

```javascript
// Use Arkworks for a custom circuit
const arkworksCircuitId = await zk.compileCircuit(arkworksCircuitSource, zk.CircuitType.ARKWORKS, "custom");
```

## Use Cases

### Private Transactions

Zero-Knowledge proofs can be used to implement private transactions on public blockchains:

```javascript
// Create a circuit for private transfers
const transferCircuitSource = `
  def main(private field sender_balance, private field receiver_balance, private field amount,
           field sender_balance_hash, field receiver_balance_hash,
           field new_sender_balance_hash, field new_receiver_balance_hash) -> bool:
    // Verify initial balances
    field computed_sender_hash = hash(sender_balance)
    field computed_receiver_hash = hash(receiver_balance)
    
    // Verify sufficient balance
    bool sufficient = sender_balance >= amount
    
    // Compute new balances
    field new_sender_balance = sender_balance - amount
    field new_receiver_balance = receiver_balance + amount
    
    // Verify new balance hashes
    field computed_new_sender_hash = hash(new_sender_balance)
    field computed_new_receiver_hash = hash(new_receiver_balance)
    
    return sufficient && 
           computed_sender_hash == sender_balance_hash &&
           computed_receiver_hash == receiver_balance_hash &&
           computed_new_sender_hash == new_sender_balance_hash &&
           computed_new_receiver_hash == new_receiver_balance_hash
`;

// Compile the circuit
const circuitId = await zk.compileCircuit(transferCircuitSource, zk.CircuitType.ZOKRATES, "private_transfer");

// Generate keys
const { provingKeyId, verificationKeyId } = await zk.generateKeys(circuitId);

// Generate a proof for a private transfer
const senderBalance = 100;
const receiverBalance = 50;
const amount = 30;

// Hash the balances (in a real implementation, these would be cryptographic hashes)
const senderBalanceHash = await zk.hash(senderBalance.toString());
const receiverBalanceHash = await zk.hash(receiverBalance.toString());
const newSenderBalanceHash = await zk.hash((senderBalance - amount).toString());
const newReceiverBalanceHash = await zk.hash((receiverBalance + amount).toString());

// Generate the proof
const publicInputs = [
  senderBalanceHash,
  receiverBalanceHash,
  newSenderBalanceHash,
  newReceiverBalanceHash
];
const privateInputs = [
  senderBalance.toString(),
  receiverBalance.toString(),
  amount.toString()
];
const proofId = await zk.generateProof(circuitId, provingKeyId, publicInputs, privateInputs);

// Verify the proof
const isValid = await zk.verifyProof(proofId, verificationKeyId, publicInputs);
console.log(`Private transfer proof is valid: ${isValid}`);
```

### Identity Verification

Zero-Knowledge proofs can be used for identity verification without revealing sensitive information:

```javascript
// Create a circuit for age verification
const ageCircuitSource = `
  def main(private field birthdate, field current_date, field min_age, field hash) -> bool:
    // Verify the birthdate hash
    field computed_hash = hash(birthdate)
    bool hash_valid = computed_hash == hash
    
    // Calculate age
    field age = (current_date - birthdate) / 365
    
    // Verify age is at least min_age
    bool age_valid = age >= min_age
    
    return hash_valid && age_valid
`;

// Compile the circuit
const circuitId = await zk.compileCircuit(ageCircuitSource, zk.CircuitType.ZOKRATES, "age_verification");

// Generate keys
const { provingKeyId, verificationKeyId } = await zk.generateKeys(circuitId);

// Generate a proof for age verification
const birthdate = 19900101; // January 1, 1990
const currentDate = 20230101; // January 1, 2023
const minAge = 21;

// Hash the birthdate (in a real implementation, this would be a cryptographic hash)
const birthdateHash = await zk.hash(birthdate.toString());

// Generate the proof
const publicInputs = [
  currentDate.toString(),
  minAge.toString(),
  birthdateHash
];
const privateInputs = [
  birthdate.toString()
];
const proofId = await zk.generateProof(circuitId, provingKeyId, publicInputs, privateInputs);

// Verify the proof
const isValid = await zk.verifyProof(proofId, verificationKeyId, publicInputs);
console.log(`Age verification proof is valid: ${isValid}`);
```

### Confidential Smart Contracts

Zero-Knowledge proofs can be used to implement confidential smart contracts:

```javascript
// Create a circuit for a confidential auction
const auctionCircuitSource = `
  def main(private field bid, private field bidder, field bid_hash, field bidder_hash,
           field highest_bid, field highest_bidder_hash) -> bool:
    // Verify bid and bidder hashes
    field computed_bid_hash = hash(bid)
    field computed_bidder_hash = hash(bidder)
    
    // Verify bid is higher than current highest bid
    bool is_higher = bid > highest_bid
    
    return computed_bid_hash == bid_hash &&
           computed_bidder_hash == bidder_hash &&
           is_higher
`;

// Compile the circuit
const circuitId = await zk.compileCircuit(auctionCircuitSource, zk.CircuitType.ZOKRATES, "confidential_auction");

// Generate keys
const { provingKeyId, verificationKeyId } = await zk.generateKeys(circuitId);

// Generate a proof for a confidential auction bid
const bid = 1000;
const bidder = 12345; // Bidder ID
const highestBid = 900;

// Hash the bid and bidder (in a real implementation, these would be cryptographic hashes)
const bidHash = await zk.hash(bid.toString());
const bidderHash = await zk.hash(bidder.toString());
const highestBidderHash = await zk.hash("previous-bidder");

// Generate the proof
const publicInputs = [
  bidHash,
  bidderHash,
  highestBid.toString(),
  highestBidderHash
];
const privateInputs = [
  bid.toString(),
  bidder.toString()
];
const proofId = await zk.generateProof(circuitId, provingKeyId, publicInputs, privateInputs);

// Verify the proof
const isValid = await zk.verifyProof(proofId, verificationKeyId, publicInputs);
console.log(`Confidential auction proof is valid: ${isValid}`);
```

## Configuration

The ZK Computing Service can be configured using environment variables or configuration files:

### Environment Variables

- `R3E_FAAS__ZK__PROVIDER`: Default ZK provider (zokrates, bulletproofs, circom, bellman, arkworks)
- `R3E_FAAS__ZK__STORAGE_TYPE`: Storage type for ZK data (memory, rocksdb)
- `R3E_FAAS__ZK__STORAGE_PATH`: Storage path for RocksDB
- `R3E_FAAS__ZK__MAX_CIRCUIT_SIZE`: Maximum circuit size in bytes
- `R3E_FAAS__ZK__TIMEOUT`: Timeout for ZK operations in seconds

### Configuration File

Configuration can also be provided in a YAML file:

```yaml
zk:
  provider: zokrates
  storage_type: rocksdb
  storage_path: /data/zk
  max_circuit_size: 10485760 # 10 MB
  timeout: 300 # 5 minutes
```

## Error Handling

The ZK Computing Service provides detailed error messages for various failure scenarios:

```javascript
try {
  const circuitId = await zk.compileCircuit(circuitSource, zk.CircuitType.ZOKRATES, "multiply");
} catch (error) {
  console.error(`Error compiling circuit: ${error.message}`);
  
  if (error.code === "COMPILATION_ERROR") {
    console.error(`Compilation error: ${error.details}`);
  } else if (error.code === "INVALID_CIRCUIT_TYPE") {
    console.error(`Invalid circuit type: ${error.details}`);
  } else if (error.code === "PROVIDER_ERROR") {
    console.error(`Provider error: ${error.details}`);
  }
}
```

## Performance Considerations

- **Circuit Complexity**: More complex circuits require more computational resources
- **Proof Generation**: Proof generation is computationally intensive
- **Storage**: Circuits, keys, and proofs can be large and require significant storage
- **Provider Selection**: Different providers have different performance characteristics

## Security Considerations

- **Trusted Setup**: Some ZK systems require a trusted setup phase
- **Key Management**: Proving and verification keys must be managed securely
- **Input Validation**: Inputs to ZK circuits must be validated
- **Provider Security**: Different providers have different security properties

## Examples

See the [examples](../examples/) directory for more examples of using the ZK Computing Service:

- [ZK Computing Example](../examples/zk_computing.js)
- [Circom Example](../examples/circom_example.js)
- [Bellman Example](../examples/bellman_example.js)
- [Arkworks Example](../examples/arkworks_example.js)
