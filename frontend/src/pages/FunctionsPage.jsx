import React from 'react';
import { Link } from 'react-router-dom';
import FunctionList from '../components/FunctionList';

const FunctionsPage = () => {
  return (
    <div className="space-y-8">
      <div className="bg-white p-6 rounded-lg shadow-md">
        <div className="flex justify-between items-center">
          <div>
            <h1 className="text-2xl font-bold mb-2">Your Functions</h1>
            <p className="text-gray-600">
              Deploy, manage, and invoke your serverless functions.
            </p>
          </div>
          
          <Link to="/functions/new" className="btn btn-primary">
            Deploy New Function
          </Link>
        </div>
      </div>
      
      <FunctionList />
      
      <div className="card">
        <h2 className="text-xl font-bold mb-4">About R3E FaaS</h2>
        <p className="text-gray-600 mb-4">
          R3E FaaS (Functions as a Service) is a serverless computing platform for Web3 applications.
          It allows you to deploy and run code without managing the underlying infrastructure.
        </p>
        
        <h3 className="text-lg font-semibold mb-2">Features:</h3>
        <ul className="list-disc list-inside space-y-2 text-gray-700 mb-4">
          <li>Deploy functions in multiple runtimes (Node.js, Python, Rust, Deno)</li>
          <li>Integrate with both Ethereum and Neo N3 blockchains</li>
          <li>Pay for execution using Gas Bank</li>
          <li>Use meta transactions for gasless interactions</li>
          <li>Secure and scalable infrastructure</li>
        </ul>
        
        <h3 className="text-lg font-semibold mb-2">Getting Started:</h3>
        <ol className="list-decimal list-inside space-y-2 text-gray-700">
          <li>Connect your wallet</li>
          <li>Deposit funds to your Gas Bank account</li>
          <li>Deploy your first function</li>
          <li>Invoke your function</li>
        </ol>
      </div>
    </div>
  );
};

export default FunctionsPage;
