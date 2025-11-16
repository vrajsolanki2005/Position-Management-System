use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Test different connection strings
    let urls = [
        "postgres://postgres:2005@localhost:5432/rust_db",
        "postgres://postgres@localhost:5432/rust_db", 
        "postgres://postgres:postgres@localhost:5432/rust_db",
    ];
    
    for url in urls {
        println!("Testing: {}", url);
        match PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(Duration::from_secs(3))
            .connect(url)
            .await 
        {
            Ok(pool) => {
                println!("✅ SUCCESS with: {}", url);
                let result: i32 = sqlx::query_scalar("SELECT 1").fetch_one(&pool).await.unwrap();
                println!("✅ Query result: {}", result);
                pool.close().await;
                return;
            }
            Err(e) => println!("❌ Failed: {}", e),
        }
    }
}