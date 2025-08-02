use crate::common::error::AppError;
use crate::domain::models::Product;
use crate::infrastructure::printing::print::print_pdf_silently;
use crate::infrastructure::printing::reports::common::util::truncate_desc;
use printpdf::{BuiltinFont, Mm, PdfDocument};
use std::io::Write;

pub fn print_product_catalog_report(rows: &[Product], printer_name: &str) -> Result<(), AppError> {
    // page & layout constants
    let page_w = Mm(210.0);
    let page_h = Mm(297.0);
    let margin_top = Mm(25.0);
    let margin_bot = Mm(15.0);
    let line_h = Mm(5.0);

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

    // track independent Y for each column
    let mut y_pos = [page_h - margin_top; 3];
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
    std::thread::spawn(move || {
        if let Err(e) = print_pdf_silently(path, &printer) {
            log::error!("print failed: {e}");
        }
    });

    Ok(())
}
