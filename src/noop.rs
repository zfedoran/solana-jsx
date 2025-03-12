use solana_sdk::{
    instruction::{Instruction, AccountMeta},
    pubkey::Pubkey,
};

pub const NOOP_PROGRAM_ID: Pubkey = Pubkey::new_from_array([
    0x0b, 0xbc, 0x0f, 0xc0, 0xbb, 0x47, 0xca, 0x2f,
    0x74, 0xc4, 0x11, 0x2e, 0x94, 0xab, 0x13, 0xcf,
    0xa3, 0xc6, 0x34, 0xe5, 0xdc, 0x17, 0xea, 0xcb,
    0x03, 0xcd, 0x1a, 0x23, 0xcd, 0x7e, 0x78, 0x7c
]);

pub fn build_noop_instruction(data: &[u8], signer_pubkeys: &[&Pubkey]) -> Instruction {
    Instruction {
        program_id: NOOP_PROGRAM_ID,
        accounts: signer_pubkeys
            .iter()
            .map(|&pubkey| AccountMeta::new_readonly(*pubkey, true))
            .collect(),
        data: data.to_vec(),
    }
}
