/**
 * Neo N3 Oracle Rate Limiting Example
 * 
 * This function demonstrates how to implement rate limiting for oracle services
 * in the Neo N3 FaaS platform.
 */

// Import the Neo and Oracle modules from the r3e runtime
import { neo } from 'r3e';
import { oracle } from 'r3e';
import { runlog } from 'r3e';
import { store } from 'r3e';

/**
 * Main handler function for the oracle rate limiting service
 */
export async function handler(event, context) {
  try {
    runlog.info('Oracle Rate Limiting Service function triggered');
    
    // Get configuration from context
    const config = context.config.oracle.rate_limiting;
    
    // Parse request
    const request = parseRequest(event);
    
    // Get client information
    const clientInfo = getClientInfo(request);
    
    // Check rate limits
    const rateLimitResult = await checkRateLimits(clientInfo, config, context);
    
    // If rate limit exceeded, return error
    if (!rateLimitResult.success) {
      runlog.warn('Rate limit exceeded:', rateLimitResult);
      return createRateLimitExceededResponse(rateLimitResult, config.response);
    }
    
    // Process the request
    const result = await processRequest(request);
    
    // Add rate limit headers to the response
    return addRateLimitHeaders(result, rateLimitResult, config.response);
    
  } catch (error) {
    runlog.error('Error in oracle rate limiting service:', error);
    return {
      status: 'error',
      message: `Error in oracle rate limiting service: ${error.message}`
    };
  }
}

/**
 * Parse the request from the event
 */
function parseRequest(event) {
  if (event.type === 'http') {
    return {
      resource: event.path.split('/')[2] || 'oracle',
      action: event.method === 'GET' ? 'read' : 'write',
      data: event.body || {},
      headers: event.headers || {},
      ip: event.ip || '0.0.0.0'
    };
  }
  
  return {
    resource: 'oracle',
    action: 'read',
    data: event.data || {},
    headers: {},
    ip: '0.0.0.0'
  };
}

/**
 * Get client information from the request
 */
function getClientInfo(request) {
  return {
    userId: request.headers['x-user-id'] || 'anonymous',
    ip: request.ip || '0.0.0.0',
    resource: request.resource || 'oracle'
  };
}

/**
 * Check rate limits for the request
 */
async function checkRateLimits(clientInfo, config, context) {
  // Check fixed window rate limit
  if (config.strategies.fixed_window && config.strategies.fixed_window.enabled) {
    const result = await checkFixedWindowRateLimit(clientInfo, config, context);
    if (!result.success) return result;
  }
  
  // Check sliding window rate limit
  if (config.strategies.sliding_window && config.strategies.sliding_window.enabled) {
    const result = await checkSlidingWindowRateLimit(clientInfo, config, context);
    if (!result.success) return result;
  }
  
  // Check token bucket rate limit
  if (config.strategies.token_bucket && config.strategies.token_bucket.enabled) {
    const result = await checkTokenBucketRateLimit(clientInfo, config, context);
    if (!result.success) return result;
  }
  
  // If all strategies pass, return success
  return { success: true, clientInfo };
}

/**
 * Check fixed window rate limit
 */
async function checkFixedWindowRateLimit(clientInfo, config, context) {
  const { userId } = clientInfo;
  const now = Date.now();
  const currentMinute = Math.floor(now / 60000);
  
  // Get user limits
  const limits = getUserLimits(userId, config);
  
  // Check per-minute limit
  const minuteKey = `rate:${userId}:minute:${currentMinute}`;
  const minuteCount = await incrementCounter(minuteKey, 60, context);
  
  if (minuteCount > limits.per_minute) {
    return {
      success: false,
      strategy: 'fixed_window',
      window: 'minute',
      limit: limits.per_minute,
      count: minuteCount,
      retry_after: Math.ceil(60 - (now % 60000) / 1000)
    };
  }
  
  return { success: true };
}

/**
 * Check sliding window rate limit
 */
async function checkSlidingWindowRateLimit(clientInfo, config, context) {
  const { userId } = clientInfo;
  const now = Date.now();
  
  // Get window configuration
  const windows = getUserWindows(userId, config);
  
  // Check each window
  for (const window of windows) {
    const windowStart = now - (window.window * 1000);
    const windowKey = `rate:${userId}:sliding:${window.window}`;
    
    // Get requests in the window
    const requests = await getRequestsInWindow(windowKey, windowStart, now, context);
    
    if (requests.length >= window.max_requests) {
      // Calculate retry after time
      const oldestRequest = requests[0];
      const retryAfter = Math.ceil((oldestRequest.timestamp + (window.window * 1000) - now) / 1000);
      
      return {
        success: false,
        strategy: 'sliding_window',
        window: window.window,
        limit: window.max_requests,
        count: requests.length,
        retry_after: retryAfter
      };
    }
  }
  
  // Record this request
  await recordRequest(userId, now, context);
  
  return { success: true };
}

/**
 * Check token bucket rate limit
 */
async function checkTokenBucketRateLimit(clientInfo, config, context) {
  const { userId } = clientInfo;
  const now = Date.now();
  
  // Get bucket configuration
  const bucketConfig = getUserBucket(userId, config);
  
  // Get current bucket state
  const bucketKey = `rate:${userId}:bucket`;
  const bucketJson = await context.store.get(bucketKey);
  
  let bucket;
  if (bucketJson) {
    bucket = JSON.parse(bucketJson);
  } else {
    // Initialize bucket
    bucket = {
      tokens: bucketConfig.capacity,
      lastRefill: now
    };
  }
  
  // Calculate token refill
  const elapsedSeconds = (now - bucket.lastRefill) / 1000;
  const refillAmount = elapsedSeconds * bucketConfig.refill_rate;
  
  // Refill bucket
  bucket.tokens = Math.min(bucket.tokens + refillAmount, bucketConfig.capacity);
  bucket.lastRefill = now;
  
  // Check if enough tokens
  if (bucket.tokens < 1) {
    // Calculate retry after time
    const retryAfter = Math.ceil((1 - bucket.tokens) / bucketConfig.refill_rate);
    
    return {
      success: false,
      strategy: 'token_bucket',
      capacity: bucketConfig.capacity,
      tokens: bucket.tokens,
      retry_after: retryAfter
    };
  }
  
  // Consume token
  bucket.tokens -= 1;
  
  // Save bucket state
  await context.store.set(bucketKey, JSON.stringify(bucket));
  
  return { success: true };
}

/**
 * Get user limits for fixed window rate limiting
 */
function getUserLimits(userId, config) {
  // Check if user has specific limits
  if (config.strategies.fixed_window.user_limits) {
    const userLimit = config.strategies.fixed_window.user_limits.find(ul => ul.user_id === userId);
    if (userLimit) {
      return userLimit.limits;
    }
  }
  
  // Return default limits
  return config.strategies.fixed_window.default_limits;
}

/**
 * Get user windows for sliding window rate limiting
 */
function getUserWindows(userId, config) {
  // Check if user has specific windows
  if (config.strategies.sliding_window.user_windows) {
    const userWindow = config.strategies.sliding_window.user_windows.find(uw => uw.user_id === userId);
    if (userWindow) {
      return userWindow.windows;
    }
  }
  
  // Return default windows
  return config.strategies.sliding_window.default_windows;
}

/**
 * Get user bucket for token bucket rate limiting
 */
function getUserBucket(userId, config) {
  // Check if user has specific bucket
  if (config.strategies.token_bucket.user_buckets) {
    const userBucket = config.strategies.token_bucket.user_buckets.find(ub => ub.user_id === userId);
    if (userBucket) {
      return userBucket.bucket;
    }
  }
  
  // Return default bucket
  return config.strategies.token_bucket.default_bucket;
}

/**
 * Increment a counter and return the new value
 */
async function incrementCounter(key, expireSeconds, context) {
  // Get current count
  const countStr = await context.store.get(key);
  const count = countStr ? parseInt(countStr) : 0;
  
  // Increment count
  const newCount = count + 1;
  
  // Store new count with expiration
  await context.store.set(key, newCount.toString(), expireSeconds);
  
  return newCount;
}

/**
 * Get requests in a time window
 */
async function getRequestsInWindow(windowKey, start, end, context) {
  // Get request list
  const requestsJson = await context.store.get(windowKey);
  const requests = requestsJson ? JSON.parse(requestsJson) : [];
  
  // Filter requests in the window
  return requests.filter(req => req.timestamp >= start && req.timestamp <= end);
}

/**
 * Record a request for sliding window rate limiting
 */
async function recordRequest(userId, timestamp, context) {
  // Get all window sizes
  const windowSizes = [60, 300]; // 1 minute, 5 minutes
  
  // Record request for each window size
  for (const windowSize of windowSizes) {
    const windowKey = `rate:${userId}:sliding:${windowSize}`;
    
    // Get existing requests
    const requestsJson = await context.store.get(windowKey);
    const requests = requestsJson ? JSON.parse(requestsJson) : [];
    
    // Add new request
    requests.push({ timestamp });
    
    // Remove old requests
    const cutoff = timestamp - (windowSize * 1000);
    const filteredRequests = requests.filter(req => req.timestamp >= cutoff);
    
    // Store updated requests
    await context.store.set(windowKey, JSON.stringify(filteredRequests), windowSize);
  }
}

/**
 * Create response for rate limit exceeded
 */
function createRateLimitExceededResponse(rateLimitResult, responseConfig) {
  // Create response body
  const body = responseConfig.body_template
    .replace('{{retry_after}}', rateLimitResult.retry_after);
  
  // Create response
  return {
    status: 'error',
    statusCode: responseConfig.status_code,
    headers: {
      [responseConfig.headers.retry_after]: rateLimitResult.retry_after.toString()
    },
    body: JSON.parse(body)
  };
}

/**
 * Add rate limit headers to response
 */
function addRateLimitHeaders(response, rateLimitResult, responseConfig) {
  // Only add headers if configured
  if (!responseConfig.include_headers) {
    return response;
  }
  
  // Add headers
  response.headers = response.headers || {};
  
  // Add rate limit headers based on the strategy
  if (rateLimitResult.strategy === 'fixed_window') {
    response.headers[responseConfig.headers.limit] = rateLimitResult.limit.toString();
    response.headers[responseConfig.headers.remaining] = Math.max(0, rateLimitResult.limit - rateLimitResult.count).toString();
    response.headers[responseConfig.headers.reset] = rateLimitResult.retry_after.toString();
  } else if (rateLimitResult.strategy === 'sliding_window') {
    response.headers[responseConfig.headers.limit] = rateLimitResult.limit.toString();
    response.headers[responseConfig.headers.remaining] = Math.max(0, rateLimitResult.limit - rateLimitResult.count).toString();
    response.headers[responseConfig.headers.reset] = rateLimitResult.retry_after.toString();
  } else if (rateLimitResult.strategy === 'token_bucket') {
    response.headers[responseConfig.headers.limit] = rateLimitResult.capacity.toString();
    response.headers[responseConfig.headers.remaining] = Math.floor(rateLimitResult.tokens).toString();
    response.headers[responseConfig.headers.reset] = '0'; // Tokens refill continuously
  }
  
  return response;
}

/**
 * Process the request
 */
async function processRequest(request) {
  // In a real implementation, this would process the request
  // For this example, we'll return mock data
  
  switch (request.resource) {
    case 'price':
      return {
        status: 'success',
        data: {
          prices: [
            { symbol: 'NEO', price: 42.5 },
            { symbol: 'GAS', price: 12.3 },
            { symbol: 'BTC', price: 50000 }
          ]
        }
      };
    
    case 'weather':
      return {
        status: 'success',
        data: {
          location: 'New York',
          temperature: 22.5,
          condition: 'partly cloudy'
        }
      };
    
    default:
      return {
        status: 'success',
        data: {
          message: 'Oracle service is available'
        }
      };
  }
}
