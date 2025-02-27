/**
 * Custom filter module for Neo N3 blockchain events
 * 
 * This module provides reusable filter functions that can be imported
 * and used in your event handler functions.
 */

// Import sub-modules
import { blockFilters } from './block-filters';
import { transactionFilters } from './transaction-filters';
import { notificationFilters } from './notification-filters';

/**
 * Check if a transaction is a high-value transaction
 * @param {Object} txData - The transaction data
 * @returns {boolean} - True if the transaction is high-value, false otherwise
 */
function isHighValueTransaction(txData) {
  // Implementation depends on transaction type
  if (txData.type === 'InvocationTransaction') {
    // For invocation transactions, we need to analyze the script
    // This is a simplified example - in a real implementation,
    // you would need to parse the script properly
    if (txData.script && txData.script.includes('transfer')) {
      // Check if this is a high-value transfer
      // This is a placeholder - in a real implementation,
      // you would need to decode the script to extract the amount
      return true;
    }
  }
  
  return false;
}

/**
 * Check if a block contains important transactions
 * @param {Object} blockData - The block data
 * @returns {boolean} - True if the block contains important transactions, false otherwise
 */
function hasImportantTransactions(blockData) {
  // Check if the block has transactions
  if (!blockData.tx || blockData.tx.length === 0) {
    return false;
  }
  
  // Check if any transaction is an important transaction
  for (const tx of blockData.tx) {
    if (tx.type === 'InvocationTransaction') {
      return true;
    }
  }
  
  return false;
}

/**
 * Check if a notification is from a whitelisted contract
 * @param {Object} notificationData - The notification data
 * @param {Array} whitelist - Array of whitelisted contract hashes
 * @returns {boolean} - True if the notification is from a whitelisted contract, false otherwise
 */
function isFromWhitelistedContract(notificationData, whitelist) {
  return whitelist.includes(notificationData.contract);
}

/**
 * Check if a notification is a token transfer
 * @param {Object} notificationData - The notification data
 * @returns {boolean} - True if the notification is a token transfer, false otherwise
 */
function isTokenTransfer(notificationData) {
  return notificationData.eventname === 'Transfer';
}

/**
 * Check if a notification is a token mint
 * @param {Object} notificationData - The notification data
 * @returns {boolean} - True if the notification is a token mint, false otherwise
 */
function isTokenMint(notificationData) {
  return notificationData.eventname === 'Mint';
}

/**
 * Check if a notification is a token burn
 * @param {Object} notificationData - The notification data
 * @returns {boolean} - True if the notification is a token burn, false otherwise
 */
function isTokenBurn(notificationData) {
  return notificationData.eventname === 'Burn';
}

// Export all filter functions
export const filters = {
  // Transaction filters
  isHighValueTransaction,
  
  // Block filters
  hasImportantTransactions,
  
  // Notification filters
  isFromWhitelistedContract,
  isTokenTransfer,
  isTokenMint,
  isTokenBurn,
  
  // Export sub-modules
  block: blockFilters,
  transaction: transactionFilters,
  notification: notificationFilters
};
