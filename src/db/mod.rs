pub mod cases;
pub mod documents;
pub mod clauses;
pub mod research;

use sqlx::{SqlitePool, Result};
use uuid::Uuid;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

// Database initialization and seed data
pub async fn seed_precedent_data(pool: &SqlitePool) -> Result<()> {
    // Insert CJEU cases from CSV data
    let cases = get_default_precedent_cases();
    
    for case in cases {
        cases::insert_case(pool, case).await?;
    }
    
    // Insert default clause patterns
    let patterns = crate::models::ClausePattern::get_default_patterns();
    for pattern in patterns {
        clauses::insert_clause_pattern(pool, pattern).await?;
    }
    
    Ok(())
}

fn get_default_precedent_cases() -> Vec<crate::models::LegalCase> {
    use chrono::{DateTime, Utc};
    use crate::models::{LegalCase, CaseCreateRequest};
    
    vec![
        LegalCase::new(CaseCreateRequest {
            case_number: "C-186/16".to_string(),
            case_name: "Andriciuc v Banca Românească SA".to_string(),
            country: "Romania".to_string(),
            date: DateTime::parse_from_rfc3339("2017-09-20T00:00:00Z").unwrap().with_timezone(&Utc),
            currency: "CHF/RON".to_string(),
            key_ruling: "Banks must provide adequate information about currency risk".to_string(),
            full_text: Some("The Court held that when a bank grants a foreign-currency loan, it must provide the borrower with sufficient information to enable him to take a prudent and well-informed decision.".to_string()),
            court: Some("CJEU".to_string()),
            case_type: "CJEU".to_string(),
        }),
        
        LegalCase::new(CaseCreateRequest {
            case_number: "C-520/21".to_string(),
            case_name: "Arkadiusz Szcześniak v Bank M. SA".to_string(),
            country: "Poland".to_string(),
            date: DateTime::parse_from_rfc3339("2023-06-15T00:00:00Z").unwrap().with_timezone(&Utc),
            currency: "CHF/PLN".to_string(),
            key_ruling: "Banks not entitled to interest on invalidated contracts; consumers can claim compensation".to_string(),
            full_text: Some("When a mortgage contract is annulled for unfair terms, banks cannot demand any additional compensation beyond the return of the principal loan amount.".to_string()),
            court: Some("CJEU".to_string()),
            case_type: "CJEU".to_string(),
        }),
        
        LegalCase::new(CaseCreateRequest {
            case_number: "C-705/21".to_string(),
            case_name: "MJ v AxFina Hungary Zrt.".to_string(),
            country: "Hungary".to_string(),
            date: DateTime::parse_from_rfc3339("2023-04-27T00:00:00Z").unwrap().with_timezone(&Utc),
            currency: "CHF/HUF".to_string(),
            key_ruling: "Full restitution required when contract invalid due to unfair currency terms".to_string(),
            full_text: Some("If a contract includes a contractual term placing the exchange rate risk on the consumer which is unfair, and if without that term the contract can't survive, then the contract should be declared invalid in its entirety.".to_string()),
            court: Some("CJEU".to_string()),
            case_type: "CJEU".to_string(),
        }),
        
        LegalCase::new(CaseCreateRequest {
            case_number: "C-609/19".to_string(),
            case_name: "BNP Paribas Personal Finance SA v VE".to_string(),
            country: "France".to_string(),
            date: DateTime::parse_from_rfc3339("2021-06-10T00:00:00Z").unwrap().with_timezone(&Utc),
            currency: "CHF/EUR".to_string(),
            key_ruling: "Currency terms placing disproportionate FX risk on consumers are unfair".to_string(),
            full_text: Some("Currency terms that place disproportionate exchange rate risk on consumers without adequate safeguards are contrary to the requirement of good faith.".to_string()),
            court: Some("CJEU".to_string()),
            case_type: "CJEU".to_string(),
        }),
        
        LegalCase::new(CaseCreateRequest {
            case_number: "C-26/13".to_string(),
            case_name: "Kásler v OTP Jelzálogbank Zrt".to_string(),
            country: "Hungary".to_string(),
            date: DateTime::parse_from_rfc3339("2014-04-30T00:00:00Z").unwrap().with_timezone(&Utc),
            currency: "CHF/HUF".to_string(),
            key_ruling: "Unfair currency clauses can be replaced by national law if contract cannot exist without them".to_string(),
            full_text: Some("A clause requiring use of different exchange rates for loan disbursement vs. repayment could be unfair if not transparent. National courts may replace unfair terms with supplementary provisions of national law in exceptional circumstances.".to_string()),
            court: Some("CJEU".to_string()),
            case_type: "CJEU".to_string(),
        }),
        
        LegalCase::new(CaseCreateRequest {
            case_number: "C-51/17".to_string(),
            case_name: "OTP Bank Nyrt v Teréz Ilyés and Emil Kiss".to_string(),
            country: "Hungary".to_string(),
            date: DateTime::parse_from_rfc3339("2018-09-20T00:00:00Z").unwrap().with_timezone(&Utc),
            currency: "CHF/HUF".to_string(),
            key_ruling: "Currency risk clauses subject to unfairness assessment; must be transparent".to_string(),
            full_text: Some("Currency risk clauses are subject to unfairness assessment under Directive 93/13/EEC and must meet transparency requirements.".to_string()),
            court: Some("CJEU".to_string()),
            case_type: "CJEU".to_string(),
        }),
        
        LegalCase::new(CaseCreateRequest {
            case_number: "C-118/17".to_string(),
            case_name: "Zsuzsanna Dunai v ERSTE Bank Hungary Zrt".to_string(),
            country: "Hungary".to_string(),
            date: DateTime::parse_from_rfc3339("2019-03-14T00:00:00Z").unwrap().with_timezone(&Utc),
            currency: "CHF/HUF".to_string(),
            key_ruling: "National legislation excluding retroactive cancellation is contrary to EU law".to_string(),
            full_text: Some("National legislation that prevents courts from retroactively cancelling unfair contract terms is contrary to EU consumer protection law.".to_string()),
            court: Some("CJEU".to_string()),
            case_type: "CJEU".to_string(),
        }),
        
        LegalCase::new(CaseCreateRequest {
            case_number: "C-630/23".to_string(),
            case_name: "ZH, KN v AxFina Hungary Zrt.".to_string(),
            country: "Hungary".to_string(),
            date: DateTime::parse_from_rfc3339("2025-04-30T00:00:00Z").unwrap().with_timezone(&Utc),
            currency: "CHF/HUF".to_string(),
            key_ruling: "Leasing agreements with unfair currency terms must be fully invalidated".to_string(),
            full_text: Some("Recent CJEU ruling requiring full invalidation and restitution for foreign currency leasing agreements with unfair exchange rate risk allocation.".to_string()),
            court: Some("CJEU".to_string()),
            case_type: "CJEU".to_string(),
        }),
    ]
}