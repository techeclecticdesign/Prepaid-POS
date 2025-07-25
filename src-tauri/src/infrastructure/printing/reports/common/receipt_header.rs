use crate::domain::models::CustomerTransaction;
use chrono::Local;
use printpdf::{IndirectFontRef, Mm, PdfLayerReference};

// Draws the top receipt header and returns the new Y offset.
pub fn receipt_header(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    bold_font: &IndirectFontRef,
    cust_tx: &CustomerTransaction,
    operator_name: &str,
    customer_name: &str,
    page_height: Mm,
) -> Mm {
    let mut y = Mm(page_height.0 - 10.0);
    let font_size = 9.0;

    // Title
    layer.use_text("Annex Receipt", font_size, Mm(22.0), y, bold_font);
    y -= Mm(10.0);

    // Timestamp
    let now = Local::now();
    let date_part = now.format("%-m/%-d/%Y").to_string();
    let time_part = now.format("%-I:%M:%S %p").to_string();
    layer.use_text(&date_part, font_size, Mm(5.0), y, font);
    layer.use_text(&time_part, font_size, Mm(50.0), y, font);
    y -= Mm(8.0);

    // Order ID
    let order_line = format!("Order ID: {}", cust_tx.order_id);
    layer.use_text(&order_line, font_size, Mm(5.0), y, font);
    y -= Mm(4.0);

    // Operator
    let op_line = format!("Operator: {operator_name} ({})", cust_tx.operator_mdoc);
    layer.use_text(&op_line, font_size, Mm(5.0), y, font);
    y -= Mm(4.0);

    // Customer
    let label = "Customer:";
    layer.use_text(label, font_size, Mm(5.0), y, font);
    let est_label_width = Mm((label.len() as f32).mul_add(1.7, 5.0));
    let display_name = if customer_name.len() > 22 {
        format!("{}…", &customer_name[..21])
    } else {
        customer_name.to_string()
    };
    let bold_text = format!(" {display_name} ({})", cust_tx.customer_mdoc);
    layer.use_text(&bold_text, font_size, est_label_width, y, bold_font);
    y -= Mm(8.0);
    y
}
