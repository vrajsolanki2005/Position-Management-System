use trading_system::solana_trade::sol_long_20x;
use solana_sdk::signature::Keypair;
use solana_client::rpc_client::RpcClient;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = RpcClient::new("https://api.devnet.solana.com");
    let trader = Keypair::new();
    let program_id = solana_sdk::pubkey::Pubkey::new_unique();
    
    println!("💰 Buying 1 SOL at market price with 20x leverage");
    println!("Price: $180 | Margin required: $9");
    
    let signature = sol_long_20x(&client, &trader, &program_id).await?;
    println!("✅ 1 SOL purchased: {}", signature);
    
    Ok(())
}