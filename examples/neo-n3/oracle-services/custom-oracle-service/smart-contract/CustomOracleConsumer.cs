using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;
using Neo.SmartContract.Framework.Services;
using System;
using System.ComponentModel;

namespace Neo.SmartContract.Examples
{
    [DisplayName("CustomOracleConsumer")]
    [ManifestExtra("Author", "Neo N3 FaaS")]
    [ManifestExtra("Email", "dev@neo.org")]
    [ManifestExtra("Description", "Example contract that consumes data from the custom oracle service")]
    public class CustomOracleConsumer : Framework.SmartContract
    {
        // Events
        [DisplayName("OracleRequest")]
        public static event Action<UInt160, string, string> OnOracleRequest;
        
        [DisplayName("OracleResponse")]
        public static event Action<UInt256, string, string> OnOracleResponse;
        
        // Storage keys
        private static readonly byte[] OracleRequestPrefix = new byte[] { 0x01 };
        private static readonly byte[] OracleResponsePrefix = new byte[] { 0x02 };
        private static readonly byte[] LatestDataPrefix = new byte[] { 0x03 };
        
        // Oracle contract hash
        private static readonly UInt160 OracleContractHash = "0x1234567890abcdef1234567890abcdef12345678".ToScriptHash();
        
        // Custom oracle service URL
        private static readonly string CustomOracleUrl = "https://oracle.example.com/custom";
        
        /// <summary>
        /// Request data from the custom oracle service
        /// </summary>
        /// <param name="source">Data source (weather, sports, social, iot, custom)</param>
        /// <param name="parameters">Additional parameters for the request</param>
        /// <returns>Request ID</returns>
        public static string RequestOracleData(string source, object parameters)
        {
            // Check if the caller is authorized
            if (!Runtime.CheckWitness(Runtime.ExecutingScriptHash))
                throw new Exception("Not authorized");
            
            // Create the request URL with parameters
            string requestUrl = $"{CustomOracleUrl}?source={source}";
            
            // Add parameters to the request URL
            if (parameters != null)
            {
                string jsonParams = StdLib.JsonSerialize(parameters);
                requestUrl += $"&params={Uri.EscapeDataString(jsonParams)}";
            }
            
            // Request data from the oracle
            object[] args = new object[] { requestUrl, "get", string.Empty };
            string requestId = (string)Contract.Call(OracleContractHash, "request", CallFlags.All, args);
            
            // Store the request
            StorageMap requests = new StorageMap(Storage.CurrentContext, OracleRequestPrefix);
            requests.Put(requestId, source);
            
            // Emit event
            OnOracleRequest(Runtime.ExecutingScriptHash, requestId, source);
            
            return requestId;
        }
        
        /// <summary>
        /// Callback method for oracle responses
        /// </summary>
        /// <param name="requestId">Request ID</param>
        /// <param name="data">Oracle response data</param>
        public static void OracleCallback(string requestId, string data)
        {
            // Check if the caller is the oracle contract
            if (Runtime.CallingScriptHash != OracleContractHash)
                throw new Exception("Unauthorized oracle callback");
            
            // Get the source from the request
            StorageMap requests = new StorageMap(Storage.CurrentContext, OracleRequestPrefix);
            string source = requests.Get(requestId);
            
            if (source == null)
                throw new Exception("Unknown request ID");
            
            // Store the response
            StorageMap responses = new StorageMap(Storage.CurrentContext, OracleResponsePrefix);
            responses.Put(requestId, data);
            
            // Store the latest data for this source
            StorageMap latestData = new StorageMap(Storage.CurrentContext, LatestDataPrefix);
            latestData.Put(source, data);
            
            // Emit event
            UInt256 txHash = Runtime.GetScriptContainer().Hash;
            OnOracleResponse(txHash, requestId, data);
        }
        
        /// <summary>
        /// Get the latest data for a specific source
        /// </summary>
        /// <param name="source">Data source (weather, sports, social, iot, custom)</param>
        /// <returns>Latest data for the source</returns>
        public static string GetLatestData(string source)
        {
            StorageMap latestData = new StorageMap(Storage.CurrentContext, LatestDataPrefix);
            return latestData.Get(source);
        }
        
        /// <summary>
        /// Get a specific oracle response
        /// </summary>
        /// <param name="requestId">Request ID</param>
        /// <returns>Oracle response data</returns>
        public static string GetOracleResponse(string requestId)
        {
            StorageMap responses = new StorageMap(Storage.CurrentContext, OracleResponsePrefix);
            return responses.Get(requestId);
        }
    }
}
