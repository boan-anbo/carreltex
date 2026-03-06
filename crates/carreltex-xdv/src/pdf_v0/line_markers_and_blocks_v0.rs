fn detect_list_prefix_v0(glyphs: &[GlyphPlanV0]) -> Option<ListPrefixV0> {
    let mut leading = 0usize;
    while leading < glyphs.len() && glyphs[leading].byte == b' ' {
        leading += 1;
    }
    let leading_advance_pt: f32 = glyphs[..leading]
        .iter()
        .map(|glyph| (glyph.advance_sp as f32) / 65_536.0)
        .sum();

    if glyphs.len() >= leading + 2
        && glyphs[leading].byte == b'-'
        && glyphs[leading + 1].byte == b' '
    {
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

fn is_display_math_placeholder_line_v0(glyphs: &[GlyphPlanV0]) -> bool {
    if !has_center_prefix_v0(glyphs) {
        return false;
    }
    let payload = glyphs[2..]
        .iter()
        .map(|glyph| glyph.byte)
        .collect::<Vec<u8>>();
    payload.as_slice() == DISPLAY_MATH_PLACEHOLDER_SHORT_V0
        || payload.as_slice() == DISPLAY_MATH_PLACEHOLDER_MEDIUM_V0
        || payload.as_slice() == DISPLAY_MATH_PLACEHOLDER_LONG_V0
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

fn has_table_spec_prefix_v0(glyphs: &[GlyphPlanV0]) -> bool {
    glyphs.len() >= TABLE_SPEC_PREFIX_MARKER_V0.len()
        && glyphs[..TABLE_SPEC_PREFIX_MARKER_V0.len()]
            .iter()
            .map(|glyph| glyph.byte)
            .eq(TABLE_SPEC_PREFIX_MARKER_V0.iter().copied())
}

fn has_figure_box_marker_prefix_v0(glyphs: &[GlyphPlanV0]) -> bool {
    glyphs.len() >= FIGURE_BOX_PREFIX_MARKER_V0.len()
        && glyphs[..FIGURE_BOX_PREFIX_MARKER_V0.len()]
            .iter()
            .map(|glyph| glyph.byte)
            .eq(FIGURE_BOX_PREFIX_MARKER_V0.iter().copied())
}

fn parse_figure_box_line_v0(glyphs: &[GlyphPlanV0]) -> Option<FigurePlacementHintV0> {
    if !has_figure_box_marker_prefix_v0(glyphs) {
        return None;
    }
    if glyphs.len() == FIGURE_BOX_PREFIX_MARKER_V0.len() {
        return Some(FigurePlacementHintV0::Inline);
    }
    if glyphs.len() == FIGURE_BOX_PREFIX_MARKER_V0.len() + 2
        && glyphs[FIGURE_BOX_PREFIX_MARKER_V0.len()].byte == b' '
        && glyphs[FIGURE_BOX_PREFIX_MARKER_V0.len() + 1].byte == b't'
    {
        return Some(FigurePlacementHintV0::Top);
    }
    None
}

fn has_figure_box_prefix_v0(glyphs: &[GlyphPlanV0]) -> bool {
    parse_figure_box_line_v0(glyphs).is_some()
}

fn has_figure_caption_prefix_v0(glyphs: &[GlyphPlanV0]) -> bool {
    glyphs.len() >= FIGURE_CAPTION_PREFIX_MARKER_V0.len()
        && glyphs[..FIGURE_CAPTION_PREFIX_MARKER_V0.len()]
            .iter()
            .map(|glyph| glyph.byte)
            .eq(FIGURE_CAPTION_PREFIX_MARKER_V0.iter().copied())
}

fn has_figure_image_prefix_v0(glyphs: &[GlyphPlanV0]) -> bool {
    glyphs.len() >= FIGURE_IMAGE_PREFIX_MARKER_V0.len()
        && glyphs[..FIGURE_IMAGE_PREFIX_MARKER_V0.len()]
            .iter()
            .map(|glyph| glyph.byte)
            .eq(FIGURE_IMAGE_PREFIX_MARKER_V0.iter().copied())
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
        if index + 1 < glyphs.len() && glyphs[index].byte == b'|' && glyphs[index + 1].byte == b'|'
        {
            cells.push(trim_space_glyph_edges_v0(&current));
            current.clear();
            index += 2;
            continue;
        }
        current.push(glyphs[index].clone());
        index += 1;
    }
    cells.push(trim_space_glyph_edges_v0(&current));
    if cells.is_empty() || cells.iter().any(|cell| cell.is_empty()) {
        return None;
    }
    Some(cells)
}

fn parse_table_align_spec_line_v0(glyphs: &[GlyphPlanV0]) -> Option<Vec<u8>> {
    if !has_table_spec_prefix_v0(glyphs) {
        return None;
    }
    let mut spec = Vec::<u8>::new();
    for glyph in &glyphs[TABLE_SPEC_PREFIX_MARKER_V0.len()..] {
        if !matches!(glyph.byte, b'l' | b'c' | b'r') {
            return None;
        }
        spec.push(glyph.byte);
    }
    if spec.is_empty() {
        return None;
    }
    Some(spec)
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

fn is_safe_figure_image_path_byte_v0(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'.' | b'_' | b'-')
}

#[derive(Clone)]
struct FigureImageMetadataV0 {
    image_path: Vec<u8>,
    width_pt: f32,
    height_pt: f32,
}

fn parse_figure_image_line_v0(glyphs: &[GlyphPlanV0]) -> Option<FigureImageMetadataV0> {
    if !has_figure_image_prefix_v0(glyphs) {
        return None;
    }
    let mut cursor = FIGURE_IMAGE_PREFIX_MARKER_V0.len();
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

    let mut path_end = glyphs.len();
    let mut width_pt = DEFAULT_FIGURE_PLACEHOLDER_WIDTH_PT_V0;
    let mut height_pt = DEFAULT_FIGURE_PLACEHOLDER_HEIGHT_PT_V0;
    let mut parsed_size_suffix = false;
    for split in cursor..glyphs.len() {
        if glyphs[split].byte != b' ' {
            continue;
        }
        if split + 1 >= glyphs.len() || !glyphs[split + 1].byte.is_ascii_digit() {
            continue;
        }
        let mut second_sep = split + 1;
        while second_sep < glyphs.len() && glyphs[second_sep].byte.is_ascii_digit() {
            second_sep += 1;
        }
        if second_sep >= glyphs.len() || glyphs[second_sep].byte != b' ' {
            continue;
        }
        let mut height_start = second_sep + 1;
        if height_start >= glyphs.len() || !glyphs[height_start].byte.is_ascii_digit() {
            continue;
        }
        while height_start < glyphs.len() && glyphs[height_start].byte.is_ascii_digit() {
            height_start += 1;
        }
        if height_start != glyphs.len() {
            continue;
        }
        let width_raw = glyphs[split + 1..second_sep]
            .iter()
            .map(|glyph| glyph.byte)
            .collect::<Vec<u8>>();
        let height_raw = glyphs[second_sep + 1..height_start]
            .iter()
            .map(|glyph| glyph.byte)
            .collect::<Vec<u8>>();
        let width_mpt = std::str::from_utf8(&width_raw).ok()?.parse::<u32>().ok()?;
        let height_mpt = std::str::from_utf8(&height_raw).ok()?.parse::<u32>().ok()?;
        if width_mpt == 0 || height_mpt == 0 {
            return None;
        }
        width_pt = (width_mpt as f32) / 1000.0;
        height_pt = (height_mpt as f32) / 1000.0;
        if width_pt <= 0.0
            || height_pt <= 0.0
            || width_pt > MAX_FIGURE_PLACEHOLDER_WIDTH_PT_V0
            || height_pt > MAX_FIGURE_PLACEHOLDER_HEIGHT_PT_V0
        {
            return None;
        }
        path_end = split;
        parsed_size_suffix = true;
        break;
    }

    let mut image_path = Vec::<u8>::new();
    for glyph in &glyphs[cursor..path_end] {
        if !is_safe_figure_image_path_byte_v0(glyph.byte) {
            return None;
        }
        image_path.push(glyph.byte);
    }
    if image_path.is_empty() {
        return None;
    }
    if !parsed_size_suffix && path_end != glyphs.len() {
        return None;
    }
    Some(FigureImageMetadataV0 {
        image_path,
        width_pt,
        height_pt,
    })
}

fn placeholder_segments_v0(image_path: Option<&[u8]>) -> Vec<PdfStyledSegmentV0> {
    let mut placeholder_bytes = Vec::<u8>::new();
    if let Some(path) = image_path {
        placeholder_bytes.extend_from_slice(b"[ Figure placeholder: ");
        placeholder_bytes.extend_from_slice(path);
        placeholder_bytes.extend_from_slice(b" ]");
    } else {
        placeholder_bytes.extend_from_slice(FIGURE_PLACEHOLDER_LINE_V0);
    }
    let glyphs: Vec<GlyphPlanV0> = placeholder_bytes
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

