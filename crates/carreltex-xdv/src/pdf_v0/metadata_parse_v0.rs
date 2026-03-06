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
            b':' | b'/'
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

fn has_pageref_line_prefix_v0(glyphs: &[GlyphPlanV0]) -> bool {
    glyphs.len() >= PAGEREF_LINE_PREFIX_MARKER_V0.len()
        && glyphs[..PAGEREF_LINE_PREFIX_MARKER_V0.len()]
            .iter()
            .map(|glyph| glyph.byte)
            .eq(PAGEREF_LINE_PREFIX_MARKER_V0.iter().copied())
}

fn has_ref_anchor_link_line_prefix_v0(glyphs: &[GlyphPlanV0]) -> bool {
    glyphs.len() >= REF_ANCHOR_LINK_LINE_PREFIX_MARKER_V0.len()
        && glyphs[..REF_ANCHOR_LINK_LINE_PREFIX_MARKER_V0.len()]
            .iter()
            .map(|glyph| glyph.byte)
            .eq(REF_ANCHOR_LINK_LINE_PREFIX_MARKER_V0.iter().copied())
}

fn has_pageref_page_link_line_prefix_v0(glyphs: &[GlyphPlanV0]) -> bool {
    glyphs.len() >= PAGEREF_PAGE_LINK_LINE_PREFIX_MARKER_V0.len()
        && glyphs[..PAGEREF_PAGE_LINK_LINE_PREFIX_MARKER_V0.len()]
            .iter()
            .map(|glyph| glyph.byte)
            .eq(PAGEREF_PAGE_LINK_LINE_PREFIX_MARKER_V0.iter().copied())
}

fn has_equation_line_prefix_v0(glyphs: &[GlyphPlanV0]) -> bool {
    glyphs.len() >= EQUATION_LINE_PREFIX_MARKER_V0.len()
        && glyphs[..EQUATION_LINE_PREFIX_MARKER_V0.len()]
            .iter()
            .map(|glyph| glyph.byte)
            .eq(EQUATION_LINE_PREFIX_MARKER_V0.iter().copied())
}

fn has_bibitem_line_prefix_v0(glyphs: &[GlyphPlanV0]) -> bool {
    glyphs.len() >= BIBITEM_LINE_PREFIX_MARKER_V0.len()
        && glyphs[..BIBITEM_LINE_PREFIX_MARKER_V0.len()]
            .iter()
            .map(|glyph| glyph.byte)
            .eq(BIBITEM_LINE_PREFIX_MARKER_V0.iter().copied())
}

fn has_cite_line_prefix_v0(glyphs: &[GlyphPlanV0]) -> bool {
    glyphs.len() >= CITE_LINE_PREFIX_MARKER_V0.len()
        && glyphs[..CITE_LINE_PREFIX_MARKER_V0.len()]
            .iter()
            .map(|glyph| glyph.byte)
            .eq(CITE_LINE_PREFIX_MARKER_V0.iter().copied())
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
    let level = parts.next()?.trim().parse::<u32>().ok()?;
    let title = parts.next()?.trim();
    if key.is_empty() || anchor_id == 0 {
        return None;
    }
    if kind != "heading" && kind != "figure" && kind != "equation" {
        return None;
    }
    if kind == "heading" && !(1..=2).contains(&level) {
        return None;
    }
    if (kind == "figure" || kind == "equation") && level == 0 {
        return None;
    }
    if title.is_empty() {
        return None;
    }
    Some(())
}

fn parse_equation_line_v0(glyphs: &[GlyphPlanV0]) -> Option<EquationMetadataV0> {
    if glyphs.len() < EQUATION_LINE_PREFIX_MARKER_V0.len() {
        return None;
    }
    let bytes: Vec<u8> = glyphs.iter().map(|glyph| glyph.byte).collect();
    if !bytes.starts_with(EQUATION_LINE_PREFIX_MARKER_V0) {
        return None;
    }
    let line = String::from_utf8(bytes).ok()?;
    let mut parts = line.splitn(3, ' ');
    let prefix = parts.next()?;
    if prefix != "!eq" {
        return None;
    }
    let anchor_id = parts.next()?.trim().parse::<u32>().ok()?;
    let ordinal = parts.next()?.trim().parse::<u32>().ok()?;
    if anchor_id == 0 || ordinal == 0 {
        return None;
    }
    Some(EquationMetadataV0 { anchor_id, ordinal })
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

fn parse_pageref_line_v0(glyphs: &[GlyphPlanV0]) -> Option<()> {
    if glyphs.len() < PAGEREF_LINE_PREFIX_MARKER_V0.len() {
        return None;
    }
    let bytes: Vec<u8> = glyphs.iter().map(|glyph| glyph.byte).collect();
    if !bytes.starts_with(PAGEREF_LINE_PREFIX_MARKER_V0) {
        return None;
    }
    let line = String::from_utf8(bytes).ok()?;
    let mut parts = line.splitn(4, ' ');
    let prefix = parts.next()?;
    if prefix != "!pr" {
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

fn parse_ref_anchor_link_line_v0(glyphs: &[GlyphPlanV0]) -> Option<RefAnchorLinkMetadataV0> {
    if glyphs.len() < REF_ANCHOR_LINK_LINE_PREFIX_MARKER_V0.len() {
        return None;
    }
    let bytes: Vec<u8> = glyphs.iter().map(|glyph| glyph.byte).collect();
    if !bytes.starts_with(REF_ANCHOR_LINK_LINE_PREFIX_MARKER_V0) {
        return None;
    }
    let line = String::from_utf8(bytes).ok()?;
    let mut parts = line.splitn(3, ' ');
    let prefix = parts.next()?;
    if prefix != "!ra" {
        return None;
    }
    let link_id = parts.next()?.trim().parse::<u32>().ok()?;
    let anchor_id = parts.next()?.trim().parse::<u32>().ok()?;
    if link_id == 0 || anchor_id == 0 {
        return None;
    }
    Some(RefAnchorLinkMetadataV0 { link_id, anchor_id })
}

fn parse_pageref_page_link_line_v0(glyphs: &[GlyphPlanV0]) -> Option<PagerefPageLinkMetadataV0> {
    if glyphs.len() < PAGEREF_PAGE_LINK_LINE_PREFIX_MARKER_V0.len() {
        return None;
    }
    let bytes: Vec<u8> = glyphs.iter().map(|glyph| glyph.byte).collect();
    if !bytes.starts_with(PAGEREF_PAGE_LINK_LINE_PREFIX_MARKER_V0) {
        return None;
    }
    let line = String::from_utf8(bytes).ok()?;
    let mut parts = line.splitn(3, ' ');
    let prefix = parts.next()?;
    if prefix != "!rp" {
        return None;
    }
    let link_id = parts.next()?.trim().parse::<u32>().ok()?;
    let anchor_id = parts.next()?.trim().parse::<u32>().ok()?;
    if link_id == 0 || anchor_id == 0 {
        return None;
    }
    Some(PagerefPageLinkMetadataV0 { link_id, anchor_id })
}

fn infer_line_glyph_advance_sp_v0(glyphs: &[GlyphPlanV0]) -> Option<i32> {
    for glyph in glyphs {
        if glyph.advance_sp <= 0 {
            continue;
        }
        if !matches!(
            glyph.byte,
            b' ' | b'.'
                | b','
                | b';'
                | b':'
                | b'!'
                | b'?'
                | b'\''
                | b'"'
                | b'i'
                | b'l'
                | b'I'
                | b'|'
                | b'm'
                | b'w'
                | b'M'
                | b'W'
        ) {
            return Some(glyph.advance_sp);
        }
    }
    glyphs
        .iter()
        .find(|glyph| glyph.advance_sp > 0)
        .map(|glyph| glyph.advance_sp)
}

fn parse_pageref_render_marker_v0(bytes: &[u8], index: usize) -> Option<(u32, usize)> {
    if index + PAGEREF_RENDER_MARKER_PREFIX_V0.len() >= bytes.len()
        || !bytes[index..].starts_with(PAGEREF_RENDER_MARKER_PREFIX_V0)
    {
        return None;
    }
    let mut cursor = index + PAGEREF_RENDER_MARKER_PREFIX_V0.len();
    let mut anchor_id = 0u32;
    let mut saw_digit = false;
    while cursor < bytes.len() && bytes[cursor].is_ascii_digit() {
        saw_digit = true;
        anchor_id = anchor_id
            .checked_mul(10)?
            .checked_add(u32::from(bytes[cursor] - b'0'))?;
        cursor += 1;
    }
    if !saw_digit
        || anchor_id == 0
        || cursor + PAGEREF_RENDER_MARKER_SUFFIX_V0.len() > bytes.len()
        || !bytes[cursor..].starts_with(PAGEREF_RENDER_MARKER_SUFFIX_V0)
    {
        return None;
    }
    Some((anchor_id, cursor + PAGEREF_RENDER_MARKER_SUFFIX_V0.len()))
}

fn replace_pageref_render_markers_v0(
    glyphs: &[GlyphPlanV0],
    page_numbers_by_anchor_id: &BTreeMap<u32, u32>,
) -> Option<Vec<GlyphPlanV0>> {
    let bytes = bytes_from_glyphs_v0(glyphs);
    let mut out = Vec::<GlyphPlanV0>::with_capacity(glyphs.len());
    let glyph_advance_sp = infer_line_glyph_advance_sp_v0(glyphs).unwrap_or(65_536);
    let mut index = 0usize;
    while index < glyphs.len() {
        if let Some((anchor_id, next_index)) = parse_pageref_render_marker_v0(&bytes, index) {
            let page_no = page_numbers_by_anchor_id.get(&anchor_id).copied()?;
            for byte in page_no.to_string().bytes() {
                let advance_sp = glyph_width_sp_v0(byte, glyph_advance_sp)?;
                out.push(GlyphPlanV0 { byte, advance_sp });
            }
            index = next_index;
            continue;
        }
        out.push(glyphs[index].clone());
        index += 1;
    }
    Some(out)
}

fn parse_bibitem_line_v0(glyphs: &[GlyphPlanV0]) -> Option<()> {
    if glyphs.len() < BIBITEM_LINE_PREFIX_MARKER_V0.len() {
        return None;
    }
    let bytes: Vec<u8> = glyphs.iter().map(|glyph| glyph.byte).collect();
    if !bytes.starts_with(BIBITEM_LINE_PREFIX_MARKER_V0) {
        return None;
    }
    let line = String::from_utf8(bytes).ok()?;
    let mut parts = line.splitn(4, ' ');
    let prefix = parts.next()?;
    if prefix != "!b" {
        return None;
    }
    let key = parts.next()?.trim();
    let ordinal = parts.next()?.trim().parse::<u32>().ok()?;
    let text = parts.next()?.trim();
    if key.is_empty() || ordinal == 0 || text.is_empty() {
        return None;
    }
    Some(())
}

fn parse_cite_line_v0(glyphs: &[GlyphPlanV0]) -> Option<()> {
    if glyphs.len() < CITE_LINE_PREFIX_MARKER_V0.len() {
        return None;
    }
    let bytes: Vec<u8> = glyphs.iter().map(|glyph| glyph.byte).collect();
    if !bytes.starts_with(CITE_LINE_PREFIX_MARKER_V0) {
        return None;
    }
    let line = String::from_utf8(bytes).ok()?;
    let mut parts = line.splitn(4, ' ');
    let prefix = parts.next()?;
    if prefix != "!c" {
        return None;
    }
    let key = parts.next()?.trim();
    let line_index = parts.next()?.trim().parse::<u32>().ok()?;
    let resolved_ordinal = parts.next()?.trim().parse::<u32>().ok()?;
    if key.is_empty() || line_index == 0 {
        return None;
    }
    if resolved_ordinal == 0 {
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

