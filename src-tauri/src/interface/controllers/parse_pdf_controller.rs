use std::sync::Arc;

use crate::application::use_cases::pdf_parse_usecases::PdfParseUseCases;
use crate::common::error::AppError;
use crate::domain::models::ClubImport;
use crate::domain::repos::{ClubImportRepoTrait, ClubTransactionRepoTrait, CustomerRepoTrait};
use crate::infrastructure::pdf_parser::PdfParser;

pub struct PdfParseController {
    uc: PdfParseUseCases,
}

impl PdfParseController {
    pub fn new(
        parser: Arc<dyn PdfParser>,
        import_repo: Arc<dyn ClubImportRepoTrait>,
        tx_repo: Arc<dyn ClubTransactionRepoTrait>,
        cust_repo: Arc<dyn CustomerRepoTrait>,
    ) -> Self {
        Self {
            uc: PdfParseUseCases::new(parser, import_repo, tx_repo, cust_repo),
        }
    }

    pub fn parse_pdf(&self, filename: String) -> Result<ClubImport, AppError> {
        self.uc.pdf_parse(filename)
    }
}
