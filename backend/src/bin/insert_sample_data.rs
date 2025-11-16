use anyhow::Result;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:2005@localhost:5432/rust_db")
        .await?;
    
    println!("📊 Inserting sample data...");
    
    // Sample markets
    sqlx::query("INSERT INTO markets (symbol, quote_mint, base_decimals, quote_decimals, price_scale) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
        .bind("BTC/USD")
        .bind(&vec![1u8; 32])
        .bind(9i16)
        .bind(6i16)
        .bind(1000000i64)
        .execute(&pool).await?;
    
    // Sample users
    sqlx::query("INSERT INTO users (owner, total_collateral) VALUES ($1, $2) ON CONFLICT DO NOTHING")
        .bind(&vec![2u8; 32])
        .bind(10000i64)
        .execute(&pool).await?;
    
    // Sample positions
    sqlx::query("INSERT INTO positions (pda, owner, symbol, side, size, entry_price, margin, leverage, liquidation_price, state) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) ON CONFLICT DO NOTHING")
        .bind(&vec![3u8; 32])
        .bind(&vec![2u8; 32])
        .bind("BTC/USD")
        .bind("long")
        .bind(100000i64)
        .bind(50000000000i64)
        .bind(5000000000i64)
        .bind(10i32)
        .bind(45000000000i64)
        .bind("open")
        .execute(&pool).await?;
    
    // Check data
    let market_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM markets").fetch_one(&pool).await?;
    let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users").fetch_one(&pool).await?;
    let position_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM positions").fetch_one(&pool).await?;
    
    println!("✅ Sample data inserted:");
    println!("  Markets: {}", market_count);
    println!("  Users: {}", user_count);
    println!("  Positions: {}", position_count);
    
    pool.close().await;
    Ok(())
}