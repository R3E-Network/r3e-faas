// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

/**
 * @file bellman_example.js
 * @description Example usage of the Bellman zkSNARK provider in the R3E FaaS platform.
 */

import { zk } from "r3e";

/**
 * This example demonstrates how to use the Bellman zkSNARK provider to create and verify
 * a zero-knowledge proof for a simple arithmetic circuit.
 */
async function main() {
  console.log("=== Bellman zkSNARK Example ===");

  // Define a circuit that proves knowledge of two numbers that multiply to a target value
  // Note: This is a simplified representation of a circuit for demonstration purposes
  const circuitSource = `
    // Circuit to prove knowledge of factors of a number
    // Input: private values a and b, public value c
    // Constraint: a * b = c
    
    struct FactorCircuit {
        a: Scalar,
        b: Scalar,
    }
    
    impl Circuit for FactorCircuit {
        fn synthesize<CS: ConstraintSystem>(self, cs: &mut CS) -> Result<(), SynthesisError> {
            // Allocate private inputs
            let a = cs.alloc(|| "a", || Ok(self.a))?;
            let b = cs.alloc(|| "b", || Ok(self.b))?;
            
            // Allocate public output
            let c = cs.alloc_input(|| "c", || Ok(self.a * self.b))?;
            
            // Enforce constraint: a * b = c
            cs.enforce(
                || "multiplication constraint",
                |lc| lc + a,
                |lc| lc + b,
                |lc| lc + c,
            );
            
            Ok(())
        }
    }
  `;

  try {
    // Step 1: Compile the circuit
    console.log("Compiling circuit...");
    const circuitId = await zk.compileCircuit(
      circuitSource,
      zk.CircuitType.BELLMAN,
      "factor_circuit"
    );
    console.log(`Circuit compiled successfully. Circuit ID: ${circuitId}`);

    // Step 2: Generate proving and verification keys
    console.log("Generating keys...");
    const { provingKeyId, verificationKeyId } = await zk.generateKeys(circuitId);
    console.log(`Keys generated successfully.`);
    console.log(`Proving Key ID: ${provingKeyId}`);
    console.log(`Verification Key ID: ${verificationKeyId}`);

    // Step 3: Generate a proof
    // We want to prove that we know factors 7 and 13 of the number 91
    // without revealing what those factors are
    console.log("Generating proof...");
    const publicInputs = ["91"]; // The public value c = 91
    const privateInputs = ["7", "13"]; // The private values a = 7, b = 13
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

    // Try with incorrect public input
    console.log("Verifying proof with incorrect public input...");
    const isInvalidProof = await zk.verifyProof(
      proofId,
      verificationKeyId,
      ["90"] // Incorrect public input
    );
    console.log(`Proof verification result with incorrect input: ${isInvalidProof ? "Valid" : "Invalid"}`);

  } catch (error) {
    console.error("Error:", error);
  }
}

// Run the example
main().catch(console.error);
