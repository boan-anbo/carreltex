use crate::{parse_dvi_v2_text_page_to_layout_v0, GlyphPlanV0, LinePlanV0, PagePlanV0, DEFAULT_LINE_ADVANCE_SP_V0};
use std::collections::BTreeMap;

const PDF_VERSION: &[u8] = b"%PDF-1.4\n";
const PDF_EOF: &[u8] = b"%%EOF\n";
const NEWLINE_MARKER_V0: u8 = 0x0a;
const PAGE_BREAK_MARKER_V0: u8 = 0x0c;

const PAGE_WIDTH_PT_V0: f32 = 612.0;
const PAGE_HEIGHT_PT_V0: f32 = 792.0;
const MARGIN_PT_V0: f32 = 72.0;
const FONT_SIZE_PT_V0: f32 = 12.0;
const TITLE_FONT_SIZE_PT_V0: f32 = 18.0;
const SECTION_HEADING_FONT_SIZE_PT_V0: f32 = 16.0;
const SUBSECTION_HEADING_FONT_SIZE_PT_V0: f32 = 14.0;
const INDENT_PT_V0: f32 = FONT_SIZE_PT_V0 * 2.0;
const LIST_BODY_INDENT_PT_V0: f32 = INDENT_PT_V0;
const ENUM_NUMBER_COLUMN_RIGHT_PT_V0: f32 = MARGIN_PT_V0 + (FONT_SIZE_PT_V0 * 1.5);
const LEADING_PT_V0: f32 = 14.0;
const TITLE_EXTRA_GAP_PT_V0: f32 = LEADING_PT_V0;
const FOOTNOTE_FONT_SIZE_PT_V0: f32 = 10.0;
const FOOTNOTE_LEADING_PT_V0: f32 = 12.0;
const FOOTNOTE_BLOCK_GAP_PT_V0: f32 = 12.0;
const FOOTNOTE_LINE_PREFIX_MARKER_V0: &[u8] = b"!f ";
const HREF_URL_LINE_PREFIX_MARKER_V0: &[u8] = b"!u ";
const LABEL_LINE_PREFIX_MARKER_V0: &[u8] = b"!l ";
const REF_LINE_PREFIX_MARKER_V0: &[u8] = b"!r ";
const NOINDENT_PREFIX_MARKER_V0: u8 = b'~';
const LINK_START_MARKER_V0: u8 = b'<';
const LINK_END_MARKER_V0: u8 = b'>';
const FOOTNOTE_MARKER_PREFIX_V0: u8 = b'^';
const FOOTNOTE_MARKER_FONT_SIZE_PT_V0: f32 = 8.0;
const FOOTNOTE_MARKER_RISE_PT_V0: f32 = 4.0;
const TABLE_ROW_PREFIX_MARKER_V0: &[u8] = b"!t ";
const FIGURE_BOX_PREFIX_MARKER_V0: &[u8] = b"!gbox";
const FIGURE_CAPTION_PREFIX_MARKER_V0: &[u8] = b"!gcap ";
const TOC_PLACEHOLDER_MARKER_V0: &[u8] = b"!toc";
const TOC_ENTRY_LINE_PREFIX_MARKER_V0: &[u8] = b"!toc ";
const TABLE_COLUMN_COUNT_V0: usize = 3;
const TABLE_CELL_PADDING_PT_V0: f32 = 6.0;
const FIGURE_PLACEHOLDER_LINE_V0: &[u8] = b"[ Figure placeholder ]";
const FIGURE_CAPTION_FONT_SIZE_PT_V0: f32 = 11.0;
const TOC_TITLE_TEXT_V0: &[u8] = b"Contents";
const TOC_TITLE_FONT_SIZE_PT_V0: f32 = 14.0;
const TOC_ENTRY_INDENT_STEP_PT_V0: f32 = 18.0;
const SECTION_HEADING_PREFIX_MARKER_V0: &[u8] = b"@S ";
const SUBSECTION_HEADING_PREFIX_MARKER_V0: &[u8] = b"@s ";
const ITALIC_START_MARKER_V0: u8 = b'[';
const ITALIC_END_MARKER_V0: u8 = b']';
const BOLD_START_MARKER_V0: u8 = b'{';
const BOLD_END_MARKER_V0: u8 = b'}';

#[derive(Clone, Copy, PartialEq, Eq)]
enum PdfTextStyleV0 {
    Regular,
    Italic,
    Bold,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum HeadingKindV0 {
    Section,
    Subsection,
}

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
    uri: Vec<u8>,
    rect: [f32; 4],
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
            while marker_end < segment.glyphs.len() && segment.glyphs[marker_end].byte.is_ascii_digit() {
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

fn detect_title_block_len_v0(lines: &[LinePlanV0]) -> usize {
    let mut index = 0usize;
    while index < lines.len() && !lines[index].glyphs.is_empty() {
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

fn detect_list_prefix_v0(glyphs: &[GlyphPlanV0]) -> Option<ListPrefixV0> {
    let mut leading = 0usize;
    while leading < glyphs.len() && glyphs[leading].byte == b' ' {
        leading += 1;
    }
    let leading_advance_pt: f32 = glyphs[..leading]
        .iter()
        .map(|glyph| (glyph.advance_sp as f32) / 65_536.0)
        .sum();

    if glyphs.len() >= leading + 2 && glyphs[leading].byte == b'-' && glyphs[leading + 1].byte == b' ' {
        return Some(ListPrefixV0 {
            kind: ListPrefixKindV0::Itemize,
            prefix_len: leading + 2,
            display_start: leading,
            display_len: 1,
            leading_advance_pt,
        });
    }

    let mut index = leading;
    while index < glyphs.len() && glyphs[index].byte.is_ascii_digit() {
        index += 1;
    }
    if index == leading || index + 1 >= glyphs.len() {
        return None;
    }
    if glyphs[index].byte != b'.' || glyphs[index + 1].byte != b' ' {
        return None;
    }
    Some(ListPrefixV0 {
        kind: ListPrefixKindV0::Enumerate,
        prefix_len: index + 2,
        display_start: leading,
        display_len: index - leading + 1,
        leading_advance_pt,
    })
}

fn detect_quote_prefix_advance_pt_v0(glyphs: &[GlyphPlanV0]) -> Option<f32> {
    if glyphs.len() < 2 || glyphs[0].byte != b'>' || glyphs[1].byte != b' ' {
        return None;
    }
    let prefix_sp = glyphs[0].advance_sp.checked_add(glyphs[1].advance_sp)?;
    Some((prefix_sp as f32) / 65_536.0)
}

fn has_center_prefix_v0(glyphs: &[GlyphPlanV0]) -> bool {
    glyphs.len() >= 2 && glyphs[0].byte == b'^' && glyphs[1].byte == b' '
}

fn has_right_prefix_v0(glyphs: &[GlyphPlanV0]) -> bool {
    glyphs.len() >= 2 && glyphs[0].byte == b'|' && glyphs[1].byte == b' '
}

fn has_noindent_prefix_v0(glyphs: &[GlyphPlanV0]) -> bool {
    glyphs.len() >= 2 && glyphs[0].byte == NOINDENT_PREFIX_MARKER_V0 && glyphs[1].byte == b' '
}

fn has_footnote_line_prefix_v0(glyphs: &[GlyphPlanV0]) -> bool {
    glyphs.len() >= FOOTNOTE_LINE_PREFIX_MARKER_V0.len()
        && glyphs[..FOOTNOTE_LINE_PREFIX_MARKER_V0.len()]
            .iter()
            .map(|glyph| glyph.byte)
            .eq(FOOTNOTE_LINE_PREFIX_MARKER_V0.iter().copied())
}

fn heading_prefix_len_v0(kind: HeadingKindV0) -> usize {
    match kind {
        HeadingKindV0::Section => SECTION_HEADING_PREFIX_MARKER_V0.len(),
        HeadingKindV0::Subsection => SUBSECTION_HEADING_PREFIX_MARKER_V0.len(),
    }
}

fn detect_heading_prefix_v0(glyphs: &[GlyphPlanV0]) -> Option<HeadingKindV0> {
    if glyphs.len() >= SECTION_HEADING_PREFIX_MARKER_V0.len()
        && glyphs[..SECTION_HEADING_PREFIX_MARKER_V0.len()]
            .iter()
            .map(|glyph| glyph.byte)
            .eq(SECTION_HEADING_PREFIX_MARKER_V0.iter().copied())
    {
        return Some(HeadingKindV0::Section);
    }
    if glyphs.len() >= SUBSECTION_HEADING_PREFIX_MARKER_V0.len()
        && glyphs[..SUBSECTION_HEADING_PREFIX_MARKER_V0.len()]
            .iter()
            .map(|glyph| glyph.byte)
            .eq(SUBSECTION_HEADING_PREFIX_MARKER_V0.iter().copied())
    {
        return Some(HeadingKindV0::Subsection);
    }
    None
}

fn has_table_row_prefix_v0(glyphs: &[GlyphPlanV0]) -> bool {
    glyphs.len() >= TABLE_ROW_PREFIX_MARKER_V0.len()
        && glyphs[..TABLE_ROW_PREFIX_MARKER_V0.len()]
            .iter()
            .map(|glyph| glyph.byte)
            .eq(TABLE_ROW_PREFIX_MARKER_V0.iter().copied())
}

fn has_figure_box_prefix_v0(glyphs: &[GlyphPlanV0]) -> bool {
    glyphs.len() == FIGURE_BOX_PREFIX_MARKER_V0.len()
        && glyphs
            .iter()
            .map(|glyph| glyph.byte)
            .eq(FIGURE_BOX_PREFIX_MARKER_V0.iter().copied())
}

fn has_figure_caption_prefix_v0(glyphs: &[GlyphPlanV0]) -> bool {
    glyphs.len() >= FIGURE_CAPTION_PREFIX_MARKER_V0.len()
        && glyphs[..FIGURE_CAPTION_PREFIX_MARKER_V0.len()]
            .iter()
            .map(|glyph| glyph.byte)
            .eq(FIGURE_CAPTION_PREFIX_MARKER_V0.iter().copied())
}

fn has_toc_placeholder_line_v0(glyphs: &[GlyphPlanV0]) -> bool {
    glyphs.len() == TOC_PLACEHOLDER_MARKER_V0.len()
        && glyphs
            .iter()
            .map(|glyph| glyph.byte)
            .eq(TOC_PLACEHOLDER_MARKER_V0.iter().copied())
}

fn has_toc_entry_line_prefix_v0(glyphs: &[GlyphPlanV0]) -> bool {
    glyphs.len() >= TOC_ENTRY_LINE_PREFIX_MARKER_V0.len()
        && glyphs[..TOC_ENTRY_LINE_PREFIX_MARKER_V0.len()]
            .iter()
            .map(|glyph| glyph.byte)
            .eq(TOC_ENTRY_LINE_PREFIX_MARKER_V0.iter().copied())
}

fn trim_space_glyph_edges_v0(glyphs: &[GlyphPlanV0]) -> Vec<GlyphPlanV0> {
    let mut start = 0usize;
    let mut end = glyphs.len();
    while start < end && glyphs[start].byte == b' ' {
        start += 1;
    }
    while start < end && glyphs[end - 1].byte == b' ' {
        end -= 1;
    }
    glyphs[start..end].to_vec()
}

fn parse_table_row_cells_v0(glyphs: &[GlyphPlanV0]) -> Option<Vec<Vec<GlyphPlanV0>>> {
    if !has_table_row_prefix_v0(glyphs) {
        return None;
    }
    let mut cells = Vec::<Vec<GlyphPlanV0>>::new();
    let mut current = Vec::<GlyphPlanV0>::new();
    let mut index = TABLE_ROW_PREFIX_MARKER_V0.len();
    while index < glyphs.len() {
        if index + 1 < glyphs.len() && glyphs[index].byte == b'|' && glyphs[index + 1].byte == b'|' {
            cells.push(trim_space_glyph_edges_v0(&current));
            current.clear();
            index += 2;
            continue;
        }
        current.push(glyphs[index].clone());
        index += 1;
    }
    cells.push(trim_space_glyph_edges_v0(&current));
    if cells.len() != TABLE_COLUMN_COUNT_V0 {
        return None;
    }
    Some(cells)
}

fn parse_figure_caption_line_v0(glyphs: &[GlyphPlanV0]) -> Option<Vec<GlyphPlanV0>> {
    if !has_figure_caption_prefix_v0(glyphs) {
        return None;
    }
    let caption = trim_space_glyph_edges_v0(&glyphs[FIGURE_CAPTION_PREFIX_MARKER_V0.len()..]);
    if caption.is_empty() {
        return None;
    }
    Some(caption)
}

fn placeholder_segments_v0() -> Vec<PdfStyledSegmentV0> {
    let glyphs: Vec<GlyphPlanV0> = FIGURE_PLACEHOLDER_LINE_V0
        .iter()
        .copied()
        .map(|byte| GlyphPlanV0 {
            byte,
            advance_sp: 65_536,
        })
        .collect();
    vec![PdfStyledSegmentV0 {
        style: PdfTextStyleV0::Regular,
        glyphs: glyphs.clone(),
        advance_pt: glyphs
            .iter()
            .map(|glyph| (glyph.advance_sp as f32) / 65_536.0)
            .sum(),
        is_link: false,
    }]
}

fn emit_table_block_v0(
    out: &mut Vec<u8>,
    rows: &[LinePlanV0],
    y: &mut f32,
    min_body_y_pt: f32,
) -> Option<()> {
    if rows.is_empty() {
        return None;
    }
    let mut parsed_rows = Vec::<Vec<Vec<PdfRenderSegmentV0>>>::new();
    let mut col_max_width_pt = [0.0f32; TABLE_COLUMN_COUNT_V0];

    for row in rows {
        let cells = parse_table_row_cells_v0(&row.glyphs)?;
        let mut parsed_cells = Vec::<Vec<PdfRenderSegmentV0>>::new();
        for (col_index, cell_glyphs) in cells.iter().enumerate() {
            let segments = parse_styled_segments_v0(cell_glyphs)?;
            let render_segments = split_superscript_segments_v0(&segments);
            if render_segments.iter().any(|segment| segment.superscript || segment.is_link) {
                return None;
            }
            let width_pt = render_segments.iter().map(|segment| segment.advance_pt).sum::<f32>();
            col_max_width_pt[col_index] = col_max_width_pt[col_index].max(width_pt);
            parsed_cells.push(render_segments);
        }
        parsed_rows.push(parsed_cells);
    }

    let table_width_pt = col_max_width_pt
        .iter()
        .copied()
        .fold(0.0f32, |acc, width_pt| acc + width_pt + (TABLE_CELL_PADDING_PT_V0 * 2.0));
    let max_content_width_pt = PAGE_WIDTH_PT_V0 - (2.0 * MARGIN_PT_V0);
    if table_width_pt > max_content_width_pt {
        return None;
    }

    for row in &parsed_rows {
        if *y < min_body_y_pt {
            return None;
        }
        let mut col_left_x = MARGIN_PT_V0;
        for col_index in 0..TABLE_COLUMN_COUNT_V0 {
            let cell_width_pt = row[col_index]
                .iter()
                .map(|segment| segment.advance_pt)
                .sum::<f32>();
            let col_content_width_pt = col_max_width_pt[col_index];
            let align_offset_pt = match col_index {
                0 => 0.0,
                1 => (col_content_width_pt - cell_width_pt) * 0.5,
                _ => col_content_width_pt - cell_width_pt,
            };
            let x_pt = col_left_x + TABLE_CELL_PADDING_PT_V0 + align_offset_pt.max(0.0);
            emit_render_segments_with_superscript_v0(
                out,
                &row[col_index],
                x_pt,
                *y,
                FONT_SIZE_PT_V0,
            );
            col_left_x += col_content_width_pt + (TABLE_CELL_PADDING_PT_V0 * 2.0);
        }
        out.extend_from_slice(b"\n");
        *y -= LEADING_PT_V0;
    }
    Some(())
}

fn emit_figure_block_v0(
    out: &mut Vec<u8>,
    caption_glyphs: &[GlyphPlanV0],
    y: &mut f32,
    min_body_y_pt: f32,
) -> Option<()> {
    if *y < min_body_y_pt {
        return None;
    }

    let placeholder_segments = placeholder_segments_v0();
    let placeholder_render_segments = split_superscript_segments_v0(&placeholder_segments);
    let placeholder_width_pt = placeholder_render_segments
        .iter()
        .map(|segment| segment.advance_pt)
        .sum::<f32>();
    let placeholder_x_pt = centered_line_x_v0(placeholder_width_pt);
    emit_render_segments_with_superscript_v0(
        out,
        &placeholder_render_segments,
        placeholder_x_pt,
        *y,
        FONT_SIZE_PT_V0,
    );
    out.extend_from_slice(b"\n");
    *y -= LEADING_PT_V0;

    if *y < min_body_y_pt {
        return None;
    }
    let caption_segments = parse_styled_segments_v0(caption_glyphs)?;
    let caption_render_segments = split_superscript_segments_v0(&caption_segments);
    if caption_render_segments
        .iter()
        .any(|segment| segment.superscript || segment.is_link)
    {
        return None;
    }
    let caption_width_pt = caption_render_segments
        .iter()
        .map(|segment| segment.advance_pt)
        .sum::<f32>();
    let caption_x_pt = centered_line_x_v0(caption_width_pt);
    emit_render_segments_with_superscript_v0(
        out,
        &caption_render_segments,
        caption_x_pt,
        *y,
        FIGURE_CAPTION_FONT_SIZE_PT_V0,
    );
    out.extend_from_slice(b"\n");
    *y -= LEADING_PT_V0;
    Some(())
}

fn emit_toc_block_v0(
    out: &mut Vec<u8>,
    toc_entries: &[TocEntryMetadataV0],
    y: &mut f32,
    min_body_y_pt: f32,
) -> Option<()> {
    if *y < min_body_y_pt {
        return None;
    }

    let title_glyphs: Vec<GlyphPlanV0> = TOC_TITLE_TEXT_V0
        .iter()
        .copied()
        .map(|byte| GlyphPlanV0 {
            byte,
            advance_sp: 65_536,
        })
        .collect();
    let title_segments = vec![PdfStyledSegmentV0 {
        style: PdfTextStyleV0::Bold,
        advance_pt: title_glyphs
            .iter()
            .map(|glyph| (glyph.advance_sp as f32) / 65_536.0)
            .sum(),
        glyphs: title_glyphs,
        is_link: false,
    }];
    emit_styled_segments_v0(out, &title_segments, MARGIN_PT_V0, *y, TOC_TITLE_FONT_SIZE_PT_V0);
    out.extend_from_slice(b"\n");
    *y -= LEADING_PT_V0;

    for entry in toc_entries {
        if *y < min_body_y_pt {
            return None;
        }
        let base_segments = parse_styled_segments_v0(&entry.title_glyphs)?;
        let render_segments = split_superscript_segments_v0(&base_segments);
        if render_segments
            .iter()
            .any(|segment| segment.superscript || segment.is_link)
        {
            return None;
        }
        let indent_steps = f32::from(entry.level.saturating_sub(1));
        let x_pt = MARGIN_PT_V0 + (indent_steps * TOC_ENTRY_INDENT_STEP_PT_V0);
        emit_render_segments_with_superscript_v0(out, &render_segments, x_pt, *y, FONT_SIZE_PT_V0);
        out.extend_from_slice(b"\n");
        *y -= LEADING_PT_V0;
    }
    *y -= LEADING_PT_V0;
    Some(())
}

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
    out.extend_from_slice(b"1 0 0 1 ");
    out.extend_from_slice(format!("{:.2} {:.2} Tm ", x_pt, y_pt).as_bytes());
    for segment in segments {
        if segment.glyphs.is_empty() {
            continue;
        }
        let escaped = escape_pdf_string_bytes(&bytes_from_glyphs_v0(&segment.glyphs));
        out.extend_from_slice(b"/");
        out.extend_from_slice(style_font_alias_v0(segment.style));
        out.extend_from_slice(b" ");
        out.extend_from_slice(format!("{font_size_pt}").as_bytes());
        out.extend_from_slice(b" Tf (");
        out.extend_from_slice(&escaped);
        out.extend_from_slice(b") Tj ");
    }
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
    out.extend_from_slice(b"1 0 0 1 ");
    out.extend_from_slice(format!("{:.2} {:.2} Tm ", x_pt, y_pt).as_bytes());
    for segment in segments {
        if segment.bytes.is_empty() {
            continue;
        }
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
    }
    out.extend_from_slice(b"0 Ts ");
}

fn is_heading_line_segments_v0(segments: &[PdfStyledSegmentV0]) -> bool {
    if segments.is_empty() {
        return false;
    }
    let mut saw_non_space = false;
    for segment in segments {
        if segment.style != PdfTextStyleV0::Bold {
            return false;
        }
        if segment.glyphs.iter().any(|glyph| glyph.byte != b' ') {
            saw_non_space = true;
        }
    }
    saw_non_space
}

fn is_safe_link_uri_byte_v0(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || matches!(
            byte,
            b':'
                | b'/'
                | b'?'
                | b'#'
                | b'['
                | b']'
                | b'@'
                | b'!'
                | b'$'
                | b'&'
                | b'\''
                | b'('
                | b')'
                | b'*'
                | b'+'
                | b','
                | b';'
                | b'='
                | b'%'
                | b'.'
                | b'_'
                | b'-'
                | b'~'
        )
}

fn parse_footnote_definition_line_v0(glyphs: &[GlyphPlanV0]) -> Option<(u32, Vec<GlyphPlanV0>)> {
    if glyphs.len() < FOOTNOTE_LINE_PREFIX_MARKER_V0.len() {
        return None;
    }
    if !glyphs
        .iter()
        .take(FOOTNOTE_LINE_PREFIX_MARKER_V0.len())
        .map(|glyph| glyph.byte)
        .eq(FOOTNOTE_LINE_PREFIX_MARKER_V0.iter().copied())
    {
        return None;
    }
    let marker_start = FOOTNOTE_LINE_PREFIX_MARKER_V0.len();
    let mut cursor = marker_start;
    let mut marker_id = 0u32;
    let mut saw_digit = false;
    while cursor < glyphs.len() && glyphs[cursor].byte.is_ascii_digit() {
        marker_id = marker_id
            .checked_mul(10)?
            .checked_add(u32::from(glyphs[cursor].byte - b'0'))?;
        saw_digit = true;
        cursor += 1;
    }
    if !saw_digit || marker_id == 0 {
        return None;
    }
    if cursor >= glyphs.len() || glyphs[cursor].byte != b' ' {
        return None;
    }
    cursor += 1;
    if cursor >= glyphs.len() {
        return None;
    }
    Some((marker_id, glyphs[marker_start..].to_vec()))
}

fn has_href_url_line_prefix_v0(glyphs: &[GlyphPlanV0]) -> bool {
    glyphs.len() >= HREF_URL_LINE_PREFIX_MARKER_V0.len()
        && glyphs[..HREF_URL_LINE_PREFIX_MARKER_V0.len()]
            .iter()
            .map(|glyph| glyph.byte)
            .eq(HREF_URL_LINE_PREFIX_MARKER_V0.iter().copied())
}

fn has_label_line_prefix_v0(glyphs: &[GlyphPlanV0]) -> bool {
    glyphs.len() >= LABEL_LINE_PREFIX_MARKER_V0.len()
        && glyphs[..LABEL_LINE_PREFIX_MARKER_V0.len()]
            .iter()
            .map(|glyph| glyph.byte)
            .eq(LABEL_LINE_PREFIX_MARKER_V0.iter().copied())
}

fn has_ref_line_prefix_v0(glyphs: &[GlyphPlanV0]) -> bool {
    glyphs.len() >= REF_LINE_PREFIX_MARKER_V0.len()
        && glyphs[..REF_LINE_PREFIX_MARKER_V0.len()]
            .iter()
            .map(|glyph| glyph.byte)
            .eq(REF_LINE_PREFIX_MARKER_V0.iter().copied())
}

fn parse_href_url_line_v0(glyphs: &[GlyphPlanV0]) -> Option<(u32, Vec<u8>)> {
    if glyphs.len() < HREF_URL_LINE_PREFIX_MARKER_V0.len() {
        return None;
    }
    if !glyphs
        .iter()
        .take(HREF_URL_LINE_PREFIX_MARKER_V0.len())
        .map(|glyph| glyph.byte)
        .eq(HREF_URL_LINE_PREFIX_MARKER_V0.iter().copied())
    {
        return None;
    }
    let mut cursor = HREF_URL_LINE_PREFIX_MARKER_V0.len();
    let mut marker_id = 0u32;
    let mut saw_index_digit = false;
    while cursor < glyphs.len() && glyphs[cursor].byte.is_ascii_digit() {
        marker_id = marker_id
            .checked_mul(10)?
            .checked_add(u32::from(glyphs[cursor].byte - b'0'))?;
        saw_index_digit = true;
        cursor += 1;
    }
    if !saw_index_digit || marker_id == 0 {
        return None;
    }
    if cursor >= glyphs.len() || glyphs[cursor].byte != b' ' {
        return None;
    }
    cursor += 1;
    if cursor >= glyphs.len() {
        return None;
    }
    let mut uri = Vec::<u8>::with_capacity(glyphs.len() - cursor);
    for glyph in &glyphs[cursor..] {
        let byte = glyph.byte;
        if !is_safe_link_uri_byte_v0(byte) {
            return None;
        }
        uri.push(byte);
    }
    if uri.is_empty() {
        return None;
    }
    Some((marker_id, uri))
}

fn parse_label_line_v0(glyphs: &[GlyphPlanV0]) -> Option<()> {
    if glyphs.len() < LABEL_LINE_PREFIX_MARKER_V0.len() {
        return None;
    }
    let bytes: Vec<u8> = glyphs.iter().map(|glyph| glyph.byte).collect();
    if !bytes.starts_with(LABEL_LINE_PREFIX_MARKER_V0) {
        return None;
    }
    let line = String::from_utf8(bytes).ok()?;
    let mut parts = line.splitn(6, ' ');
    let prefix = parts.next()?;
    if prefix != "!l" {
        return None;
    }
    let key = parts.next()?.trim();
    let anchor_id = parts.next()?.trim().parse::<u32>().ok()?;
    let kind = parts.next()?.trim();
    let level = parts.next()?.trim().parse::<u8>().ok()?;
    let title = parts.next()?.trim();
    if key.is_empty() || anchor_id == 0 {
        return None;
    }
    if kind != "heading" && kind != "figure" {
        return None;
    }
    if kind == "heading" && !(1..=2).contains(&level) {
        return None;
    }
    if kind == "figure" && level != 0 {
        return None;
    }
    if title.is_empty() {
        return None;
    }
    Some(())
}

fn parse_ref_line_v0(glyphs: &[GlyphPlanV0]) -> Option<()> {
    if glyphs.len() < REF_LINE_PREFIX_MARKER_V0.len() {
        return None;
    }
    let bytes: Vec<u8> = glyphs.iter().map(|glyph| glyph.byte).collect();
    if !bytes.starts_with(REF_LINE_PREFIX_MARKER_V0) {
        return None;
    }
    let line = String::from_utf8(bytes).ok()?;
    let mut parts = line.splitn(4, ' ');
    let prefix = parts.next()?;
    if prefix != "!r" {
        return None;
    }
    let key = parts.next()?.trim();
    let line_index = parts.next()?.trim().parse::<u32>().ok()?;
    let resolved_anchor_id = parts.next()?.trim().parse::<u32>().ok()?;
    if key.is_empty() || line_index == 0 {
        return None;
    }
    if resolved_anchor_id == 0 {
        return Some(());
    }
    Some(())
}

fn parse_toc_entry_line_v0(glyphs: &[GlyphPlanV0]) -> Option<TocEntryMetadataV0> {
    if !has_toc_entry_line_prefix_v0(glyphs) {
        return None;
    }
    let mut cursor = TOC_ENTRY_LINE_PREFIX_MARKER_V0.len();

    let mut level = 0u8;
    let mut saw_level_digit = false;
    while cursor < glyphs.len() && glyphs[cursor].byte.is_ascii_digit() {
        level = level
            .checked_mul(10)?
            .checked_add(glyphs[cursor].byte.checked_sub(b'0')?)?;
        saw_level_digit = true;
        cursor += 1;
    }
    if !saw_level_digit || !(1..=2).contains(&level) {
        return None;
    }
    if cursor >= glyphs.len() || glyphs[cursor].byte != b' ' {
        return None;
    }
    cursor += 1;

    let mut anchor_id = 0u32;
    let mut saw_anchor_digit = false;
    while cursor < glyphs.len() && glyphs[cursor].byte.is_ascii_digit() {
        anchor_id = anchor_id
            .checked_mul(10)?
            .checked_add(u32::from(glyphs[cursor].byte - b'0'))?;
        saw_anchor_digit = true;
        cursor += 1;
    }
    if !saw_anchor_digit || anchor_id == 0 {
        return None;
    }
    if cursor >= glyphs.len() || glyphs[cursor].byte != b' ' {
        return None;
    }
    cursor += 1;
    if cursor >= glyphs.len() {
        return None;
    }
    let title = glyphs[cursor..].to_vec();
    if title
        .iter()
        .any(|glyph| glyph.byte == NEWLINE_MARKER_V0 || glyph.byte == PAGE_BREAK_MARKER_V0)
    {
        return None;
    }
    Some(TocEntryMetadataV0 {
        level,
        anchor_id,
        title_glyphs: title,
    })
}

fn collect_link_annotations_for_line_v0(
    segments: &[PdfRenderSegmentV0],
    line_x_pt: f32,
    line_y_pt: f32,
    font_size_pt: f32,
    href_url_by_id: &BTreeMap<u32, Vec<u8>>,
    next_link_id: &mut u32,
    active_link_uri: &mut Option<Vec<u8>>,
    link_active_at_line_end: bool,
) -> Option<Vec<PdfLinkAnnotationV0>> {
    let mut annotations = Vec::<PdfLinkAnnotationV0>::new();
    let mut cursor_x = line_x_pt;
    let mut run_start_x = None::<f32>;
    let mut run_end_x = line_x_pt;

    for segment in segments {
        let start_x = cursor_x;
        let end_x = cursor_x + segment.advance_pt;
        if segment.is_link {
            if run_start_x.is_none() {
                run_start_x = Some(start_x);
                if active_link_uri.is_none() {
                    let uri = href_url_by_id.get(next_link_id)?.clone();
                    *next_link_id = next_link_id.checked_add(1)?;
                    *active_link_uri = Some(uri);
                }
            }
            run_end_x = end_x;
        } else if let Some(run_x) = run_start_x.take() {
            let uri = active_link_uri.clone()?;
            if run_end_x <= run_x {
                return None;
            }
            let rect = [
                run_x.clamp(0.0, PAGE_WIDTH_PT_V0),
                (line_y_pt - font_size_pt * 0.30).clamp(0.0, PAGE_HEIGHT_PT_V0),
                run_end_x.clamp(0.0, PAGE_WIDTH_PT_V0),
                (line_y_pt + font_size_pt * 0.85).clamp(0.0, PAGE_HEIGHT_PT_V0),
            ];
            if rect[2] <= rect[0] || rect[3] <= rect[1] {
                return None;
            }
            annotations.push(PdfLinkAnnotationV0 { uri, rect });
            *active_link_uri = None;
        }
        cursor_x = end_x;
    }

    if let Some(run_x) = run_start_x.take() {
        let uri = active_link_uri.clone()?;
        if run_end_x <= run_x {
            return None;
        }
        let rect = [
            run_x.clamp(0.0, PAGE_WIDTH_PT_V0),
            (line_y_pt - font_size_pt * 0.30).clamp(0.0, PAGE_HEIGHT_PT_V0),
            run_end_x.clamp(0.0, PAGE_WIDTH_PT_V0),
            (line_y_pt + font_size_pt * 0.85).clamp(0.0, PAGE_HEIGHT_PT_V0),
        ];
        if rect[2] <= rect[0] || rect[3] <= rect[1] {
            return None;
        }
        annotations.push(PdfLinkAnnotationV0 { uri, rect });
        if !link_active_at_line_end {
            *active_link_uri = None;
        }
    } else if !link_active_at_line_end {
        *active_link_uri = None;
    }

    Some(annotations)
}

fn collect_footnote_marker_ids_from_glyphs_v0(
    glyphs: &[GlyphPlanV0],
    marker_ids: &mut Vec<u32>,
) -> Option<()> {
    let mut index = 0usize;
    while index < glyphs.len() {
        if glyphs[index].byte != FOOTNOTE_MARKER_PREFIX_V0 {
            index += 1;
            continue;
        }
        let mut cursor = index + 1;
        let mut marker_id = 0u32;
        let mut saw_digit = false;
        while cursor < glyphs.len() && glyphs[cursor].byte.is_ascii_digit() {
            marker_id = marker_id
                .checked_mul(10)?
                .checked_add(u32::from(glyphs[cursor].byte - b'0'))?;
            saw_digit = true;
            cursor += 1;
        }
        if saw_digit && marker_id > 0 && !marker_ids.contains(&marker_id) {
            marker_ids.push(marker_id);
        }
        index = if saw_digit { cursor } else { index + 1 };
    }
    Some(())
}

fn collect_page_footnote_marker_ids_v0(lines: &[LinePlanV0]) -> Option<Vec<u32>> {
    let mut marker_ids = Vec::<u32>::new();
    for line in lines {
        collect_footnote_marker_ids_from_glyphs_v0(&line.glyphs, &mut marker_ids)?;
    }
    Some(marker_ids)
}

fn split_body_and_metadata_lines_v0(pages: &[PagePlanV0]) -> (Vec<Vec<LinePlanV0>>, Vec<LinePlanV0>) {
    let mut body_pages = vec![Vec::<LinePlanV0>::new(); pages.len()];
    let mut metadata_lines = Vec::<LinePlanV0>::new();
    let mut in_metadata = false;
    for (page_index, page) in pages.iter().enumerate() {
        for line in &page.lines {
            if has_footnote_line_prefix_v0(&line.glyphs)
                || has_href_url_line_prefix_v0(&line.glyphs)
                || has_toc_entry_line_prefix_v0(&line.glyphs)
                || has_label_line_prefix_v0(&line.glyphs)
                || has_ref_line_prefix_v0(&line.glyphs)
            {
                in_metadata = true;
            }
            if in_metadata {
                metadata_lines.push(line.clone());
            } else {
                body_pages[page_index].push(line.clone());
            }
        }
    }
    while body_pages.len() > 1 {
        let should_pop = body_pages.last().map(|lines| lines.is_empty()).unwrap_or(false);
        if !should_pop {
            break;
        }
        body_pages.pop();
    }
    (body_pages, metadata_lines)
}

fn parse_metadata_lines_v0(
    metadata_lines: &[LinePlanV0],
) -> Option<(
    BTreeMap<u32, Vec<Vec<GlyphPlanV0>>>,
    BTreeMap<u32, Vec<u8>>,
    Vec<TocEntryMetadataV0>,
)> {
    let mut footnote_defs_by_id = BTreeMap::<u32, Vec<Vec<GlyphPlanV0>>>::new();
    let mut href_url_by_id = BTreeMap::<u32, Vec<u8>>::new();
    let mut toc_entries = Vec::<TocEntryMetadataV0>::new();
    let mut current_footnote_id = None::<u32>;
    for line in metadata_lines {
        if let Some((footnote_id, footnote_line)) = parse_footnote_definition_line_v0(&line.glyphs) {
            if footnote_defs_by_id.contains_key(&footnote_id) {
                return None;
            }
            footnote_defs_by_id.insert(footnote_id, vec![footnote_line]);
            current_footnote_id = Some(footnote_id);
            continue;
        }
        if let Some((href_id, href_url)) = parse_href_url_line_v0(&line.glyphs) {
            if href_url_by_id.insert(href_id, href_url).is_some() {
                return None;
            }
            current_footnote_id = None;
            continue;
        }
        if let Some(toc_entry) = parse_toc_entry_line_v0(&line.glyphs) {
            if toc_entries
                .iter()
                .any(|entry| entry.anchor_id == toc_entry.anchor_id)
            {
                return None;
            }
            toc_entries.push(toc_entry);
            current_footnote_id = None;
            continue;
        }
        if parse_label_line_v0(&line.glyphs).is_some() {
            current_footnote_id = None;
            continue;
        }
        if parse_ref_line_v0(&line.glyphs).is_some() {
            current_footnote_id = None;
            continue;
        }
        if line.glyphs.is_empty() {
            if let Some(footnote_id) = current_footnote_id {
                footnote_defs_by_id.get_mut(&footnote_id)?.push(Vec::new());
            }
            continue;
        }
        let footnote_id = current_footnote_id?;
        footnote_defs_by_id.get_mut(&footnote_id)?.push(line.glyphs.clone());
    }
    toc_entries.sort_by_key(|entry| entry.anchor_id);
    Some((footnote_defs_by_id, href_url_by_id, toc_entries))
}

fn build_page_content_stream_v0(
    lines: &[LinePlanV0],
    footnote_defs_by_id: &BTreeMap<u32, Vec<Vec<GlyphPlanV0>>>,
    href_url_by_id: &BTreeMap<u32, Vec<u8>>,
    toc_entries: &[TocEntryMetadataV0],
    next_link_id: &mut u32,
    allow_title_block: bool,
) -> Option<PageRenderV0> {
    let mut out = Vec::new();
    out.extend_from_slice(b"BT\n");
    out.extend_from_slice(b"0 g\n");

    let page_footnote_ids = collect_page_footnote_marker_ids_v0(lines)?;
    let mut footnote_line_count = 0usize;
    for footnote_id in &page_footnote_ids {
        let Some(footnote_lines) = footnote_defs_by_id.get(footnote_id) else { return None; };
        footnote_line_count = footnote_line_count.checked_add(footnote_lines.len())?;
    }
    let footnote_reserved_height_pt = if footnote_line_count == 0 {
        0.0
    } else {
        FOOTNOTE_BLOCK_GAP_PT_V0 + (footnote_line_count as f32 * FOOTNOTE_LEADING_PT_V0)
    };
    if footnote_reserved_height_pt >= (PAGE_HEIGHT_PT_V0 - (2.0 * MARGIN_PT_V0)) {
        return None;
    }
    let min_body_y_pt = MARGIN_PT_V0 + footnote_reserved_height_pt;

    let title_block_len = if allow_title_block {
        detect_title_block_len_v0(lines)
    } else {
        0
    };
    let mut y = PAGE_HEIGHT_PT_V0 - MARGIN_PT_V0 - TITLE_FONT_SIZE_PT_V0;
    let mut previous_rendered_line_was_empty = false;
    let mut skip_indent_after_title_block = title_block_len > 0;
    let mut active_hang_indent_pt = 0.0f32;
    let mut active_quote_indent_pt = 0.0f32;
    let mut annotations = Vec::<PdfLinkAnnotationV0>::new();
    let mut style_stack = Vec::<PdfTextStyleV0>::new();
    let mut current_style = PdfTextStyleV0::Regular;
    let mut link_active = false;
    let mut active_link_uri = None::<Vec<u8>>;
    let mut line_index = 0usize;
    while line_index < lines.len() {
        let line = &lines[line_index];
        if y < MARGIN_PT_V0 {
            break;
        }
        if y < min_body_y_pt {
            return None;
        }
        if line_index >= title_block_len && has_table_row_prefix_v0(&line.glyphs) {
            let mut table_end = line_index;
            while table_end < lines.len() && has_table_row_prefix_v0(&lines[table_end].glyphs) {
                table_end += 1;
            }
            emit_table_block_v0(&mut out, &lines[line_index..table_end], &mut y, min_body_y_pt)?;
            previous_rendered_line_was_empty = false;
            skip_indent_after_title_block = false;
            active_hang_indent_pt = 0.0;
            active_quote_indent_pt = 0.0;
            line_index = table_end;
            continue;
        }
        if line_index >= title_block_len && has_figure_box_prefix_v0(&line.glyphs) {
            let caption_line = lines.get(line_index + 1)?;
            let caption_glyphs = parse_figure_caption_line_v0(&caption_line.glyphs)?;
            emit_figure_block_v0(&mut out, &caption_glyphs, &mut y, min_body_y_pt)?;
            previous_rendered_line_was_empty = false;
            skip_indent_after_title_block = false;
            active_hang_indent_pt = 0.0;
            active_quote_indent_pt = 0.0;
            line_index += 2;
            continue;
        }
        if line_index >= title_block_len && has_figure_caption_prefix_v0(&line.glyphs) {
            return None;
        }
        if line_index >= title_block_len && has_toc_placeholder_line_v0(&line.glyphs) {
            emit_toc_block_v0(&mut out, toc_entries, &mut y, min_body_y_pt)?;
            previous_rendered_line_was_empty = false;
            skip_indent_after_title_block = false;
            active_hang_indent_pt = 0.0;
            active_quote_indent_pt = 0.0;
            line_index += 1;
            continue;
        }
        if line_index >= title_block_len && has_toc_entry_line_prefix_v0(&line.glyphs) {
            return None;
        }
        let quote_prefix_advance_pt = if line_index >= title_block_len {
            detect_quote_prefix_advance_pt_v0(&line.glyphs)
        } else {
            None
        };
        let center_prefixed = line_index >= title_block_len
            && quote_prefix_advance_pt.is_none()
            && has_center_prefix_v0(&line.glyphs);
        let right_prefixed = line_index >= title_block_len
            && quote_prefix_advance_pt.is_none()
            && !center_prefixed
            && has_right_prefix_v0(&line.glyphs);
        let noindent_prefixed = line_index >= title_block_len
            && quote_prefix_advance_pt.is_none()
            && !center_prefixed
            && !right_prefixed
            && has_noindent_prefix_v0(&line.glyphs);
        let heading_kind = if line_index >= title_block_len
            && quote_prefix_advance_pt.is_none()
            && !center_prefixed
            && !right_prefixed
            && !noindent_prefixed
        {
            detect_heading_prefix_v0(&line.glyphs)
        } else {
            None
        };
        let render_glyphs_base: &[GlyphPlanV0] = if quote_prefix_advance_pt.is_some() {
            &line.glyphs[2..]
        } else if center_prefixed {
            &line.glyphs[2..]
        } else if right_prefixed {
            &line.glyphs[2..]
        } else if noindent_prefixed {
            &line.glyphs[2..]
        } else if let Some(kind) = heading_kind {
            &line.glyphs[heading_prefix_len_v0(kind)..]
        } else {
            &line.glyphs
        };
        let list_prefix = if line_index >= title_block_len
            && quote_prefix_advance_pt.is_none()
            && !center_prefixed
            && !right_prefixed
            && !noindent_prefixed
            && heading_kind.is_none()
        {
            detect_list_prefix_v0(render_glyphs_base)
        } else {
            None
        };
        let render_glyphs: &[GlyphPlanV0] = if quote_prefix_advance_pt.is_some() {
            render_glyphs_base
        } else if let Some(prefix) = list_prefix {
            &render_glyphs_base[prefix.prefix_len..]
        } else {
            render_glyphs_base
        };

        let Some(segments) = parse_styled_segments_with_state_v0(
            render_glyphs,
            &mut style_stack,
            &mut current_style,
            &mut link_active,
        ) else { return None; };
        let render_segments = split_superscript_segments_v0(&segments);
        let line_is_empty = render_segments.is_empty();
        let in_title_block = title_block_len > 0 && line_index < title_block_len;
        let font_size_pt = if in_title_block && line_index == 0 {
            TITLE_FONT_SIZE_PT_V0
        } else if matches!(heading_kind, Some(HeadingKindV0::Section)) {
            SECTION_HEADING_FONT_SIZE_PT_V0
        } else if matches!(heading_kind, Some(HeadingKindV0::Subsection)) {
            SUBSECTION_HEADING_FONT_SIZE_PT_V0
        } else {
            FONT_SIZE_PT_V0
        };
        let next_raw_line_is_empty = lines
            .get(line_index + 1)
            .map(|next_line| next_line.glyphs.is_empty())
            .unwrap_or(false);
        let heading_centered = !in_title_block
            && !line_is_empty
            && quote_prefix_advance_pt.is_none()
            && !center_prefixed
            && !right_prefixed
            && !noindent_prefixed
            && list_prefix.is_none()
            && previous_rendered_line_was_empty
            && next_raw_line_is_empty
            && is_heading_line_segments_v0(&segments);
        if !line_is_empty {
            let line_width_pt: f32 = render_segments.iter().map(|segment| segment.advance_pt).sum();
            let line_x = if in_title_block {
                centered_line_x_v0(line_width_pt)
            } else if heading_kind.is_some() || heading_centered {
                active_quote_indent_pt = 0.0;
                active_hang_indent_pt = 0.0;
                centered_line_x_v0(line_width_pt)
            } else if center_prefixed {
                active_quote_indent_pt = 0.0;
                active_hang_indent_pt = 0.0;
                centered_line_x_v0(line_width_pt)
            } else if right_prefixed {
                active_quote_indent_pt = 0.0;
                active_hang_indent_pt = 0.0;
                (PAGE_WIDTH_PT_V0 - MARGIN_PT_V0 - line_width_pt).max(MARGIN_PT_V0)
            } else if let Some(prefix_advance_pt) = quote_prefix_advance_pt {
                active_quote_indent_pt = (FONT_SIZE_PT_V0 * 2.0).max(prefix_advance_pt);
                active_hang_indent_pt = 0.0;
                MARGIN_PT_V0 + active_quote_indent_pt
            } else if let Some(prefix) = list_prefix {
                active_hang_indent_pt = LIST_BODY_INDENT_PT_V0 + prefix.leading_advance_pt;
                active_quote_indent_pt = 0.0;
                MARGIN_PT_V0 + active_hang_indent_pt
            } else if noindent_prefixed {
                active_hang_indent_pt = 0.0;
                active_quote_indent_pt = 0.0;
                MARGIN_PT_V0
            } else if active_quote_indent_pt > 0.0 {
                MARGIN_PT_V0 + active_quote_indent_pt
            } else if active_hang_indent_pt > 0.0 {
                MARGIN_PT_V0 + active_hang_indent_pt
            } else if previous_rendered_line_was_empty && !skip_indent_after_title_block {
                MARGIN_PT_V0 + INDENT_PT_V0
            } else {
                MARGIN_PT_V0
            };
            if let Some(prefix) = list_prefix {
                let display_prefix_glyphs = &render_glyphs_base
                    [prefix.display_start..prefix.display_start + prefix.display_len];
                let Some(display_prefix_segments) = parse_styled_segments_v0(display_prefix_glyphs) else {
                    return None;
                };
                let prefix_width_pt = glyphs_advance_pt_v0(display_prefix_glyphs);
                let prefix_x = match prefix.kind {
                    ListPrefixKindV0::Itemize => MARGIN_PT_V0 + prefix.leading_advance_pt,
                    ListPrefixKindV0::Enumerate => {
                        ENUM_NUMBER_COLUMN_RIGHT_PT_V0 + prefix.leading_advance_pt - prefix_width_pt
                    }
                };
                emit_styled_segments_v0(&mut out, &display_prefix_segments, prefix_x, y, font_size_pt);
            }
            let mut line_annotations = collect_link_annotations_for_line_v0(
                &render_segments,
                line_x,
                y,
                font_size_pt,
                href_url_by_id,
                next_link_id,
                &mut active_link_uri,
                link_active,
            )?;
            annotations.append(&mut line_annotations);
            let has_superscript = render_segments.iter().any(|segment| segment.superscript);
            if has_superscript {
                emit_render_segments_with_superscript_v0(
                    &mut out,
                    &render_segments,
                    line_x,
                    y,
                    font_size_pt,
                );
            } else {
                emit_styled_segments_v0(&mut out, &segments, line_x, y, font_size_pt);
            }
            out.extend_from_slice(b"\n");
            if !in_title_block && skip_indent_after_title_block {
                skip_indent_after_title_block = false;
            }
        } else {
            active_hang_indent_pt = 0.0;
            active_quote_indent_pt = 0.0;
        }
        previous_rendered_line_was_empty = line_is_empty;
        y -= LEADING_PT_V0;
        if title_block_len > 0 && line_index + 1 == title_block_len {
            y -= TITLE_EXTRA_GAP_PT_V0;
        }
        line_index += 1;
    }
    if !style_stack.is_empty() || link_active {
        return None;
    }
    if active_link_uri.is_some() {
        return None;
    }

    if footnote_line_count > 0 {
        let mut footnote_y =
            MARGIN_PT_V0 + footnote_reserved_height_pt - FOOTNOTE_LEADING_PT_V0;
        for footnote_id in &page_footnote_ids {
            for footnote_line in footnote_defs_by_id.get(footnote_id)? {
                if footnote_y < MARGIN_PT_V0 {
                    return None;
                }
                let Some(segments) = parse_styled_segments_v0(footnote_line) else { return None; };
                if !segments.is_empty() {
                    let render_segments = split_superscript_segments_v0(&segments);
                    let has_superscript = render_segments.iter().any(|segment| segment.superscript);
                    if has_superscript {
                        emit_render_segments_with_superscript_v0(
                            &mut out,
                            &render_segments,
                            MARGIN_PT_V0,
                            footnote_y,
                            FOOTNOTE_FONT_SIZE_PT_V0,
                        );
                    } else {
                        emit_styled_segments_v0(
                            &mut out,
                            &segments,
                            MARGIN_PT_V0,
                            footnote_y,
                            FOOTNOTE_FONT_SIZE_PT_V0,
                        );
                    }
                    out.extend_from_slice(b"\n");
                }
                footnote_y -= FOOTNOTE_LEADING_PT_V0;
            }
        }
    }

    out.extend_from_slice(b"ET\n");
    Some(PageRenderV0 {
        stream: out,
        annotations,
    })
}

fn build_pdf_for_pages_v0(pages: &[PagePlanV0]) -> Vec<u8> {
    let (body_pages, metadata_lines) = split_body_and_metadata_lines_v0(pages);
    let Some((footnote_defs_by_id, href_url_by_id, toc_entries)) = parse_metadata_lines_v0(&metadata_lines) else {
        return Vec::new();
    };

    let mut next_link_id = 1u32;
    let mut page_renders = Vec::<PageRenderV0>::with_capacity(body_pages.len());
    for (page_index, body_lines) in body_pages.iter().enumerate() {
        let Some(rendered) = build_page_content_stream_v0(
            body_lines,
            &footnote_defs_by_id,
            &href_url_by_id,
            &toc_entries,
            &mut next_link_id,
            page_index == 0,
        ) else { return Vec::new(); };
        page_renders.push(rendered);
    }
    if usize::try_from(next_link_id.saturating_sub(1)).ok() != Some(href_url_by_id.len()) {
        return Vec::new();
    }

    // Object numbering:
    // 1: Catalog
    // 2: Pages
    // 3..(3+page_count-1): Page objects
    // (3+page_count)..(3+2*page_count-1): Content stream objects
    // next: annotation objects (if any)
    // next: annotation action objects (if any)
    // last: Font objects (regular/italic/bold)
    let page_count = page_renders.len() as u32;
    let total_annotations = page_renders
        .iter()
        .map(|page| u32::try_from(page.annotations.len()).unwrap_or(0))
        .sum::<u32>();
    let first_page_id = 3u32;
    let first_stream_id = first_page_id + page_count;
    let first_annotation_id = first_stream_id + page_count;
    let first_action_id = first_annotation_id + total_annotations;
    let font_regular_id = first_action_id + total_annotations;
    let font_italic_id = font_regular_id + 1;
    let font_bold_id = font_regular_id + 2;
    let pages_id = 2u32;

    let mut out = Vec::<u8>::new();
    out.extend_from_slice(PDF_VERSION);

    let mut offsets = Vec::<u32>::new();
    offsets.push(0);

    // 1: Catalog
    offsets.push(write_pdf_obj(
        &mut out,
        1,
        format!("<< /Type /Catalog /Pages {pages_id} 0 R >>").as_bytes(),
    ));

    // 2: Pages (Kids filled in after we know ids, but ids are deterministic)
    let mut kids = Vec::<u8>::new();
    kids.extend_from_slice(b"[");
    for page_index in 0..page_count {
        let page_id = first_page_id + page_index;
        kids.extend_from_slice(b" ");
        kids.extend_from_slice(page_id.to_string().as_bytes());
        kids.extend_from_slice(b" 0 R");
    }
    kids.extend_from_slice(b" ]");
    offsets.push(write_pdf_obj(
        &mut out,
        pages_id,
        format!(
            "<< /Type /Pages /Kids {} /Count {} >>",
            core::str::from_utf8(&kids).unwrap_or("[]"),
            page_count
        )
        .as_bytes(),
    ));

    // Page objects
    let mut annotation_offset = 0u32;
    for page_index in 0..page_count {
        let page_id = first_page_id + page_index;
        let stream_id = first_stream_id + page_index;
        let mut annots_part = String::new();
        let page_annotation_count = page_renders[page_index as usize].annotations.len() as u32;
        if page_annotation_count > 0 {
            annots_part.push_str(" /Annots [");
            for annotation_index in 0..page_annotation_count {
                let annotation_id = first_annotation_id + annotation_offset + annotation_index;
                annots_part.push(' ');
                annots_part.push_str(&annotation_id.to_string());
                annots_part.push_str(" 0 R");
            }
            annots_part.push_str(" ]");
        }
        let body = format!(
            "<< /Type /Page /Parent {pages_id} 0 R /MediaBox [0 0 {PAGE_WIDTH_PT_V0} {PAGE_HEIGHT_PT_V0}] /Resources << /Font << /F1 {font_regular_id} 0 R /F2 {font_italic_id} 0 R /F3 {font_bold_id} 0 R >> >> /Contents {stream_id} 0 R{annots_part} >>"
        );
        offsets.push(write_pdf_obj(&mut out, page_id, body.as_bytes()));
        annotation_offset += page_annotation_count;
    }

    // Content stream objects
    for page_index in 0..page_count {
        let stream_id = first_stream_id + page_index;
        offsets.push(write_pdf_stream_obj(
            &mut out,
            stream_id,
            &page_renders[page_index as usize].stream,
        ));
    }

    let mut annotation_counter = 0u32;
    for page in &page_renders {
        for annotation in &page.annotations {
            let annotation_id = first_annotation_id + annotation_counter;
            let action_id = first_action_id + annotation_counter;
            let annot_body = format!(
                "<< /Type /Annot /Subtype /Link /Rect [{:.2} {:.2} {:.2} {:.2}] /Border [0 0 0] /A {action_id} 0 R >>",
                annotation.rect[0],
                annotation.rect[1],
                annotation.rect[2],
                annotation.rect[3],
            );
            offsets.push(write_pdf_obj(&mut out, annotation_id, annot_body.as_bytes()));
            let mut action_body = Vec::<u8>::new();
            action_body.extend_from_slice(b"<< /S /URI /URI (");
            action_body.extend_from_slice(&escape_pdf_string_bytes(&annotation.uri));
            action_body.extend_from_slice(b") >>");
            offsets.push(write_pdf_obj(&mut out, action_id, &action_body));
            annotation_counter += 1;
        }
    }

    // Fonts
    offsets.push(write_pdf_obj(
        &mut out,
        font_regular_id,
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>",
    ));
    offsets.push(write_pdf_obj(
        &mut out,
        font_italic_id,
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Oblique >>",
    ));
    offsets.push(write_pdf_obj(
        &mut out,
        font_bold_id,
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold >>",
    ));

    let xref_offset = out.len() as u32;
    out.extend_from_slice(b"xref\n0 ");
    out.extend_from_slice((offsets.len() as u32).to_string().as_bytes());
    out.extend_from_slice(b"\n");
    out.extend_from_slice(b"0000000000 65535 f \n");
    for offset in offsets.iter().skip(1) {
        out.extend_from_slice(format!("{:010} 00000 n \n", offset).as_bytes());
    }

    out.extend_from_slice(b"trailer\n<< /Size ");
    out.extend_from_slice((offsets.len() as u32).to_string().as_bytes());
    out.extend_from_slice(b" /Root 1 0 R >>\nstartxref\n");
    out.extend_from_slice(xref_offset.to_string().as_bytes());
    out.extend_from_slice(b"\n");
    out.extend_from_slice(PDF_EOF);
    out
}

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
