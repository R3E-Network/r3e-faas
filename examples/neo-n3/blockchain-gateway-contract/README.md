# Neo N3 Blockchain Gateway Contract

This directory contains a Neo N3 smart contract that serves as a gateway for blockchain interactions in the R3E FaaS platform. The contract provides dual functionality: it acts as an entry point for meta transactions and serves as an oracle price feed provider.

## Overview

The Blockchain Gateway Contract is a versatile smart contract that enables:

1. **Meta Transaction Processing**: Allows users to execute transactions on the blockchain without directly paying for gas, with support for both Neo N3 and Ethereum meta transactions.

2. **Oracle Price Feed**: Maintains up-to-date cryptocurrency price data on-chain, making it available to other smart contracts in an efficient, indexed format.

## Features

### Meta Transaction Functionality

- **Dual Blockchain Support**: Process both Neo N3 and Ethereum meta transactions
- **Relayer Authorization**: Only authorized relayers (from the FaaS platform) can submit transactions
- **Signature Verification**: Verify transaction signatures using appropriate cryptographic curves (secp256r1 for Neo, secp256k1 for Ethereum)
- **Replay Protection**: Prevent transaction replay attacks using nonce tracking
- **Expiration Mechanism**: Transactions expire after a specified deadline
- **Gas Bank Integration**: Ethereum transaction fees are paid by the Gas Bank account specified by the target contract

### Oracle Price Feed Functionality

- **Indexed Price Data**: Cryptocurrency prices are stored using space-efficient indices (e.g., 0 for NEO/USD, 1 for GAS/USD)
- **Multiple Price Pairs**: Support for various cryptocurrency/fiat pairs
- **Automatic Updates**: Price data can be updated via Oracle service integration
- **Timestamp Tracking**: Each price update includes a timestamp for freshness verification
- **Extensible Design**: New price pairs can be added dynamically

## Security Checks

The contract implements several security checks to ensure the integrity and authenticity of interactions:

1. **Relayer Authorization**: Verifies that meta transactions are submitted by authorized relayers from the FaaS platform
2. **Deadline Verification**: Ensures that transactions are not processed after their expiration deadline
3. **Nonce Tracking**: Prevents replay attacks by tracking used nonces for each sender
4. **Signature Verification**: Validates that transactions are properly signed by the original sender
5. **Transaction Type Validation**: Ensures that only supported transaction types are processed
6. **Oracle Authorization**: Only the Oracle contract or contract owner can update price data

## Gas Bank Integration

For Ethereum meta transactions, the contract integrates with the Gas Bank service to handle transaction fees:

1. Each target contract has an associated Gas Bank account
2. When an Ethereum meta transaction is executed, the Gas Bank account specified by the target contract pays for the transaction fees
3. The contract owner can set and update Gas Bank accounts for different target contracts

## Price Feed Indexing

The contract uses an efficient indexing mechanism for price data:

- **Index 0**: NEO/USD
- **Index 1**: GAS/USD
- **Index 2**: BTC/USD
- **Index 3**: ETH/USD

This approach saves storage space and gas costs when accessing price data. Other contracts can fetch price data using either the index or the price pair name.

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

### Managing Price Feed Data

The contract owner can manage price feed data:

```csharp
// Update price data for a specific asset
UpdatePriceData(byte priceIndex, BigInteger price)

// Add a new price pair to the index map
AddPricePair(string pricePair, byte priceIndex)

// Set the Oracle contract hash
SetOracleContractHash(UInt160 oracleContractHash)

// Request price data update from the Oracle
RequestPriceUpdate(string pricePair)
```

### Accessing Price Feed Data

Any contract can access the price feed data:

```csharp
// Get price data by index
GetPriceByIndex(byte priceIndex)

// Get price data by pair name
GetPriceByPair(string pricePair)

// Get the last update time for a price
GetLastUpdateTime(byte priceIndex)
```

## Integration with FaaS Platform

The R3E FaaS platform integrates with this contract to provide a seamless experience:

1. **Meta Transactions**: Users sign transactions using their private keys, and the FaaS platform submits them to the gateway contract
2. **Oracle Price Feed**: The FaaS platform's Oracle service updates price data in the contract, making it available to other contracts

## Events

The contract emits events to track activities:

```csharp
// Emitted when a meta transaction is executed
[DisplayName("MetaTxExecuted")]
public static event Action<UInt160, UInt256, string, string> OnMetaTxExecuted;

// Emitted when a meta transaction is rejected
[DisplayName("MetaTxRejected")]
public static event Action<UInt160, string, string> OnMetaTxRejected;

// Emitted when price data is updated
[DisplayName("PriceDataUpdated")]
public static event Action<byte, string, BigInteger, UInt256> OnPriceDataUpdated;
```

## Example Use Cases

### Meta Transaction Example

1. A user wants to execute a transaction on a target contract but doesn't want to pay for gas
2. The user signs the transaction data with their private key
3. The FaaS platform receives the signed transaction and submits it to the gateway contract
4. The gateway contract verifies the signature and other security checks
5. If all checks pass, the gateway contract forwards the transaction to the target contract
6. For Ethereum transactions, the Gas Bank account pays for the transaction fees

### Oracle Price Feed Example

1. A smart contract needs to know the current NEO/USD price
2. The contract calls `GetPriceByIndex(0)` or `GetPriceByPair("NEO/USD")` on the gateway contract
3. The gateway contract returns the latest price data
4. The contract can use this price data for its business logic

## License

Copyright @ 2023 - 2024, R3E Network
All Rights Reserved
