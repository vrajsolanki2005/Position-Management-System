use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Row};

#[tokio::main]
async fn main() -> Result<()> {
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:2005@localhost:5432/rust_db")
        .await?;
    
    // Add new position
    sqlx::query("INSERT INTO positions (owner, symbol, side, size, entry_price, margin, leverage) VALUES ($1, $2, $3, $4, $5, $6, $7)")
        .bind("user456")
        .bind("ETH/USD")
        .bind("short")
        .bind(50000)
        .bind(3000)
        .bind(2500)
        .bind(5)
        .execute(&pool).await?;
    
    println!("✅ New position added!");
    
    // Show all positions
    let positions = sqlx::query("SELECT * FROM positions ORDER BY id")
        .fetch_all(&pool).await?;
    
    println!("📊 All positions:");
    for row in positions {
        let id: i32 = row.get("id");
        let owner: String = row.get("owner");
        let symbol: String = row.get("symbol");
        let side: String = row.get("side");
        let size: i64 = row.get("size");
        let entry_price: i64 = row.get("entry_price");
        let margin: i64 = row.get("margin");
        let leverage: i32 = row.get("leverage");
        println!("  ID: {}, Owner: {}, Symbol: {}, Side: {}, Size: {}, Entry: {}, Margin: {}, Leverage: {}x", 
                 id, owner, symbol, side, size, entry_price, margin, leverage);
    }
    
    pool.close().await;
    Ok(())
}