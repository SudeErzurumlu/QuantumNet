// simulator.rs - Controls the overall quantum network simulation.

// Purpose of this module: 
// - Manages quantum nodes and their interactions
// - Simulates entanglement, cryptography, and error correction
// - Provides a testing environment for quantum communication

use crate::core::quantum_network::{QuantumNetwork, QuantumNode};
use crate::core::quantum_entanglement::QuantumEntanglement;
use crate::core::quantum_cryptography::QuantumCryptography;
use crate::core::quantum_error_correction::QuantumErrorCorrection;

/// Represents the main quantum network simulator.
pub struct QuantumSimulator {
    network: QuantumNetwork,
}

impl QuantumSimulator {
    /// Creates a new instance of the Quantum Simulator.
    pub fn new() -> Self {
        QuantumSimulator {
            network: QuantumNetwork::new(),
        }
    }

    /// Adds a quantum node to the simulation.
    ///
    /// # Arguments
    /// * `node_id` - The ID of the new quantum node.
    pub fn add_node(&mut self, node_id: u32) {
        self.network.add_node(QuantumNode::new(node_id));
    }

    /// Establishes quantum entanglement between two nodes.
    ///
    /// # Arguments
    /// * `node_id_1` - The ID of the first node.
    /// * `node_id_2` - The ID of the second node.
    ///
    /// # Returns
    /// * `true` if entanglement was successfully established.
    /// * `false` if the operation failed.
    pub fn entangle_nodes(&mut self, node_id_1: u32, node_id_2: u32) -> bool {
        QuantumEntanglement::entangle(&mut self.network, node_id_1, node_id_2)
    }

    /// Performs quantum key distribution (QKD) between two nodes.
    ///
    /// # Arguments
    /// * `node_id_1` - The ID of the first node.
    /// * `node_id_2` - The ID of the second node.
    ///
    /// # Returns
    /// * `Some(Vec<u8>)` - The generated quantum key if successful.
    /// * `None` - If QKD fails.
    pub fn perform_qkd(&mut self, node_id_1: u32, node_id_2: u32) -> Option<Vec<u8>> {
        match QuantumCryptography::quantum_key_distribution(&mut self.network, node_id_1, node_id_2) {
            Ok(key) => Some(key),
            Err(_) => None,
        }
    }

    /// Encrypts and transmits a message securely.
    ///
    /// # Arguments
    /// * `message` - The plaintext message.
    /// * `key` - The encryption key.
    ///
    /// # Returns
    /// * `Vec<u8>` - The encrypted message.
    pub fn secure_transmit(&self, message: &str, key: &Vec<u8>) -> Vec<u8> {
        QuantumCryptography::encrypt(message, key)
    }

    /// Receives and decrypts a quantum-secure message.
    ///
    /// # Arguments
    /// * `ciphertext` - The encrypted message.
    /// * `key` - The decryption key.
    ///
    /// # Returns
    /// * `String` - The decrypted message.
    pub fn secure_receive(&self, ciphertext: &Vec<u8>, key: &Vec<u8>) -> String {
        QuantumCryptography::decrypt(ciphertext, key)
    }

    /// Introduces errors into a specific quantum node.
    ///
    /// # Arguments
    /// * `node_id` - The ID of the node where errors will be introduced.
    ///
    /// # Returns
    /// * `Option<String>` - The type of error if introduced.
    pub fn introduce_errors(&mut self, node_id: u32) -> Option<String> {
        if let Some(node) = self.network.get_node_mut(node_id) {
            let error = QuantumErrorCorrection::introduce_error(node);
            Some(format!("{:?}", error))
        } else {
            None
        }
    }

    /// Detects and corrects errors in a given quantum node.
    ///
    /// # Arguments
    /// * `node_id` - The ID of the node to be checked.
    ///
    /// # Returns
    /// * `bool` - `true` if the error was detected and corrected, `false` otherwise.
    pub fn detect_and_correct_errors(&mut self, node_id: u32) -> bool {
        if let Some(node) = self.network.get_node_mut(node_id) {
            let expected_state = QuantumNode::new(node_id).state; // Assume ideal state
            QuantumErrorCorrection::correct_error(node, &expected_state)
        } else {
            false
        }
    }
}
