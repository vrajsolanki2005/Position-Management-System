use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:2005@localhost:5432/rust_db".to_string());
    
    println!("🔍 Database Status Check");
    println!("📍 Connection string: {}", database_url);
    
    // Quick connection test with short timeout
    match PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(2))
        .connect(&database_url)
        .await 
    {
        Ok(pool) => {
            println!("✅ Database connection successful!");
            
            // Test basic query
            let result: i32 = sqlx::query_scalar("SELECT 1")
                .fetch_one(&pool)
                .await?;
            
            println!("✅ Database query test successful! (result: {})", result);
            
            // Check schema
            let schema_exists: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM information_schema.schemata WHERE schema_name = 'perp')"
            )
            .fetch_one(&pool)
            .await?;
            
            if schema_exists {
                println!("✅ Schema 'perp' exists");
                
                let table_count: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'perp'"
                )
                .fetch_one(&pool)
                .await?;
                
                println!("✅ Found {} tables in perp schema", table_count);
            } else {
                println!("⚠️  Schema 'perp' does not exist - run schema migration");
            }
            
            pool.close().await;
            println!("✅ Database is fully functional!");
        }
        Err(e) => {
            println!("❌ Database connection failed: {}", e);
            println!("\n🔧 Troubleshooting:");
            println!("1. PostgreSQL is not running");
            println!("2. Start PostgreSQL service");
            println!("3. Verify database 'rust_db' exists");
            println!("4. Check credentials in .env file");
            println!("\n💡 To start PostgreSQL:");
            println!("   - Windows: Start 'postgresql' service");
            println!("   - Or install PostgreSQL if not installed");
        }
    }
    
    Ok(())
}