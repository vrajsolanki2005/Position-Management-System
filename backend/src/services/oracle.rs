use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait PriceOracle: Send + Sync {
    async fn price(&self, symbol: &str) -> Result<f64>;
}

// Simple mock oracle for local dev
#[derive(Clone, Default)]
pub struct MockOracle;

#[async_trait]
impl PriceOracle for MockOracle {
    async fn price(&self, _symbol: &str) -> Result<f64> {
        Ok(30_000.0)
    }
}