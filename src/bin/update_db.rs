use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:2005@localhost:5432/rust_db".to_string());
    
    println!("Updating database schema...");
    let pool = PgPool::connect(&database_url).await?;
    
    // Add columns to demo_positions
    sqlx::query("ALTER TABLE demo_positions ADD COLUMN IF NOT EXISTS take_profit BIGINT")
        .execute(&pool).await?;
    sqlx::query("ALTER TABLE demo_positions ADD COLUMN IF NOT EXISTS stop_loss BIGINT")
        .execute(&pool).await?;
    
    // Create demo_orders table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS demo_orders (
            id UUID PRIMARY KEY,
            position_id UUID NOT NULL REFERENCES demo_positions(id),
            order_type VARCHAR(20) NOT NULL,
            trigger_price BIGINT NOT NULL,
            status VARCHAR(20) NOT NULL DEFAULT 'ACTIVE',
            created_at TIMESTAMPTZ NOT NULL,
            triggered_at TIMESTAMPTZ
        )"
    ).execute(&pool).await?;
    
    println!("✅ Database schema updated successfully!");
    Ok(())
}