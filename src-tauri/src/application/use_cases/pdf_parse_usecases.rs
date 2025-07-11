use crate::common::error::AppError;
use crate::domain::models::ParsedPdf;
use crate::infrastructure::pdf_parser::PdfParser;
use chrono::NaiveDateTime;
use serde_bytes::ByteBuf;

pub struct ParsePdfInput {
    pub filename: String,
    pub date: NaiveDateTime,
    pub pdf_bytes: ByteBuf,
}

pub struct PdfParseUseCase {
    parser: Box<dyn PdfParser>,
}

impl PdfParseUseCase {
    pub fn new(parser: Box<dyn PdfParser>) -> Self {
        Self { parser }
    }

    pub fn execute(&self, input: ParsePdfInput) -> Result<ParsedPdf, AppError> {
        let text = self
            .parser
            .parse(input.pdf_bytes)
            .map_err(|e| AppError::Unexpected(e.to_string()))?;

        let domain = ParsedPdf {
            filename: input.filename,
            date: input.date,
            text,
        };

        Ok(domain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::pdf_parser::StubPdfParser;

    #[test]
    fn use_case_builds_domain_model() {
        let uc = PdfParseUseCase::new(Box::new(StubPdfParser));
        let input = ParsePdfInput {
            filename: "f".into(),
            date: chrono::Utc::now().naive_utc(),
            pdf_bytes: ByteBuf::from(vec![]),
        };
        let out = uc.execute(input).unwrap();
        assert_eq!(out.text, "suckcess");
    }
}
