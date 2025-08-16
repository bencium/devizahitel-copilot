use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Document {
    pub id: Uuid,
    pub filename: String,
    pub content_type: String,
    pub file_size: i64,
    pub original_text: Option<String>,
    pub extracted_text: Option<String>,
    pub document_type: String, // mortgage_contract, loan_agreement, correspondence, etc.
    pub language: String,
    pub client_id: Option<String>,
    pub case_reference: Option<String>,
    pub processing_status: String, // uploaded, processing, completed, error
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentUploadRequest {
    pub filename: String,
    pub content_type: String,
    pub file_data: String, // base64 encoded
    pub document_type: String,
    pub language: Option<String>,
    pub client_id: Option<String>,
    pub case_reference: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentProcessingResult {
    pub document_id: Uuid,
    pub extracted_text: String,
    pub extracted_clauses: Vec<ExtractedClause>,
    pub language_detected: String,
    pub confidence_score: f32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ExtractedClause {
    pub id: Uuid,
    pub document_id: Uuid,
    pub clause_type: String,
    pub clause_text: String,
    pub original_language: String,
    pub english_translation: Option<String>,
    pub start_position: Option<i32>,
    pub end_position: Option<i32>,
    pub confidence_score: f32,
    pub risk_level: String, // low, medium, high, critical
    pub created_at: DateTime<Utc>,
}

impl Document {
    pub fn new(request: DocumentUploadRequest) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            filename: request.filename,
            content_type: request.content_type,
            file_size: 0, // Will be calculated after processing
            original_text: None,
            extracted_text: None,
            document_type: request.document_type,
            language: request.language.unwrap_or_else(|| "unknown".to_string()),
            client_id: request.client_id,
            case_reference: request.case_reference,
            processing_status: "uploaded".to_string(),
            error_message: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_mortgage_contract(&self) -> bool {
        matches!(self.document_type.as_str(), "mortgage_contract" | "loan_agreement")
    }

    pub fn supports_ocr(&self) -> bool {
        matches!(self.content_type.as_str(), "application/pdf" | "image/jpeg" | "image/png")
    }
}

impl ExtractedClause {
    pub fn new(
        document_id: Uuid,
        clause_type: String,
        clause_text: String,
        language: String,
        confidence: f32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            document_id,
            clause_type,
            clause_text,
            original_language: language,
            english_translation: None,
            start_position: None,
            end_position: None,
            confidence_score: confidence,
            risk_level: "medium".to_string(),
            created_at: Utc::now(),
        }
    }

    pub fn is_fx_risk_clause(&self) -> bool {
        let fx_patterns = [
            "foreign currency", "deviza", "árfolyam", "exchange rate", 
            "currency risk", "CHF", "EUR", "USD", "waluta", "kurs"
        ];
        
        fx_patterns.iter().any(|&pattern| {
            self.clause_text.to_lowercase().contains(&pattern.to_lowercase())
        })
    }

    pub fn calculate_risk_level(&mut self) {
        if self.is_fx_risk_clause() && self.confidence_score > 0.8 {
            self.risk_level = "critical".to_string();
        } else if self.is_fx_risk_clause() && self.confidence_score > 0.6 {
            self.risk_level = "high".to_string();
        } else if self.confidence_score > 0.7 {
            self.risk_level = "medium".to_string();
        } else {
            self.risk_level = "low".to_string();
        }
    }
}