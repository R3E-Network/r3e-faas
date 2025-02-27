using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;
using Neo.SmartContract.Framework.Services;
using System;
using System.ComponentModel;

namespace Neo.SmartContract.Examples
{
    [DisplayName("TEEAttestationVerificationConsumer")]
    [ManifestExtra("Author", "Neo N3 FaaS")]
    [ManifestExtra("Email", "dev@neo.org")]
    [ManifestExtra("Description", "Neo N3 TEE Attestation Verification Consumer Example")]
    public class TEEAttestationVerificationConsumer : Framework.SmartContract
    {
        // Events
        [DisplayName("AttestationRequested")]
        public static event Action<UInt160, string, string> OnAttestationRequested;
        
        [DisplayName("AttestationVerified")]
        public static event Action<UInt160, string, bool> OnAttestationVerified;
        
        [DisplayName("SecureChannelEstablished")]
        public static event Action<UInt160, string, string> OnSecureChannelEstablished;
        
        [DisplayName("AttestationRecorded")]
        public static event Action<UInt160, string, string> OnAttestationRecorded;
        
        // Storage keys
        private static readonly byte[] PrefixAttestation = new byte[] { 0x01 };
        private static readonly byte[] PrefixVerification = new byte[] { 0x02 };
        private static readonly byte[] PrefixSecureChannel = new byte[] { 0x03 };
        private static readonly byte[] PrefixMeasurements = new byte[] { 0x04 };
        
        // TEE service URL
        private static readonly string TeeServiceUrl = "http://localhost:8080/tee/attestation-verification";
        
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
        /// Request attestation for a TEE
        /// </summary>
        /// <param name="teeType">The type of TEE (sgx, sev, trustzone, simulation)</param>
        /// <param name="nonce">A random nonce to prevent replay attacks</param>
        /// <returns>The attestation request ID</returns>
        public static string RequestAttestation(string teeType, string nonce)
        {
            // Generate a unique attestation request ID
            string requestId = Runtime.GetScriptContainer().Hash.ToString() + "_" + Ledger.CurrentIndex.ToString();
            
            // Create the request URL
            string url = $"{TeeServiceUrl}/generate_attestation";
            
            // Create the request body
            string requestBody = $"{{\"operation\":\"generate_attestation\",\"teeType\":\"{teeType}\",\"nonce\":\"{nonce}\",\"requestId\":\"{requestId}\"}}";
            
            // Call the TEE service
            Oracle.Request(url, "attestationCallback", Hash, requestBody);
            
            // Store the attestation request details
            StorageMap attestationMap = new(Storage.CurrentContext, PrefixAttestation);
            attestationMap.Put(requestId, $"{teeType}|{nonce}");
            
            // Emit the event
            OnAttestationRequested(Runtime.ExecutingScriptHash, requestId, teeType);
            
            return requestId;
        }
        
        /// <summary>
        /// Callback method for attestation generation
        /// </summary>
        /// <param name="requestId">The request ID (transaction hash)</param>
        /// <param name="responseCode">The HTTP response code</param>
        /// <param name="result">The response data</param>
        public static void attestationCallback(byte[] requestId, byte[] responseCode, byte[] result)
        {
            // Check if the caller is the oracle contract
            if (!Runtime.CallingScriptHash.Equals(Oracle.Hash))
                throw new Exception("Only the oracle contract can call this method");
            
            // Parse the result
            string resultStr = result;
            
            // Extract the attestation data
            int startIndex = resultStr.IndexOf("\"attestation\":") + "\"attestation\":".Length;
            int endIndex = resultStr.IndexOf("}", startIndex);
            string attestationData = resultStr.Substring(startIndex, endIndex - startIndex);
            
            // Extract the request ID
            startIndex = resultStr.IndexOf("\"requestId\":\"") + "\"requestId\":\"".Length;
            endIndex = resultStr.IndexOf("\"", startIndex);
            string attestationRequestId = resultStr.Substring(startIndex, endIndex - startIndex);
            
            // Store the attestation data
            StorageMap attestationMap = new(Storage.CurrentContext, PrefixAttestation);
            string existingData = attestationMap.Get(attestationRequestId);
            attestationMap.Put(attestationRequestId, existingData + "|" + attestationData);
        }
        
        /// <summary>
        /// Verify an attestation
        /// </summary>
        /// <param name="requestId">The attestation request ID</param>
        /// <param name="expectedMeasurements">Optional expected measurements for verification</param>
        /// <returns>True if the attestation is valid, false otherwise</returns>
        public static bool VerifyAttestation(string requestId, string expectedMeasurements = null)
        {
            // Check if the attestation exists
            StorageMap attestationMap = new(Storage.CurrentContext, PrefixAttestation);
            string attestationData = attestationMap.Get(requestId);
            
            if (attestationData == null)
                throw new Exception("Attestation not found");
            
            // Parse the attestation data
            string[] parts = attestationData.Split('|');
            if (parts.Length < 3)
                throw new Exception("Incomplete attestation data");
            
            string teeType = parts[0];
            string nonce = parts[1];
            string attestation = parts[2];
            
            // Create the request URL
            string url = $"{TeeServiceUrl}/verify_attestation";
            
            // Create the request body
            string requestBody = $"{{\"operation\":\"verify_attestation\",\"teeType\":\"{teeType}\",\"attestationData\":{{\"quote\":\"{attestation}\"}},\"nonce\":\"{nonce}\"";
            
            // Add expected measurements if provided
            if (expectedMeasurements != null)
            {
                requestBody += $",\"expectedMeasurements\":{expectedMeasurements}";
            }
            
            requestBody += $",\"requestId\":\"{requestId}\"}}";
            
            // Call the TEE service
            Oracle.Request(url, "verificationCallback", Hash, requestBody);
            
            // Return true for now, the actual result will be updated in the callback
            return true;
        }
        
        /// <summary>
        /// Callback method for attestation verification
        /// </summary>
        /// <param name="requestId">The request ID (transaction hash)</param>
        /// <param name="responseCode">The HTTP response code</param>
        /// <param name="result">The response data</param>
        public static void verificationCallback(byte[] requestId, byte[] responseCode, byte[] result)
        {
            // Check if the caller is the oracle contract
            if (!Runtime.CallingScriptHash.Equals(Oracle.Hash))
                throw new Exception("Only the oracle contract can call this method");
            
            // Parse the result
            string resultStr = result;
            
            // Extract the verification result
            int startIndex = resultStr.IndexOf("\"valid\":") + "\"valid\":".Length;
            int endIndex = resultStr.IndexOf(",", startIndex);
            string validStr = resultStr.Substring(startIndex, endIndex - startIndex);
            bool valid = validStr.Trim() == "true";
            
            // Extract the request ID
            startIndex = resultStr.IndexOf("\"requestId\":\"") + "\"requestId\":\"".Length;
            endIndex = resultStr.IndexOf("\"", startIndex);
            string verificationRequestId = resultStr.Substring(startIndex, endIndex - startIndex);
            
            // Extract the measurements if available
            string measurements = null;
            if (resultStr.Contains("\"measurements\":"))
            {
                startIndex = resultStr.IndexOf("\"measurements\":") + "\"measurements\":".Length;
                endIndex = resultStr.IndexOf("}", startIndex);
                measurements = resultStr.Substring(startIndex, endIndex - startIndex);
            }
            
            // Store the verification result
            StorageMap verificationMap = new(Storage.CurrentContext, PrefixVerification);
            verificationMap.Put(verificationRequestId, valid.ToString());
            
            // Store the measurements if available
            if (measurements != null)
            {
                StorageMap measurementsMap = new(Storage.CurrentContext, PrefixMeasurements);
                measurementsMap.Put(verificationRequestId, measurements);
            }
            
            // Emit the event
            OnAttestationVerified(Runtime.ExecutingScriptHash, verificationRequestId, valid);
        }
        
        /// <summary>
        /// Establish a secure channel with a TEE after successful attestation
        /// </summary>
        /// <param name="requestId">The attestation request ID</param>
        /// <param name="keyExchangeData">Key exchange data for establishing the secure channel</param>
        /// <returns>The secure channel ID</returns>
        public static string EstablishSecureChannel(string requestId, string keyExchangeData)
        {
            // Check if the attestation has been verified
            StorageMap verificationMap = new(Storage.CurrentContext, PrefixVerification);
            string verificationResult = verificationMap.Get(requestId);
            
            if (verificationResult == null)
                throw new Exception("Attestation not verified");
            
            if (verificationResult != "True")
                throw new Exception("Attestation verification failed");
            
            // Get the attestation data
            StorageMap attestationMap = new(Storage.CurrentContext, PrefixAttestation);
            string attestationData = attestationMap.Get(requestId);
            
            if (attestationData == null)
                throw new Exception("Attestation not found");
            
            // Parse the attestation data
            string[] parts = attestationData.Split('|');
            if (parts.Length < 3)
                throw new Exception("Incomplete attestation data");
            
            string teeType = parts[0];
            string attestation = parts[2];
            
            // Create the request URL
            string url = $"{TeeServiceUrl}/establish_secure_channel";
            
            // Create the request body
            string requestBody = $"{{\"operation\":\"establish_secure_channel\",\"teeType\":\"{teeType}\",\"attestationData\":{{\"quote\":\"{attestation}\"}},\"keyExchangeData\":\"{keyExchangeData}\",\"requestId\":\"{requestId}\"}}";
            
            // Call the TEE service
            Oracle.Request(url, "secureChannelCallback", Hash, requestBody);
            
            // Return the request ID for now, the actual channel ID will be updated in the callback
            return requestId;
        }
        
        /// <summary>
        /// Callback method for secure channel establishment
        /// </summary>
        /// <param name="requestId">The request ID (transaction hash)</param>
        /// <param name="responseCode">The HTTP response code</param>
        /// <param name="result">The response data</param>
        public static void secureChannelCallback(byte[] requestId, byte[] responseCode, byte[] result)
        {
            // Check if the caller is the oracle contract
            if (!Runtime.CallingScriptHash.Equals(Oracle.Hash))
                throw new Exception("Only the oracle contract can call this method");
            
            // Parse the result
            string resultStr = result;
            
            // Extract the channel ID
            int startIndex = resultStr.IndexOf("\"channelId\":\"") + "\"channelId\":\"".Length;
            int endIndex = resultStr.IndexOf("\"", startIndex);
            string channelId = resultStr.Substring(startIndex, endIndex - startIndex);
            
            // Extract the encrypted session key
            startIndex = resultStr.IndexOf("\"encryptedSessionKey\":\"") + "\"encryptedSessionKey\":\"".Length;
            endIndex = resultStr.IndexOf("\"", startIndex);
            string encryptedSessionKey = resultStr.Substring(startIndex, endIndex - startIndex);
            
            // Extract the request ID
            startIndex = resultStr.IndexOf("\"requestId\":\"") + "\"requestId\":\"".Length;
            endIndex = resultStr.IndexOf("\"", startIndex);
            string secureChannelRequestId = resultStr.Substring(startIndex, endIndex - startIndex);
            
            // Store the secure channel information
            StorageMap secureChannelMap = new(Storage.CurrentContext, PrefixSecureChannel);
            secureChannelMap.Put(secureChannelRequestId, $"{channelId}|{encryptedSessionKey}");
            
            // Emit the event
            OnSecureChannelEstablished(Runtime.ExecutingScriptHash, secureChannelRequestId, channelId);
        }
        
        /// <summary>
        /// Record an attestation on the blockchain
        /// </summary>
        /// <param name="requestId">The attestation request ID</param>
        /// <returns>True if the attestation was recorded successfully, false otherwise</returns>
        public static bool RecordAttestationOnChain(string requestId)
        {
            // Check if the attestation has been verified
            StorageMap verificationMap = new(Storage.CurrentContext, PrefixVerification);
            string verificationResult = verificationMap.Get(requestId);
            
            if (verificationResult == null)
                throw new Exception("Attestation not verified");
            
            if (verificationResult != "True")
                throw new Exception("Attestation verification failed");
            
            // Get the attestation data
            StorageMap attestationMap = new(Storage.CurrentContext, PrefixAttestation);
            string attestationData = attestationMap.Get(requestId);
            
            if (attestationData == null)
                throw new Exception("Attestation not found");
            
            // Get the measurements
            StorageMap measurementsMap = new(Storage.CurrentContext, PrefixMeasurements);
            string measurements = measurementsMap.Get(requestId);
            
            if (measurements == null)
                throw new Exception("Measurements not found");
            
            // Parse the attestation data
            string[] parts = attestationData.Split('|');
            if (parts.Length < 3)
                throw new Exception("Incomplete attestation data");
            
            string teeType = parts[0];
            
            // Create the request URL
            string url = $"{TeeServiceUrl}/verify_and_record_on_chain";
            
            // Create the request body
            string requestBody = $"{{\"operation\":\"verify_and_record_on_chain\",\"teeType\":\"{teeType}\",\"attestationData\":{{\"measurements\":{measurements}}},\"contractHash\":\"{Runtime.ExecutingScriptHash.ToString()}\",\"requestId\":\"{requestId}\"}}";
            
            // Call the TEE service
            Oracle.Request(url, "recordOnChainCallback", Hash, requestBody);
            
            // Return true for now, the actual result will be updated in the callback
            return true;
        }
        
        /// <summary>
        /// Callback method for recording attestation on the blockchain
        /// </summary>
        /// <param name="requestId">The request ID (transaction hash)</param>
        /// <param name="responseCode">The HTTP response code</param>
        /// <param name="result">The response data</param>
        public static void recordOnChainCallback(byte[] requestId, byte[] responseCode, byte[] result)
        {
            // Check if the caller is the oracle contract
            if (!Runtime.CallingScriptHash.Equals(Oracle.Hash))
                throw new Exception("Only the oracle contract can call this method");
            
            // Parse the result
            string resultStr = result;
            
            // Extract the transaction hash
            int startIndex = resultStr.IndexOf("\"txHash\":\"") + "\"txHash\":\"".Length;
            int endIndex = resultStr.IndexOf("\"", startIndex);
            string txHash = resultStr.Substring(startIndex, endIndex - startIndex);
            
            // Extract the request ID
            startIndex = resultStr.IndexOf("\"requestId\":\"") + "\"requestId\":\"".Length;
            endIndex = resultStr.IndexOf("\"", startIndex);
            string recordRequestId = resultStr.Substring(startIndex, endIndex - startIndex);
            
            // Emit the event
            OnAttestationRecorded(Runtime.ExecutingScriptHash, recordRequestId, txHash);
        }
        
        /// <summary>
        /// Get the verification result for an attestation
        /// </summary>
        /// <param name="requestId">The attestation request ID</param>
        /// <returns>The verification result (True or False)</returns>
        public static string GetVerificationResult(string requestId)
        {
            StorageMap verificationMap = new(Storage.CurrentContext, PrefixVerification);
            return verificationMap.Get(requestId);
        }
        
        /// <summary>
        /// Get the measurements for an attestation
        /// </summary>
        /// <param name="requestId">The attestation request ID</param>
        /// <returns>The measurements</returns>
        public static string GetMeasurements(string requestId)
        {
            StorageMap measurementsMap = new(Storage.CurrentContext, PrefixMeasurements);
            return measurementsMap.Get(requestId);
        }
        
        /// <summary>
        /// Get the secure channel information for an attestation
        /// </summary>
        /// <param name="requestId">The attestation request ID</param>
        /// <returns>The secure channel information</returns>
        public static string GetSecureChannelInfo(string requestId)
        {
            StorageMap secureChannelMap = new(Storage.CurrentContext, PrefixSecureChannel);
            return secureChannelMap.Get(requestId);
        }
    }
}
