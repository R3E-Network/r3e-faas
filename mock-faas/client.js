const http = require('http');

// Function to invoke
const data = {
  function: 'helloWorld',
  args: { name: 'R3E Network' }
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
  });
});

req.on('error', error => {
  console.error('Error:', error);
});

req.write(JSON.stringify(data));
req.end();
