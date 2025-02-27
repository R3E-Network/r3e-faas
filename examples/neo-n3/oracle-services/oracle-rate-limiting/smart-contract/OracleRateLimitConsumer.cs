using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;
using Neo.SmartContract.Framework.Services;
using System;
using System.ComponentModel;

namespace Neo.SmartContract.Examples
{
    [DisplayName("OracleRateLimitConsumer")]
    [ManifestExtra("Author", "Neo N3 FaaS")]
    [ManifestExtra("Email", "dev@neo.org")]
    [ManifestExtra("Description", "Neo N3 Oracle Rate Limiting Consumer Example")]
    public class OracleRateLimitConsumer : Framework.SmartContract
    {
        // Events
        [DisplayName("OracleRequest")]
        public static event Action<UInt160, string, string> OnOracleRequest;
        
        [DisplayName("OracleResponse")]
        public static event Action<UInt256, string, byte[]> OnOracleResponse;
        
        [DisplayName("RateLimitExceeded")]
        public static event Action<string, int> OnRateLimitExceeded;
        
        // Storage keys
        private static readonly byte[] PrefixRequest = new byte[] { 0x01 };
        private static readonly byte[] PrefixResponse = new byte[] { 0x02 };
        private static readonly byte[] PrefixData = new byte[] { 0x03 };
        private static readonly byte[] PrefixRateLimit = new byte[] { 0x04 };
        
        // Oracle URL
        private static readonly string OracleUrl = "http://localhost:8080/oracle/rate-limited";
        
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
        /// Request price data from the oracle
        /// </summary>
        /// <param name="symbol">The symbol to get the price for</param>
        public static void RequestPrice(string symbol)
        {
            // Check if the caller is the contract owner
            if (!IsOwner())
                throw new Exception("Only the contract owner can request price data");
            
            // Create the request URL with the symbol parameter
            string url = $"{OracleUrl}/price?symbol={symbol}";
            
            // Call the oracle
            Oracle.Request(url, "getPriceCallback", Hash, null);
            
            // Get the request ID (transaction hash)
            UInt256 requestId = Runtime.GetScriptContainer().Hash;
            
            // Store the request details
            StorageMap requestMap = new(Storage.CurrentContext, PrefixRequest);
            requestMap.Put(requestId.ToArray(), symbol);
            
            // Emit the event
            OnOracleRequest(Runtime.ExecutingScriptHash, url, symbol);
        }
        
        /// <summary>
        /// Request price data from the oracle with a specific rate limit strategy
        /// </summary>
        /// <param name="symbol">The symbol to get the price for</param>
        /// <param name="strategy">The rate limit strategy to use (fixed_window, sliding_window, token_bucket)</param>
        public static void RequestPriceWithStrategy(string symbol, string strategy)
        {
            // Check if the caller is the contract owner
            if (!IsOwner())
                throw new Exception("Only the contract owner can request price data");
            
            // Create the request URL with the symbol parameter and strategy
            string url = $"{OracleUrl}/price?symbol={symbol}&strategy={strategy}";
            
            // Call the oracle
            Oracle.Request(url, "getPriceCallback", Hash, null);
            
            // Get the request ID (transaction hash)
            UInt256 requestId = Runtime.GetScriptContainer().Hash;
            
            // Store the request details
            StorageMap requestMap = new(Storage.CurrentContext, PrefixRequest);
            requestMap.Put(requestId.ToArray(), symbol);
            
            // Emit the event
            OnOracleRequest(Runtime.ExecutingScriptHash, url, symbol);
        }
        
        /// <summary>
        /// Batch request price data from the oracle
        /// This is useful for testing rate limiting
        /// </summary>
        /// <param name="symbol">The symbol to get the price for</param>
        /// <param name="count">The number of requests to make</param>
        public static void BatchRequestPrice(string symbol, int count)
        {
            // Check if the caller is the contract owner
            if (!IsOwner())
                throw new Exception("Only the contract owner can request price data");
            
            // Limit the number of requests to prevent abuse
            if (count > 10)
                count = 10;
            
            // Make multiple requests
            for (int i = 0; i < count; i++)
            {
                // Create the request URL with the symbol parameter
                string url = $"{OracleUrl}/price?symbol={symbol}&batch={i}";
                
                // Call the oracle
                Oracle.Request(url, "getPriceCallback", Hash, null);
                
                // Get the request ID (transaction hash)
                UInt256 requestId = Runtime.GetScriptContainer().Hash;
                
                // Store the request details
                StorageMap requestMap = new(Storage.CurrentContext, PrefixRequest);
                requestMap.Put(requestId.ToArray(), symbol);
                
                // Emit the event
                OnOracleRequest(Runtime.ExecutingScriptHash, url, symbol);
            }
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
            
            // Check if the response is a rate limit exceeded error
            string responseStr = System.Text.Encoding.UTF8.GetString(result);
            if (responseStr.Contains("rate_limit_exceeded"))
            {
                // Parse the retry after value
                // In a real implementation, this would parse the JSON response
                // For this example, we'll just extract a hardcoded value
                int retryAfter = 30;
                
                // Store the rate limit information
                StorageMap rateLimitMap = new(Storage.CurrentContext, PrefixRateLimit);
                rateLimitMap.Put(symbol, retryAfter.ToString());
                
                // Emit the rate limit exceeded event
                OnRateLimitExceeded(symbol, retryAfter);
                
                return;
            }
            
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
        
        /// <summary>
        /// Check if a symbol is rate limited
        /// </summary>
        /// <param name="symbol">The symbol to check</param>
        /// <returns>The retry after time in seconds, or 0 if not rate limited</returns>
        public static int IsRateLimited(string symbol)
        {
            StorageMap rateLimitMap = new(Storage.CurrentContext, PrefixRateLimit);
            byte[] retryAfterBytes = rateLimitMap.Get(symbol);
            
            if (retryAfterBytes == null || retryAfterBytes.Length == 0)
                return 0;
            
            return int.Parse(System.Text.Encoding.UTF8.GetString(retryAfterBytes));
        }
        
        /// <summary>
        /// Clear the rate limit for a symbol
        /// </summary>
        /// <param name="symbol">The symbol to clear</param>
        public static void ClearRateLimit(string symbol)
        {
            // Check if the caller is the contract owner
            if (!IsOwner())
                throw new Exception("Only the contract owner can clear rate limits");
            
            StorageMap rateLimitMap = new(Storage.CurrentContext, PrefixRateLimit);
            rateLimitMap.Delete(symbol);
        }
    }
}
