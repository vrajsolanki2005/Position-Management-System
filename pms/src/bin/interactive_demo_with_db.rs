use std::io::{self, Write};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to database (with better error handling)
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:2005@localhost:5432/rust_db".to_string());
    
    println!("Connecting to database: {}", database_url);
    let pool = match PgPool::connect(&database_url).await {
        Ok(pool) => pool,
        Err(e) => {
            println!("❌ Database connection failed: {}", e);
            println!("💡 Make sure PostgreSQL is running on localhost:5432");
            println!("💡 Or set DATABASE_URL environment variable");
            return Err(e.into());
        }
    };
    
    println!("🎬 INTERACTIVE TRADING SYSTEM DEMO");
    println!("==================================================\n");
    
    // Demo sections (same as before)...
    demo_section("1. SYSTEM INITIALIZATION").await;
    println!("✅ Trading Engine: ONLINE");
    println!("✅ Risk Manager: ACTIVE");
    println!("✅ Database: CONNECTED");
    wait_for_enter().await;
    
    // Get trading inputs
    let username = get_input("Enter username: ").await;
    let username = if username.trim().is_empty() { "demo_trader" } else { username.trim() };
    
    let symbol = get_input("Trading pair: ").await;
    let symbol = if symbol.trim().is_empty() { "BTC-USD" } else { symbol.trim() };
    
    let wallet_balance = get_float_input("Wallet Balance: $", 1000.0).await;
    println!("💳 Current Wallet Balance: ${:.2}", wallet_balance);
    
    let entry_price = get_float_input("Entry price : ", 45000.0).await;
    let position_size = get_float_input("Position size (units): ", 1.0).await;
    let leverage = get_float_input("Leverage 1-2: ", 10.0).await;
    
    let side = get_input("Side LONG/SHORT: ").await;
    let side = if side.trim().is_empty() { "LONG" } else { side.trim() };
    
    // Advanced orders
    let take_profit = get_optional_price("Take Profit price (optional): ").await;
    let stop_loss = get_optional_price("Stop Loss price (optional): ").await;
    
    // Save to database
    let position_id = Uuid::new_v4();
    let now = Utc::now();
    
    sqlx::query(
        "INSERT INTO demo_positions (id, username, symbol, side, size, entry_price, leverage, created_at, status, take_profit, stop_loss)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"
    )
    .bind(position_id)
    .bind(username)
    .bind(&symbol)
    .bind(side)
    .bind((position_size * 100.0) as i64) // Store as cents
    .bind((entry_price * 100.0) as i64) // Store as cents
    .bind(leverage as i32)
    .bind(now)
    .bind("OPEN")
    .bind(take_profit.map(|p| p as i64))
    .bind(stop_loss.map(|p| p as i64))
    .execute(&pool)
    .await?;
    
    // Create advanced orders
    if let Some(tp_price) = take_profit {
        create_order(&pool, position_id, "TAKE_PROFIT", tp_price).await?;
        println!("✅ Take Profit order set at ${:.2}", tp_price);
    }
    if let Some(sl_price) = stop_loss {
        create_order(&pool, position_id, "STOP_LOSS", sl_price).await?;
        println!("✅ Stop Loss order set at ${:.2}", sl_price);
    }
    
    println!("✅ Position saved to database with ID: {}", position_id);
    
    // Calculate margin and investment
    let notional_value = position_size * entry_price;
    let margin_required = notional_value / leverage;
    let remaining_balance = wallet_balance - margin_required;
    println!("💰 Investment: ${:.2} (Margin Required: ${:.2}, Notional Value: ${:.2}, Leverage: {:.1}x)", margin_required, margin_required, notional_value, leverage);
    println!("💳 Remaining Balance: ${:.2}", remaining_balance);
    
    // Market simulation with DB updates
    let mut current_price: f64 = entry_price;
    println!("\n💡 Commands: Enter new price, 'exit' or 'quit' to close position, or ENTER to finish");
    loop {
        let price_input = get_input(&format!("New market price (current: ${:.2}): ", current_price)).await;
        let input = price_input.trim().to_lowercase();
        
        if input.is_empty() || input == "exit" || input == "quit" { break; }
        
        if let Ok(new_price) = input.parse::<f64>() {
            current_price = new_price;
            let pnl = calculate_pnl(entry_price, current_price, position_size, side);
            
            // Check for triggered orders
            if let Some(triggered) = check_orders(&pool, position_id, current_price, side).await? {
                let final_pnl = calculate_pnl(entry_price, current_price, position_size, side);
                println!("🚨 {} TRIGGERED at ${:.2}!", triggered.0, current_price);
                println!("💵 Final PnL: ${:.2}", final_pnl);
                close_position(&pool, position_id, current_price, &triggered.0).await?;
                println!("🎉 Position closed by {} order!", triggered.0);
                return Ok(());
            }
            
            // Update database with new price and PnL
            sqlx::query(
                "INSERT INTO demo_price_updates (position_id, price, pnl, timestamp)
                 VALUES ($1, $2, $3, $4)"
            )
            .bind(position_id)
            .bind((current_price * 100.0) as i64)
            .bind(pnl)
            .bind(Utc::now())
            .execute(&pool)
            .await?;
            
            println!("📊 Market Update: {} = ${:.2} (PnL: ${:.2}) - SAVED TO DB", symbol, current_price, pnl);
        }
    }
    
    // Final update
    sqlx::query("UPDATE demo_positions SET status = 'CLOSED', exit_price = $1 WHERE id = $2")
        .bind((current_price * 100.0) as i64)
        .bind(position_id)
        .execute(&pool)
        .await?;
    
    println!("🎉 Demo complete! All data saved to database.");
    Ok(())
}

// Helper functions (same as original)
async fn demo_section(title: &str) {
    println!("\n{}", "=".repeat(50));
    println!("📋 {}", title);
    println!("{}", "=".repeat(50));
}

async fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

async fn get_float_input(prompt: &str, default: f64) -> f64 {
    loop {
        let input = get_input(prompt).await;
        if input.is_empty() { return default; }
        match input.parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number. Please try again."),
        }
    }
}

fn calculate_pnl(entry_price: f64, current_price: f64, position_size: f64, side: &str) -> f64 {
    let price_diff = current_price - entry_price;
    let pnl = price_diff * position_size;
    if side == "LONG" { pnl } else { -pnl }
}

async fn get_optional_price(prompt: &str) -> Option<f64> {
    let input = get_input(prompt).await;
    if input.is_empty() { None } else { input.parse().ok() }
}

async fn create_order(pool: &PgPool, position_id: Uuid, order_type: &str, trigger_price: f64) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::query(
        "INSERT INTO demo_orders (id, position_id, order_type, trigger_price, created_at)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(Uuid::new_v4())
    .bind(position_id)
    .bind(order_type)
    .bind((trigger_price * 100.0) as i64)
    .bind(Utc::now())
    .execute(pool)
    .await?;
    Ok(())
}

async fn check_orders(pool: &PgPool, position_id: Uuid, current_price: f64, side: &str) -> Result<Option<(String, f64)>, Box<dyn std::error::Error>> {
    let orders = sqlx::query_as::<_, (String, i64)>(
        "SELECT order_type, trigger_price FROM demo_orders WHERE position_id = $1 AND status = 'ACTIVE'"
    )
    .bind(position_id)
    .fetch_all(pool)
    .await?;
    
    for (order_type, trigger_price) in orders {
        let trigger = trigger_price as f64 / 100.0;
        let triggered = match (order_type.as_str(), side) {
            ("TAKE_PROFIT", "LONG") => current_price >= trigger,
            ("TAKE_PROFIT", "SHORT") => current_price <= trigger,
            ("STOP_LOSS", "LONG") => current_price <= trigger,
            ("STOP_LOSS", "SHORT") => current_price >= trigger,
            _ => false,
        };
        
        if triggered {
            sqlx::query("UPDATE demo_orders SET status = 'TRIGGERED', triggered_at = $1 WHERE position_id = $2 AND order_type = $3")
                .bind(Utc::now())
                .bind(position_id)
                .bind(&order_type)
                .execute(pool)
                .await?;
            return Ok(Some((order_type, trigger)));
        }
    }
    Ok(None)
}

async fn close_position(pool: &PgPool, position_id: Uuid, exit_price: f64, reason: &str) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::query("UPDATE demo_positions SET status = $1, exit_price = $2 WHERE id = $3")
        .bind(format!("CLOSED_{}", reason))
        .bind((exit_price * 100.0) as i64)
        .bind(position_id)
        .execute(pool)
        .await?;
    Ok(())
}

async fn wait_for_enter() {
    print!("\n⏸️  Press ENTER to continue...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}