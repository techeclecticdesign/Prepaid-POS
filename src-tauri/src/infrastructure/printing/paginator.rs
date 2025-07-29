use printpdf::{
    IndirectFontRef, Mm, PdfDocumentReference, PdfLayerIndex, PdfLayerReference, PdfPageIndex,
};

// A simple Y‐driven PDF paginator.
// You feed it your header closure and footer closure, along
// with page metrics, then call `layer_for(required_height)`
// before each draw, and `advance(dy)` after.
pub struct Paginator<'a> {
    doc: &'a PdfDocumentReference,
    pages: Vec<(PdfPageIndex, PdfLayerIndex)>,
    margin_top: Mm,
    margin_bottom: Mm,
    footer_height: Mm,
    line_height: Mm,
    page_width: Mm,
    page_height: Mm,
    next_y: Mm,
    draw_header: Box<dyn Fn(&PdfLayerReference) + 'a>,
    draw_footer: Box<dyn Fn(&PdfLayerReference) + 'a>,
}

impl<'a> Paginator<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        doc: &'a PdfDocumentReference,
        first_page: PdfPageIndex,
        first_layer: PdfLayerIndex,
        page_width: Mm,
        page_height: Mm,
        margin_top: Mm,
        margin_bottom: Mm,
        line_height: Mm,
        footer_height: Mm,
        draw_header: impl Fn(&PdfLayerReference) + 'a,
        draw_footer: impl Fn(&PdfLayerReference) + 'a,
    ) -> Self {
        let pages = vec![(first_page, first_layer)];
        let this = Self {
            doc,
            pages,
            margin_top,
            margin_bottom,
            footer_height,
            line_height,
            page_width,
            page_height,
            next_y: page_height - margin_top - line_height,
            draw_header: Box::new(draw_header),
            draw_footer: Box::new(draw_footer),
        };
        // draw header on first page
        {
            let (p, l) = this.pages[0];
            let layer = this.doc.get_page(p).get_layer(l);
            (this.draw_header)(&layer);
        }
        this
    }

    // Ensure there's room for `needed` more Mm.  If not, finish current
    // page (footer), create a new one, draw its header, and reset `y`.
    // Returns the fresh layer reference.
    pub fn layer_for(&mut self, needed: Mm) -> PdfLayerReference {
        let bottom_limit = self.margin_bottom + self.footer_height;
        if self.next_y < bottom_limit + needed {
            // finish footer on old page
            let &(p_old, l_old) = self.pages.last().unwrap();
            let foot_layer = self.doc.get_page(p_old).get_layer(l_old);
            (self.draw_footer)(&foot_layer);

            // new page
            let idx = self.pages.len();
            let (p, l) = self.doc.add_page(
                self.page_width,
                self.page_height,
                format!("Layer{}", idx + 1),
            );
            self.pages.push((p, l));
            let layer = self.doc.get_page(p).get_layer(l);
            // draw header
            (self.draw_header)(&layer);
            // reset y
            self.next_y = self.page_height - self.margin_top - self.line_height;
            layer
        } else {
            let &(p, l) = self.pages.last().unwrap();
            self.doc.get_page(p).get_layer(l)
        }
    }

    // Move down by `dy` Mm after drawing
    pub fn advance(&mut self, dy: Mm) {
        self.next_y -= dy;
    }

    // After all content, draw the footer on the last page
    pub fn finalize(&mut self) {
        let &(p, l) = self.pages.last().unwrap();
        let layer = self.doc.get_page(p).get_layer(l);
        (self.draw_footer)(&layer);
    }

    // Iterate pages in creation order
    pub fn pages(&self) -> &[(PdfPageIndex, PdfLayerIndex)] {
        &self.pages
    }

    pub fn current_y(&self) -> Mm {
        self.next_y
    }

    // Draw "Page X of Y" in bottom‑right of each page.
    pub fn draw_page_numbers(&self, font: &IndirectFontRef) {
        let total = self.pages.len();
        for (i, &(page_idx, layer_idx)) in self.pages.iter().enumerate() {
            let layer = self.doc.get_page(page_idx).get_layer(layer_idx);
            layer.use_text(
                format!("Page {} of {}", i + 1, total),
                8.0,
                Mm(self.page_width.0 - 30.0),
                Mm(10.0),
                font,
            );
        }
    }
}
