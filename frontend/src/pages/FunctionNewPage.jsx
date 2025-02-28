import React from 'react';
import FunctionForm from '../components/FunctionForm';

const FunctionNewPage = () => {
  return (
    <div className="space-y-8">
      <div className="bg-white p-6 rounded-lg shadow-md">
        <h1 className="text-2xl font-bold mb-2">Deploy New Function</h1>
        <p className="text-gray-600">
          Create and deploy a new serverless function to the R3E FaaS platform.
        </p>
      </div>
      
      <FunctionForm />
      
      <div className="card">
        <h2 className="text-xl font-bold mb-4">Deployment Guidelines</h2>
        
        <div className="space-y-4">
          <div>
            <h3 className="text-lg font-semibold mb-2">Supported Runtimes</h3>
            <ul className="list-disc list-inside space-y-1 text-gray-700">
              <li>Node.js 16 - JavaScript/TypeScript runtime</li>
              <li>Python 3.9 - Python runtime</li>
              <li>Rust 1.60 - Rust runtime</li>
              <li>Deno 1.24 - Secure JavaScript/TypeScript runtime</li>
            </ul>
          </div>
          
          <div>
            <h3 className="text-lg font-semibold mb-2">Function Structure</h3>
            <p className="text-gray-600 mb-2">
              Your function should export a default handler function that receives request and context parameters:
            </p>
            
            <pre className="bg-gray-100 p-4 rounded-md overflow-x-auto font-mono text-sm">
{`// Node.js example
export default async function handler(req, context) {
  // Your code here
  return { message: 'Hello, World!' };
}`}
            </pre>
          </div>
          
          <div>
            <h3 className="text-lg font-semibold mb-2">Blockchain Integration</h3>
            <p className="text-gray-600 mb-2">
              Your function can interact with blockchain services based on the selected blockchain type:
            </p>
            
            <ul className="list-disc list-inside space-y-1 text-gray-700">
              <li>Neo N3 - Access to Neo N3 blockchain APIs and smart contracts</li>
              <li>Ethereum - Access to Ethereum blockchain APIs and smart contracts</li>
            </ul>
          </div>
          
          <div>
            <h3 className="text-lg font-semibold mb-2">Environment Variables</h3>
            <p className="text-gray-600">
              You can define environment variables that will be available to your function at runtime.
              These are useful for storing configuration values, API keys, and other secrets.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default FunctionNewPage;
