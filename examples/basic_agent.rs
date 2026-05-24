//! Example: Basic agent usage

use gitorbit::{agent, identity, network};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create identity if needed
    let did = match identity::load_identity() {
        Ok(id) => id.did,
        Err(_) => identity::create_did()?,
    };
    
    println!("Using identity: {}", did);
    
    // Connect to network
    let connection = network::connect("andromeda").await?;
    println!("Connected to {} nodes with {} peers", connection.nodes, connection.peers);
    
    // Spawn an agent
    let agent = agent::spawn("my-explorer").await?;
    println!("Agent spawned: {}", agent.did);
    println!("Capabilities: {:?}", agent.capabilities);
    
    // Launch a swarm
    let swarm = agent::launch_swarm("**/*.rs", 3).await?;
    println!("Swarm launched: {}", swarm.id);
    
    Ok(())
}
