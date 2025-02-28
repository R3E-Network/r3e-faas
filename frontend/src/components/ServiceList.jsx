import React, { useState, useEffect } from 'react';
import { useApi } from '../contexts/ApiContext';

const ServiceList = () => {
  const { listServices, loading, error } = useApi();
  
  const [services, setServices] = useState([]);
  const [loadingServices, setLoadingServices] = useState(false);
  const [errorMessage, setErrorMessage] = useState(null);
  
  // Load services
  useEffect(() => {
    const fetchServices = async () => {
      try {
        setLoadingServices(true);
        setErrorMessage(null);
        
        const data = await listServices();
        setServices(data);
      } catch (error) {
        console.error('Failed to load services:', error);
        setErrorMessage(error.message);
      } finally {
        setLoadingServices(false);
      }
    };
    
    fetchServices();
  }, [listServices]);
  
  return (
    <div className="card">
      <h2 className="text-xl font-bold mb-4">Available Services</h2>
      
      {errorMessage && (
        <div className="bg-red-100 text-red-700 p-3 rounded-md mb-4">
          {errorMessage}
        </div>
      )}
      
      {loadingServices ? (
        <div className="text-center py-8">
          <p className="text-gray-600">Loading services...</p>
        </div>
      ) : services.length === 0 ? (
        <div className="text-center py-8">
          <p className="text-gray-600">No services available.</p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {services.map((service) => (
            <div
              key={service.id}
              className="border rounded-md p-4 hover:shadow-md transition-shadow"
            >
              <h3 className="text-lg font-semibold mb-2">{service.name}</h3>
              <p className="text-sm text-gray-600 mb-3">{service.description}</p>
              
              <div className="flex justify-between items-center">
                <span className="text-xs bg-blue-100 text-blue-800 px-2 py-1 rounded-full">
                  {service.blockchain_type}
                </span>
                
                <a
                  href={`/services/${service.id}`}
                  className="text-sm text-blue-600 hover:text-blue-800"
                >
                  View Details
                </a>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default ServiceList;
