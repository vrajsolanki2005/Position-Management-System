use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Row};
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:2005@localhost:5432/rust_db")
        .await?;
    
    // SOL position data
    let pda = vec![7u8; 32]; // Position PDA
    let owner = vec![8u8; 32]; // Trader pubkey
    let symbol = "SOL-PERP";
    let side = "long";
    let size = 1000000i64; // 1 SOL (6 decimals)
    let entry_price = 180000000i64; // $180 (6 decimals)
    let margin = 9000000i64; // $9 margin (20x leverage)
    let leverage = 20i32;
    let liquidation_price = 171000000i64; // $171 (5% below entry)
    
    // Insert SOL position
    sqlx::query(r#"
        INSERT INTO positions 
        (owner, symbol, side, size, entry_price, margin, leverage)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
    "#)
    .bind("sol_trader")
    .bind(symbol)
    .bind(side)
    .bind(size)
    .bind(entry_price)
    .bind(margin)
    .bind(leverage)
    .execute(&pool).await?;
    
    // Verify position
    let position = sqlx::query("SELECT symbol, size, entry_price, margin, leverage FROM positions WHERE symbol = 'SOL-PERP'")
        .fetch_one(&pool).await?;
    
    let symbol: String = position.get("symbol");
    let size: i64 = position.get("size");
    let entry_price: i64 = position.get("entry_price");
    let margin: i64 = position.get("margin");
    let leverage: i32 = position.get("leverage");
    
    println!("✅ SOL position added to database:");
    println!("  Symbol: {}", symbol);
    println!("  Size: {} SOL", size as f64 / 1000000.0);
    println!("  Entry Price: ${}", entry_price as f64 / 1000000.0);
    println!("  Margin: ${}", margin as f64 / 1000000.0);
    println!("  Leverage: {}x", leverage);
    
    pool.close().await;
    Ok(())
}