// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

/**
 * @module zk
 * @description Zero-Knowledge computing module for the R3E FaaS platform.
 */

import { core } from "./infra.js";

/**
 * Compile a Zero-Knowledge circuit.
 * 
 * @param {string} circuitSource - The source code of the circuit.
 * @param {string} circuitType - The type of the circuit (e.g., "zokrates", "bulletproofs", "circom").
 * @param {string} circuitName - The name of the circuit.
 * @param {Object} parameters - Additional parameters for the compilation.
 * @returns {string} The ID of the compiled circuit.
 */
export function compileCircuit(circuitSource, circuitType, circuitName, parameters = {}) {
  return core.ops.op_zk_compile_circuit(circuitSource, circuitType, circuitName, parameters);
}

/**
 * Generate proving and verification keys for a Zero-Knowledge circuit.
 * 
 * @param {string} circuitId - The ID of the circuit.
 * @param {Object} parameters - Additional parameters for key generation.
 * @returns {Object} An object containing the proving key ID and verification key ID.
 */
export function generateKeys(circuitId, parameters = {}) {
  const [provingKeyId, verificationKeyId] = core.ops.op_zk_generate_keys(circuitId, parameters);
  return { provingKeyId, verificationKeyId };
}

/**
 * Generate a Zero-Knowledge proof.
 * 
 * @param {string} circuitId - The ID of the circuit.
 * @param {string} provingKeyId - The ID of the proving key.
 * @param {Array<string>} publicInputs - The public inputs for the proof.
 * @param {Array<string>} privateInputs - The private inputs for the proof.
 * @param {Object} parameters - Additional parameters for proof generation.
 * @returns {string} The ID of the generated proof.
 */
export function generateProof(circuitId, provingKeyId, publicInputs, privateInputs, parameters = {}) {
  return core.ops.op_zk_generate_proof(circuitId, provingKeyId, publicInputs, privateInputs, parameters);
}

/**
 * Verify a Zero-Knowledge proof.
 * 
 * @param {string} proofId - The ID of the proof.
 * @param {string} verificationKeyId - The ID of the verification key.
 * @param {Array<string>} publicInputs - The public inputs for the proof.
 * @param {Object} parameters - Additional parameters for proof verification.
 * @returns {boolean} Whether the proof is valid.
 */
export function verifyProof(proofId, verificationKeyId, publicInputs, parameters = {}) {
  return core.ops.op_zk_verify_proof(proofId, verificationKeyId, publicInputs, parameters);
}

/**
 * ZK circuit types supported by the platform.
 */
export const CircuitType = {
  ZOKRATES: "zokrates",
  BULLETPROOFS: "bulletproofs",
  CIRCOM: "circom",
};

/**
 * Example usage of the ZK module.
 * 
 * @example
 * ```javascript
 * import { zk } from "r3e";
 * 
 * // Compile a ZoKrates circuit
 * const circuitSource = `
 *   def main(private field a, private field b, field c) -> bool:
 *     return a * b == c
 * `;
 * const circuitId = zk.compileCircuit(circuitSource, zk.CircuitType.ZOKRATES, "multiply");
 * 
 * // Generate keys
 * const { provingKeyId, verificationKeyId } = zk.generateKeys(circuitId);
 * 
 * // Generate a proof
 * const publicInputs = ["6"];
 * const privateInputs = ["2", "3"];
 * const proofId = zk.generateProof(circuitId, provingKeyId, publicInputs, privateInputs);
 * 
 * // Verify the proof
 * const isValid = zk.verifyProof(proofId, verificationKeyId, publicInputs);
 * console.log(`Proof is valid: ${isValid}`);
 * ```
 */
