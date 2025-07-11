use crate::application::use_cases::pdf_parse_usecases::{ParsePdfInput, PdfParseUseCase};
use crate::common::error::AppError;
use crate::domain::models::ParsedPdf;
use crate::interface::dto::pdf_parse_dto::PdfParseDto;
use validator::Validate;

pub struct PdfParseController {
    uc: PdfParseUseCase,
}

impl PdfParseController {
    pub fn new(parser: impl crate::infrastructure::pdf_parser::PdfParser + 'static) -> Self {
        Self {
            uc: PdfParseUseCase::new(Box::new(parser)),
        }
    }

    pub fn parse(&self, dto: PdfParseDto) -> Result<ParsedPdf, AppError> {
        dto.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let parsed_date = crate::interface::common::date_utils::parse_rfc3339(&dto.date)
            .map_err(|e| AppError::Validation(format!("date: {}", e)))?;

        let input = ParsePdfInput {
            filename: dto.filename,
            date: parsed_date,
            pdf_bytes: dto.pdf_bytes,
        };

        self.uc.execute(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::pdf_parser::StubPdfParser;
    use serde_bytes::ByteBuf;

    #[test]
    fn controller_returns_stubbed_domain_model() {
        let ctrl = PdfParseController::new(StubPdfParser);
        let dto = PdfParseDto {
            filename: "f".into(),
            date: "2023-01-01T12:00:00Z".into(),
            pdf_bytes: ByteBuf::from(vec![]),
        };
        let res = ctrl.parse(dto).unwrap();
        assert_eq!(res.text, "suckcess");
    }
}
