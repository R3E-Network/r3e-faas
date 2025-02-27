/**
 * Block-specific filters for Neo N3 blockchain events
 */

/**
 * Check if a block is a milestone block (every 10000 blocks)
 * @param {Object} blockData - The block data
 * @returns {boolean} - True if the block is a milestone block, false otherwise
 */
function isMilestoneBlock(blockData) {
  return blockData.index % 10000 === 0;
}

/**
 * Check if a block is recent (within the last 24 hours)
 * @param {Object} blockData - The block data
 * @returns {boolean} - True if the block is recent, false otherwise
 */
function isRecentBlock(blockData) {
  const blockTime = new Date(blockData.time * 1000);
  const now = new Date();
  const hoursDiff = (now - blockTime) / (1000 * 60 * 60);
  
  return hoursDiff <= 24;
}

/**
 * Check if a block has a minimum number of transactions
 * @param {Object} blockData - The block data
 * @param {number} minTxCount - Minimum number of transactions
 * @returns {boolean} - True if the block has at least minTxCount transactions, false otherwise
 */
function hasMinimumTransactions(blockData, minTxCount) {
  return blockData.tx && blockData.tx.length >= minTxCount;
}

/**
 * Check if a block has a specific transaction type
 * @param {Object} blockData - The block data
 * @param {string} txType - Transaction type to check for
 * @returns {boolean} - True if the block has at least one transaction of the specified type, false otherwise
 */
function hasTransactionType(blockData, txType) {
  if (!blockData.tx || blockData.tx.length === 0) {
    return false;
  }
  
  return blockData.tx.some(tx => tx.type === txType);
}

/**
 * Check if a block is in a specific index range
 * @param {Object} blockData - The block data
 * @param {number} minIndex - Minimum block index
 * @param {number} maxIndex - Maximum block index
 * @returns {boolean} - True if the block index is within the specified range, false otherwise
 */
function isInIndexRange(blockData, minIndex, maxIndex) {
  return blockData.index >= minIndex && blockData.index <= maxIndex;
}

// Export all block filter functions
export const blockFilters = {
  isMilestoneBlock,
  isRecentBlock,
  hasMinimumTransactions,
  hasTransactionType,
  isInIndexRange
};
