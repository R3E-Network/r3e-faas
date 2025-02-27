# Neo N3 Secure Key Management Example

This example demonstrates how to implement secure key management using Trusted Execution Environments (TEEs) in the Neo N3 FaaS platform. Secure key management is essential for protecting sensitive cryptographic keys and ensuring the security of blockchain applications.

## Overview

The secure key management example shows how to:

1. Generate and store cryptographic keys securely within a TEE
2. Use keys for signing transactions without exposing them outside the TEE
3. Implement key rotation and lifecycle management
4. Enforce access control policies for key usage
5. Securely share keys between authorized parties

## Files

- `function.js`: The JavaScript function that implements the secure key management service
- `config.yaml`: Configuration file for the secure key management service
- `register.js`: Script to register the function with the FaaS platform
- `smart-contract/`: Directory containing a sample Neo N3 smart contract that interacts with the secure key management service
- `lib/`: Directory containing key management libraries and utilities

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

3. (Optional) Deploy the sample smart contract to interact with the secure key management service

## How It Works

The secure key management service leverages Trusted Execution Environments (TEEs) to protect cryptographic keys throughout their lifecycle. TEEs provide hardware-based isolation that ensures keys are never exposed in plaintext outside the secure environment.

### Key Generation

Keys are generated within the TEE using a secure random number generator, ensuring that even the function developer cannot access the raw key material:

```javascript
// Example key generation within TEE
const keyPair = await tee.crypto.generateKeyPair({
  algorithm: 'secp256r1',
  extractable: false,
  usages: ['sign', 'verify']
});

// The private key never leaves the TEE
const keyId = await tee.storage.storeKey(keyPair.privateKey);
```

### Key Storage

Keys are stored in encrypted form within the TEE's secure storage, with encryption keys derived from the TEE's hardware-bound keys:

```javascript
// Example key storage within TEE
const keyMetadata = {
  id: keyId,
  algorithm: 'secp256r1',
  created: Date.now(),
  owner: userId,
  policies: {
    rotation: { period: 90 * 24 * 60 * 60 * 1000 }, // 90 days
    usage: { maxOperations: 10000 }
  }
};

await tee.storage.storeMetadata(keyId, keyMetadata);
```

### Key Usage

Keys can be used for cryptographic operations without ever exposing them outside the TEE:

```javascript
// Example key usage within TEE
const message = 'Message to sign';
const messageBuffer = new TextEncoder().encode(message);

// The signing operation happens entirely within the TEE
const signature = await tee.crypto.sign(
  { name: 'ECDSA', hash: 'SHA-256' },
  keyId,
  messageBuffer
);
```

### Key Rotation

Keys are automatically rotated based on policy, with the rotation happening securely within the TEE:

```javascript
// Example key rotation within TEE
const oldKeyId = keyId;
const newKeyPair = await tee.crypto.generateKeyPair({
  algorithm: 'secp256r1',
  extractable: false,
  usages: ['sign', 'verify']
});

const newKeyId = await tee.storage.storeKey(newKeyPair.privateKey);

// Update references to use the new key
await updateKeyReferences(oldKeyId, newKeyId);

// Securely delete the old key
await tee.storage.deleteKey(oldKeyId);
```

### Access Control

Access to keys is controlled by policies enforced by the TEE:

```javascript
// Example access control check
async function checkAccess(userId, keyId, operation) {
  const keyMetadata = await tee.storage.getMetadata(keyId);
  
  // Check if user is authorized
  if (keyMetadata.owner !== userId && !keyMetadata.authorizedUsers.includes(userId)) {
    throw new Error('Unauthorized access to key');
  }
  
  // Check if operation is allowed
  if (!keyMetadata.policies.operations.includes(operation)) {
    throw new Error(`Operation ${operation} not allowed for this key`);
  }
  
  // Check usage limits
  if (keyMetadata.usageCount >= keyMetadata.policies.usage.maxOperations) {
    throw new Error('Key usage limit exceeded');
  }
  
  // Update usage count
  keyMetadata.usageCount += 1;
  await tee.storage.storeMetadata(keyId, keyMetadata);
  
  return true;
}
```

### Remote Attestation

The service uses remote attestation to verify the integrity of the TEE before trusting it with sensitive keys:

```javascript
// Example remote attestation
async function verifyTEE() {
  // Generate attestation report
  const report = await tee.attestation.generateReport();
  
  // Send to verification service
  const verificationResult = await verifyAttestationReport(report);
  
  if (!verificationResult.verified) {
    throw new Error('TEE attestation failed: ' + verificationResult.reason);
  }
  
  return true;
}
```

## Security Considerations

When using this example, be aware of the following security considerations:

1. **TEE Limitations**: While TEEs provide strong security guarantees, they are not immune to all attacks. Stay updated on the latest security advisories for your TEE technology.

2. **Key Backup**: Consider implementing secure key backup mechanisms to prevent key loss while maintaining security.

3. **Access Control**: Implement strong authentication and authorization before allowing access to key operations.

4. **Attestation**: Always verify the TEE environment through remote attestation before trusting it with sensitive keys.

5. **Side-Channel Attacks**: Be aware that some sophisticated side-channel attacks might still be possible against TEEs.

## Customization

You can customize this example by:

- Implementing additional key types and algorithms
- Creating custom key usage policies
- Integrating with hardware security modules (HSMs) for additional security
- Implementing multi-party computation for distributed key management
- Adding support for threshold signatures

## Additional Resources

- [Neo N3 TEE Services Documentation](../../docs/neo-n3/components/tee-services.md)
- [Neo N3 FaaS Platform Documentation](../../docs/neo-n3/README.md)
- [JavaScript Function Development Guide](../../docs/neo-n3/guides/function-development.md)
- [Neo N3 Smart Contract Development Guide](https://docs.neo.org/docs/en-us/develop/write/basics.html)
- [Intel SGX Documentation](https://software.intel.com/content/www/us/en/develop/topics/software-guard-extensions.html)
- [ARM TrustZone Documentation](https://developer.arm.com/ip-products/security-ip/trustzone)
