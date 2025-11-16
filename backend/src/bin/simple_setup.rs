use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Row};

#[tokio::main]
async fn main() -> Result<()> {
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:2005@localhost:5432/rust_db")
        .await?;
    
    println!("🔧 Creating simple tables...");
    
    // Create simple positions table
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS positions (
            id SERIAL PRIMARY KEY,
            owner TEXT NOT NULL,
            symbol TEXT NOT NULL,
            side TEXT NOT NULL,
            size BIGINT NOT NULL,
            entry_price BIGINT NOT NULL,
            margin BIGINT NOT NULL,
            leverage INTEGER NOT NULL,
            state TEXT DEFAULT 'open',
            created_at TIMESTAMP DEFAULT NOW()
        )
    "#).execute(&pool).await?;
    
    println!("✅ Table created!");
    
    // Insert sample data
    sqlx::query("INSERT INTO positions (owner, symbol, side, size, entry_price, margin, leverage) VALUES ($1, $2, $3, $4, $5, $6, $7)")
        .bind("user123")
        .bind("BTC/USD")
        .bind("long")
        .bind(100000)
        .bind(50000)
        .bind(5000)
        .bind(10)
        .execute(&pool).await?;
    
    // Check data
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM positions").fetch_one(&pool).await?;
    println!("✅ Sample data inserted! Total positions: {}", count);
    
    // Show data
    let rows = sqlx::query("SELECT owner, symbol, side, size FROM positions LIMIT 5")
        .fetch_all(&pool).await?;
    
    println!("📊 Sample positions:");
    for row in rows {
        let owner: String = row.get("owner");
        let symbol: String = row.get("symbol");
        let side: String = row.get("side");
        let size: i64 = row.get("size");
        println!("  {} - {} {} size:{}", owner, symbol, side, size);
    }
    
    pool.close().await;
    println!("✅ Database working perfectly!");
    
    Ok(())
}