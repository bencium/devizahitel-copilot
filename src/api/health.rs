use actix_web::{HttpResponse, Result};
use serde_json::json;

pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "Legal Research System",
        "version": "0.1.0",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}