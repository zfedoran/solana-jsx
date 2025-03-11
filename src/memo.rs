use solana_sdk::{
    instruction::{Instruction, AccountMeta},
    pubkey::Pubkey,
};
use spl_memo::id as memo_program_id;

pub fn build_memo(memo: &[u8], metadata: &str, signer_pubkeys: &[&Pubkey]) -> Instruction {
    let mut data = Vec::new();
    data.extend_from_slice(memo);
    data.extend_from_slice(metadata.as_bytes());
    Instruction {
        program_id: memo_program_id(),
        accounts: signer_pubkeys
            .iter()
            .map(|&pubkey| AccountMeta::new_readonly(*pubkey, true))
            .collect(),
        data,
    }
}
