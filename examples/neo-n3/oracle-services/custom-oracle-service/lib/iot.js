/**
 * IoT Data Source for Neo N3 Custom Oracle Service
 * 
 * This module provides functions for fetching IoT device data from various sources.
 */

// Import required modules
const mqtt = require('mqtt');

/**
 * Fetch IoT data from MQTT broker
 * @param {Object} config - Configuration for the IoT data source
 * @param {Array} topics - MQTT topics to subscribe to
 * @returns {Promise<Object>} - IoT data
 */
async function fetchFromMQTT(config, topics) {
  return new Promise((resolve, reject) => {
    try {
      // Connect to MQTT broker
      const client = mqtt.connect(config.broker);
      
      // Prepare result object
      const result = {
        network: config.network || 'default',
        devices: 0,
        readings: [],
        timestamp: Date.now()
      };
      
      // Set timeout for data collection
      const timeout = setTimeout(() => {
        client.end();
        resolve(result);
      }, config.timeout || 5000);
      
      // Handle connection
      client.on('connect', () => {
        // Subscribe to topics
        for (const topic of topics) {
          client.subscribe(topic);
        }
      });
      
      // Handle messages
      client.on('message', (topic, message) => {
        try {
          // Parse message
          const data = JSON.parse(message.toString());
          
          // Extract sensor and location from topic
          const parts = topic.split('/');
          const sensor = parts[1] || 'unknown';
          const location = data.location || 'unknown';
          
          // Add reading to result
          result.readings.push({
            sensor: sensor,
            location: location,
            value: data.value,
            timestamp: data.timestamp || Date.now()
          });
          
          // Update device count (based on unique device IDs)
          if (data.device_id) {
            result.devices = new Set([...result.devices, data.device_id]).size;
          }
        } catch (error) {
          console.error(`Error processing MQTT message: ${error.message}`);
        }
      });
      
      // Handle errors
      client.on('error', (error) => {
        clearTimeout(timeout);
        client.end();
        reject(new Error(`MQTT client error: ${error.message}`));
      });
    } catch (error) {
      reject(new Error(`Error fetching IoT data from MQTT: ${error.message}`));
    }
  });
}

/**
 * Fetch IoT data from HTTP API
 * @param {Object} config - Configuration for the IoT data source
 * @param {string} network - IoT network to fetch data for
 * @returns {Promise<Object>} - IoT data
 */
async function fetchFromHTTP(config, network) {
  try {
    // In a real implementation, this would make HTTP requests to IoT platforms
    // For this example, we'll return mock data
    return {
      network: network,
      devices: 5,
      readings: [
        { sensor: 'temperature', location: 'downtown', value: 24.5 },
        { sensor: 'humidity', location: 'downtown', value: 58 },
        { sensor: 'air_quality', location: 'downtown', value: 42 },
        { sensor: 'temperature', location: 'suburb', value: 22.8 },
        { sensor: 'humidity', location: 'suburb', value: 62 }
      ],
      timestamp: Date.now()
    };
  } catch (error) {
    throw new Error(`Error fetching IoT data from HTTP: ${error.message}`);
  }
}

/**
 * Fetch IoT data from the specified source
 * @param {Object} config - Configuration for the IoT data source
 * @param {Array} topics - MQTT topics to subscribe to (for MQTT source)
 * @param {string} network - IoT network to fetch data for (for HTTP source)
 * @param {string} source - Source to fetch IoT data from
 * @returns {Promise<Object>} - IoT data
 */
async function fetchIoTData(config, topics, network, source = 'mqtt') {
  switch (source) {
    case 'mqtt':
      return await fetchFromMQTT(config, topics);
    case 'http':
      return await fetchFromHTTP(config, network);
    default:
      throw new Error(`Unsupported IoT data source: ${source}`);
  }
}

module.exports = {
  fetchIoTData
};
