fn glyphs_advance_pt_v0(glyphs: &[GlyphPlanV0]) -> f32 {
    glyphs
        .iter()
        .map(|glyph| (glyph.advance_sp as f32) / 65_536.0)
        .sum()
}

fn emit_styled_segments_v0(
    out: &mut Vec<u8>,
    segments: &[PdfStyledSegmentV0],
    x_pt: f32,
    y_pt: f32,
    font_size_pt: f32,
) {
    if segments.is_empty() {
        return;
    }
    let mut render_segments = Vec::<PdfRenderSegmentV0>::with_capacity(segments.len());
    for segment in segments {
        render_segments.push(PdfRenderSegmentV0 {
            style: segment.style,
            bytes: bytes_from_glyphs_v0(&segment.glyphs),
            advance_pt: segment.advance_pt,
            is_link: segment.is_link,
            superscript: false,
        });
    }
    emit_render_segments_with_superscript_v0(out, &render_segments, x_pt, y_pt, font_size_pt);
}

fn emit_render_segments_with_superscript_v0(
    out: &mut Vec<u8>,
    segments: &[PdfRenderSegmentV0],
    x_pt: f32,
    y_pt: f32,
    font_size_pt: f32,
) {
    if segments.is_empty() {
        return;
    }
    let mut cursor_x = f64::from(x_pt);
    let y_pt_f64 = f64::from(y_pt);
    for segment in segments {
        if segment.bytes.is_empty() {
            cursor_x += f64::from(segment.advance_pt);
            continue;
        }
        out.extend_from_slice(b"1 0 0 1 ");
        out.extend_from_slice(format!("{cursor_x:.3} {y_pt_f64:.3} Tm ").as_bytes());
        let escaped = escape_pdf_string_bytes(&segment.bytes);
        let segment_font_size_pt = if segment.superscript {
            FOOTNOTE_MARKER_FONT_SIZE_PT_V0
        } else {
            font_size_pt
        };
        let text_rise_pt = if segment.superscript {
            FOOTNOTE_MARKER_RISE_PT_V0
        } else {
            0.0
        };
        out.extend_from_slice(format!("{text_rise_pt} Ts ").as_bytes());
        out.extend_from_slice(b"/");
        out.extend_from_slice(style_font_alias_v0(segment.style));
        out.extend_from_slice(b" ");
        out.extend_from_slice(format!("{segment_font_size_pt}").as_bytes());
        out.extend_from_slice(b" Tf (");
        out.extend_from_slice(&escaped);
        out.extend_from_slice(b") Tj ");
        cursor_x += f64::from(segment.advance_pt);
    }
    out.extend_from_slice(b"0 Ts ");
}
