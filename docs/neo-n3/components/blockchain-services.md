# Neo N3 Blockchain Services

The R3E FaaS platform provides several essential blockchain services for Neo N3 that enable developers to build more user-friendly and efficient dApps. These services handle common blockchain interaction patterns and reduce friction for end users.

## Overview

The Neo N3 Blockchain Services include:

1. **Gas Bank Service** - A service for managing and optimizing gas fees for users
2. **Meta Transactions Service** - A service for enabling gasless transactions
3. **Abstract Account Service** - A service for providing flexible account management

These services are designed to work together to provide a comprehensive solution for Neo N3 dApp developers, addressing common challenges in blockchain application development.

## Gas Bank Service

The Gas Bank Service provides a way for dApp developers to manage gas fees for their users, reducing friction in the user experience.

### Key Features

- **Gas Fee Management**: Manage gas fees for users, allowing them to interact with dApps without worrying about gas costs.
- **Multiple Fee Models**: Support for various fee models, including fixed fees, percentage-based fees, dynamic fees, and free transactions.
- **Credit System**: Allow users to operate with a credit limit, enabling transactions even when they don't have sufficient GAS.
- **Transaction Batching**: Optimize gas usage by batching multiple transactions together.
- **Gas Price Estimation**: Provide accurate gas price estimates for transactions.

### Usage

The Gas Bank Service can be accessed through the JavaScript API:

```javascript
// Create a gas bank account
const account = r3e.NeoServices.GasBank.createAccount({
  address: "neo1abc123...",
  fee_model: "fixed",
  fee_value: 10,
  credit_limit: 1000
});

// Deposit GAS to an account
const deposit = r3e.NeoServices.GasBank.deposit({
  tx_hash: "0x1234...",
  address: "neo1abc123...",
  amount: 100
});

// Pay gas for a transaction
const payment = r3e.NeoServices.GasBank.payGas({
  tx_hash: "0x5678...",
  address: "neo1abc123...",
  amount: 50
});
```

### Configuration

The Gas Bank Service can be configured with the following options:

- **Default Fee Model**: Set the default fee model for all transactions.
- **Default Credit Limit**: Set the default credit limit for all accounts.
- **Gas Price Source**: Configure the source for gas price data (e.g., RPC node, oracle).
- **Transaction Confirmation Blocks**: Set the number of blocks required for transaction confirmation.

## Meta Transactions Service

The Meta Transactions Service enables gasless transactions for Neo N3, allowing users to interact with dApps without needing to hold GAS tokens.

### Key Features

- **Gasless Transactions**: Allow users to submit transactions without paying for gas.
- **Signature Verification**: Verify transaction signatures to ensure authenticity.
- **Relayer Network**: Distribute transaction relaying across a network of relayers.
- **Fee Models**: Support various fee models for relayers, including fixed fees, percentage-based fees, and dynamic fees.
- **Nonce Management**: Manage transaction nonces to prevent replay attacks.

### Usage

The Meta Transactions Service can be accessed through the JavaScript API:

```javascript
// Submit a meta transaction
const response = r3e.NeoServices.MetaTx.submit({
  tx_data: "0x...",
  sender: "neo1abc123...",
  signature: "0x...",
  nonce: 1,
  deadline: Math.floor(Date.now() / 1000) + 3600, // 1 hour from now
  fee_model: "fixed",
  fee_amount: 10
});

// Get the status of a meta transaction
const status = r3e.NeoServices.MetaTx.getStatus("request-id");

// Get the next nonce for a sender
const nonce = r3e.NeoServices.MetaTx.getNextNonce("neo1abc123...");
```

### Security Considerations

- **Signature Verification**: All meta transactions must be properly signed by the sender.
- **Deadline Enforcement**: Transactions with expired deadlines are rejected.
- **Nonce Validation**: Each transaction must have a unique nonce to prevent replay attacks.
- **Fee Protection**: Relayers are protected from excessive fees through fee models and limits.

## Abstract Account Service

The Abstract Account Service provides flexible account management for Neo N3, enabling advanced features like multi-signature accounts, account recovery, and programmable authorization.

### Key Features

- **Multi-Signature Support**: Create accounts that require multiple signatures for transactions.
- **Account Recovery**: Recover accounts through designated recovery addresses.
- **Programmable Authorization**: Define custom authorization logic for accounts.
- **Policy-Based Control**: Set policies for account operations, including time locks and thresholds.
- **Controller Management**: Add and remove controllers for an account.

### Usage

The Abstract Account Service can be accessed through the JavaScript API:

```javascript
// Create an abstract account
const account = r3e.NeoServices.AbstractAccount.createAccount({
  owner: "neo1abc123...",
  controllers: [
    { address: "neo1def456...", weight: 1 },
    { address: "neo1ghi789...", weight: 1 }
  ],
  recovery_addresses: ["neo1jkl012..."],
  policy_type: "multi_sig",
  required_signatures: 2,
  total_signatures: 2,
  signature: "0x..."
});

// Execute an operation on an abstract account
const response = r3e.NeoServices.AbstractAccount.executeOperation({
  account_address: "neo1abstract...",
  operation_type: "transfer",
  operation_data: JSON.stringify({
    asset: "GAS",
    to: "neo1recipient...",
    amount: "10.0"
  }),
  signatures: [
    { signer: "neo1def456...", signature: "0x...", signature_type: "standard" },
    { signer: "neo1ghi789...", signature: "0x...", signature_type: "standard" }
  ],
  nonce: 1,
  deadline: Math.floor(Date.now() / 1000) + 3600 // 1 hour from now
});
```

### Security Considerations

- **Signature Verification**: All operations must be properly signed by the required controllers.
- **Policy Enforcement**: Operations must comply with the account's policy.
- **Recovery Protection**: Account recovery requires proper authentication from recovery addresses.
- **Nonce Validation**: Each operation must have a unique nonce to prevent replay attacks.

## Integration with Neo N3 Smart Contracts

The Neo N3 Blockchain Services can be integrated with Neo N3 smart contracts to provide a seamless experience for users.

### Gas Bank Integration

```csharp
// GasBankConsumer.cs
using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Services;
using System;

public class GasBankConsumer : SmartContract
{
    [DisplayName("GasPaymentRequested")]
    public static event Action<UInt160, string, int> OnGasPaymentRequested;

    public static bool RequestGasPayment(UInt160 from, string operation, int amount)
    {
        // Emit event for the FaaS platform to handle
        OnGasPaymentRequested(from, operation, amount);
        return true;
    }
}
```

### Meta Transactions Integration

```csharp
// MetaTxConsumer.cs
using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Services;
using System;

public class MetaTxConsumer : SmartContract
{
    [DisplayName("MetaTxRequested")]
    public static event Action<UInt160, byte[], byte[]> OnMetaTxRequested;

    public static bool SubmitMetaTx(UInt160 sender, byte[] txData, byte[] signature)
    {
        // Verify the signature
        if (!VerifySignature(txData, signature, sender))
            return false;

        // Emit event for the FaaS platform to handle
        OnMetaTxRequested(sender, txData, signature);
        return true;
    }
}
```

### Abstract Account Integration

```csharp
// AbstractAccountConsumer.cs
using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Services;
using System;

public class AbstractAccountConsumer : SmartContract
{
    [DisplayName("AccountOperationRequested")]
    public static event Action<UInt160, string, byte[], byte[][]> OnAccountOperationRequested;

    public static bool ExecuteOperation(UInt160 account, string operationType, byte[] operationData, byte[][] signatures)
    {
        // Emit event for the FaaS platform to handle
        OnAccountOperationRequested(account, operationType, operationData, signatures);
        return true;
    }
}
```

## Example Use Cases

### Gasless NFT Minting

```javascript
// Function to mint an NFT without requiring the user to pay gas
async function mintNFT(userAddress, metadata) {
  // Create the transaction data
  const txData = createNFTMintTransaction(userAddress, metadata);
  
  // Get the next nonce for the user
  const nonce = await r3e.NeoServices.MetaTx.getNextNonce(userAddress);
  
  // Submit the meta transaction
  const response = await r3e.NeoServices.MetaTx.submit({
    tx_data: txData,
    sender: userAddress,
    signature: await getUserSignature(txData, userAddress),
    nonce: nonce,
    deadline: Math.floor(Date.now() / 1000) + 3600, // 1 hour from now
    fee_model: "fixed",
    fee_amount: 10
  });
  
  return response;
}
```

### Multi-Signature Wallet

```javascript
// Function to create a multi-signature wallet
async function createMultiSigWallet(ownerAddress, controllerAddresses) {
  // Create the controllers array
  const controllers = controllerAddresses.map(address => ({
    address: address,
    weight: 1,
    controller_type: "standard",
    added_at: Math.floor(Date.now() / 1000),
    status: "active"
  }));
  
  // Create the abstract account
  const account = await r3e.NeoServices.AbstractAccount.createAccount({
    owner: ownerAddress,
    controllers: controllers,
    recovery_addresses: [ownerAddress],
    policy_type: "multi_sig",
    required_signatures: Math.ceil(controllers.length / 2), // Require majority
    total_signatures: controllers.length,
    signature: await getOwnerSignature(ownerAddress)
  });
  
  return account;
}
```

### Subscription Service

```javascript
// Function to handle a subscription payment
async function processSubscription(userAddress, subscriptionId) {
  // Check if the user has a gas bank account
  const account = await r3e.NeoServices.GasBank.getAccount(userAddress);
  
  if (!account) {
    // Create a gas bank account for the user
    await r3e.NeoServices.GasBank.createAccount({
      address: userAddress,
      fee_model: "percentage",
      fee_value: 2.5,
      credit_limit: 1000
    });
  }
  
  // Create the subscription payment transaction
  const txHash = await createSubscriptionPaymentTx(userAddress, subscriptionId);
  
  // Pay gas for the transaction
  const payment = await r3e.NeoServices.GasBank.payGas({
    tx_hash: txHash,
    address: userAddress,
    amount: 50
  });
  
  return payment;
}
```

## Conclusion

The Neo N3 Blockchain Services provided by the R3E FaaS platform offer a comprehensive solution for dApp developers to improve the user experience and reduce friction in blockchain interactions. By leveraging these services, developers can create more user-friendly and efficient dApps on the Neo N3 blockchain.
