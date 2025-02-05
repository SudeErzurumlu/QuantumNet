// quantum_entanglement.rs - Handles quantum entanglement operations

// Purpose of this module: Implements quantum entanglement mechanisms
// for simulating entangled quantum states within the network.

use crate::core::quantum_network::{QuantumNode, QuantumState, QuantumNetwork};

/// A structure to manage entanglement within the quantum network.
pub struct QuantumEntanglement;

impl QuantumEntanglement {
    /// Establishes entanglement between two quantum nodes.
    ///
    /// # Arguments
    /// * `network` - The mutable reference to the quantum network.
    /// * `node_id_1` - The ID of the first node.
    /// * `node_id_2` - The ID of the second node.
    ///
    /// # Returns
    /// * `Ok(())` if the entanglement is successful.
    /// * `Err(String)` if the entanglement process fails.
    pub fn entangle_nodes(network: &mut QuantumNetwork, node_id_1: u32, node_id_2: u32) -> Result<(), String> {
        let node_1 = network.get_node_mut(node_id_1);
        let node_2 = network.get_node_mut(node_id_2);

        if let (Some(node_1), Some(node_2)) = (node_1, node_2) {
            // If both nodes exist, entangle them by linking their quantum states
            let entangled_state = QuantumState::Entangled(Box::new(node_1.state.clone()));
            node_2.state = entangled_state;

            Ok(())
        } else {
            Err("One or both nodes were not found.".to_string())
        }
    }

    /// Checks if two nodes are entangled.
    ///
    /// # Arguments
    /// * `node_1` - A reference to the first quantum node.
    /// * `node_2` - A reference to the second quantum node.
    ///
    /// # Returns
    /// * `true` if the nodes are entangled.
    /// * `false` otherwise.
    pub fn are_entangled(node_1: &QuantumNode, node_2: &QuantumNode) -> bool {
        match &node_2.state {
            QuantumState::Entangled(entangled_state) => {
                // Compare entangled state with node_1's state
                *entangled_state.clone() == node_1.state
            }
            _ => false,
        }
    }

    /// Breaks the entanglement between two nodes, resetting their states.
    ///
    /// # Arguments
    /// * `network` - The mutable reference to the quantum network.
    /// * `node_id` - The ID of the node whose entanglement should be removed.
    ///
    /// # Returns
    /// * `Ok(())` if successful.
    /// * `Err(String)` if the node is not found or not entangled.
    pub fn break_entanglement(network: &mut QuantumNetwork, node_id: u32) -> Result<(), String> {
        if let Some(node) = network.get_node_mut(node_id) {
            match node.state {
                QuantumState::Entangled(_) => {
                    node.state = QuantumState::Zero; // Reset state to ground state
                    Ok(())
                }
                _ => Err("Node is not in an entangled state.".to_string()),
            }
        } else {
            Err("Node not found.".to_string())
        }
    }
}
