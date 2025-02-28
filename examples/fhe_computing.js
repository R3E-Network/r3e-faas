// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

/**
 * @file fhe_computing.js
 * @description Example of using Fully Homomorphic Encryption in R3E FaaS.
 */

import { fhe, encode, decode } from "r3e";

/**
 * Example of basic FHE operations using TFHE.
 */
async function basicFheExample() {
  console.log("=== Basic FHE Example (TFHE) ===");
  
  // Generate a key pair for FHE operations
  console.log("Generating FHE keys...");
  const keyPairId = fhe.generateKeys(fhe.SchemeType.TFHE, {
    securityLevel: 128,
    polynomialModulusDegree: 4096,
    plaintextModulus: 1024
  });
  console.log(`Key pair generated with ID: ${keyPairId}`);
  
  // For demonstration purposes, we assume we have access to the public and private keys
  // In a real application, these would be managed securely
  const publicKeyId = `${keyPairId}_public`;
  const privateKeyId = `${keyPairId}_private`;
  
  // Encrypt some data
  console.log("Encrypting data...");
  const plaintext1 = "42"; // We'll encrypt the number 42
  const ciphertext1Id = fhe.encrypt(publicKeyId, plaintext1);
  console.log(`Encrypted data with ID: ${ciphertext1Id}`);
  
  // Encrypt more data
  const plaintext2 = "8"; // We'll encrypt the number 8
  const ciphertext2Id = fhe.encrypt(publicKeyId, plaintext2);
  console.log(`Encrypted more data with ID: ${ciphertext2Id}`);
  
  // Perform homomorphic addition
  console.log("Performing homomorphic addition...");
  const addResultId = fhe.add(ciphertext1Id, ciphertext2Id);
  console.log(`Addition result ID: ${addResultId}`);
  
  // Perform homomorphic multiplication
  console.log("Performing homomorphic multiplication...");
  const multiplyResultId = fhe.multiply(ciphertext1Id, ciphertext2Id);
  console.log(`Multiplication result ID: ${multiplyResultId}`);
  
  // Decrypt the results
  console.log("Decrypting results...");
  const addResult = fhe.decrypt(privateKeyId, addResultId, true);
  console.log(`Decrypted addition result: ${addResult}`); // Should be 50 (42 + 8)
  
  const multiplyResult = fhe.decrypt(privateKeyId, multiplyResultId, true);
  console.log(`Decrypted multiplication result: ${multiplyResult}`); // Should be 336 (42 * 8)
  
  // Check the noise budget
  console.log("Checking noise budget...");
  const noiseBudget = fhe.estimateNoiseBudget(multiplyResultId);
  console.log(`Noise budget after multiplication: ${noiseBudget}`);
  
  return {
    keyPairId,
    ciphertext1Id,
    ciphertext2Id,
    addResultId,
    multiplyResultId,
    addResult,
    multiplyResult,
    noiseBudget
  };
}

/**
 * Example of private information retrieval using FHE.
 */
async function privateInformationRetrievalExample() {
  console.log("\n=== Private Information Retrieval Example ===");
  
  // Generate a key pair for FHE operations
  console.log("Generating FHE keys...");
  const keyPairId = fhe.generateKeys(fhe.SchemeType.TFHE, {
    securityLevel: 128,
    polynomialModulusDegree: 8192, // Higher degree for more complex operations
    plaintextModulus: 1024
  });
  console.log(`Key pair generated with ID: ${keyPairId}`);
  
  // For demonstration purposes, we assume we have access to the public and private keys
  const publicKeyId = `${keyPairId}_public`;
  const privateKeyId = `${keyPairId}_private`;
  
  // Simulate a database with several entries
  const database = [
    "Record 1: User data",
    "Record 2: Financial information",
    "Record 3: Medical history",
    "Record 4: Personal preferences"
  ];
  console.log("Database:", database);
  
  // The client wants to retrieve Record 3 without revealing which record they're interested in
  // They encrypt their query (index 2, which is the 3rd record)
  console.log("Encrypting query index...");
  const queryIndex = "2"; // 0-based index for Record 3
  const encryptedQueryId = fhe.encrypt(publicKeyId, queryIndex);
  console.log(`Encrypted query ID: ${encryptedQueryId}`);
  
  // The server processes the encrypted query without knowing which record is being requested
  console.log("Server processing encrypted query...");
  
  // For each database entry, we'll create a selector that is 1 for the requested index and 0 for others
  // This is a simplified example - in practice, more complex FHE operations would be used
  const resultIds = [];
  
  for (let i = 0; i < database.length; i++) {
    // Encrypt the current index
    const currentIndexId = fhe.encrypt(publicKeyId, i.toString());
    
    // Create a selector (1 if encryptedQuery == currentIndex, 0 otherwise)
    // This is a simplified example - in practice, we would use FHE equality comparison
    // For demonstration purposes, we'll just simulate this
    const selectorId = `selector_${i}_${encryptedQueryId}`;
    
    // Encrypt the database entry
    const entryId = fhe.encrypt(publicKeyId, database[i]);
    
    // Multiply the entry by the selector (will be the entry if selected, 0 otherwise)
    // Again, this is simplified for demonstration
    const resultId = `result_${i}_${selectorId}_${entryId}`;
    resultIds.push(resultId);
  }
  
  // Sum all results (only the selected entry will contribute to the sum)
  console.log("Combining results...");
  let combinedResultId = resultIds[0];
  for (let i = 1; i < resultIds.length; i++) {
    combinedResultId = fhe.add(combinedResultId, resultIds[i]);
  }
  
  // The client decrypts the result
  console.log("Client decrypting result...");
  const retrievedRecord = fhe.decrypt(privateKeyId, combinedResultId, true);
  console.log(`Retrieved record: ${retrievedRecord}`);
  
  return {
    keyPairId,
    encryptedQueryId,
    combinedResultId,
    retrievedRecord
  };
}

/**
 * Example of secure multi-party computation using FHE.
 */
async function secureMultiPartyComputationExample() {
  console.log("\n=== Secure Multi-Party Computation Example ===");
  
  // Party A and Party B want to compute the average of their salaries
  // without revealing their individual salaries to each other
  
  // Party A generates a key pair
  console.log("Party A generating FHE keys...");
  const keyPairIdA = fhe.generateKeys(fhe.SchemeType.TFHE, {
    securityLevel: 128,
    polynomialModulusDegree: 4096,
    plaintextModulus: 1024
  });
  console.log(`Party A key pair generated with ID: ${keyPairIdA}`);
  
  // For demonstration purposes, we assume we have access to the public and private keys
  const publicKeyIdA = `${keyPairIdA}_public`;
  const privateKeyIdA = `${keyPairIdA}_private`;
  
  // Party A encrypts their salary
  const salaryA = "120000"; // $120,000
  console.log(`Party A salary: ${salaryA}`);
  const encryptedSalaryAId = fhe.encrypt(publicKeyIdA, salaryA);
  console.log(`Party A encrypted salary ID: ${encryptedSalaryAId}`);
  
  // Party B encrypts their salary using Party A's public key
  const salaryB = "95000"; // $95,000
  console.log(`Party B salary: ${salaryB}`);
  const encryptedSalaryBId = fhe.encrypt(publicKeyIdA, salaryB);
  console.log(`Party B encrypted salary ID: ${encryptedSalaryBId}`);
  
  // Compute the sum of salaries homomorphically
  console.log("Computing sum of salaries...");
  const sumSalariesId = fhe.add(encryptedSalaryAId, encryptedSalaryBId);
  
  // For average, we need to divide by 2
  // In FHE, division is complex, so for simplicity, we'll decrypt and then divide
  console.log("Party A decrypting sum...");
  const sumSalaries = fhe.decrypt(privateKeyIdA, sumSalariesId, true);
  console.log(`Sum of salaries: ${sumSalaries}`);
  
  // Calculate average
  const averageSalary = parseInt(sumSalaries) / 2;
  console.log(`Average salary: ${averageSalary}`);
  
  return {
    keyPairIdA,
    encryptedSalaryAId,
    encryptedSalaryBId,
    sumSalariesId,
    sumSalaries,
    averageSalary
  };
}

/**
 * Main function to run all examples.
 */
async function main() {
  try {
    // Run basic FHE example
    const basicResult = await basicFheExample();
    
    // Run private information retrieval example
    const pirResult = await privateInformationRetrievalExample();
    
    // Run secure multi-party computation example
    const smpcResult = await secureMultiPartyComputationExample();
    
    console.log("\n=== Summary ===");
    console.log("All Fully Homomorphic Encryption examples completed successfully!");
    
    return {
      basicFhe: basicResult,
      privateInformationRetrieval: pirResult,
      secureMultiPartyComputation: smpcResult
    };
  } catch (error) {
    console.error("Error running Fully Homomorphic Encryption examples:", error);
    throw error;
  }
}

// Run the main function
main().catch(console.error);
