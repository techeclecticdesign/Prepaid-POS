use crate::common::error::AppError;
use crate::domain::models::Product;
use crate::infrastructure::printing::print::print_pdf_silently;
use crate::infrastructure::printing::reports::common::util::truncate_desc;
use dotenvy::var;
use printpdf::{BuiltinFont, Mm, PdfDocument};
use std::io::Write;

pub fn print_product_catalog_report(
    rows: &[Product],
    printer_name: &str,
    sumatra_location: &str,
) -> Result<(), AppError> {
    // page & layout constants
    let page_w = Mm(210.0);
    let page_h = Mm(297.0);
    let margin_top = Mm(25.0);
    let margin_bot = Mm(15.0);
    let line_h = Mm(5.0);

    // build centered title + club name
    let facility = var("CLUB_NAME").unwrap_or_default();
    let title = format!("{facility} Product Catalog");
    let title_size = 14.0;
    let avg_char_w_mm = title_size * 0.5 * 0.3528;
    let title_w = Mm(avg_char_w_mm * title.len() as f32);
    let title_x = Mm((page_w.0 - title_w.0) / 2.0);
    let title_y = page_h - margin_top;

    // three column X offsets
    let col_x = [Mm(10.0), Mm(10.0 + 67.0), Mm(10.0 + 134.0)];

    // make the PDF
    let (doc, first_page, first_layer) =
        PdfDocument::new("Product Catalog", page_w, page_h, "Layer1");
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

    // break the full list into pages manually
    let mut page_idx = first_page;
    let mut page_number = 2;
    let mut layer_idx = first_layer;
    let mut layer = doc.get_page(page_idx).get_layer(layer_idx);

    // draw title once on the first page
    layer.use_text(&title, title_size, title_x, title_y, &bold);
    // push down the three columns' starting Y so they don’t overlap the title
    let mut y_pos = [
        page_h - margin_top - Mm(title_size * 1.5), /* left col */
        page_h - margin_top - Mm(title_size * 1.5), /* middle */
        page_h - margin_top - Mm(title_size * 1.5),
    ]; /* right */

    // track independent Y for each column
    let mut last_cat = [None; 3];
    let mut col = 0;

    for r in rows {
        // figure out how many lines this record needs
        // header + item = 2 lines, otherwise just 1
        let needed = if last_cat[col] != Some(r.category.as_str()) {
            line_h * 2.0
        } else {
            line_h
        };
        // if we don't have space for all of it, skip to next column/page
        if y_pos[col] - needed < margin_bot {
            col += 1;
            if col >= 3 {
                // all 3 columns used up -> new page
                let (np, nl) = doc.add_page(page_w, page_h, format!("Layer{page_number}"));
                page_number += 1;
                page_idx = np;
                layer_idx = nl;
                layer = doc.get_page(page_idx).get_layer(layer_idx);
                // reset for the new page
                y_pos = [page_h - margin_top; 3];
                last_cat = [None; 3];
                col = 0;
            }
        }

        // if this column is already past bottom, start a new page
        if y_pos[col] < margin_bot {
            // spawn new page
            let (np, nl) = doc.add_page(page_w, page_h, format!("Layer{page_number}"));
            page_number += 1;
            page_idx = np;
            layer_idx = nl;
            layer = doc.get_page(page_idx).get_layer(layer_idx);

            // reset for new page
            y_pos = [page_h - margin_top; 3];
            last_cat = [None; 3];
        }

        // current Y in that column
        let y = y_pos[col];

        // print category header if it changed
        if last_cat[col] != Some(r.category.as_str()) {
            layer.use_text(&r.category, 9.0, col_x[col], y, &bold);
            // consume one line
            y_pos[col] -= line_h;
            // then print the item on next line
            let y2 = y_pos[col];
            layer.use_text(
                format!("${:.2}", r.price as f64 / 100.0),
                7.0,
                col_x[col],
                y2,
                &font,
            );
            let desc = truncate_desc(&r.desc, 40);
            layer.use_text(desc, 7.0, col_x[col] + Mm(8.0), y2, &font);
            y_pos[col] -= line_h;
            last_cat[col] = Some(r.category.as_str());
        } else {
            // just print the item
            layer.use_text(
                format!("${:.2}", r.price as f64 / 100.0),
                7.0,
                col_x[col],
                y,
                &font,
            );
            let desc = truncate_desc(&r.desc, 40);
            layer.use_text(desc, 7.0, col_x[col] + Mm(8.0), y, &font);
            y_pos[col] -= line_h;
        }
    }

    // save & print
    let path = "product_catalog.pdf";
    let mut file = std::io::BufWriter::new(std::fs::File::create(path)?);
    doc.save(&mut file)?;
    file.flush()?;

    let printer = printer_name.to_string();
    let sumatra_location = sumatra_location.to_string();
    std::thread::spawn(move || {
        if let Err(e) = print_pdf_silently(path, &printer, &sumatra_location) {
            log::error!("print failed: {e}");
        }
    });

    Ok(())
}
