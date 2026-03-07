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
    BodyWrappedProseV27,
    WrappedAlignedV28,
    WrappedIndentedV29,
    FootnoteProseV26,
    BodyProseInlineMathV15,
}

const BODY_PROSE_ITALIC_SCALE_PERCENT_V13: u8 = 97;
const BODY_PROSE_BOLD_SCALE_PERCENT_V13: u8 = 95;
const BODY_PROSE_INLINE_MATH_ITALIC_SCALE_PERCENT_V15: u8 = 99;
const BODY_PROSE_INLINE_MATH_BOLD_SCALE_PERCENT_V15: u8 = 97;

fn footnote_prose_style_scale_percent_v26(segment: &PdfRenderSegmentV0) -> u8 {
    if segment.superscript {
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

fn wrapped_body_prose_style_scale_percent_v27(segment: &PdfRenderSegmentV0) -> u8 {
    if segment.superscript {
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

fn wrapped_aligned_style_scale_percent_v28(segment: &PdfRenderSegmentV0) -> u8 {
    if segment.superscript {
        return 100;
    }
    if !segment.bytes.iter().any(|byte| byte.is_ascii_alphabetic()) {
        return 100;
    }
    match segment.style {
        PdfTextStyleV0::Regular => 100,
        PdfTextStyleV0::Italic => 85,
        PdfTextStyleV0::Bold => 83,
    }
}

fn wrapped_indented_style_scale_percent_v29(segment: &PdfRenderSegmentV0) -> u8 {
    if segment.superscript || segment.is_link {
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

fn body_prose_style_scale_percent_v13(segment: &PdfRenderSegmentV0) -> u8 {
    if segment.superscript || segment.is_link {
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
        SegmentEmitProfileV0::BodyWrappedProseV27 => {
            wrapped_body_prose_style_scale_percent_v27(segment)
        }
        SegmentEmitProfileV0::WrappedAlignedV28 => {
            wrapped_aligned_style_scale_percent_v28(segment)
        }
        SegmentEmitProfileV0::WrappedIndentedV29 => {
            wrapped_indented_style_scale_percent_v29(segment)
        }
        SegmentEmitProfileV0::FootnoteProseV26 => footnote_prose_style_scale_percent_v26(segment),
        SegmentEmitProfileV0::BodyProseInlineMathV15 => {
            if segment.superscript || segment.is_link {
                return 100;
            }
            if !segment.bytes.iter().any(|byte| byte.is_ascii_alphabetic()) {
                return 100;
            }
            match segment.style {
                PdfTextStyleV0::Regular => 100,
                PdfTextStyleV0::Italic => BODY_PROSE_INLINE_MATH_ITALIC_SCALE_PERCENT_V15,
                PdfTextStyleV0::Bold => BODY_PROSE_INLINE_MATH_BOLD_SCALE_PERCENT_V15,
            }
        }
    }
}

fn render_advance_pt_for_segment_with_profile_v0(
    segment: &PdfRenderSegmentV0,
    profile: SegmentEmitProfileV0,
) -> f32 {
    if !matches!(
        profile,
        SegmentEmitProfileV0::FootnoteProseV26
            | SegmentEmitProfileV0::BodyWrappedProseV27
            | SegmentEmitProfileV0::WrappedAlignedV28
            | SegmentEmitProfileV0::WrappedIndentedV29
    ) {
        return segment.advance_pt;
    }
    let style_scale_percent = style_scale_percent_for_profile_v0(segment, profile);
    if style_scale_percent == 100 {
        segment.advance_pt
    } else {
        segment.advance_pt * (style_scale_percent as f32 / 100.0)
    }
}

fn trailing_space_bounded_seam_trim_pt_v30(
    segment: &PdfRenderSegmentV0,
    next_segment: Option<&PdfRenderSegmentV0>,
    profile: SegmentEmitProfileV0,
    font_size_pt: f32,
) -> f32 {
    if !matches!(
        profile,
        SegmentEmitProfileV0::BodyProseV13
            | SegmentEmitProfileV0::BodyWrappedProseV27
            | SegmentEmitProfileV0::WrappedAlignedV28
            | SegmentEmitProfileV0::WrappedIndentedV29
            | SegmentEmitProfileV0::FootnoteProseV26
    ) {
        return 0.0;
    }
    if segment.superscript || segment.is_link || matches!(segment.style, PdfTextStyleV0::Regular) {
        if !matches!(
            profile,
            SegmentEmitProfileV0::BodyProseV13
                | SegmentEmitProfileV0::BodyWrappedProseV27
                | SegmentEmitProfileV0::WrappedAlignedV28
                | SegmentEmitProfileV0::WrappedIndentedV29
                | SegmentEmitProfileV0::FootnoteProseV26
        ) || segment.superscript
            || segment.is_link
            || !matches!(segment.style, PdfTextStyleV0::Regular)
        {
            return 0.0;
        }
        let Some(next_segment) = next_segment else {
            return 0.0;
        };
        let require_prose_prefix_floor = matches!(
            profile,
            SegmentEmitProfileV0::BodyProseV13 | SegmentEmitProfileV0::BodyWrappedProseV27
        );
        if next_segment.superscript
            || next_segment.is_link
            || (require_prose_prefix_floor
                && (segment.bytes.len() < 8
                    || !segment.bytes.iter().any(|byte| byte.is_ascii_alphabetic())))
            || !segment.bytes.last().is_some_and(|byte| *byte == b' ')
        {
            return 0.0;
        }
        let requested_trim_pt = match next_segment.style {
            PdfTextStyleV0::Italic => font_size_pt * 0.12,
            PdfTextStyleV0::Bold => font_size_pt * 0.15,
            PdfTextStyleV0::Regular => 0.0,
        };
        let long_indented_prefix_bias_pt = if matches!(profile, SegmentEmitProfileV0::FootnoteProseV26)
            && segment.advance_pt >= 150.0
        {
            (segment.advance_pt * 0.10).min(font_size_pt * 2.5)
        } else if matches!(profile, SegmentEmitProfileV0::FootnoteProseV26)
            && segment.advance_pt >= 95.0
        {
            (segment.advance_pt * 0.068).min(font_size_pt * 1.68)
        } else if matches!(profile, SegmentEmitProfileV0::WrappedAlignedV28)
            && matches!(next_segment.style, PdfTextStyleV0::Bold)
            && segment.advance_pt >= 95.0
        {
            (segment.advance_pt * 0.06).min(font_size_pt * 1.4)
        } else if matches!(profile, SegmentEmitProfileV0::WrappedAlignedV28)
            && matches!(next_segment.style, PdfTextStyleV0::Bold)
            && segment.advance_pt >= 80.0
        {
            (segment.advance_pt * 0.05).min(font_size_pt * 1.1)
        } else if matches!(profile, SegmentEmitProfileV0::WrappedAlignedV28)
            && matches!(next_segment.style, PdfTextStyleV0::Bold)
            && segment.advance_pt >= 70.0
        {
            (segment.advance_pt * 0.045).min(font_size_pt * 1.0)
        } else if matches!(profile, SegmentEmitProfileV0::WrappedAlignedV28)
            && matches!(next_segment.style, PdfTextStyleV0::Bold)
            && segment.advance_pt >= 55.0
        {
            (segment.advance_pt * 0.035).min(font_size_pt * 0.85)
        } else if matches!(profile, SegmentEmitProfileV0::WrappedAlignedV28)
            && matches!(next_segment.style, PdfTextStyleV0::Bold)
            && segment.advance_pt >= 40.0
        {
            (segment.advance_pt * 0.03).min(font_size_pt * 0.68)
        } else if matches!(profile, SegmentEmitProfileV0::WrappedAlignedV28)
            && matches!(next_segment.style, PdfTextStyleV0::Bold)
            && segment.advance_pt >= 28.0
        {
            (segment.advance_pt * 0.023).min(font_size_pt * 0.52)
        } else if matches!(profile, SegmentEmitProfileV0::WrappedAlignedV28)
            && matches!(next_segment.style, PdfTextStyleV0::Italic)
            && segment.advance_pt >= 70.0
        {
            (segment.advance_pt * 0.04).min(font_size_pt * 0.95)
        } else if matches!(profile, SegmentEmitProfileV0::WrappedAlignedV28)
            && segment.advance_pt >= 70.0
        {
            (segment.advance_pt * 0.035).min(font_size_pt * 0.9)
        } else if matches!(profile, SegmentEmitProfileV0::WrappedAlignedV28)
            && matches!(next_segment.style, PdfTextStyleV0::Italic)
            && segment.advance_pt >= 55.0
        {
            (segment.advance_pt * 0.03).min(font_size_pt * 0.75)
        } else if matches!(profile, SegmentEmitProfileV0::WrappedAlignedV28)
            && matches!(next_segment.style, PdfTextStyleV0::Italic)
            && segment.advance_pt >= 40.0
        {
            (segment.advance_pt * 0.034).min(font_size_pt * 0.78)
        } else if matches!(profile, SegmentEmitProfileV0::WrappedAlignedV28)
            && matches!(next_segment.style, PdfTextStyleV0::Italic)
            && segment.advance_pt >= 28.0
        {
            (segment.advance_pt * 0.021).min(font_size_pt * 0.48)
        } else if matches!(profile, SegmentEmitProfileV0::WrappedAlignedV28)
            && matches!(next_segment.style, PdfTextStyleV0::Italic)
            && segment.advance_pt >= 14.0
        {
            font_size_pt * 0.14
        } else if matches!(profile, SegmentEmitProfileV0::WrappedAlignedV28)
            && matches!(next_segment.style, PdfTextStyleV0::Italic)
            && segment.advance_pt >= 12.0
        {
            font_size_pt * 0.13
        } else if matches!(profile, SegmentEmitProfileV0::WrappedAlignedV28)
            && matches!(next_segment.style, PdfTextStyleV0::Regular)
            && segment.advance_pt >= 70.0
        {
            (segment.advance_pt * 0.041).min(font_size_pt * 0.96)
        } else if matches!(profile, SegmentEmitProfileV0::WrappedAlignedV28)
            && matches!(next_segment.style, PdfTextStyleV0::Regular)
            && segment.advance_pt >= 55.0
        {
            (segment.advance_pt * 0.038).min(font_size_pt * 0.88)
        } else if matches!(profile, SegmentEmitProfileV0::WrappedAlignedV28)
            && matches!(next_segment.style, PdfTextStyleV0::Regular)
            && segment.advance_pt >= 40.0
        {
            (segment.advance_pt * 0.039).min(font_size_pt * 0.84)
        } else if matches!(profile, SegmentEmitProfileV0::WrappedAlignedV28)
            && matches!(next_segment.style, PdfTextStyleV0::Regular)
            && segment.advance_pt >= 28.0
        {
            (segment.advance_pt * 0.024).min(font_size_pt * 0.56)
        } else if matches!(profile, SegmentEmitProfileV0::WrappedAlignedV28)
            && segment.advance_pt >= 95.0
        {
            (segment.advance_pt * 0.052).min(font_size_pt * 1.25)
        } else if matches!(profile, SegmentEmitProfileV0::WrappedIndentedV29)
            && segment.advance_pt >= 250.0
        {
            (segment.advance_pt * 0.16).min(font_size_pt * 3.8)
        } else if matches!(profile, SegmentEmitProfileV0::WrappedIndentedV29)
            && segment.advance_pt >= 120.0
        {
            (segment.advance_pt * 0.13).min(font_size_pt * 3.0)
        } else if matches!(profile, SegmentEmitProfileV0::WrappedIndentedV29)
            && segment.advance_pt >= 80.0
        {
            (segment.advance_pt * 0.094).min(font_size_pt * 2.16)
        } else if matches!(profile, SegmentEmitProfileV0::WrappedIndentedV29)
            && segment.advance_pt >= 60.0
        {
            (segment.advance_pt * 0.078).min(font_size_pt * 1.68)
        } else {
            0.0
        };
        return (requested_trim_pt + long_indented_prefix_bias_pt).min(segment.advance_pt * 0.4);
    }
    if matches!(
        profile,
        SegmentEmitProfileV0::BodyProseV13 | SegmentEmitProfileV0::WrappedAlignedV28
    ) {
        return 0.0;
    }
    let Some(next_segment) = next_segment else {
        return 0.0;
    };
    if next_segment.bytes.first().copied() != Some(b' ') {
        return 0.0;
    }
    let requested_trim_pt = match segment.style {
        PdfTextStyleV0::Italic => font_size_pt * 0.12,
        PdfTextStyleV0::Bold => font_size_pt * 0.15,
        PdfTextStyleV0::Regular => 0.0,
    };
    requested_trim_pt.min(segment.advance_pt * 0.25)
}

fn contextual_render_advance_pt_for_segment_v30(
    segments: &[PdfRenderSegmentV0],
    index: usize,
    profile: SegmentEmitProfileV0,
    font_size_pt: f32,
) -> f32 {
    let Some(segment) = segments.get(index) else {
        return 0.0;
    };
    let base_advance_pt = render_advance_pt_for_segment_with_profile_v0(segment, profile);
    let seam_trim_pt =
        trailing_space_bounded_seam_trim_pt_v30(segment, segments.get(index + 1), profile, font_size_pt);
    (base_advance_pt - seam_trim_pt).max(0.0)
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
    for (index, segment) in segments.iter().enumerate() {
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
        cursor_x += f64::from(contextual_render_advance_pt_for_segment_v30(
            segments,
            index,
            profile,
            segment_font_size_pt,
        ));
    }
    out.extend_from_slice(b"0 Ts ");
}
