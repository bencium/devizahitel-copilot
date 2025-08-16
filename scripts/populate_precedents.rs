use std::error::Error;
use sqlx::PgPool;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = PgPool::connect(&database_url).await?;
    
    println!("Connected to database, populating precedent data...");
    
    // Run migrations first
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("Migrations completed");
    
    // Populate with precedent data
    devizahitel_legal_research::db::seed_precedent_data(&pool).await?;
    println!("Precedent data populated successfully!");
    
    // Display statistics
    let case_count = sqlx::query!("SELECT COUNT(*) as count FROM legal_cases")
        .fetch_one(&pool)
        .await?;
    
    let pattern_count = sqlx::query!("SELECT COUNT(*) as count FROM clause_patterns")
        .fetch_one(&pool)
        .await?;
    
    println!("Database populated with:");
    println!("- {} legal cases", case_count.count.unwrap_or(0));
    println!("- {} clause patterns", pattern_count.count.unwrap_or(0));
    
    Ok(())
}