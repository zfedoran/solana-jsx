use solana_client::{rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs;
use std::io::Write;
use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose};
use crate::noop::build_noop_instruction;

pub async fn deploy(client: &RpcClient, payer: &Keypair, filename: &str) -> Result<()> {
    let file_data = fs::read(filename)?;
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&file_data)?;
    let compressed_data = encoder.finish()?;

    let chunks: Vec<&[u8]> = compressed_data.chunks(1200).collect(); // Increased chunk size
    let total_chunks = chunks.len();

    let mut prev_tx_id: Option<String> = None;

    for (i, chunk) in chunks.iter().enumerate() {
        let metadata = if let Some(prev) = &prev_tx_id {
            format!("|{}|{}", i + 1, prev)
        } else {
            format!("|{}|start", i + 1)
        };

        let encoded_chunk = general_purpose::STANDARD.encode(chunk);
        let mut instruction_data = Vec::new();
        instruction_data.extend_from_slice(encoded_chunk.as_bytes());
        instruction_data.extend_from_slice(metadata.as_bytes());

        let total_size = instruction_data.len();

        println!(
            "Chunk {}/{}: Raw size: {} bytes, Encoded size: {} bytes, Metadata: {} bytes, Total: {} bytes",
            i + 1, total_chunks, chunk.len(), encoded_chunk.len(), metadata.len(), total_size
        );
        println!("Metadata content: {}", metadata);

        if total_size > 1232 { // Noop has higher limit than memo
            return Err(anyhow!(
                "Instruction data exceeds 1232 bytes: {} bytes",
                total_size
            ));
        }

        let instruction = build_noop_instruction(&instruction_data, &[&payer.pubkey()]);
        let recent_blockhash = client.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[payer],
            recent_blockhash,
        );

        let config = RpcSendTransactionConfig {
            skip_preflight: true,
            preflight_commitment: None,
            encoding: None,
            max_retries: None,
            min_context_slot: None,
        };
        let signature = client.send_transaction_with_config(&tx, config)?;
        prev_tx_id = Some(signature.to_string());

        println!("Chunk {}/{} deployed: {}", i + 1, total_chunks, signature);
    }

    if let Some(tx_id) = prev_tx_id {
        println!("Final Transaction ID: {}", tx_id);
        println!("\n");
        println!("To verify the deployment, run:");
        println!("solana-jsx serve -t {}", tx_id);
        println!("\n");
    } else {
        return Err(anyhow!("No transactions sent"));
    }

    Ok(())
}
