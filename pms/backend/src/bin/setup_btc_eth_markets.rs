use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Row};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:2005@localhost:5432/rust_db")
        .await?;
    
    println!("🏦 Setting up BTC/ETH perpetual markets...");
    
    // BTC-PERP market
    sqlx::query(r#"
        INSERT INTO markets (symbol, quote_mint, base_decimals, quote_decimals, price_scale, im_rate_ppm, mm_rate_ppm)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (symbol) DO UPDATE SET
            im_rate_ppm = EXCLUDED.im_rate_ppm,
            mm_rate_ppm = EXCLUDED.mm_rate_ppm
    "#)
    .bind("BTC-PERP")
    .bind(&vec![1u8; 32]) // USDC mint placeholder
    .bind(8i16) // BTC has 8 decimals
    .bind(6i16) // USDC has 6 decimals
    .bind(1000000i64) // Price scale 1e6
    .bind(5000i64) // 0.5% initial margin
    .bind(2500i64) // 0.25% maintenance margin
    .execute(&pool).await?;
    
    // ETH-PERP market
    sqlx::query(r#"
        INSERT INTO markets (symbol, quote_mint, base_decimals, quote_decimals, price_scale, im_rate_ppm, mm_rate_ppm)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (symbol) DO UPDATE SET
            im_rate_ppm = EXCLUDED.im_rate_ppm,
            mm_rate_ppm = EXCLUDED.mm_rate_ppm
    "#)
    .bind("ETH-PERP")
    .bind(&vec![1u8; 32]) // USDC mint placeholder
    .bind(18i16) // ETH has 18 decimals
    .bind(6i16) // USDC has 6 decimals
    .bind(1000000i64) // Price scale 1e6
    .bind(10000i64) // 1% initial margin
    .bind(5000i64) // 0.5% maintenance margin
    .execute(&pool).await?;
    
    // Verify markets
    let markets = sqlx::query("SELECT symbol, im_rate_ppm, mm_rate_ppm FROM markets WHERE symbol IN ('BTC-PERP', 'ETH-PERP')")
        .fetch_all(&pool).await?;
    
    println!("✅ Markets configured:");
    for market in markets {
        let symbol: String = market.get("symbol");
        let im_rate: i64 = market.get("im_rate_ppm");
        let mm_rate: i64 = market.get("mm_rate_ppm");
        println!("  {} - IM: {}bps, MM: {}bps", symbol, im_rate, mm_rate);
    }
    
    pool.close().await;
    Ok(())
}