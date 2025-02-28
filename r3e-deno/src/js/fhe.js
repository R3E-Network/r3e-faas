// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

/**
 * @module fhe
 * @description Fully Homomorphic Encryption module for the R3E FaaS platform.
 */

import { core } from "./infra.js";
import { encode, decode } from "./encoding.js";

/**
 * Generate a key pair for FHE operations.
 * 
 * @param {string} schemeType - The type of FHE scheme to use (e.g., "TFHE", "OpenFHE").
 * @param {Object} parameters - Additional parameters for key generation.
 * @returns {string} The ID of the generated key pair.
 */
export function generateKeys(schemeType, parameters = {}) {
  return core.ops.op_fhe_generate_keys(schemeType, parameters);
}

/**
 * Encrypt data using a public key.
 * 
 * @param {string} publicKeyId - The ID of the public key.
 * @param {Uint8Array|string} plaintext - The data to encrypt (Uint8Array or string).
 * @returns {string} The ID of the generated ciphertext.
 */
export function encrypt(publicKeyId, plaintext) {
  // Convert string to Uint8Array if necessary
  const data = typeof plaintext === 'string' ? encode(plaintext) : plaintext;
  return core.ops.op_fhe_encrypt(publicKeyId, data);
}

/**
 * Decrypt data using a private key.
 * 
 * @param {string} privateKeyId - The ID of the private key.
 * @param {string} ciphertextId - The ID of the ciphertext.
 * @param {boolean} asString - Whether to return the result as a string (default: false).
 * @returns {Uint8Array|string} The decrypted data.
 */
export function decrypt(privateKeyId, ciphertextId, asString = false) {
  const data = core.ops.op_fhe_decrypt(privateKeyId, ciphertextId);
  return asString ? decode(data) : data;
}

/**
 * Add two ciphertexts homomorphically.
 * 
 * @param {string} ciphertext1Id - The ID of the first ciphertext.
 * @param {string} ciphertext2Id - The ID of the second ciphertext.
 * @returns {string} The ID of the resulting ciphertext.
 */
export function add(ciphertext1Id, ciphertext2Id) {
  return core.ops.op_fhe_add(ciphertext1Id, ciphertext2Id);
}

/**
 * Subtract one ciphertext from another homomorphically.
 * 
 * @param {string} ciphertext1Id - The ID of the first ciphertext.
 * @param {string} ciphertext2Id - The ID of the second ciphertext.
 * @returns {string} The ID of the resulting ciphertext.
 */
export function subtract(ciphertext1Id, ciphertext2Id) {
  return core.ops.op_fhe_subtract(ciphertext1Id, ciphertext2Id);
}

/**
 * Multiply two ciphertexts homomorphically.
 * 
 * @param {string} ciphertext1Id - The ID of the first ciphertext.
 * @param {string} ciphertext2Id - The ID of the second ciphertext.
 * @returns {string} The ID of the resulting ciphertext.
 */
export function multiply(ciphertext1Id, ciphertext2Id) {
  return core.ops.op_fhe_multiply(ciphertext1Id, ciphertext2Id);
}

/**
 * Negate a ciphertext homomorphically.
 * 
 * @param {string} ciphertextId - The ID of the ciphertext.
 * @returns {string} The ID of the resulting ciphertext.
 */
export function negate(ciphertextId) {
  return core.ops.op_fhe_negate(ciphertextId);
}

/**
 * Get a ciphertext by ID.
 * 
 * @param {string} ciphertextId - The ID of the ciphertext.
 * @returns {Object} The ciphertext object.
 */
export function getCiphertext(ciphertextId) {
  return core.ops.op_fhe_get_ciphertext(ciphertextId);
}

/**
 * Estimate the noise budget of a ciphertext.
 * 
 * @param {string} ciphertextId - The ID of the ciphertext.
 * @returns {number|null} The estimated noise budget, or null if not available.
 */
export function estimateNoiseBudget(ciphertextId) {
  return core.ops.op_fhe_estimate_noise_budget(ciphertextId);
}

/**
 * FHE scheme types supported by the platform.
 */
export const SchemeType = {
  TFHE: "TFHE",
  OPENFHE: "OpenFHE",
  SEAL: "SEAL",
  HELIB: "HElib",
  LATTIGO: "Lattigo",
};

/**
 * Example usage of the FHE module.
 * 
 * @example
 * ```javascript
 * import { fhe } from "r3e";
 * 
 * // Generate keys
 * const keyPairId = fhe.generateKeys(fhe.SchemeType.TFHE);
 * 
 * // Encrypt data
 * const plaintext = "Hello, FHE!";
 * const ciphertextId = fhe.encrypt(publicKeyId, plaintext);
 * 
 * // Perform homomorphic operations
 * const ciphertext2Id = fhe.encrypt(publicKeyId, "World");
 * const resultId = fhe.add(ciphertextId, ciphertext2Id);
 * 
 * // Decrypt the result
 * const result = fhe.decrypt(privateKeyId, resultId, true);
 * console.log(`Result: ${result}`);
 * 
 * // Check the noise budget
 * const noiseBudget = fhe.estimateNoiseBudget(resultId);
 * console.log(`Noise budget: ${noiseBudget}`);
 * ```
 */
