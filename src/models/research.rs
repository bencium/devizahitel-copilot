use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ResearchSheet {
    pub id: Uuid,
    pub document_id: Uuid,
    pub case_reference: Option<String>,
    pub client_name: Option<String>,
    pub generated_at: DateTime<Utc>,
    pub analysis_summary: String,
    pub legal_findings: serde_json::Value,
    pub precedent_citations: serde_json::Value,
    pub recommended_actions: serde_json::Value,
    pub draft_pleading: Option<String>,
    pub confidence_score: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResearchRequest {
    pub document_id: Uuid,
    pub research_focus: Vec<String>, // fx_risk, transparency, unfair_terms, etc.
    pub jurisdiction: String,
    pub case_type: String, // complaint, appeal, constitutional_challenge
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LegalFindings {
    pub identified_issues: Vec<LegalIssue>,
    pub applicable_precedents: Vec<ApplicablePrecedent>,
    pub legal_arguments: Vec<LegalArgument>,
    pub evidence_requirements: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LegalIssue {
    pub issue_type: String,
    pub description: String,
    pub severity: String,
    pub legal_basis: Vec<String>,
    pub supporting_clauses: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicablePrecedent {
    pub case_id: Uuid,
    pub case_number: String,
    pub case_name: String,
    pub jurisdiction: String,
    pub relevance_score: f32,
    pub key_principles: Vec<String>,
    pub citation_text: String,
    pub application_notes: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LegalArgument {
    pub argument_type: String, // primary, secondary, constitutional
    pub title: String,
    pub description: String,
    pub supporting_precedents: Vec<Uuid>,
    pub legal_reasoning: String,
    pub strength_assessment: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DraftPleadingRequest {
    pub research_sheet_id: Uuid,
    pub pleading_type: String, // complaint, motion, appeal
    pub court: String,
    pub style_preference: String, // formal, academic, aggressive
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedPleading {
    pub id: Uuid,
    pub research_sheet_id: Uuid,
    pub pleading_type: String,
    pub title: String,
    pub content: String,
    pub citations: Vec<Citation>,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Citation {
    pub case_id: Uuid,
    pub citation_format: String,
    pub full_citation: String,
    pub paragraph_reference: Option<String>,
    pub page_reference: Option<String>,
}

impl ResearchSheet {
    pub fn new(document_id: Uuid, case_reference: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            document_id,
            case_reference,
            client_name: None,
            generated_at: now,
            analysis_summary: String::new(),
            legal_findings: serde_json::json!({}),
            precedent_citations: serde_json::json!([]),
            recommended_actions: serde_json::json!([]),
            draft_pleading: None,
            confidence_score: 0.0,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_legal_findings(&mut self, findings: LegalFindings) {
        self.legal_findings = serde_json::to_value(&findings).unwrap_or_default();
        self.update_confidence_score();
    }

    pub fn add_precedent_citations(&mut self, precedents: Vec<ApplicablePrecedent>) {
        self.precedent_citations = serde_json::to_value(&precedents).unwrap_or_default();
        self.update_confidence_score();
    }

    fn update_confidence_score(&mut self) {
        let findings_weight = if !self.legal_findings.is_null() { 0.4 } else { 0.0 };
        let precedents_weight = if !self.precedent_citations.is_null() { 0.6 } else { 0.0 };
        
        self.confidence_score = findings_weight + precedents_weight;
        self.updated_at = Utc::now();
    }
}

impl LegalFindings {
    pub fn new() -> Self {
        Self {
            identified_issues: Vec::new(),
            applicable_precedents: Vec::new(),
            legal_arguments: Vec::new(),
            evidence_requirements: Vec::new(),
        }
    }

    pub fn add_fx_risk_issue(&mut self, clause_ids: Vec<Uuid>) {
        let issue = LegalIssue {
            issue_type: "fx_risk_unfair_term".to_string(),
            description: "Foreign currency risk clauses that place disproportionate risk on consumer without adequate disclosure".to_string(),
            severity: "critical".to_string(),
            legal_basis: vec![
                "EU Directive 93/13/EEC on unfair terms".to_string(),
                "CJEU Andriciuc v. Banca Românească (C-186/16)".to_string(),
                "CJEU Kásler v. OTP Bank (C-26/13)".to_string(),
            ],
            supporting_clauses: clause_ids,
        };
        self.identified_issues.push(issue);
    }

    pub fn add_transparency_issue(&mut self, clause_ids: Vec<Uuid>) {
        let issue = LegalIssue {
            issue_type: "transparency_violation".to_string(),
            description: "Lack of clear and intelligible information about currency risk".to_string(),
            severity: "high".to_string(),
            legal_basis: vec![
                "EU Directive 93/13/EEC Article 5".to_string(),
                "CJEU requirement for 'sufficient information for prudent decision'".to_string(),
            ],
            supporting_clauses: clause_ids,
        };
        self.identified_issues.push(issue);
    }
}

impl GeneratedPleading {
    pub fn generate_fx_mortgage_complaint(
        research_sheet_id: Uuid,
        findings: &LegalFindings,
        precedents: &[ApplicablePrecedent],
    ) -> Self {
        let content = Self::build_complaint_content(findings, precedents);
        let citations = Self::extract_citations(precedents);

        Self {
            id: Uuid::new_v4(),
            research_sheet_id,
            pleading_type: "complaint".to_string(),
            title: "Complaint for Unfair Foreign Currency Mortgage Terms".to_string(),
            content,
            citations,
            generated_at: Utc::now(),
        }
    }

    fn build_complaint_content(findings: &LegalFindings, precedents: &[ApplicablePrecedent]) -> String {
        let mut content = String::new();
        
        content.push_str("COMPLAINT FOR UNFAIR FOREIGN CURRENCY MORTGAGE TERMS\n\n");
        content.push_str("I. FACTUAL BACKGROUND\n\n");
        content.push_str("The Plaintiff entered into a foreign currency mortgage agreement with the Defendant bank, ");
        content.push_str("which contained unfair terms placing disproportionate currency exchange risk on the consumer ");
        content.push_str("without adequate disclosure or warning of the risks involved.\n\n");
        
        content.push_str("II. LEGAL ARGUMENTS\n\n");
        
        for (i, issue) in findings.identified_issues.iter().enumerate() {
            content.push_str(&format!("{}. {}\n", i + 1, issue.description));
            content.push_str(&format!("Legal Basis: {}\n\n", issue.legal_basis.join("; ")));
        }
        
        content.push_str("III. SUPPORTING PRECEDENTS\n\n");
        
        for precedent in precedents {
            content.push_str(&format!(
                "In {}, the Court ruled: {}\n\n",
                precedent.citation_text,
                precedent.key_principles.join("; ")
            ));
        }
        
        content.push_str("IV. RELIEF REQUESTED\n\n");
        content.push_str("WHEREFORE, Plaintiff respectfully requests that this Court:\n");
        content.push_str("1. Declare the foreign currency clauses unfair and void under EU Directive 93/13/EEC;\n");
        content.push_str("2. Order full restitution of all payments made under the invalid contract;\n");
        content.push_str("3. Award damages for harm suffered due to the unfair terms;\n");
        content.push_str("4. Grant such other relief as the Court deems just and proper.\n");
        
        content
    }

    fn extract_citations(precedents: &[ApplicablePrecedent]) -> Vec<Citation> {
        precedents.iter().map(|p| Citation {
            case_id: p.case_id,
            citation_format: "CJEU".to_string(),
            full_citation: p.citation_text.clone(),
            paragraph_reference: None,
            page_reference: None,
        }).collect()
    }
}