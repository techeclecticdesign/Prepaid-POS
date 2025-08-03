use printpdf::{IndirectFontRef, Mm, PdfLayerReference};

pub fn draw_line(layer: &PdfLayerReference, font: &IndirectFontRef, start_y: Mm) -> Mm {
    let font_size = 9.0;
    let x_pos = Mm(15.0);
    let y = Mm(start_y.0 - 2.0);
    let line = "_________________________________________________________________________________________________________";

    layer.use_text(line, font_size, x_pos, y, font);
    y
}
