use crate::common::error::AppError;
use crate::domain::models::Customer;
use crate::infrastructure::printing::print::print_pdf_silently;
use crate::infrastructure::printing::reports::common::{account_footer, util::format_cents};
use printpdf::{BuiltinFont, Mm, PdfDocument};
use std::io::Write;

pub fn print_customer_balance_report(
    rows: &[(Customer, i32)],
    total_amount: i32,
    printer_name: &str,
) -> Result<(), AppError> {
    let page_w = Mm(210.0);
    let page_h = Mm(297.0);
    let margin_top = Mm(25.0);
    let margin_bot = Mm(15.0);
    let line_h = Mm(7.0);
    let footer_h = Mm(14.0);

    let usable_y = page_h - margin_top - margin_bot - footer_h;
    let lines_per_col = (usable_y.0 / line_h.0).floor() as usize;
    let cols = 2;
    let rows_per_page = lines_per_col * cols;

    // partition pages
    let pages: Vec<_> = rows.chunks(rows_per_page).collect();

    // create pdf
    let (doc, first_page, first_layer) =
        PdfDocument::new("Customer Balances", page_w, page_h, "Layer1");
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

    for (pidx, page_chunk) in pages.iter().enumerate() {
        // for the very first page we reuse the one from PdfDocument::new
        let (page, layer) = if pidx == 0 {
            (first_page, first_layer)
        } else {
            doc.add_page(page_w, page_h, format!("Layer{}", pidx + 1))
        };
        let layer_ref = doc.get_page(page).get_layer(layer);
        // header row
        let mut y = page_h - margin_top;
        for &x_off in &[Mm(10.0), Mm(113.0)] {
            layer_ref.use_text("MDOC", 11.0, x_off, y, &bold);
            layer_ref.use_text("Name", 11.0, x_off + Mm(18.0), y, &bold);
            layer_ref.use_text("Balance", 11.0, x_off + Mm(62.0), y, &bold);
        }
        y -= line_h;

        // draw each column
        for (idx, (cust, bal)) in page_chunk.iter().enumerate() {
            let col = idx / lines_per_col;
            let row = idx % lines_per_col;
            let x_off = if col == 0 { Mm(10.0) } else { Mm(113.0) };
            let y_pos = y - Mm((row as f32) * line_h.0);

            layer_ref.use_text(cust.mdoc.to_string(), 9.0, x_off, y_pos, &font);
            layer_ref.use_text(&cust.name, 9.0, x_off + Mm(18.0), y_pos, &font);
            layer_ref.use_text(format_cents(*bal), 9.0, x_off + Mm(63.0), y_pos, &font);
        }

        // footer + page number
        let footer_layer = &layer_ref;
        account_footer::account_footer(footer_layer, &font, &bold, total_amount);
        footer_layer.use_text(
            format!("Page {} of {}", pidx + 1, pages.len()),
            8.0,
            Mm(page_w.0 - 40.0),
            Mm(10.0),
            &font,
        );
    }

    // save & print
    let path = "customer_balance_report.pdf";
    let mut file = std::io::BufWriter::new(std::fs::File::create(path)?);
    doc.save(&mut file)?;
    file.flush()?;
    let printer = printer_name.to_string();
    std::thread::spawn(move || {
        if let Err(e) = print_pdf_silently(path, &printer) {
            log::error!("Failed to print PDF: {e}");
        }
    });
    Ok(())
}
