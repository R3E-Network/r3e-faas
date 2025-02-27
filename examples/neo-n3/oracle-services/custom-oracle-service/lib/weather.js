/**
 * Weather Data Source for Neo N3 Custom Oracle Service
 * 
 * This module provides functions for fetching weather data from various sources.
 * 
 * Note: Before using this module, you need to install the axios package:
 * npm install axios
 */

// Import required modules
// const axios = require('axios'); // Uncomment this line after installing axios

/**
 * Fetch weather data from OpenWeatherMap API
 * @param {Object} config - Configuration for the weather data source
 * @param {string} location - Location to fetch weather data for
 * @returns {Promise<Object>} - Weather data
 */
async function fetchFromOpenWeatherMap(config, location) {
  try {
    const response = await axios.get('https://api.openweathermap.org/data/2.5/weather', {
      params: {
        q: location,
        appid: config.apiKey,
        units: 'metric'
      }
    });
    
    return {
      source: 'openweathermap',
      location: response.data.name,
      temperature: response.data.main.temp,
      condition: response.data.weather[0].description,
      humidity: response.data.main.humidity,
      pressure: response.data.main.pressure,
      wind: {
        speed: response.data.wind.speed,
        direction: getWindDirection(response.data.wind.deg)
      },
      timestamp: Date.now()
    };
  } catch (error) {
    throw new Error(`Error fetching weather data from OpenWeatherMap: ${error.message}`);
  }
}

/**
 * Fetch weather data from WeatherAPI
 * @param {Object} config - Configuration for the weather data source
 * @param {string} location - Location to fetch weather data for
 * @returns {Promise<Object>} - Weather data
 */
async function fetchFromWeatherAPI(config, location) {
  try {
    const response = await axios.get('https://api.weatherapi.com/v1/current.json', {
      params: {
        key: config.apiKey,
        q: location
      }
    });
    
    return {
      source: 'weatherapi',
      location: response.data.location.name,
      temperature: response.data.current.temp_c,
      condition: response.data.current.condition.text,
      humidity: response.data.current.humidity,
      pressure: response.data.current.pressure_mb,
      wind: {
        speed: response.data.current.wind_kph,
        direction: response.data.current.wind_dir
      },
      timestamp: Date.now()
    };
  } catch (error) {
    throw new Error(`Error fetching weather data from WeatherAPI: ${error.message}`);
  }
}

/**
 * Get wind direction from degrees
 * @param {number} degrees - Wind direction in degrees
 * @returns {string} - Wind direction as a string (N, NE, E, etc.)
 */
function getWindDirection(degrees) {
  const directions = ['N', 'NNE', 'NE', 'ENE', 'E', 'ESE', 'SE', 'SSE', 'S', 'SSW', 'SW', 'WSW', 'W', 'WNW', 'NW', 'NNW'];
  const index = Math.round(degrees / 22.5) % 16;
  return directions[index];
}

/**
 * Fetch weather data from the specified source
 * @param {Object} config - Configuration for the weather data source
 * @param {string} location - Location to fetch weather data for
 * @param {string} source - Source to fetch weather data from
 * @returns {Promise<Object>} - Weather data
 */
async function fetchWeatherData(config, location, source = 'openweathermap') {
  switch (source) {
    case 'openweathermap':
      return await fetchFromOpenWeatherMap(config, location);
    case 'weatherapi':
      return await fetchFromWeatherAPI(config, location);
    default:
      throw new Error(`Unsupported weather data source: ${source}`);
  }
}

module.exports = {
  fetchWeatherData
};
