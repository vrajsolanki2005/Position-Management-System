use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use solana_client::rpc_client::RpcClient;
use anyhow::Result;

pub struct SolanaMarketConfig {
    pub symbol: String,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub oracle: Pubkey,
    pub im_rate: u64,
    pub mm_rate: u64,
}

pub fn get_btc_eth_markets() -> Vec<SolanaMarketConfig> {
    vec![
        SolanaMarketConfig {
            symbol: "BTC-PERP".to_string(),
            base_mint: Pubkey::new_from_array([1; 32]), // BTC mint
            quote_mint: Pubkey::new_from_array([2; 32]), // USDC mint
            oracle: Pubkey::new_from_array([3; 32]), // Pyth BTC oracle
            im_rate: 5000, // 0.5% initial margin
            mm_rate: 2500, // 0.25% maintenance margin
        },
        SolanaMarketConfig {
            symbol: "ETH-PERP".to_string(),
            base_mint: Pubkey::new_from_array([4; 32]), // ETH mint
            quote_mint: Pubkey::new_from_array([2; 32]), // USDC mint
            oracle: Pubkey::new_from_array([5; 32]), // Pyth ETH oracle
            im_rate: 10000, // 1% initial margin
            mm_rate: 5000,  // 0.5% maintenance margin
        },
    ]
}

pub fn derive_market_pda(program_id: &Pubkey, symbol: &str) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"market", symbol.as_bytes()],
        program_id
    )
}

pub async fn initialize_market(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &Pubkey,
    config: &SolanaMarketConfig,
) -> Result<String> {
    let (market_pda, _bump) = derive_market_pda(program_id, &config.symbol);
    
    // Create market account instruction (simplified)
    let create_ix = system_instruction::create_account(
        &payer.pubkey(),
        &market_pda,
        client.get_minimum_balance_for_rent_exemption(256)?,
        256,
        program_id,
    );
    
    let recent_blockhash = client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );
    
    let signature = client.send_and_confirm_transaction(&tx)?;
    Ok(signature.to_string())
}