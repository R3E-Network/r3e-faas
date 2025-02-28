// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

/**
 * @file arkworks_example.js
 * @description Example usage of the Arkworks zkSNARK provider in the R3E FaaS platform.
 */

import { zk } from "r3e";

/**
 * This example demonstrates how to use the Arkworks zkSNARK provider to create and verify
 * a zero-knowledge proof for a range proof circuit.
 */
async function main() {
  console.log("=== Arkworks zkSNARK Example ===");

  // Define a circuit that proves a number is within a specific range
  // Note: This is a simplified representation of a circuit for demonstration purposes
  const circuitSource = `
    // Circuit to prove a number is within a range [lower, upper]
    // Input: private value x, public values lower and upper
    // Constraint: lower <= x <= upper
    
    struct RangeCircuit<F: PrimeField> {
        x: F,
        lower: F,
        upper: F,
    }
    
    impl<F: PrimeField> ConstraintSynthesizer<F> for RangeCircuit<F> {
        fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
            // Allocate private input
            let x_var = cs.new_witness_variable(|| Ok(self.x))?;
            
            // Allocate public inputs
            let lower_var = cs.new_input_variable(|| Ok(self.lower))?;
            let upper_var = cs.new_input_variable(|| Ok(self.upper))?;
            
            // Enforce x >= lower
            // This is done by introducing a new variable y such that x = lower + y
            let y_var = cs.new_witness_variable(|| Ok(self.x - self.lower))?;
            cs.enforce_constraint(
                lc!() + x_var - lower_var,
                lc!() + CS::one(),
                lc!() + y_var,
            )?;
            
            // Enforce x <= upper
            // This is done by introducing a new variable z such that upper = x + z
            let z_var = cs.new_witness_variable(|| Ok(self.upper - self.x))?;
            cs.enforce_constraint(
                lc!() + upper_var - x_var,
                lc!() + CS::one(),
                lc!() + z_var,
            )?;
            
            Ok(())
        }
    }
  `;

  try {
    // Step 1: Compile the circuit
    console.log("Compiling circuit...");
    const circuitId = await zk.compileCircuit(
      circuitSource,
      zk.CircuitType.ARKWORKS,
      "range_proof"
    );
    console.log(`Circuit compiled successfully. Circuit ID: ${circuitId}`);

    // Step 2: Generate proving and verification keys
    console.log("Generating keys...");
    const { provingKeyId, verificationKeyId } = await zk.generateKeys(circuitId);
    console.log(`Keys generated successfully.`);
    console.log(`Proving Key ID: ${provingKeyId}`);
    console.log(`Verification Key ID: ${verificationKeyId}`);

    // Step 3: Generate a proof
    // We want to prove that a secret number 42 is within the range [10, 100]
    // without revealing what the number is
    console.log("Generating proof...");
    const publicInputs = ["10", "100"]; // The public range [lower, upper]
    const privateInputs = ["42"]; // The private value x = 42
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

    // Try with a different range that doesn't include our secret number
    console.log("Verifying proof with incorrect range...");
    const isInvalidProof = await zk.verifyProof(
      proofId,
      verificationKeyId,
      ["50", "100"] // Range that doesn't include our secret number
    );
    console.log(`Proof verification result with incorrect range: ${isInvalidProof ? "Valid" : "Invalid"}`);

  } catch (error) {
    console.error("Error:", error);
  }
}

// Run the example
main().catch(console.error);
