use solana_sdk::signature::Keypair;
use std::path::PathBuf;
use anyhow::{Result, anyhow};
use std::fs;

pub fn load_keypair(path: &PathBuf) -> Result<Keypair> {
    let data = fs::read_to_string(path)
        .map_err(|e| anyhow!("Failed to read keypair file {}: {}", path.display(), e))?;
    let bytes: Vec<u8> = serde_json::from_str(&data)
        .map_err(|e| anyhow!("Failed to parse keypair JSON: {}", e))?;
    Keypair::from_bytes(&bytes)
        .map_err(|e| anyhow!("Failed to create keypair from bytes: {}", e))
}
