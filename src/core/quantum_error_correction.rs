// quantum_error_correction.rs - Implements quantum error correction algorithms.

// Purpose of this module: Provides error correction techniques for preserving
// quantum information by detecting and correcting quantum errors.

use crate::core::quantum_network::{QuantumNode, QuantumState};
use rand::Rng;

/// Represents different types of quantum errors that can occur.
#[derive(Debug, Clone, PartialEq)]
pub enum QuantumError {
    BitFlip,  // X error: Flips the quantum state |0> ↔ |1>
    PhaseFlip, // Z error: Alters the phase of a quantum state
    Depolarizing, // Randomizes the state
}

/// A structure that handles quantum error correction.
pub struct QuantumErrorCorrection;

impl QuantumErrorCorrection {
    /// Simulates the introduction of quantum errors into a node's quantum state.
    ///
    /// # Arguments
    /// * `node` - A mutable reference to the quantum node.
    ///
    /// # Returns
    /// * `QuantumError` - The type of error applied.
    pub fn introduce_error(node: &mut QuantumNode) -> QuantumError {
        let mut rng = rand::thread_rng();
        let error_type = rng.gen_range(0..=2);

        let error = match error_type {
            0 => QuantumError::BitFlip,
            1 => QuantumError::PhaseFlip,
            _ => QuantumError::Depolarizing,
        };

        match error {
            QuantumError::BitFlip => {
                if let QuantumState::Zero = node.state {
                    node.state = QuantumState::One;
                } else {
                    node.state = QuantumState::Zero;
                }
            }
            QuantumError::PhaseFlip => {
                if let QuantumState::Entangled(_) = node.state {
                    // Simulate phase flip by disrupting entanglement
                    node.state = QuantumState::Zero;
                }
            }
            QuantumError::Depolarizing => {
                node.state = QuantumState::Zero; // Reset to base state for simplicity
            }
        }

        error
    }

    /// Detects if an error has occurred in a given quantum node.
    ///
    /// # Arguments
    /// * `original_state` - The expected original quantum state.
    /// * `current_state` - The current quantum state after transmission.
    ///
    /// # Returns
    /// * `Some(QuantumError)` if an error is detected.
    /// * `None` if no error is found.
    pub fn detect_error(original_state: &QuantumState, current_state: &QuantumState) -> Option<QuantumError> {
        if original_state == current_state {
            None
        } else {
            Some(QuantumError::Depolarizing) // Generic error detection
        }
    }

    /// Corrects a quantum error in a node based on a redundancy check.
    ///
    /// # Arguments
    /// * `node` - A mutable reference to the quantum node.
    /// * `expected_state` - The original expected state of the node.
    ///
    /// # Returns
    /// * `true` if the error was successfully corrected.
    /// * `false` if correction was unsuccessful.
    pub fn correct_error(node: &mut QuantumNode, expected_state: &QuantumState) -> bool {
        if let Some(_) = QuantumErrorCorrection::detect_error(expected_state, &node.state) {
            node.state = expected_state.clone(); // Restore the expected state
            true
        } else {
            false
        }
    }
}
