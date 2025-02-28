import React from 'react';

const Footer = () => {
  return (
    <footer className="bg-gray-800 text-white py-6">
      <div className="container mx-auto px-4">
        <div className="flex flex-col md:flex-row justify-between items-center">
          <div className="mb-4 md:mb-0">
            <h3 className="text-lg font-bold">R3E FaaS Portal</h3>
            <p className="text-sm text-gray-400">
              Function as a Service for blockchain applications
            </p>
          </div>
          
          <div className="flex space-x-6">
            <a
              href="https://github.com/R3E-Network"
              target="_blank"
              rel="noopener noreferrer"
              className="text-gray-400 hover:text-white"
            >
              GitHub
            </a>
            <a
              href="https://docs.r3e.network"
              target="_blank"
              rel="noopener noreferrer"
              className="text-gray-400 hover:text-white"
            >
              Documentation
            </a>
            <a
              href="https://r3e.network"
              target="_blank"
              rel="noopener noreferrer"
              className="text-gray-400 hover:text-white"
            >
              Website
            </a>
          </div>
        </div>
        
        <div className="mt-6 pt-6 border-t border-gray-700 text-center text-sm text-gray-400">
          <p>&copy; {new Date().getFullYear()} R3E Network. All rights reserved.</p>
        </div>
      </div>
    </footer>
  );
};

export default Footer;
