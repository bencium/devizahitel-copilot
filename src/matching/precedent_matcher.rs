use uuid::Uuid;
use sqlx::SqlitePool;
use chrono::Datelike;
use crate::models::{LegalCase, ExtractedClause, CaseMatch, ApplicablePrecedent};
use crate::matching::SimilarityEngine;
use std::collections::HashMap;

pub struct PrecedentMatcher {
    similarity_engine: SimilarityEngine,
}

#[derive(Debug)]
pub struct MatchingResult {
    pub clause_matches: Vec<ClauseMatch>,
    pub overall_case_matches: Vec<CaseMatch>,
    pub confidence_score: f32,
}

#[derive(Debug)]
pub struct ClauseMatch {
    pub clause_id: Uuid,
    pub matched_cases: Vec<CaseMatch>,
    pub match_reasoning: String,
}

impl PrecedentMatcher {
    pub fn new() -> Self {
        Self {
            similarity_engine: SimilarityEngine::new(),
        }
    }

    pub async fn match_precedents(
        &self,
        pool: &SqlitePool,
        clauses: &[ExtractedClause],
    ) -> Result<MatchingResult, Box<dyn std::error::Error>> {
        let mut clause_matches = Vec::new();
        let mut case_score_map: HashMap<Uuid, f32> = HashMap::new();
        let mut case_map: HashMap<Uuid, LegalCase> = HashMap::new();

        // Get all relevant precedent cases
        let fx_cases = crate::db::cases::get_fx_related_cases(pool).await?;
        
        for case in &fx_cases {
            case_map.insert(case.id, case.clone());
        }

        // Match each clause against precedents
        for clause in clauses {
            let matches = self.match_clause_to_precedents(clause, &fx_cases).await?;
            
            // Update case scores
            for case_match in &matches {
                let current_score = case_score_map.get(&case_match.case.id).unwrap_or(&0.0);
                case_score_map.insert(
                    case_match.case.id, 
                    current_score + case_match.similarity_score
                );
            }

            let match_reasoning = self.generate_match_reasoning(clause, &matches);
            clause_matches.push(ClauseMatch {
                clause_id: clause.id,
                matched_cases: matches,
                match_reasoning,
            });
        }

        // Generate overall case matches
        let mut overall_matches = Vec::new();
        for (case_id, total_score) in case_score_map {
            if let Some(case) = case_map.get(&case_id) {
                let normalized_score = total_score / clauses.len() as f32;
                if normalized_score > 0.3 { // Threshold for relevance
                    let matching_clauses = self.get_matching_clause_types(clauses, case);
                    let relevance_explanation = self.generate_relevance_explanation(case, &matching_clauses);
                    
                    overall_matches.push(CaseMatch {
                        case: case.clone(),
                        similarity_score: normalized_score,
                        matching_clauses,
                        relevance_explanation,
                    });
                }
            }
        }

        // Sort by similarity score
        overall_matches.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap());

        let confidence_score = if !overall_matches.is_empty() {
            overall_matches.iter().map(|m| m.similarity_score).sum::<f32>() / overall_matches.len() as f32
        } else {
            0.0
        };

        Ok(MatchingResult {
            clause_matches,
            overall_case_matches: overall_matches,
            confidence_score,
        })
    }

    async fn match_clause_to_precedents(
        &self,
        clause: &ExtractedClause,
        cases: &[LegalCase],
    ) -> Result<Vec<CaseMatch>, Box<dyn std::error::Error>> {
        let mut matches = Vec::new();

        for case in cases {
            let similarity_score = self.calculate_clause_case_similarity(clause, case);
            
            if similarity_score > 0.2 {
                let matching_clauses = vec![clause.clause_type.clone()];
                let relevance_explanation = self.generate_case_relevance(clause, case, similarity_score);
                
                matches.push(CaseMatch {
                    case: case.clone(),
                    similarity_score,
                    matching_clauses,
                    relevance_explanation,
                });
            }
        }

        // Sort by similarity score
        matches.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap());
        
        // Return top 5 matches
        matches.truncate(5);
        Ok(matches)
    }

    fn calculate_clause_case_similarity(&self, clause: &ExtractedClause, case: &LegalCase) -> f32 {
        let mut similarity = 0.0;

        // Check clause type relevance
        match clause.clause_type.as_str() {
            "fx_risk" => {
                if case.is_foreign_currency_case() {
                    similarity += 0.8;
                }
                // Specific case relevance
                match case.case_number.as_str() {
                    "C-186/16" => similarity += 0.3, // Andriciuc - information requirements
                    "C-705/21" | "C-630/23" => similarity += 0.4, // AxFina cases - contract invalidation
                    "C-520/21" => similarity += 0.3, // Szcześniak - restitution
                    _ => {}
                }
            },
            "transparency" => {
                if case.key_ruling.to_lowercase().contains("information") ||
                   case.key_ruling.to_lowercase().contains("disclosure") ||
                   case.case_number == "C-186/16" { // Andriciuc
                    similarity += 0.9;
                }
            },
            "interest_rate" => {
                if case.key_ruling.to_lowercase().contains("interest") {
                    similarity += 0.7;
                }
            },
            "penalty" => {
                if case.key_ruling.to_lowercase().contains("compensation") ||
                   case.key_ruling.to_lowercase().contains("restitution") {
                    similarity += 0.6;
                }
            },
            _ => {
                similarity += 0.1; // Base relevance for any clause
            }
        }

        // Boost for recent cases (more authoritative)
        if case.date.year() >= 2020 {
            similarity += 0.1;
        }
        if case.date.year() >= 2023 {
            similarity += 0.1;
        }

        // Boost for CJEU cases
        if case.case_number.starts_with("C-") {
            similarity += 0.2;
        }

        // Text similarity (simplified)
        let text_similarity = self.similarity_engine.calculate_text_similarity(
            &clause.clause_text,
            &case.key_ruling,
        );
        similarity += text_similarity * 0.3;

        similarity.clamp(0.0, 1.0)
    }

    fn get_matching_clause_types(&self, clauses: &[ExtractedClause], case: &LegalCase) -> Vec<String> {
        let mut matching_types = Vec::new();

        for clause in clauses {
            let similarity = self.calculate_clause_case_similarity(clause, case);
            if similarity > 0.3 {
                matching_types.push(clause.clause_type.clone());
            }
        }

        matching_types.sort();
        matching_types.dedup();
        matching_types
    }

    fn generate_match_reasoning(&self, clause: &ExtractedClause, matches: &[CaseMatch]) -> String {
        if matches.is_empty() {
            return "No relevant precedents found for this clause type.".to_string();
        }

        let top_match = &matches[0];
        let clause_type_explanation = match clause.clause_type.as_str() {
            "fx_risk" => "foreign currency risk allocation",
            "transparency" => "information disclosure requirements", 
            "interest_rate" => "interest rate modification terms",
            "penalty" => "penalty and fee provisions",
            _ => "contractual terms",
        };

        format!(
            "This {} clause is most similar to the precedent in {} (similarity: {:.1}%), which established that {}. {} additional related cases were found.",
            clause_type_explanation,
            top_match.case.case_name,
            top_match.similarity_score * 100.0,
            top_match.case.key_ruling.to_lowercase(),
            matches.len() - 1
        )
    }

    fn generate_relevance_explanation(&self, case: &LegalCase, matching_clauses: &[String]) -> String {
        let jurisdiction = case.get_jurisdiction();
        let clause_description = if matching_clauses.len() == 1 {
            format!("the {} clause", matching_clauses[0])
        } else {
            format!("{} clause types", matching_clauses.len())
        };

        format!(
            "This {} case from {} is relevant to {} in your document. The court ruled: {}",
            jurisdiction,
            case.date.format("%Y"),
            clause_description,
            case.key_ruling
        )
    }

    fn generate_case_relevance(&self, clause: &ExtractedClause, case: &LegalCase, similarity: f32) -> String {
        format!(
            "Relevant to {} clause ({}% similarity): {}",
            clause.clause_type,
            (similarity * 100.0) as i32,
            case.key_ruling
        )
    }

    pub fn create_applicable_precedents(&self, matches: &[CaseMatch]) -> Vec<ApplicablePrecedent> {
        matches.iter().map(|case_match| {
            let citation_text = self.format_citation(&case_match.case);
            let key_principles = self.extract_key_principles(&case_match.case);
            let application_notes = self.generate_application_notes(&case_match.case);

            ApplicablePrecedent {
                case_id: case_match.case.id,
                case_number: case_match.case.case_number.clone(),
                case_name: case_match.case.case_name.clone(),
                jurisdiction: case_match.case.get_jurisdiction(),
                relevance_score: case_match.similarity_score,
                key_principles,
                citation_text,
                application_notes,
            }
        }).collect()
    }

    fn format_citation(&self, case: &LegalCase) -> String {
        if case.case_number.starts_with("C-") {
            format!(
                "Case {}, {}, ECLI:EU:C:{}",
                case.case_number,
                case.case_name,
                case.date.format("%Y:%m:%d")
            )
        } else {
            format!(
                "{}, {} ({} {})",
                case.case_name,
                case.court.as_ref().unwrap_or(&"Court".to_string()),
                case.country,
                case.date.format("%Y")
            )
        }
    }

    fn extract_key_principles(&self, case: &LegalCase) -> Vec<String> {
        // Extract key legal principles from the case ruling
        let mut principles = Vec::new();
        let ruling = case.key_ruling.to_lowercase();

        if ruling.contains("adequate information") || ruling.contains("sufficient information") {
            principles.push("Banks must provide adequate information about currency risks".to_string());
        }

        if ruling.contains("unfair") && ruling.contains("currency") {
            principles.push("Currency clauses placing disproportionate risk on consumers are unfair".to_string());
        }

        if ruling.contains("restitution") || ruling.contains("compensation") {
            principles.push("Full restitution required when contracts are invalidated for unfair terms".to_string());
        }

        if ruling.contains("transparent") {
            principles.push("Contract terms must be transparent and intelligible".to_string());
        }

        if ruling.contains("invalid") || ruling.contains("void") {
            principles.push("Contracts with unfair terms can be declared invalid in their entirety".to_string());
        }

        if principles.is_empty() {
            principles.push(case.key_ruling.clone());
        }

        principles
    }

    fn generate_application_notes(&self, case: &LegalCase) -> String {
        match case.case_number.as_str() {
            "C-186/16" => "Establishes duty to inform consumers about FX risks. Banks must explain how currency depreciation would affect payments.".to_string(),
            "C-705/21" | "C-630/23" => "Recent CJEU ruling requiring full contract invalidation when FX risk clauses are unfair. Provides strong precedent for complete restitution.".to_string(),
            "C-520/21" => "Confirms banks cannot claim compensation when contracts are invalidated. Consumers entitled to full refund of payments made.".to_string(),
            "C-26/13" => "Early precedent on currency clause transparency. Shows evolution of CJEU thinking toward stronger consumer protection.".to_string(),
            _ => format!("Relevant precedent from {} addressing foreign currency mortgage issues.", case.country),
        }
    }
}