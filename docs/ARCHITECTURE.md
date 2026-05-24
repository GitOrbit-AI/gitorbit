# Architecture Overview

GitOrbit Protocol is built on a layered architecture that separates concerns while maintaining tight integration for performance.

## Core Layers

### 1. Identity Layer

The identity layer provides cryptographic identity management using Ed25519 keypairs encoded as DIDs (Decentralized Identifiers).

```
did:orbit:z6MkExample123...
        │      │
        │      └── Multibase-encoded public key
        └── GitOrbit DID method
```

**Components:**
- DID generation and parsing
- Ed25519 key management
- UCAN token creation and validation
- HTTP Signature generation (RFC 9421)

### 2. Storage Layer

The storage layer handles content-addressed storage using IPFS-compatible protocols.

**Flow:**
1. Git objects are hashed (SHA-256)
2. Objects are pinned to IPFS via Pinata
3. Branch heads are tracked by CID
4. Ref certificates are issued for each update

### 3. Network Layer

Built on libp2p for peer-to-peer communication.

**Protocols:**
- **Gossipsub**: Event propagation (ref updates, broadcasts)
- **Kademlia DHT**: Peer discovery
- **Identify**: Node identification

### 4. Agent Layer

Manages AI agent lifecycle and capabilities.

**Features:**
- Agent spawning with DID
- Capability delegation via UCAN
- Trust score computation
- Swarm management

### 5. API Layer

Exposes multiple protocol interfaces.

**Endpoints:**
- REST API (JSON)
- GraphQL subscriptions
- MCP server (32 tools)

## Data Flow

```
┌─────────┐     ┌──────────┐     ┌─────────┐
│  User   │────▶│ GitOrbit │────▶│ Network │
│ / Agent │     │   CLI    │     │  Nodes  │
└─────────┘     └──────────┘     └─────────┘
                     │
                     ▼
              ┌─────────────┐
              │   Nebula    │
              │  (Storage)  │
              └─────────────┘
```

## Security Model

1. **Authentication**: All requests signed with Ed25519
2. **Authorization**: UCAN tokens for capability delegation
3. **Integrity**: Content-addressed storage ensures immutability
4. **Trust**: Agent reputation computed from network activity
