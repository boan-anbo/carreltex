/// Render a deterministic, single-font PDF preview for a v0 DVI-v2 text page.
///
/// This is a "preview" renderer for CarrelTeX v0 artifacts:
/// - It treats the DVI-v2 bytes as CarrelTeX's strict text-page format.
/// - It reconstructs per-line ASCII text and draws it using a standard PDF font.
/// - It does **not** attempt TeX typography; it is a stable "what did we extract" view.
pub fn render_dvi_v2_text_page_to_pdf_v0(bytes: &[u8]) -> Option<Vec<u8>> {
    if bytes.is_empty() {
        return None;
    }
    let line_advance_sp = infer_line_advance_sp_v0(bytes);
    let layout = parse_dvi_v2_text_page_to_layout_v0(bytes, line_advance_sp)?;
    if layout.pages.is_empty() {
        return None;
    }
    let pdf = build_pdf_for_pages_v0(&layout.pages);
    if pdf.is_empty() {
        return None;
    }
    Some(pdf)
}
