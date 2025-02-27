/**
 * Rate Limiter Library for Neo N3 Oracle Services
 * 
 * This library provides various rate limiting strategies for Neo N3 oracle services.
 */

/**
 * Fixed Window Rate Limiter
 * 
 * Implements a fixed window rate limiting strategy where a fixed number of
 * requests are allowed within a fixed time window (e.g., 100 requests per minute).
 */
export class FixedWindowRateLimiter {
  /**
   * Create a new fixed window rate limiter
   * 
   * @param {Object} store - Storage interface for persisting rate limit data
   * @param {Object} options - Configuration options
   * @param {string} options.keyPrefix - Prefix for rate limit keys
   * @param {number} options.perSecond - Requests allowed per second
   * @param {number} options.perMinute - Requests allowed per minute
   * @param {number} options.perHour - Requests allowed per hour
   * @param {number} options.perDay - Requests allowed per day
   */
  constructor(store, options = {}) {
    this.store = store;
    this.keyPrefix = options.keyPrefix || 'rate:fixed:';
    this.limits = {
      perSecond: options.perSecond || 5,
      perMinute: options.perMinute || 100,
      perHour: options.perHour || 1000,
      perDay: options.perDay || 10000
    };
  }
  
  /**
   * Check if a request is allowed
   * 
   * @param {string} id - Identifier for the requester (user ID, IP, etc.)
   * @returns {Promise<Object>} - Result of the rate limit check
   */
  async check(id) {
    const now = Date.now();
    const currentSecond = Math.floor(now / 1000);
    const currentMinute = Math.floor(now / 60000);
    const currentHour = Math.floor(now / 3600000);
    const currentDay = Math.floor(now / 86400000);
    
    // Check per-second limit
    const secondKey = `${this.keyPrefix}${id}:second:${currentSecond}`;
    const secondCount = await this.increment(secondKey, 1);
    
    if (secondCount > this.limits.perSecond) {
      return {
        allowed: false,
        limit: this.limits.perSecond,
        remaining: 0,
        reset: 1,
        retryAfter: 1
      };
    }
    
    // Check per-minute limit
    const minuteKey = `${this.keyPrefix}${id}:minute:${currentMinute}`;
    const minuteCount = await this.increment(minuteKey, 60);
    
    if (minuteCount > this.limits.perMinute) {
      return {
        allowed: false,
        limit: this.limits.perMinute,
        remaining: 0,
        reset: 60 - (now % 60000) / 1000,
        retryAfter: Math.ceil(60 - (now % 60000) / 1000)
      };
    }
    
    // Check per-hour limit
    const hourKey = `${this.keyPrefix}${id}:hour:${currentHour}`;
    const hourCount = await this.increment(hourKey, 3600);
    
    if (hourCount > this.limits.perHour) {
      return {
        allowed: false,
        limit: this.limits.perHour,
        remaining: 0,
        reset: 3600 - (now % 3600000) / 1000,
        retryAfter: Math.ceil(3600 - (now % 3600000) / 1000)
      };
    }
    
    // Check per-day limit
    const dayKey = `${this.keyPrefix}${id}:day:${currentDay}`;
    const dayCount = await this.increment(dayKey, 86400);
    
    if (dayCount > this.limits.perDay) {
      return {
        allowed: false,
        limit: this.limits.perDay,
        remaining: 0,
        reset: 86400 - (now % 86400000) / 1000,
        retryAfter: Math.ceil(86400 - (now % 86400000) / 1000)
      };
    }
    
    // All checks passed
    return {
      allowed: true,
      limit: this.limits.perMinute,
      remaining: this.limits.perMinute - minuteCount,
      reset: Math.ceil(60 - (now % 60000) / 1000),
      retryAfter: 0
    };
  }
  
  /**
   * Increment a counter and return the new value
   * 
   * @param {string} key - Key for the counter
   * @param {number} expireSeconds - Expiration time in seconds
   * @returns {Promise<number>} - New counter value
   */
  async increment(key, expireSeconds) {
    // Get current count
    const countStr = await this.store.get(key);
    const count = countStr ? parseInt(countStr) : 0;
    
    // Increment count
    const newCount = count + 1;
    
    // Store new count with expiration
    await this.store.set(key, newCount.toString(), expireSeconds);
    
    return newCount;
  }
}

/**
 * Sliding Window Rate Limiter
 * 
 * Implements a sliding window rate limiting strategy that considers a sliding
 * time window, providing smoother rate limiting.
 */
export class SlidingWindowRateLimiter {
  /**
   * Create a new sliding window rate limiter
   * 
   * @param {Object} store - Storage interface for persisting rate limit data
   * @param {Object} options - Configuration options
   * @param {string} options.keyPrefix - Prefix for rate limit keys
   * @param {Array<Object>} options.windows - Window configurations
   */
  constructor(store, options = {}) {
    this.store = store;
    this.keyPrefix = options.keyPrefix || 'rate:sliding:';
    this.windows = options.windows || [
      { window: 60, maxRequests: 100 },
      { window: 300, maxRequests: 300 }
    ];
  }
  
  /**
   * Check if a request is allowed
   * 
   * @param {string} id - Identifier for the requester (user ID, IP, etc.)
   * @returns {Promise<Object>} - Result of the rate limit check
   */
  async check(id) {
    const now = Date.now();
    
    // Check each window
    for (const window of this.windows) {
      const windowStart = now - (window.window * 1000);
      const windowKey = `${this.keyPrefix}${id}:${window.window}`;
      
      // Get requests in the window
      const requests = await this.getRequestsInWindow(windowKey, windowStart, now);
      
      if (requests.length >= window.maxRequests) {
        // Calculate retry after time
        const oldestRequest = requests[0];
        const retryAfter = Math.ceil((oldestRequest.timestamp + (window.window * 1000) - now) / 1000);
        
        return {
          allowed: false,
          limit: window.maxRequests,
          remaining: 0,
          reset: retryAfter,
          retryAfter: retryAfter
        };
      }
    }
    
    // Record this request
    await this.recordRequest(id, now);
    
    // All checks passed
    const window = this.windows[0];
    return {
      allowed: true,
      limit: window.maxRequests,
      remaining: window.maxRequests - 1, // We just recorded one request
      reset: window.window,
      retryAfter: 0
    };
  }
  
  /**
   * Get requests in a time window
   * 
   * @param {string} windowKey - Key for the window
   * @param {number} start - Start timestamp
   * @param {number} end - End timestamp
   * @returns {Promise<Array<Object>>} - Requests in the window
   */
  async getRequestsInWindow(windowKey, start, end) {
    // Get request list
    const requestsJson = await this.store.get(windowKey);
    const requests = requestsJson ? JSON.parse(requestsJson) : [];
    
    // Filter requests in the window
    return requests.filter(req => req.timestamp >= start && req.timestamp <= end);
  }
  
  /**
   * Record a request for sliding window rate limiting
   * 
   * @param {string} id - Identifier for the requester
   * @param {number} timestamp - Request timestamp
   * @returns {Promise<void>}
   */
  async recordRequest(id, timestamp) {
    // Record request for each window size
    for (const window of this.windows) {
      const windowKey = `${this.keyPrefix}${id}:${window.window}`;
      
      // Get existing requests
      const requestsJson = await this.store.get(windowKey);
      const requests = requestsJson ? JSON.parse(requestsJson) : [];
      
      // Add new request
      requests.push({ timestamp });
      
      // Remove old requests
      const cutoff = timestamp - (window.window * 1000);
      const filteredRequests = requests.filter(req => req.timestamp >= cutoff);
      
      // Store updated requests
      await this.store.set(windowKey, JSON.stringify(filteredRequests), window.window);
    }
  }
}

/**
 * Token Bucket Rate Limiter
 * 
 * Implements a token bucket rate limiting strategy that allows for bursts of
 * traffic while maintaining a long-term rate limit.
 */
export class TokenBucketRateLimiter {
  /**
   * Create a new token bucket rate limiter
   * 
   * @param {Object} store - Storage interface for persisting rate limit data
   * @param {Object} options - Configuration options
   * @param {string} options.keyPrefix - Prefix for rate limit keys
   * @param {number} options.capacity - Bucket capacity (maximum tokens)
   * @param {number} options.refillRate - Token refill rate per second
   */
  constructor(store, options = {}) {
    this.store = store;
    this.keyPrefix = options.keyPrefix || 'rate:bucket:';
    this.capacity = options.capacity || 100;
    this.refillRate = options.refillRate || 1;
  }
  
  /**
   * Check if a request is allowed
   * 
   * @param {string} id - Identifier for the requester (user ID, IP, etc.)
   * @returns {Promise<Object>} - Result of the rate limit check
   */
  async check(id) {
    const now = Date.now();
    const bucketKey = `${this.keyPrefix}${id}`;
    
    // Get current bucket state
    const bucketJson = await this.store.get(bucketKey);
    
    let bucket;
    if (bucketJson) {
      bucket = JSON.parse(bucketJson);
    } else {
      // Initialize bucket
      bucket = {
        tokens: this.capacity,
        lastRefill: now
      };
    }
    
    // Calculate token refill
    const elapsedSeconds = (now - bucket.lastRefill) / 1000;
    const refillAmount = elapsedSeconds * this.refillRate;
    
    // Refill bucket
    bucket.tokens = Math.min(bucket.tokens + refillAmount, this.capacity);
    bucket.lastRefill = now;
    
    // Check if enough tokens
    if (bucket.tokens < 1) {
      // Calculate retry after time
      const retryAfter = Math.ceil((1 - bucket.tokens) / this.refillRate);
      
      // Save bucket state
      await this.store.set(bucketKey, JSON.stringify(bucket));
      
      return {
        allowed: false,
        limit: this.capacity,
        remaining: 0,
        reset: retryAfter,
        retryAfter: retryAfter
      };
    }
    
    // Consume token
    bucket.tokens -= 1;
    
    // Save bucket state
    await this.store.set(bucketKey, JSON.stringify(bucket));
    
    return {
      allowed: true,
      limit: this.capacity,
      remaining: Math.floor(bucket.tokens),
      reset: 0, // Tokens refill continuously
      retryAfter: 0
    };
  }
}

/**
 * Adaptive Rate Limiter
 * 
 * Implements an adaptive rate limiting strategy that adjusts rate limits
 * based on system load.
 */
export class AdaptiveRateLimiter {
  /**
   * Create a new adaptive rate limiter
   * 
   * @param {Object} store - Storage interface for persisting rate limit data
   * @param {Object} options - Configuration options
   * @param {string} options.keyPrefix - Prefix for rate limit keys
   * @param {Object} options.baseLimiter - Base rate limiter
   * @param {Object} options.thresholds - System load thresholds
   * @param {Object} options.adjustmentFactors - Adjustment factors for different load levels
   */
  constructor(store, options = {}) {
    this.store = store;
    this.keyPrefix = options.keyPrefix || 'rate:adaptive:';
    this.baseLimiter = options.baseLimiter;
    this.thresholds = options.thresholds || {
      low: 0.3,
      medium: 0.6,
      high: 0.8
    };
    this.adjustmentFactors = options.adjustmentFactors || {
      low: 1.2,
      medium: 1.0,
      high: 0.8,
      critical: 0.5
    };
  }
  
  /**
   * Check if a request is allowed
   * 
   * @param {string} id - Identifier for the requester (user ID, IP, etc.)
   * @returns {Promise<Object>} - Result of the rate limit check
   */
  async check(id) {
    // Get system load
    const systemLoad = await this.getSystemLoad();
    
    // Calculate adaptive factor
    const adaptiveFactor = this.calculateAdaptiveFactor(systemLoad);
    
    // Apply adaptive factor to base limiter
    const result = await this.baseLimiter.check(id);
    
    // If already not allowed, return as is
    if (!result.allowed) {
      return result;
    }
    
    // Adjust limit based on system load
    const adjustedLimit = Math.floor(result.limit * adaptiveFactor);
    
    // Check if over adjusted limit
    if (result.limit - result.remaining > adjustedLimit) {
      return {
        allowed: false,
        limit: adjustedLimit,
        remaining: 0,
        reset: 30, // Default reset time for adaptive limiting
        retryAfter: 30
      };
    }
    
    // Adjust remaining based on adaptive factor
    return {
      allowed: true,
      limit: adjustedLimit,
      remaining: Math.max(0, adjustedLimit - (result.limit - result.remaining)),
      reset: result.reset,
      retryAfter: 0
    };
  }
  
  /**
   * Get system load
   * 
   * @returns {Promise<number>} - System load (0-1)
   */
  async getSystemLoad() {
    // In a real implementation, this would get the actual system load
    // For this example, we'll return a random value
    return Math.random();
  }
  
  /**
   * Calculate adaptive factor based on system load
   * 
   * @param {number} systemLoad - System load (0-1)
   * @returns {number} - Adjustment factor
   */
  calculateAdaptiveFactor(systemLoad) {
    if (systemLoad < this.thresholds.low) {
      return this.adjustmentFactors.low;
    } else if (systemLoad < this.thresholds.medium) {
      return this.adjustmentFactors.medium;
    } else if (systemLoad < this.thresholds.high) {
      return this.adjustmentFactors.high;
    } else {
      return this.adjustmentFactors.critical;
    }
  }
}

/**
 * Multi-Level Rate Limiter
 * 
 * Implements a multi-level rate limiting strategy that applies different
 * limits at different levels (user, IP, resource, etc.).
 */
export class MultiLevelRateLimiter {
  /**
   * Create a new multi-level rate limiter
   * 
   * @param {Object} store - Storage interface for persisting rate limit data
   * @param {Object} options - Configuration options
   * @param {Array<Object>} options.levels - Level configurations
   */
  constructor(store, options = {}) {
    this.store = store;
    this.levels = options.levels || [];
  }
  
  /**
   * Check if a request is allowed
   * 
   * @param {Object} context - Request context with identifiers for each level
   * @returns {Promise<Object>} - Result of the rate limit check
   */
  async check(context) {
    // Check each level
    for (const level of this.levels) {
      const id = context[level.id];
      
      if (!id) {
        continue;
      }
      
      const result = await level.limiter.check(id);
      
      if (!result.allowed) {
        return {
          allowed: false,
          level: level.id,
          limit: result.limit,
          remaining: result.remaining,
          reset: result.reset,
          retryAfter: result.retryAfter
        };
      }
    }
    
    // All levels passed
    return {
      allowed: true,
      level: null,
      limit: null,
      remaining: null,
      reset: null,
      retryAfter: 0
    };
  }
}

/**
 * Rate Limiter Factory
 * 
 * Factory for creating rate limiters based on configuration.
 */
export class RateLimiterFactory {
  /**
   * Create a rate limiter based on configuration
   * 
   * @param {Object} store - Storage interface for persisting rate limit data
   * @param {Object} config - Rate limiter configuration
   * @returns {Object} - Rate limiter instance
   */
  static create(store, config) {
    if (!config || !config.strategies) {
      return null;
    }
    
    const limiters = {};
    
    // Create fixed window rate limiter
    if (config.strategies.fixed_window && config.strategies.fixed_window.enabled) {
      limiters.fixedWindow = new FixedWindowRateLimiter(store, {
        keyPrefix: config.storage?.key_prefix || 'rate:',
        perSecond: config.strategies.fixed_window.default_limits.per_second,
        perMinute: config.strategies.fixed_window.default_limits.per_minute,
        perHour: config.strategies.fixed_window.default_limits.per_hour,
        perDay: config.strategies.fixed_window.default_limits.per_day
      });
    }
    
    // Create sliding window rate limiter
    if (config.strategies.sliding_window && config.strategies.sliding_window.enabled) {
      limiters.slidingWindow = new SlidingWindowRateLimiter(store, {
        keyPrefix: config.storage?.key_prefix || 'rate:',
        windows: config.strategies.sliding_window.default_windows
      });
    }
    
    // Create token bucket rate limiter
    if (config.strategies.token_bucket && config.strategies.token_bucket.enabled) {
      limiters.tokenBucket = new TokenBucketRateLimiter(store, {
        keyPrefix: config.storage?.key_prefix || 'rate:',
        capacity: config.strategies.token_bucket.default_bucket.capacity,
        refillRate: config.strategies.token_bucket.default_bucket.refill_rate
      });
    }
    
    // Create adaptive rate limiter
    if (config.adaptive && config.adaptive.enabled && limiters.fixedWindow) {
      limiters.adaptive = new AdaptiveRateLimiter(store, {
        keyPrefix: config.storage?.key_prefix || 'rate:',
        baseLimiter: limiters.fixedWindow,
        thresholds: config.adaptive.thresholds,
        adjustmentFactors: config.adaptive.adjustment_factors
      });
    }
    
    // Create multi-level rate limiter
    if (config.multi_level && config.multi_level.enabled) {
      const levels = [];
      
      // Add levels based on configuration
      if (config.multi_level.levels) {
        for (const level of config.multi_level.levels) {
          // Use the appropriate limiter for each level
          const limiter = limiters.fixedWindow || limiters.slidingWindow || limiters.tokenBucket;
          
          if (limiter) {
            levels.push({
              id: level.level,
              priority: level.priority,
              limiter: limiter
            });
          }
        }
        
        // Sort levels by priority
        levels.sort((a, b) => a.priority - b.priority);
      }
      
      limiters.multiLevel = new MultiLevelRateLimiter(store, {
        levels: levels
      });
    }
    
    return limiters;
  }
}
