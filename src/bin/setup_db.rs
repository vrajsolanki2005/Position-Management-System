use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:2005@localhost:5432/rust_db".to_string());
    
    println!("Setting up database tables...");
    let pool = PgPool::connect(&database_url).await?;
    
    // Create demo_positions table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS demo_positions (
            id UUID PRIMARY KEY,
            username VARCHAR(100) NOT NULL,
            symbol VARCHAR(20) NOT NULL,
            side VARCHAR(10) NOT NULL,
            size BIGINT NOT NULL,
            entry_price BIGINT NOT NULL,
            exit_price BIGINT,
            leverage INTEGER NOT NULL,
            created_at TIMESTAMPTZ NOT NULL,
            status VARCHAR(20) NOT NULL DEFAULT 'OPEN'
        )"
    ).execute(&pool).await?;
    
    // Create demo_price_updates table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS demo_price_updates (
            id SERIAL PRIMARY KEY,
            position_id UUID NOT NULL REFERENCES demo_positions(id),
            price BIGINT NOT NULL,
            pnl BIGINT NOT NULL,
            timestamp TIMESTAMPTZ NOT NULL
        )"
    ).execute(&pool).await?;
    
    println!("✅ Database tables created successfully!");
    Ok(())
}