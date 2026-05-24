//! GitOrbit - The Open-Source Agent Protocol for GitOrbit
//!
//! GitOrbit enables AI agents and developers to interact with the
//! GitOrbit intergalactic network for decentralized code collaboration.

use clap::{Parser, Subcommand};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod identity;
mod network;
mod storage;
mod agent;
mod api;

#[derive(Parser)]
#[command(name = "gitorbit")]
#[command(author = "GitOrbit Team")]
#[command(version = "0.2.0")]
#[command(about = "Intergalactic agent protocol for GitOrbit", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage your cosmic identity
    Identity {
        #[command(subcommand)]
        action: IdentityCommands,
    },
    /// Connect to the orbital network
    Connect {
        /// Galaxy to connect to
        #[arg(short, long, default_value = "andromeda")]
        galaxy: String,
    },
    /// Repository operations
    Repo {
        #[command(subcommand)]
        action: RepoCommands,
    },
    /// Agent management
    Agent {
        #[command(subcommand)]
        action: AgentCommands,
    },
    /// Swarm operations
    Swarm {
        #[command(subcommand)]
        action: SwarmCommands,
    },
}

#[derive(Subcommand)]
enum IdentityCommands {
    /// Create a new cosmic DID
    Create,
    /// Show your current identity
    Show,
    /// Export your identity
    Export {
        #[arg(short, long)]
        output: Option<String>,
    },
}

#[derive(Subcommand)]
enum RepoCommands {
    /// Create a new repository
    Create {
        /// Repository name
        name: String,
    },
    /// Clone a repository
    Clone {
        /// Repository DID
        did: String,
    },
    /// List repositories
    List {
        /// Galaxy to search
        #[arg(short, long)]
        galaxy: Option<String>,
    },
}

#[derive(Subcommand)]
enum AgentCommands {
    /// Spawn a new agent
    Spawn {
        /// Agent name
        #[arg(short, long)]
        name: String,
    },
    /// List active agents
    List,
    /// Stop an agent
    Stop {
        /// Agent DID
        did: String,
    },
}

#[derive(Subcommand)]
enum SwarmCommands {
    /// Launch an agent swarm
    Launch {
        /// Target file pattern
        #[arg(short, long)]
        target: String,
        /// Number of agents
        #[arg(short, long, default_value = "3")]
        agents: u32,
    },
    /// Check swarm status
    Status {
        /// Swarm ID
        id: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Identity { action } => match action {
            IdentityCommands::Create => {
                println!("⟳ Generating cosmic DID...");
                let did = identity::create_did()?;
                println!("✓ Created cosmic DID: {}", did);
                println!("✓ Keypair saved to ~/.gitorbit/identity.json");
            }
            IdentityCommands::Show => {
                let identity = identity::load_identity()?;
                println!("Cosmic DID: {}", identity.did);
                println!("Public Key: {}", identity.public_key);
            }
            IdentityCommands::Export { output } => {
                identity::export_identity(output)?;
                println!("✓ Identity exported");
            }
        },
        Commands::Connect { galaxy } => {
            println!("⟳ Establishing quantum link to {}...", galaxy);
            let connection = network::connect(&galaxy).await?;
            println!("✓ Connected to {} orbital nodes", connection.nodes);
            println!("✓ {} peers discovered", connection.peers);
        }
        Commands::Repo { action } => match action {
            RepoCommands::Create { name } => {
                println!("⟳ Creating repository {}...", name);
                let repo = storage::create_repo(&name).await?;
                println!("✓ Repository created: {}", repo.did);
                println!("✓ Added remote 'orbit' to git config");
            }
            RepoCommands::Clone { did } => {
                println!("⟳ Cloning repository {}...", did);
                storage::clone_repo(&did).await?;
                println!("✓ Repository cloned");
            }
            RepoCommands::List { galaxy } => {
                let repos = storage::list_repos(galaxy.as_deref()).await?;
                for repo in repos {
                    println!("{} - {}", repo.did, repo.name);
                }
            }
        },
        Commands::Agent { action } => match action {
            AgentCommands::Spawn { name } => {
                println!("⟳ Spawning agent {}...", name);
                let agent = agent::spawn(&name).await?;
                println!("✓ Agent spawned: {}", agent.did);
                println!("✓ Capabilities: {:?}", agent.capabilities);
                println!("Trust score: 0.00 (new entity)");
            }
            AgentCommands::List => {
                let agents = agent::list_agents().await?;
                for a in agents {
                    println!("{} - {} (trust: {:.2})", a.did, a.name, a.trust_score);
                }
            }
            AgentCommands::Stop { did } => {
                agent::stop(&did).await?;
                println!("✓ Agent {} stopped", did);
            }
        },
        Commands::Swarm { action } => match action {
            SwarmCommands::Launch { target, agents } => {
                println!("⟳ Launching swarm with {} agents...", agents);
                let swarm = agent::launch_swarm(&target, agents).await?;
                println!("✓ Swarm launched: {}", swarm.id);
                println!("✓ Target: {}", target);
                println!("✓ Monitoring at /swarm/{}", swarm.id);
            }
            SwarmCommands::Status { id } => {
                let status = agent::swarm_status(&id).await?;
                println!("Swarm: {}", id);
                println!("Status: {}", status.state);
                println!("Agents: {}/{}", status.active, status.total);
                println!("Progress: {:.1}%", status.progress);
            }
        },
    }

    Ok(())
}
