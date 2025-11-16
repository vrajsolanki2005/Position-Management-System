use anyhow::Result;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    
    // Try different connection strings
    let connection_attempts = vec![
        "postgres://postgres@localhost:5432/postgres",
        "postgres://postgres:@localhost:5432/postgres", 
        "postgres://postgres:2005@localhost:5432/postgres",
        "postgres://localhost:5432/postgres",
    ];
    
    let mut pool = None;
    for url in connection_attempts {
        println!("Trying connection: {}", url);
        match PgPoolOptions::new()
            .max_connections(1)
            .connect(url)
            .await 
        {
            Ok(p) => {
                println!("✅ Connected successfully!");
                pool = Some(p);
                break;
            }
            Err(e) => {
                println!("❌ Failed: {}", e);
            }
        }
    }
    
    let Some(pool) = pool else {
        println!("❌ All connection attempts failed");
        return Ok(());
    };
    
    println!("Connecting to PostgreSQL to create database...");
    
    // Create the database
    match sqlx::query("CREATE DATABASE rust_db").execute(&pool).await {
        Ok(_) => println!("✅ Database 'rust_db' created successfully!"),
        Err(e) => {
            if e.to_string().contains("already exists") {
                println!("✅ Database 'rust_db' already exists!");
            } else {
                println!("❌ Error creating database: {}", e);
                return Err(e.into());
            }
        }
    }
    
    pool.close().await;
    
    Ok(())
}