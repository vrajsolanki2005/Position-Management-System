use trading_system::{
    trading_engine::TradingEngine,
    risk_manager::UserTier,
    advanced_orders::{AdvancedOrder, OrderType},
    analytics::TradeRecord,
    models::PositionView,
};
use solana_sdk::pubkey::Pubkey;
use chrono::Utc;

#[tokio::main]
async fn main() {
    let mut engine = TradingEngine::new();
    
    // Set user tier
    engine.set_user_tier("user1".to_string(), UserTier::Premium);
    
    // Create sample position
    let position = PositionView {
        pda: Pubkey::new_unique(),
        owner: Pubkey::new_unique(),
        symbol: "BTC-USD".to_string(),
        size: 1000,
        collateral: 5000,
        entry_price: 50000,
    };
    
    // Validate position
    let is_valid = engine.validate_new_position(&position, "user1", 51000).await;
    println!("Position valid: {}", is_valid);
    
    // Add stop-loss order
    let stop_loss = AdvancedOrder {
        id: "sl1".to_string(),
        owner: position.owner,
        symbol: position.symbol.clone(),
        order_type: OrderType::StopLoss { trigger_price: 48000 },
        size: position.size,
        created_at: Utc::now(),
        is_active: true,
    };
    
    engine.add_advanced_order(stop_loss).await;
    
    // Record a trade
    let trade = TradeRecord {
        symbol: "BTC-USD".to_string(),
        entry_price: 50000,
        exit_price: 52000,
        size: 1000,
        pnl: 2000,
        entry_time: Utc::now(),
        exit_time: Utc::now(),
    };
    
    engine.record_trade(trade).await;
    
    // Create snapshot
    let snapshot_id = engine.create_snapshot(vec![position]).await;
    println!("Created snapshot: {}", snapshot_id);
    
    // Process market update
    engine.process_market_update("BTC-USD", 47000, 0.15).await;
    
    // Get performance metrics
    let metrics = engine.get_performance_metrics().await;
    println!("Performance: PnL={}, Win Rate={:.2}%, Sharpe={:.2}", 
             metrics.total_pnl, metrics.win_rate * 100.0, metrics.sharpe_ratio);
}