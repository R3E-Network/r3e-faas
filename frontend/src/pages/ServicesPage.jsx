import React from 'react';
import ServiceList from '../components/ServiceList';

const ServicesPage = () => {
  return (
    <div className="space-y-8">
      <div className="bg-white p-6 rounded-lg shadow-md">
        <h1 className="text-2xl font-bold mb-4">Available Services</h1>
        <p className="text-gray-600">
          Browse and interact with the available R3E FaaS services.
          Connect your wallet to access and invoke these services.
        </p>
      </div>
      
      <ServiceList />
    </div>
  );
};

export default ServicesPage;
