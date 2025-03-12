use solana_client::{rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature};
use solana_transaction_status::UiTransactionEncoding;
use anyhow::{Result, anyhow};
use warp::Filter;
use crate::utils::decompress;
use crate::noop::NOOP_PROGRAM_ID;

pub async fn serve(client: &RpcClient, tx_id: Option<&str>) -> Result<()> {
    let decompressed = if let Some(tx_id) = tx_id {
        let mut chunks = Vec::new();
        let mut current_tx_id = tx_id.to_string();

        loop {
            println!("Fetching transaction: {}", current_tx_id);
            let signature = current_tx_id.parse::<Signature>()
                .map_err(|e| anyhow!("Invalid transaction ID: {}", e))?;

            let tx = client.get_transaction_with_config(
                &signature,
                RpcTransactionConfig {
                    encoding: Some(UiTransactionEncoding::Base64),
                    commitment: Some(CommitmentConfig::confirmed()),
                    max_supported_transaction_version: Some(0),
                },
            )?;

            let tx = tx.transaction.transaction.decode()
                .ok_or_else(|| anyhow!("Failed to decode transaction: {}", current_tx_id))?;

            let instruction = tx.message.instructions()
                .into_iter()
                .find(|ix| {
                    *ix.program_id(&tx.message.static_account_keys()) == NOOP_PROGRAM_ID
                })
                .ok_or_else(|| anyhow!("No noop instruction found in tx: {}", current_tx_id))?;

            let instruction_data = instruction.data.clone();

            // Find the second-to-last '|' to locate the start of metadata
            let pipe_positions = instruction_data
                .iter()
                .rposition(|&b| b == b'|')
                .and_then(|last| {
                    instruction_data[..last]
                        .iter()
                        .rposition(|&b| b == b'|')
                });

            let metadata_start = pipe_positions
                .ok_or_else(|| anyhow!("No valid metadata separator found in instruction data"))?;

            let chunk_data = &instruction_data[..metadata_start];
            let metadata = String::from_utf8(instruction_data[metadata_start..].to_vec())?;
            let parts: Vec<&str> = metadata.split('|').collect();

            if parts.len() != 3 {
                return Err(anyhow!("Invalid metadata format: {}", metadata));
            }

            let chunk_num = parts[1].parse::<usize>()?;
            let prev = parts[2];

            chunks.push((chunk_num, chunk_data.to_vec()));

            if prev == "start" {
                break;
            } else {
                current_tx_id = prev.to_string();
            }
        }

        chunks.sort_by_key(|&(num, _)| num);
        let mut data = Vec::new();
        for (_, chunk) in chunks {
            data.extend_from_slice(&chunk);
        }

        decompress(&data)?
    } else {
        return Err(anyhow!("No transaction ID provided for serving"));
    };

    let jsx_content = String::from_utf8(decompressed)?;
    let cleaned_jsx_content = jsx_content
        .lines()
        .filter(|line| !line.trim().starts_with("import") && !line.trim().starts_with("export"))
        .collect::<Vec<&str>>()
        .join("\n");

    let html_content = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
          <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Solana React Component</title>
            <script src="https://unpkg.com/react@18/umd/react.production.min.js"></script>
            <script src="https://unpkg.com/react-dom@18/umd/react-dom.production.min.js"></script>
            <script src="https://unpkg.com/@babel/standalone/babel.min.js"></script>
          </head>
          <body>
            <div id="root"></div>
            <script type="text/babel">
              {cleaned_jsx_content}
              const root = ReactDOM.createRoot(document.getElementById("root"));
              root.render(<App />);
            </script>
          </body>
        </html>
        "#
    );

    let html = warp::path::end().map(move || warp::reply::html(html_content.clone()));
    println!("\n");
    println!("Serving at http://localhost:3030");
    println!("\n");
    warp::serve(html).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
