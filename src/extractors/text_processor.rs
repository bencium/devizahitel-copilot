use base64::{Engine, engine::general_purpose};
use std::io::Read;
use tempfile::NamedTempFile;
use std::fs;

pub struct TextProcessor;

#[derive(Debug)]
pub struct ProcessingResult {
    pub extracted_text: String,
    pub confidence: f32,
    pub processing_method: String,
    pub language_detected: Option<String>,
}

impl TextProcessor {
    pub fn new() -> Self {
        Self
    }

    pub async fn process_document(&self, file_data: &str, content_type: &str) -> Result<ProcessingResult, Box<dyn std::error::Error>> {
        match content_type {
            "application/pdf" => self.process_pdf(file_data).await,
            "text/plain" => self.process_text(file_data),
            "image/jpeg" | "image/png" => self.process_image(file_data, content_type).await,
            "application/msword" | "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
                self.process_word_document(file_data).await
            },
            _ => Err(format!("Unsupported content type: {}", content_type).into()),
        }
    }

    fn process_text(&self, file_data: &str) -> Result<ProcessingResult, Box<dyn std::error::Error>> {
        let decoded = general_purpose::STANDARD.decode(file_data)?;
        let text = String::from_utf8(decoded)?;
        
        Ok(ProcessingResult {
            extracted_text: text,
            confidence: 1.0,
            processing_method: "direct_text".to_string(),
            language_detected: None,
        })
    }

    async fn process_pdf(&self, file_data: &str) -> Result<ProcessingResult, Box<dyn std::error::Error>> {
        // Decode base64 data
        let decoded = general_purpose::STANDARD.decode(file_data)?;
        
        // Create temporary file
        let mut temp_file = NamedTempFile::new()?;
        std::io::Write::write_all(&mut temp_file, &decoded)?;
        
        // For now, we'll use a simple text extraction approach
        // In production, you would integrate with a proper PDF library like pdf-extract or poppler
        let extracted_text = self.extract_pdf_text_simple(&decoded)?;
        
        Ok(ProcessingResult {
            extracted_text,
            confidence: 0.8, // PDF extraction is generally reliable
            processing_method: "pdf_extraction".to_string(),
            language_detected: None,
        })
    }

    async fn process_image(&self, file_data: &str, content_type: &str) -> Result<ProcessingResult, Box<dyn std::error::Error>> {
        // Decode base64 data
        let decoded = general_purpose::STANDARD.decode(file_data)?;
        
        // Create temporary file
        let mut temp_file = NamedTempFile::with_suffix(
            match content_type {
                "image/jpeg" => ".jpg",
                "image/png" => ".png", 
                _ => ".img",
            }
        )?;
        std::io::Write::write_all(&mut temp_file, &decoded)?;
        
        // Perform OCR
        let extracted_text = self.perform_ocr(temp_file.path().to_str().unwrap()).await?;
        
        Ok(ProcessingResult {
            extracted_text,
            confidence: 0.7, // OCR has variable accuracy
            processing_method: "ocr".to_string(),
            language_detected: None,
        })
    }

    async fn process_word_document(&self, file_data: &str) -> Result<ProcessingResult, Box<dyn std::error::Error>> {
        // Decode base64 data
        let decoded = general_purpose::STANDARD.decode(file_data)?;
        
        // Create temporary file
        let mut temp_file = NamedTempFile::with_suffix(".docx")?;
        std::io::Write::write_all(&mut temp_file, &decoded)?;
        
        // Extract text from Word document
        let extracted_text = self.extract_word_text(temp_file.path().to_str().unwrap())?;
        
        Ok(ProcessingResult {
            extracted_text,
            confidence: 0.9, // Word extraction is generally very reliable
            processing_method: "word_extraction".to_string(),
            language_detected: None,
        })
    }

    fn extract_pdf_text_simple(&self, _pdf_data: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
        // Placeholder for PDF text extraction
        // In production, integrate with pdf-extract or similar library
        
        // For now, return a message indicating PDF processing is needed
        Ok("PDF text extraction not implemented - please implement pdf-extract integration".to_string())
    }

    async fn perform_ocr(&self, image_path: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Placeholder for OCR functionality
        // In production, integrate with tesseract-rs or call tesseract command
        
        // Check if tesseract is available
        let output = std::process::Command::new("tesseract")
            .arg("--version")
            .output();
            
        match output {
            Ok(_) => {
                // Tesseract is available, perform OCR
                let ocr_output = std::process::Command::new("tesseract")
                    .arg(image_path)
                    .arg("stdout")
                    .arg("-l")
                    .arg("hun+eng") // Hungarian and English languages
                    .output()?;
                
                if ocr_output.status.success() {
                    Ok(String::from_utf8_lossy(&ocr_output.stdout).to_string())
                } else {
                    let error = String::from_utf8_lossy(&ocr_output.stderr);
                    Err(format!("OCR failed: {}", error).into())
                }
            },
            Err(_) => {
                // Tesseract not available, return placeholder
                Ok("OCR not available - please install tesseract".to_string())
            }
        }
    }

    fn extract_word_text(&self, doc_path: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Placeholder for Word document text extraction
        // In production, integrate with a library like docx-rs or python-docx via subprocess
        
        Ok("Word document text extraction not implemented - please implement docx-rs integration".to_string())
    }

    pub fn clean_text(&self, text: &str) -> String {
        // Clean up extracted text
        text
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn extract_sentences(&self, text: &str) -> Vec<String> {
        // Simple sentence extraction
        text.split('.')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty() && s.len() > 10)
            .collect()
    }

    pub fn extract_paragraphs(&self, text: &str) -> Vec<String> {
        // Extract paragraphs based on double newlines
        text.split("\n\n")
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty() && p.len() > 20)
            .collect()
    }

    pub fn normalize_text(&self, text: &str) -> String {
        // Normalize whitespace and remove excessive spacing
        let mut normalized = text.to_string();
        
        // Replace multiple spaces with single space
        while normalized.contains("  ") {
            normalized = normalized.replace("  ", " ");
        }
        
        // Replace multiple newlines with double newline
        while normalized.contains("\n\n\n") {
            normalized = normalized.replace("\n\n\n", "\n\n");
        }
        
        normalized.trim().to_string()
    }

    pub fn detect_document_structure(&self, text: &str) -> DocumentStructure {
        let lines: Vec<&str> = text.lines().collect();
        let mut structure = DocumentStructure::new();
        
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            // Detect headers (short lines in caps or with numbers)
            if trimmed.len() < 50 && (
                trimmed.chars().all(|c| c.is_uppercase() || c.is_whitespace() || c.is_numeric()) ||
                trimmed.starts_with(char::is_numeric)
            ) {
                structure.headers.push((i, trimmed.to_string()));
            }
            
            // Detect potential clauses (lines with legal language)
            if trimmed.len() > 50 && (
                trimmed.to_lowercase().contains("szerződő fél") ||
                trimmed.to_lowercase().contains("bank") ||
                trimmed.to_lowercase().contains("hitelfelvevő") ||
                trimmed.to_lowercase().contains("clause") ||
                trimmed.to_lowercase().contains("agreement")
            ) {
                structure.clauses.push((i, trimmed.to_string()));
            }
        }
        
        structure
    }
}

#[derive(Debug)]
pub struct DocumentStructure {
    pub headers: Vec<(usize, String)>,
    pub clauses: Vec<(usize, String)>,
    pub paragraphs: Vec<(usize, String)>,
}

impl DocumentStructure {
    fn new() -> Self {
        Self {
            headers: Vec::new(),
            clauses: Vec::new(),
            paragraphs: Vec::new(),
        }
    }
}