use crate::models::PositionView;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RiskLimits {
    pub max_leverage: f64,
    pub max_position_size: u64,
    pub max_open_interest: u64,
    pub max_margin_utilization: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserTier {
    Basic,
    Premium,
    Pro,
}

pub struct RiskManager {
    tier_limits: HashMap<UserTier, RiskLimits>,
    symbol_open_interest: HashMap<String, u64>,
}

impl RiskManager {
    pub fn new() -> Self {
        let mut tier_limits = HashMap::new();
        
        tier_limits.insert(UserTier::Basic, RiskLimits {
            max_leverage: 10.0,
            max_position_size: 1000,
            max_open_interest: 10000,
            max_margin_utilization: 0.8,
        });
        
        tier_limits.insert(UserTier::Premium, RiskLimits {
            max_leverage: 25.0,
            max_position_size: 5000,
            max_open_interest: 50000,
            max_margin_utilization: 0.9,
        });
        
        tier_limits.insert(UserTier::Pro, RiskLimits {
            max_leverage: 50.0,
            max_position_size: 20000,
            max_open_interest: 200000,
            max_margin_utilization: 0.95,
        });

        Self {
            tier_limits,
            symbol_open_interest: HashMap::new(),
        }
    }

    pub fn calculate_dynamic_leverage(&self, symbol: &str, volatility: f64, base_leverage: f64) -> f64 {
        let volatility_factor = 1.0 - (volatility * 2.0).min(0.5);
        base_leverage * volatility_factor
    }

    pub fn validate_position(&self, position: &PositionView, tier: &UserTier, mark_price: u64) -> bool {
        let limits = &self.tier_limits[tier];
        let leverage = (position.size as f64 * mark_price as f64) / position.collateral as f64;
        
        leverage <= limits.max_leverage && 
        position.size <= limits.max_position_size &&
        self.symbol_open_interest.get(&position.symbol).unwrap_or(&0) <= &limits.max_open_interest
    }

    pub fn update_open_interest(&mut self, symbol: String, delta: i64) {
        let current = self.symbol_open_interest.get(&symbol).unwrap_or(&0);
        self.symbol_open_interest.insert(symbol, (*current as i64 + delta).max(0) as u64);
    }
}