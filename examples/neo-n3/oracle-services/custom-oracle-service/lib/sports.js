/**
 * Sports Data Source for Neo N3 Custom Oracle Service
 * 
 * This module provides functions for fetching sports data from various sources.
 */

// Import required modules
const axios = require('axios');

/**
 * Fetch sports data from SportsData.io API
 * @param {Object} config - Configuration for the sports data source
 * @param {string} sport - Sport to fetch data for (e.g., 'nba', 'nfl', 'mlb')
 * @returns {Promise<Object>} - Sports data
 */
async function fetchFromSportsDataIO(config, sport) {
  try {
    let url;
    switch (sport.toLowerCase()) {
      case 'nba':
        url = 'https://api.sportsdata.io/v3/nba/scores/json/Games';
        break;
      case 'nfl':
        url = 'https://api.sportsdata.io/v3/nfl/scores/json/Scores';
        break;
      case 'mlb':
        url = 'https://api.sportsdata.io/v3/mlb/scores/json/Games';
        break;
      default:
        throw new Error(`Unsupported sport: ${sport}`);
    }
    
    const response = await axios.get(url, {
      headers: {
        'Ocp-Apim-Subscription-Key': config.apiKey
      }
    });
    
    // Process the response based on the sport
    return {
      source: 'sportsdataio',
      sport: sport,
      games: processGames(response.data, sport),
      timestamp: Date.now()
    };
  } catch (error) {
    throw new Error(`Error fetching sports data from SportsData.io: ${error.message}`);
  }
}

/**
 * Process games data based on the sport
 * @param {Array} games - Games data from the API
 * @param {string} sport - Sport type
 * @returns {Array} - Processed games data
 */
function processGames(games, sport) {
  switch (sport.toLowerCase()) {
    case 'nba':
      return games.map(game => ({
        home: game.HomeTeam,
        away: game.AwayTeam,
        score: `${game.HomeTeamScore}-${game.AwayTeamScore}`,
        status: getGameStatus(game.Status),
        date: game.DateTime
      }));
    case 'nfl':
      return games.map(game => ({
        home: game.HomeTeam,
        away: game.AwayTeam,
        score: `${game.HomeScore}-${game.AwayScore}`,
        status: getGameStatus(game.Status),
        date: game.Date
      }));
    case 'mlb':
      return games.map(game => ({
        home: game.HomeTeam,
        away: game.AwayTeam,
        score: `${game.HomeTeamRuns}-${game.AwayTeamRuns}`,
        status: getGameStatus(game.Status),
        date: game.DateTime
      }));
    default:
      return games;
  }
}

/**
 * Get game status in a standardized format
 * @param {string} status - Status from the API
 * @returns {string} - Standardized status
 */
function getGameStatus(status) {
  switch (status) {
    case 'Final':
    case 'F':
      return 'final';
    case 'InProgress':
    case 'IP':
      return 'in_progress';
    case 'Scheduled':
    case 'S':
      return 'scheduled';
    case 'Postponed':
    case 'P':
      return 'postponed';
    case 'Canceled':
    case 'C':
      return 'canceled';
    default:
      return status.toLowerCase();
  }
}

/**
 * Fetch sports data from the specified source
 * @param {Object} config - Configuration for the sports data source
 * @param {string} sport - Sport to fetch data for
 * @param {string} source - Source to fetch sports data from
 * @returns {Promise<Object>} - Sports data
 */
async function fetchSportsData(config, sport, source = 'sportsdataio') {
  switch (source) {
    case 'sportsdataio':
      return await fetchFromSportsDataIO(config, sport);
    default:
      throw new Error(`Unsupported sports data source: ${source}`);
  }
}

module.exports = {
  fetchSportsData
};
