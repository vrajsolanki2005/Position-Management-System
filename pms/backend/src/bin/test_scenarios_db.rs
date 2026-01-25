use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Row};
use serde_json::json;

#[derive(Debug)]
pub struct TestScenarioResult {
    pub scenario_name: String,
    pub symbol: String,
    pub trades_executed: i32,
    pub pnl: i64,
    pub max_drawdown: f64,
    pub success: bool,
    pub details: String,
    pub test_data: serde_json::Value,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:2005@localhost:5432/rust_db")
        .await?;
    
    println!("🧪 Running crypto trading scenarios and storing results...");
    
    // Create test results table if not exists
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS test_scenario_results (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            scenario_name TEXT NOT NULL,
            symbol TEXT NOT NULL,
            trades_executed INTEGER NOT NULL,
            pnl BIGINT NOT NULL,
            max_drawdown DOUBLE PRECISION NOT NULL,
            success BOOLEAN NOT NULL,
            details TEXT,
            test_data JSONB,
            created_at TIMESTAMPTZ DEFAULT NOW()
        )
    "#).execute(&pool).await?;
    
    // Generate test scenarios
    let scenarios = generate_test_scenarios();
    
    for scenario in scenarios {
        insert_test_result(&pool, &scenario).await?;
        println!("✅ Stored: {} - PnL: ${}", scenario.scenario_name, scenario.pnl);
    }
    
    // Query and display results
    display_test_summary(&pool).await?;
    
    pool.close().await;
    Ok(())
}

fn generate_test_scenarios() -> Vec<TestScenarioResult> {
    vec![
        TestScenarioResult {
            scenario_name: "Bull Market Rally".to_string(),
            symbol: "BTC-USD".to_string(),
            trades_executed: 3,
            pnl: 150000,
            max_drawdown: 0.02,
            success: true,
            details: "Strong uptrend with minimal drawdown".to_string(),
            test_data: json!({
                "entry_price": 50000,
                "exit_price": 65000,
                "position_size": 100000,
                "leverage": 10,
                "duration_hours": 24,
                "volatility": 0.08
            }),
        },
        TestScenarioResult {
            scenario_name: "Bear Market Crash".to_string(),
            symbol: "ETH-USD".to_string(),
            trades_executed: 4,
            pnl: -200000,
            max_drawdown: 0.40,
            success: false,
            details: "Severe downtrend with high volatility".to_string(),
            test_data: json!({
                "entry_price": 3000,
                "exit_price": 1800,
                "position_size": 500000,
                "leverage": 5,
                "duration_hours": 48,
                "volatility": 0.25
            }),
        },
        TestScenarioResult {
            scenario_name: "High Volatility Sideways".to_string(),
            symbol: "BTC-USD".to_string(),
            trades_executed: 8,
            pnl: 8000,
            max_drawdown: 0.15,
            success: true,
            details: "Profitable volatility trading with tight risk management".to_string(),
            test_data: json!({
                "price_range": [40000, 49000],
                "avg_position_size": 20000,
                "leverage": 15,
                "duration_hours": 12,
                "volatility": 0.35
            }),
        },
        TestScenarioResult {
            scenario_name: "Stop Loss Cascade".to_string(),
            symbol: "SOL-USD".to_string(),
            trades_executed: 2,
            pnl: -75000,
            max_drawdown: 0.15,
            success: true,
            details: "Stop loss successfully limited downside risk".to_string(),
            test_data: json!({
                "entry_price": 100,
                "stop_loss_price": 90,
                "exit_price": 85,
                "position_size": 1000000,
                "leverage": 10,
                "duration_hours": 6,
                "volatility": 0.20
            }),
        },
        TestScenarioResult {
            scenario_name: "Liquidation Scenario".to_string(),
            symbol: "BTC-USD".to_string(),
            trades_executed: 1,
            pnl: -4000,
            max_drawdown: 1.0,
            success: false,
            details: "Position liquidated due to excessive leverage".to_string(),
            test_data: json!({
                "entry_price": 50000,
                "liquidation_price": 42000,
                "position_size": 200000,
                "leverage": 25,
                "collateral": 4000,
                "duration_hours": 3,
                "volatility": 0.30
            }),
        },
        TestScenarioResult {
            scenario_name: "Take Profit Ladder".to_string(),
            symbol: "ETH-USD".to_string(),
            trades_executed: 3,
            pnl: 60000,
            max_drawdown: 0.0,
            success: true,
            details: "Systematic profit taking at predetermined levels".to_string(),
            test_data: json!({
                "entry_price": 2000,
                "tp_levels": [2200, 2400, 2500],
                "position_size": 300000,
                "leverage": 5,
                "duration_hours": 18,
                "volatility": 0.12
            }),
        },
        TestScenarioResult {
            scenario_name: "Flash Crash Recovery".to_string(),
            symbol: "BTC-USD".to_string(),
            trades_executed: 5,
            pnl: 25000,
            max_drawdown: 0.25,
            success: true,
            details: "Quick recovery from flash crash with DCA strategy".to_string(),
            test_data: json!({
                "entry_price": 45000,
                "flash_low": 35000,
                "recovery_price": 47000,
                "position_size": 150000,
                "leverage": 8,
                "duration_hours": 2,
                "volatility": 0.45
            }),
        },
        TestScenarioResult {
            scenario_name: "Range Bound Trading".to_string(),
            symbol: "ETH-USD".to_string(),
            trades_executed: 12,
            pnl: 45000,
            max_drawdown: 0.08,
            success: true,
            details: "Consistent profits from range-bound market conditions".to_string(),
            test_data: json!({
                "support_level": 2800,
                "resistance_level": 3200,
                "avg_position_size": 80000,
                "leverage": 3,
                "duration_hours": 72,
                "volatility": 0.10
            }),
        },
    ]
}

async fn insert_test_result(pool: &sqlx::PgPool, result: &TestScenarioResult) -> Result<()> {
    sqlx::query(r#"
        INSERT INTO test_scenario_results 
        (scenario_name, symbol, trades_executed, pnl, max_drawdown, success, details, test_data)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    "#)
    .bind(&result.scenario_name)
    .bind(&result.symbol)
    .bind(result.trades_executed)
    .bind(result.pnl)
    .bind(result.max_drawdown)
    .bind(result.success)
    .bind(&result.details)
    .bind(&result.test_data)
    .execute(pool)
    .await?;
    
    Ok(())
}

async fn display_test_summary(pool: &sqlx::PgPool) -> Result<()> {
    println!("\n📊 TEST SCENARIO SUMMARY");
    println!("========================");
    
    let total_scenarios: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM test_scenario_results")
        .fetch_one(pool).await.unwrap_or(0);
    
    let successful_scenarios: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM test_scenario_results WHERE success = true")
        .fetch_one(pool).await.unwrap_or(0);
    
    let total_pnl: i64 = sqlx::query_scalar("SELECT COALESCE(SUM(pnl), 0) FROM test_scenario_results")
        .fetch_one(pool).await.unwrap_or(0);
    
    let avg_drawdown: f64 = sqlx::query_scalar("SELECT COALESCE(AVG(max_drawdown), 0) FROM test_scenario_results")
        .fetch_one(pool).await.unwrap_or(0.0);
    
    println!("Total Scenarios: {}", total_scenarios);
    println!("Successful: {} ({:.1}%)", successful_scenarios, 
             (successful_scenarios as f64 / total_scenarios as f64) * 100.0);
    println!("Total PnL: ${}", total_pnl);
    println!("Average Max Drawdown: {:.2}%", avg_drawdown * 100.0);
    
    // Simple query without macro
    println!("\n🏆 TOP PERFORMING SCENARIOS:");
    let rows = sqlx::query("SELECT scenario_name, symbol, pnl, max_drawdown, trades_executed FROM test_scenario_results WHERE success = true ORDER BY pnl DESC LIMIT 3")
        .fetch_all(pool).await?;
    
    for (i, row) in rows.iter().enumerate() {
        let scenario_name: String = row.get("scenario_name");
        let symbol: String = row.get("symbol");
        let pnl: i64 = row.get("pnl");
        let max_drawdown: f64 = row.get("max_drawdown");
        let trades_executed: i32 = row.get("trades_executed");
        
        println!("{}. {} ({}): ${} PnL, {:.1}% drawdown, {} trades", 
                 i + 1, scenario_name, symbol, pnl, max_drawdown * 100.0, trades_executed);
    }
    
    // Risk analysis
    println!("\n⚠️  HIGH RISK SCENARIOS:");
    let risk_rows = sqlx::query("SELECT scenario_name, symbol, pnl, max_drawdown FROM test_scenario_results WHERE max_drawdown > 0.2 ORDER BY max_drawdown DESC")
        .fetch_all(pool).await?;
    
    for row in risk_rows {
        let scenario_name: String = row.get("scenario_name");
        let symbol: String = row.get("symbol");
        let pnl: i64 = row.get("pnl");
        let max_drawdown: f64 = row.get("max_drawdown");
        
        println!("• {} ({}): {:.1}% drawdown, ${} PnL", 
                 scenario_name, symbol, max_drawdown * 100.0, pnl);
    }
    
    Ok(())
}