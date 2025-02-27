/**
 * Notification-specific filters for Neo N3 blockchain events
 */

/**
 * Check if a notification is from a specific contract
 * @param {Object} notificationData - The notification data
 * @param {string} contractHash - Contract hash to check for
 * @returns {boolean} - True if the notification is from the specified contract, false otherwise
 */
function isFromContract(notificationData, contractHash) {
  return notificationData.contract === contractHash;
}

/**
 * Check if a notification has a specific event name
 * @param {Object} notificationData - The notification data
 * @param {string} eventName - Event name to check for
 * @returns {boolean} - True if the notification has the specified event name, false otherwise
 */
function hasEventName(notificationData, eventName) {
  return notificationData.eventname === eventName;
}

/**
 * Check if a notification event name matches a pattern
 * @param {Object} notificationData - The notification data
 * @param {string|RegExp} pattern - Pattern to check for
 * @returns {boolean} - True if the notification event name matches the pattern, false otherwise
 */
function eventNameMatches(notificationData, pattern) {
  if (typeof pattern === 'string') {
    return notificationData.eventname.includes(pattern);
  } else if (pattern instanceof RegExp) {
    return pattern.test(notificationData.eventname);
  }
  
  return false;
}

/**
 * Check if a notification state contains a specific value
 * @param {Object} notificationData - The notification data
 * @param {*} value - Value to check for
 * @returns {boolean} - True if the notification state contains the value, false otherwise
 */
function stateContains(notificationData, value) {
  if (!notificationData.state || !Array.isArray(notificationData.state)) {
    return false;
  }
  
  // Check if any state item contains the value
  return notificationData.state.some(item => {
    if (typeof item === 'object' && item.value !== undefined) {
      return item.value === value;
    }
    return item === value;
  });
}

/**
 * Check if a notification is a high-value transfer
 * @param {Object} notificationData - The notification data
 * @param {number} threshold - Minimum amount to consider high-value
 * @returns {boolean} - True if the notification is a high-value transfer, false otherwise
 */
function isHighValueTransfer(notificationData, threshold = 100) {
  // Check if this is a Transfer notification
  if (notificationData.eventname !== 'Transfer') {
    return false;
  }
  
  // Check if the state has the expected format for a transfer
  if (!notificationData.state || !Array.isArray(notificationData.state) || notificationData.state.length < 3) {
    return false;
  }
  
  // Extract the amount from the state
  let amount;
  const amountItem = notificationData.state[2];
  
  if (typeof amountItem === 'object' && amountItem.value !== undefined) {
    amount = parseFloat(amountItem.value);
  } else {
    amount = parseFloat(amountItem);
  }
  
  // Convert from satoshi to whole tokens
  amount = amount / 100000000;
  
  // Check if the amount exceeds the threshold
  return amount >= threshold;
}

// Export all notification filter functions
export const notificationFilters = {
  isFromContract,
  hasEventName,
  eventNameMatches,
  stateContains,
  isHighValueTransfer
};
