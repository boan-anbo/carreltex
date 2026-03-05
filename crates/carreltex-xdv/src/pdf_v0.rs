use crate::{parse_dvi_v2_text_page_to_layout_v0, GlyphPlanV0, LinePlanV0, PagePlanV0, DEFAULT_LINE_ADVANCE_SP_V0};

const PDF_VERSION: &[u8] = b"%PDF-1.4\n";
const PDF_EOF: &[u8] = b"%%EOF\n";

const PAGE_WIDTH_PT_V0: f32 = 612.0;
const PAGE_HEIGHT_PT_V0: f32 = 792.0;
const MARGIN_PT_V0: f32 = 72.0;
const FONT_SIZE_PT_V0: f32 = 12.0;
const TITLE_FONT_SIZE_PT_V0: f32 = 18.0;
const INDENT_PT_V0: f32 = FONT_SIZE_PT_V0 * 2.0;
const LEADING_PT_V0: f32 = 14.0;
const TITLE_EXTRA_GAP_PT_V0: f32 = LEADING_PT_V0;
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
    bytes: Vec<u8>,
    advance_pt: f32,
}

fn parse_styled_segments_v0(glyphs: &[GlyphPlanV0]) -> Option<Vec<PdfStyledSegmentV0>> {
    let mut style_stack = Vec::<PdfTextStyleV0>::new();
    let mut current_style = PdfTextStyleV0::Regular;
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
            _ => {
                let advance_pt = (glyph.advance_sp as f32) / 65_536.0;
                if let Some(segment) = segments.last_mut() {
                    if segment.style == current_style {
                        segment.bytes.push(byte);
                        segment.advance_pt += advance_pt;
                        continue;
                    }
                }
                segments.push(PdfStyledSegmentV0 {
                    style: current_style,
                    bytes: vec![byte],
                    advance_pt,
                });
            }
        }
    }
    if !style_stack.is_empty() {
        return None;
    }
    Some(segments)
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

fn build_page_content_stream_v0(lines: &[LinePlanV0]) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    out.extend_from_slice(b"BT\n");
    out.extend_from_slice(b"0 g\n");

    let title_block_len = detect_title_block_len_v0(lines);
    let mut y = PAGE_HEIGHT_PT_V0 - MARGIN_PT_V0 - TITLE_FONT_SIZE_PT_V0;
    let mut previous_rendered_line_was_empty = false;
    let mut skip_indent_after_title_block = title_block_len > 0;
    for (line_index, line) in lines.iter().enumerate() {
        if y < MARGIN_PT_V0 {
            break;
        }
        let segments = parse_styled_segments_v0(&line.glyphs)?;
        let line_is_empty = segments.is_empty();
        let in_title_block = title_block_len > 0 && line_index < title_block_len;
        let font_size_pt = if in_title_block && line_index == 0 {
            TITLE_FONT_SIZE_PT_V0
        } else {
            FONT_SIZE_PT_V0
        };
        if !line_is_empty {
            let line_width_pt = (line.width_sp as f32) / 65_536.0;
            let line_x = if in_title_block {
                centered_line_x_v0(line_width_pt)
            } else if previous_rendered_line_was_empty && !skip_indent_after_title_block {
                MARGIN_PT_V0 + INDENT_PT_V0
            } else {
                MARGIN_PT_V0
            };
            let mut segment_x = line_x;
            for segment in segments {
                if segment.bytes.is_empty() {
                    segment_x += segment.advance_pt;
                    continue;
                }
                let escaped = escape_pdf_string_bytes(&segment.bytes);
                out.extend_from_slice(b"1 0 0 1 ");
                out.extend_from_slice(format!("{:.2} {:.2} Tm ", segment_x, y).as_bytes());
                out.extend_from_slice(b"/");
                out.extend_from_slice(style_font_alias_v0(segment.style));
                out.extend_from_slice(b" ");
                out.extend_from_slice(format!("{font_size_pt}").as_bytes());
                out.extend_from_slice(b" Tf (");
                out.extend_from_slice(&escaped);
                out.extend_from_slice(b") Tj ");
                segment_x += segment.advance_pt;
            }
            out.extend_from_slice(b"\n");
            if !in_title_block && skip_indent_after_title_block {
                skip_indent_after_title_block = false;
            }
        }
        previous_rendered_line_was_empty = line_is_empty;
        y -= LEADING_PT_V0;
        if title_block_len > 0 && line_index + 1 == title_block_len {
            y -= TITLE_EXTRA_GAP_PT_V0;
        }
    }

    out.extend_from_slice(b"ET\n");
    Some(out)
}

fn build_pdf_for_pages_v0(pages: &[PagePlanV0]) -> Vec<u8> {
    // Object numbering:
    // 1: Catalog
    // 2: Pages
    // 3..(3+page_count-1): Page objects
    // (3+page_count)..(3+2*page_count-1): Content stream objects
    // last: Font objects (regular/italic/bold)
    let page_count = pages.len() as u32;
    let first_page_id = 3u32;
    let first_stream_id = first_page_id + page_count;
    let font_regular_id = first_stream_id + page_count;
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
    for page_index in 0..page_count {
        let page_id = first_page_id + page_index;
        let stream_id = first_stream_id + page_index;
        let body = format!(
            "<< /Type /Page /Parent {pages_id} 0 R /MediaBox [0 0 {PAGE_WIDTH_PT_V0} {PAGE_HEIGHT_PT_V0}] /Resources << /Font << /F1 {font_regular_id} 0 R /F2 {font_italic_id} 0 R /F3 {font_bold_id} 0 R >> >> /Contents {stream_id} 0 R >>"
        );
        offsets.push(write_pdf_obj(&mut out, page_id, body.as_bytes()));
    }

    // Content stream objects
    for page_index in 0..page_count {
        let stream_id = first_stream_id + page_index;
        let stream = match build_page_content_stream_v0(&pages[page_index as usize].lines) {
            Some(bytes) => bytes,
            None => return Vec::new(),
        };
        offsets.push(write_pdf_stream_obj(&mut out, stream_id, &stream));
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
