use trading_system::solana_trade::execute_market_trade;
use trading_system::advanced_orders::{OrderManager, AdvancedOrder, OrderType};
use position_service::db::repo::PgRepo;
use position_service::models::{OpenPositionInput, Side};
use solana_sdk::signature::{Keypair, Signer};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use chrono::Utc;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Database connection
    let db_url = "postgresql://trader:trading123@localhost:5432/trading_system";
    let repo = PgRepo::connect(db_url).await?;
    
    let client = RpcClient::new("https://api.devnet.solana.com");
    let trader = Keypair::new();
    let program_id = Pubkey::new_unique();
    
    // Trade parameters
    let qty = 0.1;
    let leverage = 15u8;
    let target_price = 130.0; // $130 target
    let stop_loss_price = 150.0; // $150 stop-loss
    let size_in_lamports = (qty * 1_000_000.0) as u64; // Convert to 6 decimals
    
    println!("🔻 Opening SHORT position on SOL");
    println!("Quantity: {} SOL", qty);
    println!("Leverage: {}x", leverage);
    println!("Target: ${}", target_price);
    println!("Stop-loss: ${}", stop_loss_price);
    println!("Trader: {}", trader.pubkey());
    
    // Store trade intent in database
    let position_pda = Pubkey::new_unique();
    let position_input = OpenPositionInput {
        symbol: "SOL-PERP".to_string(),
        side: Side::Short,
        size: size_in_lamports,
        entry_price: 180_000000, // $180 in 6 decimals
        leverage: leverage as u16,
        margin_token_account: Pubkey::new_unique(),
        quote_mint: Pubkey::new_unique(),
    };
    
    repo.insert_position_open_intent(&trader.pubkey(), &position_pda, &position_input).await?;
    println!("💾 Trade stored in database");
    
    // Execute the short trade
    match execute_market_trade(
        &client,
        &trader,
        &program_id,
        "SOL-PERP",
        size_in_lamports,
        leverage,
        false, // false = SHORT position
    ).await {
        Ok(signature) => {
            println!("✅ SHORT position opened: {}", signature);
            
            // Set up advanced orders
            let mut order_manager = OrderManager::new();
            
            // Take-profit order at $130
            let take_profit = AdvancedOrder {
                id: format!("tp_{}", signature),
                owner: trader.pubkey(),
                symbol: "SOL-PERP".to_string(),
                order_type: OrderType::TakeProfit { 
                    target_price: (target_price * 1_000_000.0) as u64 
                },
                size: size_in_lamports,
                created_at: Utc::now(),
                is_active: true,
            };
            
            // Stop-loss order at $150
            let stop_loss = AdvancedOrder {
                id: format!("sl_{}", signature),
                owner: trader.pubkey(),
                symbol: "SOL-PERP".to_string(),
                order_type: OrderType::StopLoss { 
                    trigger_price: (stop_loss_price * 1_000_000.0) as u64 
                },
                size: size_in_lamports,
                created_at: Utc::now(),
                is_active: true,
            };
            
            order_manager.add_order(take_profit);
            order_manager.add_order(stop_loss);
            
            println!("📋 Advanced orders set:");
            println!("  Take-profit: ${} (close SHORT when price drops)", target_price);
            println!("  Stop-loss: ${} (close SHORT when price rises)", stop_loss_price);
            
            // Simulate price monitoring
            println!("\n🔍 Monitoring price movements...");
            let test_prices = [145.0, 140.0, 135.0, 130.0]; // Simulated price drops
            
            for price in test_prices {
                let price_lamports = (price * 1_000_000.0) as u64;
                let triggered = order_manager.check_triggers("SOL-PERP", price_lamports);
                
                if !triggered.is_empty() {
                    for order in triggered {
                        match order.order_type {
                            OrderType::TakeProfit { .. } => {
                                println!("🎯 Take-profit triggered at ${} - Position closed with PROFIT!", price);
                            },
                            OrderType::StopLoss { .. } => {
                                println!("🛑 Stop-loss triggered at ${} - Position closed to limit LOSS!", price);
                            },
                            _ => {}
                        }
                    }
                    break;
                }
                println!("  Price: ${} - Orders still active", price);
            }
        },
        Err(e) => println!("❌ SHORT trade failed: {}", e),
    }
    
    Ok(())
}