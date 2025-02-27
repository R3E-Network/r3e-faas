# Neo N3 Price Feed Oracle Example

This example demonstrates how to use the Neo N3 FaaS platform's oracle services to fetch and provide price feed data to Neo N3 smart contracts and functions.

## Overview

The price feed oracle example shows how to:

1. Create a function that fetches price data from external sources
2. Provide this data to Neo N3 smart contracts in a secure and reliable way
3. Configure the oracle service with different data sources and update frequencies
4. Implement error handling and fallback mechanisms
5. Verify the authenticity of the price data

## Files

- `function.js`: The JavaScript function that implements the price feed oracle
- `config.yaml`: Configuration file for the price feed oracle
- `register.js`: Script to register the function with the FaaS platform
- `smart-contract/`: Directory containing a sample Neo N3 smart contract that uses the price feed

## Prerequisites

- Neo N3 FaaS platform installed and running
- Access to a Neo N3 node (testnet or mainnet)
- Node.js installed for running the registration script
- Neo N3 smart contract development environment (optional, for testing the smart contract)

## Setup

1. Configure your Neo N3 node connection and price feed sources in `config.yaml`
2. Register the function using the registration script:

```bash
node register.js
```

3. (Optional) Deploy the sample smart contract to interact with the price feed oracle

## How It Works

The price feed oracle function fetches price data from multiple sources, aggregates it, and provides it to Neo N3 smart contracts. The function is triggered on a schedule or on-demand when a smart contract requests price data.

### Data Sources

The example demonstrates fetching price data from multiple sources:

- Cryptocurrency exchanges (e.g., Binance, Coinbase)
- Financial data providers (e.g., CoinGecko, CoinMarketCap)
- Traditional finance APIs (e.g., Yahoo Finance, Alpha Vantage)

### Aggregation Methods

The example implements several aggregation methods:

- Simple average
- Volume-weighted average price (VWAP)
- Median price
- Trimmed mean (removing outliers)

### Security Features

The example includes several security features:

- Data source authentication
- Outlier detection and removal
- Timestamp validation
- Digital signatures for data integrity
- Rate limiting to prevent abuse

## Smart Contract Integration

The sample smart contract demonstrates how to:

1. Request price data from the oracle
2. Verify the authenticity of the data
3. Use the price data in contract logic (e.g., for a decentralized exchange)

## Customization

You can customize this example by:

- Adding additional price data sources
- Implementing different aggregation methods
- Modifying the update frequency
- Extending the smart contract to use the price data in different ways

## Additional Resources

- [Neo N3 Oracle Services Documentation](../../docs/neo-n3/components/oracle-services.md)
- [Neo N3 FaaS Platform Documentation](../../docs/neo-n3/README.md)
- [JavaScript Function Development Guide](../../docs/neo-n3/guides/function-development.md)
- [Neo N3 Smart Contract Development Guide](https://docs.neo.org/docs/en-us/develop/write/basics.html)
