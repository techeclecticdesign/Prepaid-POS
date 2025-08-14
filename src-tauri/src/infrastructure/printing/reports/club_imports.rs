use crate::common::error::AppError;
use crate::domain::models::club_import::ClubImport;
use crate::domain::report_models::club_import_report::{ClubTransactionRow, PeriodTotals};
use crate::infrastructure::printing::paginator::Paginator;
use crate::infrastructure::printing::print::print_pdf_silently;
use crate::infrastructure::printing::reports::common::account_footer;
use crate::infrastructure::printing::reports::common::util::format_cents;
use dotenvy::var;
use printpdf::{BuiltinFont, Mm, PdfDocument, PdfLayerReference};
use std::io::Write;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

pub fn print_club_import_report(
    import: &ClubImport,
    txs: &[ClubTransactionRow],
    total_amount: i32,
    totals: &PeriodTotals,
    printer_name: &str,
    sumatra_location: &str,
) -> Result<(), AppError> {
    let page_width = Mm(210.0);
    let page_height = Mm(297.0);
    let margin_top = Mm(15.0);
    let margin_bottom = Mm(15.0);
    let line_height = Mm(7.0);
    let footer_height = Mm(12.0);

    // Create document & fonts
    let (doc, first_page, first_layer) =
        PdfDocument::new("Club Import Report", page_width, page_height, "Layer1");
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

    // Title
    let facility = var("CLUB_NAME").unwrap_or_default();
    let title = format!(
        "{facility} Club Exception Report  {} - {}",
        import.activity_from.format("%Y/%m/%d"),
        import.activity_to.format("%Y/%m/%d"),
    );
    let title_size = 16.0;
    // Rough char-width estimate to center
    let avg_w_pt = title_size * 0.5;
    let title_w_mm = Mm(avg_w_pt * (title.len() as f32) * 0.3528);
    let title_x = Mm((page_width.0 - title_w_mm.0) / 2.0);

    // Header
    let first_flag = Arc::new(AtomicBool::new(true));
    let draw_header = {
        let first_flag = first_flag.clone();
        let bold = bold.clone();
        move |layer: &PdfLayerReference| {
            let y: Mm = page_height - margin_top;
            // draw title only on first page
            if first_flag.swap(false, Ordering::SeqCst) {
                layer.use_text(&title, title_size, title_x, y, &bold);
            }
        }
    };

    // Footer
    let draw_footer = {
        let font = font.clone();
        let bold = bold.clone();
        move |layer: &PdfLayerReference| {
            account_footer::account_footer(layer, &font, &bold, total_amount);
        }
    };

    // Pagination
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

        pg.advance(Mm(line_height.0 * 1.5));

        // Club Import section
        let period_net = totals.period_pos_sum + totals.period_neg_sum;
        let left1 = import.source_file.clone();
        let last_running_total = txs.last().map(|r| r.running_total).unwrap_or(period_net);
        let left2 = format_cents(last_running_total);
        let left3 = format!(
            "{} - {}",
            import.activity_from.format("%Y-%m-%d"),
            import.activity_to.format("%Y-%m-%d")
        );
        let right1 = format_cents(totals.period_pos_sum);
        let right2 = format_cents(totals.period_neg_sum);
        let right3 = format_cents(period_net);

        for i in 0..3 {
            let layer = pg.layer_for(line_height);
            // Left column values
            let (label_l, value_l) = match i {
                0 => ("Source File", left1.as_str()),
                1 => ("Balance", left2.as_str()),
                2 => ("Date Range", left3.as_str()),
                _ => unreachable!(),
            };
            // Right column values
            let (label_r, value_r) = match i {
                0 => ("Customer Deposits", right1.as_str()),
                1 => ("Customer Withdrawals", right2.as_str()),
                2 => ("Net Customer Change", right3.as_str()),
                _ => unreachable!(),
            };

            if i == 2 {
                let line_layer = pg.layer_for(line_height);
                line_layer.use_text(
                    "_______",
                    11.0,
                    Mm(176.0),
                    pg.current_y() + Mm(line_height.0 * 0.75),
                    &font,
                );
            }
            // Left column
            layer.use_text(format!("{label_l}:"), 11.0, Mm(15.0), pg.current_y(), &bold);
            layer.use_text(value_l.to_string(), 11.0, Mm(50.0), pg.current_y(), &font);
            // Right column
            layer.use_text(
                format!("{label_r}:"),
                11.0,
                Mm(125.0),
                pg.current_y(),
                &bold,
            );
            layer.use_text(value_r.to_string(), 11.0, Mm(175.0), pg.current_y(), &font);
            pg.advance(line_height);
        }

        pg.advance(line_height * 1.8);

        // Column headers
        {
            let layer = pg.layer_for(line_height);
            layer.use_text("Date", 11.0, Mm(15.0), pg.current_y(), &bold);
            layer.use_text("Tx Type", 11.0, Mm(45.0), pg.current_y(), &bold);
            layer.use_text("Received From", 11.0, Mm(85.0), pg.current_y(), &bold);
            layer.use_text("Amount", 11.0, Mm(145.0), pg.current_y(), &bold);
            layer.use_text("Available", 11.0, Mm(175.0), pg.current_y(), &bold);
        }
        pg.advance(line_height);

        // Rows
        for (i, row) in txs.iter().enumerate() {
            let tx = &row.tx;
            let layer = pg.layer_for(line_height);

            let date_str = tx.date.format("%Y-%m-%d").to_string();
            layer.use_text(&date_str, 9.0, Mm(15.0), pg.current_y(), &font);

            layer.use_text(
                format!("{:?}", tx.tx_type),
                9.0,
                Mm(45.0),
                pg.current_y(),
                &font,
            );

            let from = if let Some(m) = tx.mdoc {
                format!("{} ({})", tx.entity_name, m)
            } else {
                tx.entity_name.clone()
            };
            layer.use_text(&from, 9.0, Mm(85.0), pg.current_y(), &font);

            layer.use_text(
                format_cents(tx.amount),
                9.0,
                Mm(145.0),
                pg.current_y(),
                &font,
            );

            layer.use_text(
                format_cents(row.running_total),
                9.0,
                Mm(175.0),
                pg.current_y(),
                &font,
            );

            // advance unless last
            if i < txs.len() - 1 {
                pg.advance(line_height);
            }
        }

        // finalize last page
        pg.finalize();
        pg.draw_page_numbers(&font);
    }

    // save and print
    let path = "club_import_report.pdf";
    let mut f = std::io::BufWriter::new(std::fs::File::create(path)?);
    doc.save(&mut f)?;
    f.flush()?;

    let printer = printer_name.to_string();
    let sumatra_location = sumatra_location.to_string();
    std::thread::spawn(move || {
        if let Err(e) = print_pdf_silently(path, &printer, &sumatra_location) {
            log::error!("Print failed: {e}");
        }
    });

    Ok(())
}
