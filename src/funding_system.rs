use anyhow::Result;
use chrono::{DateTime, Utc};
use super::{FundingUpdate, FundingSystem};

pub struct DefaultFundingSystem {
    symbols: Vec<String>,
    base_rate: f64,
}

impl DefaultFundingSystem {
    pub fn new(symbols: Vec<String>, base_rate: f64) -> Self {
        Self { symbols, base_rate }
    }
}

#[async_trait::async_trait]
impl FundingSystem for DefaultFundingSystem {
    async fn compute_and_publish(&self) -> Result<Vec<FundingUpdate>> {
        let now = Utc::now();
        let mut updates = Vec::new();
        
        for symbol in &self.symbols {
            // Simplified funding rate calculation
            let rate_per_hour = self.base_rate * (1.0 + rand::random::<f64>() * 0.1 - 0.05);
            let cum_funding_per_base = rate_per_hour * 24.0; // Daily cumulative
            
            updates.push(FundingUpdate {
                symbol: symbol.clone(),
                rate_per_hour,
                cum_funding_per_base,
                ts: now,
            });
        }
        
        Ok(updates)
    }

    async fn apply_on_chain(&self, updates: &[FundingUpdate]) -> Result<Vec<String>> {
        let mut tx_sigs = Vec::new();
        
        for update in updates {
            let tx_sig = format!("funding_{}_{}", update.symbol, uuid::Uuid::new_v4().to_string()[..8].to_string());
            tx_sigs.push(tx_sig);
        }
        
        Ok(tx_sigs)
    }
}