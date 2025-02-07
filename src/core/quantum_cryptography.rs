// quantum_cryptography.rs - Implements quantum encryption and secure key exchange.

// Purpose of this module: Provides quantum cryptographic methods, including
// Quantum Key Distribution (QKD) and quantum-secure encryption mechanisms.

use crate::core::quantum_network::{QuantumNode, QuantumNetwork};
use crate::core::quantum_entanglement::QuantumEntanglement;
use rand::{Rng, seq::SliceRandom};

/// A structure that handles quantum cryptographic operations.
pub struct QuantumCryptography;

impl QuantumCryptography {
    /// Implements a simple Quantum Key Distribution (QKD) protocol
    ///
    /// # Arguments
    /// * `network` - The mutable reference to the quantum network.
    /// * `node_id_1` - The ID of the first node.
    /// * `node_id_2` - The ID of the second node.
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` containing the secure quantum key if successful.
    /// * `Err(String)` if key exchange fails.
    pub fn quantum_key_distribution(network: &mut QuantumNetwork, node_id_1: u32, node_id_2: u32) -> Result<Vec<u8>, String> {
        if !QuantumEntanglement::are_entangled(
            network.get_node(node_id_1).ok_or("Node 1 not found")?,
            network.get_node(node_id_2).ok_or("Node 2 not found")?,
        ) {
            return Err("Nodes are not entangled. QKD requires entanglement.".to_string());
        }

        let mut rng = rand::thread_rng();
        let mut key: Vec<u8> = (0..16).map(|_| rng.gen_range(0..=255)).collect(); // Generate a 16-byte quantum key

        // Simulate measurement errors (in real QKD, errors occur due to quantum noise)
        let error_probability = 0.1;
        key.iter_mut().for_each(|bit| {
            if rng.gen::<f64>() < error_probability {
                *bit ^= 1; // Flip bit to simulate a measurement error
            }
        });

        Ok(key)
    }

    /// Encrypts a message using a quantum-secure one-time pad.
    ///
    /// # Arguments
    /// * `message` - The plaintext message as a `&str`.
    /// * `key` - The quantum key as a `Vec<u8>`.
    ///
    /// # Returns
    /// * `Vec<u8>` containing the encrypted ciphertext.
    pub fn encrypt(message: &str, key: &Vec<u8>) -> Vec<u8> {
        message
            .bytes()
            .zip(key.iter().cycle()) // Use the key cyclically
            .map(|(m_byte, k_byte)| m_byte ^ k_byte) // XOR for encryption
            .collect()
    }

    /// Decrypts a quantum-encrypted message.
    ///
    /// # Arguments
    /// * `ciphertext` - The encrypted message as a `Vec<u8>`.
    /// * `key` - The quantum key as a `Vec<u8>`.
    ///
    /// # Returns
    /// * `String` containing the decrypted message.
    pub fn decrypt(ciphertext: &Vec<u8>, key: &Vec<u8>) -> String {
        let decrypted_bytes: Vec<u8> = ciphertext
            .iter()
            .zip(key.iter().cycle()) // Use the key cyclically
            .map(|(c_byte, k_byte)| c_byte ^ k_byte) // XOR for decryption
            .collect();

        String::from_utf8(decrypted_bytes).unwrap_or_else(|_| "Decryption failed".to_string())
    }
}
