using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;
using Neo.SmartContract.Framework.Services;
using System;
using System.ComponentModel;

namespace Neo.SmartContract.Examples
{
    [DisplayName("OracleAuthConsumer")]
    [ManifestExtra("Author", "Neo N3 FaaS")]
    [ManifestExtra("Email", "dev@neo.org")]
    [ManifestExtra("Description", "Neo N3 Oracle Authentication Consumer Example")]
    public class OracleAuthConsumer : Framework.SmartContract
    {
        // Events
        [DisplayName("OracleRequest")]
        public static event Action<UInt160, string, string> OnOracleRequest;
        
        [DisplayName("OracleResponse")]
        public static event Action<UInt256, string, byte[]> OnOracleResponse;
        
        // Storage keys
        private static readonly byte[] PrefixRequest = new byte[] { 0x01 };
        private static readonly byte[] PrefixResponse = new byte[] { 0x02 };
        private static readonly byte[] PrefixData = new byte[] { 0x03 };
        
        // Oracle URL
        private static readonly string OracleUrl = "http://localhost:8080/api/oracle/auth";
        
        // Authentication data
        private static readonly string ApiKey = "valid-api-key-123";
        private static readonly string JwtToken = "valid-jwt-token-user1";
        
        // Contract owner
        [InitialValue("NZpsgXn9VQQYjcjjfRXA5ExBrNLvwQNx7a", ContractParameterType.Hash160)]
        private static readonly UInt160 Owner = default;
        
        // Check if the caller is the contract owner
        private static bool IsOwner() => Runtime.CheckWitness(Owner);
        
        // Only allow the contract owner to call this method
        public static void OnNEP17Payment(UInt160 from, BigInteger amount, object data)
        {
            // Do nothing, just accept the payment
        }
        
        /// <summary>
        /// Request price data from the oracle using API key authentication
        /// </summary>
        /// <param name="symbol">The symbol to get the price for</param>
        public static void RequestPriceWithApiKey(string symbol)
        {
            // Check if the caller is the contract owner
            if (!IsOwner())
                throw new Exception("Only the contract owner can request price data");
            
            // Create the request URL with the symbol parameter
            string url = $"{OracleUrl}/price?symbol={symbol}";
            
            // Create the request with API key authentication
            object[] customData = new object[]
            {
                // Request method
                "GET",
                
                // Request headers
                new object[]
                {
                    // API key header
                    new object[] { "X-API-Key", ApiKey }
                }
            };
            
            // Call the oracle
            Oracle.Request(url, "getPriceCallback", this.Hash, customData);
            
            // Get the request ID (transaction hash)
            UInt256 requestId = Runtime.GetScriptContainer().Hash;
            
            // Store the request details
            StorageMap requestMap = new(Storage.CurrentContext, PrefixRequest);
            requestMap.Put(requestId.ToArray(), symbol);
            
            // Emit the event
            OnOracleRequest(Runtime.ExecutingScriptHash, url, symbol);
        }
        
        /// <summary>
        /// Request price data from the oracle using JWT authentication
        /// </summary>
        /// <param name="symbol">The symbol to get the price for</param>
        public static void RequestPriceWithJwt(string symbol)
        {
            // Check if the caller is the contract owner
            if (!IsOwner())
                throw new Exception("Only the contract owner can request price data");
            
            // Create the request URL with the symbol parameter
            string url = $"{OracleUrl}/price?symbol={symbol}";
            
            // Create the request with JWT authentication
            object[] customData = new object[]
            {
                // Request method
                "GET",
                
                // Request headers
                new object[]
                {
                    // JWT token header
                    new object[] { "Authorization", $"Bearer {JwtToken}" }
                }
            };
            
            // Call the oracle
            Oracle.Request(url, "getPriceCallback", this.Hash, customData);
            
            // Get the request ID (transaction hash)
            UInt256 requestId = Runtime.GetScriptContainer().Hash;
            
            // Store the request details
            StorageMap requestMap = new(Storage.CurrentContext, PrefixRequest);
            requestMap.Put(requestId.ToArray(), symbol);
            
            // Emit the event
            OnOracleRequest(Runtime.ExecutingScriptHash, url, symbol);
        }
        
        /// <summary>
        /// Request price data from the oracle using blockchain authentication
        /// </summary>
        /// <param name="symbol">The symbol to get the price for</param>
        /// <param name="signature">The signature of the request</param>
        /// <param name="publicKey">The public key of the signer</param>
        public static void RequestPriceWithBlockchain(string symbol, byte[] signature, byte[] publicKey)
        {
            // Check if the caller is the contract owner
            if (!IsOwner())
                throw new Exception("Only the contract owner can request price data");
            
            // Create the request URL with the symbol parameter
            string url = $"{OracleUrl}/price?symbol={symbol}";
            
            // Create the timestamp
            string timestamp = Runtime.Time.ToString();
            
            // Create the request with blockchain authentication
            object[] customData = new object[]
            {
                // Request method
                "GET",
                
                // Request headers
                new object[]
                {
                    // Signature header
                    new object[] { "X-Signature", signature },
                    
                    // Public key header
                    new object[] { "X-Public-Key", publicKey },
                    
                    // Timestamp header
                    new object[] { "X-Timestamp", timestamp }
                }
            };
            
            // Call the oracle
            Oracle.Request(url, "getPriceCallback", this.Hash, customData);
            
            // Get the request ID (transaction hash)
            UInt256 requestId = Runtime.GetScriptContainer().Hash;
            
            // Store the request details
            StorageMap requestMap = new(Storage.CurrentContext, PrefixRequest);
            requestMap.Put(requestId.ToArray(), symbol);
            
            // Emit the event
            OnOracleRequest(Runtime.ExecutingScriptHash, url, symbol);
        }
        
        /// <summary>
        /// Callback method for the oracle response
        /// </summary>
        /// <param name="requestId">The request ID (transaction hash)</param>
        /// <param name="responseCode">The HTTP response code</param>
        /// <param name="result">The response data</param>
        public static void getPriceCallback(byte[] requestId, byte[] responseCode, byte[] result)
        {
            // Check if the caller is the oracle contract
            if (!Runtime.CallingScriptHash.Equals(Oracle.Hash))
                throw new Exception("Only the oracle contract can call this method");
            
            // Get the request details
            StorageMap requestMap = new(Storage.CurrentContext, PrefixRequest);
            string symbol = requestMap.Get(requestId);
            
            // Store the response
            StorageMap responseMap = new(Storage.CurrentContext, PrefixResponse);
            responseMap.Put(requestId, result);
            
            // Parse the response and store the price data
            // In a real implementation, this would parse the JSON response
            // For this example, we'll just store the raw response
            StorageMap dataMap = new(Storage.CurrentContext, PrefixData);
            dataMap.Put(symbol, result);
            
            // Emit the event
            OnOracleResponse(new UInt256(requestId), symbol, result);
        }
        
        /// <summary>
        /// Get the price data for a symbol
        /// </summary>
        /// <param name="symbol">The symbol to get the price for</param>
        /// <returns>The price data</returns>
        public static byte[] GetPrice(string symbol)
        {
            StorageMap dataMap = new(Storage.CurrentContext, PrefixData);
            return dataMap.Get(symbol);
        }
        
        /// <summary>
        /// Get the response for a request
        /// </summary>
        /// <param name="requestId">The request ID (transaction hash)</param>
        /// <returns>The response data</returns>
        public static byte[] GetResponse(UInt256 requestId)
        {
            StorageMap responseMap = new(Storage.CurrentContext, PrefixResponse);
            return responseMap.Get(requestId.ToArray());
        }
    }
}
