mod config;
mod errors;
mod models;
mod solana;
mod services;
mod db;
mod api;

use anyhow::Result;
use tracing_subscriber::{fmt, EnvFilter};
use services::{manager::PositionManager, monitor::PositionMonitor};
use api::http::start_http_server;
use db::repo::PgRepo;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let cfg = config::Config::from_env()?;
    let repo = PgRepo::connect(&cfg.database_url).await?;
    repo.migrate().await?;

    let sol1 = solana::client::SolanaCtx::new(&cfg).await?;
    let sol2 = solana::client::SolanaCtx::new(&cfg).await?;
    let margin_calc = services::margin::MarginCalculator::default();
    let pnl_tracker = services::pnl::PnLTracker::default();

    let manager = PositionManager::new(sol1, repo.clone(), margin_calc.clone(), pnl_tracker.clone(), cfg.program_id);
    let monitor = PositionMonitor::new(sol2, repo.clone(), cfg.price_oracle_source.clone(), cfg.risk_alert_threshold);

    // Start background monitor
    tokio::spawn(async move {
        if let Err(e) = monitor.run().await {
            tracing::error!("monitor stopped: {:?}", e);
        }
    });

    // Start API (HTTP + WS)
    start_http_server(cfg.http_addr.clone(), manager, repo.clone()).await?;

    Ok(())
}