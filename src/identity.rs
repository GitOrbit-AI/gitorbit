//! Identity module for Cosmic DID management

use anyhow::Result;
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Cosmic DID identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub did: String,
    pub public_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
}

/// Create a new Cosmic DID identity
pub fn create_did() -> Result<String> {
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key: VerifyingKey = (&signing_key).into();
    
    // Encode public key to multibase
    let public_key_bytes = verifying_key.to_bytes();
    let multibase_key = bs58::encode(&public_key_bytes).into_string();
    
    // Generate DID
    let did = format!("did:orbit:z6Mk{}", &multibase_key[..32]);
    
    // Save identity
    let identity = Identity {
        did: did.clone(),
        public_key: multibase_key,
        private_key: Some(bs58::encode(signing_key.to_bytes()).into_string()),
    };
    
    save_identity(&identity)?;
    
    Ok(did)
}

/// Load identity from disk
pub fn load_identity() -> Result<Identity> {
    let path = get_identity_path()?;
    let content = std::fs::read_to_string(path)?;
    let identity: Identity = serde_json::from_str(&content)?;
    Ok(identity)
}

/// Save identity to disk
fn save_identity(identity: &Identity) -> Result<()> {
    let path = get_identity_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(identity)?;
    std::fs::write(path, content)?;
    Ok(())
}

/// Export identity
pub fn export_identity(output: Option<String>) -> Result<()> {
    let identity = load_identity()?;
    let export = Identity {
        did: identity.did,
        public_key: identity.public_key,
        private_key: None, // Never export private key
    };
    
    let content = serde_json::to_string_pretty(&export)?;
    
    if let Some(path) = output {
        std::fs::write(path, content)?;
    } else {
        println!("{}", content);
    }
    
    Ok(())
}

/// Get identity file path
fn get_identity_path() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    Ok(home.join(".gitorbit").join("identity.json"))
}

/// Sign data with identity
pub fn sign(data: &[u8]) -> Result<Vec<u8>> {
    let identity = load_identity()?;
    let private_key = identity.private_key.ok_or_else(|| anyhow::anyhow!("No private key"))?;
    let key_bytes = bs58::decode(&private_key).into_vec()?;
    let signing_key = SigningKey::from_bytes(&key_bytes.try_into().map_err(|_| anyhow::anyhow!("Invalid key"))?);
    
    use ed25519_dalek::Signer;
    let signature = signing_key.sign(data);
    Ok(signature.to_bytes().to_vec())
}

/// Verify a signature
pub fn verify(data: &[u8], signature: &[u8], public_key: &str) -> Result<bool> {
    let key_bytes = bs58::decode(public_key).into_vec()?;
    let verifying_key = VerifyingKey::from_bytes(&key_bytes.try_into().map_err(|_| anyhow::anyhow!("Invalid key"))?)?;
    
    let sig = ed25519_dalek::Signature::from_bytes(&signature.try_into().map_err(|_| anyhow::anyhow!("Invalid signature"))?);
    
    use ed25519_dalek::Verifier;
    Ok(verifying_key.verify(data, &sig).is_ok())
}
