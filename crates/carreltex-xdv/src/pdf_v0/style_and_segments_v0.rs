fn style_font_alias_v0(style: PdfTextStyleV0) -> &'static [u8] {
    match style {
        PdfTextStyleV0::Regular => b"F1",
        PdfTextStyleV0::Italic => b"F2",
        PdfTextStyleV0::Bold => b"F3",
    }
}

fn infer_line_advance_sp_v0(bytes: &[u8]) -> i32 {
    const DVI_DOWN3: u8 = 160;
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] == DVI_DOWN3 {
            if index + 4 <= bytes.len() {
                let raw = ((bytes[index + 1] as i32) << 16)
                    | ((bytes[index + 2] as i32) << 8)
                    | (bytes[index + 3] as i32);
                let value = if (raw & 0x80_0000) != 0 {
                    raw | !0x00ff_ffff
                } else {
                    raw
                };
                if value > 0 {
                    return value;
                }
            }
        }
        index += 1;
    }
    DEFAULT_LINE_ADVANCE_SP_V0
}

fn escape_pdf_string_bytes(text: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(text.len());
    for &byte in text {
        match byte {
            b'\\' | b'(' | b')' => {
                out.push(b'\\');
                out.push(byte);
            }
            0x20..=0x7e => out.push(byte),
            _ => out.push(b'?'),
        }
    }
    out
}

fn write_pdf_obj(out: &mut Vec<u8>, id: u32, body: &[u8]) -> u32 {
    let offset = out.len() as u32;
    out.extend_from_slice(id.to_string().as_bytes());
    out.extend_from_slice(b" 0 obj\n");
    out.extend_from_slice(body);
    out.extend_from_slice(b"\nendobj\n");
    offset
}

fn write_pdf_stream_obj(out: &mut Vec<u8>, id: u32, stream: &[u8]) -> u32 {
    let header = format!("<< /Length {} >>\nstream\n", stream.len());
    let mut body = Vec::with_capacity(header.len() + stream.len() + 11);
    body.extend_from_slice(header.as_bytes());
    body.extend_from_slice(stream);
    body.extend_from_slice(b"\nendstream");
    write_pdf_obj(out, id, &body)
}

#[derive(Clone)]
struct PdfStyledSegmentV0 {
    style: PdfTextStyleV0,
    glyphs: Vec<GlyphPlanV0>,
    advance_pt: f32,
    is_link: bool,
}

#[derive(Clone)]
struct PdfRenderSegmentV0 {
    style: PdfTextStyleV0,
    bytes: Vec<u8>,
    advance_pt: f32,
    is_link: bool,
    superscript: bool,
}

#[derive(Clone)]
struct PdfLinkAnnotationV0 {
    target: PdfLinkTargetV0,
    rect: [f32; 4],
}

#[derive(Clone)]
enum PdfLinkTargetV0 {
    Uri(Vec<u8>),
    Anchor(u32),
    AnchorPage(u32),
}

#[derive(Clone)]
struct PageRenderV0 {
    stream: Vec<u8>,
    annotations: Vec<PdfLinkAnnotationV0>,
}

#[derive(Clone)]
struct TocEntryMetadataV0 {
    level: u8,
    anchor_id: u32,
    title_glyphs: Vec<GlyphPlanV0>,
}

#[derive(Clone, Copy)]
struct EquationMetadataV0 {
    anchor_id: u32,
    ordinal: u32,
}

#[derive(Clone)]
struct RefAnchorLinkMetadataV0 {
    link_id: u32,
    anchor_id: u32,
}

#[derive(Clone)]
struct PagerefPageLinkMetadataV0 {
    link_id: u32,
    anchor_id: u32,
}

#[derive(Clone, Copy)]
struct AnchorDestinationV0 {
    page_index: usize,
    y_pt: f32,
}

#[derive(Clone)]
struct OutlineItemPlanV0 {
    anchor_id: u32,
    title_bytes: Vec<u8>,
    parent_index: Option<usize>,
    first_child_index: Option<usize>,
    last_child_index: Option<usize>,
    prev_sibling_index: Option<usize>,
    next_sibling_index: Option<usize>,
    child_count: u32,
}

fn parse_styled_segments_v0(glyphs: &[GlyphPlanV0]) -> Option<Vec<PdfStyledSegmentV0>> {
    let mut style_stack = Vec::<PdfTextStyleV0>::new();
    let mut current_style = PdfTextStyleV0::Regular;
    let mut link_active = false;
    let segments = parse_styled_segments_with_state_v0(
        glyphs,
        &mut style_stack,
        &mut current_style,
        &mut link_active,
    )?;
    if !style_stack.is_empty() || link_active {
        return None;
    }
    Some(segments)
}

fn parse_styled_segments_with_state_v0(
    glyphs: &[GlyphPlanV0],
    style_stack: &mut Vec<PdfTextStyleV0>,
    current_style: &mut PdfTextStyleV0,
    link_active: &mut bool,
) -> Option<Vec<PdfStyledSegmentV0>> {
    let mut segments = Vec::<PdfStyledSegmentV0>::new();

    for glyph in glyphs {
        let byte = glyph.byte;
        match byte {
            ITALIC_START_MARKER_V0 => {
                style_stack.push(*current_style);
                *current_style = PdfTextStyleV0::Italic;
            }
            ITALIC_END_MARKER_V0 => {
                if *current_style != PdfTextStyleV0::Italic {
                    return None;
                }
                *current_style = style_stack.pop()?;
            }
            BOLD_START_MARKER_V0 => {
                style_stack.push(*current_style);
                *current_style = PdfTextStyleV0::Bold;
            }
            BOLD_END_MARKER_V0 => {
                if *current_style != PdfTextStyleV0::Bold {
                    return None;
                }
                *current_style = style_stack.pop()?;
            }
            LINK_START_MARKER_V0 => {
                if *link_active {
                    return None;
                }
                *link_active = true;
            }
            LINK_END_MARKER_V0 => {
                if !*link_active {
                    return None;
                }
                *link_active = false;
            }
            _ => {
                let advance_pt = (glyph.advance_sp as f32) / 65_536.0;
                if let Some(segment) = segments.last_mut() {
                    if segment.style == *current_style && segment.is_link == *link_active {
                        segment.glyphs.push(glyph.clone());
                        segment.advance_pt += advance_pt;
                        continue;
                    }
                }
                segments.push(PdfStyledSegmentV0 {
                    style: *current_style,
                    glyphs: vec![glyph.clone()],
                    advance_pt,
                    is_link: *link_active,
                });
            }
        }
    }
    Some(segments)
}

fn bytes_from_glyphs_v0(glyphs: &[GlyphPlanV0]) -> Vec<u8> {
    glyphs.iter().map(|glyph| glyph.byte).collect()
}

fn is_footnote_marker_glyph_at_v0(glyphs: &[GlyphPlanV0], index: usize) -> bool {
    if glyphs[index].byte != FOOTNOTE_MARKER_PREFIX_V0 || index + 1 >= glyphs.len() {
        return false;
    }
    glyphs[index + 1].byte.is_ascii_digit()
}

fn split_superscript_segments_v0(segments: &[PdfStyledSegmentV0]) -> Vec<PdfRenderSegmentV0> {
    let mut out = Vec::<PdfRenderSegmentV0>::new();
    for segment in segments {
        let mut normal_start = 0usize;
        let mut cursor = 0usize;
        while cursor < segment.glyphs.len() {
            if !is_footnote_marker_glyph_at_v0(&segment.glyphs, cursor) {
                cursor += 1;
                continue;
            }
            let mut marker_end = cursor + 2;
            while marker_end < segment.glyphs.len()
                && segment.glyphs[marker_end].byte.is_ascii_digit()
            {
                marker_end += 1;
            }
            if cursor > normal_start {
                let glyph_slice = &segment.glyphs[normal_start..cursor];
                out.push(PdfRenderSegmentV0 {
                    style: segment.style,
                    bytes: bytes_from_glyphs_v0(glyph_slice),
                    advance_pt: glyph_slice
                        .iter()
                        .map(|glyph| (glyph.advance_sp as f32) / 65_536.0)
                        .sum(),
                    is_link: segment.is_link,
                    superscript: false,
                });
            }
            let marker_slice = &segment.glyphs[cursor..marker_end];
            out.push(PdfRenderSegmentV0 {
                style: segment.style,
                bytes: bytes_from_glyphs_v0(marker_slice),
                advance_pt: marker_slice
                    .iter()
                    .map(|glyph| (glyph.advance_sp as f32) / 65_536.0)
                    .sum(),
                is_link: segment.is_link,
                superscript: true,
            });
            normal_start = marker_end;
            cursor = marker_end;
        }
        if normal_start < segment.glyphs.len() {
            let glyph_slice = &segment.glyphs[normal_start..];
            out.push(PdfRenderSegmentV0 {
                style: segment.style,
                bytes: bytes_from_glyphs_v0(glyph_slice),
                advance_pt: glyph_slice
                    .iter()
                    .map(|glyph| (glyph.advance_sp as f32) / 65_536.0)
                    .sum(),
                is_link: segment.is_link,
                superscript: false,
            });
        }
    }
    out
}

fn is_structured_non_title_line_v0(glyphs: &[GlyphPlanV0]) -> bool {
    detect_heading_prefix_v0(glyphs).is_some()
        || detect_list_prefix_v0(glyphs).is_some()
        || detect_quote_prefix_advance_pt_v0(glyphs).is_some()
        || has_center_prefix_v0(glyphs)
        || has_right_prefix_v0(glyphs)
        || has_noindent_prefix_v0(glyphs)
        || has_table_spec_prefix_v0(glyphs)
        || has_table_row_prefix_v0(glyphs)
        || has_figure_box_prefix_v0(glyphs)
        || has_figure_caption_prefix_v0(glyphs)
        || has_figure_image_prefix_v0(glyphs)
        || has_toc_placeholder_line_v0(glyphs)
        || has_toc_entry_line_prefix_v0(glyphs)
        || has_footnote_line_prefix_v0(glyphs)
        || has_href_url_line_prefix_v0(glyphs)
        || has_label_line_prefix_v0(glyphs)
        || has_ref_line_prefix_v0(glyphs)
        || has_pageref_line_prefix_v0(glyphs)
        || has_ref_anchor_link_line_prefix_v0(glyphs)
        || has_pageref_page_link_line_prefix_v0(glyphs)
        || has_equation_line_prefix_v0(glyphs)
        || has_bibitem_line_prefix_v0(glyphs)
        || has_cite_line_prefix_v0(glyphs)
}

fn detect_title_block_len_v0(lines: &[LinePlanV0]) -> usize {
    let mut index = 0usize;
    while index < lines.len()
        && !lines[index].glyphs.is_empty()
        && !is_structured_non_title_line_v0(&lines[index].glyphs)
    {
        index += 1;
    }
    if index > 0 && index < lines.len() {
        index
    } else {
        0
    }
}

fn centered_line_x_v0(line_width_pt: f32) -> f32 {
    let width_pt = line_width_pt.max(0.0);
    let centered = (PAGE_WIDTH_PT_V0 - width_pt) * 0.5;
    centered.clamp(MARGIN_PT_V0, PAGE_WIDTH_PT_V0 - MARGIN_PT_V0)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ListPrefixKindV0 {
    Itemize,
    Enumerate,
}

#[derive(Clone, Copy)]
struct ListPrefixV0 {
    kind: ListPrefixKindV0,
    prefix_len: usize,
    display_start: usize,
    display_len: usize,
    leading_advance_pt: f32,
}

