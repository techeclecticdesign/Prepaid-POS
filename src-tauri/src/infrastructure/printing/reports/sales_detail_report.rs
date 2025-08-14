use crate::common::error::AppError;
use crate::domain::report_models::product_sales::SalesTotals;
use crate::domain::report_models::sales_details::SalesReportDetails;
use crate::infrastructure::printing::reports::common::account_footer;
use crate::infrastructure::printing::reports::common::horizontal_line;
use crate::infrastructure::printing::reports::common::util::{format_cents, format_number};
use crate::infrastructure::printing::{paginator::Paginator, print::print_pdf_silently};
use chrono::NaiveDateTime;
use dotenvy::var;
use printpdf::{BuiltinFont, Mm, PdfDocument, PdfLayerReference};
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Prints a chronological list of transactions (with their detail rows)
/// between `start` and `end`.
pub fn print_sales_detail_report(
    txs: &[SalesReportDetails],
    start: NaiveDateTime,
    end: NaiveDateTime,
    sales_totals: SalesTotals,
    total_amount: i32,
    printer_name: &str,
    sumatra_location: &str,
) -> Result<(), AppError> {
    // paper/layout
    let w = Mm(210.0);
    let h = Mm(297.0);
    let margin_top = Mm(15.0);
    let margin_bottom = Mm(15.0);
    let line_height = Mm(7.0);
    let footer_height = Mm(12.0);

    // prepare title string with facility name
    let facility_name = var("CLUB_NAME").unwrap_or_default();
    let title = format!(
        "{} Transactions from {} to {}",
        facility_name,
        start.format("%Y-%m-%d"),
        end.format("%Y-%m-%d")
    );
    let title_font_size = 14.0;
    let avg_char_width_pt = title_font_size * 0.5;
    let title_width_mm = Mm(avg_char_width_pt * (title.len() as f32) * 0.3528);
    let centered_x = Mm((w.0 - title_width_mm.0) / 2.0);

    // create PDF
    let (doc, first_page, first_layer) = PdfDocument::new("Tx History", w, h, "Layer1");
    let font = Arc::new(doc.add_builtin_font(BuiltinFont::Helvetica)?);
    let bold = Arc::new(doc.add_builtin_font(BuiltinFont::HelveticaBold)?);

    // draw title flag
    let first_flag = Arc::new(AtomicBool::new(true));
    let header_printed = Arc::new(AtomicBool::new(false));
    let draw_header = {
        let facility_name = var("CLUB_NAME").unwrap_or_else(|_| "".into());
        let title = format!(
            "{facility_name} - Transactions from {} to {}",
            start.format("%Y-%m-%d"),
            end.format("%Y-%m-%d")
        );
        let flag = first_flag.clone();
        let bold = bold.clone();
        let header_printed = header_printed.clone();
        move |layer: &PdfLayerReference| {
            let mut y = h - margin_top;
            // on first invocation only:
            let first = flag.swap(false, Ordering::SeqCst); // local `first` avoids double-swap
            if first {
                layer.use_text(&title, title_font_size, centered_x, y, &bold);
                y -= line_height;
            }
            layer.use_text("Order#", 10.0, Mm(15.0), y, &bold);
            layer.use_text("Date", 10.0, Mm(40.0), y, &bold);
            layer.use_text("Customer", 10.0, Mm(80.0), y, &bold);
            layer.use_text("Items", 10.0, Mm(140.0), y, &bold);
            layer.use_text("Total", 10.0, Mm(165.0), y, &bold);

            // Draw underlines + grand totals right below headers on first page
            if first {
                // totals row
                y -= line_height;
                layer.use_text(
                    format_number(sales_totals.total_quantity),
                    9.0,
                    Mm(140.0),
                    y,
                    &bold,
                );
                layer.use_text(
                    format_cents(sales_totals.total_value),
                    9.0,
                    Mm(165.0),
                    y,
                    &bold,
                );

                // small gap before first data row
                y -= Mm(6.0);
                header_printed.store(true, Ordering::SeqCst);
            }
        }
    };
    // footer closure
    let draw_footer = |layer: &PdfLayerReference| {
        account_footer::account_footer(layer, &font, &bold, total_amount);
    };

    // paginate
    {
        // estimate of how much vertical space the header block consumes on first page
        let header_space = line_height * 1.8;

        let mut pg = Paginator::new(
            &doc,
            first_page,
            first_layer,
            w,
            h,
            margin_top,
            margin_bottom,
            line_height,
            footer_height,
            draw_header,
            draw_footer,
        );
        // advance past the header+totals block so first data row starts lower
        pg.advance(header_space);
        let mut first_tx = true;
        for t in txs {
            let tx = &t.tx;
            let lines_needed = 1 + t.details.len();
            let needed_height = line_height * (lines_needed as f32) + line_height;
            // ask paginator for a page that can fit the whole block
            let layer = pg.layer_for(needed_height);
            if first_tx && header_printed.load(Ordering::SeqCst) {
                // position just below totals
                horizontal_line::draw_line(&layer, &font, pg.current_y() + Mm(6.8));
                first_tx = false;
            } else {
                horizontal_line::draw_line(&layer, &font, pg.current_y() + line_height);
            }
            let layer = pg.layer_for(line_height);
            // print the header row for this tx
            layer.use_text(
                tx.order_id.to_string(),
                9.0,
                Mm(15.0),
                pg.current_y(),
                &bold,
            );
            layer.use_text(
                tx.date
                    .expect("tx.date should be present")
                    .format("%Y-%m-%d")
                    .to_string(),
                9.0,
                Mm(40.0),
                pg.current_y(),
                &bold,
            );
            layer.use_text(
                format!("{} ({})", t.customer_name, tx.customer_mdoc),
                9.0,
                Mm(80.0),
                pg.current_y(),
                &bold,
            );
            layer.use_text(
                t.item_count.to_string(),
                9.0,
                Mm(140.0),
                pg.current_y(),
                &bold,
            );
            layer.use_text(
                format_cents(t.order_total),
                9.0,
                Mm(165.0),
                pg.current_y(),
                &bold,
            );
            pg.advance(line_height);

            // print each detail row
            for d in &t.details {
                let layer = pg.layer_for(line_height);
                layer.use_text(&d.upc, 8.0, Mm(40.0), pg.current_y(), &font);
                layer.use_text(&d.product_name, 8.0, Mm(80.0), pg.current_y(), &font);
                layer.use_text(
                    d.quantity.to_string(),
                    8.0,
                    Mm(140.0),
                    pg.current_y(),
                    &font,
                );
                layer.use_text(format_cents(d.price), 8.0, Mm(165.0), pg.current_y(), &font);
                pg.advance(line_height);
            }
        }

        // grand totals at the bottom
        let sep_layer = pg.layer_for(Mm(7.0));
        // first underline
        sep_layer.use_text("____", 9.0, Mm(140.0), pg.current_y(), &font);
        sep_layer.use_text("__________", 9.0, Mm(165.0), pg.current_y(), &font);
        // double-line
        pg.advance(Mm(0.4));
        sep_layer.use_text("____", 9.0, Mm(140.0), pg.current_y(), &font);
        sep_layer.use_text("__________", 9.0, Mm(165.0), pg.current_y(), &font);
        pg.advance(Mm(5.0));

        let tot_layer = pg.layer_for(line_height);
        tot_layer.use_text(
            format_number(sales_totals.total_quantity),
            9.0,
            Mm(140.0),
            pg.current_y(),
            &bold,
        );
        tot_layer.use_text("Total:", 9.0, Mm(154.0), pg.current_y(), &bold);
        tot_layer.use_text(
            format_cents(sales_totals.total_value),
            9.0,
            Mm(165.0),
            pg.current_y(),
            &bold,
        );

        // finish PDF
        pg.finalize();
        pg.draw_page_numbers(&font);
    }

    // save & print
    let path = "sales_details_report.pdf";
    let mut file = std::fs::File::create(path)?;
    let mut buf = std::io::BufWriter::new(&mut file);
    doc.save(&mut buf)?;
    buf.flush()?;

    std::thread::spawn({
        let printer = printer_name.to_string();
        let sumatra_location = sumatra_location.to_string();
        move || {
            if let Err(e) = print_pdf_silently(path, &printer, &sumatra_location) {
                log::error!("Print failed: {e}");
            }
        }
    });

    Ok(())
}
