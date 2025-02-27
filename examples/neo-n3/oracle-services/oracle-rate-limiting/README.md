# Neo N3 Oracle Rate Limiting Example

This example demonstrates how to implement effective rate limiting for oracle services in the Neo N3 FaaS platform. Rate limiting is essential for protecting oracle services from abuse, ensuring fair usage, and maintaining service stability.

## Overview

The oracle rate limiting example shows how to:

1. Implement multiple rate limiting strategies for oracle services
2. Configure rate limits based on different criteria (user, IP, resource)
3. Handle rate limit exceeded scenarios gracefully
4. Monitor and analyze rate limiting metrics
5. Implement adaptive rate limiting based on system load

## Files

- `function.js`: The JavaScript function that implements the oracle rate limiting service
- `config.yaml`: Configuration file for the oracle rate limiting service
- `register.js`: Script to register the function with the FaaS platform
- `smart-contract/`: Directory containing a sample Neo N3 smart contract that interacts with rate-limited oracle services
- `lib/`: Directory containing rate limiting libraries and utilities

## Prerequisites

- Neo N3 FaaS platform installed and running
- Access to a Neo N3 node (testnet or mainnet)
- Node.js installed for running the registration script
- Neo N3 smart contract development environment (optional, for testing the smart contract)

## Setup

1. Configure your Neo N3 node connection and rate limiting settings in `config.yaml`
2. Register the function using the registration script:

```bash
node register.js
```

3. (Optional) Deploy the sample smart contract to interact with the rate-limited oracle service

## How It Works

The oracle rate limiting service implements several rate limiting strategies to protect oracle services from abuse and ensure fair usage. The service can be configured to use one or more rate limiting strategies based on requirements.

### Rate Limiting Strategies

The example demonstrates several rate limiting strategies:

#### 1. Fixed Window Rate Limiting

The simplest form of rate limiting, where a fixed number of requests are allowed within a fixed time window (e.g., 100 requests per minute).

```javascript
// Example fixed window rate limiting
const currentMinute = Math.floor(Date.now() / 60000);
const key = `rate:${userId}:${currentMinute}`;
const count = await incrementCounter(key, 60);
if (count > limits.perMinute) {
  return { error: 'Rate limit exceeded', retry_after: 60 - (Date.now() % 60000) / 1000 };
}
```

#### 2. Sliding Window Rate Limiting

A more sophisticated approach that considers a sliding time window, providing smoother rate limiting (e.g., 100 requests per 60-second window, calculated continuously).

```javascript
// Example sliding window rate limiting
const now = Date.now();
const windowStart = now - (limits.window * 1000);
const requests = await getRequestsInTimeRange(userId, windowStart, now);
if (requests.length >= limits.count) {
  const oldestRequest = requests[0];
  const retryAfter = Math.ceil((oldestRequest.timestamp + (limits.window * 1000) - now) / 1000);
  return { error: 'Rate limit exceeded', retry_after: retryAfter };
}
```

#### 3. Token Bucket Rate Limiting

A flexible approach that allows for bursts of traffic while maintaining a long-term rate limit. Each user has a bucket of tokens that refills at a constant rate.

```javascript
// Example token bucket rate limiting
const bucket = await getTokenBucket(userId);
const now = Date.now();
const elapsedSeconds = (now - bucket.lastRefill) / 1000;
const refillAmount = elapsedSeconds * limits.refillRate;
bucket.tokens = Math.min(bucket.tokens + refillAmount, limits.bucketSize);
bucket.lastRefill = now;

if (bucket.tokens < 1) {
  const retryAfter = Math.ceil((1 - bucket.tokens) / limits.refillRate);
  return { error: 'Rate limit exceeded', retry_after: retryAfter };
}

bucket.tokens -= 1;
await saveTokenBucket(userId, bucket);
```

### Multi-Level Rate Limiting

The example also demonstrates multi-level rate limiting, where different limits are applied at different levels:

```javascript
// Example multi-level rate limiting
const levels = [
  { type: 'user', id: userId, limits: { perSecond: 5, perMinute: 100, perHour: 1000 } },
  { type: 'ip', id: ip, limits: { perSecond: 10, perMinute: 200, perHour: 2000 } },
  { type: 'resource', id: resource, limits: { perSecond: 100, perMinute: 1000, perHour: 10000 } }
];

for (const level of levels) {
  const result = await checkRateLimit(level.id, level.limits);
  if (!result.success) {
    return { error: `Rate limit exceeded for ${level.type}`, retry_after: result.retry_after };
  }
}
```

### Adaptive Rate Limiting

The example includes adaptive rate limiting, where rate limits are adjusted based on system load:

```javascript
// Example adaptive rate limiting
const systemLoad = await getSystemLoad();
const adaptiveFactor = calculateAdaptiveFactor(systemLoad);
const adjustedLimit = Math.floor(baseLimit * adaptiveFactor);

if (count > adjustedLimit) {
  return { error: 'Rate limit exceeded due to high system load', retry_after: 30 };
}
```

### Rate Limit Headers

The service includes standard rate limit headers in responses:

```javascript
// Example rate limit headers
const headers = {
  'X-RateLimit-Limit': limits.perMinute,
  'X-RateLimit-Remaining': Math.max(0, limits.perMinute - count),
  'X-RateLimit-Reset': Math.ceil(60 - (Date.now() % 60000) / 1000)
};
```

## Customization

You can customize this example by:

- Adjusting rate limits for different users or resources
- Implementing additional rate limiting strategies
- Modifying the response format for rate limit exceeded scenarios
- Extending the monitoring and analytics capabilities

## Additional Resources

- [Neo N3 Oracle Services Documentation](../../docs/neo-n3/components/oracle-services.md)
- [Neo N3 FaaS Platform Documentation](../../docs/neo-n3/README.md)
- [JavaScript Function Development Guide](../../docs/neo-n3/guides/function-development.md)
- [Neo N3 Smart Contract Development Guide](https://docs.neo.org/docs/en-us/develop/write/basics.html)
