use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use validator_derive::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct PdfParseDto {
    #[validate(length(min = 1, message = "filename cannot be empty"))]
    pub filename: String,

    // expect an RFC3339 date, but we’ll just check non-empty for now TODO NEXT
    #[validate(length(min = 1, message = "date cannot be empty"))]
    pub date: String,

    // raw PDF bytes from the frontend
    pub pdf_bytes: ByteBuf,
}
