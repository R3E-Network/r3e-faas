// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

// Oracle service JavaScript API

/**
 * Oracle service for accessing external data
 */
class Oracle {
  /**
   * Submit a generic oracle request
   * @param {string} requestType - Type of oracle request (price, random, weather, sports, custom)
   * @param {Object} data - Request data
   * @param {string} requesterId - Requester ID
   * @param {string} [callbackUrl] - Optional callback URL
   * @returns {Promise<string>} Request ID
   */
  static async submitRequest(requestType, data, requesterId, callbackUrl = null) {
    const config = {
      request_type: requestType,
      data,
      callback_url: callbackUrl,
      requester_id: requesterId,
    };
    
    const result = Deno.core.ops.op_oracle_submit_request(config);
    return result.request_id;
  }
  
  /**
   * Get the status of an oracle request
   * @param {string} requestId - Request ID
   * @returns {Promise<string>} Request status (pending, processing, completed, failed)
   */
  static async getRequestStatus(requestId) {
    const result = Deno.core.ops.op_oracle_get_request_status(requestId);
    return result.status;
  }
  
  /**
   * Get the response for a completed oracle request
   * @param {string} requestId - Request ID
   * @returns {Promise<Object>} Oracle response
   */
  static async getResponse(requestId) {
    return Deno.core.ops.op_oracle_get_response(requestId);
  }
  
  /**
   * Cancel an oracle request
   * @param {string} requestId - Request ID
   * @returns {Promise<boolean>} Success flag
   */
  static async cancelRequest(requestId) {
    const result = Deno.core.ops.op_oracle_cancel_request(requestId);
    return result.success;
  }
  
  /**
   * Get price data for a cryptocurrency
   * @param {string} symbol - Asset symbol (e.g., "NEO", "GAS")
   * @param {string} [currency="USD"] - Currency to convert to
   * @param {string[]} [sources=[]] - Preferred price sources
   * @param {string} requesterId - Requester ID
   * @returns {Promise<string>} Request ID
   */
  static async getPrice(symbol, currency = "USD", sources = [], requesterId) {
    const config = {
      symbol,
      currency,
      sources,
      requester_id: requesterId,
    };
    
    const result = Deno.core.ops.op_oracle_get_price(config);
    return result.request_id;
  }
  
  /**
   * Get price data by index
   * @param {number} index - Price index (0 for NEO/USD, 1 for GAS/USD, 2 for BTC/USD, 3 for ETH/USD)
   * @param {string[]} [sources=[]] - Preferred price sources
   * @param {string} requesterId - Requester ID
   * @returns {Promise<string>} Request ID
   */
  static async getPriceByIndex(index, sources = [], requesterId) {
    const config = {
      index,
      sources,
      requester_id: requesterId,
    };
    
    const result = Deno.core.ops.op_oracle_get_price_by_index(config);
    return result.request_id;
  }
  
  /**
   * Update price data on the blockchain
   * @param {string} symbol - Asset symbol (e.g., "NEO", "GAS")
   * @param {string} requesterId - Requester ID
   * @returns {Promise<string>} Transaction hash
   */
  static async updatePriceOnBlockchain(symbol, requesterId) {
    const config = {
      symbol,
      requester_id: requesterId,
    };
    
    const result = Deno.core.ops.op_oracle_update_price_on_blockchain(config);
    return result.tx_hash;
  }
  
  /**
   * Get random numbers
   * @param {Object} options - Random number options
   * @param {number} [options.min=0] - Minimum value (inclusive)
   * @param {number} [options.max=Number.MAX_SAFE_INTEGER] - Maximum value (inclusive)
   * @param {number} [options.count=1] - Number of random values to generate
   * @param {string} [options.method="secure"] - Random number generation method (secure, blockchain, vrf)
   * @param {string} [options.seed] - Optional seed for deterministic generation
   * @param {string} requesterId - Requester ID
   * @returns {Promise<string>} Request ID
   */
  static async getRandom(options, requesterId) {
    const config = {
      min: options.min,
      max: options.max,
      count: options.count,
      method: options.method,
      seed: options.seed,
      requester_id: requesterId,
    };
    
    const result = Deno.core.ops.op_oracle_get_random(config);
    return result.request_id;
  }
  
  /**
   * Wait for an oracle request to complete
   * @param {string} requestId - Request ID
   * @param {number} [timeout=30000] - Timeout in milliseconds
   * @param {number} [interval=1000] - Polling interval in milliseconds
   * @returns {Promise<Object>} Oracle response
   */
  static async waitForResponse(requestId, timeout = 30000, interval = 1000) {
    const startTime = Date.now();
    
    while (Date.now() - startTime < timeout) {
      const status = await Oracle.getRequestStatus(requestId);
      
      if (status === "completed") {
        return await Oracle.getResponse(requestId);
      } else if (status === "failed") {
        const response = await Oracle.getResponse(requestId);
        throw new Error(`Oracle request failed: ${response.error || "Unknown error"}`);
      }
      
      // Wait for the next polling interval
      await new Promise(resolve => setTimeout(resolve, interval));
    }
    
    throw new Error(`Oracle request timed out after ${timeout}ms`);
  }
  
  /**
   * Get Neo price data
   * @param {string} [currency="USD"] - Currency to convert to
   * @param {string[]} [sources=[]] - Preferred price sources
   * @param {string} requesterId - Requester ID
   * @returns {Promise<Object>} Price data
   */
  static async getNeoPrice(currency = "USD", sources = [], requesterId) {
    const requestId = await Oracle.getPrice("NEO", currency, sources, requesterId);
    return await Oracle.waitForResponse(requestId);
  }
  
  /**
   * Get GAS price data
   * @param {string} [currency="USD"] - Currency to convert to
   * @param {string[]} [sources=[]] - Preferred price sources
   * @param {string} requesterId - Requester ID
   * @returns {Promise<Object>} Price data
   */
  static async getGasPrice(currency = "USD", sources = [], requesterId) {
    const requestId = await Oracle.getPrice("GAS", currency, sources, requesterId);
    return await Oracle.waitForResponse(requestId);
  }
  
  /**
   * Get price data by index from the blockchain
   * @param {number} index - Price index (0 for NEO/USD, 1 for GAS/USD, 2 for BTC/USD, 3 for ETH/USD)
   * @param {string} requesterId - Requester ID
   * @returns {Promise<Object>} Price data
   */
  static async getPriceFromBlockchain(index, requesterId) {
    const config = {
      index,
      requester_id: requesterId,
    };
    
    return Deno.core.ops.op_oracle_get_price_from_blockchain(config);
  }
  
  /**
   * Update price data on the blockchain
   * @param {string} symbol - Asset symbol (e.g., "NEO", "GAS")
   * @param {string} requesterId - Requester ID
   * @returns {Promise<Object>} Transaction result
   */
  static async updatePriceOnBlockchain(symbol, requesterId) {
    const config = {
      symbol,
      requester_id: requesterId,
    };
    
    return Deno.core.ops.op_oracle_update_price_on_blockchain(config);
  }
  
  /**
   * Update price data by index on the blockchain
   * @param {number} index - Price index (0 for NEO/USD, 1 for GAS/USD, 2 for BTC/USD, 3 for ETH/USD)
   * @param {string} requesterId - Requester ID
   * @returns {Promise<Object>} Transaction result
   */
  static async updatePriceByIndexOnBlockchain(index, requesterId) {
    const config = {
      index,
      requester_id: requesterId,
    };
    
    return Deno.core.ops.op_oracle_update_price_by_index_on_blockchain(config);
  }
  
  /**
   * Get a secure random number
   * @param {number} [min=0] - Minimum value (inclusive)
   * @param {number} [max=Number.MAX_SAFE_INTEGER] - Maximum value (inclusive)
   * @param {string} requesterId - Requester ID
   * @returns {Promise<number>} Random number
   */
  static async getSecureRandom(min = 0, max = Number.MAX_SAFE_INTEGER, requesterId) {
    const requestId = await Oracle.getRandom({
      min,
      max,
      count: 1,
      method: "secure",
    }, requesterId);
    
    const response = await Oracle.waitForResponse(requestId);
    return response.data.values[0];
  }
  
  /**
   * Get a blockchain-based random number
   * @param {number} [min=0] - Minimum value (inclusive)
   * @param {number} [max=Number.MAX_SAFE_INTEGER] - Maximum value (inclusive)
   * @param {string} requesterId - Requester ID
   * @returns {Promise<Object>} Random number with proof
   */
  static async getBlockchainRandom(min = 0, max = Number.MAX_SAFE_INTEGER, requesterId) {
    const requestId = await Oracle.getRandom({
      min,
      max,
      count: 1,
      method: "blockchain",
    }, requesterId);
    
    const response = await Oracle.waitForResponse(requestId);
    return {
      value: response.data.values[0],
      proof: response.data.proof,
    };
  }
}

export { Oracle };
