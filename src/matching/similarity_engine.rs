use std::collections::HashMap;
use regex::Regex;

pub struct SimilarityEngine {
    fx_keywords: Vec<String>,
    legal_keywords: Vec<String>,
    transparency_keywords: Vec<String>,
}

impl SimilarityEngine {
    pub fn new() -> Self {
        Self {
            fx_keywords: vec![
                "foreign currency".to_string(),
                "deviza".to_string(),
                "árfolyam".to_string(),
                "exchange rate".to_string(),
                "currency risk".to_string(),
                "CHF".to_string(),
                "Swiss franc".to_string(),
                "svájci frank".to_string(),
                "waluta".to_string(),
                "kurs wymiany".to_string(),
                "měnové riziko".to_string(),
            ],
            legal_keywords: vec![
                "unfair".to_string(),
                "invalid".to_string(),
                "void".to_string(),
                "restitution".to_string(),
                "compensation".to_string(),
                "directive".to_string(),
                "consumer protection".to_string(),
                "méltánytalan".to_string(),
                "érvénytelen".to_string(),
                "megtérítés".to_string(),
                "fogyasztóvédelem".to_string(),
            ],
            transparency_keywords: vec![
                "information".to_string(),
                "disclosure".to_string(),
                "warning".to_string(),
                "transparent".to_string(),
                "tájékoztatás".to_string(),
                "figyelmeztetés".to_string(),
                "felvilágosítás".to_string(),
                "átlátható".to_string(),
                "informacja".to_string(),
                "ostrzeżenie".to_string(),
            ],
        }
    }

    pub fn calculate_text_similarity(&self, text1: &str, text2: &str) -> f32 {
        let tokens1 = self.tokenize(text1);
        let tokens2 = self.tokenize(text2);
        
        let mut intersection = 0;
        let mut union = std::collections::HashSet::new();
        
        for token in &tokens1 {
            union.insert(token.clone());
        }
        for token in &tokens2 {
            union.insert(token.clone());
        }
        
        for token in &tokens1 {
            if tokens2.contains(token) {
                intersection += 1;
            }
        }
        
        if union.is_empty() {
            return 0.0;
        }
        
        // Jaccard similarity
        let jaccard = intersection as f32 / union.len() as f32;
        
        // Boost similarity for domain-specific keywords
        let keyword_bonus = self.calculate_keyword_bonus(text1, text2);
        
        (jaccard + keyword_bonus).clamp(0.0, 1.0)
    }

    pub fn calculate_semantic_similarity(&self, clause_type: &str, case_ruling: &str) -> f32 {
        let ruling_lower = case_ruling.to_lowercase();
        
        match clause_type {
            "fx_risk" => {
                let mut score: f32 = 0.0;
                for keyword in &self.fx_keywords {
                    if ruling_lower.contains(&keyword.to_lowercase()) {
                        score += 0.15;
                    }
                }
                score.clamp(0.0, 1.0)
            },
            "transparency" => {
                let mut score: f32 = 0.0;
                for keyword in &self.transparency_keywords {
                    if ruling_lower.contains(&keyword.to_lowercase()) {
                        score += 0.2;
                    }
                }
                score.clamp(0.0, 1.0)
            },
            _ => {
                // General legal similarity
                let mut score: f32 = 0.0;
                for keyword in &self.legal_keywords {
                    if ruling_lower.contains(&keyword.to_lowercase()) {
                        score += 0.1;
                    }
                }
                score.clamp(0.0, 1.0)
            }
        }
    }

    pub fn calculate_precedent_strength(&self, case_year: i32, jurisdiction: &str, citation_count: Option<i32>) -> f32 {
        let mut strength: f32 = 0.5; // Base strength
        
        // Recency bonus (more recent cases are stronger)
        let years_ago = 2025 - case_year;
        if years_ago <= 2 {
            strength += 0.3;
        } else if years_ago <= 5 {
            strength += 0.2;
        } else if years_ago <= 10 {
            strength += 0.1;
        }
        
        // Jurisdiction bonus
        match jurisdiction {
            "CJEU" => strength += 0.4, // EU Court of Justice is highest authority
            "Hungary" | "Poland" | "Romania" | "Croatia" => strength += 0.2, // National courts
            _ => {}
        }
        
        // Citation count bonus (if available)
        if let Some(citations) = citation_count {
            if citations > 100 {
                strength += 0.2;
            } else if citations > 50 {
                strength += 0.1;
            } else if citations > 10 {
                strength += 0.05;
            }
        }
        
        strength.clamp(0.0, 1.0)
    }

    pub fn extract_key_phrases(&self, text: &str) -> Vec<String> {
        let mut phrases = Vec::new();
        
        // Extract FX-related phrases
        let fx_patterns = [
            r"(?i)(deviza.*hitel|foreign\s+currency\s+loan)",
            r"(?i)(árfolyam.*kockázat|exchange\s+rate\s+risk)",
            r"(?i)(svájci\s+frank|Swiss\s+franc)",
            r"(?i)(deviza.*szerződés|foreign\s+currency\s+contract)",
        ];
        
        for pattern_str in fx_patterns {
            if let Ok(regex) = Regex::new(pattern_str) {
                for mat in regex.find_iter(text) {
                    phrases.push(mat.as_str().to_string());
                }
            }
        }
        
        // Extract legal phrases
        let legal_patterns = [
            r"(?i)(tisztességtelen\s+szerződési\s+feltétel|unfair\s+contract\s+term)",
            r"(?i)(fogyasztóvédelem|consumer\s+protection)",
            r"(?i)(tájékoztatási\s+kötelezettség|duty\s+to\s+inform)",
            r"(?i)(megtérítés|restitution)",
        ];
        
        for pattern_str in legal_patterns {
            if let Ok(regex) = Regex::new(pattern_str) {
                for mat in regex.find_iter(text) {
                    phrases.push(mat.as_str().to_string());
                }
            }
        }
        
        phrases.sort();
        phrases.dedup();
        phrases
    }

    pub fn calculate_clause_criticality(&self, clause_text: &str, clause_type: &str) -> f32 {
        let mut criticality: f32 = 0.3; // Base criticality
        
        let text_lower = clause_text.to_lowercase();
        
        // FX risk clauses are inherently critical
        if clause_type == "fx_risk" {
            criticality = 0.8;
            
            // Boost if no warning language found
            let warning_terms = ["warning", "risk", "figyelmeztetés", "kockázat", "ostrzeżenie"];
            let has_warning = warning_terms.iter().any(|&term| text_lower.contains(term));
            if !has_warning {
                criticality += 0.2; // More critical if no warning
            }
        }
        
        // Transparency issues
        if clause_type == "transparency" {
            if text_lower.contains("nem") && text_lower.contains("tájékoztat") {
                criticality = 0.9; // "nem tájékoztatott" = very critical
            }
        }
        
        // Unilateral modification clauses
        if text_lower.contains("egyoldalú") || text_lower.contains("unilateral") {
            criticality += 0.3;
        }
        
        // Bank discretion clauses
        if text_lower.contains("bank dönt") || text_lower.contains("bank discretion") {
            criticality += 0.2;
        }
        
        criticality.clamp(0.0, 1.0)
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        text.to_lowercase()
            .split_whitespace()
            .filter(|word| word.len() > 2) // Filter out very short words
            .map(|word| {
                // Remove punctuation
                word.chars()
                    .filter(|c| c.is_alphabetic())
                    .collect::<String>()
            })
            .filter(|word| !word.is_empty())
            .collect()
    }

    fn calculate_keyword_bonus(&self, text1: &str, text2: &str) -> f32 {
        let text1_lower = text1.to_lowercase();
        let text2_lower = text2.to_lowercase();
        let mut bonus: f32 = 0.0;
        
        // FX keywords bonus
        for keyword in &self.fx_keywords {
            let keyword_lower = keyword.to_lowercase();
            if text1_lower.contains(&keyword_lower) && text2_lower.contains(&keyword_lower) {
                bonus += 0.1;
            }
        }
        
        // Legal keywords bonus
        for keyword in &self.legal_keywords {
            let keyword_lower = keyword.to_lowercase();
            if text1_lower.contains(&keyword_lower) && text2_lower.contains(&keyword_lower) {
                bonus += 0.05;
            }
        }
        
        // Transparency keywords bonus
        for keyword in &self.transparency_keywords {
            let keyword_lower = keyword.to_lowercase();
            if text1_lower.contains(&keyword_lower) && text2_lower.contains(&keyword_lower) {
                bonus += 0.08;
            }
        }
        
        bonus.clamp(0.0, 0.5) // Cap the bonus
    }

    pub fn generate_similarity_explanation(&self, score: f32, shared_terms: &[String]) -> String {
        let quality = if score > 0.8 {
            "very high"
        } else if score > 0.6 {
            "high"
        } else if score > 0.4 {
            "moderate"
        } else if score > 0.2 {
            "low"
        } else {
            "very low"
        };
        
        if shared_terms.is_empty() {
            format!("Similarity: {} ({:.1}%)", quality, score * 100.0)
        } else {
            format!(
                "Similarity: {} ({:.1}%) - shared terms: {}",
                quality,
                score * 100.0,
                shared_terms.join(", ")
            )
        }
    }
}