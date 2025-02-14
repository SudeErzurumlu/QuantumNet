// handlers.rs - Handles HTTP requests for the Quantum Network API.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    Json as AxumJson,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::core::api::QuantumAPI;
use crate::core::quantum_packet::QuantumPacket;

/// Represents the shared application state.
#[derive(Clone)]
pub struct AppState {
    pub api: Arc<QuantumAPI>,
}

/// Request structure for registering a new quantum node.
#[derive(Deserialize)]
pub struct RegisterNodeRequest {
    pub node_id: u32,
}

/// Request structure for establishing entanglement between nodes.
#[derive(Deserialize)]
pub struct EntangleNodesRequest {
    pub node1: u32,
    pub node2: u32,
}

/// Request structure for quantum key exchange.
#[derive(Deserialize)]
pub struct KeyExchangeRequest {
    pub node1: u32,
    pub node2: u32,
}

/// Request structure for sending a secure quantum message.
#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub sender_id: u32,
    pub receiver_id: u32,
    pub message: String,
}

/// Response structure for retrieving the status of a quantum node.
#[derive(Serialize)]
pub struct NodeStatusResponse {
    pub entangled_nodes: Vec<u32>,
    pub key_count: usize,
}

/// Handles the registration of a new quantum node.
pub async fn register_node(
    State(state): State<AppState>,
    AxumJson(payload): AxumJson<RegisterNodeRequest>,
) -> StatusCode {
    if state.api.register_node(payload.node_id) {
        StatusCode::CREATED
    } else {
        StatusCode::CONFLICT
    }
}

/// Handles the establishment of quantum entanglement between two nodes.
pub async fn entangle_nodes(
    State(state): State<AppState>,
    AxumJson(payload): AxumJson<EntangleNodesRequest>,
) -> StatusCode {
    if state.api.entangle_nodes(payload.node1, payload.node2) {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    }
}

/// Handles the quantum key distribution (QKD) process.
pub async fn exchange_keys(
    State(state): State<AppState>,
    AxumJson(payload): AxumJson<KeyExchangeRequest>,
) -> StatusCode {
    if state.api.exchange_keys(payload.node1, payload.node2) {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    }
}

/// Handles sending a quantum-secure message.
pub async fn send_message(
    State(state): State<AppState>,
    AxumJson(payload): AxumJson<SendMessageRequest>,
) -> Json<Option<QuantumPacket>> {
    let packet = state
        .api
        .send_message(payload.sender_id, payload.receiver_id, &payload.message);
    Json(packet)
}

/// Handles retrieving the status of a quantum node.
pub async fn get_node_status(
    State(state): State<AppState>,
    Path(node_id): Path<u32>,
) -> Json<Option<NodeStatusResponse>> {
    let status = state.api.get_node_status(node_id);
    Json(status.map(|(entangled_nodes, key_count)| NodeStatusResponse {
        entangled_nodes,
        key_count,
    }))
}
