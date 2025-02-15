# app.py - FastAPI-based web interface for the Quantum Network.

from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from typing import Dict, Optional, List
import requests

# Initialize FastAPI
app = FastAPI(title="Quantum Network API", version="1.0")

# Define the backend Rust API URL
RUST_API_URL = "http://localhost:8000"

# Data models for request payloads
class RegisterNodeRequest(BaseModel):
    node_id: int

class EntangleNodesRequest(BaseModel):
    node1: int
    node2: int

class KeyExchangeRequest(BaseModel):
    node1: int
    node2: int

class SendMessageRequest(BaseModel):
    sender_id: int
    receiver_id: int
    message: str

# API Endpoints

@app.post("/register")
def register_node(request: RegisterNodeRequest):
    """ Registers a quantum node. """
    response = requests.post(f"{RUST_API_URL}/register", json=request.dict())
    if response.status_code == 201:
        return {"message": "Node registered successfully."}
    elif response.status_code == 409:
        raise HTTPException(status_code=409, detail="Node already exists.")
    raise HTTPException(status_code=response.status_code, detail=response.text)

@app.post("/entangle")
def entangle_nodes(request: EntangleNodesRequest):
    """ Establishes quantum entanglement between two nodes. """
    response = requests.post(f"{RUST_API_URL}/entangle", json=request.dict())
    if response.status_code == 200:
        return {"message": "Nodes entangled successfully."}
    raise HTTPException(status_code=response.status_code, detail=response.text)

@app.post("/exchange_keys")
def exchange_keys(request: KeyExchangeRequest):
    """ Initiates quantum key distribution (QKD). """
    response = requests.post(f"{RUST_API_URL}/exchange_keys", json=request.dict())
    if response.status_code == 200:
        return {"message": "Keys exchanged successfully."}
    raise HTTPException(status_code=response.status_code, detail=response.text)

@app.post("/send_message")
def send_message(request: SendMessageRequest):
    """ Sends a quantum-secure message. """
    response = requests.post(f"{RUST_API_URL}/send_message", json=request.dict())
    if response.status_code == 200:
        return response.json()
    raise HTTPException(status_code=response.status_code, detail=response.text)

@app.get("/node_status/{node_id}")
def get_node_status(node_id: int):
    """ Retrieves the status of a quantum node. """
    response = requests.get(f"{RUST_API_URL}/node_status/{node_id}")
    if response.status_code == 200:
        return response.json()
    raise HTTPException(status_code=response.status_code, detail=response.text)
