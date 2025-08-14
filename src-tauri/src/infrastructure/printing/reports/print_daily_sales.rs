use crate::common::error::AppError;
use crate::domain::report_models::daily_sales::DailySales;
use crate::domain::report_models::product_sales::SalesTotals;
use crate::infrastructure::printing::paginator::Paginator;
use crate::infrastructure::printing::print::print_pdf_silently;
use crate::infrastructure::printing::reports::common::{account_footer, util::format_cents};
use chrono::NaiveDateTime;
use dotenvy::var;
use printpdf::{BuiltinFont, Mm, PdfDocument, PdfLayerReference};
use std::io::Write;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

// Prints a “Sales by Day” report PDF and sends to printer.
pub fn print_daily_sales(
    rows: &[DailySales],
    start: NaiveDateTime,
    end: NaiveDateTime,
    sales_totals: SalesTotals,
    total_amount: i32,
    printer_name: &str,
    sumatra_location: &str,
) -> Result<(), AppError> {
    // layout constants
    let page_width = Mm(210.0);
    let page_height = Mm(297.0);
    let margin_top = Mm(15.0);
    let margin_bottom = Mm(15.0);
    let line_height = Mm(7.0);
    let footer_height = Mm(12.0);

    // column x positions
    let date_x1 = Mm(20.0);
    let total_x1 = Mm(55.0);
    let date_x2 = Mm(125.0);
    let total_x2 = Mm(160.0);

    // create PDF
    let (doc, first_page, first_layer) =
        PdfDocument::new("Sales by Day", page_width, page_height, "Layer1");
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

    // facility + title
    let facility = var("CLUB_NAME").unwrap_or_default();
    let title = format!(
        "{facility} Sales by Day from {} to {}",
        start.format("%Y-%m-%d"),
        end.format("%Y-%m-%d")
    );
    let title_size = 14.0;
    // rough width calculation for centering
    let avg_char_w_pt = title_size * 0.5;
    let title_w_mm = Mm(avg_char_w_pt * (title.len() as f32) * 0.3528);
    let title_x = Mm((page_width.0 - title_w_mm.0) / 2.0);

    // first-page-only flag
    let first_flag = Arc::new(AtomicBool::new(true));

    // header closure
    let draw_header = {
        let first_flag = first_flag.clone();
        let bold = bold.clone();
        let title = title.clone();
        move |layer: &PdfLayerReference| {
            let mut y = page_height - margin_top;
            if first_flag.swap(false, Ordering::SeqCst) {
                layer.use_text(&title, title_size, title_x, y, &bold);
                y -= Mm(line_height.0 * 2.0);
            }
            // draw column headers for both left & right columns
            layer.use_text("Date", 11.0, date_x1, y, &bold);
            layer.use_text("Total", 11.0, total_x1, y, &bold);
            layer.use_text("Date", 11.0, date_x2, y, &bold);
            layer.use_text("Total", 11.0, total_x2, y, &bold);
        }
    };

    // footer closure
    let draw_footer = {
        let font = font.clone();
        let bold = bold.clone();
        move |layer: &PdfLayerReference| {
            account_footer::account_footer(layer, &font, &bold, total_amount);
        }
    };

    // paginate & draw rows
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

        pg.advance(line_height * 2.0);

        // compute how many rows fit vertically
        let start_y = pg.current_y();
        let usable_mm = (start_y.0 - margin_bottom.0 - footer_height.0).max(0.0);
        let mut rows_per_column = (usable_mm / line_height.0).floor() as usize;
        if rows_per_column == 0 {
            rows_per_column = 1;
        }

        let mut idx = 0usize;
        while idx < rows.len() {
            // for each vertical slot (0..rows_per_column)
            for slot in 0..rows_per_column {
                let left_idx = idx + slot;
                let right_idx = idx + slot + rows_per_column;

                // If both indices are past the end, we're done for this page
                if left_idx >= rows.len() && right_idx >= rows.len() {
                    break;
                }

                let layer = pg.layer_for(line_height);

                // left column entry
                if left_idx < rows.len() {
                    let r = &rows[left_idx];
                    let date_str = r.day.format("%Y-%m-%d").to_string();
                    layer.use_text(&date_str, 9.0, date_x1, pg.current_y(), &font);
                    layer.use_text(
                        format_cents(r.total_sales),
                        9.0,
                        total_x1,
                        pg.current_y(),
                        &font,
                    );
                }

                // right column entry (if present)
                if right_idx < rows.len() {
                    let r = &rows[right_idx];
                    let date_str = r.day.format("%Y-%m-%d").to_string();
                    layer.use_text(&date_str, 9.0, date_x2, pg.current_y(), &font);
                    layer.use_text(
                        format_cents(r.total_sales),
                        9.0,
                        total_x2,
                        pg.current_y(),
                        &font,
                    );
                }

                // advance one slot (shared for both columns)
                pg.advance(line_height);
            }

            // advance index by the number of rows we consumed on this page
            idx += rows_per_column * 2;
        }

        // bottom grand totals with double underline
        pg.advance(Mm(2.0));
        let sep_layer = pg.layer_for(Mm(7.0));
        // first underline pair (left & right totals)
        sep_layer.use_text("____", 9.0, total_x1, pg.current_y(), &font);
        sep_layer.use_text("________", 9.0, total_x2, pg.current_y(), &font);
        // small vertical nudge, then second underline pair to make it a double-line
        pg.advance(Mm(0.4));
        sep_layer.use_text("____", 9.0, total_x1, pg.current_y(), &font);
        sep_layer.use_text("________", 9.0, total_x2, pg.current_y(), &font);
        pg.advance(Mm(5.0));

        // draw the grand total values (quantity + money) using the SalesTotals struct
        let tot_layer = pg.layer_for(line_height);

        tot_layer.use_text("Total Quantity:", 9.0, Mm(25.0), pg.current_y(), &bold);

        tot_layer.use_text(
            sales_totals.total_quantity.to_string(),
            9.0,
            total_x1,
            pg.current_y(),
            &bold,
        );

        tot_layer.use_text("Grand Total:", 9.0, Mm(133.0), pg.current_y(), &bold);

        tot_layer.use_text(
            format_cents(sales_totals.total_value),
            9.0,
            total_x2,
            pg.current_y(),
            &bold,
        );

        // finish PDF
        pg.finalize();
        pg.draw_page_numbers(&font);
    }

    // save & dispatch print
    let path = "sales_by_day_report.pdf";
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
