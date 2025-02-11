// quantum_node.rs - Defines quantum nodes in the network.

// Purpose of this module: 
// - Represents individual quantum network nodes.
// - Manages entanglement and quantum key distribution (QKD).
// - Handles quantum packet transmission and reception.

use crate::core::quantum_packet::{QuantumPacket, QuantumPacketType};
use crate::core::quantum_cryptography::QuantumCryptography;
use crate::core::quantum_entanglement::QuantumEntanglement;
use std::collections::HashMap;

/// Represents a quantum node in the network.
#[derive(Debug, Clone)]
pub struct QuantumNode {
    pub id: u32,                     // Unique node ID
    pub entangled_nodes: Vec<u32>,   // List of entangled node IDs
    pub key_store: HashMap<u32, Vec<u8>>, // Stores quantum keys (per node)
}

impl QuantumNode {
    /// Creates a new quantum node.
    ///
    /// # Arguments
    /// * `id` - The unique identifier for the node.
    ///
    /// # Returns
    /// * `QuantumNode` - A new quantum node instance.
    pub fn new(id: u32) -> Self {
        QuantumNode {
            id,
            entangled_nodes: Vec::new(),
            key_store: HashMap::new(),
        }
    }

    /// Establishes quantum entanglement with another node.
    ///
    /// # Arguments
    /// * `peer_id` - The ID of the node to entangle with.
    ///
    /// # Returns
    /// * `true` if entanglement was successful, `false` otherwise.
    pub fn entangle_with(&mut self, peer_id: u32) -> bool {
        if QuantumEntanglement::entangle_nodes(self.id, peer_id) {
            self.entangled_nodes.push(peer_id);
            true
        } else {
            false
        }
    }

    /// Performs Quantum Key Distribution (QKD) with an entangled node.
    ///
    /// # Arguments
    /// * `peer_id` - The ID of the node to exchange keys with.
    ///
    /// # Returns
    /// * `true` if the key was successfully exchanged, `false` otherwise.
    pub fn exchange_keys(&mut self, peer_id: u32) -> bool {
        if self.entangled_nodes.contains(&peer_id) {
            if let Ok(key) = QuantumCryptography::quantum_key_distribution(self.id, peer_id) {
                self.key_store.insert(peer_id, key);
                return true;
            }
        }
        false
    }

    /// Sends a quantum data packet to another node.
    ///
    /// # Arguments
    /// * `receiver_id` - The ID of the destination node.
    /// * `data` - The plaintext message.
    ///
    /// # Returns
    /// * `Option<QuantumPacket>` - The encrypted packet if successful.
    pub fn send_packet(&self, receiver_id: u32, data: &str) -> Option<QuantumPacket> {
        if let Some(key) = self.key_store.get(&receiver_id) {
            let encrypted_packet = QuantumPacket::new(
                QuantumPacketType::EncryptedData,
                self.id,
                receiver_id,
                QuantumCryptography::encrypt(data, key),
            );
            Some(encrypted_packet)
        } else {
            None
        }
    }

    /// Receives and decrypts a quantum data packet.
    ///
    /// # Arguments
    /// * `packet` - The incoming encrypted quantum packet.
    ///
    /// # Returns
    /// * `Option<String>` - The decrypted message if successful.
    pub fn receive_packet(&self, packet: &QuantumPacket) -> Option<String> {
        if let Some(key) = self.key_store.get(&packet.sender_id) {
            Some(QuantumCryptography::decrypt(&packet.payload, key))
        } else {
            None
        }
    }
}
