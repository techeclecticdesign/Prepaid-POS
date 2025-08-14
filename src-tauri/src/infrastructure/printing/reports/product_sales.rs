use crate::common::error::AppError;
use crate::domain::report_models::product_sales::{ProductSalesByCategory, SalesTotals};
use crate::infrastructure::printing::paginator::Paginator;
use crate::infrastructure::printing::print::print_pdf_silently;
use crate::infrastructure::printing::reports::common::account_footer;
use crate::infrastructure::printing::reports::common::util::{format_cents, format_number};
use chrono::NaiveDateTime;
use dotenvy::var;
use printpdf::{BuiltinFont, Mm, PdfDocument, PdfLayerReference};
use std::io::Write;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

pub fn print_product_sales(
    rows: &[ProductSalesByCategory],
    start: NaiveDateTime,
    end: NaiveDateTime,
    sales_totals: SalesTotals,
    total_amount: i32,
    printer_name: &str,
    sumatra_location: &str,
) -> Result<(), AppError> {
    let page_width = Mm(210.0);
    let page_height = Mm(297.0);
    let margin_top = Mm(15.0);
    let margin_bottom = Mm(15.0);
    let line_height = Mm(7.0);
    let footer_height = Mm(14.0);

    let (doc, first_page, first_layer) =
        PdfDocument::new("Sales by Category", page_width, page_height, "Layer1");

    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

    // clone for closures
    let font_cl = font.clone();
    let bold_cl = bold.clone();

    // title
    let facility = var("CLUB_NAME").unwrap_or_default();
    let title = format!(
        "{facility} Sales from {} to {}",
        start.format("%Y-%m-%d"),
        end.format("%Y-%m-%d")
    );
    let title_size = 14.0;
    let avg_char_w_pt = title_size * 0.5;
    let title_w_mm = Mm(avg_char_w_pt * (title.len() as f32) * 0.3528);
    let title_x = Mm((page_width.0 - title_w_mm.0) / 2.0);

    // header draws once-per-page
    let first_flag = Arc::new(AtomicBool::new(true));
    let draw_header = {
        let first_flag = first_flag.clone();
        let bold = bold_cl.clone();
        move |layer: &PdfLayerReference| {
            let mut y = page_height - margin_top;
            // title - only on first page
            if first_flag.swap(false, Ordering::SeqCst) {
                layer.use_text(&title, title_size, title_x, y, &bold);
                y -= Mm(line_height.0 * 1.1);
            }
            // column labels
            layer.use_text("Qty", 11.0, Mm(10.0), y, &bold);
            layer.use_text("Product", 11.0, Mm(30.0), y, &bold);
            layer.use_text("UPC", 11.0, Mm(100.0), y, &bold);
            layer.use_text("Price", 11.0, Mm(140.0), y, &bold);
            layer.use_text("Total", 11.0, Mm(170.0), y, &bold);
        }
    };

    // footer draws category‐independent total_amount
    let draw_footer = {
        let font = font_cl.clone();
        let bold = bold_cl.clone();
        move |layer: &PdfLayerReference| {
            account_footer::account_footer(layer, &font, &bold, total_amount);
        }
    };

    // paginate and draw all rows
    {
        let mut pg = Paginator::new(
            &doc,
            first_page,
            first_layer,
            page_width,
            page_height,
            margin_top,
            margin_bottom,
            line_height,
            footer_height,
            draw_header,
            draw_footer,
        );

        let mut last_cat: Option<&str> = None;

        pg.advance(line_height * 1.3);

        for (i, r) in rows.iter().enumerate() {
            // Category header (only before the first non‐summary of each category)
            if !r.is_summary && last_cat != Some(r.category.as_str()) {
                let layer = pg.layer_for(Mm(line_height.0 * 2.0));
                layer.use_text(r.category.as_str(), 12.0, Mm(10.0), pg.current_y(), &bold);
                pg.advance(Mm(line_height.0 * 1.5));
                last_cat = Some(r.category.as_str());
            }

            // Draw either summary or detail
            let layer = pg.layer_for(line_height);
            if r.is_summary {
                pg.advance(line_height * -0.8);
                if i > 1 {
                    let qty_line_layer = pg.layer_for(line_height);
                    qty_line_layer.use_text("____", 11.0, Mm(10.0), pg.current_y(), &font);
                    let total_line_layer = pg.layer_for(line_height);
                    total_line_layer.use_text("_______", 11.0, Mm(170.0), pg.current_y(), &font);
                }
                // category‐total line in bold (Qty + Total only)
                layer.use_text(
                    format_number(r.quantity_sold),
                    9.0,
                    Mm(10.0),
                    pg.current_y() - Mm(line_height.0) + Mm(2.0),
                    &bold,
                );
                layer.use_text(
                    format_cents(r.total_sales),
                    9.0,
                    Mm(170.0),
                    pg.current_y() - Mm(line_height.0) + Mm(2.0),
                    &bold,
                );
                pg.advance(line_height * 1.4);
            } else {
                // regular detail row
                layer.use_text(
                    format_number(r.quantity_sold),
                    9.0,
                    Mm(10.0),
                    pg.current_y(),
                    &font,
                );
                layer.use_text(&r.name, 9.0, Mm(30.0), pg.current_y(), &font);
                layer.use_text(&r.upc, 9.0, Mm(100.0), pg.current_y(), &font);
                layer.use_text(format_cents(r.price), 9.0, Mm(140.0), pg.current_y(), &font);
                layer.use_text(
                    format_cents(r.total_sales),
                    9.0,
                    Mm(170.0),
                    pg.current_y(),
                    &font,
                );
            }

            if i < rows.len() - 1 {
                pg.advance(line_height);
            }
        }

        // grand totals
        pg.advance(Mm(2.0));
        let sep_layer = pg.layer_for(Mm(7.0));
        // first line
        sep_layer.use_text("____", 9.0, Mm(10.0), pg.current_y(), &font);
        sep_layer.use_text("__________", 9.0, Mm(170.0), pg.current_y(), &font);
        // double line
        pg.advance(Mm(0.4));
        sep_layer.use_text("____", 9.0, Mm(10.0), pg.current_y(), &font);
        sep_layer.use_text("__________", 9.0, Mm(170.0), pg.current_y(), &font);
        pg.advance(Mm(5.0));

        let tot_layer = pg.layer_for(line_height);
        tot_layer.use_text(
            format_number(sales_totals.total_quantity),
            9.0,
            Mm(10.0),
            pg.current_y(),
            &bold,
        );
        tot_layer.use_text("Grand Total:", 9.0, Mm(150.0), pg.current_y(), &bold);
        tot_layer.use_text(
            format_cents(sales_totals.total_value),
            9.0,
            Mm(170.0),
            pg.current_y(),
            &bold,
        );

        // finish and paginate
        pg.finalize();
        pg.draw_page_numbers(&font);
    }

    // save & send to printer
    let path = "sales_by_category.pdf";
    let mut file = std::io::BufWriter::new(std::fs::File::create(path)?);
    doc.save(&mut file)?;
    file.flush()?;

    let printer = printer_name.to_string();
    let sumatra_location = sumatra_location.to_string();
    std::thread::spawn(move || {
        if let Err(e) = print_pdf_silently(path, &printer, &sumatra_location) {
            log::error!("Print failed: {e}");
        }
    });

    Ok(())
}
