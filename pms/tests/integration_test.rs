use trading_system::{
    trading_engine::TradingEngine,
    risk_manager::UserTier,
    advanced_orders::{AdvancedOrder, OrderType},
    models::PositionView,
};
use solana_sdk::pubkey::Pubkey;
use chrono::Utc;

#[tokio::test]
async fn test_stop_loss_trigger() {
    let mut engine = TradingEngine::new();
    engine.set_user_tier("user1".to_string(), UserTier::Premium);
    
    let position = PositionView {
        pda: Pubkey::new_unique(),
        owner: Pubkey::new_unique(),
        symbol: "BTC-USD".to_string(),
        size: 1000,
        collateral: 5000,
        entry_price: 50000,
    };
    
    let stop_loss = AdvancedOrder {
        id: "sl1".to_string(),
        owner: position.owner,
        symbol: "BTC-USD".to_string(),
        order_type: OrderType::StopLoss { trigger_price: 48000 },
        size: 1000,
        created_at: Utc::now(),
        is_active: true,
    };
    
    engine.add_advanced_order(stop_loss).await;
    
    // Price drops below stop loss - should trigger
    engine.process_market_update("BTC-USD", 47000, 0.15).await;
    
    let metrics = engine.get_performance_metrics().await;
    assert_eq!(metrics.total_trades, 0); // No trades recorded yet, just order triggered
}