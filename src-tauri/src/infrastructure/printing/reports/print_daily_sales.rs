use crate::common::error::AppError;
use crate::domain::report_models::daily_sales::DailySales;
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

/// Prints a “Sales by Day” report PDF and sends to printer.
/// Columns: Date | Total
pub fn print_daily_sales(
    rows: &[DailySales],
    start: NaiveDateTime,
    end: NaiveDateTime,
    total_amount: i32,
    printer_name: &str,
) -> Result<(), AppError> {
    // layout constants
    let page_width = Mm(210.0);
    let page_height = Mm(297.0);
    let margin_top = Mm(15.0);
    let margin_bottom = Mm(15.0);
    let line_height = Mm(7.0);
    let footer_height = Mm(12.0);

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
                y -= Mm(line_height.0 * 1.5);
            }
            layer.use_text("Date", 11.0, Mm(20.0), y, &bold);
            layer.use_text("Total", 11.0, Mm(140.0), y, &bold);
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

        pg.advance(line_height * 1.5);

        for (i, r) in rows.iter().enumerate() {
            let layer = pg.layer_for(line_height);
            // date column
            let date_str = r.day.format("%Y-%m-%d").to_string();
            layer.use_text(&date_str, 9.0, Mm(20.0), pg.current_y(), &font);
            // total column
            layer.use_text(
                format_cents(r.total_sales),
                9.0,
                Mm(140.0),
                pg.current_y(),
                &font,
            );
            if i < rows.len() - 1 {
                pg.advance(line_height);
            }
        }

        pg.finalize();
        pg.draw_page_numbers(&font);
    }

    // save & dispatch print
    let path = "sales_by_day_report.pdf";
    let mut file = std::io::BufWriter::new(std::fs::File::create(path)?);
    doc.save(&mut file)?;
    file.flush()?;

    let printer = printer_name.to_string();
    std::thread::spawn(move || {
        if let Err(e) = print_pdf_silently(path, &printer) {
            log::error!("Print failed: {e}");
        }
    });

    Ok(())
}
