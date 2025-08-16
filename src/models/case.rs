use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LegalCase {
    pub id: Uuid,
    pub case_number: String,
    pub case_name: String,
    pub country: String,
    pub date: DateTime<Utc>,
    pub currency: String,
    pub key_ruling: String,
    pub full_text: Option<String>,
    pub court: Option<String>,
    pub case_type: String, // CJEU, National Court, etc.
    pub significance_score: Option<f32>,
    pub embedding: Option<Vec<f32>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseCreateRequest {
    pub case_number: String,
    pub case_name: String,
    pub country: String,
    pub date: DateTime<Utc>,
    pub currency: String,
    pub key_ruling: String,
    pub full_text: Option<String>,
    pub court: Option<String>,
    pub case_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseSearchRequest {
    pub query: String,
    pub country: Option<String>,
    pub currency: Option<String>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub case_type: Option<String>,
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaseMatch {
    pub case: LegalCase,
    pub similarity_score: f32,
    pub matching_clauses: Vec<String>,
    pub relevance_explanation: String,
}

impl LegalCase {
    pub fn new(request: CaseCreateRequest) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            case_number: request.case_number,
            case_name: request.case_name,
            country: request.country,
            date: request.date,
            currency: request.currency,
            key_ruling: request.key_ruling,
            full_text: request.full_text,
            court: request.court,
            case_type: request.case_type,
            significance_score: None,
            embedding: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_foreign_currency_case(&self) -> bool {
        let fx_indicators = ["CHF", "EUR", "USD", "foreign currency", "deviza", "waluta"];
        fx_indicators.iter().any(|&indicator| {
            self.currency.contains(indicator) || 
            self.key_ruling.to_lowercase().contains(&indicator.to_lowercase()) ||
            self.case_name.to_lowercase().contains(&indicator.to_lowercase())
        })
    }

    pub fn get_jurisdiction(&self) -> String {
        if self.case_number.starts_with("C-") {
            "CJEU".to_string()
        } else {
            self.country.clone()
        }
    }
}