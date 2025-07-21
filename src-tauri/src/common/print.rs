use crate::common::error::AppError;
use crate::domain::models::CustomerTransaction;
use crate::interface::dto::printer_dto::PrintableLineItem;
use chrono::Local;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use std::process::Command;

// helper to send a PDF to Sumatra silently
pub fn print_pdf_silently(pdf_path: &str, printer_name: &str) -> Result<(), AppError> {
    let sumatra = r"C:\52770\new_annex\mdoc-annex-pos\src-tauri\Sumatra.exe"; // TODO: adjust this for production
    let abs_path = PathBuf::from(pdf_path)
        .canonicalize()
        .map(|p| p.to_string_lossy().trim_start_matches(r"\\?\").to_string())
        .map_err(|e| AppError::Unexpected(format!("Failed to resolve PDF path: {}", e)))?;

    let mut cmd = Command::new(sumatra);

    cmd.args([
        "-print-to",
        printer_name,
        "-silent",
        "-exit-when-done",
        "-print-settings",
        "noscale",
        abs_path.as_str(),
    ]);

    let status = cmd
        .status()
        .map_err(|e| AppError::Unexpected(format!("Failed to launch Sumatra: {}", e)))?;
    if !status.success() {
        return Err(AppError::Unexpected(format!(
            "Sumatra exited with {:?}",
            status.code()
        )));
    }

    Ok(())
}

// common header
fn receipt_header(
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
    let op_line = format!("Operator: {} ({})", operator_name, cust_tx.operator_mdoc);
    layer.use_text(&op_line, font_size, Mm(5.0), y, font);
    y -= Mm(4.0);

    // Customer
    let label = "Customer:";
    layer.use_text(label, font_size, Mm(5.0), y, font);
    let est_label_width = Mm(5.0 + label.len() as f32 * 1.7);
    let display_name = if customer_name.len() > 22 {
        format!("{}…", &customer_name[..21])
    } else {
        customer_name.to_string()
    };
    let bold_text = format!(" {} ({})", display_name, cust_tx.customer_mdoc);
    layer.use_text(&bold_text, font_size, est_label_width, y, bold_font);
    y -= Mm(8.0);
    y
}

// customer copy
pub fn print_customer_receipt(
    tx: &CustomerTransaction,
    details: &[PrintableLineItem],
    operator_name: &str,
    customer_name: &str,
    printer_name: &str,
) -> Result<(), AppError> {
    let lines = 3 + details.len() + 2;
    let dynamic_height = (lines as f32) * 7.0 + 20.0;
    let min_height = 100.0;
    let page_height = Mm(dynamic_height.max(min_height));
    let (doc, page, layer) = PdfDocument::new("Customer Receipt", Mm(80.0), page_height, "L1");

    let current = doc.get_page(page).get_layer(layer);
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let bold_font = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;
    let mut y = receipt_header(
        &current,
        &font,
        &bold_font,
        tx,
        operator_name,
        customer_name,
        page_height,
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
            format!("{:.2}", d.price as f64 / 100.0),
            8.0,
            Mm(60.0),
            y,
            &font,
        );
        y -= Mm(4.0);
    }

    // total
    y -= Mm(8.0);
    let total = details.iter().map(|d| d.quantity * d.price).sum::<i32>() as f64 / 100.0;
    current.use_text(format!("Total: {:.2}", total), 10.0, Mm(5.0), y, &bold_font);

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

    let path = "customer_receipt.pdf";
    let mut file = BufWriter::new(File::create(path)?);
    doc.save(&mut file)?; // buffer the PDF data
    drop(file); // drop writer to flush & unlock file
    print_pdf_silently(path, printer_name)?;
    Ok(())
}

// business office copy
pub fn print_business_receipt(
    tx: &CustomerTransaction,
    details: &[PrintableLineItem],
    operator_name: &str,
    customer_name: &str,
    balance: i32,
    printer_name: &str,
) -> Result<(), AppError> {
    //  Calculate a height that just fits header + items + footer:
    let lines = 3 + details.len() + 2;
    let dynamic_height = (lines as f32) * 7.0 + 20.0;
    let min_height = 100.0;
    let page_height = Mm(dynamic_height.max(min_height));
    let (doc, page, layer) = PdfDocument::new("Customer Receipt", Mm(80.0), page_height, "L1");

    let current = doc.get_page(page).get_layer(layer);
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let bold_font = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;
    let mut y = receipt_header(
        &current,
        &font,
        &bold_font,
        tx,
        operator_name,
        customer_name,
        page_height,
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
            format!("{:.2}", d.price as f64 / 100.0),
            8.0,
            Mm(60.0),
            y,
            &font,
        );
        y -= Mm(4.0);
    }

    // total
    y -= Mm(8.0);
    let total = details.iter().map(|d| d.quantity * d.price).sum::<i32>() as f64 / 100.0;
    current.use_text(format!("Total: {:.2}", total), 10.0, Mm(5.0), y, &bold_font);

    // balance
    y -= Mm(6.0);
    current.use_text(
        format!("Balance: {:.2}", balance as f64 / 100.0),
        10.0,
        Mm(5.0),
        y,
        &bold_font,
    );

    // signature line
    y -= Mm(12.0);
    current.use_text(
        "_______________________________________",
        8.0,
        Mm(5.0),
        y,
        &bold_font,
    );
    // signature label centered
    y -= Mm(4.0);
    let label = "signature";
    let text_x = Mm((80.0 - (label.len() as f32 * 8.0 * 0.35)) / 2.0);
    current.use_text(label, 8.0, text_x, y, &font);

    // centered roughly
    let text_x = Mm((80.0 - (label.len() as f32 * 8.0 * 0.35)) / 2.0);
    current.use_text(label, 8.0, text_x, y, &font);

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

    let path = "customer_receipt.pdf";
    let mut file = BufWriter::new(File::create(path)?);
    doc.save(&mut file)?; // buffer the PDF data
    drop(file); // drop writer to flush & unlock file
    print_pdf_silently(path, printer_name)?;
    Ok(())
}
