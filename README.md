# Solana-JSX

Welcome to `solana-jsx`, the world's most impractical utility for "deploying" 
JSX React components to the Solana blockchain using the SPL Memo program. Born 
from a ridiculous question—"Can you run JavaScript on Solana?"—this tool 
answers with a resounding "No, but lets keep going anyway!"

## What Does It Do?
- **Deploy**: Compresses your JSX file, chunks it, and stores it as
base64-encoded memos on Solana.
- **Serve**: Retrieves those chunks, decompresses them, and serves your React
component as a live webpage via a local server.
- **Laugh**: Reminds you that this is a terrible idea for production but
hilarious anyway.

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
- A Solana keypair (default: \`~/.config/solana/id.json\`).
- Some SOL on your chosen cluster (Devnet is free!).

### Commands

#### Deploy a JSX File
```bash
    cargo run --release -- deploy -f example/SolanaApp.jsx
```
- `-f`: Path to your JSX file.
- `-k`: (Optional) Path to your keypair.
- `-u`: (Optional) Cluster (`l`ocalnet, `m`ainnet, `d`evnet (default), `t`estnet).

This compresses your JSX, splits it into memo-sized chunks, and uploads them to
Solana. Copy the final transaction ID printed at the end.

#### Serve a Deployed Component
```bash
    cargo run --release -- serve -t <final-tx-id>
```
- `-t`: The final transaction ID from the deploy step.

This fetches the chunks, reassembles your JSX, strips imports/exports (because Solana doesn’t care), and serves it at `http://localhost:3030`.

### Example
1. Deploy:
```bash
    cargo run --release -- deploy -f example/SolanaApp.jsx
```
Output: `Final Transaction ID: <some-tx-id>`

2. Serve:
```bash
   cargo run -- release -- serve -t <some-tx-id>
```
3. Open `http://localhost:3030` and marvel at your decentralized React app!

## How It Works
1. **Compression**: Your JSX is gzipped to save space.
2. **Chunking**: The compressed data is split into 200-byte chunks and base64-encoded (CU limited).
3. **Memo Magic**: Each chunk is stored in a Solana memo with metadata linking to the previous chunk.
4. **Retrieval**: The serve command follows the chain of transaction IDs, reassembles the chunks, decompresses, and serves it with React and Babel via a Warp server.

## Limitations
- Max memo size is 1024 bytes, so large JSX files might break (CU limits also matter).
- Only supports single-file components.
- Imports/exports in JSX are stripped (Solana isn’t Node.js, sorry).
- This is a joke—don’t use it for anything serious.

## Contributing
Feel free to fork, PR, or just laugh at this absurdity. Ideas:
- Add JSX minification.
- Support multi-file components (because why stop at one?).
- Make it even dumber somehow.

## License
MIT—do whatever you want, just don’t blame me when it breaks.

## Disclaimer
This is not a real deployment solution. Solana doesn’t run JavaScript. This is a meme. Enjoy responsibly.
