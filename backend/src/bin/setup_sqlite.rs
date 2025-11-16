use anyhow::Result;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔧 Setting up SQLite database...");
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:rust_trading.db")
        .await?;
    
    println!("✅ SQLite database created: rust_trading.db");
    
    // Create basic schema
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS positions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            owner TEXT NOT NULL,
            symbol TEXT NOT NULL,
            side TEXT NOT NULL,
            size INTEGER NOT NULL,
            entry_price INTEGER NOT NULL,
            margin INTEGER NOT NULL,
            leverage INTEGER NOT NULL,
            state TEXT NOT NULL DEFAULT 'open',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
    "#).execute(&pool).await?;
    
    println!("✅ Schema created successfully!");
    
    // Test insert
    sqlx::query("INSERT INTO positions (owner, symbol, side, size, entry_price, margin, leverage) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .bind("test_owner")
        .bind("BTC/USD")
        .bind("long")
        .bind(100)
        .bind(50000)
        .bind(1000)
        .bind(10)
        .execute(&pool).await?;
    
    // Test query
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM positions")
        .fetch_one(&pool).await?;
    
    println!("✅ Test record inserted! Total positions: {}", count);
    
    pool.close().await;
    println!("✅ SQLite database is ready to use!");
    
    Ok(())
}