use actix_web::{web, HttpResponse, Result};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Datelike;
use crate::models::{CaseSearchRequest};
use crate::db;
use serde_json::json;

pub async fn get_cases(
    pool: web::Data<SqlitePool>,
    query: web::Query<CaseQuery>,
) -> Result<HttpResponse> {
    let pool = pool.get_ref();
    
    match db::cases::get_all_cases(pool, query.limit).await {
        Ok(cases) => Ok(HttpResponse::Ok().json(json!({
            "cases": cases,
            "total": cases.len()
        }))),
        Err(e) => {
            log::error!("Failed to fetch cases: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to fetch cases",
                "details": e.to_string()
            })))
        }
    }
}

pub async fn get_case(
    pool: web::Data<SqlitePool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let pool = pool.get_ref();
    let case_id = path.into_inner();
    
    match db::cases::get_case_by_id(pool, case_id).await {
        Ok(Some(case)) => Ok(HttpResponse::Ok().json(case)),
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "error": "Case not found"
        }))),
        Err(e) => {
            log::error!("Failed to fetch case {}: {}", case_id, e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to fetch case",
                "details": e.to_string()
            })))
        }
    }
}

pub async fn search_cases(
    pool: web::Data<SqlitePool>,
    request: web::Json<CaseSearchRequest>,
) -> Result<HttpResponse> {
    let pool = pool.get_ref();
    
    match db::cases::search_cases(pool, request.into_inner()).await {
        Ok(cases) => {
            // Enhance results with relevance information
            let enhanced_results: Vec<_> = cases.into_iter().map(|case| {
                let relevance_score = calculate_search_relevance(&case);
                json!({
                    "case": case,
                    "relevance_score": relevance_score,
                    "is_fx_related": case.is_foreign_currency_case(),
                    "jurisdiction": case.get_jurisdiction()
                })
            }).collect();
            
            Ok(HttpResponse::Ok().json(json!({
                "results": enhanced_results,
                "total": enhanced_results.len()
            })))
        },
        Err(e) => {
            log::error!("Failed to search cases: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to search cases",
                "details": e.to_string()
            })))
        }
    }
}

fn calculate_search_relevance(case: &crate::models::LegalCase) -> f32 {
    let mut relevance: f32 = 0.5;
    
    // Boost for FX-related cases
    if case.is_foreign_currency_case() {
        relevance += 0.3;
    }
    
    // Boost for recent cases
    if case.date.year() >= 2020 {
        relevance += 0.2;
    }
    
    // Boost for CJEU cases
    if case.case_number.starts_with("C-") {
        relevance += 0.2;
    }
    
    // Check for key terms in ruling
    let ruling_lower = case.key_ruling.to_lowercase();
    if ruling_lower.contains("unfair") || ruling_lower.contains("invalid") {
        relevance += 0.1;
    }
    
    if ruling_lower.contains("restitution") || ruling_lower.contains("compensation") {
        relevance += 0.1;
    }
    
    relevance.clamp(0.0, 1.0)
}

#[derive(serde::Deserialize)]
pub struct CaseQuery {
    pub limit: Option<i32>,
    pub country: Option<String>,
    pub case_type: Option<String>,
    pub fx_only: Option<bool>,
}