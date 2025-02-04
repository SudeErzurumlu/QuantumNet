// quantum_network.rs - Main module for quantum network simulation.

// Purpose of the module: To implement core functionalities for quantum networks.
// Quantum networks enable quantum tunneling, entanglement-based data transfer, and quantum security.

// Import necessary libraries.
use rand::Rng;  // To generate random numbers
use std::fmt;   // For error messages and formatting

// Define the structure for a Quantum Node
#[derive(Debug, Clone)]
pub struct QuantumNode {
    pub id: u32,               // Node identifier
    pub position: (f64, f64),  // 2D position of the node in space
    pub state: QuantumState,   // Quantum state of the node
}

#[derive(Debug, Clone)]
pub enum QuantumState {
    Zero,  // Ground state
    One,   // First state
    Entangled(Box<QuantumState>), // Entangled states
}

// Define the Quantum Network structure
#[derive(Debug)]
pub struct QuantumNetwork {
    nodes: Vec<QuantumNode>,  // List of quantum nodes in the network
}

impl QuantumNetwork {
    // Function to create a new Quantum Network
    pub fn new() -> Self {
        QuantumNetwork {
            nodes: Vec::new(),
        }
    }

    // Function to add a new node to the quantum network
    pub fn add_node(&mut self, id: u32, position: (f64, f64), state: QuantumState) {
        let node = QuantumNode {
            id,
            position,
            state,
        };
        self.nodes.push(node);
    }

    // Function to get a node by its ID
    pub fn get_node(&self, id: u32) -> Option<&QuantumNode> {
        self.nodes.iter().find(|&node| node.id == id)
    }

    // Function to simulate entangling two nodes
    pub fn entangle_nodes(&mut self, node_id_1: u32, node_id_2: u32) -> Result<(), String> {
        let node_1 = self.get_node_mut(node_id_1);
        let node_2 = self.get_node_mut(node_id_2);

        if let (Some(node_1), Some(node_2)) = (node_1, node_2) {
            let new_state = QuantumState::Entangled(Box::new(node_1.state.clone()));
            node_2.state = new_state; // Entangle node 2 with the state of node 1
            Ok(())
        } else {
            Err("One or both nodes not found.".to_string())
        }
    }

    // Function to simulate quantum tunneling between two nodes
    pub fn quantum_tunneling(&mut self, node_id_1: u32, node_id_2: u32) -> Result<(), String> {
        let node_1 = self.get_node_mut(node_id_1);
        let node_2 = self.get_node_mut(node_id_2);

        if let (Some(node_1), Some(node_2)) = (node_1, node_2) {
            let mut rng = rand::thread_rng();
            let tunneling_probability: f64 = rng.gen(); // Random value for tunneling probability

            if tunneling_probability < 0.5 {
                // Simulate tunneling if probability is less than 0.5
                node_1.state = node_2.state.clone();
                Ok(())
            } else {
                Err("Quantum tunneling failed.".to_string())
            }
        } else {
            Err("One or both nodes not found.".to_string())
        }
    }

    // Helper function to get a mutable reference to a node by ID
    fn get_node_mut(&mut self, id: u32) -> Option<&mut QuantumNode> {
        self.nodes.iter_mut().find(|node| node.id == id)
    }
}

// Implement the Display trait for easy printing of QuantumNetwork
impl fmt::Display for QuantumNetwork {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Quantum Network with {} nodes", self.nodes.len())
    }
}

