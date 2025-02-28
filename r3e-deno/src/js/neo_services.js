// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

// Neo N3 Blockchain Services JavaScript API for r3e-faas platform

/**
 * Gas Bank Service
 * Provides gas management services for Neo N3 blockchain
 */
export class GasBankService {
  /**
   * Creates a new gas bank account
   * @param {Object} request - Account creation request
   * @param {string} request.address - Account address
   * @param {string} request.fee_model - Fee model (fixed, percentage, dynamic, free)
   * @param {number} request.fee_value - Fee value (amount for fixed, percentage for percentage)
   * @param {number} request.credit_limit - Credit limit for the account
   * @returns {Object} Created gas bank account
   */
  static createAccount(request) {
    const result = Deno.core.ops.op_neo_gas_bank_create_account(request);
    return JSON.parse(result);
  }

  /**
   * Gets a gas bank account
   * @param {string} address - Account address
   * @returns {Object} Gas bank account
   */
  static getAccount(address) {
    const result = Deno.core.ops.op_neo_gas_bank_get_account(address);
    return JSON.parse(result);
  }

  /**
   * Deposits gas to an account
   * @param {Object} request - Deposit request
   * @param {string} request.tx_hash - Transaction hash
   * @param {string} request.address - Account address
   * @param {number} request.amount - Amount to deposit
   * @returns {Object} Deposit record
   */
  static deposit(request) {
    const result = Deno.core.ops.op_neo_gas_bank_deposit(request);
    return JSON.parse(result);
  }

  /**
   * Withdraws gas from an account
   * @param {Object} request - Withdrawal request
   * @param {string} request.address - Account address
   * @param {number} request.amount - Amount to withdraw
   * @returns {Object} Withdrawal record
   */
  static withdraw(request) {
    const result = Deno.core.ops.op_neo_gas_bank_withdraw(request);
    return JSON.parse(result);
  }

  /**
   * Pays gas for a transaction
   * @param {Object} request - Gas payment request
   * @param {string} request.tx_hash - Transaction hash
   * @param {string} request.address - Account address
   * @param {number} request.amount - Amount to pay
   * @returns {Object} Transaction record
   */
  static payGas(request) {
    const result = Deno.core.ops.op_neo_gas_bank_pay_gas(request);
    return JSON.parse(result);
  }

  /**
   * Gets the current gas price
   * @returns {number} Gas price
   */
  static getGasPrice() {
    return Deno.core.ops.op_neo_gas_bank_get_gas_price();
  }
}

/**
 * Meta Transaction Service
 * Provides gasless transaction services for Neo N3 and Ethereum blockchains
 */
export class MetaTxService {
  /**
   * Blockchain types for meta transactions
   */
  static BlockchainType = {
    /**
     * Neo N3 blockchain
     */
    NEO_N3: "neo",
    
    /**
     * Ethereum blockchain
     */
    ETHEREUM: "ethereum"
  };
  
  /**
   * Signature curve types
   */
  static SignatureCurve = {
    /**
     * secp256r1 curve (used by Neo)
     */
    SECP256R1: "secp256r1",
    
    /**
     * secp256k1 curve (used by Ethereum)
     */
    SECP256K1: "secp256k1"
  };

  /**
   * Submits a meta transaction
   * @param {Object} request - Meta transaction request
   * @param {string} request.tx_data - Transaction data
   * @param {string} request.sender - Sender address
   * @param {string} request.signature - Transaction signature
   * @param {number} request.nonce - Transaction nonce
   * @param {number} request.deadline - Transaction deadline (timestamp)
   * @param {string} request.fee_model - Fee model (fixed, percentage, dynamic, free)
   * @param {number} request.fee_amount - Fee amount
   * @param {string} [request.blockchain_type] - Blockchain type (neo or ethereum, defaults to neo)
   * @param {string} [request.signature_curve] - Signature curve (secp256r1 or secp256k1, defaults to secp256r1)
   * @param {string} [request.target_contract] - Target contract address (required for Ethereum transactions)
   * @returns {Object} Meta transaction response
   */
  static submit(request) {
    const result = Deno.core.ops.op_neo_meta_tx_submit(request);
    return JSON.parse(result);
  }

  /**
   * Submits a Neo N3 meta transaction
   * @param {Object} request - Meta transaction request
   * @param {string} request.tx_data - Transaction data
   * @param {string} request.sender - Sender address
   * @param {string} request.signature - Transaction signature
   * @param {number} request.nonce - Transaction nonce
   * @param {number} request.deadline - Transaction deadline (timestamp)
   * @param {string} request.fee_model - Fee model (fixed, percentage, dynamic, free)
   * @param {number} request.fee_amount - Fee amount
   * @returns {Object} Meta transaction response
   */
  static submitNeoTx(request) {
    const neoRequest = {
      ...request,
      blockchain_type: MetaTxService.BlockchainType.NEO_N3,
      signature_curve: MetaTxService.SignatureCurve.SECP256R1
    };
    return MetaTxService.submit(neoRequest);
  }

  /**
   * Creates EIP-712 typed data for a meta transaction
   * @param {Object} request - Meta transaction request
   * @param {string} request.tx_data - Transaction data
   * @param {string} request.sender - Sender address
   * @param {number} request.nonce - Transaction nonce
   * @param {number} request.deadline - Transaction deadline (timestamp)
   * @param {string} request.fee_model - Fee model (fixed, percentage, dynamic, free)
   * @param {number} request.fee_amount - Fee amount
   * @param {string} request.target_contract - Target contract address
   * @param {number} [request.chain_id=1] - Chain ID (defaults to 1 for Ethereum mainnet)
   * @returns {Object} EIP-712 typed data
   */
  static createEIP712TypedData(request) {
    if (!request.target_contract) {
      throw new Error("Target contract is required for Ethereum transactions");
    }
    
    // Create domain separator
    const domain = {
      name: "R3E FaaS Meta Transaction",
      version: "1",
      chainId: request.chain_id || 1,
      verifyingContract: request.target_contract
    };
    
    // Create message
    const message = {
      from: request.sender,
      to: request.target_contract,
      data: request.tx_data,
      nonce: request.nonce,
      deadline: request.deadline,
      fee_model: request.fee_model,
      fee_amount: request.fee_amount
    };
    
    // Create types
    const types = {
      EIP712Domain: [
        { name: "name", type: "string" },
        { name: "version", type: "string" },
        { name: "chainId", type: "uint256" },
        { name: "verifyingContract", type: "address" }
      ],
      MetaTransaction: [
        { name: "from", type: "address" },
        { name: "to", type: "address" },
        { name: "data", type: "bytes" },
        { name: "nonce", type: "uint256" },
        { name: "deadline", type: "uint256" },
        { name: "fee_model", type: "string" },
        { name: "fee_amount", type: "uint256" }
      ]
    };
    
    return {
      types,
      domain,
      primaryType: "MetaTransaction",
      message
    };
  }
  
  /**
   * Submits an Ethereum meta transaction
   * @param {Object} request - Meta transaction request
   * @param {string} request.tx_data - Transaction data
   * @param {string} request.sender - Sender address
   * @param {string} request.signature - Transaction signature
   * @param {number} request.nonce - Transaction nonce
   * @param {number} request.deadline - Transaction deadline (timestamp)
   * @param {string} request.fee_model - Fee model (fixed, percentage, dynamic, free)
   * @param {number} request.fee_amount - Fee amount
   * @param {string} request.target_contract - Target contract address
   * @param {number} [request.chain_id] - Chain ID (defaults to 1 for Ethereum mainnet)
   * @returns {Object} Meta transaction response
   */
  static submitEthereumTx(request) {
    if (!request.target_contract) {
      throw new Error("Target contract is required for Ethereum transactions");
    }
    
    const ethRequest = {
      ...request,
      blockchain_type: MetaTxService.BlockchainType.ETHEREUM,
      signature_curve: MetaTxService.SignatureCurve.SECP256K1
    };
    return MetaTxService.submit(ethRequest);
  }
  
  /**
   * Signs an EIP-712 typed data with a private key (client-side only)
   * @param {Object} typedData - EIP-712 typed data
   * @param {string} privateKey - Private key (hex string)
   * @returns {string} Signature (hex string)
   */
  static signEIP712TypedData(typedData, privateKey) {
    // This method should be implemented client-side using ethers.js or web3.js
    // It's included here for documentation purposes only
    throw new Error("This method must be implemented client-side using ethers.js or web3.js");
  }

  /**
   * Gets the status of a meta transaction
   * @param {string} requestId - Request ID
   * @returns {string} Transaction status
   */
  static getStatus(requestId) {
    return Deno.core.ops.op_neo_meta_tx_get_status(requestId);
  }

  /**
   * Gets a meta transaction
   * @param {string} requestId - Request ID
   * @returns {Object} Meta transaction record
   */
  static getTransaction(requestId) {
    const result = Deno.core.ops.op_neo_meta_tx_get_transaction(requestId);
    return JSON.parse(result);
  }

  /**
   * Gets the next nonce for a sender
   * @param {string} sender - Sender address
   * @returns {number} Next nonce
   */
  static getNextNonce(sender) {
    return Deno.core.ops.op_neo_meta_tx_get_next_nonce(sender);
  }
}

/**
 * Abstract Account Service
 * Provides programmable account services for Neo N3 blockchain
 */
export class AbstractAccountService {
  /**
   * Creates a new abstract account
   * @param {Object} request - Account creation request
   * @param {string} request.owner - Account owner
   * @param {string[]} request.controllers - Account controllers
   * @param {string[]} request.recovery_addresses - Recovery addresses
   * @param {string} request.policy_type - Policy type (single_sig, multi_sig, threshold_sig, time_locked, custom)
   * @param {number} request.required_signatures - Required signatures
   * @param {number} request.total_signatures - Total signatures
   * @param {string} request.signature - Creation signature
   * @returns {Object} Created abstract account
   */
  static createAccount(request) {
    const result = Deno.core.ops.op_neo_abstract_account_create(request);
    return JSON.parse(result);
  }

  /**
   * Gets an abstract account
   * @param {string} address - Account address
   * @returns {Object} Abstract account
   */
  static getAccount(address) {
    const result = Deno.core.ops.op_neo_abstract_account_get(address);
    return JSON.parse(result);
  }

  /**
   * Executes an operation on an abstract account
   * @param {Object} request - Operation request
   * @param {string} request.account_address - Account address
   * @param {string} request.operation_type - Operation type (transfer, invoke, add_controller, remove_controller, update_policy, recover, custom)
   * @param {string} request.operation_data - Operation data (JSON string)
   * @param {string[]} request.signatures - Operation signatures
   * @param {number} request.nonce - Operation nonce
   * @param {number} request.deadline - Operation deadline (timestamp)
   * @returns {Object} Operation response
   */
  static executeOperation(request) {
    const result = Deno.core.ops.op_neo_abstract_account_execute_operation(request);
    return JSON.parse(result);
  }

  /**
   * Gets the status of an operation
   * @param {string} requestId - Request ID
   * @returns {string} Operation status
   */
  static getOperationStatus(requestId) {
    return Deno.core.ops.op_neo_abstract_account_get_operation_status(requestId);
  }

  /**
   * Gets the next nonce for an account
   * @param {string} address - Account address
   * @returns {number} Next nonce
   */
  static getNextNonce(address) {
    return Deno.core.ops.op_neo_abstract_account_get_next_nonce(address);
  }
}
