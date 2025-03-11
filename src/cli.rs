use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "solana-jsx", about = "A Solana React component deployer (yes, really)")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short = 'k', long = "keypair")]
    pub keypair_path: Option<PathBuf>,

    #[arg(short = 'u', long = "url", default_value = "d")]
    pub cluster: Cluster,
}

#[derive(Subcommand)]
pub enum Commands {
    Deploy {
        #[arg(short, long)]
        filename: String,
    },
    Serve {
        #[arg(short, long)]
        tx_id: Option<String>,
    },
}

#[derive(ValueEnum, Clone)]
pub enum Cluster {
    #[value(name = "l")]
    Localnet,
    #[value(name = "m")]
    Mainnet,
    #[value(name = "d")]
    Devnet,
    #[value(name = "t")]
    Testnet,
}

impl Cluster {
    pub fn rpc_url(&self) -> String {
        match self {
            Cluster::Localnet => "http://127.0.0.1:8899".to_string(),
            Cluster::Mainnet => "https://api.mainnet-beta.solana.com".to_string(),
            Cluster::Devnet => "https://api.devnet.solana.com".to_string(),
            Cluster::Testnet => "https://api.testnet.solana.com".to_string(),
        }
    }
}
