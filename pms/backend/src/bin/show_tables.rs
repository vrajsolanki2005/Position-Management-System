use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Row};

#[tokio::main]
async fn main() -> Result<()> {
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:2005@localhost:5432/rust_db")
        .await?;
    
    println!("📋 Checking database and tables...");
    
    // Show current database
    let db_name: String = sqlx::query_scalar("SELECT current_database()").fetch_one(&pool).await?;
    println!("Current database: {}", db_name);
    
    // Show all schemas
    let schemas = sqlx::query("SELECT schema_name FROM information_schema.schemata WHERE schema_name NOT IN ('information_schema', 'pg_catalog', 'pg_toast')")
        .fetch_all(&pool).await?;
    
    println!("\nSchemas:");
    for row in schemas {
        let schema: String = row.get("schema_name");
        println!("  - {}", schema);
    }
    
    // Show all tables in public schema
    let tables = sqlx::query("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'")
        .fetch_all(&pool).await?;
    
    println!("\nTables in public schema:");
    for row in tables {
        let table: String = row.get("table_name");
        println!("  - {}", table);
    }
    
    // Show data in positions table
    let positions = sqlx::query("SELECT * FROM positions LIMIT 5")
        .fetch_all(&pool).await?;
    
    println!("\nData in positions table:");
    for row in positions {
        let id: i32 = row.get("id");
        let owner: String = row.get("owner");
        let symbol: String = row.get("symbol");
        println!("  ID: {}, Owner: {}, Symbol: {}", id, owner, symbol);
    }
    
    pool.close().await;
    Ok(())
}