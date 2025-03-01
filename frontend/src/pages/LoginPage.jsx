import React, { useState, useEffect } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import NeoWalletConnect from '../components/NeoWalletConnect';
import { isAuthenticated, setupTokenRefresh } from '../services/authService';
import { setupErrorHandling } from '../utils/apiClient';

// Page container style
const PAGE_STYLE = {
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'center',
  justifyContent: 'center',
  padding: '2rem',
  maxWidth: '600px',
  margin: '0 auto',
  marginTop: '2rem',
  borderRadius: '12px',
  boxShadow: '0 8px 24px rgba(0, 0, 0, 0.1)',
  background: '#fff',
};

// Card style for wallet options
const CARD_STYLE = {
  width: '100%',
  borderRadius: '10px',
  padding: '1.5rem',
  marginBottom: '1.5rem',
  border: '1px solid #eaeaea',
  boxShadow: '0 2px 10px rgba(0, 0, 0, 0.05)',
};

// Alert styles
const ALERT_STYLES = {
  error: {
    padding: '1rem',
    backgroundColor: '#ffebee',
    color: '#c62828',
    borderRadius: '4px',
    marginBottom: '1rem',
    width: '100%',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
  },
  success: {
    padding: '1rem',
    backgroundColor: '#e8f5e9',
    color: '#2e7d32',
    borderRadius: '4px',
    marginBottom: '1rem',
    width: '100%',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
  },
  warning: {
    padding: '1rem',
    backgroundColor: '#fff8e1',
    color: '#ff8f00',
    borderRadius: '4px',
    marginBottom: '1rem',
    width: '100%',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
  },
  info: {
    padding: '1rem',
    backgroundColor: '#e3f2fd',
    color: '#0d47a1',
    borderRadius: '4px',
    marginBottom: '1rem',
    width: '100%',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
  },
};

// Close button style
const CLOSE_BUTTON_STYLE = {
  background: 'none',
  border: 'none',
  cursor: 'pointer',
  fontWeight: 'bold',
  fontSize: '16px',
  padding: '0 5px',
};

/**
 * Check if the browser is supported
 * @returns {boolean} - True if the browser is supported
 */
const isBrowserSupported = () => {
  // Check for localStorage/sessionStorage
  try {
    const storage = window.sessionStorage;
    const testKey = 'test';
    storage.setItem(testKey, '1');
    storage.removeItem(testKey);
  } catch (error) {
    return false;
  }

  // Check for basic required features
  const requiredFeatures = [
    window.fetch, 
    window.Promise, 
    window.localStorage,
    window.sessionStorage
  ];
  
  return requiredFeatures.every(Boolean);
};

/**
 * Login Page
 * 
 * A page that allows users to log in to the application using their
 * Neo N3 wallets.
 */
const LoginPage = () => {
  const navigate = useNavigate();
  const [alerts, setAlerts] = useState([]);
  const [loading, setLoading] = useState(true);
  const [browserSupported, setBrowserSupported] = useState(true);

  // Check if user is already authenticated and set up token refresh
  useEffect(() => {
    const initAuth = async () => {
      // Check browser compatibility
      if (!isBrowserSupported()) {
        setBrowserSupported(false);
        setLoading(false);
        return;
      }
      
      // Set up token auto-refresh
      setupTokenRefresh();
      
      // Set up global error handling
      setupErrorHandling((error) => {
        if (error.response?.status === 401) {
          addAlert('error', 'Your session has expired. Please login again.');
        } else if (error.response?.status >= 500) {
          addAlert('error', 'Server error. Please try again later.');
        }
      });
      
      // Check if already authenticated
      if (isAuthenticated()) {
        // Redirect to dashboard or home page
        navigate('/dashboard');
      } else {
        setLoading(false);
      }
    };

    initAuth();
  }, [navigate]);

  // Generate a unique ID for alerts
  const generateAlertId = () => {
    return `alert-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  };

  // Add an alert
  const addAlert = (type, message, timeout = 5000) => {
    const id = generateAlertId();
    const newAlert = { id, type, message };
    
    setAlerts(prev => [...prev, newAlert]);
    
    // Auto-dismiss non-error alerts
    if (type !== 'error' && timeout > 0) {
      setTimeout(() => {
        dismissAlert(id);
      }, timeout);
    }
    
    return id;
  };

  // Dismiss an alert
  const dismissAlert = (id) => {
    setAlerts(prev => prev.filter(alert => alert.id !== id));
  };

  // Handle wallet connection
  const handleWalletConnect = (wallet) => {
    addAlert('info', `Connected to wallet: ${wallet.address}`, 3000);
  };

  // Handle successful authentication
  const handleAuthentication = (authData) => {
    const alertId = addAlert('success', `Successfully authenticated with blockchain: ${authData.blockchain_type}`, 0);
    
    // Redirect to dashboard after a short delay
    setTimeout(() => {
      dismissAlert(alertId);
      navigate('/dashboard');
    }, 1500);
  };

  // Handle errors
  const handleError = (errorMessage) => {
    addAlert('error', errorMessage);
  };

  // Render alerts
  const renderAlerts = () => {
    return alerts.map(alert => (
      <div key={alert.id} style={ALERT_STYLES[alert.type]}>
        <span>{alert.message}</span>
        <button 
          onClick={() => dismissAlert(alert.id)} 
          style={CLOSE_BUTTON_STYLE}
        >
          ×
        </button>
      </div>
    ));
  };

  // Show loading state
  if (loading) {
    return (
      <div style={PAGE_STYLE}>
        <div style={{ textAlign: 'center' }}>
          <h2>Loading...</h2>
          <div 
            style={{ 
              width: '50px', 
              height: '50px', 
              border: '5px solid #f3f3f3',
              borderTop: '5px solid #3498db',
              borderRadius: '50%',
              margin: '20px auto',
              animation: 'spin 1s linear infinite',
            }} 
          />
          <style>{`
            @keyframes spin {
              0% { transform: rotate(0deg); }
              100% { transform: rotate(360deg); }
            }
          `}</style>
        </div>
      </div>
    );
  }
  
  // Show browser not supported message
  if (!browserSupported) {
    return (
      <div style={PAGE_STYLE}>
        <h1>Browser Not Supported</h1>
        <p>Your browser doesn't support all the features needed for this application.</p>
        <p>Please try using a modern browser like:</p>
        <ul>
          <li>Google Chrome (recommended)</li>
          <li>Mozilla Firefox</li>
          <li>Microsoft Edge</li>
          <li>Safari (latest version)</li>
        </ul>
      </div>
    );
  }

  return (
    <div style={PAGE_STYLE}>
      <h1>Login to R3E Platform</h1>
      <p>Connect your Neo N3 wallet to access the platform.</p>
      
      {renderAlerts()}

      <div style={CARD_STYLE}>
        <NeoWalletConnect
          onConnect={handleWalletConnect}
          onAuthenticate={handleAuthentication}
          onError={handleError}
          autoConnect={true}
          pollForWallets={true}
        />
      </div>
      
      <div style={{ marginTop: '2rem', textAlign: 'center' }}>
        <h3>Why connect with a wallet?</h3>
        <p style={{ color: '#666', maxWidth: '500px', lineHeight: '1.6' }}>
          Connecting with your Neo N3 wallet provides secure, passwordless authentication.
          Your private keys never leave your device, and you can authorize actions with 
          your wallet's secure signing capabilities.
        </p>
        
        <p style={{ 
          marginTop: '2rem', 
          textAlign: 'center', 
          color: '#666', 
          borderTop: '1px solid #eaeaea',
          paddingTop: '1rem'
        }}>
          Don't have a Neo N3 wallet? <a 
            href="https://neoline.io/" 
            target="_blank" 
            rel="noopener noreferrer"
            style={{ color: '#3366cc', textDecoration: 'none', fontWeight: 'bold' }}
          >
            Get NeoLine
          </a>
        </p>
        
        <p style={{ fontSize: '14px', color: '#999', marginTop: '10px' }}>
          <Link to="/help/wallets" style={{ color: '#3366cc', textDecoration: 'none' }}>
            Learn more about Neo N3 wallets →
          </Link>
        </p>
      </div>
    </div>
  );
};

export default LoginPage; 