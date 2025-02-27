/**
 * Neo N3 Custom Oracle Service Example
 * 
 * This function demonstrates how to implement a custom oracle service
 * that fetches data from specialized sources and provides it to Neo N3
 * smart contracts in a customized format.
 */

// Import the Neo and Oracle modules from the r3e runtime
import { neo } from 'r3e';
import { oracle } from 'r3e';
import { runlog } from 'r3e';
import { crypto } from 'r3e';

// Import custom oracle service libraries
import { weatherSource } from './lib/weather';
import { sportsSource } from './lib/sports';
import { socialSource } from './lib/social';
import { iotSource } from './lib/iot';
import { customSource } from './lib/custom';

// Define data source types
const DATA_SOURCES = {
  WEATHER: 'weather',
  SPORTS: 'sports',
  SOCIAL: 'social',
  IOT: 'iot',
  CUSTOM: 'custom'
};

/**
 * Main handler function for the custom oracle
 */
export async function handler(event, context) {
  try {
    runlog.info('Custom Oracle Service function triggered');
    
    // Determine if this is a scheduled update or a direct request
    const isScheduled = event.type === 'schedule';
    
    // Parse request parameters
    const requestParams = isScheduled 
      ? { sources: [DATA_SOURCES.WEATHER], format: 'json' } 
      : parseRequestParams(event.data);
    
    // Validate request parameters
    validateRequestParams(requestParams);
    
    // Fetch data from requested sources
    const data = await fetchDataFromSources(requestParams.sources, context);
    
    // Process the data according to request parameters
    const processedData = processData(data, requestParams);
    
    // Generate proof for the data
    const proof = await generateProof(processedData);
    
    // If this is a scheduled update, store the result
    if (isScheduled) {
      await storeOracleResult(processedData, proof, context);
    }
    
    // Return the result
    return {
      status: 'success',
      timestamp: Date.now(),
      data: processedData,
      proof: proof
    };
    
  } catch (error) {
    // Log any errors
    runlog.error('Error in custom oracle service:', error);
    
    // Return error information
    return {
      status: 'error',
      message: `Error in custom oracle service: ${error.message}`,
      error: error.stack
    };
  }
}

/**
 * Parse request parameters from the event data
 */
function parseRequestParams(data) {
  // Default parameters
  const defaultParams = {
    sources: [DATA_SOURCES.WEATHER],
    format: 'json',
    filters: {},
    transformations: []
  };
  
  // Merge with provided parameters
  return { ...defaultParams, ...data };
}

/**
 * Validate request parameters
 */
function validateRequestParams(params) {
  // Check if sources is an array
  if (!Array.isArray(params.sources)) {
    throw new Error('Sources must be an array');
  }
  
  // Check if sources is empty
  if (params.sources.length === 0) {
    throw new Error('At least one source must be specified');
  }
  
  // Check if all sources are valid
  for (const source of params.sources) {
    if (!Object.values(DATA_SOURCES).includes(source)) {
      throw new Error(`Invalid source: ${source}`);
    }
  }
  
  // Check if format is valid
  if (!['json', 'neo', 'xml', 'csv'].includes(params.format)) {
    throw new Error(`Invalid format: ${params.format}`);
  }
}

/**
 * Fetch data from requested sources
 */
async function fetchDataFromSources(sources, context) {
  const result = {};
  
  // Fetch data from each source
  for (const source of sources) {
    try {
      switch (source) {
        case DATA_SOURCES.WEATHER:
          result[source] = await fetchWeatherData(context);
          break;
        case DATA_SOURCES.SPORTS:
          result[source] = await fetchSportsData(context);
          break;
        case DATA_SOURCES.SOCIAL:
          result[source] = await fetchSocialData(context);
          break;
        case DATA_SOURCES.IOT:
          result[source] = await fetchIoTData(context);
          break;
        case DATA_SOURCES.CUSTOM:
          result[source] = await fetchCustomData(context);
          break;
      }
    } catch (error) {
      runlog.error(`Error fetching data from ${source}:`, error);
      result[source] = { error: error.message };
    }
  }
  
  return result;
}

/**
 * Fetch weather data
 */
async function fetchWeatherData(context) {
  // In a real implementation, this would use the weatherSource library
  // For this example, we'll return mock data
  return {
    location: "New York",
    temperature: 22.5,
    condition: "partly cloudy",
    humidity: 65,
    pressure: 1013,
    wind: {
      speed: 5.2,
      direction: "NE"
    },
    timestamp: Date.now()
  };
}

/**
 * Fetch sports data
 */
async function fetchSportsData(context) {
  // In a real implementation, this would use the sportsSource library
  // For this example, we'll return mock data
  return {
    sport: "basketball",
    league: "NBA",
    games: [
      {
        home: "Lakers",
        away: "Warriors",
        score: "105-98",
        status: "final"
      },
      {
        home: "Celtics",
        away: "Nets",
        score: "112-104",
        status: "final"
      }
    ],
    timestamp: Date.now()
  };
}

/**
 * Fetch social media data
 */
async function fetchSocialData(context) {
  // In a real implementation, this would use the socialSource library
  // For this example, we'll return mock data
  return {
    platform: "Twitter",
    trending: ["#Neo", "#Blockchain", "#Web3", "#NFT", "#DeFi"],
    sentiment: {
      neo: 0.85,
      blockchain: 0.72,
      web3: 0.68
    },
    volume: {
      neo: 12500,
      blockchain: 45000,
      web3: 28000
    },
    timestamp: Date.now()
  };
}

/**
 * Fetch IoT data
 */
async function fetchIoTData(context) {
  // In a real implementation, this would use the iotSource library
  // For this example, we'll return mock data
  return {
    network: "SmartCity",
    devices: 5,
    readings: [
      { sensor: "temperature", location: "downtown", value: 24.5 },
      { sensor: "humidity", location: "downtown", value: 58 },
      { sensor: "air_quality", location: "downtown", value: 42 },
      { sensor: "temperature", location: "suburb", value: 22.8 },
      { sensor: "humidity", location: "suburb", value: 62 }
    ],
    timestamp: Date.now()
  };
}

/**
 * Fetch custom data
 */
async function fetchCustomData(context) {
  // In a real implementation, this would use the customSource library
  // For this example, we'll return mock data
  return {
    custom_field_1: "custom_value_1",
    custom_field_2: 42,
    custom_field_3: true,
    custom_field_4: {
      nested_field_1: "nested_value_1",
      nested_field_2: 123
    },
    timestamp: Date.now()
  };
}

/**
 * Process data according to request parameters
 */
function processData(data, params) {
  // Apply filters if provided
  let processedData = applyFilters(data, params.filters);
  
  // Apply transformations if provided
  processedData = applyTransformations(processedData, params.transformations);
  
  // Format the data according to the requested format
  if (params.format === 'neo') {
    // Convert to Neo-compatible format
    return formatForNeo(processedData);
  } else if (params.format === 'xml') {
    // Convert to XML format
    return formatAsXML(processedData);
  } else if (params.format === 'csv') {
    // Convert to CSV format
    return formatAsCSV(processedData);
  }
  
  // Default to JSON format
  return processedData;
}

/**
 * Apply filters to the data
 */
function applyFilters(data, filters) {
  // If no filters, return the original data
  if (!filters || Object.keys(filters).length === 0) {
    return data;
  }
  
  // Clone the data to avoid modifying the original
  const filteredData = JSON.parse(JSON.stringify(data));
  
  // Apply filters to each source
  for (const source in filteredData) {
    // Skip if this source doesn't have filters
    if (!filters[source]) continue;
    
    // Apply source-specific filters
    switch (source) {
      case DATA_SOURCES.WEATHER:
        applyWeatherFilters(filteredData[source], filters[source]);
        break;
      case DATA_SOURCES.SPORTS:
        applySportsFilters(filteredData[source], filters[source]);
        break;
      case DATA_SOURCES.SOCIAL:
        applySocialFilters(filteredData[source], filters[source]);
        break;
      case DATA_SOURCES.IOT:
        applyIoTFilters(filteredData[source], filters[source]);
        break;
      case DATA_SOURCES.CUSTOM:
        applyCustomFilters(filteredData[source], filters[source]);
        break;
    }
  }
  
  return filteredData;
}

/**
 * Apply weather-specific filters
 */
function applyWeatherFilters(data, filters) {
  // Example implementation
  if (filters.location && data.location !== filters.location) {
    data.filtered = true;
  }
  
  if (filters.minTemperature && data.temperature < filters.minTemperature) {
    data.filtered = true;
  }
  
  if (filters.maxTemperature && data.temperature > filters.maxTemperature) {
    data.filtered = true;
  }
}

/**
 * Apply sports-specific filters
 */
function applySportsFilters(data, filters) {
  // Example implementation
  if (filters.sport && data.sport !== filters.sport) {
    data.filtered = true;
  }
  
  if (filters.league && data.league !== filters.league) {
    data.filtered = true;
  }
  
  if (filters.team) {
    data.games = data.games.filter(game => 
      game.home === filters.team || game.away === filters.team
    );
  }
}

/**
 * Apply social media-specific filters
 */
function applySocialFilters(data, filters) {
  // Example implementation
  if (filters.platform && data.platform !== filters.platform) {
    data.filtered = true;
  }
  
  if (filters.minSentiment) {
    for (const topic in data.sentiment) {
      if (data.sentiment[topic] < filters.minSentiment) {
        delete data.sentiment[topic];
      }
    }
  }
}

/**
 * Apply IoT-specific filters
 */
function applyIoTFilters(data, filters) {
  // Example implementation
  if (filters.network && data.network !== filters.network) {
    data.filtered = true;
  }
  
  if (filters.sensor) {
    data.readings = data.readings.filter(reading => 
      reading.sensor === filters.sensor
    );
  }
  
  if (filters.location) {
    data.readings = data.readings.filter(reading => 
      reading.location === filters.location
    );
  }
}

/**
 * Apply custom filters
 */
function applyCustomFilters(data, filters) {
  // Example implementation
  for (const key in filters) {
    if (data[key] !== filters[key]) {
      data.filtered = true;
    }
  }
}

/**
 * Apply transformations to the data
 */
function applyTransformations(data, transformations) {
  // If no transformations, return the original data
  if (!transformations || transformations.length === 0) {
    return data;
  }
  
  // Clone the data to avoid modifying the original
  const transformedData = JSON.parse(JSON.stringify(data));
  
  // Apply each transformation
  for (const transformation of transformations) {
    switch (transformation.type) {
      case 'convert':
        applyConversion(transformedData, transformation);
        break;
      case 'aggregate':
        applyAggregation(transformedData, transformation);
        break;
      case 'enrich':
        applyEnrichment(transformedData, transformation);
        break;
      case 'custom':
        applyCustomTransformation(transformedData, transformation);
        break;
    }
  }
  
  return transformedData;
}

/**
 * Apply conversion transformation
 */
function applyConversion(data, transformation) {
  // Example implementation for temperature conversion
  if (transformation.from === 'celsius' && transformation.to === 'fahrenheit') {
    if (data.weather && data.weather.temperature) {
      data.weather.temperature = (data.weather.temperature * 9/5) + 32;
      data.weather.temperature_unit = 'F';
    }
  }
  
  // Example implementation for wind speed conversion
  if (transformation.from === 'kph' && transformation.to === 'mph') {
    if (data.weather && data.weather.wind && data.weather.wind.speed) {
      data.weather.wind.speed = data.weather.wind.speed / 1.60934;
      data.weather.wind.speed_unit = 'mph';
    }
  }
}

/**
 * Apply aggregation transformation
 */
function applyAggregation(data, transformation) {
  // Example implementation for IoT data aggregation
  if (transformation.source === 'iot' && transformation.operation === 'average') {
    if (data.iot && data.iot.readings) {
      const readings = data.iot.readings;
      const aggregated = {};
      
      // Group readings by sensor
      for (const reading of readings) {
        if (!aggregated[reading.sensor]) {
          aggregated[reading.sensor] = {
            values: [],
            locations: new Set()
          };
        }
        
        aggregated[reading.sensor].values.push(reading.value);
        aggregated[reading.sensor].locations.add(reading.location);
      }
      
      // Calculate averages
      const result = [];
      for (const sensor in aggregated) {
        const values = aggregated[sensor].values;
        const average = values.reduce((sum, val) => sum + val, 0) / values.length;
        
        result.push({
          sensor: sensor,
          value: average,
          locations: Array.from(aggregated[sensor].locations),
          count: values.length
        });
      }
      
      data.iot.aggregated = result;
    }
  }
}

/**
 * Apply enrichment transformation
 */
function applyEnrichment(data, transformation) {
  // Example implementation for weather data enrichment
  if (transformation.source === 'weather' && transformation.operation === 'add_forecast') {
    if (data.weather) {
      // Add mock forecast data
      data.weather.forecast = [
        { day: 'tomorrow', temperature: data.weather.temperature + 2, condition: 'sunny' },
        { day: 'day_after', temperature: data.weather.temperature - 1, condition: 'rainy' }
      ];
    }
  }
  
  // Example implementation for sports data enrichment
  if (transformation.source === 'sports' && transformation.operation === 'add_standings') {
    if (data.sports) {
      // Add mock standings data
      data.sports.standings = [
        { team: 'Lakers', wins: 42, losses: 30 },
        { team: 'Warriors', wins: 45, losses: 27 },
        { team: 'Celtics', wins: 48, losses: 24 },
        { team: 'Nets', wins: 38, losses: 34 }
      ];
    }
  }
}

/**
 * Apply custom transformation
 */
function applyCustomTransformation(data, transformation) {
  // Example implementation
  if (transformation.code) {
    try {
      // In a real implementation, this would use a sandboxed evaluation
      // For this example, we'll just add a note
      data.custom_transformation = {
        applied: true,
        name: transformation.name || 'unnamed',
        timestamp: Date.now()
      };
    } catch (error) {
      runlog.error('Error applying custom transformation:', error);
    }
  }
}

/**
 * Format data for Neo N3 smart contracts
 */
function formatForNeo(data) {
  // Convert data to a format that Neo N3 smart contracts can use
  const neoData = {};
  
  for (const source in data) {
    // Convert complex objects to strings
    if (typeof data[source] === 'object') {
      neoData[source] = JSON.stringify(data[source]);
    } else {
      neoData[source] = data[source];
    }
  }
  
  return {
    format: 'neo',
    data: neoData,
    timestamp: Date.now()
  };
}

/**
 * Format data as XML
 */
function formatAsXML(data) {
  // In a real implementation, this would convert the data to XML
  // For this example, we'll just return a mock XML string
  return {
    format: 'xml',
    data: `<oracle><timestamp>${Date.now()}</timestamp><data>${JSON.stringify(data)}</data></oracle>`,
    timestamp: Date.now()
  };
}

/**
 * Format data as CSV
 */
function formatAsCSV(data) {
  // In a real implementation, this would convert the data to CSV
  // For this example, we'll just return a mock CSV string
  let csv = 'source,key,value\n';
  
  for (const source in data) {
    if (typeof data[source] === 'object') {
      for (const key in data[source]) {
        const value = typeof data[source][key] === 'object' 
          ? JSON.stringify(data[source][key]) 
          : data[source][key];
        
        csv += `${source},${key},${value}\n`;
      }
    } else {
      csv += `${source},"value",${data[source]}\n`;
    }
  }
  
  return {
    format: 'csv',
    data: csv,
    timestamp: Date.now()
  };
}

/**
 * Generate proof for the data
 */
async function generateProof(data) {
  // Create a hash of the data
  const dataString = JSON.stringify(data);
  const dataHash = await crypto.sha256(dataString);
  
  // Sign the hash with the oracle's private key
  const signature = await crypto.sign(dataHash, 'oracle-custom');
  
  return {
    hash: dataHash,
    signature: signature,
    timestamp: Date.now()
  };
}

/**
 * Store oracle result in the persistent storage
 */
async function storeOracleResult(data, proof, context) {
  try {
    // Create a unique key for this result
    const key = `oracle:custom:${Date.now()}`;
    
    // Store the result
    await context.store.set(key, JSON.stringify({
      data: data,
      proof: proof,
      timestamp: Date.now()
    }));
    
    // Update the oracle history index
    const historyKey = 'oracle:custom:history';
    const historyJson = await context.store.get(historyKey) || '[]';
    const history = JSON.parse(historyJson);
    
    // Add the new key to the history
    history.push(key);
    
    // Keep only the last 100 entries
    if (history.length > 100) {
      history.shift();
    }
    
    // Save the updated history
    await context.store.set(historyKey, JSON.stringify(history));
    
    runlog.info('Oracle result stored successfully');
  } catch (error) {
    runlog.error('Error storing oracle result:', error);
    throw error;
  }
}
