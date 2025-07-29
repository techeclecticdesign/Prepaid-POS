use crate::common::error::AppError;
use crate::domain::models::CustomerTransaction;
use crate::infrastructure::printing::reports::customer_receipt::receipt_header::receipt_header;
use crate::interface::dto::printer_dto::PrintableLineItem;
use chrono::Local;
use printpdf::{Mm, PdfDocument};
use std::fs::File;
use std::io::BufWriter;

use super::common::receipt_header;
use crate::infrastructure::printing::print::print_pdf_silently;

// Prints the customer copy PDF and sends to printer.
pub fn print_customer_receipt(
    tx: &CustomerTransaction,
    details: &[PrintableLineItem],
    operator_name: &str,
    customer_name: &str,
    printer_name: &str,
) -> Result<(), AppError> {
    let lines = 3 + details.len() + 2;
    let height = Mm((lines as f32 * 7.0) + 20.0).max(Mm(100.0));
    let (doc, page, layer) = PdfDocument::new("Customer Receipt", Mm(80.0), height, "L1");
    let current = doc.get_page(page).get_layer(layer);
    let font = doc
        .add_builtin_font(printpdf::BuiltinFont::Helvetica)
        .map_err(AppError::Pdf)?;
    let bold_font = doc
        .add_builtin_font(printpdf::BuiltinFont::HelveticaBold)
        .map_err(AppError::Pdf)?;
    let mut y = receipt_header(
        &current,
        &font,
        &bold_font,
        tx,
        operator_name,
        customer_name,
        height,
    );

    // column headers
    let header_font_size = 8.0;

    current.use_text("Description", header_font_size, Mm(5.0), y, &bold_font);
    current.use_text("Qty", header_font_size, Mm(50.0), y, &bold_font);
    current.use_text("Price", header_font_size, Mm(60.0), y, &bold_font);
    y -= Mm(4.0);

    for d in details {
        let desc = if d.desc.len() > 30 {
            format!("{}…", &d.desc[..29])
        } else {
            d.desc.clone()
        };
        current.use_text(&desc, 8.0, Mm(5.0), y, &font);
        current.use_text(d.quantity.to_string(), 8.0, Mm(50.0), y, &font);
        current.use_text(
            format!("{:.2}", f64::from(d.price) / 100.0),
            8.0,
            Mm(60.0),
            y,
            &font,
        );
        y -= Mm(4.0);
    }

    // total
    y -= Mm(8.0);
    let total = f64::from(details.iter().map(|d| d.quantity * d.price).sum::<i32>()) / 100.0;
    current.use_text(format!("Total: {total:.2}"), 10.0, Mm(5.0), y, &bold_font);

    // printed timestamp
    y -= Mm(8.0);
    current.use_text(
        format!(
            "Printed: {}",
            Local::now().format("%-m/%-d/%Y %-I:%M:%S %p")
        ),
        8.0,
        Mm(5.0),
        y,
        &font,
    );

    // finalize
    let path = "customer_receipt.pdf";
    let mut file = BufWriter::new(File::create(path)?);
    doc.save(&mut file)?;
    drop(file);
    print_pdf_silently(path, printer_name)
}
