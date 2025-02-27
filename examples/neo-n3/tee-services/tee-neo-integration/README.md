# Neo N3 TEE Integration Example

This example demonstrates how to integrate Trusted Execution Environments (TEEs) with Neo N3 blockchain applications in the Neo N3 FaaS platform. The integration enables secure and verifiable execution of smart contracts and off-chain computations with blockchain-based verification.

## Overview

The TEE integration with Neo N3 example shows how to:

1. Deploy and execute Neo N3 smart contracts within a TEE
2. Verify on-chain data inside a TEE environment
3. Sign transactions using keys protected by a TEE
4. Provide verifiable proofs of computation for Neo N3 smart contracts
5. Implement secure oracles with TEE-based integrity guarantees

## Files

- `function.js`: The JavaScript function that implements the TEE integration service
- `config.yaml`: Configuration file for the TEE integration service
- `register.js`: Script to register the function with the FaaS platform
- `smart-contract/`: Directory containing sample Neo N3 smart contracts that integrate with TEEs
- `lib/`: Directory containing TEE integration libraries and utilities

## Prerequisites

- Neo N3 FaaS platform installed and running
- Access to a Neo N3 node (testnet or mainnet)
- Node.js installed for running the registration script
- Neo N3 smart contract development environment
- TEE-enabled hardware (Intel SGX, AMD SEV, or ARM TrustZone)

## Setup

1. Configure your Neo N3 node connection and TEE settings in `config.yaml`
2. Register the function using the registration script:

```bash
node register.js
```

3. Deploy the sample smart contracts to interact with the TEE integration service

## How It Works

The TEE integration with Neo N3 leverages the security properties of TEEs to enhance blockchain applications with confidential computing capabilities. The integration works through several key mechanisms:

### 1. Secure Smart Contract Execution

Smart contracts can be executed within a TEE, providing confidentiality for the contract's state and execution:

```javascript
// Example of executing a Neo N3 smart contract within a TEE
async function executeContractInTEE(contractHash, operation, args, privateKey) {
  // Verify the TEE environment through attestation
  await verifyTEEEnvironment();
  
  // Create the transaction inside the TEE
  const tx = await neo.createInvocationTransaction({
    scriptHash: contractHash,
    operation: operation,
    args: args
  });
  
  // Sign the transaction with the protected private key
  const signedTx = await tee.crypto.signTransaction(tx, privateKey);
  
  // Send the transaction to the Neo N3 network
  const txHash = await neo.sendTransaction(signedTx);
  
  // Generate a proof of execution
  const executionProof = await tee.attestation.generateExecutionProof(txHash);
  
  return {
    txHash,
    executionProof
  };
}
```

### 2. Verifiable Off-Chain Computation

Off-chain computations can be performed within a TEE and verified on-chain:

```javascript
// Example of performing a verifiable off-chain computation
async function performVerifiableComputation(inputData, computationFunction) {
  // Verify the TEE environment through attestation
  await verifyTEEEnvironment();
  
  // Perform the computation inside the TEE
  const result = await computationFunction(inputData);
  
  // Generate a proof of computation
  const computationProof = await tee.attestation.generateComputationProof({
    input: inputData,
    result: result,
    function: computationFunction.toString()
  });
  
  return {
    result,
    computationProof
  };
}
```

### 3. Secure Key Management for Neo N3

Private keys for Neo N3 wallets can be securely managed within a TEE:

```javascript
// Example of secure key management for Neo N3
async function createSecureWallet() {
  // Verify the TEE environment through attestation
  await verifyTEEEnvironment();
  
  // Generate a new key pair inside the TEE
  const keyPair = await tee.crypto.generateKeyPair();
  
  // Create a Neo N3 wallet from the key pair
  const wallet = await neo.createWalletFromPrivateKey(keyPair.privateKey);
  
  // Store the private key securely within the TEE
  await tee.keyManagement.storeKey(wallet.address, keyPair.privateKey);
  
  return {
    address: wallet.address,
    publicKey: keyPair.publicKey
  };
}
```

### 4. TEE-Based Oracle for Neo N3

Oracles can leverage TEEs to provide verifiable data to Neo N3 smart contracts:

```javascript
// Example of a TEE-based oracle for Neo N3
async function provideOracleData(dataSource, dataQuery, contractHash) {
  // Verify the TEE environment through attestation
  await verifyTEEEnvironment();
  
  // Fetch the data from the source inside the TEE
  const data = await fetchDataInTEE(dataSource, dataQuery);
  
  // Create a transaction to update the oracle contract
  const tx = await neo.createInvocationTransaction({
    scriptHash: contractHash,
    operation: 'updateOracleData',
    args: [
      neo.sc.ContractParam.string(dataQuery),
      neo.sc.ContractParam.string(JSON.stringify(data))
    ]
  });
  
  // Sign the transaction with the oracle's private key
  const privateKey = await tee.keyManagement.retrieveKey('oracle');
  const signedTx = await tee.crypto.signTransaction(tx, privateKey);
  
  // Send the transaction to the Neo N3 network
  const txHash = await neo.sendTransaction(signedTx);
  
  // Generate a proof of the oracle data
  const oracleProof = await tee.attestation.generateOracleProof({
    source: dataSource,
    query: dataQuery,
    data: data,
    txHash: txHash
  });
  
  return {
    txHash,
    oracleProof
  };
}
```

### 5. Blockchain-Based Attestation Verification

Attestation reports can be verified on-chain to establish trust in TEE environments:

```javascript
// Example of blockchain-based attestation verification
async function verifyAttestationOnChain(attestationReport, contractHash) {
  // Create a transaction to verify the attestation on-chain
  const tx = await neo.createInvocationTransaction({
    scriptHash: contractHash,
    operation: 'verifyAttestation',
    args: [
      neo.sc.ContractParam.string(JSON.stringify(attestationReport))
    ]
  });
  
  // Sign and send the transaction
  const txHash = await neo.sendTransaction(tx);
  
  // Wait for the transaction to be confirmed
  const receipt = await neo.getTransactionReceipt(txHash);
  
  // Parse the verification result from the transaction events
  const verificationResult = parseVerificationResultFromEvents(receipt.events);
  
  return verificationResult;
}
```

## Neo N3 Smart Contract Integration

The example includes Neo N3 smart contracts that integrate with TEEs:

### TEE Verifier Contract

This contract verifies attestation reports and execution proofs:

```csharp
public static bool VerifyAttestationReport(string attestationReportJson)
{
    // Parse the attestation report
    Map<string, object> attestationReport = (Map<string, object>)StdLib.JsonDeserialize(attestationReportJson);
    
    // Verify the signature
    byte[] message = (byte[])attestationReport["message"];
    byte[] signature = (byte[])attestationReport["signature"];
    byte[] publicKey = (byte[])attestationReport["publicKey"];
    
    return Crypto.VerifySignature(message, signature, publicKey);
}

public static bool VerifyExecutionProof(string executionProofJson)
{
    // Parse the execution proof
    Map<string, object> executionProof = (Map<string, object>)StdLib.JsonDeserialize(executionProofJson);
    
    // Verify the proof
    // Implementation depends on the specific proof format
    
    return true; // Placeholder
}
```

### TEE Oracle Consumer Contract

This contract consumes data from TEE-based oracles:

```csharp
public static object GetOracleData(string dataQuery)
{
    // Get the oracle data
    StorageMap oracleDataMap = new(Storage.CurrentContext, PrefixOracleData);
    string dataJson = oracleDataMap.Get(dataQuery);
    
    if (dataJson == null)
        return null;
    
    // Parse and return the data
    return StdLib.JsonDeserialize(dataJson);
}

public static void UpdateOracleData(string dataQuery, string dataJson)
{
    // Verify the caller is the authorized oracle
    if (!Runtime.CheckWitness(OracleAddress))
        throw new Exception("Unauthorized");
    
    // Store the oracle data
    StorageMap oracleDataMap = new(Storage.CurrentContext, PrefixOracleData);
    oracleDataMap.Put(dataQuery, dataJson);
    
    // Emit an event
    OnOracleDataUpdated(dataQuery, dataJson);
}
```

### TEE Computation Verifier Contract

This contract verifies off-chain computations performed in a TEE:

```csharp
public static bool VerifyComputation(string inputJson, string resultJson, string computationProofJson)
{
    // Parse the inputs and proof
    object input = StdLib.JsonDeserialize(inputJson);
    object result = StdLib.JsonDeserialize(resultJson);
    Map<string, object> computationProof = (Map<string, object>)StdLib.JsonDeserialize(computationProofJson);
    
    // Verify the computation proof
    // Implementation depends on the specific proof format
    
    return true; // Placeholder
}
```

## Security Considerations

When using this example, be aware of the following security considerations:

1. **TEE Limitations**: While TEEs provide strong security guarantees, they are not immune to all attacks. Stay updated on the latest security advisories for your TEE technology.

2. **Key Management**: Even within a TEE, proper key management practices should be followed. Limit the use of long-term keys and prefer ephemeral keys when possible.

3. **Attestation Freshness**: Always check the timestamp of attestation reports to prevent replay attacks.

4. **Smart Contract Security**: The security of the overall system still depends on the security of your smart contracts. Follow best practices for Neo N3 smart contract development.

5. **Data Validation**: Always validate inputs to TEE functions, even if they are expected to come from a trusted source.

## Customization

You can customize this example by:

- Implementing additional TEE-based services for Neo N3
- Extending the smart contracts for specific use cases
- Adding support for other TEE technologies
- Implementing domain-specific attestation and verification mechanisms

## Additional Resources

- [Neo N3 TEE Services Documentation](../../docs/neo-n3/components/tee-services.md)
- [Neo N3 FaaS Platform Documentation](../../docs/neo-n3/README.md)
- [JavaScript Function Development Guide](../../docs/neo-n3/guides/function-development.md)
- [Neo N3 Smart Contract Development Guide](https://docs.neo.org/docs/en-us/develop/write/basics.html)
- [Intel SGX Documentation](https://software.intel.com/content/www/us/en/develop/topics/software-guard-extensions.html)
- [AMD SEV Documentation](https://developer.amd.com/sev/)
- [ARM TrustZone Documentation](https://developer.arm.com/ip-products/security-ip/trustzone)
