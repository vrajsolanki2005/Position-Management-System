use solana_sdk::pubkey::Pubkey;

pub fn user_pda(program: &Pubkey, owner: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"user", owner.as_ref()], program)
}
pub fn position_pda(program: &Pubkey, owner: &Pubkey, symbol: &str) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"position", owner.as_ref(), symbol.as_bytes()], program)
}
pub fn vault_pda(program: &Pubkey, quote_mint: &Pubkey) -> (Pubkey, u8) {
    // In the on-chain code, vault PDA seed was ["vault", quote_mint]; authority = ["vault_authority"]
    Pubkey::find_program_address(&[b"vault", quote_mint.as_ref()], program)
}
pub fn vault_authority_pda(program: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"vault_authority"], program)
}