use std::time::Duration;
use anyhow::Result;
use tracing::info;

use crate::{db::repo::PgRepo, solana::client::SolanaCtx};
use super::oracle::{PriceOracle, MockOracle};
use crate::models::{PositionView, PositionState};

#[derive(Clone)]
pub struct PositionMonitor {
    sol: std::sync::Arc<SolanaCtx>,
    repo: PgRepo,
    oracle_src: String,
    alert_threshold: f64,
}

impl PositionMonitor {
    pub fn new(sol: SolanaCtx, repo: PgRepo, oracle_src: String, alert_threshold: f64) -> Self {
        Self { sol: std::sync::Arc::new(sol), repo, oracle_src, alert_threshold }
    }

    pub async fn run(self) -> Result<()> {
        let oracle = MockOracle::default(); // replace with Pyth impl
        loop {
            self.scan(&oracle).await?;
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }

    async fn scan(&self, oracle: &impl PriceOracle) -> Result<()> {
        let positions = self.repo.fetch_open_positions().await?;
        for p in positions {
            if let Err(e) = self.check_position(&p, oracle).await {
                tracing::warn!("check_position error: {}", e);
            }
        }
        Ok(())
    }

    async fn check_position(&self, p: &PositionView, oracle: &impl PriceOracle) -> Result<()> {
        let price = oracle.price(&p.symbol).await?;
        let size = p.size as f64;
        let entry = p.entry_price as f64;
        let collateral = p.margin as f64;

        let upnl = if matches!(p.side, crate::models::Side::Long) {
            size * (price - entry)
        } else {
            size * (entry - price)
        };
        let mr = (collateral + upnl) / (size * price).max(1.0);

        if mr < self.alert_threshold {
            info!("ALERT: {:?} {} MR={:.4} at price {:.2}", p.owner, p.symbol, mr, price);
            self.repo.insert_liq_alert(p, mr, price).await?;
        }
        self.repo.upsert_position_snapshot(p, price, upnl as i64, mr).await?;
        Ok(())
    }
}