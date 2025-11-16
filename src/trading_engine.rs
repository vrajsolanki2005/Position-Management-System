use crate::{
    risk_manager::{RiskManager, UserTier},
    performance_optimizer::{PositionCache, BatchProcessor},
    advanced_orders::{OrderManager, AdvancedOrder},
    analytics::{Analytics, TradeRecord},
    state_manager::StateManager,
    models::PositionView,
};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

pub struct TradingEngine {
    risk_manager: RiskManager,
    position_cache: PositionCache,
    batch_processor: BatchProcessor,
    order_manager: Arc<RwLock<OrderManager>>,
    analytics: Arc<RwLock<Analytics>>,
    state_manager: Arc<RwLock<StateManager>>,
    user_tiers: HashMap<String, UserTier>,
}

impl TradingEngine {
    pub fn new() -> Self {
        Self {
            risk_manager: RiskManager::new(),
            position_cache: PositionCache::new(),
            batch_processor: BatchProcessor::new(100),
            order_manager: Arc::new(RwLock::new(OrderManager::new())),
            analytics: Arc::new(RwLock::new(Analytics::new())),
            state_manager: Arc::new(RwLock::new(StateManager::new())),
            user_tiers: HashMap::new(),
        }
    }

    pub async fn process_market_update(&self, symbol: &str, price: u64, volatility: f64) {
        // Check order triggers
        let triggered_orders = {
            let mut order_manager = self.order_manager.write().await;
            order_manager.check_triggers(symbol, price)
        };

        // Execute triggered orders
        for order in triggered_orders {
            self.execute_order(order).await;
        }

        // Update dynamic leverage limits
        let base_leverage = 20.0;
        let _dynamic_leverage = self.risk_manager.calculate_dynamic_leverage(symbol, volatility, base_leverage);
    }

    pub async fn validate_new_position(&self, position: &PositionView, user_id: &str, mark_price: u64) -> bool {
        let tier = self.user_tiers.get(user_id).unwrap_or(&UserTier::Basic);
        self.risk_manager.validate_position(position, tier, mark_price)
    }

    pub async fn add_advanced_order(&self, order: AdvancedOrder) {
        let mut order_manager = self.order_manager.write().await;
        order_manager.add_order(order);
    }

    pub async fn record_trade(&self, trade: TradeRecord) {
        let mut analytics = self.analytics.write().await;
        analytics.add_trade(trade);
    }

    pub async fn create_snapshot(&self, positions: Vec<PositionView>) -> String {
        let mut state_manager = self.state_manager.write().await;
        state_manager.create_snapshot(positions)
    }

    pub async fn get_performance_metrics(&self) -> crate::analytics::PerformanceMetrics {
        let analytics = self.analytics.read().await;
        analytics.calculate_metrics()
    }

    pub async fn batch_process_positions<F, Fut>(&self, positions: Vec<PositionView>, processor: F)
    where
        F: Fn(Vec<PositionView>) -> Fut + Send + Sync,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        self.batch_processor.process_positions(positions, processor).await;
    }

    async fn execute_order(&self, _order: AdvancedOrder) {
        // Order execution logic would go here
    }

    pub fn set_user_tier(&mut self, user_id: String, tier: UserTier) {
        self.user_tiers.insert(user_id, tier);
    }
}