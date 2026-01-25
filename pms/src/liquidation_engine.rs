use anyhow::Result;
use crate::models::PositionView;
use super::{LiqOrder, LiquidationEngine};

pub struct DefaultLiquidationEngine {
    liquidation_threshold: f64, // e.g., 0.8 for 80% margin ratio
}

impl DefaultLiquidationEngine {
    pub fn new(liquidation_threshold: f64) -> Self {
        Self { liquidation_threshold }
    }
}

#[async_trait::async_trait]
impl LiquidationEngine for DefaultLiquidationEngine {
    async fn evaluate(&self, positions: &[PositionView], mark_prices: &[(String, f64)]) -> Result<Vec<LiqOrder>> {
        let price_map: std::collections::HashMap<String, f64> = mark_prices.iter().cloned().collect();
        
        let mut liq_orders = Vec::new();
        
        for position in positions {
            if let Some(&mark_price) = price_map.get(&position.symbol) {
                let margin_ratio = position.collateral as f64 / (position.size as f64 * mark_price);
                
                if margin_ratio <= self.liquidation_threshold {
                    liq_orders.push(LiqOrder {
                        pda: position.pda,
                        owner: position.owner,
                        symbol: position.symbol.clone(),
                        close_base: position.size,
                    });
                }
            }
        }
        
        Ok(liq_orders)
    }

    async fn execute(&self, order: &LiqOrder) -> Result<String> {
        // Simulate transaction execution
        let tx_sig = format!("liq_tx_{}", uuid::Uuid::new_v4().to_string()[..8].to_string());
        Ok(tx_sig)
    }
}