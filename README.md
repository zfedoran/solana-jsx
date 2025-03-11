# Solana-JSX
![license][license-image]
![version][version-image]

[version-image]: https://img.shields.io/badge/version-0.1.0-blue.svg?style=flat
[license-image]: https://img.shields.io/badge/license-MIT-blue.svg?style=flat

![Screenshot 2025-03-11 at 3 01 25 PM](https://github.com/user-attachments/assets/e251a725-cd65-4820-98d2-c4a15491553c)

Welcome to `solana-jsx`, the world's most impractical utility for "deploying" 
React components to the Solana blockchain using the SPL Memo program. Born 
from a ridiculous question—"Can you run JavaScript on Solana?"—this tool 
answers with a resounding "No, but lets keep going anyway!"

> [!WARNING]
> Although it works, this is not a real deployment solution. Solana doesn’t run JavaScript. This is a meme. Enjoy responsibly.

## What Does It Do?
- **Deploy**: Compresses your React component, chunks it, and sends it to Solana as a linked-list of [memos](https://github.com/solana-program/memo) inside one or more transactions.
- **Serve**: Retrieves those chunks, decompresses them, and serves your React
component as a live webpage via a local server.

## Is it free?
You can use this for free on devnet and testnet, where you get SOL [airdroped](https://solana.com/developers/guides/getstarted/solana-token-airdrop-and-faucets) for free. Optionally, you can be **bold** and use this on mainnet for 5000 lamports per 200 bytes.

## Why?
Because.

## Installation
1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Clone this repo:
```bash
   git clone https://github.com/zfedoran/solana-jsx.git
   cd solana-jsx
```
3. Build it:
```bash
   cargo build --release
```

## Usage

### Prerequisites
- A Solana [keypair](https://solana.com/docs/intro/installation#create-wallet) (default: \`~/.config/solana/id.json\`).
- Some SOL on your chosen cluster ([Devnet is free](https://solana.com/docs/intro/installation#airdrop-sol)!).

### Commands

#### Deploy a JSX File
```bash
solana-jsx deploy -f example/app.jsx
```
- `-f`: Path to your JSX file.
- `-k`: (Optional) Path to your keypair.
- `-u`: (Optional) Cluster (`l`ocalnet, `m`ainnet, `d`evnet (default), `t`estnet).

This compresses your JSX, splits it into memo-sized chunks, and uploads them to
Solana. Copy the final transaction ID printed at the end.

#### Serve a Deployed Component
```bash
    solana-jsx serve -t <final-tx-id>
```
- `-t`: The final transaction ID from the deploy step.

This fetches the chunks, reassembles your JSX, strips imports/exports (because Solana doesn’t care), and serves it at `http://localhost:3030`.

### Example

To run the example that ships with this project, clone the repository and run the following.

1. Deploy:
```bash
    cargo run -- deploy -f example/app.jsx
```
Output: `Final Transaction ID: <tx>`

2. Serve:
```bash
   cargo run -- serve -t <tx>
```

3. Open `http://localhost:3030` and marvel at your decentralized React app!

## How It Works
1. **Compression**: Your JS is gzipped to save space.
2. **Chunking**: The compressed data is split into 200-byte chunks and base64-encoded.
3. **Memo Magic**: Each chunk is stored in a Solana memo with metadata linking to the previous chunk.
4. **Retrieval**: The serve command follows the chain of transaction IDs, reassembles the chunks, decompresses, and serves it with React and Babel.

## Limitations
- Max memo size is [566 bytes](https://spl.solana.com/memo#compute-limits).
- Only supports single-file components.
- Imports/exports in JSX are stripped (Solana isn’t Node.js, sorry).

## Contributing
Feel free to fork, PR, or just enjoy its absurdity. Ideas:
- Add JSX minification.
- Support multi-file components (because why stop at one?).
- Make it even dumber somehow.

## License
MIT—do whatever you want, just don’t blame me when it breaks.

