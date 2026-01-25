use anyhow::Result;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:2005@localhost:5432/rust_db".to_string());
    
    println!("Testing database connection: {}", database_url);
    
    // Try to connect to the database
    match PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await 
    {
        Ok(pool) => {
            println!("✅ Database connection successful!");
            
            // Test schema migration
            let schema = include_str!("../db/schema.sql");
            let statements: Vec<&str> = schema.split(';')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty() && !s.starts_with("--"))
                .collect();
            
            println!("Executing {} schema statements...", statements.len());
            
            for (i, stmt) in statements.iter().enumerate() {
                if let Err(e) = sqlx::query(stmt).execute(&pool).await {
                    println!("❌ Error executing statement {}: {}", i + 1, e);
                    println!("Statement: {}", stmt);
                    return Err(e.into());
                }
            }
            
            println!("✅ Schema migration completed successfully!");
            
            // Test a simple query
            let result = sqlx::query("SELECT COUNT(*) as count FROM positions")
                .fetch_one(&pool)
                .await?;
            
            println!("✅ Database query test successful!");
            println!("Positions table exists and is accessible");
            
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