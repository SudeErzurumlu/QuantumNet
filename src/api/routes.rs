// routes.rs - Defines API endpoints for interacting with the Quantum Network.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Json as AxumJson, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::core::api::QuantumAPI;
use crate::core::quantum_packet::QuantumPacket;

/// Represents the shared state of the API.
#[derive(Clone)]
struct AppState {
    api: Arc<QuantumAPI>,
}

/// Defines the structure of a request for node registration.
#[derive(Deserialize)]
struct RegisterNodeRequest {
    node_id: u32,
}

/// Defines the structure of a request for entangling two nodes.
#[derive(Deserialize)]
struct EntangleNodesRequest {
    node1: u32,
    node2: u32,
}

/// Defines the structure of a request for key exchange.
#[derive(Deserialize)]
struct KeyExchangeRequest {
    node1: u32,
    node2: u32,
}

/// Defines the structure of a message-sending request.
#[derive(Deserialize)]
struct SendMessageRequest {
    sender_id: u32,
    receiver_id: u32,
    message: String,
}

/// Defines the structure of a response for node status.
#[derive(Serialize)]
struct NodeStatusResponse {
    entangled_nodes: Vec<u32>,
    key_count: usize,
}

/// Registers a new quantum node.
async fn register_node(
    State(state): State<AppState>,
    AxumJson(payload): AxumJson<RegisterNodeRequest>,
) -> StatusCode {
    if state.api.register_node(payload.node_id) {
        StatusCode::CREATED
    } else {
        StatusCode::CONFLICT
    }
}

/// Establishes entanglement between two nodes.
async fn entangle_nodes(
    State(state): State<AppState>,
    AxumJson(payload): AxumJson<EntangleNodesRequest>,
) -> StatusCode {
    if state.api.entangle_nodes(payload.node1, payload.node2) {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    }
}

/// Initiates Quantum Key Distribution (QKD).
async fn exchange_keys(
    State(state): State<AppState>,
    AxumJson(payload): AxumJson<KeyExchangeRequest>,
) -> StatusCode {
    if state.api.exchange_keys(payload.node1, payload.node2) {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    }
}

/// Sends a quantum-secure message.
async fn send_message(
    State(state): State<AppState>,
    AxumJson(payload): AxumJson<SendMessageRequest>,
) -> Json<Option<QuantumPacket>> {
    let packet = state
        .api
        .send_message(payload.sender_id, payload.receiver_id, &payload.message);
    Json(packet)
}

/// Retrieves the status of a quantum node.
async fn get_node_status(
    State(state): State<AppState>,
    Path(node_id): Path<u32>,
) -> Json<Option<NodeStatusResponse>> {
    let status = state.api.get_node_status(node_id);
    Json(status.map(|(entangled_nodes, key_count)| NodeStatusResponse {
        entangled_nodes,
        key_count,
    }))
}

/// Sets up the router and defines all API routes.
pub fn create_router(api: Arc<QuantumAPI>) -> Router {
    let state = AppState { api };

    Router::new()
        .route("/register", post(register_node))
        .route("/entangle", post(entangle_nodes))
        .route("/exchange_keys", post(exchange_keys))
        .route("/send_message", post(send_message))
        .route("/node_status/:node_id", get(get_node_status))
        .with_state(state)
}
