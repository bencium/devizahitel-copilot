use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ClausePattern {
    pub id: Uuid,
    pub name: String,
    pub pattern_type: String, // regex, keyword, semantic
    pub pattern_text: String,
    pub language: String,
    pub clause_category: String, // fx_risk, interest_rate, penalty, etc.
    pub severity: String, // informational, warning, critical
    pub description: String,
    pub legal_basis: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClauseExtractionRequest {
    pub document_id: Uuid,
    pub language: Option<String>,
    pub extraction_method: Option<String>, // regex, ml, hybrid
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClauseAnalysisResult {
    pub extracted_clauses: Vec<AnalyzedClause>,
    pub risk_assessment: RiskAssessment,
    pub recommendations: Vec<String>,
    pub precedent_matches: Vec<PrecedentMatch>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzedClause {
    pub clause: crate::models::ExtractedClause,
    pub analysis: ClauseAnalysis,
    pub related_precedents: Vec<Uuid>, // case IDs
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClauseAnalysis {
    pub unfairness_score: f32,
    pub transparency_score: f32,
    pub consumer_detriment_score: f32,
    pub eu_compliance_issues: Vec<String>,
    pub hungarian_law_issues: Vec<String>,
    pub suggested_challenges: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub overall_risk_level: String,
    pub fx_risk_exposure: String,
    pub consumer_protection_gaps: Vec<String>,
    pub litigation_prospects: String,
    pub estimated_success_probability: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrecedentMatch {
    pub case_id: Uuid,
    pub similarity_score: f32,
    pub matching_elements: Vec<String>,
    pub applicability_assessment: String,
}

impl ClausePattern {
    pub fn new_fx_risk_pattern() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Foreign Currency Risk Clause".to_string(),
            pattern_type: "regex".to_string(),
            pattern_text: r"(?i)(deviza|foreign\s+currency|árfolyam|exchange\s+rate|currency\s+risk|CHF|švýcarský\s+frank)".to_string(),
            language: "multilingual".to_string(),
            clause_category: "fx_risk".to_string(),
            severity: "critical".to_string(),
            description: "Clauses relating to foreign currency exchange rate risk".to_string(),
            legal_basis: Some("EU Directive 93/13/EEC on unfair terms".to_string()),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn new_transparency_pattern() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Information Disclosure Requirements".to_string(),
            pattern_type: "keyword".to_string(),
            pattern_text: "tájékoztatás|information|disclosure|warning|risk|figyelmeztetés".to_string(),
            language: "multilingual".to_string(),
            clause_category: "transparency".to_string(),
            severity: "warning".to_string(),
            description: "Clauses related to information disclosure and transparency requirements".to_string(),
            legal_basis: Some("CJEU Andriciuc v. Banca Românească".to_string()),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn get_default_patterns() -> Vec<Self> {
        vec![
            Self::new_fx_risk_pattern(),
            Self::new_transparency_pattern(),
            // Interest rate clauses
            Self {
                id: Uuid::new_v4(),
                name: "Interest Rate Variation Clause".to_string(),
                pattern_type: "regex".to_string(),
                pattern_text: r"(?i)(kamat|interest\s+rate|úrok|změna\s+úroku|rate\s+change)".to_string(),
                language: "multilingual".to_string(),
                clause_category: "interest_rate".to_string(),
                severity: "warning".to_string(),
                description: "Clauses allowing unilateral interest rate changes".to_string(),
                legal_basis: Some("EU consumer protection directives".to_string()),
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            // Penalty clauses
            Self {
                id: Uuid::new_v4(),
                name: "Penalty and Fee Clauses".to_string(),
                pattern_type: "keyword".to_string(),
                pattern_text: "penalty|fee|költség|díj|sankce|poplatek|fine".to_string(),
                language: "multilingual".to_string(),
                clause_category: "penalty".to_string(),
                severity: "medium".to_string(),
                description: "Clauses imposing penalties or additional fees".to_string(),
                legal_basis: Some("Unfair Terms Directive".to_string()),
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        ]
    }
}

impl ClauseAnalysis {
    pub fn new() -> Self {
        Self {
            unfairness_score: 0.0,
            transparency_score: 0.0,
            consumer_detriment_score: 0.0,
            eu_compliance_issues: Vec::new(),
            hungarian_law_issues: Vec::new(),
            suggested_challenges: Vec::new(),
        }
    }

    pub fn calculate_scores(&mut self, clause: &crate::models::ExtractedClause) {
        // Calculate unfairness based on FX risk and lack of transparency
        if clause.is_fx_risk_clause() {
            self.unfairness_score = 0.9; // Very high for FX risk clauses
            self.consumer_detriment_score = 0.95;
            
            // Check for transparency issues
            let transparency_indicators = ["warning", "risk", "tájékoztatás", "figyelmeztetés"];
            let has_warnings = transparency_indicators.iter().any(|&indicator| {
                clause.clause_text.to_lowercase().contains(&indicator.to_lowercase())
            });
            
            self.transparency_score = if has_warnings { 0.6 } else { 0.1 };
            
            // Add compliance issues
            if !has_warnings {
                self.eu_compliance_issues.push("Insufficient warning about currency risk (Andriciuc v. Banca Românească)".to_string());
                self.hungarian_law_issues.push("Violation of transparency requirements under Hungarian consumer protection law".to_string());
                self.suggested_challenges.push("Challenge under EU Directive 93/13/EEC for lack of transparency".to_string());
                self.suggested_challenges.push("Cite CJEU precedents on FX risk disclosure requirements".to_string());
            }
        }
    }
}