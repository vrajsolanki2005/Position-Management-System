use crate::models::PositionView;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TradeRecord {
    pub symbol: String,
    pub entry_price: u64,
    pub exit_price: u64,
    pub size: u64,
    pub pnl: i64,
    pub entry_time: DateTime<Utc>,
    pub exit_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub total_pnl: i64,
    pub win_rate: f64,
    pub profit_factor: f64,
    pub sharpe_ratio: f64,
    pub max_drawdown: f64,
    pub total_trades: usize,
}

#[derive(Debug, Clone)]
pub struct PortfolioRisk {
    pub total_exposure: u64,
    pub var_95: f64,
    pub concentration_risk: f64,
}

pub struct Analytics {
    trades: Vec<TradeRecord>,
    daily_returns: Vec<f64>,
}

impl Analytics {
    pub fn new() -> Self {
        Self {
            trades: Vec::new(),
            daily_returns: Vec::new(),
        }
    }

    pub fn add_trade(&mut self, trade: TradeRecord) {
        self.trades.push(trade);
    }

    pub fn calculate_metrics(&self) -> PerformanceMetrics {
        let total_pnl: i64 = self.trades.iter().map(|t| t.pnl).sum();
        let winning_trades = self.trades.iter().filter(|t| t.pnl > 0).count();
        let win_rate = winning_trades as f64 / self.trades.len() as f64;
        
        let gross_profit: i64 = self.trades.iter().filter(|t| t.pnl > 0).map(|t| t.pnl).sum();
        let gross_loss: i64 = self.trades.iter().filter(|t| t.pnl < 0).map(|t| t.pnl.abs()).sum();
        let profit_factor = if gross_loss > 0 { gross_profit as f64 / gross_loss as f64 } else { 0.0 };
        
        let sharpe_ratio = self.calculate_sharpe_ratio();
        let max_drawdown = self.calculate_max_drawdown();

        PerformanceMetrics {
            total_pnl,
            win_rate,
            profit_factor,
            sharpe_ratio,
            max_drawdown,
            total_trades: self.trades.len(),
        }
    }

    pub fn calculate_portfolio_risk(&self, positions: &[PositionView], mark_prices: &HashMap<String, u64>) -> PortfolioRisk {
        let total_exposure: u64 = positions.iter()
            .map(|p| p.size * mark_prices.get(&p.symbol).unwrap_or(&0))
            .sum();

        let var_95 = self.calculate_var(0.95, positions, mark_prices);
        let concentration_risk = self.calculate_concentration_risk(positions, mark_prices);

        PortfolioRisk {
            total_exposure,
            var_95,
            concentration_risk,
        }
    }

    fn calculate_sharpe_ratio(&self) -> f64 {
        if self.daily_returns.is_empty() {
            return 0.0;
        }
        
        let mean_return: f64 = self.daily_returns.iter().sum::<f64>() / self.daily_returns.len() as f64;
        let variance: f64 = self.daily_returns.iter()
            .map(|r| (r - mean_return).powi(2))
            .sum::<f64>() / self.daily_returns.len() as f64;
        let std_dev = variance.sqrt();
        
        if std_dev > 0.0 { mean_return / std_dev } else { 0.0 }
    }

    fn calculate_max_drawdown(&self) -> f64 {
        let mut peak = 0i64;
        let mut max_dd = 0.0;
        let mut running_pnl = 0i64;

        for trade in &self.trades {
            running_pnl += trade.pnl;
            if running_pnl > peak {
                peak = running_pnl;
            }
            let drawdown = (peak - running_pnl) as f64 / peak.max(1) as f64;
            if drawdown > max_dd {
                max_dd = drawdown;
            }
        }

        max_dd
    }

    fn calculate_var(&self, confidence: f64, _positions: &[PositionView], _mark_prices: &HashMap<String, u64>) -> f64 {
        if self.daily_returns.is_empty() {
            return 0.0;
        }
        
        let mut sorted_returns = self.daily_returns.clone();
        sorted_returns.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let index = ((1.0 - confidence) * sorted_returns.len() as f64) as usize;
        sorted_returns.get(index).copied().unwrap_or(0.0).abs()
    }

    fn calculate_concentration_risk(&self, positions: &[PositionView], mark_prices: &HashMap<String, u64>) -> f64 {
        let mut symbol_exposure: HashMap<String, u64> = HashMap::new();
        let mut total_exposure = 0u64;

        for position in positions {
            let exposure = position.size * mark_prices.get(&position.symbol).unwrap_or(&0);
            *symbol_exposure.entry(position.symbol.clone()).or_insert(0) += exposure;
            total_exposure += exposure;
        }

        if total_exposure == 0 {
            return 0.0;
        }

        symbol_exposure.values()
            .map(|&exposure| {
                let weight = exposure as f64 / total_exposure as f64;
                weight * weight
            })
            .sum::<f64>()
            .sqrt()
    }

    pub fn add_daily_return(&mut self, return_pct: f64) {
        self.daily_returns.push(return_pct);
    }
}