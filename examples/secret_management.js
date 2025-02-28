// Example of using secret management in R3E FaaS

// Function that uses secret management
export function main(args) {
  // Get API key from secrets
  const apiKey = secrets.get('api-key');
  
  // Use the API key to make an authenticated request
  const response = r3e.oracle.fetchWithAuth('https://api.example.com/data', apiKey);
  
  return {
    success: true,
    data: response,
    secretLength: apiKey.length
  };
}
