use super::mistral_client::{MistralClient, DocumentAnalysis, DamageCalculation, CaseData};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MultiCaseAnalysis {
    pub cases: Vec<CaseAnalysis>,
    pub total_recovery: f64,
    pub analysis_date: String,
    pub confidence_level: String,
    pub user_overrides: Vec<UserOverride>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CaseAnalysis {
    pub id: String,
    pub bank_name: String,
    pub loan_contracts: Vec<LoanContract>,
    pub payment_history: Vec<PaymentRecord>,
    pub correspondence: Vec<CorrespondenceRecord>,
    pub total_damages: f64,
    pub case_strength: String,
    pub recommended_actions: Vec<String>,
    pub legal_strategy: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoanContract {
    pub contract_id: String,
    pub bank_name: String,
    pub loan_type: String, // "mortgage", "personal_loan", "credit_line"
    pub currency: String,  // "CHF", "EUR", "USD", "HUF"
    pub original_amount: f64,
    pub start_date: String,
    pub end_date: Option<String>,
    pub fx_risk_disclosure: String, // "excellent", "good", "poor", "none"
    pub interest_rate: Option<f64>,
    pub key_unfair_clauses: Vec<String>,
    pub document_source: String, // filename where this was extracted
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentRecord {
    pub bank_name: String,
    pub currency: String,
    pub total_paid: f64,
    pub payment_period_start: String,
    pub payment_period_end: String,
    pub overpayment_amount: f64,
    pub exchange_rate_losses: f64,
    pub document_source: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CorrespondenceRecord {
    pub date: String,
    pub sender: String,
    pub recipient: String,
    pub document_type: String, // "complaint", "response", "notice", "settlement_offer"
    pub key_points: Vec<String>,
    pub legal_significance: String,
    pub document_source: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserOverride {
    pub field_path: String, // e.g., "cases[0].loan_contracts[0].original_amount"
    pub original_value: String,
    pub user_value: String,
    pub timestamp: String,
    pub reason: Option<String>,
}

pub struct CaseAnalyzer {
    mistral_client: MistralClient,
    ocr_dir: String,
    precedents_dir: String,
}

impl CaseAnalyzer {
    pub fn new() -> Result<Self> {
        let mistral_client = MistralClient::new()?;
        let ocr_dir = std::env::var("OCR_OUTPUT_DIR")
            .unwrap_or_else(|_| "./ocr_output".to_string());
        let precedents_dir = std::env::var("PRECEDENTS_DIR")
            .unwrap_or_else(|_| "./Precedents".to_string());

        Ok(Self {
            mistral_client,
            ocr_dir,
            precedents_dir,
        })
    }

    pub async fn analyze_full_case(&self, user_overrides: Option<Vec<UserOverride>>) -> Result<MultiCaseAnalysis> {
        println!("🔍 Starting comprehensive case analysis...");
        
        // 1. Read all documents from OCR output
        let documents = self.read_all_documents().await?;
        println!("📄 Found {} documents to analyze", documents.len());

        // 2. Use AI to analyze documents and extract case information
        let ai_analysis = self.mistral_client.analyze_documents(&documents).await?;
        println!("🤖 AI analysis completed");

        // 3. Convert AI analysis to structured case data
        let mut multi_case = self.structure_case_data(ai_analysis).await?;

        // 4. Apply user overrides if provided
        if let Some(overrides) = user_overrides {
            self.apply_user_overrides(&mut multi_case, overrides)?;
        }

        // 5. Calculate damages for each case
        for case in &mut multi_case.cases {
            self.calculate_case_damages(case).await?;
        }

        // 6. Calculate total recovery across all cases
        multi_case.total_recovery = multi_case.cases.iter()
            .map(|case| case.total_damages)
            .sum();

        // 7. Generate recommendations and strategy
        self.generate_case_strategy(&mut multi_case).await?;

        multi_case.analysis_date = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();

        println!("✅ Case analysis completed. Total recovery potential: {:.2} HUF", multi_case.total_recovery);
        Ok(multi_case)
    }

    async fn read_all_documents(&self) -> Result<Vec<String>> {
        let mut documents = Vec::new();
        let ocr_path = Path::new(&self.ocr_dir);

        if !ocr_path.exists() {
            return Err(anyhow!("OCR output directory not found: {}", self.ocr_dir));
        }

        for entry in fs::read_dir(ocr_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if matches!(extension.to_str(), Some("md") | Some("txt") | Some("rtf")) {
                        if let Ok(content) = fs::read_to_string(&path) {
                            let filename = path.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("unknown");
                            
                            documents.push(format!(
                                "DOCUMENT: {}\n\n{}\n\n",
                                filename,
                                content
                            ));
                        }
                    }
                }
            }
        }

        Ok(documents)
    }

    async fn structure_case_data(&self, ai_analysis: DocumentAnalysis) -> Result<MultiCaseAnalysis> {
        let mut cases = Vec::new();
        
        // Group contracts by bank to handle multiple banks
        let mut bank_groups: std::collections::HashMap<String, Vec<_>> = std::collections::HashMap::new();
        
        for contract in ai_analysis.contracts {
            bank_groups.entry(contract.bank_name.clone())
                .or_insert_with(Vec::new)
                .push(contract);
        }

        // Create a case for each bank
        for (bank_name, contracts) in bank_groups {
            let case_id = format!("case_{}_{}", 
                bank_name.to_lowercase().replace(" ", "_"),
                Utc::now().timestamp()
            );

            let loan_contracts: Vec<LoanContract> = contracts.into_iter()
                .map(|c| LoanContract {
                    contract_id: format!("{}_{}", c.bank_name, c.start_date),
                    bank_name: c.bank_name,
                    loan_type: c.contract_type,
                    currency: c.currency,
                    original_amount: c.original_amount,
                    start_date: c.start_date,
                    end_date: None,
                    fx_risk_disclosure: c.fx_risk_disclosure,
                    interest_rate: None,
                    key_unfair_clauses: c.key_clauses,
                    document_source: "ai_extracted".to_string(),
                })
                .collect();

            // Find payment records for this bank
            let payment_history: Vec<PaymentRecord> = ai_analysis.payment_statements.iter()
                .filter(|p| p.bank_name.eq_ignore_ascii_case(&bank_name))
                .map(|p| PaymentRecord {
                    bank_name: p.bank_name.clone(),
                    currency: p.currency.clone(),
                    total_paid: p.total_payments,
                    payment_period_start: p.payment_period.split(" to ").next().unwrap_or("").to_string(),
                    payment_period_end: p.payment_period.split(" to ").nth(1).unwrap_or("").to_string(),
                    overpayment_amount: p.total_payments * 0.3, // Estimate 30% overpayment
                    exchange_rate_losses: p.exchange_rate_losses,
                    document_source: "ai_extracted".to_string(),
                })
                .collect();

            // Find correspondence for this bank
            let correspondence: Vec<CorrespondenceRecord> = ai_analysis.correspondence.iter()
                .map(|c| CorrespondenceRecord {
                    date: c.date.clone(),
                    sender: bank_name.clone(),
                    recipient: "Client".to_string(),
                    document_type: c.doc_type.clone(),
                    key_points: c.key_points.clone(),
                    legal_significance: "To be analyzed".to_string(),
                    document_source: "ai_extracted".to_string(),
                })
                .collect();

            let case = CaseAnalysis {
                id: case_id,
                bank_name: bank_name.clone(),
                loan_contracts,
                payment_history,
                correspondence,
                total_damages: 0.0, // Will be calculated later
                case_strength: ai_analysis.case_strength.clone(),
                recommended_actions: vec![],
                legal_strategy: "".to_string(),
            };

            cases.push(case);
        }

        Ok(MultiCaseAnalysis {
            cases,
            total_recovery: 0.0,
            analysis_date: "".to_string(),
            confidence_level: "medium".to_string(),
            user_overrides: vec![],
        })
    }

    fn apply_user_overrides(&self, multi_case: &mut MultiCaseAnalysis, overrides: Vec<UserOverride>) -> Result<()> {
        for override_item in overrides {
            self.apply_single_override(multi_case, &override_item)?;
            multi_case.user_overrides.push(override_item);
        }
        Ok(())
    }

    fn apply_single_override(&self, multi_case: &mut MultiCaseAnalysis, override_item: &UserOverride) -> Result<()> {
        // Parse field path like "cases[0].loan_contracts[0].original_amount"
        let parts: Vec<&str> = override_item.field_path.split('.').collect();
        
        if parts.len() >= 3 {
            // Extract case index
            if let Some(case_idx_str) = parts[0].strip_prefix("cases[").and_then(|s| s.strip_suffix("]")) {
                if let Ok(case_idx) = case_idx_str.parse::<usize>() {
                    if case_idx < multi_case.cases.len() {
                        let case = &mut multi_case.cases[case_idx];
                        
                        // Handle different field types
                        match parts[1] {
                            s if s.starts_with("loan_contracts[") => {
                                if let Some(contract_idx_str) = s.strip_prefix("loan_contracts[").and_then(|s| s.strip_suffix("]")) {
                                    if let Ok(contract_idx) = contract_idx_str.parse::<usize>() {
                                        if contract_idx < case.loan_contracts.len() {
                                            self.update_contract_field(&mut case.loan_contracts[contract_idx], parts[2], &override_item.user_value)?;
                                        }
                                    }
                                }
                            },
                            s if s.starts_with("payment_history[") => {
                                if let Some(payment_idx_str) = s.strip_prefix("payment_history[").and_then(|s| s.strip_suffix("]")) {
                                    if let Ok(payment_idx) = payment_idx_str.parse::<usize>() {
                                        if payment_idx < case.payment_history.len() {
                                            self.update_payment_field(&mut case.payment_history[payment_idx], parts[2], &override_item.user_value)?;
                                        }
                                    }
                                }
                            },
                            _ => {
                                println!("⚠️ Unknown field path for override: {}", override_item.field_path);
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    fn update_contract_field(&self, contract: &mut LoanContract, field: &str, value: &str) -> Result<()> {
        match field {
            "original_amount" => {
                contract.original_amount = value.parse::<f64>()
                    .map_err(|_| anyhow!("Invalid number format for original_amount: {}", value))?;
            },
            "currency" => {
                contract.currency = value.to_string();
            },
            "bank_name" => {
                contract.bank_name = value.to_string();
            },
            "start_date" => {
                contract.start_date = value.to_string();
            },
            "fx_risk_disclosure" => {
                contract.fx_risk_disclosure = value.to_string();
            },
            _ => return Err(anyhow!("Unknown contract field: {}", field)),
        }
        Ok(())
    }

    fn update_payment_field(&self, payment: &mut PaymentRecord, field: &str, value: &str) -> Result<()> {
        match field {
            "total_paid" => {
                payment.total_paid = value.parse::<f64>()
                    .map_err(|_| anyhow!("Invalid number format for total_paid: {}", value))?;
            },
            "overpayment_amount" => {
                payment.overpayment_amount = value.parse::<f64>()
                    .map_err(|_| anyhow!("Invalid number format for overpayment_amount: {}", value))?;
            },
            "exchange_rate_losses" => {
                payment.exchange_rate_losses = value.parse::<f64>()
                    .map_err(|_| anyhow!("Invalid number format for exchange_rate_losses: {}", value))?;
            },
            _ => return Err(anyhow!("Unknown payment field: {}", field)),
        }
        Ok(())
    }

    async fn calculate_case_damages(&self, case: &mut CaseAnalysis) -> Result<()> {
        // Aggregate all loans for this bank
        let total_loan_amount: f64 = case.loan_contracts.iter()
            .map(|c| c.original_amount)
            .sum();
        
        let total_payments: f64 = case.payment_history.iter()
            .map(|p| p.total_paid)
            .sum();

        // Get primary currency (most common in contracts)
        let primary_currency = case.loan_contracts.iter()
            .map(|c| c.currency.as_str())
            .collect::<std::collections::HashMap<_, usize>>()
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(currency, _)| currency)
            .unwrap_or("HUF");

        // Calculate average loan start date
        let start_date = case.loan_contracts.iter()
            .map(|c| c.start_date.as_str())
            .min()
            .unwrap_or("2006-01-01");

        // Assess FX disclosure quality (worst case across all contracts)
        let fx_disclosure = case.loan_contracts.iter()
            .map(|c| match c.fx_risk_disclosure.as_str() {
                "none" => 0,
                "poor" => 1,
                "good" => 2,
                "excellent" => 3,
                _ => 1,
            })
            .min()
            .map(|score| match score {
                0 => "none",
                1 => "poor",
                2 => "good",
                3 => "excellent",
                _ => "poor",
            })
            .unwrap_or("poor");

        let case_data = CaseData {
            bank_name: case.bank_name.clone(),
            loan_amount: total_loan_amount,
            currency: primary_currency.to_string(),
            total_payments,
            start_date: start_date.to_string(),
            current_date: Utc::now().format("%Y-%m-%d").to_string(),
            fx_disclosure_quality: fx_disclosure.to_string(),
            case_strength: case.case_strength.clone(),
            estimated_total_damages: 0.0,
        };

        let damage_calc = self.mistral_client.calculate_damages(&case_data).await?;
        case.total_damages = damage_calc.total_recovery;

        println!("💰 Calculated damages for {}: {:.2} HUF", case.bank_name, case.total_damages);
        
        Ok(())
    }

    async fn generate_case_strategy(&self, multi_case: &mut MultiCaseAnalysis) -> Result<()> {
        for case in &mut multi_case.cases {
            let case_data = CaseData {
                bank_name: case.bank_name.clone(),
                loan_amount: case.loan_contracts.iter().map(|c| c.original_amount).sum(),
                currency: case.loan_contracts.first().map(|c| c.currency.clone()).unwrap_or("HUF".to_string()),
                total_payments: case.payment_history.iter().map(|p| p.total_paid).sum(),
                start_date: case.loan_contracts.iter().map(|c| c.start_date.as_str()).min().unwrap_or("2006-01-01").to_string(),
                current_date: Utc::now().format("%Y-%m-%d").to_string(),
                fx_disclosure_quality: case.loan_contracts.first().map(|c| c.fx_risk_disclosure.clone()).unwrap_or("poor".to_string()),
                case_strength: case.case_strength.clone(),
                estimated_total_damages: case.total_damages,
            };

            case.legal_strategy = self.mistral_client.generate_action_steps(&case_data).await?;
        }

        Ok(())
    }

    pub async fn generate_legal_documents(&self, case_analysis: &MultiCaseAnalysis, document_types: &[String]) -> Result<Vec<GeneratedDocument>> {
        let mut documents = Vec::new();

        for case in &case_analysis.cases {
            let case_data = CaseData {
                bank_name: case.bank_name.clone(),
                loan_amount: case.loan_contracts.iter().map(|c| c.original_amount).sum(),
                currency: case.loan_contracts.first().map(|c| c.currency.clone()).unwrap_or("HUF".to_string()),
                total_payments: case.payment_history.iter().map(|p| p.total_paid).sum(),
                start_date: case.loan_contracts.iter().map(|c| c.start_date.as_str()).min().unwrap_or("2006-01-01").to_string(),
                current_date: Utc::now().format("%Y-%m-%d").to_string(),
                fx_disclosure_quality: case.loan_contracts.first().map(|c| c.fx_risk_disclosure.clone()).unwrap_or("poor".to_string()),
                case_strength: case.case_strength.clone(),
                estimated_total_damages: case.total_damages,
            };

            for doc_type in document_types {
                let content = self.mistral_client.generate_legal_document(doc_type, &case_data).await?;
                
                documents.push(GeneratedDocument {
                    document_type: doc_type.clone(),
                    bank_name: case.bank_name.clone(),
                    filename: format!("{}_{}.txt", doc_type, case.bank_name.replace(" ", "_")),
                    content,
                    generated_date: Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                });
            }
        }

        Ok(documents)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedDocument {
    pub document_type: String,
    pub bank_name: String,
    pub filename: String,
    pub content: String,
    pub generated_date: String,
}