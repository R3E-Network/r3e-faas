// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

/**
 * @file circom_example.js
 * @description Example usage of the Circom zkSNARK provider in the R3E FaaS platform.
 */

import { zk } from "r3e";

/**
 * This example demonstrates how to use the Circom zkSNARK provider to create and verify
 * a zero-knowledge proof for a Merkle tree inclusion proof.
 */
async function main() {
  console.log("=== Circom zkSNARK Example ===");

  // Define a circuit for a Merkle tree inclusion proof
  // This circuit proves that a leaf is included in a Merkle tree without revealing the path
  const circuitSource = `
    pragma circom 2.0.0;
    
    include "mimcsponge.circom";
    
    // MiMC hash function
    template MiMCSponge(nInputs) {
        signal input ins[nInputs];
        signal output out;
        
        component mimc = MiMCSponge(nInputs, 220, 1);
        for (var i = 0; i < nInputs; i++) {
            mimc.ins[i] <== ins[i];
        }
        mimc.k <== 0;
        
        out <== mimc.outs[0];
    }
    
    // Computes the hash of two inputs
    template HashLeftRight() {
        signal input left;
        signal input right;
        signal output hash;
        
        component hasher = MiMCSponge(2);
        hasher.ins[0] <== left;
        hasher.ins[1] <== right;
        
        hash <== hasher.out;
    }
    
    // Merkle tree inclusion proof for a tree of depth 4
    template MerkleTreeInclusionProof(depth) {
        signal input leaf;
        signal input pathElements[depth];
        signal input pathIndices[depth];
        signal output root;
        
        component hashers[depth];
        component selectors[depth];
        
        signal levelHashes[depth + 1];
        levelHashes[0] <== leaf;
        
        for (var i = 0; i < depth; i++) {
            // We need to select if the current element is on the left or right
            hashers[i] = HashLeftRight();
            
            // pathIndices[i] == 0 means the current element is on the left
            // pathIndices[i] == 1 means the current element is on the right
            selectors[i] = Selector();
            selectors[i].in[0] <== levelHashes[i];
            selectors[i].in[1] <== pathElements[i];
            selectors[i].s <== pathIndices[i];
            
            hashers[i].left <== selectors[i].out[0];
            hashers[i].right <== selectors[i].out[1];
            
            levelHashes[i + 1] <== hashers[i].hash;
        }
        
        root <== levelHashes[depth];
    }
    
    // Selector component
    template Selector() {
        signal input in[2];
        signal input s;
        signal output out[2];
        
        s * (1 - s) === 0;
        
        out[0] <== (1 - s) * in[0] + s * in[1];
        out[1] <== s * in[0] + (1 - s) * in[1];
    }
    
    // Main component
    component main {public [root]} = MerkleTreeInclusionProof(4);
  `;

  try {
    // Step 1: Compile the circuit
    console.log("Compiling circuit...");
    const circuitId = await zk.compileCircuit(
      circuitSource,
      zk.CircuitType.CIRCOM,
      "merkle_inclusion_proof"
    );
    console.log(`Circuit compiled successfully. Circuit ID: ${circuitId}`);

    // Step 2: Generate proving and verification keys
    console.log("Generating keys...");
    const { provingKeyId, verificationKeyId } = await zk.generateKeys(circuitId);
    console.log(`Keys generated successfully.`);
    console.log(`Proving Key ID: ${provingKeyId}`);
    console.log(`Verification Key ID: ${verificationKeyId}`);

    // Step 3: Generate a proof
    // We want to prove that a leaf is included in a Merkle tree with a specific root
    // without revealing the path from the leaf to the root
    console.log("Generating proof...");
    
    // For demonstration purposes, we'll use placeholder values
    // In a real application, these would be actual Merkle tree values
    const root = "12345678901234567890123456789012";
    const leaf = "98765432109876543210987654321098";
    const pathElements = [
      "11111111111111111111111111111111",
      "22222222222222222222222222222222",
      "33333333333333333333333333333333",
      "44444444444444444444444444444444"
    ];
    const pathIndices = [0, 1, 0, 1]; // Left, Right, Left, Right
    
    const publicInputs = [root]; // The public root of the Merkle tree
    const privateInputs = [
      leaf, // The leaf we're proving inclusion for
      JSON.stringify(pathElements), // The path elements
      JSON.stringify(pathIndices) // The path indices
    ];
    
    const proofId = await zk.generateProof(
      circuitId,
      provingKeyId,
      publicInputs,
      privateInputs
    );
    console.log(`Proof generated successfully. Proof ID: ${proofId}`);

    // Step 4: Verify the proof
    console.log("Verifying proof...");
    const isValid = await zk.verifyProof(
      proofId,
      verificationKeyId,
      publicInputs
    );
    console.log(`Proof verification result: ${isValid ? "Valid" : "Invalid"}`);

    // Try with incorrect root
    console.log("Verifying proof with incorrect root...");
    const isInvalidProof = await zk.verifyProof(
      proofId,
      verificationKeyId,
      ["99999999999999999999999999999999"] // Incorrect root
    );
    console.log(`Proof verification result with incorrect root: ${isInvalidProof ? "Valid" : "Invalid"}`);

  } catch (error) {
    console.error("Error:", error);
  }
}

// Run the example
main().catch(console.error);
