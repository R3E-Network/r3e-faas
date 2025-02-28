using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;
using Neo.SmartContract.Framework.Services;
using System;
using System.ComponentModel;
using System.Numerics;

namespace Neo.SmartContract.Examples
{
    [DisplayName("BlockchainGatewayContract")]
    [ManifestExtra("Author", "R3E Network")]
    [ManifestExtra("Email", "dev@r3e.network")]
    [ManifestExtra("Description", "Gateway contract for meta transactions and oracle data in the R3E FaaS platform")]
    public class BlockchainGatewayContract : Framework.SmartContract
    {
        // Events
        [DisplayName("MetaTxExecuted")]
        public static event Action<UInt160, UInt256, string, string> OnMetaTxExecuted;
        
        [DisplayName("MetaTxRejected")]
        public static event Action<UInt160, string, string> OnMetaTxRejected;
        
        [DisplayName("PriceDataUpdated")]
        public static event Action<byte, string, BigInteger, UInt256> OnPriceDataUpdated;
        
        // Storage keys
        private static readonly byte[] RelayerPrefix = new byte[] { 0x01 };
        private static readonly byte[] GasBankPrefix = new byte[] { 0x02 };
        private static readonly byte[] ExecutedTxPrefix = new byte[] { 0x03 };
        private static readonly byte[] ContractOwnerKey = new byte[] { 0x04 };
        private static readonly byte[] PriceDataPrefix = new byte[] { 0x05 };
        private static readonly byte[] PriceIndexMapPrefix = new byte[] { 0x06 };
        private static readonly byte[] LastUpdateTimePrefix = new byte[] { 0x07 };
        
        // Transaction types
        private static readonly string NeoTxType = "neo";
        private static readonly string EthereumTxType = "ethereum";
        
        // Price feed indices
        private static readonly byte NEO_USD_INDEX = 0;
        private static readonly byte GAS_USD_INDEX = 1;
        private static readonly byte BTC_USD_INDEX = 2;
        private static readonly byte ETH_USD_INDEX = 3;
        
        // Oracle contract hash
        private static readonly UInt160 OracleContractHash = "0x0000000000000000000000000000000000000000".ToScriptHash();
        
        // Contract owner
        private static UInt160 Owner => (UInt160)Storage.Get(Storage.CurrentContext, ContractOwnerKey);
        
        /// <summary>
        /// Contract initialization
        /// </summary>
        public static void _deploy(object data, bool update)
        {
            if (update) return;
            
            // Set contract owner
            Storage.Put(Storage.CurrentContext, ContractOwnerKey, (ByteString)Runtime.ExecutingScriptHash);
            
            // Initialize price index map
            StorageMap priceIndexMap = new StorageMap(Storage.CurrentContext, PriceIndexMapPrefix);
            priceIndexMap.Put("NEO/USD", NEO_USD_INDEX);
            priceIndexMap.Put("GAS/USD", GAS_USD_INDEX);
            priceIndexMap.Put("BTC/USD", BTC_USD_INDEX);
            priceIndexMap.Put("ETH/USD", ETH_USD_INDEX);
        }
        
        #region Meta Transaction Functions
        
        /// <summary>
        /// Add a relayer to the authorized relayers list
        /// </summary>
        /// <param name="relayerAddress">Relayer address</param>
        public static void AddRelayer(UInt160 relayerAddress)
        {
            // Only contract owner can add relayers
            if (!Runtime.CheckWitness(Owner))
                throw new Exception("Not authorized");
            
            // Add relayer to the authorized list
            StorageMap relayers = new StorageMap(Storage.CurrentContext, RelayerPrefix);
            relayers.Put(relayerAddress, 1);
        }
        
        /// <summary>
        /// Remove a relayer from the authorized relayers list
        /// </summary>
        /// <param name="relayerAddress">Relayer address</param>
        public static void RemoveRelayer(UInt160 relayerAddress)
        {
            // Only contract owner can remove relayers
            if (!Runtime.CheckWitness(Owner))
                throw new Exception("Not authorized");
            
            // Remove relayer from the authorized list
            StorageMap relayers = new StorageMap(Storage.CurrentContext, RelayerPrefix);
            relayers.Delete(relayerAddress);
        }
        
        /// <summary>
        /// Check if an address is an authorized relayer
        /// </summary>
        /// <param name="relayerAddress">Relayer address</param>
        /// <returns>True if the address is an authorized relayer, false otherwise</returns>
        public static bool IsRelayer(UInt160 relayerAddress)
        {
            StorageMap relayers = new StorageMap(Storage.CurrentContext, RelayerPrefix);
            return relayers.Get(relayerAddress) != null;
        }
        
        /// <summary>
        /// Set the Gas Bank account for a contract
        /// </summary>
        /// <param name="contractHash">Contract hash</param>
        /// <param name="gasBankAccount">Gas Bank account address</param>
        public static void SetGasBankAccount(UInt160 contractHash, UInt160 gasBankAccount)
        {
            // Only contract owner can set Gas Bank accounts
            if (!Runtime.CheckWitness(Owner))
                throw new Exception("Not authorized");
            
            // Set Gas Bank account for the contract
            StorageMap gasBankAccounts = new StorageMap(Storage.CurrentContext, GasBankPrefix);
            gasBankAccounts.Put(contractHash, gasBankAccount);
        }
        
        /// <summary>
        /// Get the Gas Bank account for a contract
        /// </summary>
        /// <param name="contractHash">Contract hash</param>
        /// <returns>Gas Bank account address</returns>
        public static UInt160 GetGasBankAccount(UInt160 contractHash)
        {
            StorageMap gasBankAccounts = new StorageMap(Storage.CurrentContext, GasBankPrefix);
            ByteString gasBankAccount = gasBankAccounts.Get(contractHash);
            
            if (gasBankAccount == null)
                throw new Exception("Gas Bank account not found for contract");
            
            return (UInt160)gasBankAccount;
        }
        
        /// <summary>
        /// Execute a meta transaction
        /// </summary>
        /// <param name="sender">Original sender address</param>
        /// <param name="target">Target contract address</param>
        /// <param name="txData">Transaction data</param>
        /// <param name="signature">Transaction signature</param>
        /// <param name="nonce">Transaction nonce</param>
        /// <param name="deadline">Transaction deadline (timestamp)</param>
        /// <param name="txType">Transaction type (neo or ethereum)</param>
        /// <returns>Transaction execution result</returns>
        public static object ExecuteMetaTx(UInt160 sender, UInt160 target, byte[] txData, byte[] signature, BigInteger nonce, BigInteger deadline, string txType)
        {
            // Check if the caller is an authorized relayer
            if (!IsRelayer(Runtime.CallingScriptHash))
                throw new Exception("Unauthorized relayer");
            
            // Check if the transaction has expired
            if (Runtime.Time > deadline)
            {
                OnMetaTxRejected(sender, "expired", txType);
                throw new Exception("Transaction expired");
            }
            
            // Check if the transaction has already been executed
            StorageMap executedTxs = new StorageMap(Storage.CurrentContext, ExecutedTxPrefix);
            string txKey = $"{sender}:{nonce}";
            
            if (executedTxs.Get(txKey) != null)
            {
                OnMetaTxRejected(sender, "already_executed", txType);
                throw new Exception("Transaction already executed");
            }
            
            // Verify the signature
            byte[] message = CreateMessage(sender, target, txData, nonce, deadline);
            bool isValidSignature = VerifySignature(message, signature, sender, txType);
            
            if (!isValidSignature)
            {
                OnMetaTxRejected(sender, "invalid_signature", txType);
                throw new Exception("Invalid signature");
            }
            
            // Mark the transaction as executed
            executedTxs.Put(txKey, 1);
            
            // Execute the transaction
            object result;
            
            if (txType == NeoTxType)
            {
                // Execute Neo N3 transaction
                result = Contract.Call(target, "executeTransaction", CallFlags.All, new object[] { sender, txData });
            }
            else if (txType == EthereumTxType)
            {
                // Get Gas Bank account for the target contract
                UInt160 gasBankAccount = GetGasBankAccount(target);
                
                // Execute Ethereum transaction with Gas Bank account
                result = Contract.Call(target, "executeEthereumTransaction", CallFlags.All, new object[] { sender, txData, gasBankAccount });
            }
            else
            {
                OnMetaTxRejected(sender, "invalid_tx_type", txType);
                throw new Exception("Invalid transaction type");
            }
            
            // Emit event
            UInt256 txHash = Runtime.GetScriptContainer().Hash;
            OnMetaTxExecuted(sender, txHash, txType, StdLib.Base64Encode(txData));
            
            return result;
        }
        
        /// <summary>
        /// Create a message to be signed
        /// </summary>
        /// <param name="sender">Original sender address</param>
        /// <param name="target">Target contract address</param>
        /// <param name="txData">Transaction data</param>
        /// <param name="nonce">Transaction nonce</param>
        /// <param name="deadline">Transaction deadline (timestamp)</param>
        /// <returns>Message bytes</returns>
        private static byte[] CreateMessage(UInt160 sender, UInt160 target, byte[] txData, BigInteger nonce, BigInteger deadline)
        {
            // Create a message that includes all transaction details
            // This message will be signed by the sender and verified by the contract
            byte[] message = Helper.Concat(
                sender.ToArray(),
                target.ToArray(),
                txData,
                nonce.ToByteArray(),
                deadline.ToByteArray()
            );
            
            return CryptoLib.Sha256(message);
        }
        
        /// <summary>
        /// Verify the signature of a transaction
        /// </summary>
        /// <param name="message">Message that was signed</param>
        /// <param name="signature">Signature</param>
        /// <param name="sender">Sender address</param>
        /// <param name="txType">Transaction type (neo or ethereum)</param>
        /// <returns>True if the signature is valid, false otherwise</returns>
        private static bool VerifySignature(byte[] message, byte[] signature, UInt160 sender, string txType)
        {
            if (txType == NeoTxType)
            {
                // Verify Neo N3 signature
                return CryptoLib.VerifyWithECDsa(message, signature, sender, CryptoLib.NamedCurve.secp256r1);
            }
            else if (txType == EthereumTxType)
            {
                // Verify Ethereum signature (secp256k1)
                // Note: This is a simplified implementation
                // In a real implementation, this would use the Ethereum signature verification algorithm
                return CryptoLib.VerifyWithECDsa(message, signature, sender, CryptoLib.NamedCurve.secp256k1);
            }
            
            return false;
        }
        
        #endregion
        
        #region Oracle Price Feed Functions
        
        /// <summary>
        /// Update price data for a specific asset
        /// </summary>
        /// <param name="priceIndex">Price index (0 for NEO/USD, 1 for GAS/USD, etc.)</param>
        /// <param name="price">Price value (multiplied by 10^8 to handle decimals)</param>
        /// <returns>True if the price was updated successfully</returns>
        public static bool UpdatePriceData(byte priceIndex, BigInteger price)
        {
            // Only the oracle contract or the contract owner can update price data
            if (Runtime.CallingScriptHash != OracleContractHash && !Runtime.CheckWitness(Owner))
                throw new Exception("Not authorized to update price data");
            
            // Store the price data
            StorageMap priceData = new StorageMap(Storage.CurrentContext, PriceDataPrefix);
            priceData.Put(priceIndex, price);
            
            // Update the last update time
            StorageMap lastUpdateTime = new StorageMap(Storage.CurrentContext, LastUpdateTimePrefix);
            lastUpdateTime.Put(priceIndex, Runtime.Time);
            
            // Get the price pair name for the event
            string pricePair = GetPricePairByIndex(priceIndex);
            
            // Emit event
            UInt256 txHash = Runtime.GetScriptContainer().Hash;
            OnPriceDataUpdated(priceIndex, pricePair, price, txHash);
            
            return true;
        }
        
        /// <summary>
        /// Get price data for a specific asset by index
        /// </summary>
        /// <param name="priceIndex">Price index (0 for NEO/USD, 1 for GAS/USD, etc.)</param>
        /// <returns>Price value (multiplied by 10^8 to handle decimals)</returns>
        public static BigInteger GetPriceByIndex(byte priceIndex)
        {
            StorageMap priceData = new StorageMap(Storage.CurrentContext, PriceDataPrefix);
            ByteString price = priceData.Get(priceIndex);
            
            if (price == null)
                throw new Exception("Price data not found for the specified index");
            
            return (BigInteger)price;
        }
        
        /// <summary>
        /// Get price data for a specific asset by pair name
        /// </summary>
        /// <param name="pricePair">Price pair name (e.g., "NEO/USD", "GAS/USD")</param>
        /// <returns>Price value (multiplied by 10^8 to handle decimals)</returns>
        public static BigInteger GetPriceByPair(string pricePair)
        {
            // Get the price index for the pair
            StorageMap priceIndexMap = new StorageMap(Storage.CurrentContext, PriceIndexMapPrefix);
            ByteString priceIndexBytes = priceIndexMap.Get(pricePair);
            
            if (priceIndexBytes == null)
                throw new Exception("Price pair not found");
            
            byte priceIndex = (byte)(BigInteger)priceIndexBytes;
            
            // Get the price data for the index
            return GetPriceByIndex(priceIndex);
        }
        
        /// <summary>
        /// Get the last update time for a specific price index
        /// </summary>
        /// <param name="priceIndex">Price index (0 for NEO/USD, 1 for GAS/USD, etc.)</param>
        /// <returns>Last update time (timestamp)</returns>
        public static BigInteger GetLastUpdateTime(byte priceIndex)
        {
            StorageMap lastUpdateTime = new StorageMap(Storage.CurrentContext, LastUpdateTimePrefix);
            ByteString time = lastUpdateTime.Get(priceIndex);
            
            if (time == null)
                return 0;
            
            return (BigInteger)time;
        }
        
        /// <summary>
        /// Add a new price pair to the index map
        /// </summary>
        /// <param name="pricePair">Price pair name (e.g., "NEO/USD", "GAS/USD")</param>
        /// <param name="priceIndex">Price index to assign</param>
        /// <returns>True if the price pair was added successfully</returns>
        public static bool AddPricePair(string pricePair, byte priceIndex)
        {
            // Only contract owner can add price pairs
            if (!Runtime.CheckWitness(Owner))
                throw new Exception("Not authorized");
            
            // Check if the price pair already exists
            StorageMap priceIndexMap = new StorageMap(Storage.CurrentContext, PriceIndexMapPrefix);
            if (priceIndexMap.Get(pricePair) != null)
                throw new Exception("Price pair already exists");
            
            // Add the price pair to the index map
            priceIndexMap.Put(pricePair, priceIndex);
            
            return true;
        }
        
        /// <summary>
        /// Get the price pair name for a specific index
        /// </summary>
        /// <param name="priceIndex">Price index</param>
        /// <returns>Price pair name</returns>
        private static string GetPricePairByIndex(byte priceIndex)
        {
            if (priceIndex == NEO_USD_INDEX)
                return "NEO/USD";
            else if (priceIndex == GAS_USD_INDEX)
                return "GAS/USD";
            else if (priceIndex == BTC_USD_INDEX)
                return "BTC/USD";
            else if (priceIndex == ETH_USD_INDEX)
                return "ETH/USD";
            
            // For custom indices, we need to search the map
            StorageMap priceIndexMap = new StorageMap(Storage.CurrentContext, PriceIndexMapPrefix);
            
            // This is inefficient but necessary for reverse lookup
            // In a production environment, consider maintaining a bidirectional map
            Iterator<string> pairs = priceIndexMap.Find();
            
            while (pairs.Next())
            {
                if ((byte)(BigInteger)priceIndexMap.Get(pairs.Value) == priceIndex)
                    return pairs.Value;
            }
            
            return "Unknown";
        }
        
        /// <summary>
        /// Set the Oracle contract hash
        /// </summary>
        /// <param name="oracleContractHash">Oracle contract hash</param>
        /// <returns>True if the Oracle contract hash was set successfully</returns>
        public static bool SetOracleContractHash(UInt160 oracleContractHash)
        {
            // Only contract owner can set the Oracle contract hash
            if (!Runtime.CheckWitness(Owner))
                throw new Exception("Not authorized");
            
            // Store the Oracle contract hash
            Storage.Put(Storage.CurrentContext, "OracleContractHash", oracleContractHash);
            
            return true;
        }
        
        /// <summary>
        /// Get the Oracle contract hash
        /// </summary>
        /// <returns>Oracle contract hash</returns>
        public static UInt160 GetOracleContractHash()
        {
            ByteString oracleContractHash = Storage.Get(Storage.CurrentContext, "OracleContractHash");
            
            if (oracleContractHash == null)
                return OracleContractHash;
            
            return (UInt160)oracleContractHash;
        }
        
        /// <summary>
        /// Request price data update from the Oracle
        /// </summary>
        /// <param name="pricePair">Price pair name (e.g., "NEO/USD", "GAS/USD")</param>
        /// <returns>Request ID</returns>
        public static string RequestPriceUpdate(string pricePair)
        {
            // Only contract owner can request price updates
            if (!Runtime.CheckWitness(Owner))
                throw new Exception("Not authorized");
            
            // Get the Oracle contract hash
            UInt160 oracleContract = GetOracleContractHash();
            
            // Create the request URL
            string url = $"https://api.coingecko.com/api/v3/simple/price?ids={GetCoinId(pricePair)}&vs_currencies=usd";
            
            // Request data from the Oracle
            object[] args = new object[] { url, "get", string.Empty };
            string requestId = (string)Contract.Call(oracleContract, "request", CallFlags.All, args);
            
            return requestId;
        }
        
        /// <summary>
        /// Oracle callback method for price data updates
        /// </summary>
        /// <param name="requestId">Request ID</param>
        /// <param name="data">Oracle response data</param>
        public static void OraclePriceCallback(string requestId, string data)
        {
            // Check if the caller is the Oracle contract
            UInt160 oracleContract = GetOracleContractHash();
            if (Runtime.CallingScriptHash != oracleContract)
                throw new Exception("Unauthorized oracle callback");
            
            // Parse the response data
            // Example response: {"neo":{"usd":10.5}}
            Map<string, Map<string, object>> responseData = (Map<string, Map<string, object>>)StdLib.JsonDeserialize(data);
            
            // Process each coin in the response
            foreach (var coinEntry in responseData)
            {
                string coinId = coinEntry.Key;
                Map<string, object> priceData = coinEntry.Value;
                
                if (priceData.ContainsKey("usd"))
                {
                    // Get the price value
                    double priceValue = (double)priceData["usd"];
                    
                    // Convert to BigInteger (multiply by 10^8 to handle decimals)
                    BigInteger price = (BigInteger)(priceValue * 100000000);
                    
                    // Get the price pair and index
                    string pricePair = GetPricePairFromCoinId(coinId);
                    byte priceIndex = GetPriceIndexFromPair(pricePair);
                    
                    // Update the price data
                    UpdatePriceData(priceIndex, price);
                }
            }
        }
        
        /// <summary>
        /// Get the coin ID for a price pair
        /// </summary>
        /// <param name="pricePair">Price pair name (e.g., "NEO/USD", "GAS/USD")</param>
        /// <returns>Coin ID for the API request</returns>
        private static string GetCoinId(string pricePair)
        {
            // Extract the base currency from the price pair
            string baseCurrency = pricePair.Split('/')[0].ToLower();
            
            // Map to CoinGecko IDs
            if (baseCurrency == "neo")
                return "neo";
            else if (baseCurrency == "gas")
                return "gas";
            else if (baseCurrency == "btc")
                return "bitcoin";
            else if (baseCurrency == "eth")
                return "ethereum";
            
            // Default to the base currency as the ID
            return baseCurrency;
        }
        
        /// <summary>
        /// Get the price pair from a coin ID
        /// </summary>
        /// <param name="coinId">Coin ID</param>
        /// <returns>Price pair name</returns>
        private static string GetPricePairFromCoinId(string coinId)
        {
            if (coinId == "neo")
                return "NEO/USD";
            else if (coinId == "gas")
                return "GAS/USD";
            else if (coinId == "bitcoin")
                return "BTC/USD";
            else if (coinId == "ethereum")
                return "ETH/USD";
            
            // Default to uppercase + /USD
            return $"{coinId.ToUpper()}/USD";
        }
        
        /// <summary>
        /// Get the price index for a price pair
        /// </summary>
        /// <param name="pricePair">Price pair name</param>
        /// <returns>Price index</returns>
        private static byte GetPriceIndexFromPair(string pricePair)
        {
            StorageMap priceIndexMap = new StorageMap(Storage.CurrentContext, PriceIndexMapPrefix);
            ByteString priceIndexBytes = priceIndexMap.Get(pricePair);
            
            if (priceIndexBytes == null)
            {
                // Use default indices for known pairs
                if (pricePair == "NEO/USD")
                    return NEO_USD_INDEX;
                else if (pricePair == "GAS/USD")
                    return GAS_USD_INDEX;
                else if (pricePair == "BTC/USD")
                    return BTC_USD_INDEX;
                else if (pricePair == "ETH/USD")
                    return ETH_USD_INDEX;
                
                throw new Exception("Price pair not found");
            }
            
            return (byte)(BigInteger)priceIndexBytes;
        }
        
        #endregion
    }
}
