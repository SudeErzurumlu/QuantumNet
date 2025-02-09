// quantum_packet.rs - Defines quantum data packets for secure communication.

// Purpose of this module: 
// - Structures quantum information into transmittable packets.
// - Encodes and decodes quantum data.
// - Ensures integrity using quantum cryptographic techniques.

use crate::core::quantum_cryptography::QuantumCryptography;

/// Represents different types of quantum packets.
#[derive(Debug, Clone, PartialEq)]
pub enum QuantumPacketType {
    Entanglement,   // Used for quantum entanglement distribution
    KeyExchange,    // Used for quantum key distribution (QKD)
    EncryptedData,  // Secure data transmission
    ErrorCorrection, // Error correction metadata
}

/// Struct representing a quantum packet.
#[derive(Debug, Clone)]
pub struct QuantumPacket {
    pub packet_type: QuantumPacketType, // Type of quantum packet
    pub sender_id: u32,   // ID of the sending quantum node
    pub receiver_id: u32, // ID of the receiving quantum node
    pub payload: Vec<u8>, // Encoded quantum data
}

impl QuantumPacket {
    /// Creates a new quantum packet.
    ///
    /// # Arguments
    /// * `packet_type` - The type of the quantum packet.
    /// * `sender_id` - The sender node ID.
    /// * `receiver_id` - The receiver node ID.
    /// * `payload` - The data to be transmitted.
    ///
    /// # Returns
    /// * `QuantumPacket` - A new quantum data packet.
    pub fn new(packet_type: QuantumPacketType, sender_id: u32, receiver_id: u32, payload: Vec<u8>) -> Self {
        QuantumPacket {
            packet_type,
            sender_id,
            receiver_id,
            payload,
        }
    }

    /// Encrypts the quantum packet using a quantum-secure key.
    ///
    /// # Arguments
    /// * `key` - The encryption key.
    ///
    /// # Returns
    /// * `QuantumPacket` - The encrypted quantum packet.
    pub fn encrypt(&self, key: &Vec<u8>) -> QuantumPacket {
        let encrypted_payload = QuantumCryptography::encrypt(&String::from_utf8_lossy(&self.payload), key);
        QuantumPacket {
            packet_type: self.packet_type.clone(),
            sender_id: self.sender_id,
            receiver_id: self.receiver_id,
            payload: encrypted_payload,
        }
    }

    /// Decrypts the quantum packet using a quantum-secure key.
    ///
    /// # Arguments
    /// * `key` - The decryption key.
    ///
    /// # Returns
    /// * `QuantumPacket` - The decrypted quantum packet.
    pub fn decrypt(&self, key: &Vec<u8>) -> QuantumPacket {
        let decrypted_payload = QuantumCryptography::decrypt(&self.payload, key);
        QuantumPacket {
            packet_type: self.packet_type.clone(),
            sender_id: self.sender_id,
            receiver_id: self.receiver_id,
            payload: decrypted_payload.into_bytes(),
        }
    }
}
