use anyhow::Result;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:2005@localhost:5432/rust_db".to_string());
    
    println!("🔍 Checking database: {}", database_url);
    
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await?;
    
    println!("✅ Database connection successful!");
    
    // Check if tables exist
    let tables = sqlx::query_scalar::<_, String>(
        "SELECT table_name FROM information_schema.tables WHERE table_schema = 'perp'"
    )
    .fetch_all(&pool)
    .await?;
    
    if tables.is_empty() {
        println!("⚠️  No tables found in 'perp' schema");
        
        // Check public schema
        let public_tables = sqlx::query_scalar::<_, String>(
            "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'"
        )
        .fetch_all(&pool)
        .await?;
        
        println!("📋 Tables in public schema: {:?}", public_tables);
    } else {
        println!("✅ Found {} tables in perp schema:", tables.len());
        for table in &tables {
            println!("  - {}", table);
        }
        
        // Test a simple query on positions table
        if tables.contains(&"positions".to_string()) {
            let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM perp.positions")
                .fetch_one(&pool)
                .await?;
            println!("✅ Positions table has {} records", count);
        }
    }
    
    pool.close().await;
    println!("✅ Database check completed successfully!");
    
    Ok(())
}