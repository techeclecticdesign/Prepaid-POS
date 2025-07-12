use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use validator_derive::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct PdfParseDto {
    #[validate(length(min = 1, message = "filename cannot be empty"))]
    pub filename: String,

    pub date: String,

    // raw PDF bytes from the frontend
    pub pdf_bytes: ByteBuf,
}
