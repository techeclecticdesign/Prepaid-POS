use crate::common::error::AppError;
use lopdf::Document;
use std::fs;

pub trait PdfParser: Sync + Send {
    fn parse(&self, filename: String) -> Result<String, AppError>;
}

pub struct LopdfParser;

impl PdfParser for LopdfParser {
    fn parse(&self, filename: String) -> Result<String, AppError> {
        let data = fs::read(&filename)
            .map_err(|e| AppError::Unexpected(format!("fs::read error: {}", e)))?;

        let doc = Document::load_mem(&data)
            .map_err(|e| AppError::Unexpected(format!("PDF load error: {}", e)))?;

        let page_numbers: Vec<u32> = doc.get_pages().keys().copied().collect();

        let text = doc
            .extract_text(&page_numbers)
            .map_err(|e| AppError::Unexpected(format!("Text extraction error: {}", e)))?;

        Ok(text)
    }
}
