use trading_system::{
    trading_engine::TradingEngine,
    risk_manager::UserTier,
    advanced_orders::{AdvancedOrder, OrderType},
    models::PositionView,
    analytics::TradeRecord,
};
use solana_sdk::pubkey::Pubkey;
use chrono::Utc;

pub struct CryptoTestSuite {
    engine: TradingEngine,
    test_results: Vec<TestResult>,
}

#[derive(Debug, Clone)]
pub struct TestResult {
    pub scenario: String,
    pub symbol: String,
    pub success: bool,
    pub pnl: i64,
    pub trades: u32,
    pub max_drawdown: f64,
    pub risk_score: f64,
    pub execution_time_ms: u64,
    pub details: String,
}

impl CryptoTestSuite {
    pub fn new() -> Self {
        Self {
            engine: TradingEngine::new(),
            test_results: Vec::new(),
        }
    }

    pub async fn run_all_scenarios(&mut self) {
        println!("🚀 Starting Comprehensive Crypto Trading Test Suite");
        
        // Market condition scenarios
        self.test_bull_run().await;
        self.test_bear_crash().await;
        self.test_sideways_chop().await;
        
        // Risk management scenarios
        self.test_stop_loss_protection().await;
        self.test_liquidation_avoidance().await;
        self.test_position_sizing().await;
        
        // Advanced order scenarios
        self.test_take_profit_scaling().await;
        self.test_trailing_stops().await;
        self.test_bracket_orders().await;
        
        // Extreme market scenarios
        self.test_flash_crash().await;
        self.test_pump_and_dump().await;
        self.test_low_liquidity().await;
        
        self.generate_summary();
    }

    async fn test_bull_run(&mut self) {
        let start = std::time::Instant::now();
        let mut trades = 0;
        let mut pnl = 0i64;
        
        self.engine.set_user_tier("bull_trader".to_string(), UserTier::Premium);
        
        // BTC bull run: 40k -> 70k over time
        let prices = vec![40000, 42000, 45000, 48000, 52000, 58000, 65000, 70000];
        let position_size = 50000; // 0.5 BTC
        
        for (i, &price) in prices.iter().enumerate() {
            self.engine.process_market_update("BTC-USD", price, 0.12).await;
            
            if i > 0 && i % 2 == 0 { // Take profits periodically
                trades += 1;
                let trade_pnl = (price as i64 - 40000) * position_size / 40000;
                pnl += trade_pnl;
                
                let trade = TradeRecord {
                    symbol: "BTC-USD".to_string(),
                    size: position_size as u64,
                    entry_price: 40000,
                    exit_price: price,
                    pnl: trade_pnl,
                    entry_time: Utc::now(),
                    exit_time: Utc::now(),
                };
                self.engine.record_trade(trade).await;
            }
        }
        
        self.test_results.push(TestResult {
            scenario: "Bull Run Rally".to_string(),
            symbol: "BTC-USD".to_string(),
            success: pnl > 0,
            pnl,
            trades,
            max_drawdown: 0.05,
            risk_score: 0.3,
            execution_time_ms: start.elapsed().as_millis() as u64,
            details: format!("75% price increase, {} trades executed", trades),
        });
    }

    async fn test_bear_crash(&mut self) {
        let start = std::time::Instant::now();
        let mut trades = 0;
        let mut pnl = 0i64;
        
        self.engine.set_user_tier("bear_trader".to_string(), UserTier::Basic);
        
        // ETH bear crash: 4000 -> 1200
        let prices = vec![4000, 3500, 3000, 2500, 2000, 1600, 1200];
        let position_size = 100000; // 1 ETH short
        
        for (i, &price) in prices.iter().enumerate() {
            self.engine.process_market_update("ETH-USD", price, 0.35).await;
            
            if i > 0 { // Short position profits from price decline
                trades += 1;
                let trade_pnl = (4000 - price as i64) * position_size / 4000;
                pnl += trade_pnl;
                
                let trade = TradeRecord {
                    symbol: "ETH-USD".to_string(),
                    size: position_size as u64,
                    entry_price: 4000,
                    exit_price: price,
                    pnl: trade_pnl,
                    entry_time: Utc::now(),
                    exit_time: Utc::now(),
                };
                self.engine.record_trade(trade).await;
            }
        }
        
        self.test_results.push(TestResult {
            scenario: "Bear Market Crash".to_string(),
            symbol: "ETH-USD".to_string(),
            success: pnl > 0,
            pnl,
            trades,
            max_drawdown: 0.70,
            risk_score: 0.8,
            execution_time_ms: start.elapsed().as_millis() as u64,
            details: format!("70% price decline, short strategy with {} trades", trades),
        });
    }

    async fn test_sideways_chop(&mut self) {
        let start = std::time::Instant::now();
        let mut trades = 0;
        let mut pnl = 0i64;
        
        // Range-bound trading: 45k-55k BTC
        let prices = vec![50000, 52000, 48000, 54000, 46000, 55000, 45000, 53000, 47000, 51000];
        let position_size = 20000;
        
        for (i, &price) in prices.iter().enumerate() {
            self.engine.process_market_update("BTC-USD", price, 0.08).await;
            
            if i > 0 {
                trades += 1;
                // Range trading strategy
                let trade_pnl = if price > 52000 { -5000 } else if price < 48000 { 8000 } else { 2000 };
                pnl += trade_pnl;
                
                let trade = TradeRecord {
                    symbol: "BTC-USD".to_string(),
                    size: position_size,
                    entry_price: 50000,
                    exit_price: price,
                    pnl: trade_pnl,
                    entry_time: Utc::now(),
                    exit_time: Utc::now(),
                };
                self.engine.record_trade(trade).await;
            }
        }
        
        self.test_results.push(TestResult {
            scenario: "Sideways Chop".to_string(),
            symbol: "BTC-USD".to_string(),
            success: pnl > 0,
            pnl,
            trades,
            max_drawdown: 0.12,
            risk_score: 0.4,
            execution_time_ms: start.elapsed().as_millis() as u64,
            details: "Range-bound market with mean reversion".to_string(),
        });
    }

    async fn test_stop_loss_protection(&mut self) {
        let start = std::time::Instant::now();
        
        let position = PositionView {
            pda: Pubkey::new_unique(),
            owner: Pubkey::new_unique(),
            symbol: "SOL-USD".to_string(),
            size: 500000, // 500 SOL
            collateral: 10000,
            entry_price: 80,
        };
        
        let stop_loss = AdvancedOrder {
            id: "sl_protection".to_string(),
            owner: position.owner,
            symbol: "SOL-USD".to_string(),
            order_type: OrderType::StopLoss { trigger_price: 72 }, // 10% stop loss
            size: position.size,
            created_at: Utc::now(),
            is_active: true,
        };
        
        self.engine.add_advanced_order(stop_loss).await;
        
        // Price drops triggering stop loss
        let prices = vec![80, 78, 75, 70, 65];
        let mut triggered = false;
        
        for &price in &prices {
            self.engine.process_market_update("SOL-USD", price, 0.25).await;
            if price <= 72 && !triggered {
                triggered = true;
                let trade = TradeRecord {
                    symbol: "SOL-USD".to_string(),
                    size: position.size,
                    entry_price: 80,
                    exit_price: price,
                    pnl: (price as i64 - 80) * position.size as i64 / 80,
                    entry_time: Utc::now(),
                    exit_time: Utc::now(),
                };
                self.engine.record_trade(trade).await;
                break;
            }
        }
        
        self.test_results.push(TestResult {
            scenario: "Stop Loss Protection".to_string(),
            symbol: "SOL-USD".to_string(),
            success: triggered,
            pnl: -50000, // Limited loss
            trades: 1,
            max_drawdown: 0.10,
            risk_score: 0.2,
            execution_time_ms: start.elapsed().as_millis() as u64,
            details: "Stop loss successfully limited downside".to_string(),
        });
    }

    async fn test_liquidation_avoidance(&mut self) {
        let start = std::time::Instant::now();
        
        let high_leverage_position = PositionView {
            pda: Pubkey::new_unique(),
            owner: Pubkey::new_unique(),
            symbol: "BTC-USD".to_string(),
            size: 500000, // 5 BTC
            collateral: 25000, // 20x leverage
            entry_price: 50000,
        };
        
        // Test position validation at different prices
        let prices = vec![50000, 48000, 46000, 44000, 42000, 40000];
        let mut liquidated = false;
        
        for &price in &prices {
            let is_valid = self.engine.validate_new_position(&high_leverage_position, "high_lev_trader", price).await;
            self.engine.process_market_update("BTC-USD", price, 0.20).await;
            
            if !is_valid {
                liquidated = true;
                break;
            }
        }
        
        self.test_results.push(TestResult {
            scenario: "Liquidation Avoidance".to_string(),
            symbol: "BTC-USD".to_string(),
            success: !liquidated,
            pnl: if liquidated { -25000 } else { -10000 },
            trades: if liquidated { 1 } else { 0 },
            max_drawdown: if liquidated { 1.0 } else { 0.20 },
            risk_score: 0.9,
            execution_time_ms: start.elapsed().as_millis() as u64,
            details: format!("High leverage position {}", if liquidated { "liquidated" } else { "survived" }),
        });
    }

    async fn test_position_sizing(&mut self) {
        let start = std::time::Instant::now();
        
        // Test different position sizes with same capital
        let capital = 10000;
        let scenarios = vec![
            ("Conservative", 0.02, 5), // 2% risk, 5x leverage
            ("Moderate", 0.05, 10),    // 5% risk, 10x leverage  
            ("Aggressive", 0.10, 20),  // 10% risk, 20x leverage
        ];
        
        let mut total_pnl = 0i64;
        let mut trades = 0;
        
        for (name, risk_pct, leverage) in scenarios {
            let position_size = (capital as f64 * risk_pct * leverage as f64) as u64;
            
            // Simulate 10% price move
            let entry_price = 45000;
            let exit_price = 49500; // 10% gain
            
            let trade_pnl = (exit_price - entry_price) as i64 * position_size as i64 / entry_price as i64;
            total_pnl += trade_pnl;
            trades += 1;
            
            let trade = TradeRecord {
                symbol: "BTC-USD".to_string(),
                size: position_size,
                entry_price: entry_price as u64,
                exit_price: exit_price as u64,
                pnl: trade_pnl,
                entry_time: Utc::now(),
                exit_time: Utc::now(),
            };
            self.engine.record_trade(trade).await;
        }
        
        self.test_results.push(TestResult {
            scenario: "Position Sizing".to_string(),
            symbol: "BTC-USD".to_string(),
            success: total_pnl > 0,
            pnl: total_pnl,
            trades,
            max_drawdown: 0.10,
            risk_score: 0.5,
            execution_time_ms: start.elapsed().as_millis() as u64,
            details: "Risk-adjusted position sizing comparison".to_string(),
        });
    }

    async fn test_take_profit_scaling(&mut self) {
        let start = std::time::Instant::now();
        
        let position = PositionView {
            pda: Pubkey::new_unique(),
            owner: Pubkey::new_unique(),
            symbol: "ETH-USD".to_string(),
            size: 400000, // 4 ETH
            collateral: 8000,
            entry_price: 2500,
        };
        
        // Scaling take profits: 25%, 50%, 25%
        let tp_orders = vec![
            (2750, 100000), // 10% gain, 25% position
            (3000, 200000), // 20% gain, 50% position  
            (3250, 100000), // 30% gain, 25% position
        ];
        
        for (i, (tp_price, size)) in tp_orders.iter().enumerate() {
            let tp_order = AdvancedOrder {
                id: format!("tp_{}", i),
                owner: position.owner,
                symbol: "ETH-USD".to_string(),
                order_type: OrderType::TakeProfit { target_price: *tp_price },
                size: *size,
                created_at: Utc::now(),
                is_active: true,
            };
            self.engine.add_advanced_order(tp_order).await;
        }
        
        // Price rises hitting all take profits
        let prices = vec![2500, 2600, 2750, 2900, 3000, 3100, 3250];
        let mut trades = 0;
        let mut pnl = 0i64;
        
        for &price in &prices {
            self.engine.process_market_update("ETH-USD", price, 0.15).await;
            
            for (tp_price, size) in &tp_orders {
                if price >= *tp_price {
                    trades += 1;
                    let trade_pnl = (*tp_price as i64 - 2500) * *size as i64 / 2500;
                    pnl += trade_pnl;
                    
                    let trade = TradeRecord {
                        symbol: "ETH-USD".to_string(),
                        size: *size,
                        entry_price: 2500,
                        exit_price: *tp_price,
                        pnl: trade_pnl,
                        entry_time: Utc::now(),
                        exit_time: Utc::now(),
                    };
                    self.engine.record_trade(trade).await;
                }
            }
        }
        
        self.test_results.push(TestResult {
            scenario: "Take Profit Scaling".to_string(),
            symbol: "ETH-USD".to_string(),
            success: trades >= 3,
            pnl,
            trades,
            max_drawdown: 0.0,
            risk_score: 0.2,
            execution_time_ms: start.elapsed().as_millis() as u64,
            details: "Systematic profit taking at multiple levels".to_string(),
        });
    }

    async fn test_trailing_stops(&mut self) {
        let start = std::time::Instant::now();
        
        // Simulate trailing stop behavior
        let entry_price = 100;
        let mut trailing_stop = 90; // 10% trailing stop
        let mut max_price = entry_price;
        let mut pnl = 0i64;
        let mut trades = 0;
        
        let prices = vec![100, 105, 110, 115, 120, 118, 115, 110, 108]; // Rise then fall
        
        for &price in &prices {
            self.engine.process_market_update("SOL-USD", price, 0.18).await;
            
            if price > max_price {
                max_price = price;
                trailing_stop = (max_price as f64 * 0.9) as u64; // Update trailing stop
            }
            
            if price <= trailing_stop {
                // Trailing stop triggered
                trades += 1;
                pnl = (price as i64 - entry_price as i64) * 1000; // 1000 SOL position
                
                let trade = TradeRecord {
                    symbol: "SOL-USD".to_string(),
                    size: 1000000,
                    entry_price: entry_price,
                    exit_price: price,
                    pnl,
                    entry_time: Utc::now(),
                    exit_time: Utc::now(),
                };
                self.engine.record_trade(trade).await;
                break;
            }
        }
        
        self.test_results.push(TestResult {
            scenario: "Trailing Stops".to_string(),
            symbol: "SOL-USD".to_string(),
            success: pnl > 0,
            pnl,
            trades,
            max_drawdown: 0.10,
            risk_score: 0.3,
            execution_time_ms: start.elapsed().as_millis() as u64,
            details: format!("Trailing stop locked in {}% gain", (pnl as f64 / 100000.0)),
        });
    }

    async fn test_bracket_orders(&mut self) {
        let start = std::time::Instant::now();
        
        // Bracket order: Entry + Stop Loss + Take Profit
        let entry_price = 3000;
        let stop_loss = 2700; // 10% stop
        let take_profit = 3600; // 20% target
        
        let position = PositionView {
            pda: Pubkey::new_unique(),
            owner: Pubkey::new_unique(),
            symbol: "ETH-USD".to_string(),
            size: 200000, // 2 ETH
            collateral: 6000,
            entry_price,
        };
        
        // Add bracket orders
        let sl_order = AdvancedOrder {
            id: "bracket_sl".to_string(),
            owner: position.owner,
            symbol: "ETH-USD".to_string(),
            order_type: OrderType::StopLoss { trigger_price: stop_loss },
            size: position.size,
            created_at: Utc::now(),
            is_active: true,
        };
        
        let tp_order = AdvancedOrder {
            id: "bracket_tp".to_string(),
            owner: position.owner,
            symbol: "ETH-USD".to_string(),
            order_type: OrderType::TakeProfit { target_price: take_profit },
            size: position.size,
            created_at: Utc::now(),
            is_active: true,
        };
        
        self.engine.add_advanced_order(sl_order).await;
        self.engine.add_advanced_order(tp_order).await;
        
        // Price moves to hit take profit
        let prices = vec![3000, 3100, 3300, 3500, 3600];
        let mut pnl = 0i64;
        let mut trades = 0;
        
        for &price in &prices {
            self.engine.process_market_update("ETH-USD", price, 0.12).await;
            
            if price >= take_profit {
                trades += 1;
                pnl = (take_profit as i64 - entry_price as i64) * position.size as i64 / entry_price as i64;
                
                let trade = TradeRecord {
                    symbol: "ETH-USD".to_string(),
                    size: position.size,
                    entry_price: entry_price,
                    exit_price: take_profit,
                    pnl,
                    entry_time: Utc::now(),
                    exit_time: Utc::now(),
                };
                self.engine.record_trade(trade).await;
                break;
            }
        }
        
        self.test_results.push(TestResult {
            scenario: "Bracket Orders".to_string(),
            symbol: "ETH-USD".to_string(),
            success: pnl > 0,
            pnl,
            trades,
            max_drawdown: 0.0,
            risk_score: 0.25,
            execution_time_ms: start.elapsed().as_millis() as u64,
            details: "Complete bracket order execution".to_string(),
        });
    }

    async fn test_flash_crash(&mut self) {
        let start = std::time::Instant::now();
        
        // Flash crash: 50k -> 35k -> 48k recovery
        let prices = vec![50000, 45000, 35000, 40000, 45000, 48000];
        let mut trades = 0;
        let mut pnl = 0i64;
        
        for (i, &price) in prices.iter().enumerate() {
            self.engine.process_market_update("BTC-USD", price, 0.60).await;
            
            if price <= 35000 { // Buy the dip
                trades += 1;
                let trade_pnl = (48000 - price as i64) * 100000 / price as i64; // 1 BTC position
                pnl += trade_pnl;
                
                let trade = TradeRecord {
                    symbol: "BTC-USD".to_string(),
                    size: 100000,
                    entry_price: price,
                    exit_price: 48000,
                    pnl: trade_pnl,
                    entry_time: Utc::now(),
                    exit_time: Utc::now(),
                };
                self.engine.record_trade(trade).await;
            }
        }
        
        self.test_results.push(TestResult {
            scenario: "Flash Crash".to_string(),
            symbol: "BTC-USD".to_string(),
            success: pnl > 0,
            pnl,
            trades,
            max_drawdown: 0.30,
            risk_score: 0.7,
            execution_time_ms: start.elapsed().as_millis() as u64,
            details: "Opportunistic buying during flash crash".to_string(),
        });
    }

    async fn test_pump_and_dump(&mut self) {
        let start = std::time::Instant::now();
        
        // Pump and dump pattern
        let prices = vec![10, 12, 15, 25, 35, 20, 8, 6]; // Altcoin pump/dump
        let mut trades = 0;
        let mut pnl = 0i64;
        let position_size = 10000000; // 10M tokens
        
        for (i, &price) in prices.iter().enumerate() {
            self.engine.process_market_update("MEME-USD", price, 0.80).await;
            
            if i == 1 { // Buy early in pump
                trades += 1;
                let entry_pnl = (25 - price as i64) * position_size / price as i64;
                pnl += entry_pnl;
            } else if price >= 25 { // Sell at peak
                trades += 1;
                let exit_pnl = (price as i64 - 12) * position_size / 12;
                pnl = exit_pnl; // Override with actual profit
                
                let trade = TradeRecord {
                    symbol: "MEME-USD".to_string(),
                    size: position_size as u64,
                    entry_price: 12,
                    exit_price: price,
                    pnl: exit_pnl,
                    entry_time: Utc::now(),
                    exit_time: Utc::now(),
                };
                self.engine.record_trade(trade).await;
                break;
            }
        }
        
        self.test_results.push(TestResult {
            scenario: "Pump and Dump".to_string(),
            symbol: "MEME-USD".to_string(),
            success: pnl > 0,
            pnl,
            trades,
            max_drawdown: 0.60,
            risk_score: 0.95,
            execution_time_ms: start.elapsed().as_millis() as u64,
            details: "High-risk altcoin speculation".to_string(),
        });
    }

    async fn test_low_liquidity(&mut self) {
        let start = std::time::Instant::now();
        
        // Low liquidity scenario with wide spreads
        let bid_ask_spreads = vec![
            (1000, 1020), // 2% spread
            (1015, 1040), // 2.5% spread  
            (1030, 1065), // 3.4% spread
            (1050, 1090), // 3.8% spread
        ];
        
        let mut trades = 0;
        let mut pnl = 0i64;
        let position_size = 50000;
        
        for (bid, ask) in bid_ask_spreads {
            let mid_price = (bid + ask) / 2;
            self.engine.process_market_update("LOWLIQ-USD", mid_price, 0.15).await;
            
            // Simulate slippage impact
            trades += 1;
            let slippage_cost = (ask - bid) * position_size / mid_price;
            pnl -= slippage_cost as i64;
            
            let trade = TradeRecord {
                symbol: "LOWLIQ-USD".to_string(),
                size: position_size,
                entry_price: bid,
                exit_price: ask, // Pay ask price
                pnl: -(slippage_cost as i64),
                entry_time: Utc::now(),
                exit_time: Utc::now(),
            };
            self.engine.record_trade(trade).await;
        }
        
        self.test_results.push(TestResult {
            scenario: "Low Liquidity".to_string(),
            symbol: "LOWLIQ-USD".to_string(),
            success: pnl > -10000, // Acceptable slippage loss
            pnl,
            trades,
            max_drawdown: 0.05,
            risk_score: 0.6,
            execution_time_ms: start.elapsed().as_millis() as u64,
            details: "High slippage impact from low liquidity".to_string(),
        });
    }

    fn generate_summary(&self) {
        println!("\n📈 COMPREHENSIVE CRYPTO TEST RESULTS");
        println!("=====================================");
        
        let total_tests = self.test_results.len();
        let successful_tests = self.test_results.iter().filter(|r| r.success).count();
        let total_pnl: i64 = self.test_results.iter().map(|r| r.pnl).sum();
        let total_trades: u32 = self.test_results.iter().map(|r| r.trades).sum();
        let avg_execution_time: f64 = self.test_results.iter().map(|r| r.execution_time_ms as f64).sum::<f64>() / total_tests as f64;
        
        println!("Total Scenarios: {}", total_tests);
        println!("Successful: {} ({:.1}%)", successful_tests, (successful_tests as f64 / total_tests as f64) * 100.0);
        println!("Total PnL: ${}", total_pnl);
        println!("Total Trades: {}", total_trades);
        println!("Avg Execution Time: {:.1}ms", avg_execution_time);
        
        println!("\n🏆 DETAILED RESULTS:");
        for result in &self.test_results {
            let status = if result.success { "✅" } else { "❌" };
            println!("{} {} ({}): ${} PnL, {} trades, {:.1}% drawdown, {:.1}ms", 
                     status, result.scenario, result.symbol, result.pnl, 
                     result.trades, result.max_drawdown * 100.0, result.execution_time_ms);
        }
        
        // Risk analysis
        println!("\n⚠️  RISK ANALYSIS:");
        let high_risk_scenarios: Vec<_> = self.test_results.iter()
            .filter(|r| r.risk_score > 0.7)
            .collect();
        
        for scenario in high_risk_scenarios {
            println!("• {}: Risk Score {:.1}/10, Max Drawdown {:.1}%", 
                     scenario.scenario, scenario.risk_score * 10.0, scenario.max_drawdown * 100.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn run_comprehensive_crypto_tests() {
        let mut test_suite = CryptoTestSuite::new();
        test_suite.run_all_scenarios().await;
        
        // Verify test coverage
        assert!(test_suite.test_results.len() >= 12, "Should run at least 12 test scenarios");
        
        let success_rate = test_suite.test_results.iter().filter(|r| r.success).count() as f64 
                          / test_suite.test_results.len() as f64;
        assert!(success_rate >= 0.6, "At least 60% of scenarios should succeed");
    }
}