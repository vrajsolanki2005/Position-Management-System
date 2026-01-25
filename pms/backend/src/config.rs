use anyhow::{anyhow, Result};
use serde::Deserialize;
use solana_sdk::pubkey::Pubkey;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub rpc_url: String,
    pub ws_url: String,
    pub keypair_path: String,
    pub program_id: Pubkey,
    pub http_addr: String,             // "0.0.0.0:8080"
    pub database_url: String,          // "postgres://postgres:postgres@localhost:5432/postgres"
    pub price_oracle_source: String,   // e.g., "pyth:BTC/USD" or "mock"
    pub risk_alert_threshold: f64,     // e.g., 0.15 (15% MR)
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let program_id = std::env::var("PROGRAM_ID")?.parse::<Pubkey>()?;
        Ok(Self {
            rpc_url: std::env::var("RPC_URL")?,
            ws_url: std::env::var("WS_URL")?,
            keypair_path: std::env::var("KEYPAIR_PATH")?,
            program_id,
            http_addr: std::env::var("HTTP_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string()),
            database_url: std::env::var("DATABASE_URL")?,
            price_oracle_source: std::env::var("PRICE_ORACLE_SOURCE").unwrap_or_else(|_| "mock".to_string()),
            risk_alert_threshold: std::env::var("RISK_ALERT_THRESHOLD").ok()
                .and_then(|s| s.parse().ok()).unwrap_or(0.20),
        })
    }
}