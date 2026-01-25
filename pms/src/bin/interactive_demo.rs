use std::io::{self, Write};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎬 INTERACTIVE TRADING SYSTEM DEMO");
    println!("==================================\n");
    
    // Demo 1: System Initialization
    demo_section("1. SYSTEM INITIALIZATION").await;
    println!("✅ Trading Engine: ONLINE");
    println!("✅ Risk Manager: ACTIVE");
    println!("✅ Database: CONNECTED");
    println!("✅ Solana RPC: READY");
    wait_for_enter().await;
    
    // Demo 2: User Setup
    demo_section("2. USER CONFIGURATION").await;
    let username = get_input("Enter username (default: demo_trader): ").await;
    let username = if username.trim().is_empty() { "demo_trader" } else { username.trim() };
    
    println!("Setting up user account...");
    sleep(Duration::from_millis(500)).await;
    println!("✅ User: {}", username);
    println!("✅ Tier: Premium (20x max leverage)");
    println!("✅ Position Limit: $100,000");
    wait_for_enter().await;
    
    // Demo 3: Position Setup
    demo_section("3. POSITION SETUP").await;
    
    let symbol = get_input("Trading pair (default: BTC-USD): ").await;
    let symbol = if symbol.trim().is_empty() { "BTC-USD" } else { symbol.trim() };
    
    let entry_price = get_number_input("Entry price (default: 45000): ", 45000).await;
    let position_size = get_number_input("Position size USD (default: 50000): ", 50000).await;
    let leverage = get_number_input("Leverage 1-20x (default: 10): ", 10).await;
    
    let side = get_input("Side LONG/SHORT (default: LONG): ").await;
    let side = if side.trim().is_empty() { "LONG".to_string() } else { side.trim().to_uppercase() };
    
    println!("\n📊 Market Data:");
    println!("   {}: ${}", symbol, entry_price);
    println!("   Volatility: 15%");
    println!("   Funding Rate: 0.01%");
    
    // Risk Validation
    println!("\n🔍 Risk Validation:");
    sleep(Duration::from_millis(300)).await;
    
    let required_margin = position_size / leverage;
    let is_valid = validate_position(leverage, position_size, required_margin);
    
    if !is_valid {
        println!("❌ Position rejected due to risk limits");
        return Ok(());
    }
    
    println!("\n📈 Opening Position:");
    println!("   Symbol: {}", symbol);
    println!("   Side: {}", side);
    println!("   Size: ${}", position_size);
    println!("   Leverage: {}x", leverage);
    println!("   Entry Price: ${}", entry_price);
    println!("   Required Margin: ${}", required_margin);
    println!("   ✅ Position opened successfully!");
    wait_for_enter().await;
    
    // Demo 4: Advanced Orders
    demo_section("4. ADVANCED ORDERS").await;
    
    let stop_loss_pct = get_number_input("Stop-loss % below entry (default: 7): ", 7).await;
    let take_profit_pct = get_number_input("Take-profit % above entry (default: 10): ", 10).await;
    
    let stop_price = if side == "LONG" {
        entry_price * (100 - stop_loss_pct) / 100
    } else {
        entry_price * (100 + stop_loss_pct) / 100
    };
    
    let tp_price = if side == "LONG" {
        entry_price * (100 + take_profit_pct) / 100
    } else {
        entry_price * (100 - take_profit_pct) / 100
    };
    
    println!("🎯 Stop-Loss Order:");
    println!("   Trigger Price: ${} (-{}%)", stop_price, stop_loss_pct);
    println!("   ✅ Stop-loss order active");
    
    println!("\n🎯 Take-Profit Order:");
    println!("   Trigger Price: ${} (+{}%)", tp_price, take_profit_pct);
    println!("   ✅ Take-profit order active");
    wait_for_enter().await;
    
    // Demo 5: Market Simulation
    demo_section("5. MARKET SIMULATION").await;
    println!("Enter market prices to simulate trading (press ENTER with empty input to finish):");
    
    let mut current_price = entry_price;
    loop {
        let price_input = get_input(&format!("New market price (current: ${}): ", current_price)).await;
        if price_input.trim().is_empty() {
            break;
        }
        
        if let Ok(new_price) = price_input.trim().parse::<u64>() {
            current_price = new_price;
            let pnl = calculate_pnl(entry_price, current_price, position_size, &side);
            
            println!("📊 Market Update: {} = ${}", symbol, current_price);
            println!("   Unrealized PnL: ${}", pnl);
            
            // Check order triggers
            if (side == "LONG" && current_price <= stop_price) || 
               (side == "SHORT" && current_price >= stop_price) {
                println!("   🛑 STOP-LOSS TRIGGERED!");
                println!("   ✅ Position closed at ${}", current_price);
                println!("   💰 Realized PnL: ${}", pnl);
                break;
            }
            
            if (side == "LONG" && current_price >= tp_price) || 
               (side == "SHORT" && current_price <= tp_price) {
                println!("   🎯 TAKE-PROFIT TRIGGERED!");
                println!("   ✅ Position closed at ${}", current_price);
                println!("   💰 Realized PnL: ${}", pnl);
                break;
            }
            
            println!();
        } else {
            println!("Invalid price format. Please enter a number.");
        }
    }
    
    wait_for_enter().await;
    
    // Demo 6: Final Summary
    demo_section("6. TRADING SUMMARY").await;
    let final_pnl = calculate_pnl(entry_price, current_price, position_size, &side);
    let roi = (final_pnl as f64 / required_margin as f64) * 100.0;
    
    println!("📈 Trading Session Complete:");
    println!("   Entry Price: ${}", entry_price);
    println!("   Exit Price: ${}", current_price);
    println!("   Position Size: ${}", position_size);
    println!("   Leverage: {}x", leverage);
    println!("   Final PnL: ${}", final_pnl);
    println!("   ROI: {:.1}%", roi);
    
    println!("\n🎉 DEMONSTRATION COMPLETE!");
    println!("System processed your custom trading scenario successfully.");
    
    Ok(())
}

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

async fn get_number_input(prompt: &str, default: u64) -> u64 {
    loop {
        let input = get_input(prompt).await;
        if input.is_empty() {
            return default;
        }
        match input.parse::<u64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number. Please try again."),
        }
    }
}

fn validate_position(leverage: u64, position_size: u64, required_margin: u64) -> bool {
    let max_leverage = 20;
    let max_position = 100000;
    
    if leverage > max_leverage {
        println!("   ❌ Leverage check: {}x > {}x (FAIL)", leverage, max_leverage);
        return false;
    }
    println!("   ✅ Leverage check: {}x ≤ {}x (PASS)", leverage, max_leverage);
    
    if position_size > max_position {
        println!("   ❌ Position size: ${} > ${} (FAIL)", position_size, max_position);
        return false;
    }
    println!("   ✅ Position size: ${} ≤ ${} (PASS)", position_size, max_position);
    
    println!("   ✅ Margin requirement: ${} available (PASS)", required_margin);
    true
}

fn calculate_pnl(entry_price: u64, current_price: u64, position_size: u64, side: &str) -> i64 {
    let price_diff = current_price as i64 - entry_price as i64;
    let pnl = (price_diff * position_size as i64) / entry_price as i64;
    if side == "LONG" { pnl } else { -pnl }
}

async fn wait_for_enter() {
    print!("\n⏸️  Press ENTER to continue...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}