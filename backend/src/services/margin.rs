use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub const SCALE: i128 = 1_000_000;

pub struct LeverageTier {
    pub max_leverage: u16,
    pub initial_margin_rate: i64,    // scaled by 1e6
    pub maintenance_margin_rate: i64,// scaled by 1e6
    pub max_position_size: i128,     // scaled by 1e6 (quote units)
}

#[derive(Debug, Clone)]
pub struct MarginCalculator {
    pub tiers: Vec<LeverageTier>,
}

impl Default for MarginCalculator {
    fn default() -> Self {
        Self {
            tiers: vec![
                LeverageTier { max_leverage: 20,  initial_margin_rate: 0.05,  maintenance_margin_rate: 0.025, max_position_size: f64::INFINITY },
                LeverageTier { max_leverage: 50,  initial_margin_rate: 0.02,  maintenance_margin_rate: 0.01,  max_position_size: 100_000.0 },
                LeverageTier { max_leverage: 100, initial_margin_rate: 0.01,  maintenance_margin_rate: 0.005, max_position_size: 50_000.0 },
                LeverageTier { max_leverage: 500, initial_margin_rate: 0.005, maintenance_margin_rate: 0.0025, max_position_size: 20_000.0 },
                LeverageTier { max_leverage: 1000,initial_margin_rate: 0.002, maintenance_margin_rate: 0.001, max_position_size: 5_000.0 },
            ]
        }
    }
}

impl MarginCalculator {
    pub fn tier_for(&self, leverage: u16, position_size_quote: f64) -> Option<LeverageTier> {
        self.tiers.iter().copied().find(|t| leverage <= t.max_leverage && position_size_quote <= t.max_position_size)
    }

    pub fn initial_margin(&self, notional: f64, leverage: f64) -> f64 {
        notional / leverage
    }

    pub fn maintenance_margin(&self, notional: f64, maintenance_rate: f64) -> f64 {
        notional * maintenance_rate
    }

    pub fn margin_ratio(&self, collateral: f64, unrealized_pnl: f64, size: f64, mark_price: f64) -> f64 {
        let position_value = size * mark_price;
        if position_value <= 0.0 { return f64::INFINITY; }
        (collateral + unrealized_pnl) / position_value
    }

    pub fn liquidation_price_long(&self, entry_price: f64, leverage: f64, mmr: f64) -> f64 {
        entry_price * (1.0 - 1.0 / leverage + mmr)
    }

    pub fn liquidation_price_short(&self, entry_price: f64, leverage: f64, mmr: f64) -> f64 {
        entry_price * (1.0 + 1.0 / leverage - mmr)
    }
}