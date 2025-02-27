# Oracle Service Example for Neo N3 FaaS Platform

This example demonstrates how to create, deploy, and use oracle services on the Neo N3 FaaS platform.

## Prerequisites

Before you begin, ensure you have the following:

- Neo N3 FaaS CLI installed (`npm install -g r3e-faas-cli`)
- A Neo N3 FaaS account
- Basic knowledge of JavaScript and Neo N3 blockchain

## Introduction to Oracle Services

Oracle services provide a secure and reliable way to access external data from within your Neo N3 FaaS functions. The platform offers built-in oracle services for common data needs, such as:

- Price feeds for cryptocurrencies and traditional assets
- Random number generation
- Weather data
- Sports results
- Financial data

You can also create custom oracle services to provide specific data for your applications.

## Project Setup

1. Create a new project directory:

```bash
mkdir neo-oracle-example
cd neo-oracle-example
```

2. Initialize a new Neo N3 FaaS project:

```bash
r3e-faas-cli init
```

This command creates the following files:

```
neo-oracle-example/
├── functions/
│   └── hello.js
├── r3e.yaml
└── package.json
```

## Using Built-in Oracle Services

### Price Feed Oracle

Create a new file `functions/price-feed.js` with the following code:

```javascript
/**
 * A function that uses the price feed oracle service
 * 
 * @param {Object} event - The event object
 * @param {Object} context - The context object
 * @returns {Object} - The response object
 */
export default async function(event, context) {
  // Log the event for debugging
  console.log('Event received:', event);
  
  // Get the asset and currency from the event parameters or use defaults
  const asset = event.params?.asset || 'NEO';
  const currency = event.params?.currency || 'USD';
  
  // Get the price from the oracle service
  let price;
  try {
    price = await context.oracle.getPrice(asset, currency);
    console.log(`${asset}/${currency} price:`, price);
  } catch (error) {
    console.error(`Error getting ${asset}/${currency} price:`, error);
    return {
      error: `Failed to get ${asset}/${currency} price: ${error.message}`
    };
  }
  
  // Get historical prices
  let historicalPrices;
  try {
    historicalPrices = await context.oracle.getHistoricalPrices(asset, currency, {
      interval: 'daily',
      limit: 7
    });
    console.log(`${asset}/${currency} historical prices:`, historicalPrices);
  } catch (error) {
    console.error(`Error getting ${asset}/${currency} historical prices:`, error);
    historicalPrices = { error: error.message };
  }
  
  // Calculate price change
  let priceChange = null;
  if (historicalPrices.prices && historicalPrices.prices.length > 1) {
    const latestPrice = historicalPrices.prices[0].price;
    const oldestPrice = historicalPrices.prices[historicalPrices.prices.length - 1].price;
    priceChange = {
      absolute: latestPrice - oldestPrice,
      percentage: ((latestPrice - oldestPrice) / oldestPrice) * 100
    };
  }
  
  // Return the price information
  return {
    asset,
    currency,
    price,
    timestamp: new Date().toISOString(),
    historicalPrices: historicalPrices.prices || [],
    priceChange
  };
}
```

This function:
1. Receives an event object with optional parameters for asset and currency
2. Gets the current price of the asset in the specified currency using the oracle service
3. Gets historical prices for the asset
4. Calculates the price change over the historical period
5. Returns the price information

### Random Number Oracle

Create a new file `functions/random-number.js` with the following code:

```javascript
/**
 * A function that uses the random number oracle service
 * 
 * @param {Object} event - The event object
 * @param {Object} context - The context object
 * @returns {Object} - The response object
 */
export default async function(event, context) {
  // Log the event for debugging
  console.log('Event received:', event);
  
  // Get the parameters from the event or use defaults
  const min = event.params?.min !== undefined ? parseInt(event.params.min) : 1;
  const max = event.params?.max !== undefined ? parseInt(event.params.max) : 100;
  const count = event.params?.count !== undefined ? parseInt(event.params.count) : 1;
  
  // Validate parameters
  if (isNaN(min) || isNaN(max) || isNaN(count)) {
    return {
      error: 'Invalid parameters. min, max, and count must be numbers.'
    };
  }
  
  if (min >= max) {
    return {
      error: 'Invalid parameters. min must be less than max.'
    };
  }
  
  if (count < 1 || count > 100) {
    return {
      error: 'Invalid parameters. count must be between 1 and 100.'
    };
  }
  
  // Get random numbers from the oracle service
  let randomNumbers;
  try {
    if (count === 1) {
      // Get a single random number
      const randomNumber = await context.oracle.getRandomNumber(min, max);
      console.log(`Random number between ${min} and ${max}:`, randomNumber);
      randomNumbers = [randomNumber];
    } else {
      // Get multiple random numbers
      randomNumbers = await context.oracle.getRandomNumbers(min, max, count);
      console.log(`${count} random numbers between ${min} and ${max}:`, randomNumbers);
    }
  } catch (error) {
    console.error('Error getting random numbers:', error);
    return {
      error: `Failed to get random numbers: ${error.message}`
    };
  }
  
  // Return the random numbers
  return {
    min,
    max,
    count,
    randomNumbers,
    timestamp: new Date().toISOString()
  };
}
```

This function:
1. Receives an event object with optional parameters for min, max, and count
2. Validates the parameters
3. Gets random numbers from the oracle service
4. Returns the random numbers

### Weather Oracle

Create a new file `functions/weather.js` with the following code:

```javascript
/**
 * A function that uses the weather oracle service
 * 
 * @param {Object} event - The event object
 * @param {Object} context - The context object
 * @returns {Object} - The response object
 */
export default async function(event, context) {
  // Log the event for debugging
  console.log('Event received:', event);
  
  // Get the location from the event parameters or use a default
  const location = event.params?.location || 'New York, NY';
  
  // Get the current weather from the oracle service
  let currentWeather;
  try {
    currentWeather = await context.oracle.getWeather(location);
    console.log(`Current weather in ${location}:`, currentWeather);
  } catch (error) {
    console.error(`Error getting weather for ${location}:`, error);
    return {
      error: `Failed to get weather for ${location}: ${error.message}`
    };
  }
  
  // Get the weather forecast from the oracle service
  let forecast;
  try {
    forecast = await context.oracle.getWeatherForecast(location, {
      days: 5
    });
    console.log(`Weather forecast for ${location}:`, forecast);
  } catch (error) {
    console.error(`Error getting weather forecast for ${location}:`, error);
    forecast = { error: error.message };
  }
  
  // Return the weather information
  return {
    location,
    currentWeather,
    forecast: forecast.days || [],
    timestamp: new Date().toISOString()
  };
}
```

This function:
1. Receives an event object with an optional parameter for location
2. Gets the current weather for the location using the oracle service
3. Gets the weather forecast for the location
4. Returns the weather information

## Configuration

Open the `r3e.yaml` file and update it with the following configuration:

```yaml
project:
  name: neo-oracle-example
  version: 0.1.0

functions:
  price-feed:
    handler: functions/price-feed.js
    runtime: javascript
    trigger:
      type: http
      path: /price-feed
      method: get
    environment:
      NODE_ENV: production
  
  random-number:
    handler: functions/random-number.js
    runtime: javascript
    trigger:
      type: http
      path: /random-number
      method: get
    environment:
      NODE_ENV: production
  
  weather:
    handler: functions/weather.js
    runtime: javascript
    trigger:
      type: http
      path: /weather
      method: get
    environment:
      NODE_ENV: production
```

This configuration:
1. Sets the project name and version
2. Defines three functions: price-feed, random-number, and weather
3. Specifies the handler file path for each function
4. Sets the runtime to JavaScript for each function
5. Configures HTTP triggers for each function
6. Sets the `NODE_ENV` environment variable to `production` for each function

## Local Testing

Before deploying, test the functions locally:

### Test Price Feed Function

```bash
r3e-faas-cli invoke-local --function price-feed
```

With parameters:

```bash
r3e-faas-cli invoke-local --function price-feed --params '{"asset": "GAS", "currency": "EUR"}'
```

### Test Random Number Function

```bash
r3e-faas-cli invoke-local --function random-number
```

With parameters:

```bash
r3e-faas-cli invoke-local --function random-number --params '{"min": 1, "max": 1000, "count": 5}'
```

### Test Weather Function

```bash
r3e-faas-cli invoke-local --function weather
```

With parameters:

```bash
r3e-faas-cli invoke-local --function weather --params '{"location": "London, UK"}'
```

## Deployment

Deploy the functions to the Neo N3 FaaS platform:

```bash
r3e-faas-cli deploy
```

This command:
1. Packages the function code
2. Uploads it to the Neo N3 FaaS platform
3. Creates the necessary resources
4. Configures the HTTP endpoints

After successful deployment, you should see output similar to:

```
Deploying project: neo-oracle-example
Deploying function: price-feed... Done!
Deploying function: random-number... Done!
Deploying function: weather... Done!
Function URLs:
- price-feed: https://faas.example.com/functions/price-feed
- random-number: https://faas.example.com/functions/random-number
- weather: https://faas.example.com/functions/weather
```

## Invoking the Functions

### Using curl

```bash
# Get NEO price in USD
curl https://faas.example.com/functions/price-feed

# Get GAS price in EUR
curl https://faas.example.com/functions/price-feed?asset=GAS&currency=EUR

# Get a random number between 1 and 100
curl https://faas.example.com/functions/random-number

# Get 5 random numbers between 1 and 1000
curl https://faas.example.com/functions/random-number?min=1&max=1000&count=5

# Get weather for New York
curl https://faas.example.com/functions/weather

# Get weather for London
curl https://faas.example.com/functions/weather?location=London,%20UK
```

### Using the CLI

```bash
# Get NEO price in USD
r3e-faas-cli invoke --function price-feed

# Get GAS price in EUR
r3e-faas-cli invoke --function price-feed --params '{"asset": "GAS", "currency": "EUR"}'

# Get a random number between 1 and 100
r3e-faas-cli invoke --function random-number

# Get 5 random numbers between 1 and 1000
r3e-faas-cli invoke --function random-number --params '{"min": 1, "max": 1000, "count": 5}'

# Get weather for New York
r3e-faas-cli invoke --function weather

# Get weather for London
r3e-faas-cli invoke --function weather --params '{"location": "London, UK"}'
```

## Creating a Custom Oracle Service

In addition to using the built-in oracle services, you can create your own custom oracle service to provide specific data for your applications.

### Custom Oracle Service Implementation

Create a new file `services/custom-oracle.js` with the following code:

```javascript
/**
 * A custom oracle service that provides stock market data
 * 
 * @param {Object} context - The context object
 * @returns {Object} - The service object
 */
export default function(context) {
  // Initialize the service
  console.log('Initializing custom oracle service');
  
  // Define the service API
  const service = {
    /**
     * Get the current stock price
     * 
     * @param {string} symbol - The stock symbol
     * @returns {Promise<number>} - The stock price
     */
    getStockPrice: async (symbol) => {
      console.log(`Getting stock price for ${symbol}`);
      
      // In a real implementation, you would fetch the data from an external API
      // For this example, we'll simulate the API call
      
      // Validate the symbol
      if (!symbol || typeof symbol !== 'string') {
        throw new Error('Invalid symbol');
      }
      
      // Simulate API call delay
      await new Promise(resolve => setTimeout(resolve, 500));
      
      // Generate a random price based on the symbol
      const basePrice = symbol.split('').reduce((sum, char) => sum + char.charCodeAt(0), 0) % 1000;
      const randomFactor = 0.9 + (Math.random() * 0.2); // Random factor between 0.9 and 1.1
      const price = basePrice * randomFactor;
      
      return parseFloat(price.toFixed(2));
    },
    
    /**
     * Get historical stock prices
     * 
     * @param {string} symbol - The stock symbol
     * @param {Object} options - The options
     * @param {string} options.interval - The interval (daily, weekly, monthly)
     * @param {number} options.limit - The number of data points
     * @returns {Promise<Object>} - The historical prices
     */
    getHistoricalStockPrices: async (symbol, options = {}) => {
      console.log(`Getting historical stock prices for ${symbol}`, options);
      
      // In a real implementation, you would fetch the data from an external API
      // For this example, we'll simulate the API call
      
      // Validate the symbol
      if (!symbol || typeof symbol !== 'string') {
        throw new Error('Invalid symbol');
      }
      
      // Set default options
      const interval = options.interval || 'daily';
      const limit = options.limit || 30;
      
      // Validate options
      if (!['daily', 'weekly', 'monthly'].includes(interval)) {
        throw new Error('Invalid interval. Must be daily, weekly, or monthly.');
      }
      
      if (limit < 1 || limit > 365) {
        throw new Error('Invalid limit. Must be between 1 and 365.');
      }
      
      // Simulate API call delay
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      // Generate random historical prices
      const basePrice = symbol.split('').reduce((sum, char) => sum + char.charCodeAt(0), 0) % 1000;
      const prices = [];
      
      const now = new Date();
      let currentDate = new Date();
      
      for (let i = 0; i < limit; i++) {
        // Adjust date based on interval
        if (interval === 'daily') {
          currentDate = new Date(now.getTime() - (i * 24 * 60 * 60 * 1000));
        } else if (interval === 'weekly') {
          currentDate = new Date(now.getTime() - (i * 7 * 24 * 60 * 60 * 1000));
        } else if (interval === 'monthly') {
          const newDate = new Date(now);
          newDate.setMonth(now.getMonth() - i);
          currentDate = newDate;
        }
        
        // Generate a random price
        const randomFactor = 0.8 + (Math.random() * 0.4); // Random factor between 0.8 and 1.2
        const price = basePrice * randomFactor;
        
        prices.push({
          date: currentDate.toISOString().split('T')[0],
          price: parseFloat(price.toFixed(2))
        });
      }
      
      return {
        symbol,
        interval,
        prices
      };
    },
    
    /**
     * Get company information
     * 
     * @param {string} symbol - The stock symbol
     * @returns {Promise<Object>} - The company information
     */
    getCompanyInfo: async (symbol) => {
      console.log(`Getting company info for ${symbol}`);
      
      // In a real implementation, you would fetch the data from an external API
      // For this example, we'll simulate the API call
      
      // Validate the symbol
      if (!symbol || typeof symbol !== 'string') {
        throw new Error('Invalid symbol');
      }
      
      // Simulate API call delay
      await new Promise(resolve => setTimeout(resolve, 800));
      
      // Define some example company information
      const companies = {
        'AAPL': {
          name: 'Apple Inc.',
          sector: 'Technology',
          industry: 'Consumer Electronics',
          description: 'Apple Inc. designs, manufactures, and markets smartphones, personal computers, tablets, wearables, and accessories worldwide.'
        },
        'MSFT': {
          name: 'Microsoft Corporation',
          sector: 'Technology',
          industry: 'Software',
          description: 'Microsoft Corporation develops, licenses, and supports software, services, devices, and solutions worldwide.'
        },
        'GOOGL': {
          name: 'Alphabet Inc.',
          sector: 'Technology',
          industry: 'Internet Content & Information',
          description: 'Alphabet Inc. provides various products and platforms in the United States, Europe, the Middle East, Africa, the Asia-Pacific, Canada, and Latin America.'
        },
        'AMZN': {
          name: 'Amazon.com, Inc.',
          sector: 'Consumer Cyclical',
          industry: 'Internet Retail',
          description: 'Amazon.com, Inc. engages in the retail sale of consumer products and subscriptions in North America and internationally.'
        }
      };
      
      // Return company information or a generic one if not found
      return companies[symbol] || {
        name: `${symbol} Corporation`,
        sector: 'Unknown',
        industry: 'Unknown',
        description: `${symbol} is a publicly traded company.`
      };
    }
  };
  
  return service;
}
```

This custom oracle service:
1. Provides methods for getting stock prices, historical stock prices, and company information
2. Simulates API calls to external data sources
3. Returns the requested data

### Function Using Custom Oracle Service

Create a new file `functions/stock-info.js` with the following code:

```javascript
/**
 * A function that uses the custom stock oracle service
 * 
 * @param {Object} event - The event object
 * @param {Object} context - The context object
 * @returns {Object} - The response object
 */
export default async function(event, context) {
  // Log the event for debugging
  console.log('Event received:', event);
  
  // Get the stock symbol from the event parameters or use a default
  const symbol = event.params?.symbol || 'AAPL';
  
  // Get the custom oracle service
  const stockOracle = context.services.customOracle;
  
  if (!stockOracle) {
    return {
      error: 'Custom oracle service not available'
    };
  }
  
  // Get the stock price
  let stockPrice;
  try {
    stockPrice = await stockOracle.getStockPrice(symbol);
    console.log(`${symbol} stock price:`, stockPrice);
  } catch (error) {
    console.error(`Error getting ${symbol} stock price:`, error);
    return {
      error: `Failed to get ${symbol} stock price: ${error.message}`
    };
  }
  
  // Get the company information
  let companyInfo;
  try {
    companyInfo = await stockOracle.getCompanyInfo(symbol);
    console.log(`${symbol} company info:`, companyInfo);
  } catch (error) {
    console.error(`Error getting ${symbol} company info:`, error);
    companyInfo = { error: error.message };
  }
  
  // Get historical stock prices
  let historicalPrices;
  try {
    historicalPrices = await stockOracle.getHistoricalStockPrices(symbol, {
      interval: 'daily',
      limit: 7
    });
    console.log(`${symbol} historical prices:`, historicalPrices);
  } catch (error) {
    console.error(`Error getting ${symbol} historical prices:`, error);
    historicalPrices = { error: error.message };
  }
  
  // Return the stock information
  return {
    symbol,
    price: stockPrice,
    company: companyInfo,
    historicalPrices: historicalPrices.prices || [],
    timestamp: new Date().toISOString()
  };
}
```

This function:
1. Receives an event object with an optional parameter for the stock symbol
2. Gets the custom oracle service from the context
3. Gets the stock price, company information, and historical prices using the custom oracle service
4. Returns the stock information

### Update Configuration

Update the `r3e.yaml` file to include the custom oracle service and the stock-info function:

```yaml
project:
  name: neo-oracle-example
  version: 0.1.0

functions:
  price-feed:
    handler: functions/price-feed.js
    runtime: javascript
    trigger:
      type: http
      path: /price-feed
      method: get
    environment:
      NODE_ENV: production
  
  random-number:
    handler: functions/random-number.js
    runtime: javascript
    trigger:
      type: http
      path: /random-number
      method: get
    environment:
      NODE_ENV: production
  
  weather:
    handler: functions/weather.js
    runtime: javascript
    trigger:
      type: http
      path: /weather
      method: get
    environment:
      NODE_ENV: production
  
  stock-info:
    handler: functions/stock-info.js
    runtime: javascript
    trigger:
      type: http
      path: /stock-info
      method: get
    environment:
      NODE_ENV: production
    services:
      - custom-oracle

services:
  custom-oracle:
    handler: services/custom-oracle.js
    type: oracle
    config:
      updateInterval: 60
```

This configuration:
1. Adds a new function `stock-info` that uses the custom oracle service
2. Defines a custom oracle service `custom-oracle` with its handler and configuration

## Testing the Custom Oracle Service

Test the stock-info function locally:

```bash
r3e-faas-cli invoke-local --function stock-info
```

With parameters:

```bash
r3e-faas-cli invoke-local --function stock-info --params '{"symbol": "MSFT"}'
```

## Deploying the Custom Oracle Service

Deploy the updated project to the Neo N3 FaaS platform:

```bash
r3e-faas-cli deploy
```

This command:
1. Packages the function and service code
2. Uploads it to the Neo N3 FaaS platform
3. Creates the necessary resources
4. Configures the HTTP endpoints

After successful deployment, you should see output similar to:

```
Deploying project: neo-oracle-example
Deploying service: custom-oracle... Done!
Deploying function: price-feed... Done!
Deploying function: random-number... Done!
Deploying function: weather... Done!
Deploying function: stock-info... Done!
Function URLs:
- price-feed: https://faas.example.com/functions/price-feed
- random-number: https://faas.example.com/functions/random-number
- weather: https://faas.example.com/functions/weather
- stock-info: https://faas.example.com/functions/stock-info
```

## Invoking the Stock Info Function

### Using curl

```bash
# Get Apple stock information
curl https://faas.example.com/functions/stock-info

# Get Microsoft stock information
curl https://faas.example.com/functions/stock-info?symbol=MSFT
```

### Using the CLI

```bash
# Get Apple stock information
r3e-faas-cli invoke --function stock-info

# Get Microsoft stock information
r3e-faas-cli invoke --function stock-info --params '{"symbol": "MSFT"}'
```

## Monitoring

View the function logs:

```bash
r3e-faas-cli logs --function stock-info
```

To follow the logs in real-time:

```bash
r3e-faas-cli logs --function stock-info --follow
```

## Cleaning Up

To remove the functions and services:

```bash
r3e-faas-cli remove
```

## Next Steps

Now that you've created, deployed, and used oracle services on the Neo N3 FaaS platform, you can:

1. Create more complex oracle services for specific data needs
2. Combine oracle services with Neo N3 blockchain events
3. Implement secure computing with TEE services
4. Create a custom dashboard to visualize the oracle data

For more examples, see:
- [TEE Service Example](./tee-service.md)
- [Service API Example](./service-api.md)

For more information, see the [Documentation](../README.md).
