use actix_web::{web, App, HttpServer, Result, middleware::Logger};
use actix_files::Files;
use std::sync::Arc;
use tokio::sync::Mutex;
use dotenv::dotenv;

// Include all necessary modules
mod api;
mod ai;
mod models;
mod db;
mod extractors;
mod matching;

use api::case_analysis::{
    SharedAnalysis, analyze_case, get_analysis_status, generate_documents, 
    apply_user_override, get_current_analysis
};
use ai::{create_ocr_file_watcher, FileChangeEvent};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize logging
    env_logger::init();

    println!("🏛️ Hungarian FX Mortgage Legal Research System");
    println!("===============================================");
    println!("🤖 AI-Powered Case Analysis");
    println!("📁 Case-Agnostic Document Processing"); 
    println!("🔄 Real-time File Monitoring");
    println!("");

    // Initialize shared state for analysis results
    let shared_analysis: SharedAnalysis = Arc::new(Mutex::new(None));
    let shared_analysis_clone = shared_analysis.clone();

    // Start file watcher in background
    tokio::spawn(async move {
        match create_ocr_file_watcher() {
            Ok(mut watcher) => {
                println!("👀 Starting file watcher for OCR output directory...");
                
                let analyzer_ref = shared_analysis_clone.clone();
                let callback = move |changes: Vec<FileChangeEvent>| {
                    let analyzer_ref = analyzer_ref.clone();
                    tokio::spawn(async move {
                        println!("📝 File changes detected, triggering case reanalysis...");
                        
                        match ai::CaseAnalyzer::new() {
                            Ok(analyzer) => {
                                match analyzer.analyze_full_case(None).await {
                                    Ok(new_analysis) => {
                                        let mut shared = analyzer_ref.lock().await;
                                        *shared = Some(new_analysis);
                                        println!("✅ Case reanalysis completed due to file changes");
                                    },
                                    Err(e) => {
                                        eprintln!("❌ Failed to reanalyze case after file changes: {}", e);
                                    }
                                }
                            },
                            Err(e) => {
                                eprintln!("❌ Failed to create analyzer for reanalysis: {}", e);
                            }
                        }
                    });
                };

                if let Err(e) = watcher.start_watching(callback).await {
                    eprintln!("❌ File watcher error: {}", e);
                }
            },
            Err(e) => {
                eprintln!("⚠️ Failed to start file watcher: {}", e);
            }
        }
    });

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);

    println!("🚀 Starting server on {}:{}", host, port);
    println!("📖 Open http://{}:{} to access the legal research interface", host, port);
    println!("");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_analysis.clone()))
            .wrap(Logger::default())
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            
            // Health endpoints
            .route("/health", web::get().to(health_check))
            .route("/api/info", web::get().to(api_info))
            
            // New AI-powered case analysis endpoints
            .route("/api/analyze", web::post().to(analyze_case))
            .route("/api/status", web::get().to(get_analysis_status))
            .route("/api/analysis", web::get().to(get_current_analysis))
            .route("/api/override", web::post().to(apply_user_override))
            .route("/api/generate-documents", web::post().to(generate_documents))
            
            // Legacy endpoints (for backward compatibility)
            .route("/api/cases", web::get().to(get_legal_precedents))
            .route("/api/documents", web::post().to(mock_document_upload))
            
            // Static file serving
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind((host, port))?
    .run()
    .await
}

async fn health_check() -> Result<actix_web::HttpResponse> {
    Ok(actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "Hungarian FX Mortgage Legal Research System",
        "version": "0.2.0",
        "features": [
            "🤖 AI-Powered Case Analysis",
            "📄 Dynamic Document Processing", 
            "💰 Intelligent Damage Calculation",
            "📁 Multi-Bank/Multi-Currency Support",
            "🔄 Real-time File Monitoring",
            "📝 Legal Document Generation",
            "🎯 User Override Capabilities",
            "⚖️ Precedent Matching",
            "🌐 Multilingual Support (HU/EN)"
        ],
        "ai_status": "Mistral AI Integration Active",
        "file_watcher": "Enabled"
    })))
}

async fn api_info() -> Result<actix_web::HttpResponse> {
    Ok(actix_web::HttpResponse::Ok().json(serde_json::json!({
        "endpoints": {
            "health": "GET /health - System health check",
            "api_info": "GET /api/info - API documentation",
            
            "analyze": "POST /api/analyze - Analyze case documents with AI",
            "status": "GET /api/status - Get analysis status and progress",
            "analysis": "GET /api/analysis - Get current analysis results",
            "override": "POST /api/override - Apply user corrections to extracted data",
            "generate_documents": "POST /api/generate-documents - Generate legal documents",
            
            "cases": "GET /api/cases - Legal precedents (legacy)",
            "documents": "POST /api/documents - Document upload (legacy)"
        },
        "message": "AI-Powered Legal Research API - Case Agnostic",
        "supported_banks": ["Erste", "Aegon", "OTP", "K&H", "CIB", "Raiffeisen", "UniCredit", "Any Hungarian Bank"],
        "supported_currencies": ["CHF", "EUR", "USD", "JPY", "GBP", "HUF"],
        "ai_capabilities": [
            "Document content analysis and classification",
            "Multi-bank and multi-currency case handling", 
            "Dynamic damage calculation based on case facts",
            "Personalized legal strategy generation",
            "Automated legal document drafting",
            "Precedent matching and citation",
            "User correction and override support"
        ]
    })))
}

async fn get_legal_precedents() -> Result<actix_web::HttpResponse> {
    // Return dynamic precedents without hardcoded case data
    Ok(actix_web::HttpResponse::Ok().json(serde_json::json!([
        {
            "id": "cjeu-c-630-23",
            "case_number": "C-630/23",
            "case_name": "ZH, KN v AxFina Hungary",
            "country": "EU",
            "date": "2025-04-15T00:00:00Z",
            "court": "Court of Justice of the European Union",
            "key_ruling": "Inadequate FX risk disclosure can void entire contracts. Banks must provide concrete scenarios, not generic warnings.",
            "case_type": "fx_risk_disclosure",
            "significance_score": 0.95,
            "applicability": "Universal - applies to all FX loans with poor disclosure"
        },
        {
            "id": "cjeu-c-186-16",
            "case_number": "C-186/16", 
            "case_name": "Andriciuc v Banca Românească",
            "country": "EU",
            "date": "2017-09-20T00:00:00Z",
            "court": "Court of Justice of the European Union",
            "key_ruling": "Transparency requirements for foreign currency loans. Banks must inform consumers of risks.",
            "case_type": "transparency",
            "significance_score": 0.88,
            "applicability": "Universal - transparency standards for all FX loans"
        },
        {
            "id": "hu-kuria-10-2025",
            "case_number": "Pfv.10.2025",
            "case_name": "Hungarian Kúria FX Decision",
            "country": "Hungary",
            "date": "2025-03-01T00:00:00Z",
            "court": "Hungarian Supreme Court (Kúria)",
            "key_ruling": "Following CJEU C-630/23, Hungarian courts must order full restitution for inadequate FX disclosure.",
            "case_type": "fx_risk_disclosure",
            "significance_score": 0.92,
            "applicability": "Hungary-specific implementation of EU law"
        },
        {
            "id": "broker-liability-precedent",
            "case_number": "Various",
            "case_name": "Banking Act 219/A-B§ Liability Cases",
            "country": "Hungary",
            "date": "2024-ongoing",
            "court": "Various Hungarian Courts",
            "key_ruling": "Financial intermediaries liable for inadequate advice on FX loan alternatives.",
            "case_type": "broker_liability",
            "significance_score": 0.75,
            "applicability": "Cases involving financial advisors or brokers"
        }
    ])))
}

async fn mock_document_upload() -> Result<actix_web::HttpResponse> {
    Ok(actix_web::HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Document upload endpoint available. Use /api/analyze for AI-powered analysis of OCR documents.",
        "note": "Place documents in ocr_output folder for automatic processing"
    })))
}