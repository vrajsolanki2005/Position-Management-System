use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
    instruction::{Instruction, AccountMeta},
};
use solana_client::rpc_client::RpcClient;
use anyhow::Result;

pub async fn execute_market_trade(
    client: &RpcClient,
    trader: &Keypair,
    program_id: &Pubkey,
    symbol: &str,
    size: u64, // 1 qty = 1000000 (6 decimals)
    leverage: u8, // 20x
    is_long: bool,
) -> Result<String> {
    let (market_pda, _) = crate::solana_markets::derive_market_pda(program_id, symbol);
    let (position_pda, _) = crate::perpetual_mechanics::derive_position_pda(program_id, &trader.pubkey(), symbol);
    let (user_pda, _) = crate::perpetual_mechanics::derive_user_pda(program_id, &trader.pubkey());
    
    // Get current market price (mock for now)
    let market_price = match symbol {
        "SOL-PERP" => 180_000000u64, // $180
        _ => return Err(anyhow::anyhow!("Unsupported symbol")),
    };
    
    let margin_required = (size * market_price / 1000000) / leverage as u64;
    
    let trade_ix = Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(trader.pubkey(), true),
            AccountMeta::new(position_pda, false),
            AccountMeta::new(user_pda, false),
            AccountMeta::new_readonly(market_pda, false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data: create_trade_data(size, leverage, is_long, market_price),
    };
    
    let recent_blockhash = client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[trade_ix],
        Some(&trader.pubkey()),
        &[trader],
        recent_blockhash,
    );
    
    let signature = client.send_and_confirm_transaction(&tx)?;
    
    println!("🚀 Trade executed:");
    println!("  Symbol: {}", symbol);
    println!("  Size: {} qty", size as f64 / 1000000.0);
    println!("  Leverage: {}x", leverage);
    println!("  Side: {}", if is_long { "LONG" } else { "SHORT" });
    println!("  Price: ${}", market_price as f64 / 1000000.0);
    println!("  Margin: ${}", margin_required as f64 / 1000000.0);
    println!("  Signature: {}", signature);
    
    Ok(signature.to_string())
}

fn create_trade_data(size: u64, leverage: u8, is_long: bool, price: u64) -> Vec<u8> {
    let mut data = Vec::new();
    data.push(0); // Open position instruction
    data.extend_from_slice(&size.to_le_bytes());
    data.push(leverage);
    data.push(if is_long { 1 } else { 0 });
    data.extend_from_slice(&price.to_le_bytes());
    data
}

pub async fn sol_long_20x(client: &RpcClient, trader: &Keypair, program_id: &Pubkey) -> Result<String> {
    execute_market_trade(client, trader, program_id, "SOL-PERP", 1000000, 20, true).await
}