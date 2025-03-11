use solana_client::{rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature};
use solana_transaction_status::{UiTransactionEncoding, option_serializer::OptionSerializer};
use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose};
use warp::Filter;
use crate::utils::decompress;

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

            let logs = tx.transaction.meta
                .ok_or_else(|| anyhow!("No metadata in transaction: {}", current_tx_id))?;

            let memo_log = match logs.log_messages {
                OptionSerializer::Some(logs) => logs
                    .into_iter()
                    .find(|log| log.contains("Memo (len"))
                    .ok_or_else(|| anyhow!("No memo found in logs for tx: {}", current_tx_id))?,
                _ => return Err(anyhow!("No log messages in transaction: {}", current_tx_id)),
            };

            let memo_data = memo_log
                .split('"')
                .nth(1)
                .ok_or_else(|| anyhow!("Failed to parse memo log: {}", memo_log))?;
            println!("Memo data: {}", memo_data);

            let parts: Vec<&str> = memo_data.split('|').collect();
            if parts.len() != 3 {
                return Err(anyhow!("Invalid memo format: {}", memo_data));
            }

            let encoded_chunk = parts[0];
            let chunk_num = parts[1].parse::<usize>()?;
            let prev = parts[2];

            let decoded_chunk = general_purpose::STANDARD.decode(encoded_chunk)?;
            chunks.push((chunk_num, decoded_chunk));

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
