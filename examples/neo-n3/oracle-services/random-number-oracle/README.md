# Neo N3 Random Number Oracle Example

This example demonstrates how to use the Neo N3 FaaS platform's oracle services to generate secure random numbers for blockchain applications.

## Overview

The random number oracle example shows how to:

1. Create a function that generates cryptographically secure random numbers
2. Provide these random numbers to Neo N3 smart contracts in a verifiable way
3. Configure the oracle service with different entropy sources and generation methods
4. Implement verification mechanisms to prove the randomness is fair and unbiased
5. Use the random numbers in various blockchain applications

## Files

- `function.js`: The JavaScript function that implements the random number oracle
- `config.yaml`: Configuration file for the random number oracle
- `register.js`: Script to register the function with the FaaS platform
- `smart-contract/`: Directory containing a sample Neo N3 smart contract that uses the random number oracle

## Prerequisites

- Neo N3 FaaS platform installed and running
- Access to a Neo N3 node (testnet or mainnet)
- Node.js installed for running the registration script
- Neo N3 smart contract development environment (optional, for testing the smart contract)

## Setup

1. Configure your Neo N3 node connection in `config.yaml`
2. Register the function using the registration script:

```bash
node register.js
```

3. (Optional) Deploy the sample smart contract to interact with the random number oracle

## How It Works

The random number oracle function generates cryptographically secure random numbers using multiple entropy sources and provides them to Neo N3 smart contracts. The function can be triggered on a schedule or on-demand when a smart contract requests a random number.

### Entropy Sources

The example demonstrates using multiple entropy sources for randomness:

- Hardware random number generators (if available)
- Cryptographic PRNGs
- Blockchain data (block hash, transaction hashes)
- External entropy sources (e.g., NIST Randomness Beacon)
- Time-based entropy

### Generation Methods

The example implements several random number generation methods:

- Direct generation using cryptographic libraries
- Verifiable random functions (VRFs)
- Commit-reveal schemes
- Multi-party computation for distributed randomness

### Verification Mechanisms

The example includes several verification mechanisms:

- Cryptographic proofs of randomness
- Entropy source validation
- Timestamp verification
- Digital signatures for integrity
- Audit logs for transparency

## Smart Contract Integration

The sample smart contract demonstrates how to:

1. Request a random number from the oracle
2. Verify the authenticity of the random number
3. Use the random number in contract logic (e.g., for a lottery or game)

## Customization

You can customize this example by:

- Adding additional entropy sources
- Implementing different random number generation methods
- Modifying the range and distribution of random numbers
- Extending the smart contract to use the random numbers in different ways

## Additional Resources

- [Neo N3 Oracle Services Documentation](../../docs/neo-n3/components/oracle-services.md)
- [Neo N3 FaaS Platform Documentation](../../docs/neo-n3/README.md)
- [JavaScript Function Development Guide](../../docs/neo-n3/guides/function-development.md)
- [Neo N3 Smart Contract Development Guide](https://docs.neo.org/docs/en-us/develop/write/basics.html)
