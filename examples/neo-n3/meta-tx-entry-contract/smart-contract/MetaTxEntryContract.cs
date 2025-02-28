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
    [DisplayName("MetaTxEntryContract")]
    [ManifestExtra("Author", "R3E Network")]
    [ManifestExtra("Email", "dev@r3e.network")]
    [ManifestExtra("Description", "Entry contract for meta transactions in the R3E FaaS platform")]
    public class MetaTxEntryContract : Framework.SmartContract
    {
        // Events
        [DisplayName("MetaTxExecuted")]
        public static event Action<UInt160, UInt256, string, string> OnMetaTxExecuted;
        
        [DisplayName("MetaTxRejected")]
        public static event Action<UInt160, string, string> OnMetaTxRejected;
        
        // Storage keys
        private static readonly byte[] RelayerPrefix = new byte[] { 0x01 };
        private static readonly byte[] GasBankPrefix = new byte[] { 0x02 };
        private static readonly byte[] ExecutedTxPrefix = new byte[] { 0x03 };
        private static readonly byte[] ContractOwnerKey = new byte[] { 0x04 };
        
        // Transaction types
        private static readonly string NeoTxType = "neo";
        private static readonly string EthereumTxType = "ethereum";
        
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
        }
        
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
    }
}
