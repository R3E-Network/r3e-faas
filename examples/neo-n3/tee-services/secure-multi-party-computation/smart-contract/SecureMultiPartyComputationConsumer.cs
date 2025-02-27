using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;
using Neo.SmartContract.Framework.Services;
using System;
using System.ComponentModel;

namespace Neo.SmartContract.Examples
{
    [DisplayName("SecureMultiPartyComputationConsumer")]
    [ManifestExtra("Author", "Neo N3 FaaS")]
    [ManifestExtra("Email", "dev@neo.org")]
    [ManifestExtra("Description", "Neo N3 Secure Multi-Party Computation Consumer Example")]
    public class SecureMultiPartyComputationConsumer : Framework.SmartContract
    {
        // Events
        [DisplayName("ComputationInitiated")]
        public static event Action<UInt160, string, string> OnComputationInitiated;
        
        [DisplayName("PartyJoined")]
        public static event Action<UInt160, string, string> OnPartyJoined;
        
        [DisplayName("InputSubmitted")]
        public static event Action<UInt160, string, string> OnInputSubmitted;
        
        [DisplayName("ComputationCompleted")]
        public static event Action<UInt160, string, string> OnComputationCompleted;
        
        [DisplayName("ResultRetrieved")]
        public static event Action<UInt160, string, string> OnResultRetrieved;
        
        // Storage keys
        private static readonly byte[] PrefixComputationId = new byte[] { 0x01 };
        private static readonly byte[] PrefixParties = new byte[] { 0x02 };
        private static readonly byte[] PrefixInputs = new byte[] { 0x03 };
        private static readonly byte[] PrefixResults = new byte[] { 0x04 };
        private static readonly byte[] PrefixStatus = new byte[] { 0x05 };
        
        // TEE service URL
        private static readonly string TeeServiceUrl = "http://localhost:8080/tee/secure-multi-party-computation";
        
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
        /// Initialize a new secure multi-party computation
        /// </summary>
        /// <param name="protocol">The MPC protocol to use</param>
        /// <param name="function">The function to compute</param>
        /// <param name="threshold">The threshold of parties required to complete the computation</param>
        /// <param name="maxParties">The maximum number of parties allowed to join</param>
        /// <returns>The computation ID</returns>
        public static string InitiateComputation(string protocol, string function, int threshold, int maxParties)
        {
            // Check if the caller is the contract owner
            if (!IsOwner())
                throw new Exception("Only the contract owner can initiate a computation");
            
            // Generate a unique computation ID
            string computationId = Runtime.GetScriptContainer().Hash.ToString() + "_" + Ledger.CurrentIndex.ToString();
            
            // Create the request URL
            string url = $"{TeeServiceUrl}/initiate";
            
            // Create the request body
            string requestBody = $"{{\"operation\":\"initiate\",\"protocol\":\"{protocol}\",\"function\":\"{function}\",\"threshold\":{threshold},\"maxParties\":{maxParties},\"computationId\":\"{computationId}\"}}";
            
            // Call the TEE service
            Oracle.Request(url, "initiateCallback", Hash, requestBody);
            
            // Store the computation details
            StorageMap computationMap = new(Storage.CurrentContext, PrefixComputationId);
            computationMap.Put(computationId, $"{protocol}|{function}|{threshold}|{maxParties}");
            
            StorageMap statusMap = new(Storage.CurrentContext, PrefixStatus);
            statusMap.Put(computationId, "initiated");
            
            // Emit the event
            OnComputationInitiated(Runtime.ExecutingScriptHash, computationId, protocol);
            
            return computationId;
        }
        
        /// <summary>
        /// Callback method for computation initiation
        /// </summary>
        /// <param name="requestId">The request ID (transaction hash)</param>
        /// <param name="responseCode">The HTTP response code</param>
        /// <param name="result">The response data</param>
        public static void initiateCallback(byte[] requestId, byte[] responseCode, byte[] result)
        {
            // Check if the caller is the oracle contract
            if (!Runtime.CallingScriptHash.Equals(Oracle.Hash))
                throw new Exception("Only the oracle contract can call this method");
            
            // Generate the computation ID
            string computationId = requestId.ToHexString() + "_" + Ledger.CurrentIndex.ToString();
            
            // Update the status
            StorageMap statusMap = new(Storage.CurrentContext, PrefixStatus);
            statusMap.Put(computationId, "ready");
        }
        
        /// <summary>
        /// Join a secure multi-party computation as a party
        /// </summary>
        /// <param name="computationId">The computation ID</param>
        /// <param name="partyId">The party ID</param>
        /// <param name="publicKey">The party's public key</param>
        public static void JoinComputation(string computationId, string partyId, string publicKey)
        {
            // Check if the computation exists
            StorageMap computationMap = new(Storage.CurrentContext, PrefixComputationId);
            if (computationMap.Get(computationId) == null)
                throw new Exception("Computation does not exist");
            
            // Check if the computation is in the right state
            StorageMap statusMap = new(Storage.CurrentContext, PrefixStatus);
            string status = statusMap.Get(computationId);
            if (status != "ready" && status != "joining")
                throw new Exception("Computation is not in the joining state");
            
            // Get the computation details
            string[] details = ((string)computationMap.Get(computationId)).Split('|');
            int maxParties = int.Parse(details[3]);
            
            // Check if the maximum number of parties has been reached
            StorageMap partiesMap = new(Storage.CurrentContext, PrefixParties);
            string partiesStr = partiesMap.Get(computationId);
            string[] parties = partiesStr != null ? partiesStr.Split(',') : new string[0];
            if (parties.Length >= maxParties)
                throw new Exception("Maximum number of parties reached");
            
            // Check if the party has already joined
            foreach (string party in parties)
            {
                if (party.Split('|')[0] == partyId)
                    throw new Exception("Party has already joined");
            }
            
            // Create the request URL
            string url = $"{TeeServiceUrl}/join";
            
            // Create the request body
            string requestBody = $"{{\"operation\":\"join\",\"computationId\":\"{computationId}\",\"partyId\":\"{partyId}\",\"publicKey\":\"{publicKey}\"}}";
            
            // Call the TEE service
            Oracle.Request(url, "joinCallback", Hash, requestBody);
            
            // Add the party to the list
            string newParty = $"{partyId}|{publicKey}";
            string newPartiesStr = partiesStr != null ? partiesStr + "," + newParty : newParty;
            partiesMap.Put(computationId, newPartiesStr);
            
            // Update the status
            statusMap.Put(computationId, "joining");
            
            // Emit the event
            OnPartyJoined(Runtime.ExecutingScriptHash, computationId, partyId);
        }
        
        /// <summary>
        /// Callback method for joining a computation
        /// </summary>
        /// <param name="requestId">The request ID (transaction hash)</param>
        /// <param name="responseCode">The HTTP response code</param>
        /// <param name="result">The response data</param>
        public static void joinCallback(byte[] requestId, byte[] responseCode, byte[] result)
        {
            // Check if the caller is the oracle contract
            if (!Runtime.CallingScriptHash.Equals(Oracle.Hash))
                throw new Exception("Only the oracle contract can call this method");
            
            // No additional processing needed
        }
        
        /// <summary>
        /// Submit input for a secure multi-party computation
        /// </summary>
        /// <param name="computationId">The computation ID</param>
        /// <param name="partyId">The party ID</param>
        /// <param name="encryptedInput">The encrypted input</param>
        public static void SubmitInput(string computationId, string partyId, string encryptedInput)
        {
            // Check if the computation exists
            StorageMap computationMap = new(Storage.CurrentContext, PrefixComputationId);
            if (computationMap.Get(computationId) == null)
                throw new Exception("Computation does not exist");
            
            // Check if the computation is in the right state
            StorageMap statusMap = new(Storage.CurrentContext, PrefixStatus);
            string status = statusMap.Get(computationId);
            if (status != "joining" && status != "input")
                throw new Exception("Computation is not in the input state");
            
            // Check if the party has joined
            StorageMap partiesMap = new(Storage.CurrentContext, PrefixParties);
            string partiesStr = partiesMap.Get(computationId);
            bool partyFound = false;
            foreach (string party in partiesStr.Split(','))
            {
                if (party.Split('|')[0] == partyId)
                {
                    partyFound = true;
                    break;
                }
            }
            if (!partyFound)
                throw new Exception("Party has not joined the computation");
            
            // Check if the party has already submitted input
            StorageMap inputsMap = new(Storage.CurrentContext, PrefixInputs);
            string inputsStr = inputsMap.Get(computationId);
            if (inputsStr != null)
            {
                foreach (string input in inputsStr.Split(','))
                {
                    if (input.Split('|')[0] == partyId)
                        throw new Exception("Party has already submitted input");
                }
            }
            
            // Create the request URL
            string url = $"{TeeServiceUrl}/input";
            
            // Create the request body
            string requestBody = $"{{\"operation\":\"input\",\"computationId\":\"{computationId}\",\"partyId\":\"{partyId}\",\"encryptedInput\":\"{encryptedInput}\"}}";
            
            // Call the TEE service
            Oracle.Request(url, "inputCallback", Hash, requestBody);
            
            // Add the input to the list
            string newInput = $"{partyId}|{encryptedInput}";
            string newInputsStr = inputsStr != null ? inputsStr + "," + newInput : newInput;
            inputsMap.Put(computationId, newInputsStr);
            
            // Update the status
            statusMap.Put(computationId, "input");
            
            // Emit the event
            OnInputSubmitted(Runtime.ExecutingScriptHash, computationId, partyId);
            
            // Check if all parties have submitted input
            string[] details = ((string)computationMap.Get(computationId)).Split('|');
            int threshold = int.Parse(details[2]);
            string[] inputs = newInputsStr.Split(',');
            if (inputs.Length >= threshold)
            {
                // Start the computation
                StartComputation(computationId);
            }
        }
        
        /// <summary>
        /// Callback method for submitting input
        /// </summary>
        /// <param name="requestId">The request ID (transaction hash)</param>
        /// <param name="responseCode">The HTTP response code</param>
        /// <param name="result">The response data</param>
        public static void inputCallback(byte[] requestId, byte[] responseCode, byte[] result)
        {
            // Check if the caller is the oracle contract
            if (!Runtime.CallingScriptHash.Equals(Oracle.Hash))
                throw new Exception("Only the oracle contract can call this method");
            
            // No additional processing needed
        }
        
        /// <summary>
        /// Start the computation once enough parties have submitted input
        /// </summary>
        /// <param name="computationId">The computation ID</param>
        private static void StartComputation(string computationId)
        {
            // Check if the computation exists
            StorageMap computationMap = new(Storage.CurrentContext, PrefixComputationId);
            if (computationMap.Get(computationId) == null)
                throw new Exception("Computation does not exist");
            
            // Check if the computation is in the right state
            StorageMap statusMap = new(Storage.CurrentContext, PrefixStatus);
            string status = statusMap.Get(computationId);
            if (status != "input")
                throw new Exception("Computation is not in the input state");
            
            // Create the request URL
            string url = $"{TeeServiceUrl}/compute";
            
            // Create the request body
            string requestBody = $"{{\"operation\":\"compute\",\"computationId\":\"{computationId}\"}}";
            
            // Call the TEE service
            Oracle.Request(url, "computeCallback", Hash, requestBody);
            
            // Update the status
            statusMap.Put(computationId, "computing");
        }
        
        /// <summary>
        /// Callback method for computation
        /// </summary>
        /// <param name="requestId">The request ID (transaction hash)</param>
        /// <param name="responseCode">The HTTP response code</param>
        /// <param name="result">The response data</param>
        public static void computeCallback(byte[] requestId, byte[] responseCode, byte[] result)
        {
            // Check if the caller is the oracle contract
            if (!Runtime.CallingScriptHash.Equals(Oracle.Hash))
                throw new Exception("Only the oracle contract can call this method");
            
            // Parse the result
            string resultStr = result;
            
            // Extract the computation ID
            int startIndex = resultStr.IndexOf("\"computationId\":\"") + "\"computationId\":\"".Length;
            int endIndex = resultStr.IndexOf("\"", startIndex);
            string computationId = resultStr.Substring(startIndex, endIndex - startIndex);
            
            // Extract the encrypted results
            startIndex = resultStr.IndexOf("\"encryptedResults\":{") + "\"encryptedResults\":{".Length;
            endIndex = resultStr.IndexOf("}", startIndex);
            string encryptedResults = resultStr.Substring(startIndex, endIndex - startIndex);
            
            // Store the results
            StorageMap resultsMap = new(Storage.CurrentContext, PrefixResults);
            resultsMap.Put(computationId, encryptedResults);
            
            // Update the status
            StorageMap statusMap = new(Storage.CurrentContext, PrefixStatus);
            statusMap.Put(computationId, "completed");
            
            // Emit the event
            OnComputationCompleted(Runtime.ExecutingScriptHash, computationId, encryptedResults);
        }
        
        /// <summary>
        /// Retrieve the result of a computation
        /// </summary>
        /// <param name="computationId">The computation ID</param>
        /// <param name="partyId">The party ID</param>
        /// <returns>The encrypted result for the party</returns>
        public static string RetrieveResult(string computationId, string partyId)
        {
            // Check if the computation exists
            StorageMap computationMap = new(Storage.CurrentContext, PrefixComputationId);
            if (computationMap.Get(computationId) == null)
                throw new Exception("Computation does not exist");
            
            // Check if the computation is completed
            StorageMap statusMap = new(Storage.CurrentContext, PrefixStatus);
            string status = statusMap.Get(computationId);
            if (status != "completed")
                throw new Exception("Computation is not completed");
            
            // Check if the party has joined
            StorageMap partiesMap = new(Storage.CurrentContext, PrefixParties);
            string partiesStr = partiesMap.Get(computationId);
            bool partyFound = false;
            foreach (string party in partiesStr.Split(','))
            {
                if (party.Split('|')[0] == partyId)
                {
                    partyFound = true;
                    break;
                }
            }
            if (!partyFound)
                throw new Exception("Party has not joined the computation");
            
            // Get the results
            StorageMap resultsMap = new(Storage.CurrentContext, PrefixResults);
            string resultsStr = resultsMap.Get(computationId);
            
            // Parse the results to find the party's result
            string[] results = resultsStr.Split(',');
            foreach (string result in results)
            {
                string[] parts = result.Split(':');
                if (parts[0].Trim('"') == partyId)
                {
                    // Emit the event
                    OnResultRetrieved(Runtime.ExecutingScriptHash, computationId, partyId);
                    
                    return parts[1].Trim('"');
                }
            }
            
            throw new Exception("Result not found for party");
        }
        
        /// <summary>
        /// Get the status of a computation
        /// </summary>
        /// <param name="computationId">The computation ID</param>
        /// <returns>The status of the computation</returns>
        public static string GetComputationStatus(string computationId)
        {
            // Check if the computation exists
            StorageMap computationMap = new(Storage.CurrentContext, PrefixComputationId);
            if (computationMap.Get(computationId) == null)
                throw new Exception("Computation does not exist");
            
            // Get the status
            StorageMap statusMap = new(Storage.CurrentContext, PrefixStatus);
            return statusMap.Get(computationId);
        }
        
        /// <summary>
        /// Get the parties of a computation
        /// </summary>
        /// <param name="computationId">The computation ID</param>
        /// <returns>The parties of the computation</returns>
        public static string GetComputationParties(string computationId)
        {
            // Check if the computation exists
            StorageMap computationMap = new(Storage.CurrentContext, PrefixComputationId);
            if (computationMap.Get(computationId) == null)
                throw new Exception("Computation does not exist");
            
            // Get the parties
            StorageMap partiesMap = new(Storage.CurrentContext, PrefixParties);
            return partiesMap.Get(computationId);
        }
        
        /// <summary>
        /// Get the details of a computation
        /// </summary>
        /// <param name="computationId">The computation ID</param>
        /// <returns>The details of the computation</returns>
        public static string GetComputationDetails(string computationId)
        {
            // Check if the computation exists
            StorageMap computationMap = new(Storage.CurrentContext, PrefixComputationId);
            if (computationMap.Get(computationId) == null)
                throw new Exception("Computation does not exist");
            
            // Get the details
            return computationMap.Get(computationId);
        }
    }
}
