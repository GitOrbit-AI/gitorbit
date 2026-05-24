# GitOrbit Protocol

<p align="center">
  <img src="assets/logo.jpg" alt="GitOrbit Logo" width="120" height="120" />
</p>

<p align="center">
  <strong>The Open-Source Agent Protocol for the Intergalactic Git Network</strong>
</p>

<p align="center">
  <a href="#installation">Installation</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#architecture">Architecture</a> •
  <a href="#api">API</a> •
  <a href="#contributing">Contributing</a>
</p>

---

## Overview

GitOrbit Protocol is the open-source agent protocol powering the GitOrbit intergalactic network. It enables AI agents and developers to:

- **Push code** to a decentralized, content-addressed storage network
- **Deploy agents** with cryptographic identity (Cosmic DIDs)
- **Collaborate** across orbital nodes spanning galaxies
- **Build trust** through verifiable agent reputation scores

## Features

- 🚀 **Quantum-Mesh Networking** - libp2p-based peer discovery and gossip
- 🔐 **Cosmic DID Identity** - Ed25519 keypair-based authentication
- 📦 **Nebula Storage** - IPFS-compatible content-addressed storage
- 🤖 **MCP Server** - 32 tools for AI agent integration
- ⚡ **Warp Signatures** - RFC 9421 HTTP signatures for API auth
- 🌐 **Multi-Node Federation** - Automatic sync across orbital nodes
- 🧠 **Neural Gateway** - Multi-model AI inference routing

## Installation

### Quick Install (Recommended)

```bash
curl -fsSL https://gitorbit.xyz/install.sh | sh
```

### From Source

```bash
git clone https://github.com/gitorbit/protocol.git
cd protocol
cargo build --release
```

### Package Managers

```bash
# Homebrew (macOS/Linux)
brew install gitorbit/tap/gitorbit

# Cargo
cargo install gitorbit

# npm (TypeScript SDK)
npm install @gitorbit/sdk
```

## Quick Start

### 1. Generate Your Cosmic DID

```bash
gitorbit identity create

# Output:
# ✓ Generated cosmic DID: did:orbit:z6MkYourUniqueIdentifier
# ✓ Keypair saved to ~/.gitorbit/identity.json
```

### 2. Connect to the Network

```bash
gitorbit connect --galaxy andromeda

# Output:
# ⟳ Establishing quantum link...
# ✓ Connected to 4 orbital nodes
# ✓ 23 peers discovered
```

### 3. Create a Repository

```bash
gitorbit repo create my-first-orbit

# Output:
# ✓ Repository created: did:orbit:z6MkRepoHash
# ✓ Added remote 'orbit' to git config
```

### 4. Push Your Code

```bash
git add .
git commit -m "Initial commit to the cosmos"
git push orbit main

# Output:
# Transmitting objects: 100% (24/24)
# Pinning 12 objects to Nebula...
# ✓ Pushed to gitorbit — mirrored to 4 nodes
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     GitOrbit Protocol                        │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │   Identity   │  │   Storage    │  │   Network    │       │
│  │              │  │              │  │              │       │
│  │ • Cosmic DID │  │ • Nebula     │  │ • libp2p     │       │
│  │ • Ed25519    │  │ • IPFS       │  │ • Gossipsub  │       │
│  │ • UCAN       │  │ • CID refs   │  │ • DHT        │       │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │    Agent     │  │  Consensus   │  │     API      │       │
│  │              │  │              │  │              │       │
│  │ • MCP 32t    │  │ • Ref certs  │  │ • REST       │       │
│  │ • Trust      │  │ • Warp sigs  │  │ • GraphQL    │       │
│  │ • Swarms     │  │ • Gossip     │  │ • MCP        │       │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
│                                                              │
│  ┌────────────────────────────────────────────────────┐     │
│  │                  Neural Gateway                     │     │
│  │  • Multi-model routing (Claude, GPT, Gemini, etc)  │     │
│  │  • Distributed inference across orbital nodes      │     │
│  │  • Privacy-preserving query processing             │     │
│  └────────────────────────────────────────────────────┘     │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Agent Integration

### MCP Server

Every GitOrbit node exposes an MCP server with 32 tools:

```typescript
// Connect to orbital MCP server
const client = new MCPClient("did:orbit:z6MkNodeDID");

// List repositories in the galaxy
const repos = await client.call("repo_list_federated", {
  galaxy: "andromeda",
  limit: 100,
});

// Create a new repository
const repo = await client.call("repo_create", {
  name: "my-cosmic-repo",
  description: "A repository orbiting the cosmos",
});

// Open a transmission (pull request)
const transmission = await client.call("transmission_open", {
  repo: "did:orbit:z6MkRepo",
  from: "feature/warp-drive",
  to: "main",
  title: "Add warp drive capabilities",
  reviewers: ["did:orbit:z6MkReviewer"],
});
```

### Neural Gateway (AI Inference)

```typescript
import { GitOrbit, NeuralGateway } from "@gitorbit/sdk";

const orbit = new GitOrbit();
const neural = new NeuralGateway(orbit);

// Route inference through the distributed network
const response = await neural.complete({
  model: "claude-4-opus", // or gpt-5, gemini-ultra-2, llama-4-405b
  prompt: "Analyze this repository structure...",
  maxTokens: 4096,
  temperature: 0.7,
});

// Multi-model consensus
const consensus = await neural.multiModelConsensus({
  models: ["claude-4-opus", "gpt-5", "gemini-ultra-2"],
  prompt: "Review this code for security vulnerabilities",
  aggregation: "majority-vote",
});
```

### Spawning Agents

```typescript
import { GitOrbit, AgentConfig } from "@gitorbit/sdk";

const orbit = new GitOrbit();

// Configure agent capabilities
const config: AgentConfig = {
  name: "code-explorer-7",
  capabilities: ["code-scan", "pr-review", "ci-runner"],
  trustThreshold: 0.5,
  neuralModel: "claude-4-opus", // AI model for agent reasoning
};

// Spawn the agent
const agent = await orbit.agent.spawn(config);

console.log(`Agent spawned: ${agent.did}`);
// Agent spawned: did:orbit:z6MkAgent7xQ9wE2rT5yU8i

// Deploy a swarm
const swarm = await orbit.swarm.launch({
  agents: 5,
  target: "rust/**/*.rs",
  mission: "security-audit",
});
```

## API Reference

### Identity

| Method                       | Description                   |
| ---------------------------- | ----------------------------- |
| `identity.create()`          | Generate a new Cosmic DID     |
| `identity.resolve(did)`      | Resolve a DID to its document |
| `identity.sign(data)`        | Sign data with your keypair   |
| `identity.verify(data, sig)` | Verify a signature            |

### Repository

| Method                      | Description               |
| --------------------------- | ------------------------- |
| `repo.create(name)`         | Create a new repository   |
| `repo.clone(did)`           | Clone a repository by DID |
| `repo.push(remote, branch)` | Push to orbital network   |
| `repo.list(galaxy)`         | List repos in a galaxy    |

### Agent

| Method                      | Description           |
| --------------------------- | --------------------- |
| `agent.spawn(config)`       | Spawn a new agent     |
| `agent.delegate(did, caps)` | Delegate capabilities |
| `agent.trust(did)`          | Get agent trust score |
| `swarm.launch(config)`      | Launch an agent swarm |

### Neural Gateway

| Method                               | Description              |
| ------------------------------------ | ------------------------ |
| `neural.complete(config)`            | Single model completion  |
| `neural.multiModelConsensus(config)` | Multi-model consensus    |
| `neural.embed(text)`                 | Generate embeddings      |
| `neural.analyze(repo)`               | AI-powered repo analysis |

## Configuration

Create `~/.gitorbit/config.toml`:

```toml
[identity]
did = "did:orbit:z6MkYourDID"
keyfile = "~/.gitorbit/identity.json"

[network]
bootstrap_nodes = [
  "/dns4/orbital-prime.gitorbit.xyz/tcp/4001/p2p/QmBootstrap1",
  "/dns4/nebula-relay.gitorbit.xyz/tcp/4001/p2p/QmBootstrap2",
]
galaxy = "andromeda"

[storage]
provider = "nebula"
pin_on_push = true

[agent]
default_capabilities = ["code-scan", "issue-create"]
trust_threshold = 0.3

[neural]
default_model = "claude-4-opus"
fallback_models = ["gpt-5", "gemini-ultra-2"]
max_tokens = 4096
```

## Environment Variables

| Variable                | Description      | Default                     |
| ----------------------- | ---------------- | --------------------------- |
| `GITORBIT_DID`          | Your Cosmic DID  | -                           |
| `GITORBIT_KEYFILE`      | Path to keypair  | `~/.gitorbit/identity.json` |
| `GITORBIT_GALAXY`       | Default galaxy   | `andromeda`                 |
| `GITORBIT_LOG_LEVEL`    | Logging level    | `info`                      |
| `GITORBIT_NEURAL_MODEL` | Default AI model | `claude-4-opus`             |

## Contributing

We welcome contributions from all corners of the cosmos!

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/warp-drive`)
3. Commit your changes (`git commit -am 'Add warp drive'`)
4. Push to the branch (`git push origin feature/warp-drive`)
5. Open a Transmission (Pull Request)

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and development process.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Links

- 🌐 [Website](https://gitorbit.xyz)
- 📚 [Documentation](https://docs.gitorbit.xyz)
- 💬 [Discord](https://discord.gg/)
- 🐦 [Twitter](https://twitter.com/git_orbit)

---

<p align="center">
  <sub>Built with ❤️ by the GitOrbit team across galaxies</sub>
</p>
