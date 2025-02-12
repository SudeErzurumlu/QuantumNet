// api.rs - Main API for Quantum Network interactions.

// Purpose of this module:
// - Provides an interface for external applications to interact with the quantum network.
// - Exposes functionalities for node creation, entanglement, key exchange, and secure messaging.

use crate::core::quantum_node::QuantumNode;
use crate::core::quantum_packet::QuantumPacket;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Represents the global quantum network API.
pub struct QuantumAPI {
    nodes: Arc<Mutex<HashMap<u32, QuantumNode>>>, // Stores all registered quantum nodes
}

impl QuantumAPI {
    /// Creates a new instance of the quantum API.
    ///
    /// # Returns
    /// * `QuantumAPI` - A new instance managing the quantum network.
    pub fn new() -> Self {
        QuantumAPI {
            nodes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Registers a new quantum node in the network.
    ///
    /// # Arguments
    /// * `node_id` - Unique identifier for the quantum node.
    ///
    /// # Returns
    /// * `true` if the node was successfully added, `false` if it already exists.
    pub fn register_node(&self, node_id: u32) -> bool {
        let mut nodes = self.nodes.lock().unwrap();
        if nodes.contains_key(&node_id) {
            false // Node already exists
        } else {
            nodes.insert(node_id, QuantumNode::new(node_id));
            true
        }
    }

    /// Establishes quantum entanglement between two nodes.
    ///
    /// # Arguments
    /// * `node1` - The first node's ID.
    /// * `node2` - The second node's ID.
    ///
    /// # Returns
    /// * `true` if entanglement was successful, `false` otherwise.
    pub fn entangle_nodes(&self, node1: u32, node2: u32) -> bool {
        let mut nodes = self.nodes.lock().unwrap();
        if let (Some(node_a), Some(node_b)) = (nodes.get_mut(&node1), nodes.get_mut(&node2)) {
            node_a.entangle_with(node2) && node_b.entangle_with(node1)
        } else {
            false
        }
    }

    /// Initiates Quantum Key Distribution (QKD) between two entangled nodes.
    ///
    /// # Arguments
    /// * `node1` - The first node's ID.
    /// * `node2` - The second node's ID.
    ///
    /// # Returns
    /// * `true` if key exchange was successful, `false` otherwise.
    pub fn exchange_keys(&self, node1: u32, node2: u32) -> bool {
        let mut nodes = self.nodes.lock().unwrap();
        if let (Some(node_a), Some(node_b)) = (nodes.get_mut(&node1), nodes.get_mut(&node2)) {
            node_a.exchange_keys(node2) && node_b.exchange_keys(node1)
        } else {
            false
        }
    }

    /// Sends a quantum-secure message between two nodes.
    ///
    /// # Arguments
    /// * `sender_id` - The ID of the sender node.
    /// * `receiver_id` - The ID of the receiver node.
    /// * `message` - The plaintext message to send.
    ///
    /// # Returns
    /// * `Option<QuantumPacket>` - The encrypted packet if successful.
    pub fn send_message(&self, sender_id: u32, receiver_id: u32, message: &str) -> Option<QuantumPacket> {
        let nodes = self.nodes.lock().unwrap();
        if let Some(sender) = nodes.get(&sender_id) {
            sender.send_packet(receiver_id, message)
        } else {
            None
        }
    }

    /// Receives and decrypts a quantum-secure message.
    ///
    /// # Arguments
    /// * `receiver_id` - The ID of the receiver node.
    /// * `packet` - The incoming encrypted quantum packet.
    ///
    /// # Returns
    /// * `Option<String>` - The decrypted message if successful.
    pub fn receive_message(&self, receiver_id: u32, packet: QuantumPacket) -> Option<String> {
        let nodes = self.nodes.lock().unwrap();
        if let Some(receiver) = nodes.get(&receiver_id) {
            receiver.receive_packet(&packet)
        } else {
            None
        }
    }

    /// Retrieves the status of a quantum node.
    ///
    /// # Arguments
    /// * `node_id` - The ID of the node.
    ///
    /// # Returns
    /// * `Option<(Vec<u32>, usize)>` - A tuple containing entangled nodes and key count.
    pub fn get_node_status(&self, node_id: u32) -> Option<(Vec<u32>, usize)> {
        let nodes = self.nodes.lock().unwrap();
        nodes.get(&node_id).map(|node| (node.entangled_nodes.clone(), node.key_store.len()))
    }
}
