use chrono::Local;
use printpdf::{IndirectFontRef, Mm, PdfLayerReference};

/// Draws the universal report footer (account total + printed timestamp).
pub fn account_footer(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    bold_font: &IndirectFontRef,
    total_amount: i32, // in cents or smallest unit
) -> Mm {
    // start 10mm from bottom
    let y = Mm(10.0);
    let font_size = 8.0;

    // Printed timestamp
    let now = Local::now();
    let ts = now.format("%-m/%-d/%Y %-I:%M:%S %p").to_string();
    layer.use_text(format!("Printed: {ts}"), font_size, Mm(5.0), y, font);

    // Account total line
    let total_value = total_amount as f64 / 100.0;
    let total_text = format!("Account Total: ${total_value:.2}");
    layer.use_text(&total_text, font_size, Mm(90.0), y, bold_font);

    y
}
