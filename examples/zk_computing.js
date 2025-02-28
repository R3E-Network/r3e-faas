// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

/**
 * @file zk_computing.js
 * @description Example of using Zero-Knowledge computing in R3E FaaS.
 */

import { zk } from "r3e";

/**
 * Example of using ZoKrates for Zero-Knowledge proofs.
 */
async function zokratesExample() {
  console.log("=== ZoKrates Example ===");
  
  // Define a simple ZoKrates circuit for proving knowledge of factors
  // This circuit proves that the prover knows two numbers a and b such that a * b = c
  // without revealing a and b
  const circuitSource = `
    def main(private field a, private field b, field c) -> bool:
      return a * b == c
  `;
  
  // Compile the circuit
  console.log("Compiling ZoKrates circuit...");
  const circuitId = zk.compileCircuit(circuitSource, zk.CircuitType.ZOKRATES, "multiply");
  console.log(`Circuit compiled with ID: ${circuitId}`);
  
  // Generate proving and verification keys
  console.log("Generating keys...");
  const { provingKeyId, verificationKeyId } = zk.generateKeys(circuitId);
  console.log(`Proving key ID: ${provingKeyId}`);
  console.log(`Verification key ID: ${verificationKeyId}`);
  
  // Generate a proof
  // We want to prove that we know factors of 6 (which are 2 and 3)
  // without revealing those factors
  console.log("Generating proof...");
  const publicInputs = ["6"]; // The public value c = 6
  const privateInputs = ["2", "3"]; // The private values a = 2, b = 3
  const proofId = zk.generateProof(circuitId, provingKeyId, publicInputs, privateInputs);
  console.log(`Proof generated with ID: ${proofId}`);
  
  // Verify the proof
  console.log("Verifying proof...");
  const isValid = zk.verifyProof(proofId, verificationKeyId, publicInputs);
  console.log(`Proof is valid: ${isValid}`);
  
  return { circuitId, provingKeyId, verificationKeyId, proofId, isValid };
}

/**
 * Example of using Bulletproofs for Zero-Knowledge range proofs.
 */
async function bulletproofsExample() {
  console.log("\n=== Bulletproofs Example ===");
  
  // Define a simple Bulletproofs circuit for range proofs
  // This circuit proves that a value is within a certain range
  // without revealing the value
  const circuitSource = `
    range_proof(value, min, max)
  `;
  
  // Compile the circuit
  console.log("Compiling Bulletproofs circuit...");
  const circuitId = zk.compileCircuit(circuitSource, zk.CircuitType.BULLETPROOFS, "range_proof");
  console.log(`Circuit compiled with ID: ${circuitId}`);
  
  // Generate proving and verification keys
  console.log("Generating keys...");
  const { provingKeyId, verificationKeyId } = zk.generateKeys(circuitId);
  console.log(`Proving key ID: ${provingKeyId}`);
  console.log(`Verification key ID: ${verificationKeyId}`);
  
  // Generate a proof
  // We want to prove that a value is between 18 and 100
  // without revealing the actual value (25)
  console.log("Generating proof...");
  const publicInputs = ["18", "100"]; // The public range: min = 18, max = 100
  const privateInputs = ["25"]; // The private value: value = 25
  const proofId = zk.generateProof(circuitId, provingKeyId, publicInputs, privateInputs);
  console.log(`Proof generated with ID: ${proofId}`);
  
  // Verify the proof
  console.log("Verifying proof...");
  const isValid = zk.verifyProof(proofId, verificationKeyId, publicInputs);
  console.log(`Proof is valid: ${isValid}`);
  
  return { circuitId, provingKeyId, verificationKeyId, proofId, isValid };
}

/**
 * Example of using Circom for Zero-Knowledge circuit proofs.
 */
async function circomExample() {
  console.log("\n=== Circom Example ===");
  
  // Define a simple Circom circuit for a Merkle tree proof
  // This circuit proves membership in a Merkle tree
  // without revealing the path
  const circuitSource = `
    pragma circom 2.0.0;
    
    template MerkleTreeCheck(levels) {
      signal input leaf;
      signal input root;
      signal private input path_elements[levels];
      signal private input path_indices[levels];
      
      // Circuit implementation would go here
      // This is a simplified example
      
      signal output out;
      out <== 1;
    }
    
    component main = MerkleTreeCheck(10);
  `;
  
  // Compile the circuit
  console.log("Compiling Circom circuit...");
  const circuitId = zk.compileCircuit(circuitSource, zk.CircuitType.CIRCOM, "merkle_proof");
  console.log(`Circuit compiled with ID: ${circuitId}`);
  
  // Generate proving and verification keys
  console.log("Generating keys...");
  const { provingKeyId, verificationKeyId } = zk.generateKeys(circuitId);
  console.log(`Proving key ID: ${provingKeyId}`);
  console.log(`Verification key ID: ${verificationKeyId}`);
  
  // Generate a proof
  // We want to prove membership in a Merkle tree
  console.log("Generating proof...");
  const publicInputs = [
    "123456789", // leaf
    "987654321"  // root
  ];
  const privateInputs = [
    "[111111, 222222, 333333, 444444, 555555, 666666, 777777, 888888, 999999, 000000]", // path_elements
    "[0, 1, 0, 1, 0, 1, 0, 1, 0, 1]" // path_indices
  ];
  const proofId = zk.generateProof(circuitId, provingKeyId, publicInputs, privateInputs);
  console.log(`Proof generated with ID: ${proofId}`);
  
  // Verify the proof
  console.log("Verifying proof...");
  const isValid = zk.verifyProof(proofId, verificationKeyId, publicInputs);
  console.log(`Proof is valid: ${isValid}`);
  
  return { circuitId, provingKeyId, verificationKeyId, proofId, isValid };
}

/**
 * Main function to run all examples.
 */
async function main() {
  try {
    // Run ZoKrates example
    const zokratesResult = await zokratesExample();
    
    // Run Bulletproofs example
    const bulletproofsResult = await bulletproofsExample();
    
    // Run Circom example
    const circomResult = await circomExample();
    
    console.log("\n=== Summary ===");
    console.log("All Zero-Knowledge computing examples completed successfully!");
    
    return {
      zokrates: zokratesResult,
      bulletproofs: bulletproofsResult,
      circom: circomResult
    };
  } catch (error) {
    console.error("Error running Zero-Knowledge computing examples:", error);
    throw error;
  }
}

// Run the main function
main().catch(console.error);
