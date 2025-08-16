use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::ai::{CaseAnalyzer, MultiCaseAnalysis, UserOverride, GeneratedDocument};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisRequest {
    pub force_reanalyze: Option<bool>,
    pub user_overrides: Option<Vec<UserOverride>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisResponse {
    pub success: bool,
    pub analysis: Option<MultiCaseAnalysis>,
    pub error: Option<String>,
    pub processing_time_seconds: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentGenerationRequest {
    pub document_types: Vec<String>, // ["central_bank", "financial_authority", "lawyer_consultation"]
    pub case_ids: Option<Vec<String>>, // If None, generate for all cases
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentGenerationResponse {
    pub success: bool,
    pub documents: Vec<GeneratedDocument>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusResponse {
    pub analysis_status: String,
    pub last_analysis_date: Option<String>,
    pub total_cases: u32,
    pub total_recovery_huf: f64,
    pub monitored_files_count: u32,
    pub file_watcher_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OverrideRequest {
    pub field_path: String,
    pub new_value: String,
    pub reason: Option<String>,
}

// Shared state for analysis results
pub type SharedAnalysis = Arc<Mutex<Option<MultiCaseAnalysis>>>;

pub async fn analyze_case(
    data: web::Json<AnalysisRequest>,
    shared_analysis: web::Data<SharedAnalysis>,
) -> Result<HttpResponse> {
    let start_time = std::time::Instant::now();
    
    println!("🔍 Starting case analysis...");
    
    match CaseAnalyzer::new() {
        Ok(analyzer) => {
            match analyzer.analyze_full_case(data.user_overrides.clone()).await {
                Ok(analysis) => {
                    let processing_time = start_time.elapsed().as_secs_f64();
                    
                    // Store analysis in shared state
                    {
                        let mut shared = shared_analysis.lock().await;
                        *shared = Some(analysis.clone());
                    }
                    
                    println!("✅ Case analysis completed in {:.2} seconds", processing_time);
                    
                    Ok(HttpResponse::Ok().json(AnalysisResponse {
                        success: true,
                        analysis: Some(analysis),
                        error: None,
                        processing_time_seconds: processing_time,
                    }))
                },
                Err(e) => {
                    eprintln!("❌ Analysis error: {}", e);
                    Ok(HttpResponse::InternalServerError().json(AnalysisResponse {
                        success: false,
                        analysis: None,
                        error: Some(e.to_string()),
                        processing_time_seconds: start_time.elapsed().as_secs_f64(),
                    }))
                }
            }
        },
        Err(e) => {
            eprintln!("❌ Failed to create analyzer: {}", e);
            Ok(HttpResponse::InternalServerError().json(AnalysisResponse {
                success: false,
                analysis: None,
                error: Some(format!("Failed to initialize analyzer: {}", e)),
                processing_time_seconds: start_time.elapsed().as_secs_f64(),
            }))
        }
    }
}

pub async fn get_analysis_status(
    shared_analysis: web::Data<SharedAnalysis>,
) -> Result<HttpResponse> {
    let analysis_guard = shared_analysis.lock().await;
    
    if let Some(analysis) = analysis_guard.as_ref() {
        let file_watcher = crate::ai::create_ocr_file_watcher();
        let (monitored_files_count, file_watcher_enabled) = match file_watcher {
            Ok(watcher) => (watcher.get_file_count() as u32, watcher.is_enabled()),
            Err(_) => (0, false),
        };

        Ok(HttpResponse::Ok().json(StatusResponse {
            analysis_status: "completed".to_string(),
            last_analysis_date: Some(analysis.analysis_date.clone()),
            total_cases: analysis.cases.len() as u32,
            total_recovery_huf: analysis.total_recovery,
            monitored_files_count,
            file_watcher_enabled,
        }))
    } else {
        Ok(HttpResponse::Ok().json(StatusResponse {
            analysis_status: "not_analyzed".to_string(),
            last_analysis_date: None,
            total_cases: 0,
            total_recovery_huf: 0.0,
            monitored_files_count: 0,
            file_watcher_enabled: false,
        }))
    }
}

pub async fn generate_documents(
    data: web::Json<DocumentGenerationRequest>,
    shared_analysis: web::Data<SharedAnalysis>,
) -> Result<HttpResponse> {
    let analysis_guard = shared_analysis.lock().await;
    
    if let Some(analysis) = analysis_guard.as_ref() {
        match CaseAnalyzer::new() {
            Ok(analyzer) => {
                match analyzer.generate_legal_documents(analysis, &data.document_types).await {
                    Ok(documents) => {
                        println!("📄 Generated {} legal documents", documents.len());
                        Ok(HttpResponse::Ok().json(DocumentGenerationResponse {
                            success: true,
                            documents,
                            error: None,
                        }))
                    },
                    Err(e) => {
                        eprintln!("❌ Document generation error: {}", e);
                        Ok(HttpResponse::InternalServerError().json(DocumentGenerationResponse {
                            success: false,
                            documents: vec![],
                            error: Some(e.to_string()),
                        }))
                    }
                }
            },
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(DocumentGenerationResponse {
                    success: false,
                    documents: vec![],
                    error: Some(format!("Failed to initialize analyzer: {}", e)),
                }))
            }
        }
    } else {
        Ok(HttpResponse::BadRequest().json(DocumentGenerationResponse {
            success: false,
            documents: vec![],
            error: Some("No case analysis available. Please analyze case first.".to_string()),
        }))
    }
}

pub async fn apply_user_override(
    data: web::Json<OverrideRequest>,
    shared_analysis: web::Data<SharedAnalysis>,
) -> Result<HttpResponse> {
    let mut analysis_guard = shared_analysis.lock().await;
    
    if let Some(ref mut analysis) = analysis_guard.as_mut() {
        let override_item = UserOverride {
            field_path: data.field_path.clone(),
            original_value: "".to_string(), // Will be filled by the analyzer
            user_value: data.new_value.clone(),
            timestamp: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
            reason: data.reason.clone(),
        };

        // Apply the override
        analysis.user_overrides.push(override_item);
        
        // Trigger reanalysis with the new override
        drop(analysis_guard); // Release the lock
        
        match CaseAnalyzer::new() {
            Ok(analyzer) => {
                let analysis_guard = shared_analysis.lock().await;
                if let Some(current_analysis) = analysis_guard.as_ref() {
                    let overrides = current_analysis.user_overrides.clone();
                    drop(analysis_guard); // Release lock before calling analyze
                    
                    match analyzer.analyze_full_case(Some(overrides)).await {
                        Ok(new_analysis) => {
                            let mut analysis_guard = shared_analysis.lock().await;
                            *analysis_guard = Some(new_analysis);
                            
                            Ok(HttpResponse::Ok().json(serde_json::json!({
                                "success": true,
                                "message": "Override applied and analysis updated"
                            })))
                        },
                        Err(e) => {
                            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                                "success": false,
                                "error": format!("Failed to reanalyze after override: {}", e)
                            })))
                        }
                    }
                } else {
                    Ok(HttpResponse::BadRequest().json(serde_json::json!({
                        "success": false,
                        "error": "No analysis available"
                    })))
                }
            },
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "success": false,
                    "error": format!("Failed to initialize analyzer: {}", e)
                })))
            }
        }
    } else {
        Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": "No case analysis available. Please analyze case first."
        })))
    }
}

pub async fn get_current_analysis(
    shared_analysis: web::Data<SharedAnalysis>,
) -> Result<HttpResponse> {
    let analysis_guard = shared_analysis.lock().await;
    
    if let Some(analysis) = analysis_guard.as_ref() {
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "analysis": analysis
        })))
    } else {
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": false,
            "message": "No analysis available"
        })))
    }
}