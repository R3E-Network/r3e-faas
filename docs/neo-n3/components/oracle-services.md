# Oracle Services

This document provides detailed information about the Oracle Services in the Neo N3 FaaS platform.

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Oracle Types](#oracle-types)
4. [JavaScript API](#javascript-api)
5. [Security](#security)
6. [Best Practices](#best-practices)

## Overview

Oracle Services are a core component of the Neo N3 FaaS platform. They provide a secure and reliable way for smart contracts and serverless functions to access external data that is not available on the blockchain. Oracles act as a bridge between the blockchain and the outside world, enabling smart contracts to interact with real-world data.

## Architecture

The Oracle Services follow a modular architecture with several key components:

```
                      +------------------------+
                      |                        |
                      |   Oracle Services      |
                      |                        |
                      +------------+-----------+
                                   |
                                   v
+----------------+    +------------+-----------+    +----------------+
|                |    |                        |    |                |
| Price Feed     |<-->|    Oracle Provider     |<-->| Random Number  |
|                |    |                        |    |                |
+----------------+    +------------+-----------+    +----------------+
                                   |
                                   v
+----------------+    +------------+-----------+    +----------------+
|                |    |                        |    |                |
| Weather        |<-->|    Oracle Auth         |<-->| Sports         |
|                |    |                        |    |                |
+----------------+    +------------+-----------+    +----------------+
                                   |
                                   v
                      +------------+-----------+
                      |                        |
                      |   External Sources     |
                      |                        |
                      +------------------------+
```

- **Oracle Services**: The main component that provides oracle services to the platform.
- **Oracle Provider**: Manages different types of oracle providers.
- **Price Feed**: Provides price data for cryptocurrencies and other assets.
- **Random Number**: Provides secure random number generation.
- **Weather**: Provides weather data for different locations.
- **Sports**: Provides sports results and statistics.
- **Oracle Auth**: Manages authentication and authorization for oracle services.
- **External Sources**: External data sources that the oracles connect to.

## Oracle Types

The Neo N3 FaaS platform supports several types of oracles, each providing different types of data:

### Price Feed Oracle

The Price Feed Oracle provides price data for cryptocurrencies, tokens, and other assets. It connects to multiple external price sources and aggregates the data to provide reliable price information.

```javascript
// Example of using the Price Feed Oracle
import { oracle } from 'r3e';

// Get NEO price in USD
const neoPrice = await oracle.getPrice('NEO', 'USD');
console.log(`NEO price: $${neoPrice.price}`);

// Get GAS price in USD
const gasPrice = await oracle.getPrice('GAS', 'USD');
console.log(`GAS price: $${gasPrice.price}`);

// Get historical price data
const neoPriceHistory = await oracle.getPriceHistory('NEO', 'USD', {
  from: '2023-01-01',
  to: '2023-01-31',
  interval: 'day'
});
console.log(`NEO price history: ${JSON.stringify(neoPriceHistory)}`);
```

The Price Feed Oracle is implemented in the `r3e-oracle/src/provider/price.rs` file:

```rust
// r3e-oracle/src/provider/price.rs
use crate::types::{OracleError, OracleResult, PriceData};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PriceFeedProvider {
    sources: Vec<Box<dyn PriceSource>>,
}

impl PriceFeedProvider {
    pub fn new() -> Self {
        let sources = vec![
            Box::new(CoinGeckoSource::new()),
            Box::new(BinanceSource::new()),
            Box::new(CoinMarketCapSource::new()),
        ];
        
        Self { sources }
    }
    
    pub async fn get_price(&self, asset: &str, currency: &str) -> OracleResult<PriceData> {
        let mut prices = Vec::new();
        
        for source in &self.sources {
            match source.get_price(asset, currency).await {
                Ok(price) => prices.push(price),
                Err(_) => continue,
            }
        }
        
        if prices.is_empty() {
            return Err(OracleError::NoDataAvailable);
        }
        
        // Aggregate prices
        let price = self.aggregate_prices(&prices);
        
        Ok(price)
    }
    
    fn aggregate_prices(&self, prices: &[PriceData]) -> PriceData {
        // Sort prices
        let mut sorted_prices = prices.to_vec();
        sorted_prices.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
        
        // Remove outliers
        let q1_idx = sorted_prices.len() / 4;
        let q3_idx = sorted_prices.len() * 3 / 4;
        let q1 = sorted_prices[q1_idx].price;
        let q3 = sorted_prices[q3_idx].price;
        let iqr = q3 - q1;
        let lower_bound = q1 - 1.5 * iqr;
        let upper_bound = q3 + 1.5 * iqr;
        
        let filtered_prices: Vec<_> = sorted_prices
            .into_iter()
            .filter(|p| p.price >= lower_bound && p.price <= upper_bound)
            .collect();
        
        // Calculate median
        let median_idx = filtered_prices.len() / 2;
        let median_price = filtered_prices[median_idx].price;
        
        // Return median price
        PriceData {
            price: median_price,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

#[async_trait]
trait PriceSource: Send + Sync {
    async fn get_price(&self, asset: &str, currency: &str) -> OracleResult<PriceData>;
}

struct CoinGeckoSource {
    // ...
}

struct BinanceSource {
    // ...
}

struct CoinMarketCapSource {
    // ...
}
```

### Random Number Oracle

The Random Number Oracle provides secure random number generation. It uses a combination of on-chain and off-chain sources of randomness to ensure that the random numbers are unpredictable and cannot be manipulated.

```javascript
// Example of using the Random Number Oracle
import { oracle } from 'r3e';

// Generate a random number between 1 and 100
const randomNumber = await oracle.getRandomNumber(1, 100);
console.log(`Random number: ${randomNumber}`);

// Generate random bytes
const randomBytes = await oracle.getRandomBytes(32);
console.log(`Random bytes: ${randomBytes.toString('hex')}`);

// Generate a random UUID
const randomUUID = await oracle.getRandomUUID();
console.log(`Random UUID: ${randomUUID}`);
```

The Random Number Oracle is implemented in the `r3e-oracle/src/provider/random.rs` file:

```rust
// r3e-oracle/src/provider/random.rs
use crate::types::{OracleError, OracleResult};
use async_trait::async_trait;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct RandomNumberProvider {
    sources: Vec<Box<dyn RandomSource>>,
}

impl RandomNumberProvider {
    pub fn new() -> Self {
        let sources = vec![
            Box::new(SystemRandomSource::new()),
            Box::new(BlockchainRandomSource::new()),
            Box::new(ExternalRandomSource::new()),
        ];
        
        Self { sources }
    }
    
    pub async fn get_random_number(&self, min: u64, max: u64) -> OracleResult<u64> {
        let random_bytes = self.get_random_bytes(8).await?;
        let random_number = u64::from_le_bytes(random_bytes.try_into().unwrap());
        let range = max - min + 1;
        let result = min + (random_number % range);
        
        Ok(result)
    }
    
    pub async fn get_random_bytes(&self, length: usize) -> OracleResult<Vec<u8>> {
        let mut entropy = Vec::new();
        
        for source in &self.sources {
            match source.get_entropy().await {
                Ok(bytes) => entropy.extend_from_slice(&bytes),
                Err(_) => continue,
            }
        }
        
        if entropy.is_empty() {
            return Err(OracleError::NoDataAvailable);
        }
        
        // Hash the entropy
        let mut hasher = Sha256::new();
        hasher.update(&entropy);
        let seed = hasher.finalize();
        
        // Generate random bytes
        let mut rng = ChaCha20Rng::from_seed(seed.into());
        let mut result = vec![0u8; length];
        rng.fill(&mut result[..]);
        
        Ok(result)
    }
}

#[async_trait]
trait RandomSource: Send + Sync {
    async fn get_entropy(&self) -> OracleResult<Vec<u8>>;
}

struct SystemRandomSource {
    // ...
}

struct BlockchainRandomSource {
    // ...
}

struct ExternalRandomSource {
    // ...
}
```

### Weather Oracle

The Weather Oracle provides weather data for different locations. It connects to weather data providers and makes the data available to smart contracts and serverless functions.

```javascript
// Example of using the Weather Oracle
import { oracle } from 'r3e';

// Get weather data for New York
const weather = await oracle.getWeather('New York');
console.log(`Temperature: ${weather.temperature}°C`);
console.log(`Humidity: ${weather.humidity}%`);
console.log(`Wind speed: ${weather.windSpeed} km/h`);

// Get weather forecast for London
const forecast = await oracle.getWeatherForecast('London', {
  days: 5
});
console.log(`Forecast: ${JSON.stringify(forecast)}`);
```

### Sports Oracle

The Sports Oracle provides sports results and statistics. It connects to sports data providers and makes the data available to smart contracts and serverless functions.

```javascript
// Example of using the Sports Oracle
import { oracle } from 'r3e';

// Get NBA results
const nbaResults = await oracle.getSportsResults('NBA');
console.log(`NBA results: ${JSON.stringify(nbaResults)}`);

// Get soccer match details
const matchDetails = await oracle.getSportsMatchDetails('soccer', 'match123');
console.log(`Match details: ${JSON.stringify(matchDetails)}`);
```

## JavaScript API

The Oracle Services provide a JavaScript API that can be used by serverless functions to access oracle data. The API is available through the `oracle` object in the function context.

```javascript
// Example of using the Oracle API in a serverless function
export default async function(event, context) {
  // Get NEO price in USD
  const neoPrice = await context.oracle.getPrice('NEO', 'USD');
  
  // Generate random number
  const randomNumber = await context.oracle.getRandomNumber(1, 100);
  
  // Get weather data
  const weather = await context.oracle.getWeather('New York');
  
  // Get sports results
  const sportsResults = await context.oracle.getSportsResults('NBA');
  
  return {
    neoPrice,
    randomNumber,
    weather,
    sportsResults
  };
}
```

The Oracle JavaScript API is implemented in the `r3e-deno/src/ext/oracle.rs` file and exposed to JavaScript through the `r3e-deno/src/js/oracle.js` file:

```javascript
// r3e-deno/src/js/oracle.js
((globalThis) => {
  const core = Deno.core;
  
  class Oracle {
    async getPrice(asset, currency) {
      const result = await core.ops.op_get_price({ asset, currency });
      return result;
    }
    
    async getPriceHistory(asset, currency, options) {
      const result = await core.ops.op_get_price_history({ asset, currency, options });
      return result;
    }
    
    async getRandomNumber(min, max) {
      const result = await core.ops.op_get_random_number({ min, max });
      return result;
    }
    
    async getRandomBytes(length) {
      const result = await core.ops.op_get_random_bytes({ length });
      return new Uint8Array(result);
    }
    
    async getRandomUUID() {
      const result = await core.ops.op_get_random_uuid({});
      return result;
    }
    
    async getWeather(location) {
      const result = await core.ops.op_get_weather({ location });
      return result;
    }
    
    async getWeatherForecast(location, options) {
      const result = await core.ops.op_get_weather_forecast({ location, options });
      return result;
    }
    
    async getSportsResults(sport) {
      const result = await core.ops.op_get_sports_results({ sport });
      return result;
    }
    
    async getSportsMatchDetails(sport, matchId) {
      const result = await core.ops.op_get_sports_match_details({ sport, matchId });
      return result;
    }
  }
  
  globalThis.r3e = globalThis.r3e || {};
  globalThis.r3e.oracle = new Oracle();
})(globalThis);
```

## Security

The Oracle Services implement several security measures to ensure the integrity and confidentiality of the data:

### Authentication

The Oracle Services use API keys and JWT tokens for authentication. Only authorized users can access the oracle data.

```javascript
// Example of using authentication with the Oracle API
import { oracle } from 'r3e';

// Set API key
oracle.setApiKey('your-api-key');

// Get NEO price in USD
const neoPrice = await oracle.getPrice('NEO', 'USD');
```

The authentication system is implemented in the `r3e-oracle/src/auth.rs` file:

```rust
// r3e-oracle/src/auth.rs
use crate::types::{OracleError, OracleResult};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct OracleAuth {
    api_keys: Arc<RwLock<HashMap<String, ApiKeyData>>>,
    jwt_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApiKeyData {
    user_id: String,
    permissions: Vec<String>,
    rate_limit: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: u64,
    permissions: Vec<String>,
}

impl OracleAuth {
    pub fn new(jwt_secret: &str) -> Self {
        Self {
            api_keys: Arc::new(RwLock::new(HashMap::new())),
            jwt_secret: jwt_secret.to_string(),
        }
    }
    
    pub fn register_api_key(&self, api_key: &str, user_id: &str, permissions: Vec<String>, rate_limit: u64) {
        let mut api_keys = self.api_keys.write().unwrap();
        api_keys.insert(
            api_key.to_string(),
            ApiKeyData {
                user_id: user_id.to_string(),
                permissions,
                rate_limit,
            },
        );
    }
    
    pub fn validate_api_key(&self, api_key: &str) -> OracleResult<ApiKeyData> {
        let api_keys = self.api_keys.read().unwrap();
        
        if let Some(data) = api_keys.get(api_key) {
            Ok(data.clone())
        } else {
            Err(OracleError::InvalidApiKey)
        }
    }
    
    pub fn generate_token(&self, user_id: &str, permissions: Vec<String>, expiration: u64) -> OracleResult<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let claims = Claims {
            sub: user_id.to_string(),
            exp: now + expiration,
            permissions,
        };
        
        let header = Header::new(Algorithm::HS256);
        let token = encode(
            &header,
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|_| OracleError::TokenGenerationFailed)?;
        
        Ok(token)
    }
    
    pub fn validate_token(&self, token: &str) -> OracleResult<Claims> {
        let validation = Validation::new(Algorithm::HS256);
        
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &validation,
        )
        .map_err(|_| OracleError::InvalidToken)?;
        
        Ok(token_data.claims)
    }
}
```

### Data Validation

The Oracle Services validate the data from external sources to ensure its integrity. Data that does not pass validation is rejected.

```rust
// r3e-oracle/src/provider/price.rs
impl PriceFeedProvider {
    // ...
    
    fn validate_price_data(&self, price: &PriceData) -> bool {
        // Check if price is positive
        if price.price <= 0.0 {
            return false;
        }
        
        // Check if timestamp is recent
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        if now - price.timestamp > 3600 {
            return false;
        }
        
        true
    }
}
```

### Rate Limiting

The Oracle Services implement rate limiting to prevent abuse. Users are limited to a certain number of requests per time period.

```rust
// r3e-oracle/src/service.rs
use crate::auth::OracleAuth;
use crate::provider::OracleProvider;
use crate::types::{OracleError, OracleResult};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct OracleService {
    auth: OracleAuth,
    provider: OracleProvider,
    rate_limiter: Arc<Mutex<HashMap<String, RateLimitData>>>,
}

#[derive(Debug)]
struct RateLimitData {
    last_request: Instant,
    request_count: u64,
}

impl OracleService {
    pub fn new(auth: OracleAuth, provider: OracleProvider) -> Self {
        Self {
            auth,
            provider,
            rate_limiter: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub async fn get_price(&self, api_key: &str, asset: &str, currency: &str) -> OracleResult<PriceData> {
        // Validate API key
        let api_key_data = self.auth.validate_api_key(api_key)?;
        
        // Check rate limit
        self.check_rate_limit(api_key, api_key_data.rate_limit)?;
        
        // Get price
        self.provider.get_price(asset, currency).await
    }
    
    fn check_rate_limit(&self, api_key: &str, rate_limit: u64) -> OracleResult<()> {
        let mut rate_limiter = self.rate_limiter.lock().unwrap();
        
        let now = Instant::now();
        let rate_limit_data = rate_limiter.entry(api_key.to_string()).or_insert(RateLimitData {
            last_request: now,
            request_count: 0,
        });
        
        // Reset counter if more than a minute has passed
        if now.duration_since(rate_limit_data.last_request) > Duration::from_secs(60) {
            rate_limit_data.last_request = now;
            rate_limit_data.request_count = 0;
        }
        
        // Check if rate limit is exceeded
        if rate_limit_data.request_count >= rate_limit {
            return Err(OracleError::RateLimitExceeded);
        }
        
        // Increment request count
        rate_limit_data.request_count += 1;
        
        Ok(())
    }
}
```

### Data Aggregation

The Oracle Services aggregate data from multiple sources to provide more reliable data. This helps to filter out outliers and prevent manipulation.

```rust
// r3e-oracle/src/provider/price.rs
impl PriceFeedProvider {
    // ...
    
    fn aggregate_prices(&self, prices: &[PriceData]) -> PriceData {
        // Sort prices
        let mut sorted_prices = prices.to_vec();
        sorted_prices.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
        
        // Remove outliers
        let q1_idx = sorted_prices.len() / 4;
        let q3_idx = sorted_prices.len() * 3 / 4;
        let q1 = sorted_prices[q1_idx].price;
        let q3 = sorted_prices[q3_idx].price;
        let iqr = q3 - q1;
        let lower_bound = q1 - 1.5 * iqr;
        let upper_bound = q3 + 1.5 * iqr;
        
        let filtered_prices: Vec<_> = sorted_prices
            .into_iter()
            .filter(|p| p.price >= lower_bound && p.price <= upper_bound)
            .collect();
        
        // Calculate median
        let median_idx = filtered_prices.len() / 2;
        let median_price = filtered_prices[median_idx].price;
        
        // Return median price
        PriceData {
            price: median_price,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}
```

## Best Practices

When using the Oracle Services, follow these best practices:

### Error Handling

Always handle errors gracefully. Oracle services may be temporarily unavailable or return unexpected data.

```javascript
// Example of error handling
try {
  const neoPrice = await context.oracle.getPrice('NEO', 'USD');
  return { price: neoPrice.price };
} catch (error) {
  console.error(`Error getting NEO price: ${error.message}`);
  return { error: error.message };
}
```

### Caching

Use caching to reduce the number of oracle requests and improve performance. Cache data that does not change frequently.

```javascript
// Example of caching
const cacheKey = 'neo-price';
let neoPrice = await context.cache.get(cacheKey);

if (!neoPrice) {
  neoPrice = await context.oracle.getPrice('NEO', 'USD');
  await context.cache.set(cacheKey, neoPrice, { ttl: 60 }); // Cache for 60 seconds
}

return { price: neoPrice.price };
```

### Retry Logic

Implement retry logic for oracle requests to handle temporary failures.

```javascript
// Example of retry logic
const maxRetries = 3;
let retries = 0;

while (retries < maxRetries) {
  try {
    const neoPrice = await context.oracle.getPrice('NEO', 'USD');
    return { price: neoPrice.price };
  } catch (error) {
    retries++;
    if (retries >= maxRetries) throw error;
    await new Promise(resolve => setTimeout(resolve, 1000 * Math.pow(2, retries)));
  }
}
```

### Data Validation

Validate oracle data before using it to ensure its integrity.

```javascript
// Example of data validation
const neoPrice = await context.oracle.getPrice('NEO', 'USD');

if (neoPrice.price <= 0) {
  throw new Error('Invalid price data');
}

if (Date.now() / 1000 - neoPrice.timestamp > 3600) {
  throw new Error('Price data is too old');
}

return { price: neoPrice.price };
```

### Multiple Sources

Use multiple oracle sources for critical data to ensure reliability.

```javascript
// Example of using multiple sources
const neoPriceA = await context.oracle.getPrice('NEO', 'USD', { source: 'coinGecko' });
const neoPriceB = await context.oracle.getPrice('NEO', 'USD', { source: 'binance' });
const neoPriceC = await context.oracle.getPrice('NEO', 'USD', { source: 'coinMarketCap' });

// Calculate median price
const prices = [neoPriceA.price, neoPriceB.price, neoPriceC.price].sort();
const medianPrice = prices[1];

return { price: medianPrice };
```
