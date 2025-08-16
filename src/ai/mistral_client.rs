use serde::{Deserialize, Serialize};
use reqwest::{Client, header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}};
use std::env;
use anyhow::{Result, anyhow};

#[derive(Debug, Serialize, Deserialize)]
pub struct MistralRequest {
    pub model: String,
    pub messages: Vec<MistralMessage>,
    pub temperature: f32,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MistralMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MistralResponse {
    pub choices: Vec<MistralChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MistralChoice {
    pub message: MistralMessage,
}

#[derive(Debug, Clone)]
pub struct MistralClient {
    client: Client,
    api_key: String,
    api_url: String,
    model_large: String,
    model_small: String,
}

impl MistralClient {
    pub fn new() -> Result<Self> {
        let api_key = env::var("MISTRAL_API_KEY")
            .map_err(|_| anyhow!("MISTRAL_API_KEY environment variable not set"))?;
        
        let api_url = env::var("MISTRAL_API_URL")
            .unwrap_or_else(|_| "https://api.mistral.ai/v1".to_string());
        
        let model_large = env::var("MISTRAL_MODEL_LARGE")
            .unwrap_or_else(|_| "mistral-large-latest".to_string());
        
        let model_small = env::var("MISTRAL_MODEL_SMALL")
            .unwrap_or_else(|_| "mistral-small-latest".to_string());

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key))?
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(120))
            .build()?;

        Ok(Self {
            client,
            api_key,
            api_url,
            model_large,
            model_small,
        })
    }

    pub async fn analyze_documents(&self, documents: &[String]) -> Result<DocumentAnalysis> {
        let prompt = self.create_document_analysis_prompt(documents);
        let response = self.call_api(&prompt, true).await?;
        self.parse_document_analysis(&response)
    }

    pub async fn calculate_damages(&self, case_data: &CaseData) -> Result<DamageCalculation> {
        let prompt = self.create_damage_calculation_prompt(case_data);
        let response = self.call_api(&prompt, true).await?;
        self.parse_damage_calculation(&response)
    }

    pub async fn generate_legal_document(&self, document_type: &str, case_data: &CaseData) -> Result<String> {
        let prompt = self.create_legal_document_prompt(document_type, case_data);
        let response = self.call_api(&prompt, false).await?;
        Ok(response)
    }

    pub async fn generate_action_steps(&self, case_data: &CaseData) -> Result<String> {
        let prompt = self.create_action_steps_prompt(case_data);
        let response = self.call_api(&prompt, true).await?;
        Ok(response)
    }

    async fn call_api(&self, prompt: &str, use_large_model: bool) -> Result<String> {
        let model = if use_large_model { &self.model_large } else { &self.model_small };
        
        let request = MistralRequest {
            model: model.clone(),
            messages: vec![
                MistralMessage {
                    role: "system".to_string(),
                    content: "You are an expert Hungarian legal AI specializing in foreign currency mortgage litigation. You provide precise, factual analysis based on EU and Hungarian law, particularly focusing on consumer protection and unfair contract terms.".to_string(),
                },
                MistralMessage {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                }
            ],
            temperature: 0.3,
            max_tokens: Some(4000),
        };

        let url = format!("{}/chat/completions", self.api_url);
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Mistral API error: {}", error_text));
        }

        let mistral_response: MistralResponse = response.json().await?;
        
        if let Some(choice) = mistral_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(anyhow!("No response from Mistral API"))
        }
    }

    fn create_document_analysis_prompt(&self, documents: &[String]) -> String {
        format!(
            r#"Analyze the following Hungarian legal documents and extract key information. Return your analysis in JSON format with the following structure:

{{
  "contracts": [
    {{
      "bank_name": "string",
      "loan_type": "CHF|EUR|USD|HUF",
      "original_amount": number,
      "currency": "string",
      "start_date": "YYYY-MM-DD",
      "contract_type": "mortgage|personal_loan|other",
      "fx_risk_disclosure": "excellent|good|poor|none",
      "key_clauses": ["clause1", "clause2"]
    }}
  ],
  "payment_statements": [
    {{
      "bank_name": "string",
      "total_payments": number,
      "currency": "HUF",
      "payment_period": "YYYY-MM to YYYY-MM",
      "exchange_rate_losses": number
    }}
  ],
  "correspondence": [
    {{
      "type": "bank_response|complaint|notice",
      "date": "YYYY-MM-DD",
      "key_points": ["point1", "point2"]
    }}
  ],
  "case_strength": "very_strong|strong|moderate|weak",
  "primary_legal_issues": ["fx_risk_disclosure", "unfair_terms", "other"],
  "estimated_damages_range": {{
    "min": number,
    "max": number,
    "currency": "HUF"
  }}
}}

Documents to analyze:
{}

Focus on:
1. Extracting specific amounts, dates, and bank names
2. Identifying currency types and FX risk disclosure quality
3. Calculating potential damages based on overpayments
4. Assessing case strength based on legal precedents like C-630/23
5. Never use hardcoded figures - extract everything from the actual documents"#,
            documents.join("\n\n---DOCUMENT SEPARATOR---\n\n")
        )
    }

    fn create_damage_calculation_prompt(&self, case_data: &CaseData) -> String {
        format!(
            r#"Calculate comprehensive damages for this Hungarian FX mortgage case. Base all calculations on the provided case data and Hungarian legal precedents.

Case Data:
- Bank: {}
- Loan Type: {} {}
- Original Amount: {} {}
- Total Payments Made: {} HUF
- Loan Period: {} to {}
- FX Risk Disclosure Quality: {}

Calculate the following damage categories in HUF:

1. **Primary Restitution**: Total overpayments due to unfair FX terms
2. **Lost Interest**: Compound interest on overpayments over the loan period
3. **Inflation Adjustment**: Purchasing power lost over time
4. **Opportunity Cost**: What overpayments could have earned if invested
5. **Credit Rating Damages**: If loan affected other financial opportunities
6. **Psychological Damages**: Stress and anxiety from unfair treatment
7. **Broker Liability**: Fees paid for inadequate financial advice
8. **Legal Costs**: Professional fees for pursuing the claim
9. **Administrative Costs**: Document preparation, translation, travel

Return calculations in JSON format:
{{
  "total_recovery": number,
  "damage_breakdown": {{
    "primary_restitution": number,
    "lost_interest": number,
    "inflation_adjustment": number,
    "opportunity_cost": number,
    "credit_rating_damages": number,
    "psychological_damages": number,
    "broker_liability": number,
    "legal_costs": number,
    "administrative_costs": number
  }},
  "calculation_notes": ["explanation1", "explanation2"],
  "confidence_level": "high|medium|low"
}}

Use realistic rates:
- Interest rates: 5-8% annually for lost interest calculations
- Inflation: 3-4% annually for purchasing power adjustment
- Opportunity cost: 6-9% annually for alternative investments"#,
            case_data.bank_name,
            case_data.loan_amount, case_data.currency,
            case_data.loan_amount, case_data.currency,
            case_data.total_payments,
            case_data.start_date, case_data.current_date,
            case_data.fx_disclosure_quality
        )
    }

    fn create_legal_document_prompt(&self, document_type: &str, case_data: &CaseData) -> String {
        match document_type {
            "central_bank" => self.create_mnb_complaint_prompt(case_data),
            "financial_authority" => self.create_pbt_complaint_prompt(case_data),
            "lawyer_consultation" => self.create_lawyer_letter_prompt(case_data),
            _ => format!("Generate a {} document for the case", document_type),
        }
    }

    fn create_mnb_complaint_prompt(&self, case_data: &CaseData) -> String {
        format!(
            r#"Generate a formal complaint letter to the Hungarian Central Bank (MNB) based on this FX mortgage case.

Case Details:
- Bank: {}
- Loan: {} {} (started: {})
- Total Payments: {} HUF
- FX Disclosure Quality: {}
- Estimated Damages: {} HUF

Write a professional complaint letter in Hungarian that:
1. References relevant EU directives (93/13/EEC) and Hungarian banking laws
2. Cites recent CJEU precedents (C-630/23, C-186/16)
3. Demands investigation of unfair FX practices
4. Requests enforcement action against the bank
5. Uses formal Hungarian legal terminology

The letter should be ready to send, with proper formatting and professional tone."#,
            case_data.bank_name,
            case_data.loan_amount, case_data.currency, case_data.start_date,
            case_data.total_payments,
            case_data.fx_disclosure_quality,
            case_data.estimated_total_damages
        )
    }

    fn create_pbt_complaint_prompt(&self, case_data: &CaseData) -> String {
        format!(
            r#"Generate a formal complaint to the Financial Arbitration Board (Pénzügyi Békéltető Testület - PBT).

Case Details:
- Bank: {}
- Loan: {} {} (started: {})
- Total Payments: {} HUF
- Estimated Recovery: {} HUF

Create a detailed PBT complaint that:
1. States clear legal grounds for the complaint
2. Demands specific remedies (contract annulment or fair recalculation)
3. Includes evidence list and damage calculations
4. References applicable legal precedents
5. Requests binding arbitration decision

Format as official PBT submission with all required sections."#,
            case_data.bank_name,
            case_data.loan_amount, case_data.currency, case_data.start_date,
            case_data.total_payments,
            case_data.estimated_total_damages
        )
    }

    fn create_lawyer_letter_prompt(&self, case_data: &CaseData) -> String {
        format!(
            r#"Generate a consultation request letter to a Hungarian consumer protection lawyer.

Case Summary:
- Bank: {}
- Loan Type: {} {}
- Damages: {} HUF
- Case Strength: {}

Write a professional letter that:
1. Summarizes the case facts concisely
2. Highlights strongest legal arguments
3. Requests case evaluation and representation
4. Asks about fee structure and timeline
5. Mentions readiness to proceed with litigation

Tone should be professional but determined, showing case preparation."#,
            case_data.bank_name,
            case_data.loan_amount, case_data.currency,
            case_data.estimated_total_damages,
            case_data.case_strength
        )
    }

    fn create_action_steps_prompt(&self, case_data: &CaseData) -> String {
        format!(
            r#"Generate a personalized action plan for this Hungarian FX mortgage case.

Case Profile:
- Bank: {}
- Loan: {} {} 
- Recovery Potential: {} HUF
- Case Strength: {}
- FX Disclosure: {}

Create a comprehensive action plan with:

1. **IMMEDIATE ACTIONS (7 days)**
   - Document collection priorities
   - Evidence preservation steps
   - Initial calculations to complete

2. **SHORT-TERM ACTIONS (2-4 weeks)**
   - Legal filing deadlines
   - Authority complaints to submit
   - Professional consultations to arrange

3. **LEGAL OPTIONS**
   - PBT arbitration process (timeline, costs, success rate)
   - Court litigation path (timeline, costs, success rate)
   - Settlement negotiation strategy

4. **FINANCIAL DOCUMENTATION**
   - Damage calculation spreadsheets to prepare
   - Evidence links to establish
   - Loss documentation methods

5. **CRITICAL WARNINGS**
   - Limitation periods to avoid missing
   - Settlement traps to avoid
   - Rights not to waive

Format as actionable checklist with realistic timelines and specific next steps for this exact case."#,
            case_data.bank_name,
            case_data.loan_amount, case_data.currency,
            case_data.estimated_total_damages,
            case_data.case_strength,
            case_data.fx_disclosure_quality
        )
    }

    fn parse_document_analysis(&self, response: &str) -> Result<DocumentAnalysis> {
        // Parse JSON response into DocumentAnalysis struct
        serde_json::from_str(response)
            .map_err(|e| anyhow!("Failed to parse document analysis: {}", e))
    }

    fn parse_damage_calculation(&self, response: &str) -> Result<DamageCalculation> {
        // Parse JSON response into DamageCalculation struct
        serde_json::from_str(response)
            .map_err(|e| anyhow!("Failed to parse damage calculation: {}", e))
    }
}

// Data structures for AI analysis
#[derive(Debug, Serialize, Deserialize)]
pub struct CaseData {
    pub bank_name: String,
    pub loan_amount: f64,
    pub currency: String,
    pub total_payments: f64,
    pub start_date: String,
    pub current_date: String,
    pub fx_disclosure_quality: String,
    pub case_strength: String,
    pub estimated_total_damages: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentAnalysis {
    pub contracts: Vec<ContractInfo>,
    pub payment_statements: Vec<PaymentInfo>,
    pub correspondence: Vec<CorrespondenceInfo>,
    pub case_strength: String,
    pub primary_legal_issues: Vec<String>,
    pub estimated_damages_range: DamageRange,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractInfo {
    pub bank_name: String,
    pub loan_type: String,
    pub original_amount: f64,
    pub currency: String,
    pub start_date: String,
    pub contract_type: String,
    pub fx_risk_disclosure: String,
    pub key_clauses: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInfo {
    pub bank_name: String,
    pub total_payments: f64,
    pub currency: String,
    pub payment_period: String,
    pub exchange_rate_losses: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CorrespondenceInfo {
    #[serde(rename = "type")]
    pub doc_type: String,
    pub date: String,
    pub key_points: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageRange {
    pub min: f64,
    pub max: f64,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageCalculation {
    pub total_recovery: f64,
    pub damage_breakdown: DamageBreakdown,
    pub calculation_notes: Vec<String>,
    pub confidence_level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageBreakdown {
    pub primary_restitution: f64,
    pub lost_interest: f64,
    pub inflation_adjustment: f64,
    pub opportunity_cost: f64,
    pub credit_rating_damages: f64,
    pub psychological_damages: f64,
    pub broker_liability: f64,
    pub legal_costs: f64,
    pub administrative_costs: f64,
}