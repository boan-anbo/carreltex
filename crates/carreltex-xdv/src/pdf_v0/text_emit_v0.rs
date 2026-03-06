fn glyphs_advance_pt_v0(glyphs: &[GlyphPlanV0]) -> f32 {
    glyphs
        .iter()
        .map(|glyph| (glyph.advance_sp as f32) / 65_536.0)
        .sum()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SegmentEmitProfileV0 {
    Default,
    BodyProseV13,
}

const BODY_PROSE_ITALIC_SCALE_PERCENT_V13: u8 = 97;
const BODY_PROSE_BOLD_SCALE_PERCENT_V13: u8 = 95;

fn body_prose_style_scale_percent_v13(segment: &PdfRenderSegmentV0) -> u8 {
    if segment.is_link || segment.superscript {
        return 100;
    }
    if !segment.bytes.iter().any(|byte| byte.is_ascii_alphabetic()) {
        return 100;
    }
    match segment.style {
        PdfTextStyleV0::Regular => 100,
        PdfTextStyleV0::Italic => BODY_PROSE_ITALIC_SCALE_PERCENT_V13,
        PdfTextStyleV0::Bold => BODY_PROSE_BOLD_SCALE_PERCENT_V13,
    }
}

fn style_scale_percent_for_profile_v0(
    segment: &PdfRenderSegmentV0,
    profile: SegmentEmitProfileV0,
) -> u8 {
    match profile {
        SegmentEmitProfileV0::Default => 100,
        SegmentEmitProfileV0::BodyProseV13 => body_prose_style_scale_percent_v13(segment),
    }
}

fn emit_styled_segments_v0(
    out: &mut Vec<u8>,
    segments: &[PdfStyledSegmentV0],
    x_pt: f32,
    y_pt: f32,
    font_size_pt: f32,
) {
    emit_styled_segments_with_profile_v0(
        out,
        segments,
        x_pt,
        y_pt,
        font_size_pt,
        SegmentEmitProfileV0::Default,
    );
}

fn emit_styled_segments_with_profile_v0(
    out: &mut Vec<u8>,
    segments: &[PdfStyledSegmentV0],
    x_pt: f32,
    y_pt: f32,
    font_size_pt: f32,
    profile: SegmentEmitProfileV0,
) {
    if segments.is_empty() {
        return;
    }
    let mut render_segments = Vec::<PdfRenderSegmentV0>::with_capacity(segments.len());
    for segment in segments {
        render_segments.push(PdfRenderSegmentV0 {
            style: segment.style,
            bytes: bytes_from_glyphs_v0(&segment.glyphs),
            advance_sp: segment.advance_sp,
            advance_pt: segment.advance_pt,
            is_link: segment.is_link,
            superscript: false,
        });
    }
    emit_render_segments_with_superscript_with_profile_v0(
        out,
        &render_segments,
        x_pt,
        y_pt,
        font_size_pt,
        profile,
    );
}

fn emit_render_segments_with_superscript_v0(
    out: &mut Vec<u8>,
    segments: &[PdfRenderSegmentV0],
    x_pt: f32,
    y_pt: f32,
    font_size_pt: f32,
) {
    emit_render_segments_with_superscript_with_profile_v0(
        out,
        segments,
        x_pt,
        y_pt,
        font_size_pt,
        SegmentEmitProfileV0::Default,
    );
}

fn emit_render_segments_with_superscript_with_profile_v0(
    out: &mut Vec<u8>,
    segments: &[PdfRenderSegmentV0],
    x_pt: f32,
    y_pt: f32,
    font_size_pt: f32,
    profile: SegmentEmitProfileV0,
) {
    if segments.is_empty() {
        return;
    }
    let mut cursor_x = f64::from(x_pt);
    let y_pt_f64 = f64::from(y_pt);
    for segment in segments {
        if segment.bytes.is_empty() {
            cursor_x += f64::from(segment.advance_sp) / 65_536.0;
            continue;
        }
        let style_scale_percent = style_scale_percent_for_profile_v0(segment, profile);
        if style_scale_percent != 100 {
            out.extend_from_slice(format!("{style_scale_percent} Tz ").as_bytes());
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
        if style_scale_percent != 100 {
            out.extend_from_slice(b"100 Tz ");
        }
        cursor_x += f64::from(segment.advance_sp) / 65_536.0;
    }
    out.extend_from_slice(b"0 Ts ");
}
