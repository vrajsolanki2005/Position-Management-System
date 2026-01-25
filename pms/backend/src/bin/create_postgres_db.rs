use anyhow::Result;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔧 Creating PostgreSQL database...");
    
    // Connect to postgres database first
    let admin_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://postgres:2005@localhost:5432/postgres")
        .await?;
    
    println!("✅ Connected to PostgreSQL server");
    
    // Create database
    sqlx::query("CREATE DATABASE rust_db")
        .execute(&admin_pool)
        .await?;
    
    println!("✅ Database 'rust_db' created");
    
    admin_pool.close().await;
    
    // Connect to new database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:2005@localhost:5432/rust_db")
        .await?;
    
    println!("✅ Connected to rust_db");
    
    // Create schema
    let schema = include_str!("../db/schema.sql");
    let statements: Vec<&str> = schema.split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty() && !s.starts_with("--"))
        .collect();
    
    for stmt in statements {
        sqlx::query(stmt).execute(&pool).await?;
    }
    
    println!("✅ Schema created successfully!");
    
    pool.close().await;
    println!("✅ PostgreSQL database is ready!");
    
    Ok(())
}