pub mod models;
pub mod liquidation_engine;
pub mod settlement_relayer;
pub mod funding_system;
pub mod risk_manager;
pub mod performance_optimizer;
pub mod advanced_orders;
pub mod analytics;
pub mod state_manager;
pub mod trading_engine;
pub mod perpetual_mechanics;
pub mod solana_markets;
pub mod solana_trade;

use anyhow::Result;
use chrono::{DateTime, Utc};
use models::PositionView;

#[derive(Debug, Clone)]
pub struct LiqOrder {
    pub pda: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub symbol: String,
    pub close_base: u64,
}

#[async_trait::async_trait]
pub trait LiquidationEngine: Send + Sync {
    async fn evaluate(&self, positions: &[PositionView], mark_prices: &[(String, f64)]) -> Result<Vec<LiqOrder>>;
    async fn execute(&self, order: &LiqOrder) -> Result<String>;
}

#[async_trait::async_trait]
pub trait SettlementRelayer: Send + Sync {
    async fn close_position(&self, owner: solana_sdk::pubkey::Pubkey, symbol: &str, exit_price: u64, funding_payment: i64) -> Result<String>;
    async fn modify_position(&self, owner: solana_sdk::pubkey::Pubkey, symbol: &str, action: models::ModifyAction) -> Result<String>;
    async fn liquidate_position(&self, owner: solana_sdk::pubkey::Pubkey, symbol: &str, close_base: u64, mark_price: u64) -> Result<String>;
}

#[derive(Debug, Clone)]
pub struct FundingUpdate {
    pub symbol: String,
    pub rate_per_hour: f64,
    pub cum_funding_per_base: f64,
    pub ts: DateTime<Utc>,
}

#[async_trait::async_trait]
pub trait FundingSystem: Send + Sync {
    async fn compute_and_publish(&self) -> Result<Vec<FundingUpdate>>;
    async fn apply_on_chain(&self, updates: &[FundingUpdate]) -> Result<Vec<String>>;
}