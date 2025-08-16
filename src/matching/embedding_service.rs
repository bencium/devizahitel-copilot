use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize)]
struct EmbeddingRequest {
    input: String,
    model: String,
}

#[derive(Debug, Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Debug, Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
}

pub struct EmbeddingService {
    client: Client,
    api_key: Option<String>,
    model: String,
}

impl EmbeddingService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            api_key: env::var("OPENAI_API_KEY").ok(),
            model: "text-embedding-ada-002".to_string(),
        }
    }

    pub async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        // If no API key, return a mock embedding
        if self.api_key.is_none() {
            return Ok(self.generate_mock_embedding(text));
        }

        let request = EmbeddingRequest {
            input: text.to_string(),
            model: self.model.clone(),
        };

        let response = self.client
            .post("https://api.openai.com/v1/embeddings")
            .header("Authorization", format!("Bearer {}", self.api_key.as_ref().unwrap()))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("API request failed: {}", response.status()).into());
        }

        let embedding_response: EmbeddingResponse = response.json().await?;
        
        if let Some(data) = embedding_response.data.first() {
            Ok(data.embedding.clone())
        } else {
            Err("No embedding data received".into())
        }
    }

    pub async fn generate_embeddings_batch(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
        let mut embeddings = Vec::new();
        
        // Process in batches to avoid API limits
        for chunk in texts.chunks(100) {
            for text in chunk {
                let embedding = self.generate_embedding(text).await?;
                embeddings.push(embedding);
            }
            
            // Small delay to respect rate limits
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        Ok(embeddings)
    }

    pub fn calculate_cosine_similarity(&self, embedding1: &[f32], embedding2: &[f32]) -> f32 {
        if embedding1.len() != embedding2.len() {
            return 0.0;
        }

        let dot_product: f32 = embedding1.iter()
            .zip(embedding2.iter())
            .map(|(a, b)| a * b)
            .sum();

        let magnitude1: f32 = embedding1.iter().map(|x| x * x).sum::<f32>().sqrt();
        let magnitude2: f32 = embedding2.iter().map(|x| x * x).sum::<f32>().sqrt();

        if magnitude1 == 0.0 || magnitude2 == 0.0 {
            return 0.0;
        }

        dot_product / (magnitude1 * magnitude2)
    }

    pub async fn find_similar_texts(&self, query_embedding: &[f32], candidate_embeddings: &[(String, Vec<f32>)], top_k: usize) -> Vec<(String, f32)> {
        let mut similarities = Vec::new();

        for (text, embedding) in candidate_embeddings {
            let similarity = self.calculate_cosine_similarity(query_embedding, embedding);
            similarities.push((text.clone(), similarity));
        }

        // Sort by similarity descending
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Return top k results
        similarities.into_iter().take(top_k).collect()
    }

    fn generate_mock_embedding(&self, text: &str) -> Vec<f32> {
        // Generate a simple hash-based mock embedding for development
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let hash = hasher.finish();
        
        // Generate 1536-dimensional vector (same as OpenAI ada-002)
        let mut embedding = Vec::with_capacity(1536);
        let mut seed = hash;
        
        for _ in 0..1536 {
            // Simple linear congruential generator
            seed = (seed.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;
            let normalized = (seed as f32 / i32::MAX as f32) * 2.0 - 1.0;
            embedding.push(normalized);
        }
        
        // Normalize to unit vector
        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if magnitude > 0.0 {
            for value in &mut embedding {
                *value /= magnitude;
            }
        }
        
        embedding
    }

    pub fn generate_text_features(&self, text: &str) -> TextFeatures {
        let words: Vec<&str> = text.split_whitespace().collect();
        let sentences: Vec<&str> = text.split('.').filter(|s| !s.trim().is_empty()).collect();
        
        let avg_word_length = if !words.is_empty() {
            words.iter().map(|w| w.len()).sum::<usize>() as f32 / words.len() as f32
        } else {
            0.0
        };

        let avg_sentence_length = if !sentences.is_empty() {
            sentences.iter().map(|s| s.split_whitespace().count()).sum::<usize>() as f32 / sentences.len() as f32
        } else {
            0.0
        };

        // Count domain-specific terms
        let fx_terms = ["deviza", "foreign currency", "CHF", "exchange rate", "árfolyam"];
        let legal_terms = ["szerződés", "contract", "clause", "bank", "hitel", "loan"];
        
        let fx_term_count = fx_terms.iter()
            .map(|term| text.to_lowercase().matches(&term.to_lowercase()).count())
            .sum::<usize>();
            
        let legal_term_count = legal_terms.iter()
            .map(|term| text.to_lowercase().matches(&term.to_lowercase()).count())
            .sum::<usize>();

        TextFeatures {
            word_count: words.len(),
            sentence_count: sentences.len(),
            avg_word_length,
            avg_sentence_length,
            fx_term_density: fx_term_count as f32 / words.len() as f32,
            legal_term_density: legal_term_count as f32 / words.len() as f32,
        }
    }
}

#[derive(Debug)]
pub struct TextFeatures {
    pub word_count: usize,
    pub sentence_count: usize,
    pub avg_word_length: f32,
    pub avg_sentence_length: f32,
    pub fx_term_density: f32,
    pub legal_term_density: f32,
}

impl TextFeatures {
    pub fn to_vector(&self) -> Vec<f32> {
        vec![
            self.word_count as f32 / 1000.0, // Normalize
            self.sentence_count as f32 / 100.0,
            self.avg_word_length / 10.0,
            self.avg_sentence_length / 50.0,
            self.fx_term_density,
            self.legal_term_density,
        ]
    }
}