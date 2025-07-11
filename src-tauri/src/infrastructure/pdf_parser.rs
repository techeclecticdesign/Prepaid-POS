use crate::common::error::AppError;
use serde_bytes::ByteBuf;

/// Defines how we parse PDF bytes → raw text
pub trait PdfParser: Sync + Send {
    fn parse(&self, bytes: ByteBuf) -> Result<String, AppError>;
}

/// A stub impl so we can wire up the layers; returns our magic word.
pub struct StubPdfParser;

impl PdfParser for StubPdfParser {
    fn parse(&self, _bytes: ByteBuf) -> Result<String, AppError> {
        // in future swap this for lopdf logic
        Ok("suckcess".into())
    }
}
