const http = require('http');

// Configuration
const config = {
  port: 3000,
  maxRunners: 10,
  maxPending: 128
};

// Create server
const server = http.createServer((req, res) => {
  if (req.method === 'POST' && req.url === '/invoke') {
    let body = '';
    req.on('data', chunk => { body += chunk.toString(); });
    req.on('end', () => {
      try {
        const data = JSON.parse(body);
        console.log(`[INFO] Function invocation: ${JSON.stringify(data)}`);
        
        // Mock execution
        const result = {
          success: true,
          data: `Executed: ${data.function || 'anonymous'}`
        };
        
        res.writeHead(200, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify(result));
      } catch (error) {
        res.writeHead(400, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ success: false, error: 'Invalid request' }));
      }
    });
  } else {
    res.writeHead(404);
    res.end('Not found');
  }
});

// Start server
server.listen(config.port, () => {
  console.log(`R3E FaaS Mock Service running on port ${config.port}`);
});
