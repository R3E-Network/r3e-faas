# Neo N3 Secure Multi-Party Computation Example

This example demonstrates how to implement secure multi-party computation (MPC) using Trusted Execution Environments (TEEs) in the Neo N3 FaaS platform. Secure multi-party computation allows multiple parties to jointly compute a function over their inputs while keeping those inputs private.

## Overview

The secure multi-party computation example shows how to:

1. Perform joint computations without revealing individual inputs
2. Implement various MPC protocols within a TEE
3. Coordinate computations between multiple parties
4. Ensure privacy and security throughout the computation process
5. Integrate MPC with Neo N3 blockchain applications

## Files

- `function.js`: The JavaScript function that implements the secure multi-party computation service
- `config.yaml`: Configuration file for the secure multi-party computation service
- `register.js`: Script to register the function with the FaaS platform
- `smart-contract/`: Directory containing a sample Neo N3 smart contract that interacts with the MPC service
- `lib/`: Directory containing MPC libraries and utilities

## Prerequisites

- Neo N3 FaaS platform installed and running
- Access to a Neo N3 node (testnet or mainnet)
- Node.js installed for running the registration script
- Neo N3 smart contract development environment (optional, for testing the smart contract)
- TEE-enabled hardware (Intel SGX, AMD SEV, or ARM TrustZone)

## Setup

1. Configure your Neo N3 node connection and TEE settings in `config.yaml`
2. Register the function using the registration script:

```bash
node register.js
```

3. (Optional) Deploy the sample smart contract to interact with the MPC service

## How It Works

The secure multi-party computation service leverages Trusted Execution Environments (TEEs) to provide a secure environment for joint computations. The TEE acts as a trusted third party that can receive encrypted inputs from multiple parties, perform the computation, and return the result without revealing the individual inputs.

### MPC Protocols

The example implements several MPC protocols:

1. **Garbled Circuit Protocol**: Based on Yao's protocol, this allows two parties to jointly evaluate a function without revealing their inputs.

2. **Secret Sharing Protocol**: Based on Shamir's Secret Sharing, this allows multiple parties to split their inputs into shares that reveal nothing individually but can be combined to compute a function.

3. **Homomorphic Encryption Protocol**: This allows computations to be performed on encrypted data without decrypting it first.

### Computation Flow

The typical flow for a secure multi-party computation is:

1. **Setup Phase**: Parties agree on the function to compute and establish secure channels with the TEE.

2. **Input Phase**: Each party encrypts their input with the TEE's public key and sends it to the TEE.

3. **Computation Phase**: The TEE decrypts the inputs, performs the computation, and encrypts the result.

4. **Output Phase**: The TEE sends the encrypted result to each party, who can then decrypt it with their private key.

### Example: Private Set Intersection

One example use case implemented in this service is private set intersection, where multiple parties want to find the common elements in their sets without revealing the elements that are not in the intersection:

```javascript
// Example private set intersection
async function privateSetIntersection(encryptedSets) {
  // Decrypt the sets inside the TEE
  const sets = await decryptSetsInTEE(encryptedSets);
  
  // Find the intersection
  const intersection = findIntersection(sets);
  
  // Encrypt the result for each party
  const encryptedResult = await encryptResultForParties(intersection, parties);
  
  return encryptedResult;
}
```

### Example: Secure Aggregation

Another example is secure aggregation, where multiple parties want to compute the sum of their values without revealing their individual values:

```javascript
// Example secure aggregation
async function secureAggregation(encryptedValues) {
  // Decrypt the values inside the TEE
  const values = await decryptValuesInTEE(encryptedValues);
  
  // Compute the sum
  const sum = values.reduce((acc, val) => acc + val, 0);
  
  // Encrypt the result for each party
  const encryptedResult = await encryptResultForParties(sum, parties);
  
  return encryptedResult;
}
```

### Example: Secure Machine Learning

The service also supports secure machine learning, where multiple parties want to train a model on their combined data without revealing their individual data:

```javascript
// Example secure machine learning
async function secureModelTraining(encryptedDatasets) {
  // Decrypt the datasets inside the TEE
  const datasets = await decryptDatasetsInTEE(encryptedDatasets);
  
  // Combine the datasets
  const combinedDataset = combineDatasets(datasets);
  
  // Train the model
  const model = await trainModel(combinedDataset);
  
  // Encrypt the model for each party
  const encryptedModel = await encryptResultForParties(model, parties);
  
  return encryptedModel;
}
```

## Blockchain Integration

The example demonstrates integration with Neo N3 blockchain for coordinating MPC computations and storing computation results:

```javascript
// Example blockchain integration
async function coordinateComputationOnChain(computationId, parties, function) {
  // Verify all parties have committed to the computation
  const allCommitted = await verifyCommitmentsOnChain(computationId, parties);
  
  if (!allCommitted) {
    throw new Error('Not all parties have committed to the computation');
  }
  
  // Perform the computation
  const result = await performMPCComputation(computationId, parties, function);
  
  // Record the result on the blockchain
  await recordResultOnChain(computationId, result);
  
  return result;
}
```

## Security Considerations

When using this example, be aware of the following security considerations:

1. **TEE Limitations**: While TEEs provide strong security guarantees, they are not immune to all attacks. Stay updated on the latest security advisories for your TEE technology.

2. **Side-Channel Attacks**: Be aware that some sophisticated side-channel attacks might still be possible against TEEs.

3. **Input Validation**: Always validate inputs to prevent malicious parties from manipulating the computation.

4. **Attestation**: Always verify the TEE environment through remote attestation before sending sensitive data.

5. **Collusion**: Be aware that if multiple parties collude, they might be able to learn more information than intended.

## Customization

You can customize this example by:

- Implementing additional MPC protocols
- Adding support for specific use cases (e.g., privacy-preserving analytics, secure auctions)
- Extending the blockchain integration for your specific requirements
- Implementing domain-specific MPC algorithms

## Additional Resources

- [Neo N3 TEE Services Documentation](../../docs/neo-n3/components/tee-services.md)
- [Neo N3 FaaS Platform Documentation](../../docs/neo-n3/README.md)
- [JavaScript Function Development Guide](../../docs/neo-n3/guides/function-development.md)
- [Neo N3 Smart Contract Development Guide](https://docs.neo.org/docs/en-us/develop/write/basics.html)
- [Secure Multi-Party Computation Resources](https://www.mpcalliance.org/learn)
