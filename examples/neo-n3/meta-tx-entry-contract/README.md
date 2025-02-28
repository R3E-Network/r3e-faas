# Neo N3 Meta Transaction Entry Contract

This directory contains a Neo N3 smart contract that serves as an entry point for meta transactions in the R3E FaaS platform. The contract verifies that transactions are from the FaaS platform, implements necessary security checks, and supports both Neo N3 and Ethereum meta transactions.

## Overview

Meta transactions allow users to execute transactions on the blockchain without directly paying for gas. Instead, a relayer (in this case, the R3E FaaS platform) submits the transaction on behalf of the user. This enables a better user experience and opens up new possibilities for decentralized applications.

The Meta Transaction Entry Contract serves as a secure gateway for these transactions, ensuring that:

1. Only authorized relayers can submit transactions
2. Transactions are properly signed by the original sender
3. Transactions are not replayed or expired
4. Gas fees are properly handled for both Neo N3 and Ethereum transactions

## Features

- **Dual Blockchain Support**: Process both Neo N3 and Ethereum meta transactions
- **Relayer Authorization**: Only authorized relayers can submit transactions
- **Signature Verification**: Verify transaction signatures using appropriate cryptographic curves (secp256r1 for Neo, secp256k1 for Ethereum)
- **Replay Protection**: Prevent transaction replay attacks using nonce tracking
- **Expiration Mechanism**: Transactions expire after a specified deadline
- **Gas Bank Integration**: Ethereum transaction fees are paid by the Gas Bank account specified by the target contract

## Security Checks

The contract implements several security checks to ensure the integrity and authenticity of meta transactions:

1. **Relayer Authorization**: Verifies that the transaction is submitted by an authorized relayer from the FaaS platform
2. **Deadline Verification**: Ensures that transactions are not processed after their expiration deadline
3. **Nonce Tracking**: Prevents replay attacks by tracking used nonces for each sender
4. **Signature Verification**: Validates that the transaction is properly signed by the original sender
5. **Transaction Type Validation**: Ensures that only supported transaction types (Neo N3 or Ethereum) are processed

## Gas Bank Integration

For Ethereum meta transactions, the contract integrates with the Gas Bank service to handle transaction fees:

1. Each target contract has an associated Gas Bank account
2. When an Ethereum meta transaction is executed, the Gas Bank account specified by the target contract pays for the transaction fees
3. The contract owner can set and update Gas Bank accounts for different target contracts

## Usage Guidelines

### Deploying the Contract

1. Deploy the contract to the Neo N3 blockchain
2. The deploying address becomes the contract owner

### Managing Relayers

Only the contract owner can manage authorized relayers:

```csharp
// Add a relayer
AddRelayer(UInt160 relayerAddress)

// Remove a relayer
RemoveRelayer(UInt160 relayerAddress)

// Check if an address is an authorized relayer
IsRelayer(UInt160 relayerAddress)
```

### Managing Gas Bank Accounts

Only the contract owner can manage Gas Bank accounts:

```csharp
// Set a Gas Bank account for a contract
SetGasBankAccount(UInt160 contractHash, UInt160 gasBankAccount)

// Get the Gas Bank account for a contract
GetGasBankAccount(UInt160 contractHash)
```

### Executing Meta Transactions

Authorized relayers can execute meta transactions:

```csharp
// Execute a meta transaction
ExecuteMetaTx(UInt160 sender, UInt160 target, byte[] txData, byte[] signature, BigInteger nonce, BigInteger deadline, string txType)
```

Parameters:
- `sender`: Original sender address
- `target`: Target contract address
- `txData`: Transaction data
- `signature`: Transaction signature
- `nonce`: Transaction nonce
- `deadline`: Transaction deadline (timestamp)
- `txType`: Transaction type ("neo" or "ethereum")

## Integration with FaaS Platform

The R3E FaaS platform integrates with this contract to provide a seamless meta transaction experience:

1. Users sign transactions using their private keys
2. The FaaS platform verifies the signature and submits the transaction to the entry contract
3. The entry contract verifies the transaction and forwards it to the target contract
4. For Ethereum transactions, the Gas Bank account pays for the transaction fees

## Events

The contract emits events to track meta transaction execution:

```csharp
// Emitted when a meta transaction is executed
[DisplayName("MetaTxExecuted")]
public static event Action<UInt160, UInt256, string, string> OnMetaTxExecuted;

// Emitted when a meta transaction is rejected
[DisplayName("MetaTxRejected")]
public static event Action<UInt160, string, string> OnMetaTxRejected;
```

## Example

Here's an example of how to use the Meta Transaction Entry Contract:

1. A user wants to execute a transaction on a target contract but doesn't want to pay for gas
2. The user signs the transaction data with their private key
3. The FaaS platform receives the signed transaction and submits it to the entry contract
4. The entry contract verifies the signature and other security checks
5. If all checks pass, the entry contract forwards the transaction to the target contract
6. For Ethereum transactions, the Gas Bank account pays for the transaction fees
7. The target contract executes the transaction as if it was directly called by the original sender

## License

Copyright @ 2023 - 2024, R3E Network
All Rights Reserved
