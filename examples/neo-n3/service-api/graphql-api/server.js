/**
 * GraphQL API Server for Neo N3 FaaS Platform
 * 
 * This script implements a GraphQL server for the Neo N3 FaaS platform.
 */

const { ApolloServer } = require('apollo-server-micro');
const { ApolloServerPluginLandingPageGraphQLPlayground } = require('apollo-server-core');
const fs = require('fs');
const path = require('path');

// Import resolvers
const Query = require('./resolvers/query');
const Mutation = require('./resolvers/mutation');
const Subscription = require('./resolvers/subscription');
const Types = require('./resolvers/types');

// Read schema
const typeDefs = fs.readFileSync(path.join(__dirname, 'schema.graphql'), 'utf8');

// Create Apollo Server
const server = new ApolloServer({
  typeDefs,
  resolvers: {
    Query,
    Mutation,
    Subscription,
    Service: Types.Service,
    Function: Types.Function,
    Trigger: Types.Trigger,
    HttpTrigger: Types.HttpTrigger,
    EventTrigger: Types.EventTrigger,
    ScheduleTrigger: Types.ScheduleTrigger,
    Execution: Types.Execution,
    LogEntry: Types.LogEntry,
    User: Types.User,
    Metrics: Types.Metrics
  },
  context: ({ req }) => {
    // Extract authentication information from request
    const token = req.headers.authorization || '';
    
    // Validate token and get user information
    const user = validateToken(token);
    
    return { user };
  },
  plugins: [
    // Enable GraphQL Playground in development
    process.env.GRAPHQL_PLAYGROUND === 'true'
      ? ApolloServerPluginLandingPageGraphQLPlayground()
      : undefined
  ].filter(Boolean),
  introspection: process.env.NODE_ENV !== 'production'
});

// Validate authentication token
function validateToken(token) {
  if (!token) {
    return null;
  }
  
  try {
    // Remove 'Bearer ' prefix if present
    const tokenValue = token.startsWith('Bearer ') ? token.slice(7) : token;
    
    // Validate JWT token and get user information
    const jwt = require('jsonwebtoken');
    const { JWT_SECRET } = process.env;
    
    if (!JWT_SECRET) {
      console.error('JWT_SECRET environment variable not set');
      return null;
    }
    
    // Verify and decode the JWT token
    const decoded = jwt.verify(tokenValue, JWT_SECRET);
    
    // Return user information from token payload
    return {
      id: decoded.sub,
      username: decoded.username,
      email: decoded.email,
      roles: decoded.roles || ['user']
    };
  } catch (error) {
    console.error('Error validating token:', error);
    return null;
  }
}

// Handler function for FaaS platform
async function handler(request, user, context) {
  try {
    // Create a request object for Apollo Server
    const req = {
      method: request.method,
      headers: request.headers,
      body: request.body
    };
    
    // Create a response object
    const res = {
      statusCode: 200,
      headers: {},
      body: null
    };
    
    // Handle OPTIONS requests for CORS
    if (req.method === 'OPTIONS') {
      res.headers = {
        'Access-Control-Allow-Origin': '*',
        'Access-Control-Allow-Methods': 'POST, GET, OPTIONS',
        'Access-Control-Allow-Headers': 'Content-Type, Authorization'
      };
      return res;
    }
    
    // Handle GET requests (GraphQL Playground)
    if (req.method === 'GET') {
      const playground = await server.executeHTTPGraphQLRequest({
        httpGraphQLRequest: {
          method: req.method,
          headers: req.headers,
          body: null,
          search: ''
        },
        context: { user }
      });
      
      res.headers = playground.headers;
      res.body = playground.body;
      return res;
    }
    
    // Handle POST requests (GraphQL operations)
    if (req.method === 'POST') {
      const result = await server.executeHTTPGraphQLRequest({
        httpGraphQLRequest: {
          method: req.method,
          headers: req.headers,
          body: req.body,
          search: ''
        },
        context: { user }
      });
      
      res.headers = result.headers;
      res.body = result.body;
      return res;
    }
    
    // Handle unsupported methods
    res.statusCode = 405;
    res.body = { error: 'Method not allowed' };
    return res;
  } catch (error) {
    console.error('Error handling GraphQL request:', error);
    
    return {
      statusCode: 500,
      headers: {
        'Content-Type': 'application/json'
      },
      body: {
        error: 'Internal Server Error',
        message: error.message
      }
    };
  }
}

// Export the handler function
module.exports = { handler };

// Start a local server if running directly
if (require.main === module) {
  const http = require('http');
  const port = process.env.PORT || 4000;
  
  // Create HTTP server
  const httpServer = http.createServer(async (req, res) => {
    // Read request body
    let body = '';
    req.on('data', chunk => {
      body += chunk.toString();
    });
    
    req.on('end', async () => {
      try {
        // Parse body if it's JSON
        let parsedBody = null;
        if (body && req.headers['content-type'] === 'application/json') {
          parsedBody = JSON.parse(body);
        }
        
        // Create request object
        const request = {
          method: req.method,
          headers: req.headers,
          body: parsedBody
        };
        
        // Call handler
        const response = await handler(request, null, {});
        
        // Set status code
        res.statusCode = response.statusCode;
        
        // Set headers
        if (response.headers) {
          Object.entries(response.headers).forEach(([key, value]) => {
            res.setHeader(key, value);
          });
        }
        
        // Set content type if not already set
        if (!res.getHeader('Content-Type')) {
          res.setHeader('Content-Type', 'application/json');
        }
        
        // Send response
        if (typeof response.body === 'object') {
          res.end(JSON.stringify(response.body));
        } else {
          res.end(response.body);
        }
      } catch (error) {
        console.error('Error handling request:', error);
        
        res.statusCode = 500;
        res.setHeader('Content-Type', 'application/json');
        res.end(JSON.stringify({
          error: 'Internal Server Error',
          message: error.message
        }));
      }
    });
  });
  
  // Start server
  httpServer.listen(port, () => {
    console.log(`GraphQL server running at http://localhost:${port}/graphql`);
    console.log(`GraphQL Playground available at http://localhost:${port}/graphql`);
  });
}
