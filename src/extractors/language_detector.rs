use std::collections::HashMap;

pub struct LanguageDetector {
    hungarian_patterns: Vec<&'static str>,
    english_patterns: Vec<&'static str>,
    czech_patterns: Vec<&'static str>,
    polish_patterns: Vec<&'static str>,
}

#[derive(Debug, Clone)]
pub struct LanguageDetection {
    pub language: String,
    pub confidence: f32,
    pub scores: HashMap<String, f32>,
}

impl LanguageDetector {
    pub fn new() -> Self {
        Self {
            hungarian_patterns: vec![
                "a", "az", "és", "vagy", "de", "hogy", "nem", "van", "volt", "lesz",
                "hitel", "bank", "szerződés", "kamat", "deviza", "kockázat", "árfolyam",
                "tájékoztatás", "figyelmeztetés", "költség", "díj", "kölcsön", "jelzálog",
                "törlesztés", "részlet", "feltétel", "módosítás", "felmondás"
            ],
            english_patterns: vec![
                "the", "and", "or", "but", "that", "not", "is", "was", "will", "have",
                "loan", "bank", "contract", "interest", "currency", "risk", "exchange",
                "information", "warning", "cost", "fee", "mortgage", "payment",
                "installment", "condition", "modification", "termination"
            ],
            czech_patterns: vec![
                "a", "je", "se", "na", "do", "za", "od", "po", "před", "při",
                "úvěr", "banka", "smlouva", "úrok", "měna", "riziko", "kurz",
                "informace", "varování", "náklad", "poplatek", "hypotéka"
            ],
            polish_patterns: vec![
                "i", "a", "w", "na", "z", "do", "od", "po", "przez", "przy",
                "kredyt", "bank", "umowa", "odsetki", "waluta", "ryzyko", "kurs",
                "informacja", "ostrzeżenie", "koszt", "opłata", "hipoteka"
            ],
        }
    }

    pub fn detect_language(&self, text: &str) -> LanguageDetection {
        let text_lower = text.to_lowercase();
        let words: Vec<&str> = text_lower.split_whitespace().collect();
        let total_words = words.len() as f32;

        if total_words == 0.0 {
            return LanguageDetection {
                language: "unknown".to_string(),
                confidence: 0.0,
                scores: HashMap::new(),
            };
        }

        let mut scores = HashMap::new();

        // Calculate Hungarian score
        let hungarian_matches = self.count_pattern_matches(&words, &self.hungarian_patterns);
        scores.insert("hungarian".to_string(), hungarian_matches / total_words);

        // Calculate English score
        let english_matches = self.count_pattern_matches(&words, &self.english_patterns);
        scores.insert("english".to_string(), english_matches / total_words);

        // Calculate Czech score
        let czech_matches = self.count_pattern_matches(&words, &self.czech_patterns);
        scores.insert("czech".to_string(), czech_matches / total_words);

        // Calculate Polish score
        let polish_matches = self.count_pattern_matches(&words, &self.polish_patterns);
        scores.insert("polish".to_string(), polish_matches / total_words);

        // Additional language-specific patterns
        let mut bonus_scores = HashMap::new();
        
        // Hungarian specific patterns
        if text_lower.contains("deviza") || text_lower.contains("árfolyam") || text_lower.contains("huf") {
            bonus_scores.insert("hungarian".to_string(), 0.2);
        }
        
        // English specific patterns
        if text_lower.contains("foreign currency") || text_lower.contains("exchange rate") {
            bonus_scores.insert("english".to_string(), 0.2);
        }

        // Czech specific patterns
        if text_lower.contains("měnové riziko") || text_lower.contains("směnný kurz") {
            bonus_scores.insert("czech".to_string(), 0.2);
        }

        // Polish specific patterns
        if text_lower.contains("ryzyko walutowe") || text_lower.contains("kurs wymiany") {
            bonus_scores.insert("polish".to_string(), 0.2);
        }

        // Apply bonus scores
        for (lang, bonus) in bonus_scores {
            if let Some(score) = scores.get_mut(&lang) {
                *score += bonus;
            }
        }

        // Find the language with highest score
        let (detected_language, confidence) = scores
            .iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(lang, score)| (lang.clone(), *score))
            .unwrap_or(("unknown".to_string(), 0.0));

        // Convert to standard language codes
        let language_code = match detected_language.as_str() {
            "hungarian" => "hu",
            "english" => "en", 
            "czech" => "cs",
            "polish" => "pl",
            _ => "unknown",
        };

        LanguageDetection {
            language: language_code.to_string(),
            confidence,
            scores,
        }
    }

    fn count_pattern_matches(&self, words: &[&str], patterns: &[&str]) -> f32 {
        let mut matches = 0;
        for word in words {
            if patterns.contains(word) {
                matches += 1;
            }
        }
        matches as f32
    }

    pub fn detect_mixed_language(&self, text: &str) -> Vec<LanguageDetection> {
        // For documents that might contain multiple languages
        let sentences: Vec<&str> = text.split('.').collect();
        let mut detections = Vec::new();

        for sentence in sentences {
            if sentence.trim().len() > 20 { // Only analyze substantial sentences
                let detection = self.detect_language(sentence);
                if detection.confidence > 0.1 {
                    detections.push(detection);
                }
            }
        }

        detections
    }

    pub fn is_central_european_language(&self, language: &str) -> bool {
        matches!(language, "hu" | "cs" | "pl" | "sk" | "ro" | "hr" | "sl")
    }

    pub fn get_language_name(&self, code: &str) -> &'static str {
        match code {
            "hu" => "Hungarian",
            "en" => "English",
            "cs" => "Czech", 
            "pl" => "Polish",
            "sk" => "Slovak",
            "ro" => "Romanian",
            "hr" => "Croatian",
            "sl" => "Slovenian",
            _ => "Unknown",
        }
    }
}