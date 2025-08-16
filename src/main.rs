use actix_web::{middleware::Logger, web, App, HttpServer, Result};
use actix_files as fs;
use actix_cors::Cors;
use dotenv::dotenv;
use log::info;
use sqlx::SqlitePool;
use std::env;

mod api;
mod db;
mod extractors;
mod matching;
mod models;

use api::{
    documents::{upload_document, get_documents, get_document},
    cases::{get_cases, get_case, search_cases},
    research::{extract_clauses, match_precedents, generate_draft, get_research_sheet},
    health::health_check,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment or .env file");
    
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    // Create database connection pool
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Run database migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    // Seed database with precedent data (only if empty)
    match db::seed_precedent_data(&pool).await {
        Ok(_) => info!("Database seeded with precedent data"),
        Err(e) => info!("Precedent data already exists or seeding failed: {}", e),
    }

    info!("Starting Legal Research System server on port {}", port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            // Health check
            .route("/health", web::get().to(health_check))
            // Document management
            .route("/api/documents", web::post().to(upload_document))
            .route("/api/documents", web::get().to(get_documents))
            .route("/api/documents/{id}", web::get().to(get_document))
            // Legal cases and precedents
            .route("/api/cases", web::get().to(get_cases))
            .route("/api/cases/{id}", web::get().to(get_case))
            .route("/api/cases/search", web::post().to(search_cases))
            // Research workflow
            .route("/api/research/extract-clauses", web::post().to(extract_clauses))
            .route("/api/research/match-precedents", web::post().to(match_precedents))
            .route("/api/research/generate-draft", web::post().to(generate_draft))
            .route("/api/research/sheet/{document_id}", web::get().to(get_research_sheet))
            // Static files and UI
            .service(fs::Files::new("/static", "./static/").index_file("index.html"))
            .route("/", web::get().to(|| async { 
                actix_web::HttpResponse::Found()
                    .append_header(("Location", "/static/"))
                    .finish()
            }))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}