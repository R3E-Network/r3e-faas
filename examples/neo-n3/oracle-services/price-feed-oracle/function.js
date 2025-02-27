/**
 * Neo N3 Price Feed Oracle Example
 * 
 * This function demonstrates how to implement a price feed oracle service
 * that fetches price data from external sources and provides it to Neo N3
 * smart contracts in a secure and reliable way.
 * 
 * @param {Object} event - The event object (schedule or request)
 * @param {Object} context - The execution context
 * @returns {Object} - Price feed data and metadata
 */

// Import the Neo and Oracle modules from the r3e runtime
import { neo } from 'r3e';
import { oracle } from 'r3e';
import { runlog } from 'r3e';
import { crypto } from 'r3e';

// Define supported asset pairs
const SUPPORTED_PAIRS = [
  'NEO/USD',
  'GAS/USD',
  'BTC/USD',
  'ETH/USD',
  'NEO/BTC',
  'GAS/BTC'
];

// Define data sources with their respective endpoints and access methods
const DATA_SOURCES = {
  coinGecko: {
    name: 'CoinGecko',
    baseUrl: 'https://api.coingecko.com/api/v3',
    endpoints: {
      price: '/simple/price',
      marketData: '/coins/markets'
    },
    getPrice: async (pair) => {
      try {
        const [base, quote] = pair.split('/');
        const response = await oracle.fetch(`${DATA_SOURCES.coinGecko.baseUrl}${DATA_SOURCES.coinGecko.endpoints.price}?ids=${mapAssetToId(base, 'coinGecko')}&vs_currencies=${quote.toLowerCase()}`);
        const data = await response.json();
        const baseId = mapAssetToId(base, 'coinGecko');
        return {
          price: data[baseId][quote.toLowerCase()],
          timestamp: Date.now(),
          source: 'CoinGecko'
        };
      } catch (error) {
        runlog.error(`Error fetching price from CoinGecko: ${error.message}`);
        return null;
      }
    }
  },
  binance: {
    name: 'Binance',
    baseUrl: 'https://api.binance.com/api/v3',
    endpoints: {
      ticker: '/ticker/price',
      depth: '/depth'
    },
    getPrice: async (pair) => {
      try {
        const symbol = formatPairForExchange(pair, 'binance');
        const response = await oracle.fetch(`${DATA_SOURCES.binance.baseUrl}${DATA_SOURCES.binance.endpoints.ticker}?symbol=${symbol}`);
        const data = await response.json();
        return {
          price: parseFloat(data.price),
          timestamp: Date.now(),
          source: 'Binance'
        };
      } catch (error) {
        runlog.error(`Error fetching price from Binance: ${error.message}`);
        return null;
      }
    }
  },
  coinbase: {
    name: 'Coinbase',
    baseUrl: 'https://api.coinbase.com/v2',
    endpoints: {
      price: '/prices',
      exchange: '/exchange-rates'
    },
    getPrice: async (pair) => {
      try {
        const [base, quote] = pair.split('/');
        const response = await oracle.fetch(`${DATA_SOURCES.coinbase.baseUrl}${DATA_SOURCES.coinbase.endpoints.price}/${base}-${quote}/spot`);
        const data = await response.json();
        return {
          price: parseFloat(data.data.amount),
          timestamp: Date.now(),
          source: 'Coinbase'
        };
      } catch (error) {
        runlog.error(`Error fetching price from Coinbase: ${error.message}`);
        return null;
      }
    }
  }
};

/**
 * Main handler function for the price feed oracle
 */
export async function handler(event, context) {
  try {
    runlog.info('Price Feed Oracle function triggered');
    
    // Determine if this is a scheduled update or a direct request
    const isScheduled = event.type === 'schedule';
    const requestedPairs = isScheduled ? SUPPORTED_PAIRS : [event.data.pair];
    
    // Validate requested pairs
    const validPairs = requestedPairs.filter(pair => SUPPORTED_PAIRS.includes(pair));
    if (validPairs.length === 0) {
      throw new Error(`Unsupported asset pair(s): ${requestedPairs.join(', ')}`);
    }
    
    // Fetch and aggregate price data for each valid pair
    const results = {};
    for (const pair of validPairs) {
      results[pair] = await fetchAndAggregatePrices(pair, context);
    }
    
    // If this is a scheduled update, store the results
    if (isScheduled) {
      await storeResults(results, context);
    }
    
    // Return the results
    return {
      status: 'success',
      timestamp: Date.now(),
      data: results
    };
    
  } catch (error) {
    // Log any errors
    runlog.error('Error in price feed oracle:', error);
    
    // Return error information
    return {
      status: 'error',
      message: `Error in price feed oracle: ${error.message}`,
      error: error.stack
    };
  }
}

/**
 * Fetch price data from multiple sources and aggregate it
 * @param {string} pair - The asset pair (e.g., 'NEO/USD')
 * @param {Object} context - The execution context
 * @returns {Object} - Aggregated price data
 */
async function fetchAndAggregatePrices(pair, context) {
  runlog.info(`Fetching price data for ${pair}`);
  
  // Fetch price data from all sources
  const pricePromises = Object.values(DATA_SOURCES).map(source => source.getPrice(pair));
  const priceResults = await Promise.all(pricePromises);
  
  // Filter out null results (failed fetches)
  const validPrices = priceResults.filter(result => result !== null);
  
  // Check if we have enough valid prices
  if (validPrices.length === 0) {
    throw new Error(`Failed to fetch price data for ${pair} from any source`);
  }
  
  // Detect and remove outliers
  const filteredPrices = removeOutliers(validPrices);
  
  // Calculate aggregated price using different methods
  const aggregatedPrice = {
    pair,
    timestamp: Date.now(),
    sources: filteredPrices.map(p => p.source),
    price: {
      average: calculateAverage(filteredPrices),
      median: calculateMedian(filteredPrices),
      min: Math.min(...filteredPrices.map(p => p.price)),
      max: Math.max(...filteredPrices.map(p => p.price))
    },
    confidence: calculateConfidence(filteredPrices)
  };
  
  // Sign the price data for verification
  aggregatedPrice.signature = await signPriceData(aggregatedPrice);
  
  return aggregatedPrice;
}

/**
 * Store price results in the persistent storage
 * @param {Object} results - The price results to store
 * @param {Object} context - The execution context
 */
async function storeResults(results, context) {
  try {
    // Store each price result
    for (const [pair, data] of Object.entries(results)) {
      // Store the latest price
      await context.store.set(`price:${pair}:latest`, JSON.stringify(data));
      
      // Store historical price (with timestamp)
      const timestamp = data.timestamp;
      await context.store.set(`price:${pair}:${timestamp}`, JSON.stringify(data));
      
      // Update the price history index
      const historyKey = `price:${pair}:history`;
      const historyJson = await context.store.get(historyKey) || '[]';
      const history = JSON.parse(historyJson);
      
      // Add the new timestamp to the history
      history.push(timestamp);
      
      // Keep only the last 100 entries
      if (history.length > 100) {
        history.shift();
      }
      
      // Save the updated history
      await context.store.set(historyKey, JSON.stringify(history));
    }
    
    runlog.info('Price results stored successfully');
  } catch (error) {
    runlog.error('Error storing price results:', error);
    throw error;
  }
}

/**
 * Remove outliers from the price data
 * @param {Array} prices - Array of price data objects
 * @returns {Array} - Filtered array with outliers removed
 */
function removeOutliers(prices) {
  // If we have fewer than 4 prices, don't remove any
  if (prices.length < 4) {
    return prices;
  }
  
  // Calculate the mean and standard deviation
  const values = prices.map(p => p.price);
  const mean = values.reduce((sum, val) => sum + val, 0) / values.length;
  const stdDev = Math.sqrt(
    values.reduce((sum, val) => sum + Math.pow(val - mean, 2), 0) / values.length
  );
  
  // Filter out prices that are more than 2 standard deviations from the mean
  return prices.filter(p => Math.abs(p.price - mean) <= 2 * stdDev);
}

/**
 * Calculate the average price
 * @param {Array} prices - Array of price data objects
 * @returns {number} - Average price
 */
function calculateAverage(prices) {
  const sum = prices.reduce((total, p) => total + p.price, 0);
  return sum / prices.length;
}

/**
 * Calculate the median price
 * @param {Array} prices - Array of price data objects
 * @returns {number} - Median price
 */
function calculateMedian(prices) {
  const values = [...prices.map(p => p.price)].sort((a, b) => a - b);
  const mid = Math.floor(values.length / 2);
  
  return values.length % 2 === 0
    ? (values[mid - 1] + values[mid]) / 2
    : values[mid];
}

/**
 * Calculate the confidence score for the price data
 * @param {Array} prices - Array of price data objects
 * @returns {number} - Confidence score (0-1)
 */
function calculateConfidence(prices) {
  // More sources = higher confidence
  const sourceFactor = Math.min(prices.length / 3, 1);
  
  // Lower variance = higher confidence
  const values = prices.map(p => p.price);
  const mean = calculateAverage(prices);
  const variance = values.reduce((sum, val) => sum + Math.pow(val - mean, 2), 0) / values.length;
  const relativeVariance = variance / (mean * mean);
  const varianceFactor = Math.max(0, 1 - relativeVariance * 100);
  
  // Combine factors
  return Math.min(sourceFactor * varianceFactor, 1);
}

/**
 * Sign the price data for verification
 * @param {Object} priceData - The price data to sign
 * @returns {string} - Signature
 */
async function signPriceData(priceData) {
  try {
    // Create a string representation of the price data
    const dataString = JSON.stringify({
      pair: priceData.pair,
      timestamp: priceData.timestamp,
      price: priceData.price.average
    });
    
    // Sign the data using the oracle's private key
    const signature = await crypto.sign(dataString, 'oracle-price-feed');
    
    return signature;
  } catch (error) {
    runlog.error('Error signing price data:', error);
    return '';
  }
}

/**
 * Map asset symbol to ID for specific data source
 * @param {string} asset - Asset symbol (e.g., 'NEO')
 * @param {string} source - Data source name
 * @returns {string} - Asset ID for the specified source
 */
function mapAssetToId(asset, source) {
  const assetMappings = {
    coinGecko: {
      'NEO': 'neo',
      'GAS': 'gas',
      'BTC': 'bitcoin',
      'ETH': 'ethereum',
      'USD': 'usd'
    },
    binance: {
      'NEO': 'NEO',
      'GAS': 'GAS',
      'BTC': 'BTC',
      'ETH': 'ETH',
      'USD': 'USDT' // Binance uses USDT as a USD proxy
    },
    coinbase: {
      'NEO': 'NEO',
      'GAS': 'GAS',
      'BTC': 'BTC',
      'ETH': 'ETH',
      'USD': 'USD'
    }
  };
  
  return assetMappings[source][asset] || asset;
}

/**
 * Format asset pair for specific exchange
 * @param {string} pair - Asset pair (e.g., 'NEO/USD')
 * @param {string} exchange - Exchange name
 * @returns {string} - Formatted pair for the specified exchange
 */
function formatPairForExchange(pair, exchange) {
  const [base, quote] = pair.split('/');
  
  switch (exchange) {
    case 'binance':
      return `${mapAssetToId(base, 'binance')}${mapAssetToId(quote, 'binance')}`;
    case 'coinbase':
      return `${mapAssetToId(base, 'coinbase')}-${mapAssetToId(quote, 'coinbase')}`;
    default:
      return pair;
  }
}
