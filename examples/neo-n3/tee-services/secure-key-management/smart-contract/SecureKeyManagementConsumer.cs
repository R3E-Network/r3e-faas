using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;
using Neo.SmartContract.Framework.Services;
using System;
using System.ComponentModel;

namespace Neo.SmartContract.Examples
{
    [DisplayName("SecureKeyManagementConsumer")]
    [ManifestExtra("Author", "Neo N3 FaaS")]
    [ManifestExtra("Email", "dev@neo.org")]
    [ManifestExtra("Description", "Neo N3 Secure Key Management Consumer Example")]
    public class SecureKeyManagementConsumer : Framework.SmartContract
    {
        // Events
        [DisplayName("KeyGenerated")]
        public static event Action<UInt160, string, string> OnKeyGenerated;
        
        [DisplayName("SignatureCreated")]
        public static event Action<UInt160, string, byte[]> OnSignatureCreated;
        
        [DisplayName("SignatureVerified")]
        public static event Action<UInt160, string, bool> OnSignatureVerified;
        
        [DisplayName("DataEncrypted")]
        public static event Action<UInt160, string> OnDataEncrypted;
        
        [DisplayName("DataDecrypted")]
        public static event Action<UInt160, string> OnDataDecrypted;
        
        // Storage keys
        private static readonly byte[] PrefixKeyId = new byte[] { 0x01 };
        private static readonly byte[] PrefixSignature = new byte[] { 0x02 };
        private static readonly byte[] PrefixEncryptedData = new byte[] { 0x03 };
        private static readonly byte[] PrefixDecryptedData = new byte[] { 0x04 };
        
        // TEE service URL
        private static readonly string TeeServiceUrl = "http://localhost:8080/tee/key-management";
        
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
        /// Generate a new key pair in the TEE
        /// </summary>
        /// <param name="algorithm">The algorithm to use (secp256r1, secp256k1, ed25519)</param>
        /// <param name="keyName">A name for the key</param>
        public static void GenerateKeyPair(string algorithm, string keyName)
        {
            // Check if the caller is the contract owner
            if (!IsOwner())
                throw new Exception("Only the contract owner can generate keys");
            
            // Create the request URL with the algorithm parameter
            string url = $"{TeeServiceUrl}/generate?algorithm={algorithm}&keyName={keyName}";
            
            // Call the TEE service
            Oracle.Request(url, "generateKeyCallback", Hash, null);
            
            // Get the request ID (transaction hash)
            UInt256 requestId = Runtime.GetScriptContainer().Hash;
            
            // Store the request details
            StorageMap keyNameMap = new(Storage.CurrentContext, PrefixKeyId);
            keyNameMap.Put(requestId.ToArray(), keyName);
            
            // Emit the event
            OnKeyGenerated(Runtime.ExecutingScriptHash, algorithm, keyName);
        }
        
        /// <summary>
        /// Callback method for key generation
        /// </summary>
        /// <param name="requestId">The request ID (transaction hash)</param>
        /// <param name="responseCode">The HTTP response code</param>
        /// <param name="result">The response data</param>
        public static void generateKeyCallback(byte[] requestId, byte[] responseCode, byte[] result)
        {
            // Check if the caller is the oracle contract
            if (!Runtime.CallingScriptHash.Equals(Oracle.Hash))
                throw new Exception("Only the oracle contract can call this method");
            
            // Get the key name
            StorageMap keyNameMap = new(Storage.CurrentContext, PrefixKeyId);
            string keyName = keyNameMap.Get(requestId);
            
            // Store the key ID
            StorageMap keyIdMap = new(Storage.CurrentContext, keyName);
            keyIdMap.Put("keyId", result);
        }
        
        /// <summary>
        /// Sign data using a key stored in the TEE
        /// </summary>
        /// <param name="keyName">The name of the key to use</param>
        /// <param name="data">The data to sign</param>
        public static void SignData(string keyName, string data)
        {
            // Check if the caller is the contract owner
            if (!IsOwner())
                throw new Exception("Only the contract owner can sign data");
            
            // Get the key ID
            StorageMap keyIdMap = new(Storage.CurrentContext, keyName);
            byte[] keyId = keyIdMap.Get("keyId");
            
            if (keyId == null || keyId.Length == 0)
                throw new Exception($"Key {keyName} not found");
            
            // Create the request URL with the key ID and data parameters
            string url = $"{TeeServiceUrl}/sign?keyId={System.Text.Encoding.UTF8.GetString(keyId)}&data={data}";
            
            // Call the TEE service
            Oracle.Request(url, "signDataCallback", Hash, null);
            
            // Get the request ID (transaction hash)
            UInt256 requestId = Runtime.GetScriptContainer().Hash;
            
            // Store the request details
            StorageMap signDataMap = new(Storage.CurrentContext, PrefixSignature);
            signDataMap.Put(requestId.ToArray(), keyName);
        }
        
        /// <summary>
        /// Callback method for data signing
        /// </summary>
        /// <param name="requestId">The request ID (transaction hash)</param>
        /// <param name="responseCode">The HTTP response code</param>
        /// <param name="result">The response data</param>
        public static void signDataCallback(byte[] requestId, byte[] responseCode, byte[] result)
        {
            // Check if the caller is the oracle contract
            if (!Runtime.CallingScriptHash.Equals(Oracle.Hash))
                throw new Exception("Only the oracle contract can call this method");
            
            // Get the key name
            StorageMap signDataMap = new(Storage.CurrentContext, PrefixSignature);
            string keyName = signDataMap.Get(requestId);
            
            // Store the signature
            StorageMap keyIdMap = new(Storage.CurrentContext, keyName);
            keyIdMap.Put("signature", result);
            
            // Emit the event
            OnSignatureCreated(Runtime.ExecutingScriptHash, keyName, result);
        }
        
        /// <summary>
        /// Verify a signature using a key stored in the TEE
        /// </summary>
        /// <param name="keyName">The name of the key to use</param>
        /// <param name="data">The original data</param>
        public static void VerifySignature(string keyName, string data)
        {
            // Check if the caller is the contract owner
            if (!IsOwner())
                throw new Exception("Only the contract owner can verify signatures");
            
            // Get the key ID
            StorageMap keyIdMap = new(Storage.CurrentContext, keyName);
            byte[] keyId = keyIdMap.Get("keyId");
            
            if (keyId == null || keyId.Length == 0)
                throw new Exception($"Key {keyName} not found");
            
            // Get the signature
            byte[] signature = keyIdMap.Get("signature");
            
            if (signature == null || signature.Length == 0)
                throw new Exception($"No signature found for key {keyName}");
            
            // Create the request URL with the key ID, data, and signature parameters
            string url = $"{TeeServiceUrl}/verify?keyId={System.Text.Encoding.UTF8.GetString(keyId)}&data={data}&signature={System.Text.Encoding.UTF8.GetString(signature)}";
            
            // Call the TEE service
            Oracle.Request(url, "verifySignatureCallback", Hash, null);
            
            // Get the request ID (transaction hash)
            UInt256 requestId = Runtime.GetScriptContainer().Hash;
            
            // Store the request details
            StorageMap verifySignatureMap = new(Storage.CurrentContext, keyName);
            verifySignatureMap.Put("verifyRequest", requestId.ToArray());
        }
        
        /// <summary>
        /// Callback method for signature verification
        /// </summary>
        /// <param name="requestId">The request ID (transaction hash)</param>
        /// <param name="responseCode">The HTTP response code</param>
        /// <param name="result">The response data</param>
        public static void verifySignatureCallback(byte[] requestId, byte[] responseCode, byte[] result)
        {
            // Check if the caller is the oracle contract
            if (!Runtime.CallingScriptHash.Equals(Oracle.Hash))
                throw new Exception("Only the oracle contract can call this method");
            
            // Find the key name for this request
            string keyName = null;
            StorageMap keyMap;
            
            // Iterate through all keys to find the one with this request ID
            // In a real implementation, this would be more efficient
            string[] keyNames = new string[] { "key1", "key2", "key3" }; // Example key names
            foreach (string name in keyNames)
            {
                keyMap = new(Storage.CurrentContext, name);
                byte[] storedRequestId = keyMap.Get("verifyRequest");
                if (storedRequestId != null && storedRequestId.Length > 0 && storedRequestId.Equals(requestId))
                {
                    keyName = name;
                    break;
                }
            }
            
            if (keyName == null)
                throw new Exception("Key name not found for this verification request");
            
            // Parse the result
            bool isValid = System.Text.Encoding.UTF8.GetString(result).Equals("true");
            
            // Store the verification result
            keyMap = new(Storage.CurrentContext, keyName);
            keyMap.Put("verificationResult", isValid ? "true" : "false");
            
            // Emit the event
            OnSignatureVerified(Runtime.ExecutingScriptHash, keyName, isValid);
        }
        
        /// <summary>
        /// Encrypt data using a key stored in the TEE
        /// </summary>
        /// <param name="keyName">The name of the key to use</param>
        /// <param name="data">The data to encrypt</param>
        public static void EncryptData(string keyName, string data)
        {
            // Check if the caller is the contract owner
            if (!IsOwner())
                throw new Exception("Only the contract owner can encrypt data");
            
            // Get the key ID
            StorageMap keyIdMap = new(Storage.CurrentContext, keyName);
            byte[] keyId = keyIdMap.Get("keyId");
            
            if (keyId == null || keyId.Length == 0)
                throw new Exception($"Key {keyName} not found");
            
            // Create the request URL with the key ID and data parameters
            string url = $"{TeeServiceUrl}/encrypt?keyId={System.Text.Encoding.UTF8.GetString(keyId)}&data={data}";
            
            // Call the TEE service
            Oracle.Request(url, "encryptDataCallback", Hash, null);
            
            // Get the request ID (transaction hash)
            UInt256 requestId = Runtime.GetScriptContainer().Hash;
            
            // Store the request details
            StorageMap encryptDataMap = new(Storage.CurrentContext, PrefixEncryptedData);
            encryptDataMap.Put(requestId.ToArray(), keyName);
        }
        
        /// <summary>
        /// Callback method for data encryption
        /// </summary>
        /// <param name="requestId">The request ID (transaction hash)</param>
        /// <param name="responseCode">The HTTP response code</param>
        /// <param name="result">The response data</param>
        public static void encryptDataCallback(byte[] requestId, byte[] responseCode, byte[] result)
        {
            // Check if the caller is the oracle contract
            if (!Runtime.CallingScriptHash.Equals(Oracle.Hash))
                throw new Exception("Only the oracle contract can call this method");
            
            // Get the key name
            StorageMap encryptDataMap = new(Storage.CurrentContext, PrefixEncryptedData);
            string keyName = encryptDataMap.Get(requestId);
            
            // Store the encrypted data
            StorageMap keyIdMap = new(Storage.CurrentContext, keyName);
            keyIdMap.Put("encryptedData", result);
            
            // Emit the event
            OnDataEncrypted(Runtime.ExecutingScriptHash, keyName);
        }
        
        /// <summary>
        /// Decrypt data using a key stored in the TEE
        /// </summary>
        /// <param name="keyName">The name of the key to use</param>
        public static void DecryptData(string keyName)
        {
            // Check if the caller is the contract owner
            if (!IsOwner())
                throw new Exception("Only the contract owner can decrypt data");
            
            // Get the key ID
            StorageMap keyIdMap = new(Storage.CurrentContext, keyName);
            byte[] keyId = keyIdMap.Get("keyId");
            
            if (keyId == null || keyId.Length == 0)
                throw new Exception($"Key {keyName} not found");
            
            // Get the encrypted data
            byte[] encryptedData = keyIdMap.Get("encryptedData");
            
            if (encryptedData == null || encryptedData.Length == 0)
                throw new Exception($"No encrypted data found for key {keyName}");
            
            // Create the request URL with the key ID and encrypted data parameters
            string url = $"{TeeServiceUrl}/decrypt?keyId={System.Text.Encoding.UTF8.GetString(keyId)}&encryptedData={System.Text.Encoding.UTF8.GetString(encryptedData)}";
            
            // Call the TEE service
            Oracle.Request(url, "decryptDataCallback", Hash, null);
            
            // Get the request ID (transaction hash)
            UInt256 requestId = Runtime.GetScriptContainer().Hash;
            
            // Store the request details
            StorageMap decryptDataMap = new(Storage.CurrentContext, PrefixDecryptedData);
            decryptDataMap.Put(requestId.ToArray(), keyName);
        }
        
        /// <summary>
        /// Callback method for data decryption
        /// </summary>
        /// <param name="requestId">The request ID (transaction hash)</param>
        /// <param name="responseCode">The HTTP response code</param>
        /// <param name="result">The response data</param>
        public static void decryptDataCallback(byte[] requestId, byte[] responseCode, byte[] result)
        {
            // Check if the caller is the oracle contract
            if (!Runtime.CallingScriptHash.Equals(Oracle.Hash))
                throw new Exception("Only the oracle contract can call this method");
            
            // Get the key name
            StorageMap decryptDataMap = new(Storage.CurrentContext, PrefixDecryptedData);
            string keyName = decryptDataMap.Get(requestId);
            
            // Store the decrypted data
            StorageMap keyIdMap = new(Storage.CurrentContext, keyName);
            keyIdMap.Put("decryptedData", result);
            
            // Emit the event
            OnDataDecrypted(Runtime.ExecutingScriptHash, keyName);
        }
        
        /// <summary>
        /// Get the key ID for a key name
        /// </summary>
        /// <param name="keyName">The name of the key</param>
        /// <returns>The key ID</returns>
        public static byte[] GetKeyId(string keyName)
        {
            StorageMap keyIdMap = new(Storage.CurrentContext, keyName);
            return keyIdMap.Get("keyId");
        }
        
        /// <summary>
        /// Get the signature for a key name
        /// </summary>
        /// <param name="keyName">The name of the key</param>
        /// <returns>The signature</returns>
        public static byte[] GetSignature(string keyName)
        {
            StorageMap keyIdMap = new(Storage.CurrentContext, keyName);
            return keyIdMap.Get("signature");
        }
        
        /// <summary>
        /// Get the verification result for a key name
        /// </summary>
        /// <param name="keyName">The name of the key</param>
        /// <returns>The verification result</returns>
        public static bool GetVerificationResult(string keyName)
        {
            StorageMap keyIdMap = new(Storage.CurrentContext, keyName);
            byte[] result = keyIdMap.Get("verificationResult");
            
            if (result == null || result.Length == 0)
                return false;
            
            return System.Text.Encoding.UTF8.GetString(result).Equals("true");
        }
        
        /// <summary>
        /// Get the encrypted data for a key name
        /// </summary>
        /// <param name="keyName">The name of the key</param>
        /// <returns>The encrypted data</returns>
        public static byte[] GetEncryptedData(string keyName)
        {
            StorageMap keyIdMap = new(Storage.CurrentContext, keyName);
            return keyIdMap.Get("encryptedData");
        }
        
        /// <summary>
        /// Get the decrypted data for a key name
        /// </summary>
        /// <param name="keyName">The name of the key</param>
        /// <returns>The decrypted data</returns>
        public static byte[] GetDecryptedData(string keyName)
        {
            StorageMap keyIdMap = new(Storage.CurrentContext, keyName);
            return keyIdMap.Get("decryptedData");
        }
    }
}
