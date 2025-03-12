use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::Signer,
};
use anyhow::Result;
use tokio;
use clap::Parser;

mod cli;
mod keypair;
mod deploy;
mod serve;
mod noop;
mod utils;

use cli::{Cli, Commands};
use keypair::load_keypair;
use deploy::deploy;
use serve::serve;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let rpc_url = cli.cluster.rpc_url();
    let client = RpcClient::new_with_commitment(rpc_url.clone(), CommitmentConfig::confirmed());

    let keypair_path = cli.keypair_path.unwrap_or_else(|| {
        dirs::home_dir()
            .unwrap()
            .join(".config/solana/id.json")
    });
    let payer = load_keypair(&keypair_path)?;
    println!("Using keypair: {} from {}", payer.pubkey(), keypair_path.display());
    println!("Connected to: {}", rpc_url);
    println!("\n");

    match cli.command {
        Commands::Deploy { filename } => deploy(&client, &payer, &filename).await,
        Commands::Serve { tx_id } => serve(&client, tx_id.as_deref()).await,
    }
}
