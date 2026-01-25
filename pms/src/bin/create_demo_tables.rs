use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/trading_db".to_string());
    
    let pool = PgPool::connect(&database_url).await?;
    
    // Create demo tables
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS demo_positions (
            id uuid PRIMARY KEY,
            username text NOT NULL,
            symbol text NOT NULL,
            side text NOT NULL,
            size bigint NOT NULL,
            entry_price bigint NOT NULL,
            exit_price bigint,
            leverage integer NOT NULL,
            status text NOT NULL DEFAULT 'OPEN',
            created_at timestamptz NOT NULL DEFAULT now(),
            closed_at timestamptz
        )"
    ).execute(&pool).await?;
    
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS demo_price_updates (
            id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
            position_id uuid NOT NULL REFERENCES demo_positions(id) ON DELETE CASCADE,
            price bigint NOT NULL,
            pnl bigint NOT NULL,
            timestamp timestamptz NOT NULL DEFAULT now()
        )"
    ).execute(&pool).await?;
    
    println!("✅ Demo tables created successfully!");
    Ok(())
}