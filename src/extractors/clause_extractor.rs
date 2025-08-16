use regex::Regex;
use std::collections::HashMap;
use uuid::Uuid;
use crate::models::{ExtractedClause, ClausePattern};

pub struct ClauseExtractor {
    patterns: HashMap<String, Vec<ClausePattern>>,
    fx_risk_regex: Regex,
    transparency_regex: Regex,
    interest_rate_regex: Regex,
    penalty_regex: Regex,
}

#[derive(Debug)]
pub struct ExtractionResult {
    pub clauses: Vec<ExtractedClause>,
    pub confidence: f32,
    pub language_detected: String,
}

impl ClauseExtractor {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        
        // Load default patterns
        let default_patterns = ClausePattern::get_default_patterns();
        for pattern in default_patterns {
            patterns.entry(pattern.language.clone())
                .or_insert_with(Vec::new)
                .push(pattern);
        }

        Self {
            patterns,
            fx_risk_regex: Regex::new(
                r"(?i)(deviza|foreign\s+currency|árfolyam|exchange\s+rate|currency\s+risk|CHF|švýcarský\s+frank|švajcarski\s+franak|waluta|kurs\s+wymiany|měnové\s+riziko)"
            ).unwrap(),
            transparency_regex: Regex::new(
                r"(?i)(tájékoztatás|information|disclosure|warning|risk|figyelmeztetés|informování|varování|ostrzeżenie|informacja)"
            ).unwrap(),
            interest_rate_regex: Regex::new(
                r"(?i)(kamat|interest\s+rate|úrok|změna\s+úroku|rate\s+change|oprocentowanie|stopa\s+procentowa)"
            ).unwrap(),
            penalty_regex: Regex::new(
                r"(?i)(penalty|fee|költség|díj|sankce|poplatek|fine|kara|opłata)"
            ).unwrap(),
        }
    }

    pub async fn extract_clauses(&self, document_id: Uuid, text: &str, language: &str) -> ExtractionResult {
        let mut clauses = Vec::new();
        let mut total_confidence = 0.0;
        let mut clause_count = 0;

        // Extract FX risk clauses
        let fx_clauses = self.extract_fx_risk_clauses(document_id, text, language);
        total_confidence += fx_clauses.iter().map(|c| c.confidence_score).sum::<f32>();
        clause_count += fx_clauses.len();
        clauses.extend(fx_clauses);

        // Extract transparency clauses
        let transparency_clauses = self.extract_transparency_clauses(document_id, text, language);
        total_confidence += transparency_clauses.iter().map(|c| c.confidence_score).sum::<f32>();
        clause_count += transparency_clauses.len();
        clauses.extend(transparency_clauses);

        // Extract interest rate clauses
        let interest_clauses = self.extract_interest_rate_clauses(document_id, text, language);
        total_confidence += interest_clauses.iter().map(|c| c.confidence_score).sum::<f32>();
        clause_count += interest_clauses.len();
        clauses.extend(interest_clauses);

        // Extract penalty clauses
        let penalty_clauses = self.extract_penalty_clauses(document_id, text, language);
        total_confidence += penalty_clauses.iter().map(|c| c.confidence_score).sum::<f32>();
        clause_count += penalty_clauses.len();
        clauses.extend(penalty_clauses);

        // Extract contextual clauses (sophisticated pattern matching)
        let contextual_clauses = self.extract_contextual_clauses(document_id, text, language);
        total_confidence += contextual_clauses.iter().map(|c| c.confidence_score).sum::<f32>();
        clause_count += contextual_clauses.len();
        clauses.extend(contextual_clauses);

        let average_confidence = if clause_count > 0 {
            total_confidence / clause_count as f32
        } else {
            0.0
        };

        ExtractionResult {
            clauses,
            confidence: average_confidence,
            language_detected: language.to_string(),
        }
    }

    fn extract_fx_risk_clauses(&self, document_id: Uuid, text: &str, language: &str) -> Vec<ExtractedClause> {
        let mut clauses = Vec::new();
        
        // Hungarian FX risk patterns
        let hungarian_patterns = [
            r"(?i)devizaalapú\s+hitel",
            r"(?i)árfolyamkockázat",
            r"(?i)deviza.*kockázat",
            r"(?i)svájci\s+frank",
            r"(?i)CHF.*alapú",
            r"(?i)devizában\s+denominált",
            r"(?i)árfolyam.*változás",
            r"(?i)deviza.*kamat",
        ];

        // English FX risk patterns
        let english_patterns = [
            r"(?i)foreign\s+currency\s+loan",
            r"(?i)exchange\s+rate\s+risk",
            r"(?i)currency\s+fluctuation",
            r"(?i)Swiss\s+franc",
            r"(?i)CHF\s+loan",
            r"(?i)foreign\s+exchange",
            r"(?i)currency\s+exposure",
            r"(?i)FX\s+risk",
        ];

        let patterns = match language {
            "hu" | "hungarian" => &hungarian_patterns[..],
            "en" | "english" => &english_patterns[..],
            _ => &hungarian_patterns[..], // Default to Hungarian for region
        };

        for pattern_str in patterns {
            if let Ok(regex) = Regex::new(pattern_str) {
                for mat in regex.find_iter(text) {
                    let context = self.extract_context(text, mat.start(), mat.end(), 200);
                    let confidence = self.calculate_fx_confidence(&context);
                    
                    if confidence > 0.3 {
                        let mut clause = ExtractedClause::new(
                            document_id,
                            "fx_risk".to_string(),
                            context,
                            language.to_string(),
                            confidence,
                        );
                        clause.start_position = Some(mat.start() as i32);
                        clause.end_position = Some(mat.end() as i32);
                        clause.calculate_risk_level();
                        clauses.push(clause);
                    }
                }
            }
        }

        clauses
    }

    fn extract_transparency_clauses(&self, document_id: Uuid, text: &str, language: &str) -> Vec<ExtractedClause> {
        let mut clauses = Vec::new();

        // Hungarian transparency patterns
        let hungarian_patterns = [
            r"(?i)tájékoztatás.*kockázat",
            r"(?i)figyelmeztetés",
            r"(?i)kockázat.*ismertetés",
            r"(?i)információ.*nyújtás",
            r"(?i)kockázat.*felvilágosítás",
        ];

        // English transparency patterns  
        let english_patterns = [
            r"(?i)risk\s+disclosure",
            r"(?i)information\s+provided",
            r"(?i)warning.*risk",
            r"(?i)disclosure.*currency",
            r"(?i)informed.*decision",
        ];

        let patterns = match language {
            "hu" | "hungarian" => &hungarian_patterns[..],
            "en" | "english" => &english_patterns[..],
            _ => &hungarian_patterns[..],
        };

        for pattern_str in patterns {
            if let Ok(regex) = Regex::new(pattern_str) {
                for mat in regex.find_iter(text) {
                    let context = self.extract_context(text, mat.start(), mat.end(), 150);
                    let confidence = self.calculate_transparency_confidence(&context);
                    
                    if confidence > 0.4 {
                        let mut clause = ExtractedClause::new(
                            document_id,
                            "transparency".to_string(),
                            context,
                            language.to_string(),
                            confidence,
                        );
                        clause.start_position = Some(mat.start() as i32);
                        clause.end_position = Some(mat.end() as i32);
                        clause.calculate_risk_level();
                        clauses.push(clause);
                    }
                }
            }
        }

        clauses
    }

    fn extract_interest_rate_clauses(&self, document_id: Uuid, text: &str, language: &str) -> Vec<ExtractedClause> {
        let mut clauses = Vec::new();

        // Hungarian interest rate patterns
        let hungarian_patterns = [
            r"(?i)kamat.*változás",
            r"(?i)kamatláb.*módosítás",
            r"(?i)kamat.*emelés",
            r"(?i)változó\s+kamat",
            r"(?i)kamat.*feltétel",
        ];

        // English interest rate patterns
        let english_patterns = [
            r"(?i)interest\s+rate\s+change",
            r"(?i)variable\s+interest",
            r"(?i)rate\s+adjustment",
            r"(?i)interest.*modification",
            r"(?i)rate\s+variation",
        ];

        let patterns = match language {
            "hu" | "hungarian" => &hungarian_patterns[..],
            "en" | "english" => &english_patterns[..],
            _ => &hungarian_patterns[..],
        };

        for pattern_str in patterns {
            if let Ok(regex) = Regex::new(pattern_str) {
                for mat in regex.find_iter(text) {
                    let context = self.extract_context(text, mat.start(), mat.end(), 120);
                    let confidence = 0.7; // Standard confidence for interest clauses
                    
                    let mut clause = ExtractedClause::new(
                        document_id,
                        "interest_rate".to_string(),
                        context,
                        language.to_string(),
                        confidence,
                    );
                    clause.start_position = Some(mat.start() as i32);
                    clause.end_position = Some(mat.end() as i32);
                    clause.calculate_risk_level();
                    clauses.push(clause);
                }
            }
        }

        clauses
    }

    fn extract_penalty_clauses(&self, document_id: Uuid, text: &str, language: &str) -> Vec<ExtractedClause> {
        let mut clauses = Vec::new();

        // Hungarian penalty patterns
        let hungarian_patterns = [
            r"(?i)késedelmi\s+kamat",
            r"(?i)díj.*felszámítás",
            r"(?i)költség.*visel",
            r"(?i)bírság",
            r"(?i)pótlék.*fizetés",
        ];

        // English penalty patterns
        let english_patterns = [
            r"(?i)penalty.*fee",
            r"(?i)additional\s+charges",
            r"(?i)late\s+payment",
            r"(?i)default\s+interest",
            r"(?i)administrative\s+fee",
        ];

        let patterns = match language {
            "hu" | "hungarian" => &hungarian_patterns[..],
            "en" | "english" => &english_patterns[..],
            _ => &hungarian_patterns[..],
        };

        for pattern_str in patterns {
            if let Ok(regex) = Regex::new(pattern_str) {
                for mat in regex.find_iter(text) {
                    let context = self.extract_context(text, mat.start(), mat.end(), 100);
                    let confidence = 0.8; // High confidence for penalty clauses
                    
                    let mut clause = ExtractedClause::new(
                        document_id,
                        "penalty".to_string(),
                        context,
                        language.to_string(),
                        confidence,
                    );
                    clause.start_position = Some(mat.start() as i32);
                    clause.end_position = Some(mat.end() as i32);
                    clause.calculate_risk_level();
                    clauses.push(clause);
                }
            }
        }

        clauses
    }

    fn extract_contextual_clauses(&self, document_id: Uuid, text: &str, language: &str) -> Vec<ExtractedClause> {
        let mut clauses = Vec::new();

        // Look for contextual patterns that might indicate unfair terms
        let contextual_patterns = [
            r"(?i)(bank|hitelező).*jogosult.*egyoldalú",
            r"(?i)bank.*right.*unilateral",
            r"(?i)szerződés.*módosítás.*bank",
            r"(?i)contract.*modification.*bank",
            r"(?i)kizárólag.*bank.*dönt",
            r"(?i)solely.*bank.*discretion",
        ];

        for pattern_str in contextual_patterns {
            if let Ok(regex) = Regex::new(pattern_str) {
                for mat in regex.find_iter(text) {
                    let context = self.extract_context(text, mat.start(), mat.end(), 180);
                    let confidence = 0.6;
                    
                    let mut clause = ExtractedClause::new(
                        document_id,
                        "unfair_term".to_string(),
                        context,
                        language.to_string(),
                        confidence,
                    );
                    clause.start_position = Some(mat.start() as i32);
                    clause.end_position = Some(mat.end() as i32);
                    clause.calculate_risk_level();
                    clauses.push(clause);
                }
            }
        }

        clauses
    }

    fn extract_context(&self, text: &str, start: usize, end: usize, context_length: usize) -> String {
        let half_context = context_length / 2;
        let context_start = start.saturating_sub(half_context);
        let context_end = std::cmp::min(end + half_context, text.len());
        
        text[context_start..context_end].to_string()
    }

    fn calculate_fx_confidence(&self, context: &str) -> f32 {
        let mut confidence = 0.5;
        
        // Boost confidence for specific FX terms
        if context.to_lowercase().contains("chf") || 
           context.to_lowercase().contains("svájci frank") ||
           context.to_lowercase().contains("swiss franc") {
            confidence += 0.3;
        }
        
        // Check for risk-related terms
        if context.to_lowercase().contains("kockázat") || 
           context.to_lowercase().contains("risk") {
            confidence += 0.2;
        }
        
        // Check for warning/disclosure terms
        if context.to_lowercase().contains("tájékoztatás") || 
           context.to_lowercase().contains("figyelmeztetés") ||
           context.to_lowercase().contains("warning") ||
           context.to_lowercase().contains("disclosure") {
            confidence += 0.1;
        } else {
            // Reduce confidence if no warning found (indicates potential unfairness)
            confidence -= 0.1;
        }

        confidence.clamp(0.0, 1.0)
    }

    fn calculate_transparency_confidence(&self, context: &str) -> f32 {
        let mut confidence = 0.4;
        
        // Look for transparency indicators
        let transparency_terms = [
            "tájékoztatás", "information", "disclosure", "warning", 
            "figyelmeztetés", "risk", "kockázat"
        ];
        
        for term in transparency_terms {
            if context.to_lowercase().contains(&term.to_lowercase()) {
                confidence += 0.1;
            }
        }
        
        // Check if it's actually providing information vs. just mentioning it
        if context.to_lowercase().contains("nem") && 
           context.to_lowercase().contains("tájékoztat") {
            // "nem tájékoztatott" = "did not inform" - this is bad for the bank
            confidence += 0.3;
        }

        confidence.clamp(0.0, 1.0)
    }
}