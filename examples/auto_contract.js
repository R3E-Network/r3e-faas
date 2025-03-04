// Example: Auto Contract
// This example demonstrates how to invoke a function on the R3E FaaS platform

const http = require('http');

// Function to invoke
const data = {
  function: 'autoContract',
  args: { 
    contractName: 'SimpleStorage',
    initialValue: 42,
    network: 'neo-testnet'
  }
};

// Make request
const req = http.request({
  hostname: 'localhost',
  port: 3000,
  path: '/invoke',
  method: 'POST',
  headers: { 'Content-Type': 'application/json' }
}, (res) => {
  let data = '';
  res.on('data', chunk => { data += chunk; });
  res.on('end', () => {
    console.log('Response:', JSON.parse(data));
    console.log('Contract deployed successfully!');
  });
});

req.on('error', error => {
  console.error('Error:', error);
});

req.write(JSON.stringify(data));
req.end();

console.log('Deploying auto contract...');
