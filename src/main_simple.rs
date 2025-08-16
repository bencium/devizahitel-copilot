use actix_web::{web, App, HttpServer, HttpResponse, Result, middleware::Logger};
use actix_files::Files;
use serde_json::json;
use std::env;
use serde::Deserialize;

#[derive(Deserialize)]
struct CasesQuery {
    fx_only: Option<bool>,
    limit: Option<u32>,
}

async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "Hungarian FX Mortgage Legal Research System",
        "version": "0.1.0",
        "database": "SQLite Ready",
        "features": [
            "Document OCR Processing",
            "Legal Clause Extraction", 
            "Precedent Matching",
            "Financial Damage Calculator",
            "Multilingual Support (HU/EN)"
        ]
    })))
}

async fn api_info() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "endpoints": {
            "health": "GET /health",
            "api_info": "GET /api/info", 
            "cases": "GET /api/cases - Legal precedents",
            "documents": "POST /api/documents - Document upload",
            "match_precedents": "POST /api/research/match-precedents",
            "generate_draft": "POST /api/research/generate-draft"
        },
        "message": "Legal Research API - Functional with Mock Data"
    })))
}

async fn get_cases(query: web::Query<CasesQuery>) -> Result<HttpResponse> {
    // Mock legal precedents data
    let cases = json!([
        {
            "id": "cjeu-c-630-23",
            "case_number": "C-630/23",
            "case_name": "ZH, KN v AxFina Hungary",
            "country": "EU",
            "date": "2025-04-15T00:00:00Z",
            "currency": "CHF",
            "court": "Court of Justice of the European Union",
            "key_ruling": "Inadequate FX risk disclosure can void entire contracts, not just modify terms. Banks must provide concrete scenarios, not generic warnings.",
            "case_type": "fx_risk_disclosure",
            "significance_score": 0.95
        },
        {
            "id": "cjeu-c-186-16",
            "case_number": "C-186/16", 
            "case_name": "Andriciuc v Banca Românească",
            "country": "EU",
            "date": "2017-09-20T00:00:00Z",
            "currency": "CHF",
            "court": "Court of Justice of the European Union",
            "key_ruling": "Transparency requirements for foreign currency loans. Banks must inform consumers of risks before contract conclusion.",
            "case_type": "transparency",
            "significance_score": 0.88
        },
        {
            "id": "hu-kuria-10-2025",
            "case_number": "Pfv.10.2025",
            "case_name": "Hungarian Kúria FX Decision",
            "country": "Hungary",
            "date": "2025-03-01T00:00:00Z", 
            "currency": "CHF",
            "court": "Hungarian Supreme Court (Kúria)",
            "key_ruling": "Following CJEU C-630/23, Hungarian courts must order full restitution for contracts with inadequate FX disclosure.",
            "case_type": "fx_risk_disclosure",
            "significance_score": 0.92
        },
        {
            "id": "aegon-broker-case",
            "case_number": "Local-2024-001",
            "case_name": "Aegon Broker Liability Case",
            "country": "Hungary", 
            "date": "2024-11-15T00:00:00Z",
            "currency": "EUR",
            "court": "Budapest Regional Court",
            "key_ruling": "Financial intermediaries liable under Banking Act 219/A-B§ for inadequate advice on FX loan alternatives.",
            "case_type": "broker_liability",
            "significance_score": 0.75
        }
    ]);
    
    Ok(HttpResponse::Ok().json(cases))
}

async fn upload_document() -> Result<HttpResponse> {
    // Mock document upload response
    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Document uploaded successfully",
        "document_id": "doc-123456789",
        "processing_status": "queued",
        "note": "This is a mock response. Full implementation coming soon."
    })))
}

async fn match_precedents() -> Result<HttpResponse> {
    // Mock precedent matching response
    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "overall_case_matches": [
            {
                "case_id": "cjeu-c-630-23",
                "case_number": "C-630/23",
                "similarity_score": 0.94,
                "matching_issues": ["fx_risk_disclosure", "contract_invalidity"],
                "recommendation": "Strong precedent for full contract invalidation"
            },
            {
                "case_id": "aegon-broker-case", 
                "case_number": "Local-2024-001",
                "similarity_score": 0.83,
                "matching_issues": ["broker_liability", "inadequate_advice"],
                "recommendation": "Additional damages for broker negligence"
            }
        ],
        "clause_matches": [
            {
                "clause_type": "fx_risk_disclosure",
                "confidence": 0.91,
                "violation_detected": true,
                "supporting_precedents": ["C-630/23", "C-186/16"]
            }
        ],
        "financial_analysis": {
            "estimated_recovery": "15-25M HUF",
            "probability": "High (85%)",
            "additional_damages": "2.9M HUF (broker liability)"
        }
    })))
}

async fn generate_draft() -> Result<HttpResponse> {
    // Mock legal draft generation
    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "pleading_type": "complaint",
        "content": "COMPLAINT FOR CONTRACT INVALIDATION\n\nTo the Honorable Court:\n\nPlaintiff respectfully submits this complaint seeking full invalidation of foreign currency mortgage contract...\n\n[This is a mock legal draft. Full implementation coming soon]",
        "citations": [
            "CJEU C-630/23 ZH, KN v AxFina Hungary ¶45-54",
            "CJEU C-186/16 Andriciuc v Banca Românească ¶49-51", 
            "Hungarian Banking Act 219/A-B§"
        ],
        "estimated_damages": "22.9M HUF total recovery",
        "note": "Mock generated content. Professional legal review required."
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init();
    
    // Load environment variables
    dotenv::dotenv().ok();
    
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{}:{}", host, port);
    
    println!("🏛️  Hungarian FX Mortgage Legal Research System");
    println!("🚀 Starting server at http://{}", bind_address);
    println!("📋 Health check: http://{}/health", bind_address);
    println!("📊 API info: http://{}/api/info", bind_address);
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/health", web::get().to(health))
            .route("/api/info", web::get().to(api_info))
            .route("/api/cases", web::get().to(get_cases))
            .route("/api/documents", web::post().to(upload_document))
            .route("/api/research/match-precedents", web::post().to(match_precedents))
            .route("/api/research/generate-draft", web::post().to(generate_draft))
            .service(Files::new("/", "static/").index_file("index.html"))
    })
    .bind(&bind_address)?
    .run()
    .await
}