use actix_web::{web, HttpResponse, Result};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::{Document, DocumentUploadRequest, DocumentProcessingResult};
use crate::extractors::{TextProcessor, LanguageDetector, ClauseExtractor};
use crate::db;
use serde_json::json;

pub async fn upload_document(
    pool: web::Data<PgPool>,
    request: web::Json<DocumentUploadRequest>,
) -> Result<HttpResponse> {
    let pool = pool.get_ref();
    let processor = TextProcessor::new();
    let language_detector = LanguageDetector::new();
    let extractor = ClauseExtractor::new();

    // Create document record
    let mut document = Document::new(request.into_inner());
    
    // Process the uploaded file
    match processor.process_document(&document.filename, &document.content_type).await {
        Ok(processing_result) => {
            document.extracted_text = Some(processing_result.extracted_text.clone());
            document.processing_status = "processing".to_string();
            
            // Detect language if not provided
            if document.language == "unknown" {
                let detection = language_detector.detect_language(&processing_result.extracted_text);
                document.language = detection.language;
            }
            
            // Save document to database
            match db::documents::insert_document(pool, document.clone()).await {
                Ok(saved_doc) => {
                    // Extract clauses asynchronously
                    if let Some(text) = &saved_doc.extracted_text {
                        let clause_result = extractor.extract_clauses(
                            saved_doc.id, 
                            text, 
                            &saved_doc.language
                        ).await;
                        
                        // Save extracted clauses
                        for clause in &clause_result.clauses {
                            if let Err(e) = db::clauses::insert_extracted_clause(pool, clause.clone()).await {
                                log::error!("Failed to save clause: {}", e);
                            }
                        }
                        
                        // Update document status
                        let _ = db::documents::update_document_status(
                            pool, 
                            saved_doc.id, 
                            "completed"
                        ).await;
                        
                        let response = DocumentProcessingResult {
                            document_id: saved_doc.id,
                            extracted_text: text.clone(),
                            extracted_clauses: clause_result.clauses,
                            language_detected: clause_result.language_detected,
                            confidence_score: clause_result.confidence,
                        };
                        
                        Ok(HttpResponse::Ok().json(response))
                    } else {
                        Ok(HttpResponse::Ok().json(saved_doc))
                    }
                },
                Err(e) => {
                    log::error!("Failed to save document: {}", e);
                    Ok(HttpResponse::InternalServerError().json(json!({
                        "error": "Failed to save document",
                        "details": e.to_string()
                    })))
                }
            }
        },
        Err(e) => {
            // Update document with error status
            document.processing_status = "error".to_string();
            document.error_message = Some(e.to_string());
            
            if let Ok(saved_doc) = db::documents::insert_document(pool, document).await {
                Ok(HttpResponse::BadRequest().json(json!({
                    "error": "Document processing failed",
                    "document_id": saved_doc.id,
                    "details": e.to_string()
                })))
            } else {
                Ok(HttpResponse::InternalServerError().json(json!({
                    "error": "Document processing and saving failed",
                    "details": e.to_string()
                })))
            }
        }
    }
}

pub async fn get_documents(
    pool: web::Data<PgPool>,
    query: web::Query<DocumentQuery>,
) -> Result<HttpResponse> {
    let pool = pool.get_ref();
    
    match db::documents::get_documents(pool, query.limit, query.offset, query.status.as_deref()).await {
        Ok(documents) => Ok(HttpResponse::Ok().json(documents)),
        Err(e) => {
            log::error!("Failed to fetch documents: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to fetch documents",
                "details": e.to_string()
            })))
        }
    }
}

pub async fn get_document(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let pool = pool.get_ref();
    let document_id = path.into_inner();
    
    match db::documents::get_document_by_id(pool, document_id).await {
        Ok(Some(document)) => {
            // Also fetch extracted clauses
            match db::clauses::get_clauses_by_document(pool, document_id).await {
                Ok(clauses) => {
                    Ok(HttpResponse::Ok().json(json!({
                        "document": document,
                        "extracted_clauses": clauses
                    })))
                },
                Err(e) => {
                    log::error!("Failed to fetch clauses for document {}: {}", document_id, e);
                    Ok(HttpResponse::Ok().json(json!({
                        "document": document,
                        "extracted_clauses": []
                    })))
                }
            }
        },
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "error": "Document not found"
        }))),
        Err(e) => {
            log::error!("Failed to fetch document {}: {}", document_id, e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to fetch document",
                "details": e.to_string()
            })))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct DocumentQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub status: Option<String>,
}