use anyhow::Result;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:2005@localhost:5432/rust_db".to_string());
    
    println!("Connecting to database: {}", database_url);
    
    // Try to connect to the database
    match PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await 
    {
        Ok(pool) => {
            println!("✅ Database connection successful!");
            
            // Test a simple query
            let result = sqlx::query("SELECT 1 as test")
                .fetch_one(&pool)
                .await?;
            
            println!("✅ Database query test successful!");
            
            // Close the pool
            pool.close().await;
        }
        Err(e) => {
            println!("❌ Database connection failed: {}", e);
            println!("\nTroubleshooting steps:");
            println!("1. Make sure PostgreSQL is running");
            println!("2. Check if database 'rust_db' exists");
            println!("3. Verify credentials in .env file");
            println!("4. Run: createdb -U postgres rust_db");
            return Err(e.into());
        }
    }
    
    Ok(())
}