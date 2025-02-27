// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

// Neo N3 JavaScript API for r3e-faas platform

/**
 * Creates a Neo N3 RPC client for interacting with the blockchain
 * @param {Object} config - Configuration for the RPC client
 * @param {string} config.url - URL of the Neo N3 RPC endpoint
 * @returns {NeoClient} A Neo client instance
 */
export function createNeoClient(config) {
  const clientId = Deno.core.ops.op_neo_create_rpc_client(config);
  return new NeoClient(clientId);
}

/**
 * Neo N3 client for interacting with the blockchain
 */
class NeoClient {
  constructor(clientId) {
    this.clientId = clientId;
  }

  /**
   * Creates a new wallet or imports an existing one
   * @param {Object} options - Wallet creation options
   * @param {string} [options.privateKey] - Private key to import (optional)
   * @returns {NeoWallet} A Neo wallet instance
   */
  createWallet(options = {}) {
    const keyPair = Deno.core.ops.op_neo_create_key_pair({
      private_key: options.privateKey
    });
    return new NeoWallet(keyPair, this);
  }

  /**
   * Invokes a smart contract read-only method
   * @param {Object} params - Invocation parameters
   * @param {string} params.scriptHash - Script hash of the contract
   * @param {string} params.operation - Method name to invoke
   * @param {Array<string>} params.args - Arguments for the method
   * @returns {Object} Result of the invocation
   */
  invokeFunction(params) {
    return Deno.core.ops.op_neo_invoke_script(params);
  }
}

/**
 * Neo N3 wallet for managing keys and signing transactions
 */
class NeoWallet {
  constructor(keyPair, client) {
    this.address = keyPair.address;
    this.publicKey = keyPair.public_key;
    this.privateKey = keyPair.private_key;
    this.client = client;
  }

  /**
   * Creates a new transaction
   * @param {Object} params - Transaction parameters
   * @param {string} params.script - Script to execute
   * @param {Array<string>} [params.signers] - Transaction signers
   * @param {number} [params.systemFee=0] - System fee for the transaction
   * @param {number} [params.networkFee=0] - Network fee for the transaction
   * @returns {NeoTransaction} A Neo transaction instance
   */
  createTransaction(params) {
    const tx = Deno.core.ops.op_neo_create_transaction({
      script: params.script,
      signers: params.signers || [this.address],
      system_fee: params.systemFee || 0,
      network_fee: params.networkFee || 0
    });
    return new NeoTransaction(tx, this);
  }
}

/**
 * Neo N3 transaction for executing operations on the blockchain
 */
class NeoTransaction {
  constructor(tx, wallet) {
    this.hash = tx.hash;
    this.size = tx.size;
    this.version = tx.version;
    this.nonce = tx.nonce;
    this.sender = tx.sender;
    this.systemFee = tx.system_fee;
    this.networkFee = tx.network_fee;
    this.validUntilBlock = tx.valid_until_block;
    this.script = tx.script;
    this.wallet = wallet;
  }

  /**
   * Signs the transaction with the wallet's private key
   * @returns {NeoTransaction} This transaction instance
   */
  sign() {
    // In a real implementation, we would sign the transaction using the wallet's private key
    console.log(`Signing transaction ${this.hash} with wallet ${this.wallet.address}`);
    return this;
  }

  /**
   * Sends the transaction to the blockchain
   * @returns {Promise<string>} Transaction hash
   */
  async send() {
    // In a real implementation, we would send the transaction to the blockchain
    console.log(`Sending transaction ${this.hash}`);
    return this.hash;
  }
}

// Neo N3 utility functions

/**
 * Converts a string to a Neo script hash
 * @param {string} input - String to convert
 * @returns {string} Script hash
 */
export function stringToScriptHash(input) {
  // In a real implementation, we would convert the string to a script hash
  return `0x${Array.from(input).map(c => c.charCodeAt(0).toString(16)).join('')}`;
}

/**
 * Converts an address to a script hash
 * @param {string} address - Neo address
 * @returns {string} Script hash
 */
export function addressToScriptHash(address) {
  // In a real implementation, we would convert the address to a script hash
  return `0x${Array.from(address).slice(0, 20).map(c => c.charCodeAt(0).toString(16)).join('')}`;
}

// Export the Neo API
export const Neo = {
  createClient: createNeoClient,
  utils: {
    stringToScriptHash,
    addressToScriptHash
  }
};
