use actix_web::{web, HttpResponse, Result};
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::models::{ClauseExtractionRequest, DraftPleadingRequest, ResearchRequest, ResearchSheet, LegalFindings, GeneratedPleading};
use crate::extractors::{ClauseExtractor, LanguageDetector};
use crate::matching::PrecedentMatcher;
use crate::db;
use serde_json::json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ExtractClausesRequest {
    pub document_id: Uuid,
    pub language: Option<String>,
    pub extraction_method: Option<String>,
}

#[derive(Deserialize)]
pub struct MatchPrecedentsRequest {
    pub document_id: Uuid,
    pub research_focus: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct GenerateDraftRequest {
    pub document_id: Uuid,
    pub pleading_type: String,
    pub court: String,
    pub style_preference: Option<String>,
}

pub async fn extract_clauses(
    pool: web::Data<SqlitePool>,
    request: web::Json<ExtractClausesRequest>,
) -> Result<HttpResponse> {
    let pool = pool.get_ref();
    let extractor = ClauseExtractor::new();
    let language_detector = LanguageDetector::new();

    // Get the document
    match db::documents::get_document_by_id(pool, request.document_id).await {
        Ok(Some(document)) => {
            if let Some(text) = &document.extracted_text {
                // Detect language if not provided
                let language = request.language.as_deref().unwrap_or(&document.language);
                let detected_language = if language == "unknown" {
                    language_detector.detect_language(text).language
                } else {
                    language.to_string()
                };

                // Extract clauses
                let extraction_result = extractor.extract_clauses(
                    document.id, 
                    text, 
                    &detected_language
                ).await;

                // Save extracted clauses to database
                for clause in &extraction_result.clauses {
                    if let Err(e) = db::clauses::insert_extracted_clause(pool, clause.clone()).await {
                        log::error!("Failed to save clause: {}", e);
                    }
                }

                Ok(HttpResponse::Ok().json(json!({
                    "document_id": document.id,
                    "extracted_clauses": extraction_result.clauses,
                    "language_detected": extraction_result.language_detected,
                    "confidence_score": extraction_result.confidence,
                    "total_clauses": extraction_result.clauses.len()
                })))
            } else {
                Ok(HttpResponse::BadRequest().json(json!({
                    "error": "Document has no extracted text available"
                })))
            }
        },
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(json!({
                "error": "Document not found"
            })))
        },
        Err(e) => {
            log::error!("Failed to fetch document: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to fetch document",
                "details": e.to_string()
            })))
        }
    }
}

pub async fn match_precedents(
    pool: web::Data<SqlitePool>,
    request: web::Json<MatchPrecedentsRequest>,
) -> Result<HttpResponse> {
    let pool = pool.get_ref();
    let matcher = PrecedentMatcher::new();

    // Get extracted clauses for the document
    match db::clauses::get_clauses_by_document(pool, request.document_id).await {
        Ok(clauses) => {
            if clauses.is_empty() {
                return Ok(HttpResponse::BadRequest().json(json!({
                    "error": "No clauses found for this document. Please extract clauses first."
                })));
            }

            // Match precedents
            match matcher.match_precedents(pool, &clauses).await {
                Ok(matching_result) => {
                    // Create applicable precedents for storage
                    let applicable_precedents = matcher.create_applicable_precedents(&matching_result.overall_case_matches);

                    // Create research sheet if it doesn't exist
                    let research_sheet = match db::research::get_research_sheet_by_document(pool, request.document_id).await {
                        Ok(Some(sheet)) => sheet,
                        Ok(None) => {
                            let mut new_sheet = ResearchSheet::new(request.document_id, None);
                            
                            // Create legal findings
                            let mut findings = LegalFindings::new();
                            
                            // Add FX risk issues
                            let fx_clauses: Vec<Uuid> = clauses.iter()
                                .filter(|c| c.is_fx_risk_clause())
                                .map(|c| c.id)
                                .collect();
                            if !fx_clauses.is_empty() {
                                findings.add_fx_risk_issue(fx_clauses);
                            }

                            // Add transparency issues
                            let transparency_clauses: Vec<Uuid> = clauses.iter()
                                .filter(|c| c.clause_type == "transparency")
                                .map(|c| c.id)
                                .collect();
                            if !transparency_clauses.is_empty() {
                                findings.add_transparency_issue(transparency_clauses);
                            }

                            new_sheet.add_legal_findings(findings);
                            new_sheet.add_precedent_citations(applicable_precedents.clone());

                            db::research::insert_research_sheet(pool, new_sheet).await.unwrap_or_else(|_| {
                                ResearchSheet::new(request.document_id, None)
                            })
                        },
                        Err(_) => ResearchSheet::new(request.document_id, None),
                    };

                    Ok(HttpResponse::Ok().json(json!({
                        "document_id": request.document_id,
                        "research_sheet_id": research_sheet.id,
                        "clause_matches": matching_result.clause_matches,
                        "overall_case_matches": matching_result.overall_case_matches,
                        "confidence_score": matching_result.confidence_score,
                        "applicable_precedents": applicable_precedents,
                        "summary": format!(
                            "Found {} relevant precedents across {} analyzed clauses with {:.1}% confidence",
                            matching_result.overall_case_matches.len(),
                            matching_result.clause_matches.len(),
                            matching_result.confidence_score * 100.0
                        )
                    })))
                },
                Err(e) => {
                    log::error!("Failed to match precedents: {}", e);
                    Ok(HttpResponse::InternalServerError().json(json!({
                        "error": "Failed to match precedents",
                        "details": e.to_string()
                    })))
                }
            }
        },
        Err(e) => {
            log::error!("Failed to fetch clauses: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to fetch clauses for document",
                "details": e.to_string()
            })))
        }
    }
}

pub async fn generate_draft(
    pool: web::Data<SqlitePool>,
    request: web::Json<GenerateDraftRequest>,
) -> Result<HttpResponse> {
    let pool = pool.get_ref();

    // Get research sheet for the document
    match db::research::get_research_sheet_by_document(pool, request.document_id).await {
        Ok(Some(research_sheet)) => {
            // Parse legal findings and precedent citations
            let legal_findings: LegalFindings = serde_json::from_value(research_sheet.legal_findings.clone())
                .unwrap_or_else(|_| LegalFindings::new());
            
            let applicable_precedents: Vec<crate::models::ApplicablePrecedent> = 
                serde_json::from_value(research_sheet.precedent_citations.clone())
                .unwrap_or_default();

            // Generate the pleading
            let generated_pleading = GeneratedPleading::generate_fx_mortgage_complaint(
                research_sheet.id,
                &legal_findings,
                &applicable_precedents,
            );

            // Save the generated pleading
            match db::research::insert_generated_pleading(pool, generated_pleading.clone()).await {
                Ok(_) => {
                    Ok(HttpResponse::Ok().json(json!({
                        "pleading_id": generated_pleading.id,
                        "research_sheet_id": research_sheet.id,
                        "title": generated_pleading.title,
                        "content": generated_pleading.content,
                        "citations": generated_pleading.citations,
                        "generated_at": generated_pleading.generated_at,
                        "metadata": {
                            "pleading_type": request.pleading_type,
                            "court": request.court,
                            "style_preference": request.style_preference,
                            "legal_issues_count": legal_findings.identified_issues.len(),
                            "precedents_cited": applicable_precedents.len()
                        }
                    })))
                },
                Err(e) => {
                    log::error!("Failed to save generated pleading: {}", e);
                    // Still return the generated content even if saving fails
                    Ok(HttpResponse::Ok().json(json!({
                        "pleading_id": generated_pleading.id,
                        "research_sheet_id": research_sheet.id,
                        "title": generated_pleading.title,
                        "content": generated_pleading.content,
                        "citations": generated_pleading.citations,
                        "generated_at": generated_pleading.generated_at,
                        "warning": "Pleading generated but not saved to database"
                    })))
                }
            }
        },
        Ok(None) => {
            Ok(HttpResponse::BadRequest().json(json!({
                "error": "No research sheet found for this document. Please run precedent matching first."
            })))
        },
        Err(e) => {
            log::error!("Failed to fetch research sheet: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to fetch research data",
                "details": e.to_string()
            })))
        }
    }
}

pub async fn get_research_sheet(
    pool: web::Data<SqlitePool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let pool = pool.get_ref();
    let document_id = path.into_inner();

    match db::research::get_research_sheet_by_document(pool, document_id).await {
        Ok(Some(research_sheet)) => {
            // Also get the associated document and clauses
            let document = db::documents::get_document_by_id(pool, document_id).await.ok().flatten();
            let clauses = db::clauses::get_clauses_by_document(pool, document_id).await.unwrap_or_default();
            let pleadings = db::research::get_pleadings_by_research_sheet(pool, research_sheet.id).await.unwrap_or_default();

            Ok(HttpResponse::Ok().json(json!({
                "research_sheet": research_sheet,
                "document": document,
                "extracted_clauses": clauses,
                "generated_pleadings": pleadings,
                "analysis_summary": {
                    "total_clauses": clauses.len(),
                    "critical_clauses": clauses.iter().filter(|c| c.risk_level == "critical").count(),
                    "high_risk_clauses": clauses.iter().filter(|c| c.risk_level == "high").count(),
                    "fx_risk_clauses": clauses.iter().filter(|c| c.is_fx_risk_clause()).count(),
                    "confidence_score": research_sheet.confidence_score,
                    "pleadings_generated": pleadings.len()
                }
            })))
        },
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(json!({
                "error": "Research sheet not found for this document"
            })))
        },
        Err(e) => {
            log::error!("Failed to fetch research sheet: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to fetch research sheet",
                "details": e.to_string()
            })))
        }
    }
}