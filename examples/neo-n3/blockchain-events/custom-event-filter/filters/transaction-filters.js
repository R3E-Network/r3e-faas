/**
 * Transaction-specific filters for Neo N3 blockchain events
 */

/**
 * Check if a transaction is of a specific type
 * @param {Object} txData - The transaction data
 * @param {string} type - Transaction type to check for
 * @returns {boolean} - True if the transaction is of the specified type, false otherwise
 */
function isType(txData, type) {
  return txData.type === type;
}

/**
 * Check if a transaction is an invocation transaction
 * @param {Object} txData - The transaction data
 * @returns {boolean} - True if the transaction is an invocation transaction, false otherwise
 */
function isInvocationTransaction(txData) {
  return txData.type === 'InvocationTransaction';
}

/**
 * Check if a transaction is a claim transaction
 * @param {Object} txData - The transaction data
 * @returns {boolean} - True if the transaction is a claim transaction, false otherwise
 */
function isClaimTransaction(txData) {
  return txData.type === 'ClaimTransaction';
}

/**
 * Check if a transaction has a minimum size
 * @param {Object} txData - The transaction data
 * @param {number} minSize - Minimum transaction size in bytes
 * @returns {boolean} - True if the transaction size is at least minSize, false otherwise
 */
function hasMinimumSize(txData, minSize) {
  return txData.size >= minSize;
}

/**
 * Check if a transaction script contains a specific pattern
 * @param {Object} txData - The transaction data
 * @param {string|RegExp} pattern - Pattern to check for
 * @returns {boolean} - True if the transaction script contains the pattern, false otherwise
 */
function scriptContains(txData, pattern) {
  if (!txData.script) {
    return false;
  }
  
  if (typeof pattern === 'string') {
    return txData.script.includes(pattern);
  } else if (pattern instanceof RegExp) {
    return pattern.test(txData.script);
  }
  
  return false;
}

/**
 * Check if a transaction involves a specific contract
 * @param {Object} txData - The transaction data
 * @param {string} contractHash - Contract hash to check for
 * @returns {boolean} - True if the transaction involves the specified contract, false otherwise
 */
function involvesContract(txData, contractHash) {
  if (!txData.script) {
    return false;
  }
  
  // This is a simplified check - in a real implementation,
  // you would need to parse the script properly
  return txData.script.includes(contractHash);
}

// Export all transaction filter functions
export const transactionFilters = {
  isType,
  isInvocationTransaction,
  isClaimTransaction,
  hasMinimumSize,
  scriptContains,
  involvesContract
};
