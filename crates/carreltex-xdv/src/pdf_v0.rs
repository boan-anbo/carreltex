use crate::{parse_dvi_v2_text_page_to_layout_v0, GlyphPlanV0, LinePlanV0, PagePlanV0, DEFAULT_LINE_ADVANCE_SP_V0};
use std::collections::VecDeque;

const PDF_VERSION: &[u8] = b"%PDF-1.4\n";
const PDF_EOF: &[u8] = b"%%EOF\n";

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
const NOINDENT_PREFIX_MARKER_V0: u8 = b'~';
const LINK_START_MARKER_V0: u8 = b'<';
const LINK_END_MARKER_V0: u8 = b'>';
const FOOTNOTE_MARKER_PREFIX_V0: u8 = b'^';
const FOOTNOTE_MARKER_FONT_SIZE_PT_V0: f32 = 8.0;
const FOOTNOTE_MARKER_RISE_PT_V0: f32 = 4.0;
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
    has_footnotes: bool,
}

fn parse_styled_segments_v0(glyphs: &[GlyphPlanV0]) -> Option<Vec<PdfStyledSegmentV0>> {
    let mut style_stack = Vec::<PdfTextStyleV0>::new();
    let mut current_style = PdfTextStyleV0::Regular;
    let mut link_active = false;
    let mut segments = Vec::<PdfStyledSegmentV0>::new();

    for glyph in glyphs {
        let byte = glyph.byte;
        match byte {
            ITALIC_START_MARKER_V0 => {
                style_stack.push(current_style);
                current_style = PdfTextStyleV0::Italic;
            }
            ITALIC_END_MARKER_V0 => {
                if current_style != PdfTextStyleV0::Italic {
                    return None;
                }
                current_style = style_stack.pop()?;
            }
            BOLD_START_MARKER_V0 => {
                style_stack.push(current_style);
                current_style = PdfTextStyleV0::Bold;
            }
            BOLD_END_MARKER_V0 => {
                if current_style != PdfTextStyleV0::Bold {
                    return None;
                }
                current_style = style_stack.pop()?;
            }
            LINK_START_MARKER_V0 => {
                if link_active {
                    return None;
                }
                link_active = true;
            }
            LINK_END_MARKER_V0 => {
                if !link_active {
                    return None;
                }
                link_active = false;
            }
            _ => {
                let advance_pt = (glyph.advance_sp as f32) / 65_536.0;
                if let Some(segment) = segments.last_mut() {
                    if segment.style == current_style && segment.is_link == link_active {
                        segment.glyphs.push(glyph.clone());
                        segment.advance_pt += advance_pt;
                        continue;
                    }
                }
                segments.push(PdfStyledSegmentV0 {
                    style: current_style,
                    glyphs: vec![glyph.clone()],
                    advance_pt,
                    is_link: link_active,
                });
            }
        }
    }
    if !style_stack.is_empty() || link_active {
        return None;
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

fn parse_href_url_line_v0(glyphs: &[GlyphPlanV0]) -> Option<Vec<u8>> {
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
    let mut saw_index_digit = false;
    while cursor < glyphs.len() && glyphs[cursor].byte.is_ascii_digit() {
        saw_index_digit = true;
        cursor += 1;
    }
    if !saw_index_digit {
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
    Some(uri)
}

fn collect_link_annotations_for_line_v0(
    segments: &[PdfRenderSegmentV0],
    line_x_pt: f32,
    line_y_pt: f32,
    font_size_pt: f32,
    pending_urls: &mut VecDeque<Vec<u8>>,
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
            }
            run_end_x = end_x;
        } else if let Some(run_x) = run_start_x.take() {
            let uri = pending_urls.pop_front()?;
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
        }
        cursor_x = end_x;
    }

    if let Some(run_x) = run_start_x.take() {
        let uri = pending_urls.pop_front()?;
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
    }

    Some(annotations)
}

fn build_page_content_stream_v0(lines: &[LinePlanV0]) -> Option<PageRenderV0> {
    let mut out = Vec::new();
    out.extend_from_slice(b"BT\n");
    out.extend_from_slice(b"0 g\n");

    let mut pending_link_urls = VecDeque::<Vec<u8>>::new();
    let mut render_lines = Vec::<LinePlanV0>::new();
    for line in lines {
        if let Some(url) = parse_href_url_line_v0(&line.glyphs) {
            pending_link_urls.push_back(url);
            continue;
        }
        render_lines.push(line.clone());
    }

    let footnote_block_start = render_lines
        .iter()
        .position(|line| has_footnote_line_prefix_v0(&line.glyphs))
        .unwrap_or(render_lines.len());
    let body_lines = &render_lines[..footnote_block_start];
    let footnote_lines = &render_lines[footnote_block_start..];
    let footnote_reserved_height_pt = if footnote_lines.is_empty() {
        0.0
    } else {
        FOOTNOTE_BLOCK_GAP_PT_V0 + (footnote_lines.len() as f32 * FOOTNOTE_LEADING_PT_V0)
    };
    if footnote_reserved_height_pt >= (PAGE_HEIGHT_PT_V0 - (2.0 * MARGIN_PT_V0)) {
        return None;
    }
    let min_body_y_pt = MARGIN_PT_V0 + footnote_reserved_height_pt;

    let title_block_len = detect_title_block_len_v0(body_lines);
    let mut y = PAGE_HEIGHT_PT_V0 - MARGIN_PT_V0 - TITLE_FONT_SIZE_PT_V0;
    let mut previous_rendered_line_was_empty = false;
    let mut skip_indent_after_title_block = title_block_len > 0;
    let mut active_hang_indent_pt = 0.0f32;
    let mut active_quote_indent_pt = 0.0f32;
    let mut annotations = Vec::<PdfLinkAnnotationV0>::new();
    for (line_index, line) in body_lines.iter().enumerate() {
        if y < MARGIN_PT_V0 {
            break;
        }
        if y < min_body_y_pt {
            break;
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

        let segments = parse_styled_segments_v0(render_glyphs)?;
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
        let next_raw_line_is_empty = body_lines
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
                let display_prefix_segments = parse_styled_segments_v0(display_prefix_glyphs)?;
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
                &mut pending_link_urls,
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
    }

    if !footnote_lines.is_empty() {
        let mut footnote_y =
            MARGIN_PT_V0 + footnote_reserved_height_pt - FOOTNOTE_LEADING_PT_V0;
        for line in footnote_lines {
            if footnote_y < MARGIN_PT_V0 {
                return None;
            }
            let render_glyphs = if has_footnote_line_prefix_v0(&line.glyphs) {
                &line.glyphs[FOOTNOTE_LINE_PREFIX_MARKER_V0.len()..]
            } else {
                &line.glyphs[..]
            };
            let segments = parse_styled_segments_v0(render_glyphs)?;
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

    if !pending_link_urls.is_empty() {
        return None;
    }

    out.extend_from_slice(b"ET\n");
    Some(PageRenderV0 {
        stream: out,
        annotations,
        has_footnotes: !footnote_lines.is_empty(),
    })
}

fn build_pdf_for_pages_v0(pages: &[PagePlanV0]) -> Vec<u8> {
    let mut page_renders = Vec::<PageRenderV0>::with_capacity(pages.len());
    for page in pages {
        let Some(rendered) = build_page_content_stream_v0(&page.lines) else {
            return Vec::new();
        };
        page_renders.push(rendered);
    }
    if page_renders.len() > 1
        && page_renders
            .iter()
            .any(|page| page.has_footnotes || !page.annotations.is_empty())
    {
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
    let page_count = pages.len() as u32;
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
