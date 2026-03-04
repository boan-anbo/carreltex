use crate::{parse_dvi_v2_text_page_to_layout_v0, DEFAULT_LINE_ADVANCE_SP_V0};

const PDF_VERSION: &[u8] = b"%PDF-1.4\n";
const PDF_EOF: &[u8] = b"%%EOF\n";

const PAGE_WIDTH_PT_V0: f32 = 612.0;
const PAGE_HEIGHT_PT_V0: f32 = 792.0;
const MARGIN_PT_V0: f32 = 72.0;
const FONT_SIZE_PT_V0: f32 = 12.0;
const LEADING_PT_V0: f32 = 14.0;

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

fn build_page_content_stream_v0(lines: &[Vec<u8>]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"BT\n");
    out.extend_from_slice(b"/F1 ");
    out.extend_from_slice(format!("{FONT_SIZE_PT_V0}").as_bytes());
    out.extend_from_slice(b" Tf\n");
    out.extend_from_slice(b"0 g\n");

    let mut y = PAGE_HEIGHT_PT_V0 - MARGIN_PT_V0 - FONT_SIZE_PT_V0;
    for line in lines {
        if y < MARGIN_PT_V0 {
            break;
        }
        let escaped = escape_pdf_string_bytes(line);
        out.extend_from_slice(b"1 0 0 1 ");
        out.extend_from_slice(format!("{:.2} {:.2} Tm ", MARGIN_PT_V0, y).as_bytes());
        out.extend_from_slice(b"(");
        out.extend_from_slice(&escaped);
        out.extend_from_slice(b") Tj\n");
        y -= LEADING_PT_V0;
    }

    out.extend_from_slice(b"ET\n");
    out
}

fn build_pdf_for_pages_v0(pages: &[Vec<Vec<u8>>]) -> Vec<u8> {
    // Object numbering:
    // 1: Catalog
    // 2: Pages
    // 3..(3+page_count-1): Page objects
    // (3+page_count)..(3+2*page_count-1): Content stream objects
    // last: Font object
    let page_count = pages.len() as u32;
    let first_page_id = 3u32;
    let first_stream_id = first_page_id + page_count;
    let font_id = first_stream_id + page_count;
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
            "<< /Type /Page /Parent {pages_id} 0 R /MediaBox [0 0 {PAGE_WIDTH_PT_V0} {PAGE_HEIGHT_PT_V0}] /Resources << /Font << /F1 {font_id} 0 R >> >> /Contents {stream_id} 0 R >>"
        );
        offsets.push(write_pdf_obj(&mut out, page_id, body.as_bytes()));
    }

    // Content stream objects
    for page_index in 0..page_count {
        let stream_id = first_stream_id + page_index;
        let stream = build_page_content_stream_v0(&pages[page_index as usize]);
        offsets.push(write_pdf_stream_obj(&mut out, stream_id, &stream));
    }

    // Font
    offsets.push(write_pdf_obj(
        &mut out,
        font_id,
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Courier >>",
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

    let mut pages = Vec::<Vec<Vec<u8>>>::new();
    for page in layout.pages {
        let mut lines = Vec::<Vec<u8>>::new();
        for line in page.lines {
            let mut out = Vec::with_capacity(line.glyphs.len());
            for glyph in line.glyphs {
                out.push(glyph.byte);
            }
            lines.push(out);
        }
        pages.push(lines);
    }
    if pages.is_empty() {
        return None;
    }
    Some(build_pdf_for_pages_v0(&pages))
}

