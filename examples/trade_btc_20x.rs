use trading_system::solana_trade::sol_long_20x;
use solana_sdk::signature::Keypair;
use solana_client::rpc_client::RpcClient;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = RpcClient::new("https://api.devnet.solana.com");
    let trader = Keypair::new();
    let program_id = solana_sdk::pubkey::Pubkey::new_unique();
    
    println!("📈 Buying 1 SOL with 20x Leverage");
    println!("Trader: {}", trader.pubkey());
    
    match sol_long_20x(&client, &trader, &program_id).await {
        Ok(signature) => println!("✅ SOL trade successful: {}", signature),
        Err(e) => println!("❌ SOL trade failed: {}", e),
    }
    
    Ok(())
}