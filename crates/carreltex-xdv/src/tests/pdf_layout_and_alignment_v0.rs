#[test]
fn pdf_renderer_keeps_multi_space_line_unwrapped_under_width_limit_v0() {
    let layout =
        plan_layout_width_v0(b"A     B", 65_536, 786_432, 300_000, 200).expect("layout plan");
    assert_eq!(layout.pages.len(), 1);
    assert_eq!(layout.pages[0].lines.len(), 1);

    let xdv = write_dvi_v2_text_page_from_layout_v0(&layout, 786_432).expect("xdv bytes");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    assert!(pdf
        .windows(b"(A     B) Tj".len())
        .any(|w| w == b"(A     B) Tj"));
}

fn max_tm_gap_pt_for_line_containing_v0(pdf: &[u8], needle: &str) -> Option<f32> {
    let text = String::from_utf8_lossy(pdf);
    for line in text.lines() {
        if !line.contains(needle) {
            continue;
        }
        let mut xs = Vec::<f32>::new();
        let fields = line.split_whitespace().collect::<Vec<_>>();
        let mut index = 0usize;
        while index + 6 < fields.len() {
            if fields[index] == "1"
                && fields[index + 1] == "0"
                && fields[index + 2] == "0"
                && fields[index + 3] == "1"
                && fields[index + 6] == "Tm"
            {
                let x_pt = fields[index + 4].parse::<f32>().ok()?;
                xs.push(x_pt);
                index += 7;
                continue;
            }
            index += 1;
        }
        if xs.len() < 2 {
            return Some(0.0);
        }
        let mut max_gap = 0.0f32;
        for pair in xs.windows(2) {
            let gap = pair[1] - pair[0];
            if gap > max_gap {
                max_gap = gap;
            }
        }
        return Some(max_gap);
    }
    None
}

fn tm_line_start_xs_for_segment_text_v0(pdf: &[u8], segment_text: &str) -> Vec<f32> {
    let target_token = format!("({segment_text})");
    let text = String::from_utf8_lossy(pdf);
    let mut xs = Vec::<f32>::new();
    for line in text.lines() {
        if !line.contains(&target_token) || !line.contains(" Tm ") {
            continue;
        }
        let fields = line.split_whitespace().collect::<Vec<_>>();
        let mut index = 0usize;
        while index + 6 < fields.len() {
            let is_tm = fields[index] == "1"
                && fields[index + 1] == "0"
                && fields[index + 2] == "0"
                && fields[index + 3] == "1"
                && fields[index + 6] == "Tm";
            if !is_tm {
                index += 1;
                continue;
            }
            if let Ok(x_pt) = fields[index + 4].parse::<f32>() {
                xs.push(x_pt);
            }
            break;
        }
    }
    xs
}

fn segment_width_pt_v0(segment: &[u8]) -> f32 {
    let layout = plan_layout_width_v0(segment, 65_536, 786_432, 10_000_000, 16)
        .expect("segment layout should parse");
    let line = &layout.pages[0].lines[0];
    line.width_sp as f32 / 65_536.0
}

fn scaled_segment_width_pt_v0(segment: &[u8], scale_percent: u8) -> f32 {
    segment_width_pt_v0(segment) * (scale_percent as f32 / 100.0)
}

fn decode_pdf_text_segments_from_line_v0(line: &str) -> Option<String> {
    let mut out = Vec::<u8>::new();
    let bytes = line.as_bytes();
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] != b'(' {
            index += 1;
            continue;
        }
        index += 1;
        while index < bytes.len() {
            let byte = bytes[index];
            if byte == b'\\' {
                index += 1;
                if index < bytes.len() {
                    out.push(bytes[index]);
                    index += 1;
                }
                continue;
            }
            if byte == b')' {
                index += 1;
                break;
            }
            out.push(byte);
            index += 1;
        }
    }
    String::from_utf8(out).ok()
}

fn rendered_text_for_line_containing_segment_v0(pdf: &[u8], segment_text: &str) -> Option<String> {
    let target_token = format!("({segment_text})");
    let text = String::from_utf8_lossy(pdf);
    for line in text.lines() {
        if !line.contains(&target_token) || !line.contains(" Tj ") {
            continue;
        }
        return decode_pdf_text_segments_from_line_v0(line);
    }
    None
}

fn rendered_text_for_line_containing_needle_v0(pdf: &[u8], needle: &str) -> Option<String> {
    let text = String::from_utf8_lossy(pdf);
    for line in text.lines() {
        if !line.contains(needle) || !line.contains(" Tj ") {
            continue;
        }
        return decode_pdf_text_segments_from_line_v0(line);
    }
    None
}

fn rendered_text_for_first_text_line_v0(pdf: &[u8]) -> Option<String> {
    let text = String::from_utf8_lossy(pdf);
    for line in text.lines() {
        if line.contains(" Tj ") {
            return decode_pdf_text_segments_from_line_v0(line);
        }
    }
    None
}

fn parse_tm_positions_in_line_v0(line: &str) -> Vec<(usize, f32, f32)> {
    let mut positions = Vec::<(usize, f32, f32)>::new();
    let mut search_from = 0usize;
    while let Some(rel_start) = line[search_from..].find("1 0 0 1 ") {
        let start = search_from + rel_start + "1 0 0 1 ".len();
        let x_end = match line[start..].find(' ') {
            Some(value) => start + value,
            None => break,
        };
        let x_pt = match line[start..x_end].parse::<f32>() {
            Ok(value) => value,
            Err(_) => break,
        };
        let y_start = x_end + 1;
        let y_end = match line[y_start..].find(' ') {
            Some(value) => y_start + value,
            None => break,
        };
        let y_pt = match line[y_start..y_end].parse::<f32>() {
            Ok(value) => value,
            Err(_) => break,
        };
        if !line[y_end..].starts_with(" Tm") {
            break;
        }
        let tm_end = y_end + " Tm".len();
        positions.push((tm_end, x_pt, y_pt));
        search_from = tm_end;
    }
    positions
}

fn tm_x_for_segment_substring_v0(
    pdf: &[u8],
    line_needle: &str,
    segment_substring: &str,
) -> Option<f32> {
    let text = String::from_utf8_lossy(pdf);
    for line in text.lines() {
        if !line.contains(line_needle) || !line.contains(segment_substring) {
            continue;
        }
        let target_index = line.find(segment_substring)?;
        let positions = parse_tm_positions_in_line_v0(line);
        let mut best_x = None::<f32>;
        for (tm_end, x_pt, _) in positions {
            if tm_end <= target_index {
                best_x = Some(x_pt);
            }
        }
        return best_x;
    }
    None
}

fn tm_count_for_line_containing_v0(pdf: &[u8], needle: &str) -> usize {
    let text = String::from_utf8_lossy(pdf);
    for line in text.lines() {
        if !line.contains(needle) {
            continue;
        }
        let fields = line.split_whitespace().collect::<Vec<_>>();
        let mut count = 0usize;
        let mut index = 0usize;
        while index + 6 < fields.len() {
            if fields[index] == "1"
                && fields[index + 1] == "0"
                && fields[index + 2] == "0"
                && fields[index + 3] == "1"
                && fields[index + 6] == "Tm"
            {
                count += 1;
                index += 7;
                continue;
            }
            index += 1;
        }
        return count;
    }
    0
}

fn tm_x_for_line_containing_text_v0(pdf: &[u8], needle: &str) -> Option<f32> {
    let text = String::from_utf8_lossy(pdf);
    for line in text.lines() {
        if !line.contains(needle) || !line.contains(" Tm ") {
            continue;
        }
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() < 7 || fields[6] != "Tm" {
            continue;
        }
        if let Ok(x_pt) = fields[4].parse::<f32>() {
            return Some(x_pt);
        }
    }
    None
}

fn tm_position_for_line_containing_text_v0(pdf: &[u8], needle: &str) -> Option<(f32, f32)> {
    let text = String::from_utf8_lossy(pdf);
    for line in text.lines() {
        if !line.contains(needle) || !line.contains(" Tm ") {
            continue;
        }
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() < 7 || fields[6] != "Tm" {
            continue;
        }
        let x_pt = fields[4].parse::<f32>().ok()?;
        let y_pt = fields[5].parse::<f32>().ok()?;
        return Some((x_pt, y_pt));
    }
    None
}

fn tf_sizes_for_line_containing_text_v0(pdf: &[u8], needle: &str) -> Vec<f32> {
    let text = String::from_utf8_lossy(pdf);
    for line in text.lines() {
        if !line.contains(needle) {
            continue;
        }
        let fields = line.split_whitespace().collect::<Vec<_>>();
        let mut sizes = Vec::<f32>::new();
        let mut index = 0usize;
        while index + 1 < fields.len() {
            if fields[index + 1] == "Tf" {
                if let Ok(size_pt) = fields[index].parse::<f32>() {
                    sizes.push(size_pt);
                }
            }
            index += 1;
        }
        return sizes;
    }
    Vec::new()
}

fn tm_xs_for_segment_text_v0(pdf: &[u8], segment_text: &str) -> Vec<f32> {
    let target_token = format!("({segment_text})");
    let text = String::from_utf8_lossy(pdf);
    let mut xs = Vec::<f32>::new();
    for line in text.lines() {
        if !line.contains(&target_token) || !line.contains(" Tm ") {
            continue;
        }
        let fields = line.split_whitespace().collect::<Vec<_>>();
        let mut index = 0usize;
        while index + 6 < fields.len() {
            let is_tm = fields[index] == "1"
                && fields[index + 1] == "0"
                && fields[index + 2] == "0"
                && fields[index + 3] == "1"
                && fields[index + 6] == "Tm";
            if !is_tm {
                index += 1;
                continue;
            }
            let Some(x_pt) = fields[index + 4].parse::<f32>().ok() else {
                index += 7;
                continue;
            };
            let mut cursor = index + 7;
            let mut matched = false;
            while cursor < fields.len() {
                if fields[cursor] == "1"
                    && cursor + 6 < fields.len()
                    && fields[cursor + 1] == "0"
                    && fields[cursor + 2] == "0"
                    && fields[cursor + 3] == "1"
                    && fields[cursor + 6] == "Tm"
                {
                    break;
                }
                if fields[cursor] == target_token {
                    matched = true;
                    break;
                }
                cursor += 1;
            }
            if matched {
                xs.push(x_pt);
            }
            index += 7;
        }
    }
    xs
}

fn tm_position_for_segment_substring_v0(pdf: &[u8], needle: &str) -> Option<(f32, f32)> {
    let text = String::from_utf8_lossy(pdf);
    for line in text.lines() {
        if !line.contains(needle) || !line.contains(" Tm ") {
            continue;
        }
        let fields = line.split_whitespace().collect::<Vec<_>>();
        let mut index = 0usize;
        while index + 6 < fields.len() {
            let is_tm = fields[index] == "1"
                && fields[index + 1] == "0"
                && fields[index + 2] == "0"
                && fields[index + 3] == "1"
                && fields[index + 6] == "Tm";
            if !is_tm {
                index += 1;
                continue;
            }
            let x_pt = fields[index + 4].parse::<f32>().ok()?;
            let y_pt = fields[index + 5].parse::<f32>().ok()?;
            let mut cursor = index + 7;
            while cursor < fields.len() {
                if cursor + 6 < fields.len()
                    && fields[cursor] == "1"
                    && fields[cursor + 1] == "0"
                    && fields[cursor + 2] == "0"
                    && fields[cursor + 3] == "1"
                    && fields[cursor + 6] == "Tm"
                {
                    break;
                }
                if fields[cursor].contains(needle) {
                    return Some((x_pt, y_pt));
                }
                cursor += 1;
            }
            index += 7;
        }
    }
    None
}

fn parse_first_link_rect_v0(pdf: &[u8]) -> Option<[f32; 4]> {
    let text = String::from_utf8_lossy(pdf);
    for line in text.lines() {
        if !line.contains("/Subtype /Link") || !line.contains("/Rect [") {
            continue;
        }
        let rect_start = line.find("/Rect [")?;
        let rect_body_start = rect_start + "/Rect [".len();
        let rect_body_end = line[rect_body_start..].find(']')?;
        let rect_text = &line[rect_body_start..rect_body_start + rect_body_end];
        let values: Vec<f32> = rect_text
            .split_whitespace()
            .filter_map(|value| value.parse::<f32>().ok())
            .collect();
        if values.len() != 4 {
            return None;
        }
        return Some([values[0], values[1], values[2], values[3]]);
    }
    None
}

fn count_pdf_page_objects_v0(pdf: &[u8]) -> usize {
    String::from_utf8_lossy(pdf)
        .matches("/Type /Page /Parent")
        .count()
}

fn parse_pdf_object_body_v0(pdf: &[u8], id: u32) -> Option<String> {
    let text = String::from_utf8_lossy(pdf);
    let start_token = format!("{id} 0 obj\n");
    let start = text.find(&start_token)? + start_token.len();
    let end = text[start..].find("\nendobj\n")? + start;
    Some(text[start..end].to_string())
}

fn parse_pdf_ref_ids_v0(body: &str, key: &str) -> Vec<u32> {
    let marker = format!("{key} [");
    let Some(start) = body.find(&marker) else {
        return Vec::new();
    };
    let values_start = start + marker.len();
    let Some(values_end_rel) = body[values_start..].find(']') else {
        return Vec::new();
    };
    body[values_start..values_start + values_end_rel]
        .split_whitespace()
        .collect::<Vec<_>>()
        .chunks(3)
        .filter_map(|chunk| match chunk {
            [id, "0", "R"] => id.parse::<u32>().ok(),
            _ => None,
        })
        .collect()
}

fn parse_pdf_single_ref_id_v0(body: &str, key: &str) -> Option<u32> {
    let marker = format!("{key} ");
    let start = body.find(&marker)? + marker.len();
    let fields = body[start..].split_whitespace().collect::<Vec<_>>();
    if fields.len() < 3 || fields[1] != "0" || fields[2] != "R" {
        return None;
    }
    fields[0].parse::<u32>().ok()
}

fn parse_pdf_outline_count_v0(body: &str) -> Option<i32> {
    let marker = "/Count ";
    let start = body.find(marker)? + marker.len();
    let token = body[start..].split_whitespace().next()?;
    token.parse::<i32>().ok()
}

fn collect_outline_item_ids_depth_first_v0(pdf: &[u8], outline_root_id: u32) -> Option<Vec<u32>> {
    let root_body = parse_pdf_object_body_v0(pdf, outline_root_id)?;
    let Some(root_first_id) = parse_pdf_single_ref_id_v0(&root_body, "/First") else {
        return Some(Vec::new());
    };
    let mut out = Vec::<u32>::new();
    let mut seen = BTreeSet::<u32>::new();
    let mut stack = vec![root_first_id];
    while let Some(item_id) = stack.pop() {
        if !seen.insert(item_id) {
            return None;
        }
        out.push(item_id);
        let body = parse_pdf_object_body_v0(pdf, item_id)?;
        if let Some(next_id) = parse_pdf_single_ref_id_v0(&body, "/Next") {
            stack.push(next_id);
        }
        if let Some(first_child_id) = parse_pdf_single_ref_id_v0(&body, "/First") {
            stack.push(first_child_id);
        }
    }
    Some(out)
}

fn parse_pdf_annotation_action_id_v0(body: &str) -> Option<u32> {
    let marker = "/A ";
    let start = body.find(marker)? + marker.len();
    let fields = body[start..].split_whitespace().collect::<Vec<_>>();
    if fields.len() < 3 || fields[1] != "0" || fields[2] != "R" {
        return None;
    }
    fields[0].parse::<u32>().ok()
}

fn parse_pdf_annotation_dest_page_id_v0(body: &str) -> Option<u32> {
    let marker = "/Dest [";
    let start = body.find(marker)? + marker.len();
    let fields = body[start..].split_whitespace().collect::<Vec<_>>();
    if fields.len() < 3 || fields[1] != "0" || fields[2] != "R" {
        return None;
    }
    fields[0].parse::<u32>().ok()
}

fn parse_pdf_annotation_dest_xyz_v0(body: &str) -> Option<(u32, f32, f32)> {
    let marker = "/Dest [";
    let start = body.find(marker)? + marker.len();
    let end = body[start..].find(']')? + start;
    let fields = body[start..end].split_whitespace().collect::<Vec<_>>();
    if fields.len() < 6 || fields[1] != "0" || fields[2] != "R" || fields[3] != "/XYZ" {
        return None;
    }
    let page_id = fields[0].parse::<u32>().ok()?;
    let x_pt = fields[4].parse::<f32>().ok()?;
    let y_token = fields[5].trim_end_matches(']');
    let y_pt = y_token.parse::<f32>().ok()?;
    Some((page_id, x_pt, y_pt))
}

fn parse_pdf_action_uri_v0(body: &str) -> Option<String> {
    let marker = "/URI (";
    let start = body.find(marker)? + marker.len();
    let end = body[start..].find(')')? + start;
    Some(body[start..end].to_string())
}

fn parse_pdf_annotation_rect_v0(body: &str) -> Option<[f32; 4]> {
    let marker = "/Rect [";
    let start = body.find(marker)? + marker.len();
    let end = body[start..].find(']')? + start;
    let values = body[start..end]
        .split_whitespace()
        .filter_map(|value| value.parse::<f32>().ok())
        .collect::<Vec<_>>();
    if values.len() != 4 {
        return None;
    }
    Some([values[0], values[1], values[2], values[3]])
}

fn tm_position_for_line_containing_text_in_body_v0(body: &str, needle: &str) -> Option<(f32, f32)> {
    for line in body.lines() {
        if !line.contains(needle) || !line.contains(" Tm ") {
            continue;
        }
        let fields = line.split_whitespace().collect::<Vec<_>>();
        let mut index = 0usize;
        while index + 6 < fields.len() {
            let is_tm = fields[index] == "1"
                && fields[index + 1] == "0"
                && fields[index + 2] == "0"
                && fields[index + 3] == "1"
                && fields[index + 6] == "Tm";
            if !is_tm {
                index += 1;
                continue;
            }
            let x_pt = fields[index + 4].parse::<f32>().ok()?;
            let y_pt = fields[index + 5].parse::<f32>().ok()?;
            let mut cursor = index + 7;
            while cursor < fields.len() {
                if cursor + 6 < fields.len()
                    && fields[cursor] == "1"
                    && fields[cursor + 1] == "0"
                    && fields[cursor + 2] == "0"
                    && fields[cursor + 3] == "1"
                    && fields[cursor + 6] == "Tm"
                {
                    break;
                }
                if fields[cursor].contains(needle) {
                    return Some((x_pt, y_pt));
                }
                cursor += 1;
            }
            index += 7;
        }
    }
    None
}

fn expected_center_x_pt_v0(width_sp: u32) -> f32 {
    let width_pt = (width_sp as f32) / 65_536.0;
    ((612.0 - width_pt) * 0.5).clamp(72.0, 612.0 - 72.0)
}

fn expected_right_x_pt_v0(width_sp: u32) -> f32 {
    let width_pt = (width_sp as f32) / 65_536.0;
    (612.0 - 72.0 - width_pt).max(72.0)
}

fn width_sp_for_prefixed_rendered_line_v0(line: &LinePlanV0, prefix: [u8; 2]) -> Option<u32> {
    if line.glyphs.len() < 2 {
        return None;
    }
    if line.glyphs[0].byte != prefix[0] || line.glyphs[1].byte != prefix[1] {
        return None;
    }
    let mut width_sp = 0u32;
    for glyph in &line.glyphs[2..] {
        let advance = u32::try_from(glyph.advance_sp).ok()?;
        width_sp = width_sp.checked_add(advance)?;
    }
    Some(width_sp)
}

fn layout_line_width_for_exact_bytes_v0(
    layout: &super::LayoutPlanV0,
    target: &[u8],
) -> Option<u32> {
    for page in &layout.pages {
        for line in &page.lines {
            let bytes: Vec<u8> = line.glyphs.iter().map(|glyph| glyph.byte).collect();
            if bytes == target {
                return Some(line.width_sp);
            }
        }
    }
    None
}

fn layout_render_width_for_substring_v0(
    layout: &super::LayoutPlanV0,
    needle: &[u8],
) -> Option<u32> {
    for page in &layout.pages {
        for line in &page.lines {
            let bytes: Vec<u8> = line.glyphs.iter().map(|glyph| glyph.byte).collect();
            if !bytes.windows(needle.len()).any(|window| window == needle) {
                continue;
            }
            if let Some(width_sp) = width_sp_for_prefixed_rendered_line_v0(line, [b'^', b' ']) {
                return Some(width_sp);
            }
            if let Some(width_sp) = width_sp_for_prefixed_rendered_line_v0(line, [b'|', b' ']) {
                return Some(width_sp);
            }
            return Some(line.width_sp);
        }
    }
    None
}

#[test]
fn pdf_renderer_caps_segment_tm_gap_for_styled_line_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"Styled [emphasis] and {bold} run.")
        .expect("writer should accept styled text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let max_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "Styled")
        .expect("pdf should include styled line");
    assert!(
        max_tm_gap <= 12.0,
        "styled line tm gap should be capped, got {max_tm_gap}"
    );
}

#[test]
fn pdf_renderer_inline_wrapper_spacing_invariants_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"word[mid]word word [lead] trail,{bold}!")
        .expect("writer should accept styled text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let rendered = rendered_text_for_line_containing_segment_v0(&pdf, "word")
        .expect("styled line should decode");
    assert_eq!(rendered, "wordmidword word lead trail,bold!");

    let word_x = tm_xs_for_segment_text_v0(&pdf, "word")[0];
    let mid_x = tm_xs_for_segment_text_v0(&pdf, "mid")[0];
    let trailing_word_x =
        tm_x_for_segment_substring_v0(&pdf, "(word)", "(word word )").expect("word word segment x");
    let lead_x = tm_xs_for_segment_text_v0(&pdf, "lead")[0];
    let trail_x =
        tm_x_for_segment_substring_v0(&pdf, "(word)", "( trail,)").expect("trail segment x");
    let bold_x = tm_xs_for_segment_text_v0(&pdf, "bold")[0];

    let epsilon_pt = 0.01f32;
    assert!(
        ((mid_x - word_x) - segment_width_pt_v0(b"word")).abs() <= epsilon_pt,
        "word->mid advance mismatch: word_x={word_x}, mid_x={mid_x}"
    );
    assert!(
        ((trailing_word_x - mid_x) - segment_width_pt_v0(b"mid")).abs() <= epsilon_pt,
        "mid->trailing advance mismatch: mid_x={mid_x}, trailing_word_x={trailing_word_x}"
    );
    assert!(
        ((trail_x - lead_x) - segment_width_pt_v0(b"lead")).abs() <= epsilon_pt,
        "lead->trail advance mismatch: lead_x={lead_x}, trail_x={trail_x}"
    );
    assert!(
        ((bold_x - trail_x) - segment_width_pt_v0(b" trail,")).abs() <= epsilon_pt,
        "trail->bold advance mismatch: trail_x={trail_x}, bold_x={bold_x}"
    );
}

#[test]
fn pdf_renderer_punctuation_adjacent_wrapper_gap_invariants_v0() {
    let xdv = write_dvi_v2_text_page_v0(
        b"word[mid],word word,[mid]word word{mid}. (a[mid]b) lead, [trail]",
    )
    .expect("writer should accept punctuation-adjacent styled text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let rendered = rendered_text_for_line_containing_segment_v0(&pdf, "word")
        .expect("punctuation-adjacent line should decode");
    assert_eq!(
        rendered,
        "wordmid,word word,midword wordmid. (amidb) lead, trail"
    );

    let word_x = tm_xs_for_segment_text_v0(&pdf, "word")[0];
    let mid_x = tm_xs_for_segment_text_v0(&pdf, "mid")[0];
    let comma_word_x = tm_x_for_segment_substring_v0(&pdf, "(word)", "(,word word,)")
        .expect("comma-word segment x");
    let epsilon_pt = 0.02f32;
    assert!(
        ((mid_x - word_x) - segment_width_pt_v0(b"word")).abs() <= epsilon_pt,
        "word->mid punctuation boundary drifted: word_x={word_x}, mid_x={mid_x}"
    );
    assert!(
        ((comma_word_x - mid_x) - segment_width_pt_v0(b"mid")).abs() <= epsilon_pt,
        "mid->comma segment boundary drifted: mid_x={mid_x}, comma_word_x={comma_word_x}"
    );
}

#[test]
fn pdf_renderer_wrapper_punctuation_patterns_are_stable_v0() {
    let cases: [(&[u8], &str); 6] = [
        (b"alpha[beta],gamma", "alphabeta,gamma"),
        (b"alpha,[beta]gamma", "alpha,betagamma"),
        (b"(alpha[beta]gamma)", "(alphabetagamma)"),
        (b"alpha{beta}. gamma", "alphabeta. gamma"),
        (b"{lead}, trail", "lead, trail"),
        (b"lead, [trail]", "lead, trail"),
    ];

    for (input, expected_rendered) in cases {
        let xdv = write_dvi_v2_text_page_v0(input).expect("writer should accept punctuation case");
        let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
        let rendered =
            rendered_text_for_first_text_line_v0(&pdf).expect("punctuation case should decode");
        assert!(
            rendered == expected_rendered,
            "rendered punctuation mismatch for input {:?}: got {:?}, want {:?}",
            String::from_utf8_lossy(input),
            rendered,
            expected_rendered,
        );
    }
}

#[test]
fn pdf_renderer_wrapper_punctuation_segment_positions_progress_monotonically_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"A[mid],B C,[mid]D E{mid}. F")
        .expect("writer should accept wrapper punctuation sequence");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let rendered = rendered_text_for_line_containing_segment_v0(&pdf, "A")
        .expect("wrapper punctuation line should decode");
    assert_eq!(rendered, "Amid,B C,midD Emid. F");

    let a_x = tm_xs_for_segment_text_v0(&pdf, "A")[0];
    let mid_x = tm_xs_for_segment_text_v0(&pdf, "mid")[0];
    let trailing_x = tm_x_for_segment_substring_v0(&pdf, "(A)", "(,B C,)")
        .expect("trailing punctuation segment x");
    let mid_two_x = tm_xs_for_segment_text_v0(&pdf, "mid")[1];
    assert!(a_x < mid_x && mid_x < trailing_x && trailing_x < mid_two_x);
    let epsilon_pt = 0.02f32;
    assert!(
        ((mid_x - a_x) - segment_width_pt_v0(b"A")).abs() <= epsilon_pt,
        "A->mid boundary drifted: a_x={a_x}, mid_x={mid_x}"
    );
    assert!(
        ((trailing_x - mid_x) - segment_width_pt_v0(b"mid")).abs() <= epsilon_pt,
        "mid->trail boundary drifted: mid_x={mid_x}, trailing_x={trailing_x}"
    );
}

#[test]
fn pdf_renderer_body_wrap_balances_lines_and_preserves_styled_punctuation_seams_v12() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"P1START aa [mid],bb cc dd P1WRAP.",
        65_536,
        786_432,
        24,
    )
    .expect("writer should accept wrapped styled paragraph");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let (start_x, start_y) =
        tm_position_for_segment_substring_v0(&pdf, "P1START").expect("paragraph start");
    let (wrap_x, wrap_y) =
        tm_position_for_line_containing_text_v0(&pdf, "P1WRAP").expect("paragraph wrap line");
    let epsilon_pt = 0.05f32;
    assert!(
        (start_x - 72.0).abs() <= epsilon_pt && (wrap_x - 72.0).abs() <= epsilon_pt,
        "body paragraph wrapped continuation should stay in body column: start_x={start_x}, wrap_x={wrap_x}"
    );
    assert!(
        (start_y - wrap_y - 13.0).abs() <= epsilon_pt,
        "wrapped body continuation rhythm should be tightened and stable: start_y={start_y}, wrap_y={wrap_y}"
    );

    let rendered_pdf_text = String::from_utf8_lossy(&pdf);
    assert!(
        !rendered_pdf_text.contains("mid ,") && !rendered_pdf_text.contains(", gamma"),
        "styled punctuation seams in wrapped body paragraphs should avoid spacing artifacts"
    );
}

#[test]
fn pdf_renderer_body_paragraph_applies_style_scaling_for_styled_seams_v13() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\nBody prose with [ITALICV13] seam and {BOLDV13} seam.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept body prose");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    let italic_line = pdf_text
        .lines()
        .find(|line| line.contains("(ITALICV13) Tj"))
        .expect("italic body segment should render");
    let bold_line = pdf_text
        .lines()
        .find(|line| line.contains("(BOLDV13) Tj"))
        .expect("bold body segment should render");

    assert!(
        italic_line.contains("97 Tz") && italic_line.contains("(ITALICV13) Tj 100 Tz"),
        "body prose italic segment should use v13 seam-scaling compensation"
    );
    assert!(
        bold_line.contains("95 Tz") && bold_line.contains("(BOLDV13) Tj 100 Tz"),
        "body prose bold segment should use v13 seam-scaling compensation"
    );
}

#[test]
fn pdf_renderer_centered_lines_do_not_use_prose_seam_scaling_v13() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\n^ Center [ITALICCENTERV13] text.\n\n> Quote {BOLDQUOTEV13} line.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept non-paragraph blocks");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    let centered_line = pdf_text
        .lines()
        .find(|line| line.contains("(ITALICCENTERV13) Tj"))
        .expect("centered styled segment should render");

    assert!(
        !centered_line.contains("97 Tz"),
        "centered non-paragraph line should not use prose seam-scaling compensation"
    );
}

#[test]
fn pdf_renderer_body_paragraph_uses_inline_math_seam_scaling_profile_v15() {
    let demo_text =
        b"Title\nAuthor\n2026-03-05\n\nBody [ITALICMATHV15] MATH seam and {BOLDMATHV15} MATH seam.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept inline math seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    let italic_line = pdf_text
        .lines()
        .find(|line| line.contains("(ITALICMATHV15) Tj"))
        .expect("italic inline-math-adjacent segment should render");
    let bold_line = pdf_text
        .lines()
        .find(|line| line.contains("(BOLDMATHV15) Tj"))
        .expect("bold inline-math-adjacent segment should render");
    assert!(
        italic_line.contains("99 Tz"),
        "inline-math-adjacent italic segment should use v15 seam scaling"
    );
    assert!(
        bold_line.contains("97 Tz"),
        "inline-math-adjacent bold segment should use v15 seam scaling"
    );
}

#[test]
fn pdf_renderer_wrap_avoids_inline_math_placeholder_at_line_start_v15() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"PSTART alpha beta gamma MATH, delta epsilon zeta eta WRAPTOKEN",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped inline-math paragraph");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let rendered_math_line =
        rendered_text_for_line_containing_needle_v0(&pdf, "MATH").expect("inline math placeholder should render");
    let (start_x, start_y) =
        tm_position_for_segment_substring_v0(&pdf, "PSTART").expect("paragraph start");
    let (wrap_x, wrap_y) =
        tm_position_for_line_containing_text_v0(&pdf, "WRAPTOKEN").expect("wrap line");
    let epsilon_pt = 0.05f32;
    assert!(
        (start_x - 72.0).abs() <= epsilon_pt && (wrap_x - 72.0).abs() <= epsilon_pt,
        "wrapped body paragraph columns should stay stable: start_x={start_x}, wrap_x={wrap_x}"
    );
    assert!(
        start_y > wrap_y,
        "wrapped inline-math paragraph line should render below paragraph start: start_y={start_y}, wrap_y={wrap_y}"
    );
    let line_steps = ((start_y - wrap_y) / 13.0).round();
    assert!(
        line_steps >= 1.0 && (start_y - wrap_y - (line_steps * 13.0)).abs() <= epsilon_pt,
        "wrapped inline-math paragraph rhythm should stay on stable 13pt steps: start_y={start_y}, wrap_y={wrap_y}, line_steps={line_steps}"
    );
    assert!(
        rendered_math_line.contains("MATH,") && !rendered_math_line.contains("MATH ,"),
        "inline math placeholder should keep punctuation-adjacent seam spacing stable under wrapping: rendered_math_line={rendered_math_line:?}"
    );
}

#[test]
fn pdf_renderer_footnote_styled_seams_track_scaled_advances_v26() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\nBody prose through <{VISIBLELINKV26}>,right beside punctuation.^1\n\n!f 1 Footnote text with [INLINEFOOTNOTEV26].\n!u 1 https://example.com/v26";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept v26 seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let footnote_line = String::from_utf8_lossy(&pdf)
        .lines()
        .find(|line| line.contains("(INLINEFOOTNOTEV26) Tj"))
        .expect("footnote line should render")
        .to_string();
    assert!(
        footnote_line.contains("97 Tz") && footnote_line.contains("(INLINEFOOTNOTEV26) Tj 100 Tz"),
        "footnote styled segment should use v26 seam compensation"
    );

    let footnote_italic_x = tm_x_for_segment_substring_v0(&pdf, "(1 Footnote text with ", "(INLINEFOOTNOTEV26)")
        .expect("footnote italic x");
    let footnote_period_x =
        tm_x_for_segment_substring_v0(&pdf, "(1 Footnote text with ", "(.)").expect("footnote period x");
    let expected_footnote_italic_width = segment_width_pt_v0(b"INLINEFOOTNOTEV26") * 0.97;
    assert!(
        ((footnote_period_x - footnote_italic_x) - expected_footnote_italic_width).abs() <= 0.3,
        "footnote styled seam should advance on compensated rendered width: italic_x={footnote_italic_x}, period_x={footnote_period_x}, expected={expected_footnote_italic_width}"
    );
}

#[test]
fn pdf_renderer_wrapped_body_paragraph_styled_seams_track_scaled_advances_v27() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\nWRAPSTART alpha alpha alpha alpha alpha alpha alpha alpha <{BODYLINKWRAPV27}> and [ITALICWRAPV27],right beside punctuation with {BOLDWRAPV27} seam before WRAPTOKENV27.\n\n!u 1 https://example.com/v27";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept wrapped v27 seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    let link_line = pdf_text
        .lines()
        .find(|line| line.contains("(BODYLINKWRAPV27) Tj"))
        .expect("wrapped body link line should render");
    let italic_line = pdf_text
        .lines()
        .find(|line| line.contains("(ITALICWRAPV27) Tj"))
        .expect("wrapped body italic line should render");
    let bold_line = pdf_text
        .lines()
        .find(|line| line.contains("(BOLDWRAPV27) Tj"))
        .expect("wrapped body bold line should render");

    assert!(
        link_line.contains("95 Tz") && link_line.contains("(BODYLINKWRAPV27) Tj 100 Tz"),
        "wrapped body link segment should use v27 seam compensation"
    );
    assert!(
        italic_line.contains("97 Tz") && italic_line.contains("(ITALICWRAPV27) Tj 100 Tz"),
        "wrapped body italic segment should use v27 seam compensation"
    );
    assert!(
        bold_line.contains("95 Tz") && bold_line.contains("(BOLDWRAPV27) Tj 100 Tz"),
        "wrapped body bold segment should use v27 seam compensation"
    );
    let (_, wrap_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "WRAPSTART").expect("wrapped paragraph start");
    let (_, wrap_token_y) =
        tm_position_for_segment_substring_v0(&pdf, "WRAPTOKENV27").expect("wrapped paragraph tail");
    assert!(
        wrap_start_y > wrap_token_y,
        "fixture should wrap onto a later body line: wrap_start_y={wrap_start_y}, wrap_token_y={wrap_token_y}"
    );
}

#[test]
fn pdf_renderer_centers_title_block_lines_within_epsilon_v0() {
    let demo_text = b"Centering Accuracy Title\nAlice Bob\n2026-03-05\n\nBody line.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept demo text");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    assert!(!layout.pages.is_empty(), "layout should contain a page");
    assert!(
        layout.pages[0].lines.len() >= 3,
        "layout should contain title block lines"
    );

    let expected_title_x = expected_center_x_pt_v0(layout.pages[0].lines[0].width_sp);
    let expected_author_x = expected_center_x_pt_v0(layout.pages[0].lines[1].width_sp);
    let expected_date_x = expected_center_x_pt_v0(layout.pages[0].lines[2].width_sp);

    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let title_x =
        tm_x_for_line_containing_text_v0(&pdf, "(Centering Accuracy Title)").expect("title line x");
    let author_x = tm_x_for_line_containing_text_v0(&pdf, "(Alice Bob)").expect("author line x");
    let date_x = tm_x_for_line_containing_text_v0(&pdf, "(2026-03-05)").expect("date line x");

    let epsilon_pt = 0.02f32;
    assert!(
        (title_x - expected_title_x).abs() <= epsilon_pt,
        "title x mismatch: actual={title_x}, expected={expected_title_x}"
    );
    assert!(
        (author_x - expected_author_x).abs() <= epsilon_pt,
        "author x mismatch: actual={author_x}, expected={expected_author_x}"
    );
    assert!(
        (date_x - expected_date_x).abs() <= epsilon_pt,
        "date x mismatch: actual={date_x}, expected={expected_date_x}"
    );
}

#[test]
fn pdf_renderer_centers_section_headings_within_epsilon_v0() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\nPrelude paragraph.\n\n{Centered Section Heading}\n\n~ Body after centered heading.";
    let xdv =
        write_dvi_v2_text_page_v0(demo_text).expect("writer should accept centered heading text");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    assert_eq!(layout.pages.len(), 1);
    let heading_line = layout.pages[0]
        .lines
        .iter()
        .find(|line| {
            line.glyphs
                .iter()
                .map(|glyph| glyph.byte)
                .collect::<Vec<_>>()
                == b"{Centered Section Heading}"
        })
        .expect("heading line in layout");
    let expected_heading_x = expected_center_x_pt_v0(heading_line.width_sp);

    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let heading_x = tm_x_for_line_containing_text_v0(&pdf, "(Centered Section Heading)")
        .expect("centered heading position");
    assert!(
        (heading_x - expected_heading_x).abs() <= 0.01,
        "centered heading x mismatch: actual={heading_x}, expected={expected_heading_x}"
    );
    assert!(
        (heading_x - 72.0).abs() > 0.5,
        "heading should not be left-margin aligned: {heading_x}"
    );
}

#[test]
fn pdf_renderer_title_and_heading_centering_per_line_width_v0() {
    let demo_text = b"Centered Title Line\nAlice Bob\n2026-03-05\n\nPrelude paragraph.\n\n{Heading Alpha}\n\n~ Body alpha paragraph.\n\n{Heading Beta}\n\n~ Body beta paragraph.";
    let xdv =
        write_dvi_v2_text_page_v0(demo_text).expect("writer should accept title+heading demo");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    let title_width = layout_line_width_for_exact_bytes_v0(&layout, b"Centered Title Line")
        .expect("title line width");
    let heading_alpha_width = layout_line_width_for_exact_bytes_v0(&layout, b"{Heading Alpha}")
        .expect("heading alpha width");
    let heading_beta_width = layout_line_width_for_exact_bytes_v0(&layout, b"{Heading Beta}")
        .expect("heading beta width");

    let expected_title_x = expected_center_x_pt_v0(title_width);
    let expected_heading_alpha_x = expected_center_x_pt_v0(heading_alpha_width);
    let expected_heading_beta_x = expected_center_x_pt_v0(heading_beta_width);

    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let title_x = tm_x_for_line_containing_text_v0(&pdf, "(Centered Title Line)").expect("title x");
    let heading_alpha_x =
        tm_x_for_line_containing_text_v0(&pdf, "(Heading Alpha)").expect("heading alpha x");
    let heading_beta_x =
        tm_x_for_line_containing_text_v0(&pdf, "(Heading Beta)").expect("heading beta x");

    let epsilon_pt = 0.01f32;
    assert!(
        (title_x - expected_title_x).abs() <= epsilon_pt,
        "title centering mismatch: actual={title_x}, expected={expected_title_x}"
    );
    assert!(
        (heading_alpha_x - expected_heading_alpha_x).abs() <= epsilon_pt,
        "heading alpha centering mismatch: actual={heading_alpha_x}, expected={expected_heading_alpha_x}"
    );
    assert!(
        (heading_beta_x - expected_heading_beta_x).abs() <= epsilon_pt,
        "heading beta centering mismatch: actual={heading_beta_x}, expected={expected_heading_beta_x}"
    );
    assert!(
        (heading_alpha_x - 72.0).abs() > 0.5 && (heading_beta_x - 72.0).abs() > 0.5,
        "heading lines should not be left-margin aligned: alpha={heading_alpha_x}, beta={heading_beta_x}"
    );
}

#[test]
fn pdf_renderer_title_centering_stays_stable_with_inline_style_segments_v1() {
    let demo_text =
        b"Center [Accurate] {Title}\nAlice Bob\n2026-03-05\n\nBody line after styled title.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept styled title demo");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    let title_width = layout_line_width_for_exact_bytes_v0(&layout, b"Center [Accurate] {Title}")
        .expect("styled title width");
    let expected_title_x = expected_center_x_pt_v0(title_width);

    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let title_x = tm_x_for_line_containing_text_v0(&pdf, "Center")
        .expect("styled title first segment x");
    assert!(
        (title_x - expected_title_x).abs() <= 0.01,
        "styled title centering mismatch: actual={title_x}, expected={expected_title_x}"
    );
}

#[test]
fn pdf_renderer_heading_font_hierarchy_invariants_v0() {
    let demo_text = b"Typography Title\nAlice Bob\n2026-03-05\n\n@S {Section Heading}\n\n@s {Subsection Heading}\n\nBody paragraph text.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept heading font demo");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let title_sizes = tf_sizes_for_line_containing_text_v0(&pdf, "(Typography Title)");
    let section_sizes = tf_sizes_for_line_containing_text_v0(&pdf, "(Section Heading)");
    let subsection_sizes = tf_sizes_for_line_containing_text_v0(&pdf, "(Subsection Heading)");
    let body_sizes = tf_sizes_for_line_containing_text_v0(&pdf, "(Body paragraph text.)");

    assert!(!title_sizes.is_empty(), "missing title sizes");
    assert!(!section_sizes.is_empty(), "missing section sizes");
    assert!(!subsection_sizes.is_empty(), "missing subsection sizes");
    assert!(!body_sizes.is_empty(), "missing body sizes");

    let title_size = title_sizes[0];
    let section_size = section_sizes[0];
    let subsection_size = subsection_sizes[0];
    let body_size = body_sizes[0];

    assert!(
        (title_size - 18.0).abs() <= 0.02,
        "title font size mismatch: {title_size}"
    );
    assert!(
        (section_size - 15.5).abs() <= 0.02,
        "section font size mismatch: {section_size}"
    );
    assert!(
        (subsection_size - 13.0).abs() <= 0.02,
        "subsection font size mismatch: {subsection_size}"
    );
    assert!(
        (body_size - 12.0).abs() <= 0.02,
        "body font size mismatch: {body_size}"
    );
    assert!(
        title_size > section_size && section_size > subsection_size && subsection_size > body_size,
        "font hierarchy must be strict: title={title_size}, section={section_size}, subsection={subsection_size}, body={body_size}"
    );
}

#[test]
fn pdf_renderer_paragraph_rhythm_and_noindent_invariants_v0() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\nFirst paragraph line one.\nSecond line same paragraph.\n\nSecond paragraph line.\n\n@S {Heading}\n\n~ After heading noindent line.\n\nIndented paragraph line.";
    let xdv =
        write_dvi_v2_text_page_v0(demo_text).expect("writer should accept paragraph rhythm demo");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (first_x, first_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(First paragraph line one.)")
            .expect("first paragraph line one");
    let (same_para_x, same_para_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Second line same paragraph.)")
            .expect("same paragraph line");
    let (second_para_x, second_para_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Second paragraph line.)")
            .expect("second paragraph line");
    let (heading_x, heading_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Heading)").expect("heading line");
    let (after_heading_x, after_heading_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(After heading noindent line.)")
            .expect("after heading line");
    let (indented_x, indented_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Indented paragraph line.)")
            .expect("indented paragraph line");

    let epsilon_pt = 0.02f32;
    assert!(
        (first_x - 72.0).abs() <= epsilon_pt,
        "first paragraph x mismatch: {first_x}"
    );
    assert!(
        (same_para_x - 72.0).abs() <= epsilon_pt,
        "same paragraph line should stay non-indented: {same_para_x}"
    );
    assert!(
        (second_para_x - 96.0).abs() <= epsilon_pt,
        "second paragraph indent mismatch: {second_para_x}"
    );
    assert!(
        (after_heading_x - 72.0).abs() <= epsilon_pt,
        "first paragraph after heading should noindent: {after_heading_x}"
    );
    assert!(
        (indented_x - 96.0).abs() <= epsilon_pt,
        "paragraph after noindent should restore indent: {indented_x}"
    );
    assert!(
        (first_y - same_para_y - 13.0).abs() <= epsilon_pt,
        "line gap mismatch inside paragraph: first_y={first_y}, same_para_y={same_para_y}"
    );
    assert!(
        (same_para_y - second_para_y - 27.0).abs() <= epsilon_pt,
        "paragraph break gap mismatch: same_para_y={same_para_y}, second_para_y={second_para_y}"
    );
    assert!(
        (second_para_y - heading_y - 24.0).abs() <= epsilon_pt,
        "paragraph->heading gap mismatch: second_para_y={second_para_y}, heading_y={heading_y}"
    );
    assert!(
        (heading_y - after_heading_y - 24.0).abs() <= epsilon_pt,
        "heading->noindent gap mismatch: heading_y={heading_y}, after_heading_y={after_heading_y}"
    );
    assert!(
        (after_heading_y - indented_y - 27.0).abs() <= epsilon_pt,
        "noindent->indented paragraph gap mismatch: after_heading_y={after_heading_y}, indented_y={indented_y}"
    );
    assert!(
        (heading_x - 72.0).abs() > 0.5,
        "heading should be centered: {heading_x}"
    );
}

#[test]
fn pdf_renderer_consecutive_blank_lines_collapse_to_single_rhythm_gap_v1() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\nFirst paragraph line.\n\n\nSecond paragraph line.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept blank-line rhythm demo");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (first_x, first_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(First paragraph line.)")
            .expect("first paragraph line");
    let (second_x, second_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Second paragraph line.)")
            .expect("second paragraph line");

    let epsilon_pt = 0.02f32;
    assert!(
        (first_x - 72.0).abs() <= epsilon_pt,
        "first paragraph should start at body margin: {first_x}"
    );
    assert!(
        (second_x - 96.0).abs() <= epsilon_pt,
        "second paragraph should keep paragraph indent: {second_x}"
    );
    assert!(
        (first_y - second_y - 27.0).abs() <= epsilon_pt,
        "consecutive blank lines should collapse to a single paragraph gap: first_y={first_y}, second_y={second_y}"
    );
}

#[test]
fn pdf_renderer_list_rhythm_and_wrap_indent_invariants_v0() {
    let demo_text = b"\nParagraph before list.\n\n- ITEMONE lead words with deterministic wrapping content to force continuation line token WRAPONE after many repeated words in this same item.\n- ITEMTWO lead words with deterministic wrapping content to force continuation line token WRAPTWO after many repeated words in this same item.\n\nParagraph after list.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept list rhythm demo");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, before_list_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Paragraph before list.)")
            .expect("before list paragraph");
    let (item_one_bullet_x, item_one_y) =
        tm_position_for_segment_substring_v0(&pdf, "(-)").expect("item one bullet position");
    let (item_one_body_x, _) =
        tm_position_for_segment_substring_v0(&pdf, "(ITEMONE").expect("item one body position");
    let (item_one_wrap_x, _) =
        tm_position_for_segment_substring_v0(&pdf, "WRAPONE").expect("item one wrap position");
    let (item_two_body_x, item_two_y) =
        tm_position_for_segment_substring_v0(&pdf, "(ITEMTWO").expect("item two body position");
    let (item_two_wrap_x, _) =
        tm_position_for_segment_substring_v0(&pdf, "WRAPTWO").expect("item two wrap position");
    let (_, after_list_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Paragraph after list.)")
            .expect("after list paragraph");

    let epsilon_pt = 0.02f32;
    assert!(
        (before_list_y - item_one_y - 24.0).abs() <= epsilon_pt,
        "before->list top gap out of range: before_list_y={before_list_y}, item_one_y={item_one_y}"
    );
    assert!(
        (item_two_y - after_list_y).abs() >= 24.0 - epsilon_pt,
        "list->after paragraph gap must be at least one paragraph break: item_two_y={item_two_y}, after_list_y={after_list_y}"
    );
    assert!(
        (item_one_body_x - 96.0).abs() <= epsilon_pt,
        "item body x mismatch: {item_one_body_x}"
    );
    let item_one_marker_gap_pt = item_one_body_x - (item_one_bullet_x + segment_width_pt_v0(b"-"));
    assert!(
        (item_one_marker_gap_pt - 8.0).abs() <= 0.25,
        "item marker/body gap mismatch: marker_gap={item_one_marker_gap_pt}"
    );
    assert!(
        (item_one_wrap_x - item_one_body_x).abs() <= epsilon_pt,
        "item one wrap continuation should keep hanging indent: body={item_one_body_x}, wrap={item_one_wrap_x}"
    );
    assert!(
        (item_two_body_x - 96.0).abs() <= epsilon_pt,
        "item two body x mismatch: {item_two_body_x}"
    );
    assert!(
        (item_two_wrap_x - item_two_body_x).abs() <= epsilon_pt,
        "item two wrap continuation should keep hanging indent: body={item_two_body_x}, wrap={item_two_wrap_x}"
    );
}

#[test]
fn pdf_renderer_paragraph_indent_and_line_gap_invariants_v0() {
    let demo_text =
        b"Title\nAuthor\n2026-03-05\n\nFirst body paragraph line.\n\nSecond paragraph line.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept demo text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (first_x, first_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(First body paragraph line.)")
            .expect("first body line position");
    let (second_x, second_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Second paragraph line.)")
            .expect("second paragraph line position");

    assert!(
        (first_x - 72.0).abs() <= 0.02,
        "first paragraph x mismatch: {first_x}"
    );
    assert!(
        (second_x - 96.0).abs() <= 0.02,
        "indented paragraph x mismatch: {second_x}"
    );
    assert!(
        (first_y - second_y - 27.0).abs() <= 0.02,
        "paragraph y-gap mismatch: first_y={first_y}, second_y={second_y}"
    );
}

#[test]
fn pdf_renderer_body_only_long_page_paragraph_block_rhythm_is_stable_v12() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\nP1START aa bb cc dd ee ff gg hh ii jj kk ll mm nn P1LAST.\n\nP2START aa bb cc dd ee P2WRAP ff gg hh ii jj kk ll mm nn.",
        65_536,
        786_432,
        20,
    )
    .expect("writer should accept wrapped body-only paragraphs");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, p1_last_y) =
        tm_position_for_segment_substring_v0(&pdf, "P1LAST").expect("first paragraph tail");
    let (p2_start_x, p2_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "P2START").expect("second paragraph start");
    let (p2_wrap_x, p2_wrap_y) =
        tm_position_for_line_containing_text_v0(&pdf, "P2WRAP").expect("second paragraph wrap");
    let epsilon_pt = 0.05f32;
    assert!(
        (p1_last_y - p2_start_y - 27.0).abs() <= epsilon_pt,
        "paragraph-to-paragraph block gap on long body-only pages should stay stable: p1_last_y={p1_last_y}, p2_start_y={p2_start_y}"
    );
    assert!(
        (p2_start_x - 96.0).abs() <= epsilon_pt && (p2_wrap_x - 72.0).abs() <= epsilon_pt,
        "second paragraph start/wrap columns should remain stable: p2_start_x={p2_start_x}, p2_wrap_x={p2_wrap_x}"
    );
    assert!(
        (p2_start_y - p2_wrap_y - 13.0).abs() <= epsilon_pt,
        "wrapped continuation rhythm in long body-only pages should stay tightened: p2_start_y={p2_start_y}, p2_wrap_y={p2_wrap_y}"
    );
}

#[test]
fn pdf_renderer_front_matter_title_to_first_body_rhythm_is_tightened_v11() {
    let demo_text = b"Front Matter Title\nAuthor Name\n2026-03-05\n\nFirst body paragraph line.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept demo text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, date_y) = tm_position_for_line_containing_text_v0(&pdf, "(2026-03-05)")
        .expect("date line position");
    let (body_x, body_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(First body paragraph line.)")
            .expect("first body line position");
    let epsilon_pt = 0.05f32;
    assert!(
        (date_y - body_y - 38.0).abs() <= epsilon_pt,
        "front-matter date->first-body rhythm should stay tightened and deterministic: date_y={date_y}, body_y={body_y}"
    );
    assert!(
        (body_x - 72.0).abs() <= 0.02,
        "first body line after front matter should remain unindented: body_x={body_x}"
    );
}

#[test]
fn pdf_renderer_tall_title_block_spacing_and_body_transition_polish_v23() {
    let demo_text = b"Front Matter Main Title\nFront Matter Subtitle\nAuthor One\nAuthor Two\n2026-03-05\n\nFirst body paragraph line.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept tall title block text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, title_y) = tm_position_for_line_containing_text_v0(&pdf, "(Front Matter Main Title)")
        .expect("title line");
    let (_, subtitle_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Front Matter Subtitle)")
            .expect("subtitle line");
    let (_, author_one_y) = tm_position_for_line_containing_text_v0(&pdf, "(Author One)")
        .expect("author one line");
    let (_, author_two_y) = tm_position_for_line_containing_text_v0(&pdf, "(Author Two)")
        .expect("author two line");
    let (_, date_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(2026-03-05)").expect("date line");
    let (body_x, body_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(First body paragraph line.)")
            .expect("body line");

    let epsilon_pt = 0.05f32;
    assert!(
        (title_y - subtitle_y - 13.0).abs() <= epsilon_pt
            && (subtitle_y - author_one_y - 13.0).abs() <= epsilon_pt
            && (author_one_y - author_two_y - 13.0).abs() <= epsilon_pt
            && (author_two_y - date_y - 13.0).abs() <= epsilon_pt,
        "tall title-block internal line spacing should stay compact and stable: title_y={title_y}, subtitle_y={subtitle_y}, author_one_y={author_one_y}, author_two_y={author_two_y}, date_y={date_y}"
    );
    assert!(
        (date_y - body_y - 33.0).abs() <= epsilon_pt,
        "tall title-block date->first-body transition should stay tightened: date_y={date_y}, body_y={body_y}"
    );
    assert!(
        (body_x - 72.0).abs() <= 0.02,
        "first body line after tall title block should remain unindented: body_x={body_x}"
    );
}

#[test]
fn pdf_renderer_tall_title_block_to_heading_transition_polish_v23() {
    let demo_text = b"Front Matter Main Title\nFront Matter Subtitle\nAuthor One\nAuthor Two\n2026-03-05\n\n@S {Heading After Tall Front Matter}\n\n~ Body after heading.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept tall title->heading text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, date_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(2026-03-05)").expect("date line");
    let (_, heading_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Heading After Tall Front Matter)")
            .expect("heading line");
    let (body_x, body_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Body after heading.)").expect("body line");

    let epsilon_pt = 0.05f32;
    assert!(
        (date_y - heading_y - 33.0).abs() <= epsilon_pt,
        "tall title-block->heading opening gap mismatch: date_y={date_y}, heading_y={heading_y}"
    );
    assert!(
        (heading_y - body_y - 24.0).abs() <= epsilon_pt,
        "heading->first-body gap after tall title block mismatch: heading_y={heading_y}, body_y={body_y}"
    );
    assert!(
        (body_x - 72.0).abs() <= 0.02,
        "first body line after heading should remain unindented: body_x={body_x}"
    );
}

#[test]
fn pdf_renderer_section_heading_spacing_invariants_v0() {
    let demo_text =
        b"Title\nAuthor\n2026-03-05\n\nIntro paragraph.\n\n{Section Heading}\n\n~ Body after heading.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept heading text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (intro_x, intro_y) = tm_position_for_line_containing_text_v0(&pdf, "(Intro paragraph.)")
        .expect("intro position");
    let (_, heading_y) = tm_position_for_line_containing_text_v0(&pdf, "(Section Heading)")
        .expect("heading position");
    assert!(!pdf
        .windows(b"(~ Body after heading.) Tj".len())
        .any(|w| w == b"(~ Body after heading.) Tj"));
    let (body_x, body_y) = tm_position_for_line_containing_text_v0(&pdf, "(Body after heading.)")
        .expect("body position");

    assert!(
        (intro_y - heading_y - 24.0).abs() <= 0.02,
        "intro->heading y-gap mismatch: intro_y={intro_y}, heading_y={heading_y}"
    );
    assert!(
        (heading_y - body_y - 24.0).abs() <= 0.02,
        "heading->body y-gap mismatch: heading_y={heading_y}, body_y={body_y}"
    );
    assert!(
        (intro_x - 72.0).abs() <= 0.02,
        "intro x mismatch: {intro_x}"
    );
    assert!(
        (body_x - 72.0).abs() <= 0.02,
        "first paragraph after heading should not indent: {body_x}"
    );
}

#[test]
fn pdf_renderer_front_matter_heading_opening_rhythm_polish_v22() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\n@S {Front Heading}\n\n~ Body after front heading.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept front-matter heading text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, date_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(2026-03-05)").expect("date position");
    let (_, heading_y) = tm_position_for_line_containing_text_v0(&pdf, "(Front Heading)")
        .expect("heading position");
    let (_, body_y) = tm_position_for_line_containing_text_v0(&pdf, "(Body after front heading.)")
        .expect("body position");

    let epsilon_pt = 0.02f32;
    assert!(
        (date_y - heading_y - 38.0).abs() <= epsilon_pt,
        "front-matter->heading opening gap mismatch: date_y={date_y}, heading_y={heading_y}"
    );
    assert!(
        (heading_y - body_y - 24.0).abs() <= epsilon_pt,
        "heading->first-body gap mismatch: heading_y={heading_y}, body_y={body_y}"
    );
}

#[test]
fn pdf_renderer_heading_transitions_across_list_quote_table_polish_v22() {
    let demo_text = b"\nPrelude paragraph.\n\n- List line one.\n- List line two.\n\n@S {After List Heading}\n\n~ Body after list heading.\n\n> Quote line one.\n> Quote line two.\n\n@S {After Quote Heading}\n\n~ Body after quote heading.\n\n!ts ll\n!t TROWONEA||TROWONEB\n!t TROWTWOA||TROWTWOB\n\n@S {After Table Heading}\n\n~ Body after table heading.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept mixed heading transition text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, list_two_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(List line two.)").expect("list line two");
    let (_, after_list_heading_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(After List Heading)")
            .expect("after-list heading");
    let (_, after_list_body_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Body after list heading.)")
            .expect("after-list body");
    let (_, quote_two_y) = tm_position_for_line_containing_text_v0(&pdf, "(Quote line two.)")
        .expect("quote line two");
    let (_, after_quote_heading_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(After Quote Heading)")
            .expect("after-quote heading");
    let (_, after_quote_body_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Body after quote heading.)")
            .expect("after-quote body");
    let (_, table_row_two_y) =
        tm_position_for_segment_substring_v0(&pdf, "TROWTWOA").expect("table row two");
    let (_, after_table_heading_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(After Table Heading)")
            .expect("after-table heading");
    let (_, after_table_body_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Body after table heading.)")
            .expect("after-table body");

    let epsilon_pt = 0.2f32;
    assert!(
        (list_two_y - after_list_heading_y - 24.0).abs() <= epsilon_pt,
        "list->heading gap mismatch: list_two_y={list_two_y}, after_list_heading_y={after_list_heading_y}"
    );
    assert!(
        (after_list_heading_y - after_list_body_y - 24.0).abs() <= epsilon_pt,
        "heading->paragraph gap mismatch after list: after_list_heading_y={after_list_heading_y}, after_list_body_y={after_list_body_y}"
    );
    assert!(
        (quote_two_y - after_quote_heading_y - 23.0).abs() <= epsilon_pt,
        "quote->heading gap mismatch: quote_two_y={quote_two_y}, after_quote_heading_y={after_quote_heading_y}"
    );
    assert!(
        (after_quote_heading_y - after_quote_body_y - 24.0).abs() <= epsilon_pt,
        "heading->paragraph gap mismatch after quote: after_quote_heading_y={after_quote_heading_y}, after_quote_body_y={after_quote_body_y}"
    );
    assert!(
        (table_row_two_y - after_table_heading_y - 24.0).abs() <= epsilon_pt,
        "table->heading gap mismatch: table_row_two_y={table_row_two_y}, after_table_heading_y={after_table_heading_y}"
    );
    assert!(
        (after_table_heading_y - after_table_body_y - 24.0).abs() <= epsilon_pt,
        "heading->paragraph gap mismatch after table: after_table_heading_y={after_table_heading_y}, after_table_body_y={after_table_body_y}"
    );
}

#[test]
fn pdf_renderer_front_matter_list_and_table_opening_transitions_are_tightened_v25() {
    let list_pdf = render_dvi_v2_text_page_to_pdf_v0(
        &write_dvi_v2_text_page_v0(
            b"Front Matter Title\nAuthor Name\n2026-03-05\n\n- LISTOPEN first list item.\n- LISTNEXT second list item.",
        )
        .expect("writer should accept front-matter list text"),
    )
    .expect("list pdf render");
    let (_, list_date_y) =
        tm_position_for_line_containing_text_v0(&list_pdf, "(2026-03-05)").expect("list date");
    let (_, list_open_y) = tm_position_for_line_containing_text_v0(&list_pdf, "(LISTOPEN first list item.)")
        .expect("list opening line");
    let (_, list_next_y) = tm_position_for_line_containing_text_v0(&list_pdf, "(LISTNEXT second list item.)")
        .expect("list second line");

    let table_pdf = render_dvi_v2_text_page_to_pdf_v0(
        &write_dvi_v2_text_page_v0(
            b"Front Matter Title\nAuthor Name\n2026-03-05\n\n!ts ll\n!t TABOPENA||TABOPENB\n!t TABNEXTA||TABNEXTB",
        )
        .expect("writer should accept front-matter table text"),
    )
    .expect("table pdf render");
    let (_, table_date_y) =
        tm_position_for_line_containing_text_v0(&table_pdf, "(2026-03-05)").expect("table date");
    let (_, table_open_y) =
        tm_position_for_segment_substring_v0(&table_pdf, "TABOPENA").expect("table opening row");
    let (_, table_next_y) =
        tm_position_for_segment_substring_v0(&table_pdf, "TABNEXTA").expect("table second row");

    let epsilon_pt = 0.05f32;
    assert!(
        (list_date_y - list_open_y - 38.0).abs() <= epsilon_pt,
        "front-matter date->list opening transition should stay tightened: list_date_y={list_date_y}, list_open_y={list_open_y}"
    );
    assert!(
        (list_open_y - list_next_y - 13.0).abs() <= epsilon_pt,
        "list internal rhythm should stay stable after front-matter opening: list_open_y={list_open_y}, list_next_y={list_next_y}"
    );
    assert!(
        (table_date_y - table_open_y - 38.0).abs() <= epsilon_pt,
        "front-matter date->table opening transition should stay tightened: table_date_y={table_date_y}, table_open_y={table_open_y}"
    );
    assert!(
        (table_open_y - table_next_y - 13.0).abs() <= epsilon_pt,
        "table row rhythm should stay stable after front-matter opening: table_open_y={table_open_y}, table_next_y={table_next_y}"
    );
}

#[test]
fn pdf_renderer_heading_list_quote_rhythm_invariants_v0() {
    let demo_text = b"\nPrelude paragraph.\n\n{Heading}\n\n~ After heading paragraph.\n\n- First list item\n- Second list item\n\n> Quote line one\n> Quote line two\n\nAfter quote paragraph.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept rhythm text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, prelude_y) = tm_position_for_line_containing_text_v0(&pdf, "(Prelude paragraph.)")
        .expect("prelude position");
    let (_, heading_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Heading)").expect("heading position");
    let (_, after_heading_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(After heading paragraph.)")
            .expect("after heading position");
    let (list_one_x, list_one_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(First list item)").expect("list one");
    let (_, list_two_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Second list item)").expect("list two");
    let (quote_one_x, quote_one_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Quote line one)").expect("quote one");
    let (quote_two_x, quote_two_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Quote line two)").expect("quote two");
    let (_, after_quote_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(After quote paragraph.)")
            .expect("after quote");

    let epsilon_pt = 0.02f32;
    assert!((prelude_y - heading_y - 24.0).abs() <= epsilon_pt);
    assert!(
        (heading_y - after_heading_y - 24.0).abs() <= epsilon_pt,
        "heading->first paragraph gap mismatch: heading_y={heading_y}, after_heading_y={after_heading_y}"
    );
    assert!(
        (after_heading_y - list_one_y - 24.0).abs() <= epsilon_pt,
        "paragraph->list gap mismatch: after_heading_y={after_heading_y}, list_one_y={list_one_y}"
    );
    assert!(
        (list_one_y - list_two_y - 13.0).abs() <= epsilon_pt,
        "list line gap mismatch: list_one_y={list_one_y}, list_two_y={list_two_y}"
    );
    assert!(
        (list_two_y - quote_one_y - 23.0).abs() <= epsilon_pt,
        "list->quote gap mismatch: list_two_y={list_two_y}, quote_one_y={quote_one_y}"
    );
    assert!(
        (quote_one_y - quote_two_y - 12.5).abs() <= epsilon_pt,
        "quote line gap mismatch: quote_one_y={quote_one_y}, quote_two_y={quote_two_y}"
    );
    assert!(
        (quote_two_y - after_quote_y - 23.0).abs() <= epsilon_pt,
        "quote->paragraph gap mismatch: quote_two_y={quote_two_y}, after_quote_y={after_quote_y}"
    );
    assert!(quote_one_x > 72.0, "quote line should be indented");
    assert!(
        quote_one_x >= list_one_x + 6.0,
        "quote indent should be visibly deeper than list body indent: list_one_x={list_one_x}, quote_one_x={quote_one_x}"
    );
    assert!(
        (quote_one_x - quote_two_x).abs() <= epsilon_pt,
        "quote x drift mismatch"
    );
}

#[test]
fn pdf_renderer_applies_hanging_indent_for_list_continuation_v0() {
    let xdv =
        write_dvi_v2_text_page_v0(b"- item\ncontinuation").expect("writer should accept text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (item_x, item_y) = tm_position_for_segment_substring_v0(&pdf, "item").expect("item");
    let (continuation_x, continuation_y) =
        tm_position_for_segment_substring_v0(&pdf, "continuation").expect("continuation");
    let epsilon_pt = 0.02f32;
    assert!(
        (item_x - 96.0).abs() <= epsilon_pt,
        "item line body x mismatch: {item_x}"
    );
    assert!(
        (continuation_x - item_x).abs() <= epsilon_pt,
        "continuation should keep hanging indent: item_x={item_x}, continuation_x={continuation_x}"
    );
    assert!(
        (item_y - continuation_y - 13.0).abs() <= epsilon_pt,
        "list continuation line rhythm mismatch: item_y={item_y}, continuation_y={continuation_y}"
    );
}

#[test]
fn pdf_renderer_itemize_bullet_and_body_x_offsets_invariants_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"- alpha\ncontinuation\n- beta\ncontinuationtwo")
        .expect("writer should accept list text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    assert!(
        !pdf.windows(b"(- alpha) Tj".len())
            .any(|w| w == b"(- alpha) Tj"),
        "prefix should be split from body"
    );

    let bullet_xs = tm_xs_for_segment_text_v0(&pdf, "-");
    let alpha_xs = tm_xs_for_segment_text_v0(&pdf, "alpha");
    let beta_xs = tm_xs_for_segment_text_v0(&pdf, "beta");
    let continuation_xs = tm_xs_for_segment_text_v0(&pdf, "continuation");
    let continuation_two_xs = tm_xs_for_segment_text_v0(&pdf, "continuationtwo");

    assert_eq!(
        bullet_xs.len(),
        2,
        "expected two bullet renders: {bullet_xs:?}"
    );
    assert_eq!(alpha_xs.len(), 1, "expected alpha render");
    assert_eq!(beta_xs.len(), 1, "expected beta render");
    assert_eq!(continuation_xs.len(), 1, "expected continuation render");
    assert_eq!(
        continuation_two_xs.len(),
        1,
        "expected continuationtwo render"
    );

    let epsilon_pt = 0.02f32;
    for x in [
        alpha_xs[0],
        beta_xs[0],
        continuation_xs[0],
        continuation_two_xs[0],
    ] {
        assert!((x - 96.0).abs() <= epsilon_pt, "item body x mismatch: {x}");
    }
    let target_gap_pt = 8.0f32;
    for (bullet_x, body_x) in bullet_xs.iter().zip([alpha_xs[0], beta_xs[0]]) {
        let marker_gap = body_x - (*bullet_x + segment_width_pt_v0(b"-"));
        assert!(
            (marker_gap - target_gap_pt).abs() <= 0.25,
            "itemize marker/body gap should stay tight and stable: marker_gap={marker_gap}, body_x={body_x}, bullet_x={bullet_x}"
        );
    }
}

#[test]
fn pdf_renderer_enumerate_number_column_alignment_invariants_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"9. nine\n10. ten")
        .expect("writer should accept enumerate text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    assert!(
        !pdf.windows(b"(9. nine) Tj".len())
            .any(|w| w == b"(9. nine) Tj"),
        "prefix should be split from body"
    );

    let nine_number_x = tm_xs_for_segment_text_v0(&pdf, "9.");
    let ten_number_x = tm_xs_for_segment_text_v0(&pdf, "10.");
    let nine_body_x = tm_xs_for_segment_text_v0(&pdf, "nine");
    let ten_body_x = tm_xs_for_segment_text_v0(&pdf, "ten");

    assert_eq!(nine_number_x.len(), 1, "expected 9. number render");
    assert_eq!(ten_number_x.len(), 1, "expected 10. number render");
    assert_eq!(nine_body_x.len(), 1, "expected nine body render");
    assert_eq!(ten_body_x.len(), 1, "expected ten body render");

    let epsilon_pt = 0.02f32;
    assert!(
        nine_number_x[0] > ten_number_x[0],
        "single-digit number should start further right: nine={:?}, ten={:?}",
        nine_number_x,
        ten_number_x
    );
    assert!(
        (nine_body_x[0] - 96.0).abs() <= epsilon_pt,
        "nine body x mismatch: {}",
        nine_body_x[0]
    );
    assert!(
        (ten_body_x[0] - 96.0).abs() <= epsilon_pt,
        "ten body x mismatch: {}",
        ten_body_x[0]
    );
    let min_gap_pt = 7.5f32;
    let nine_number_right = nine_number_x[0] + segment_width_pt_v0(b"9.");
    let ten_number_right = ten_number_x[0] + segment_width_pt_v0(b"10.");
    assert!(
        nine_body_x[0] - nine_number_right >= min_gap_pt,
        "enumerate gap for 9. should remain readable: body_x={}, number_right={}",
        nine_body_x[0],
        nine_number_right
    );
    assert!(
        ten_body_x[0] - ten_number_right >= min_gap_pt,
        "enumerate gap for 10. should remain readable: body_x={}, number_right={}",
        ten_body_x[0],
        ten_number_right
    );
}

#[test]
fn pdf_renderer_enumerate_number_column_alignment_across_wraps_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"9. [NINESTART] item with enough repeated words to force wrapping and keep deterministic body indent for continuation lines before token [WRAPNINE]\n10. [TENSTART] item with enough repeated words to force wrapping and keep deterministic body indent for continuation lines before token [WRAPTEN]")
        .expect("writer should accept long enumerate text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let nine_number_x = tm_xs_for_segment_text_v0(&pdf, "9.");
    let ten_number_x = tm_xs_for_segment_text_v0(&pdf, "10.");
    let nine_start_x = tm_xs_for_segment_text_v0(&pdf, "NINESTART");
    let ten_start_x = tm_xs_for_segment_text_v0(&pdf, "TENSTART");
    let nine_wrap_x = tm_line_start_xs_for_segment_text_v0(&pdf, "WRAPNINE");
    let ten_wrap_x = tm_line_start_xs_for_segment_text_v0(&pdf, "WRAPTEN");

    assert_eq!(nine_number_x.len(), 1, "expected 9. number render");
    assert_eq!(ten_number_x.len(), 1, "expected 10. number render");
    assert_eq!(nine_start_x.len(), 1, "expected NINESTART render");
    assert_eq!(ten_start_x.len(), 1, "expected TENSTART render");
    assert_eq!(nine_wrap_x.len(), 1, "expected WRAPNINE render");
    assert_eq!(ten_wrap_x.len(), 1, "expected WRAPTEN render");

    let epsilon_pt = 0.02f32;
    assert!(
        nine_number_x[0] > ten_number_x[0],
        "single-digit number should start further right: nine={:?}, ten={:?}",
        nine_number_x,
        ten_number_x
    );
    assert!(
        (nine_start_x[0] - 96.0).abs() <= epsilon_pt,
        "start body x mismatch for 9.: {}",
        nine_start_x[0]
    );
    assert!(
        (ten_start_x[0] - 96.0).abs() <= epsilon_pt,
        "start body x mismatch for 10.: {}",
        ten_start_x[0]
    );
    assert!(
        (nine_wrap_x[0] - nine_start_x[0]).abs() <= epsilon_pt,
        "wrap body x mismatch for 9.: start={}, wrap={}",
        nine_start_x[0],
        nine_wrap_x[0]
    );
    assert!(
        (ten_wrap_x[0] - ten_start_x[0]).abs() <= epsilon_pt,
        "wrap body x mismatch for 10.: start={}, wrap={}",
        ten_start_x[0],
        ten_wrap_x[0]
    );
    let min_gap_pt = 7.5f32;
    let nine_number_right = nine_number_x[0] + segment_width_pt_v0(b"9.");
    let ten_number_right = ten_number_x[0] + segment_width_pt_v0(b"10.");
    assert!(
        nine_start_x[0] - nine_number_right >= min_gap_pt,
        "wrapped enumerate gap for 9. should remain readable: body_x={}, number_right={}",
        nine_start_x[0],
        nine_number_right
    );
    assert!(
        ten_start_x[0] - ten_number_right >= min_gap_pt,
        "wrapped enumerate gap for 10. should remain readable: body_x={}, number_right={}",
        ten_start_x[0],
        ten_number_right
    );
}

#[test]
fn pdf_renderer_mixed_list_block_transition_and_marker_spacing_invariants_v20() {
    let xdv = write_dvi_v2_text_page_v0(b"\nParagraph before lists.\n\n- BULLETSTART alpha alpha alpha alpha alpha alpha alpha alpha alpha alpha alpha alpha alpha alpha alpha alpha BULLETWRAP\n\n10. ENUMSTART beta beta beta beta beta beta beta beta beta beta beta beta beta beta beta beta ENUMWRAP\n\nParagraph after lists.")
        .expect("writer should accept mixed list transition text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, before_lists_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Paragraph before lists.)")
            .expect("before list paragraph");
    let (bullet_start_x, bullet_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "BULLETSTART").expect("bullet start");
    let (bullet_wrap_x, bullet_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "BULLETWRAP").expect("bullet wrap");
    let (enum_start_x, enum_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "ENUMSTART").expect("enum start");
    let (enum_wrap_x, enum_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "ENUMWRAP").expect("enum wrap");
    let (_, after_lists_y) = tm_position_for_line_containing_text_v0(&pdf, "(Paragraph after lists.)")
        .expect("after list paragraph");

    let bullet_xs = tm_xs_for_segment_text_v0(&pdf, "-");
    let enum_number_xs = tm_xs_for_segment_text_v0(&pdf, "10.");
    assert_eq!(bullet_xs.len(), 1, "expected one bullet marker: {bullet_xs:?}");
    assert_eq!(
        enum_number_xs.len(),
        1,
        "expected one enumerate marker: {enum_number_xs:?}"
    );

    let epsilon_pt = 0.2f32;
    assert!(
        (before_lists_y - bullet_start_y - 24.0).abs() <= epsilon_pt,
        "paragraph->list transition should remain tightened: before_lists_y={before_lists_y}, bullet_start_y={bullet_start_y}"
    );
    assert!(
        (bullet_wrap_y - enum_start_y - 24.0).abs() <= epsilon_pt,
        "list->list block transition should remain even: bullet_wrap_y={bullet_wrap_y}, enum_start_y={enum_start_y}"
    );
    assert!(
        (enum_wrap_y - after_lists_y - 24.0).abs() <= epsilon_pt,
        "list->paragraph transition should remain tightened: enum_wrap_y={enum_wrap_y}, after_lists_y={after_lists_y}"
    );
    assert!(
        (bullet_start_y - bullet_wrap_y - 13.0).abs() <= epsilon_pt,
        "itemize wrapped-line rhythm mismatch: bullet_start_y={bullet_start_y}, bullet_wrap_y={bullet_wrap_y}"
    );
    assert!(
        (enum_start_y - enum_wrap_y - 13.0).abs() <= epsilon_pt,
        "enumerate wrapped-line rhythm mismatch: enum_start_y={enum_start_y}, enum_wrap_y={enum_wrap_y}"
    );

    assert!(
        (bullet_start_x - enum_start_x).abs() <= 0.02,
        "mixed list body columns should stay aligned: bullet_start_x={bullet_start_x}, enum_start_x={enum_start_x}"
    );
    assert!(
        (bullet_wrap_x - bullet_start_x).abs() <= 0.02
            && (enum_wrap_x - enum_start_x).abs() <= 0.02,
        "wrapped list continuation x should stay aligned: bullet_start_x={bullet_start_x}, bullet_wrap_x={bullet_wrap_x}, enum_start_x={enum_start_x}, enum_wrap_x={enum_wrap_x}"
    );
    let bullet_gap = bullet_start_x - (bullet_xs[0] + segment_width_pt_v0(b"-"));
    let enum_gap = enum_start_x - (enum_number_xs[0] + segment_width_pt_v0(b"10."));
    assert!(
        (bullet_gap - enum_gap).abs() <= 0.25,
        "mixed itemize/enumerate marker/body gap drift: bullet_gap={bullet_gap}, enum_gap={enum_gap}"
    );
}

#[test]
fn pdf_renderer_nested_mixed_width_enumerate_wrap_alignment_invariants_v20() {
    let xdv = write_dvi_v2_text_page_v0(b"- Outer item.\n  9. NINESTART gamma gamma gamma gamma gamma gamma gamma gamma gamma gamma gamma gamma gamma gamma gamma gamma NINEWRAP\n  10. TENSTART delta delta delta delta delta delta delta delta delta delta delta delta delta delta delta delta TENWRAP")
        .expect("writer should accept nested mixed-width enumerate text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let nine_number_x = tm_xs_for_segment_text_v0(&pdf, "9.");
    let ten_number_x = tm_xs_for_segment_text_v0(&pdf, "10.");
    let (nine_start_x, nine_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "NINESTART").expect("nine start");
    let (nine_wrap_x, nine_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "NINEWRAP").expect("nine wrap");
    let (ten_start_x, ten_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "TENSTART").expect("ten start");
    let (ten_wrap_x, ten_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "TENWRAP").expect("ten wrap");

    assert_eq!(nine_number_x.len(), 1, "expected one 9. marker");
    assert_eq!(ten_number_x.len(), 1, "expected one 10. marker");

    let epsilon_pt = 0.2f32;
    assert!(
        (nine_start_x - ten_start_x).abs() <= 0.02,
        "nested enumerate body column should stay aligned across mixed-width markers: nine_start_x={nine_start_x}, ten_start_x={ten_start_x}"
    );
    assert!(
        (nine_start_x - nine_wrap_x).abs() <= 0.02 && (ten_start_x - ten_wrap_x).abs() <= 0.02,
        "nested wrapped continuation x should remain stable: nine_start_x={nine_start_x}, nine_wrap_x={nine_wrap_x}, ten_start_x={ten_start_x}, ten_wrap_x={ten_wrap_x}"
    );
    let nine_gap = nine_start_x - (nine_number_x[0] + segment_width_pt_v0(b"9."));
    let ten_gap = ten_start_x - (ten_number_x[0] + segment_width_pt_v0(b"10."));
    assert!(
        (nine_gap - ten_gap).abs() <= 0.25,
        "nested enumerate marker/body gap drift across mixed-width markers: nine_gap={nine_gap}, ten_gap={ten_gap}"
    );
    assert!(
        (nine_start_y - nine_wrap_y - 13.0).abs() <= epsilon_pt,
        "nested 9. wrapped-line rhythm mismatch: nine_start_y={nine_start_y}, nine_wrap_y={nine_wrap_y}"
    );
    assert!(
        (ten_start_y - ten_wrap_y - 13.0).abs() <= epsilon_pt,
        "nested 10. wrapped-line rhythm mismatch: ten_start_y={ten_start_y}, ten_wrap_y={ten_wrap_y}"
    );
    assert!(
        (nine_wrap_y - ten_start_y - 13.0).abs() <= epsilon_pt,
        "nested enumerate entry-to-entry rhythm mismatch: nine_wrap_y={nine_wrap_y}, ten_start_y={ten_start_y}"
    );
}

#[test]
fn pdf_renderer_figure_caption_and_adjacent_table_transition_rhythm_v21() {
    let xdv = write_dvi_v2_text_page_v0(b"\nParagraph before blocks.\n\n!gbox\n!gcap Figure 1: CAPSTART [dense], {styled} caption text with punctuation, continuity check.\n\n!ts ll\n!t ROWONEA||ROWONEB\n!t ROWTWOA||ROWTWOB\n\nParagraph after blocks.")
        .expect("writer should accept figure-table transition text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, caption_y) =
        tm_position_for_segment_substring_v0(&pdf, "CAPSTART").expect("caption start");
    let (_, row_one_y) = tm_position_for_segment_substring_v0(&pdf, "ROWONEA").expect("row one");
    let (_, row_two_y) = tm_position_for_segment_substring_v0(&pdf, "ROWTWOA").expect("row two");
    let (_, after_blocks_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Paragraph after blocks.)")
            .expect("paragraph after blocks");

    let epsilon_pt = 0.2f32;
    assert!(
        (caption_y - row_one_y - 37.0).abs() <= epsilon_pt,
        "figure-caption->table transition gap should stay tightened: caption_y={caption_y}, row_one_y={row_one_y}"
    );
    assert!(
        (row_one_y - row_two_y - 13.0).abs() <= epsilon_pt,
        "table row leading should remain stable after caption transition: row_one_y={row_one_y}, row_two_y={row_two_y}"
    );
    assert!(
        (row_two_y - after_blocks_y - 24.0).abs() <= epsilon_pt,
        "table->paragraph transition should stay tightened: row_two_y={row_two_y}, after_blocks_y={after_blocks_y}"
    );
    let max_caption_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "CAPSTART")
        .expect("caption line should render and expose tm gaps");
    assert!(
        max_caption_tm_gap <= 18.0,
        "caption styled seam spacing should remain bounded: max_caption_tm_gap={max_caption_tm_gap}"
    );
}

#[test]
fn pdf_renderer_table_to_figure_caption_separation_rhythm_v21() {
    let xdv = write_dvi_v2_text_page_v0(b"\n!ts ll\n!t TROWONEA||TROWONEB\n!t TROWTWOA||TROWTWOB\n\n!gbox\n!gcap Figure 1: FIGCAPSTART compact caption text.\n\nParagraph after figure.")
        .expect("writer should accept table-figure transition text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, row_two_y) =
        tm_position_for_segment_substring_v0(&pdf, "TROWTWOA").expect("table row two");
    let (_, caption_y) =
        tm_position_for_segment_substring_v0(&pdf, "FIGCAPSTART").expect("figure caption");
    let (_, after_figure_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Paragraph after figure.)")
            .expect("paragraph after figure");

    let epsilon_pt = 0.2f32;
    assert!(
        (row_two_y - caption_y - 156.0).abs() <= epsilon_pt,
        "table->figure transition should keep stable block/caption separation: row_two_y={row_two_y}, caption_y={caption_y}"
    );
    assert!(
        (caption_y - after_figure_y - 24.0).abs() <= epsilon_pt,
        "figure-caption->paragraph transition should stay tightened: caption_y={caption_y}, after_figure_y={after_figure_y}"
    );
}

#[test]
fn pdf_renderer_bibliography_entries_use_hanging_indent_and_stable_rhythm_v14() {
    let xdv = write_dvi_v2_text_page_v0(
        b"@S {References}\n\n[1] ALPHASTART alpha alpha alpha alpha alpha alpha alpha alpha alpha alpha ALPHAWRAP\n[12] BETASTART beta beta beta beta beta beta beta beta beta beta beta beta beta beta beta beta BETAWRAP",
    )
    .expect("writer should accept bibliography-style lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (references_x, references_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(References)").expect("references heading");
    let (alpha_start_x, alpha_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "ALPHASTART").expect("alpha start");
    let (alpha_wrap_x, alpha_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "ALPHAWRAP").expect("alpha wrap");
    let (beta_start_x, beta_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "BETASTART").expect("beta start");
    let (beta_wrap_x, beta_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "BETAWRAP").expect("beta wrap");

    let one_label_x = *tm_xs_for_segment_text_v0(&pdf, "1")
        .first()
        .expect("label [1] x");
    let twelve_label_x = *tm_xs_for_segment_text_v0(&pdf, "12")
        .first()
        .expect("label [12] x");
    let one_label_right = one_label_x + segment_width_pt_v0(b"1");
    let twelve_label_right = twelve_label_x + segment_width_pt_v0(b"12");

    let epsilon_pt = 0.2f32;
    assert!(
        (references_y - alpha_start_y - 12.0).abs() <= epsilon_pt,
        "references heading -> first bibliography entry gap should be tightened and stable: references_y={references_y}, alpha_start_y={alpha_start_y}"
    );
    assert!(
        (alpha_start_y - alpha_wrap_y - 12.5).abs() <= epsilon_pt,
        "bibliography wrapped line rhythm should be stable: alpha_start_y={alpha_start_y}, alpha_wrap_y={alpha_wrap_y}"
    );
    assert!(
        (alpha_wrap_y - beta_start_y - 12.0).abs() <= epsilon_pt,
        "bibliography entry-to-entry rhythm should be stable: alpha_wrap_y={alpha_wrap_y}, beta_start_y={beta_start_y}"
    );
    assert!(
        (beta_start_y - beta_wrap_y - 12.5).abs() <= epsilon_pt,
        "bibliography wrapped line rhythm should be stable for later entries: beta_start_y={beta_start_y}, beta_wrap_y={beta_wrap_y}"
    );
    assert!(
        (alpha_start_x - beta_start_x).abs() <= epsilon_pt,
        "bibliography body column should remain stable across mixed-width ordinals: alpha_start_x={alpha_start_x}, beta_start_x={beta_start_x}"
    );
    assert!(
        (alpha_start_x - alpha_wrap_x).abs() <= epsilon_pt
            && (beta_start_x - beta_wrap_x).abs() <= epsilon_pt,
        "bibliography wrapped continuation lines should keep hanging-indent column"
    );
    assert!(
        (one_label_right - twelve_label_right).abs() <= epsilon_pt,
        "bibliography ordinal label right edge should stay aligned: one_label_right={one_label_right}, twelve_label_right={twelve_label_right}"
    );
    assert!(
        references_x >= 72.0,
        "references heading should remain inside printable area"
    );
}

#[test]
fn pdf_renderer_bibliography_styled_seams_use_indented_profile_v32() {
    let xdv = write_dvi_v2_text_page_v0(
        b"@S {References}\n\n[1] BIBSTART [ITALICBIBV32] with {BOLDBIBV32} tail.",
    )
    .expect("writer should accept bibliography seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    let italic_line = pdf_text
        .lines()
        .find(|line| line.contains("(ITALICBIBV32) Tj"))
        .expect("bibliography italic line should render");
    let bold_line = pdf_text
        .lines()
        .find(|line| line.contains("(BOLDBIBV32) Tj"))
        .expect("bibliography bold line should render");

    assert!(
        italic_line.contains("97 Tz") && italic_line.contains("(ITALICBIBV32) Tj 100 Tz"),
        "bibliography italic seam should use indented seam compensation"
    );
    assert!(
        bold_line.contains("95 Tz") && bold_line.contains("(BOLDBIBV32) Tj 100 Tz"),
        "bibliography bold seam should use indented seam compensation"
    );
}

#[test]
fn pdf_renderer_body_to_bibliography_opening_gap_is_tightened_v17() {
    let xdv = write_dvi_v2_text_page_v0(
        b"~ Body before references.\n\n@S {References}\n\n[1] ALPHASTART alpha source text.",
    )
    .expect("writer should accept bibliography transition bytes");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, body_y) = tm_position_for_line_containing_text_v0(&pdf, "(Body before references.)")
        .expect("body before references");
    let (_, references_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(References)").expect("references heading");
    let (_, alpha_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "ALPHASTART").expect("first bibliography body");

    let epsilon_pt = 0.05f32;
    assert!(
        (body_y - references_y - 24.0).abs() <= epsilon_pt,
        "body->bibliography heading transition should stay tightened: body_y={body_y}, references_y={references_y}"
    );
    assert!(
        (references_y - alpha_start_y - 12.0).abs() <= epsilon_pt,
        "bibliography heading->first entry gap should remain stable: references_y={references_y}, alpha_start_y={alpha_start_y}"
    );
}

#[test]
fn pdf_renderer_mixed_surface_quote_table_list_bibliography_flow_rhythm_v25() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Front Matter Title\nAuthor Name\n2026-03-05\n\n> QUOTESTART quote opening line.\n> QUOTECONT quote continuation line.\n\n!ts ll\n!t TABSTARTA||TABSTARTB\n!t TABNEXTA||TABNEXTB\n\n- LISTSTART list opening line.\n- LISTNEXT list continuation line.\n\n@S {References}\n\n[1] BIBSTART alpha source text.",
    )
    .expect("writer should accept mixed-surface v25 text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, date_y) = tm_position_for_line_containing_text_v0(&pdf, "(2026-03-05)")
        .expect("date");
    let (quote_x, quote_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "QUOTESTART").expect("quote opening");
    let (_, quote_cont_y) =
        tm_position_for_segment_substring_v0(&pdf, "QUOTECONT").expect("quote continuation");
    let (_, table_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "TABSTARTA").expect("table opening");
    let (_, table_next_y) =
        tm_position_for_segment_substring_v0(&pdf, "TABNEXTA").expect("table continuation");
    let (list_x, list_start_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(LISTSTART list opening line.)")
            .expect("list opening");
    let (_, list_next_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(LISTNEXT list continuation line.)")
            .expect("list continuation");
    let (_, references_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(References)").expect("references");
    let (_, bib_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "BIBSTART").expect("bibliography opening");

    let epsilon_pt = 0.05f32;
    assert!(
        (date_y - quote_start_y - 37.0).abs() <= epsilon_pt,
        "front-matter date->quote transition should stay tightened: date_y={date_y}, quote_start_y={quote_start_y}"
    );
    assert!(
        (quote_start_y - quote_cont_y - 12.5).abs() <= epsilon_pt,
        "quote internal rhythm should remain stable: quote_start_y={quote_start_y}, quote_cont_y={quote_cont_y}"
    );
    assert!(
        (quote_cont_y - table_start_y - 23.0).abs() <= epsilon_pt,
        "quote->table transition should stay tightened in mixed-surface pages: quote_cont_y={quote_cont_y}, table_start_y={table_start_y}"
    );
    assert!(
        (table_start_y - table_next_y - 13.0).abs() <= epsilon_pt,
        "table internal rhythm should remain stable: table_start_y={table_start_y}, table_next_y={table_next_y}"
    );
    assert!(
        (table_next_y - list_start_y - 24.0).abs() <= epsilon_pt,
        "table->list transition should stay tightened in mixed-surface pages: table_next_y={table_next_y}, list_start_y={list_start_y}"
    );
    assert!(
        (list_start_y - list_next_y - 13.0).abs() <= epsilon_pt,
        "list internal rhythm should remain stable after table transition: list_start_y={list_start_y}, list_next_y={list_next_y}"
    );
    assert!(
        (list_next_y - references_y - 24.0).abs() <= epsilon_pt,
        "list->bibliography opening should stay tightened: list_next_y={list_next_y}, references_y={references_y}"
    );
    assert!(
        (references_y - bib_start_y - 12.0).abs() <= epsilon_pt,
        "bibliography heading->first entry gap should remain stable: references_y={references_y}, bib_start_y={bib_start_y}"
    );
    assert!(
        quote_x >= list_x + 6.0,
        "quote indent should remain visibly deeper than list body indent on mixed-surface pages: quote_x={quote_x}, list_x={list_x}"
    );
}

#[test]
fn pdf_renderer_nested_list_indentation_and_wrap_invariants_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"- [OUTERSTART] item with enough repeated words to force wrapping in the first list level before token [OUTERWRAPTOKEN]\n  - [NESTEDSTART] item with enough repeated words to force wrapping in the second list level before token [NESTEDWRAPTOKEN]")
        .expect("writer should accept nested list text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let bullet_xs = tm_xs_for_segment_text_v0(&pdf, "-");
    let outer_start_x = tm_xs_for_segment_text_v0(&pdf, "OUTERSTART");
    let (_, outer_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "OUTERSTART").expect("outer start y");
    let outer_wrap_x = tm_line_start_xs_for_segment_text_v0(&pdf, "OUTERWRAPTOKEN");
    let (_, outer_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "OUTERWRAPTOKEN").expect("outer wrap y");
    let nested_start_x = tm_xs_for_segment_text_v0(&pdf, "NESTEDSTART");
    let (_, nested_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "NESTEDSTART").expect("nested start y");
    let nested_wrap_x = tm_line_start_xs_for_segment_text_v0(&pdf, "NESTEDWRAPTOKEN");
    let (_, nested_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "NESTEDWRAPTOKEN").expect("nested wrap y");

    assert_eq!(bullet_xs.len(), 2, "expected two bullets: {bullet_xs:?}");
    assert_eq!(outer_start_x.len(), 1, "expected outer start render");
    assert_eq!(outer_wrap_x.len(), 1, "expected outer wrap render");
    assert_eq!(nested_start_x.len(), 1, "expected nested start render");
    assert_eq!(nested_wrap_x.len(), 1, "expected nested wrap render");

    let epsilon_pt = 0.02f32;
    assert!(
        bullet_xs[1] > bullet_xs[0],
        "nested bullet should shift right: {bullet_xs:?}"
    );
    assert!(
        (outer_start_x[0] - 96.0).abs() <= epsilon_pt,
        "outer body x mismatch: {}",
        outer_start_x[0]
    );
    assert!(
        (outer_wrap_x[0] - outer_start_x[0]).abs() <= epsilon_pt,
        "outer continuation x mismatch: outer={}, wrap={}",
        outer_start_x[0],
        outer_wrap_x[0]
    );
    let outer_marker_gap = outer_start_x[0] - (bullet_xs[0] + segment_width_pt_v0(b"-"));
    let nested_marker_gap = nested_start_x[0] - (bullet_xs[1] + segment_width_pt_v0(b"-"));
    assert!(
        (outer_marker_gap - 8.0).abs() <= 0.25,
        "outer marker/body gap mismatch: {outer_marker_gap}"
    );
    assert!(
        (nested_marker_gap - 8.0).abs() <= 0.25,
        "nested marker/body gap mismatch: {nested_marker_gap}"
    );
    assert!(
        nested_start_x[0] > outer_start_x[0],
        "nested body should shift right: outer={}, nested={}",
        outer_start_x[0],
        nested_start_x[0]
    );
    assert!(
        (nested_wrap_x[0] - nested_start_x[0]).abs() <= epsilon_pt,
        "nested continuation x mismatch: nested={}, wrap={}",
        nested_start_x[0],
        nested_wrap_x[0]
    );
    assert!(
        (outer_start_y - outer_wrap_y - 13.0).abs() <= epsilon_pt,
        "outer wrapped list rhythm mismatch: outer_start_y={outer_start_y}, outer_wrap_y={outer_wrap_y}"
    );
    assert!(
        (nested_start_y - nested_wrap_y - 13.0).abs() <= epsilon_pt,
        "nested wrapped list rhythm mismatch: nested_start_y={nested_start_y}, nested_wrap_y={nested_wrap_y}"
    );
}

#[test]
fn pdf_renderer_applies_quote_indent_and_hides_prefix_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"\n> quoted line\ncontinuation line")
        .expect("writer should accept text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    assert!(!pdf
        .windows(b"(> quoted line) Tj".len())
        .any(|w| w == b"(> quoted line) Tj"));
    assert!(pdf
        .windows(b"(quoted line) Tj".len())
        .any(|w| w == b"(quoted line) Tj"));

    let pdf_text = String::from_utf8_lossy(&pdf);
    let mut xs = Vec::<f32>::new();
    for line in pdf_text.lines() {
        if !line.contains(" Tm ") {
            continue;
        }
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() < 7 || fields[6] != "Tm" {
            continue;
        }
        if let Ok(x_pt) = fields[4].parse::<f32>() {
            xs.push(x_pt);
        }
    }
    assert!(
        xs.len() >= 2,
        "expected at least two rendered lines, got {xs:?}"
    );
    assert!(
        (xs[0] - 104.0).abs() <= 0.02,
        "quote line indent should be stable and deeper than body indent: {xs:?}"
    );
    assert!(
        (xs[0] - xs[1]).abs() <= 0.02,
        "quote continuation should keep indent: {xs:?}"
    );
}

#[test]
fn pdf_renderer_quote_indent_and_paragraph_break_invariants_v0() {
    let xdv = write_dvi_v2_text_page_v0(
        b"\n> quote first line\n> quote continuation\n\n> second paragraph line\n> second continuation",
    )
    .expect("writer should accept quote text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let text = String::from_utf8_lossy(&pdf);
    assert!(
        !text.contains("(> quote first line) Tj"),
        "quote prefix should be hidden"
    );
    let (x1, y1) =
        tm_position_for_line_containing_text_v0(&pdf, "(quote first line)").expect("line 1");
    let (x2, y2) =
        tm_position_for_line_containing_text_v0(&pdf, "(quote continuation)").expect("line 2");
    let (x3, y3) =
        tm_position_for_line_containing_text_v0(&pdf, "(second paragraph line)").expect("line 3");
    let (x4, y4) =
        tm_position_for_line_containing_text_v0(&pdf, "(second continuation)").expect("line 4");

    let epsilon_pt = 0.02f32;
    assert!(
        (x1 - 104.0).abs() <= epsilon_pt,
        "quote indent baseline mismatch: {x1}"
    );
    assert!(
        (x1 - x2).abs() <= epsilon_pt,
        "quote line x drift: {x1} vs {x2}"
    );
    assert!(
        (x1 - x3).abs() <= epsilon_pt,
        "quote paragraph x drift: {x1} vs {x3}"
    );
    assert!(
        (x1 - x4).abs() <= epsilon_pt,
        "quote line x drift: {x1} vs {x4}"
    );
    assert!(
        (y1 - y2 - 12.5).abs() <= epsilon_pt,
        "quote line gap mismatch"
    );
    assert!(
        (y2 - y3 - 26.5).abs() <= epsilon_pt,
        "quote paragraph gap mismatch"
    );
    assert!(
        (y3 - y4 - 12.5).abs() <= epsilon_pt,
        "quote line gap mismatch"
    );
}

#[test]
fn pdf_renderer_paragraph_quote_transition_spacing_polish_v7() {
    let xdv = write_dvi_v2_text_page_v0(
        b"\nParagraph before quote.\n\n> QUOTESTART quote quote quote quote quote quote quote quote quote quote quote quote QUOTEWRAP\n\nParagraph after quote.",
    )
    .expect("writer should accept paragraph-quote transition text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, before_quote_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Paragraph before quote.)")
            .expect("before quote paragraph");
    let (quote_start_x, quote_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "QUOTESTART").expect("quote start");
    let (quote_wrap_x, quote_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "QUOTEWRAP").expect("quote wrap");
    let (_, after_quote_y) = tm_position_for_line_containing_text_v0(&pdf, "(Paragraph after quote.)")
        .expect("after quote paragraph");

    let epsilon_pt = 0.02f32;
    assert!(
        (before_quote_y - quote_start_y - 23.0).abs() <= epsilon_pt,
        "paragraph->quote transition gap mismatch: before_quote_y={before_quote_y}, quote_start_y={quote_start_y}"
    );
    assert!(
        (quote_start_y - quote_wrap_y - 12.5).abs() <= epsilon_pt,
        "quote wrapped-line rhythm mismatch: quote_start_y={quote_start_y}, quote_wrap_y={quote_wrap_y}"
    );
    assert!(
        (quote_wrap_y - after_quote_y - 23.0).abs() <= epsilon_pt,
        "quote->paragraph transition gap mismatch: quote_wrap_y={quote_wrap_y}, after_quote_y={after_quote_y}"
    );
    assert!(
        (quote_start_x - quote_wrap_x).abs() <= epsilon_pt,
        "wrapped quote continuation should preserve quote indent: quote_start_x={quote_start_x}, quote_wrap_x={quote_wrap_x}"
    );
}

#[test]
fn pdf_renderer_mixed_paragraph_quote_list_transition_spacing_polish_v19() {
    let xdv = write_dvi_v2_text_page_v0(
        b"\nParagraph before quote.\n\n> QUOTESTART quote quote quote quote quote quote quote quote quote quote quote quote QUOTEWRAP\n\n- List after quote line.\n- List continuation line.\n\nParagraph after list.",
    )
    .expect("writer should accept mixed paragraph/quote/list transition text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, before_quote_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Paragraph before quote.)")
            .expect("before quote paragraph");
    let (quote_start_x, quote_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "QUOTESTART").expect("quote start");
    let (quote_wrap_x, quote_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "QUOTEWRAP").expect("quote wrap");
    let (list_start_x, list_start_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(List after quote line.)")
            .expect("list after quote");
    let (_, list_next_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(List continuation line.)")
            .expect("list continuation");
    let (_, after_list_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Paragraph after list.)")
            .expect("after list paragraph");

    let epsilon_pt = 0.05f32;
    assert!(
        (before_quote_y - quote_start_y - 23.0).abs() <= epsilon_pt,
        "paragraph->quote transition gap mismatch: before_quote_y={before_quote_y}, quote_start_y={quote_start_y}"
    );
    assert!(
        (quote_start_y - quote_wrap_y - 12.5).abs() <= epsilon_pt,
        "quote wrapped-line rhythm mismatch: quote_start_y={quote_start_y}, quote_wrap_y={quote_wrap_y}"
    );
    assert!(
        (quote_wrap_y - list_start_y - 23.0).abs() <= epsilon_pt,
        "quote->list transition gap mismatch: quote_wrap_y={quote_wrap_y}, list_start_y={list_start_y}"
    );
    assert!(
        (list_start_y - list_next_y - 13.0).abs() <= epsilon_pt,
        "list internal rhythm mismatch: list_start_y={list_start_y}, list_next_y={list_next_y}"
    );
    assert!(
        (list_next_y - after_list_y - 24.0).abs() <= epsilon_pt,
        "list->paragraph transition gap mismatch: list_next_y={list_next_y}, after_list_y={after_list_y}"
    );
    assert!(
        (quote_start_x - quote_wrap_x).abs() <= epsilon_pt,
        "wrapped quote continuation should preserve quote indent: quote_start_x={quote_start_x}, quote_wrap_x={quote_wrap_x}"
    );
    assert!(
        quote_start_x >= list_start_x + 6.0,
        "quote indent should remain visibly deeper than list body indent: quote_start_x={quote_start_x}, list_start_x={list_start_x}"
    );
}

#[test]
fn pdf_renderer_hides_center_prefix_and_centers_line_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"\n^ centered line")
        .expect("writer should accept centered text");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    let centered_line = layout.pages[0]
        .lines
        .iter()
        .find(|line| width_sp_for_prefixed_rendered_line_v0(line, [b'^', b' ']).is_some())
        .expect("center-prefixed line");
    let expected_x = expected_center_x_pt_v0(
        width_sp_for_prefixed_rendered_line_v0(centered_line, [b'^', b' '])
            .expect("prefixed width"),
    );
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    assert!(!pdf
        .windows(b"(^ centered line) Tj".len())
        .any(|w| w == b"(^ centered line) Tj"));
    assert!(pdf
        .windows(b"(centered line) Tj".len())
        .any(|w| w == b"(centered line) Tj"));
    let x_pt =
        tm_x_for_line_containing_text_v0(&pdf, "(centered line)").expect("centered Tm position");
    let epsilon_pt = 0.02f32;
    assert!(
        (x_pt - expected_x).abs() <= epsilon_pt,
        "center line x mismatch: actual={x_pt}, expected={expected_x}"
    );
}

#[test]
fn pdf_renderer_hides_right_prefix_and_right_aligns_line_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"\n| right aligned line")
        .expect("writer should accept right text");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    let right_line = layout.pages[0]
        .lines
        .iter()
        .find(|line| width_sp_for_prefixed_rendered_line_v0(line, [b'|', b' ']).is_some())
        .expect("right-prefixed line");
    let expected_x = expected_right_x_pt_v0(
        width_sp_for_prefixed_rendered_line_v0(right_line, [b'|', b' ']).expect("prefixed width"),
    );
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    assert!(!pdf
        .windows(b"(| right aligned line) Tj".len())
        .any(|w| w == b"(| right aligned line) Tj"));
    assert!(pdf
        .windows(b"(right aligned line) Tj".len())
        .any(|w| w == b"(right aligned line) Tj"));
    let x_pt = tm_x_for_line_containing_text_v0(&pdf, "(right aligned line)")
        .expect("right-aligned Tm position");
    let epsilon_pt = 0.02f32;
    assert!(
        (x_pt - expected_x).abs() <= epsilon_pt,
        "right line x mismatch: actual={x_pt}, expected={expected_x}"
    );
}

#[test]
fn pdf_renderer_applies_center_alignment_per_line_width_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"\n^ center one\n^ center line two")
        .expect("writer should accept centered lines");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    let line_one = layout.pages[0]
        .lines
        .iter()
        .find(|line| {
            line.glyphs
                .iter()
                .map(|glyph| glyph.byte)
                .collect::<Vec<_>>()
                == b"^ center one"
        })
        .expect("center line one");
    let line_two = layout.pages[0]
        .lines
        .iter()
        .find(|line| {
            line.glyphs
                .iter()
                .map(|glyph| glyph.byte)
                .collect::<Vec<_>>()
                == b"^ center line two"
        })
        .expect("center line two");

    let expected_one = expected_center_x_pt_v0(
        width_sp_for_prefixed_rendered_line_v0(line_one, [b'^', b' ']).expect("line one width"),
    );
    let expected_two = expected_center_x_pt_v0(
        width_sp_for_prefixed_rendered_line_v0(line_two, [b'^', b' ']).expect("line two width"),
    );

    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let x_one = tm_x_for_line_containing_text_v0(&pdf, "(center one)").expect("center one x");
    let x_two =
        tm_x_for_line_containing_text_v0(&pdf, "(center line two)").expect("center line two x");
    let epsilon_pt = 0.02f32;
    assert!((x_one - expected_one).abs() <= epsilon_pt);
    assert!((x_two - expected_two).abs() <= epsilon_pt);
}

#[test]
fn pdf_renderer_applies_right_alignment_per_line_width_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"\n| right one\n| right line two")
        .expect("writer should accept right-aligned lines");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    let line_one = layout.pages[0]
        .lines
        .iter()
        .find(|line| {
            line.glyphs
                .iter()
                .map(|glyph| glyph.byte)
                .collect::<Vec<_>>()
                == b"| right one"
        })
        .expect("right line one");
    let line_two = layout.pages[0]
        .lines
        .iter()
        .find(|line| {
            line.glyphs
                .iter()
                .map(|glyph| glyph.byte)
                .collect::<Vec<_>>()
                == b"| right line two"
        })
        .expect("right line two");

    let expected_one = expected_right_x_pt_v0(
        width_sp_for_prefixed_rendered_line_v0(line_one, [b'|', b' ']).expect("line one width"),
    );
    let expected_two = expected_right_x_pt_v0(
        width_sp_for_prefixed_rendered_line_v0(line_two, [b'|', b' ']).expect("line two width"),
    );

    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let x_one = tm_x_for_line_containing_text_v0(&pdf, "(right one)").expect("right one x");
    let x_two =
        tm_x_for_line_containing_text_v0(&pdf, "(right line two)").expect("right line two x");
    let epsilon_pt = 0.02f32;
    assert!((x_one - expected_one).abs() <= epsilon_pt);
    assert!((x_two - expected_two).abs() <= epsilon_pt);
}

#[test]
fn pdf_renderer_center_alignment_handles_styled_segments_without_drift_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"\n^ alpha[mid],gamma\n^ short{bold}.")
        .expect("writer should accept styled centered lines");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    let line_one = layout.pages[0]
        .lines
        .iter()
        .find(|line| {
            line.glyphs
                .iter()
                .map(|glyph| glyph.byte)
                .collect::<Vec<_>>()
                == b"^ alpha[mid],gamma"
        })
        .expect("center styled line one");
    let line_two = layout.pages[0]
        .lines
        .iter()
        .find(|line| {
            line.glyphs
                .iter()
                .map(|glyph| glyph.byte)
                .collect::<Vec<_>>()
                == b"^ short{bold}."
        })
        .expect("center styled line two");

    let expected_one = expected_center_x_pt_v0(
        width_sp_for_prefixed_rendered_line_v0(line_one, [b'^', b' ']).expect("line one width"),
    );
    let expected_two = expected_center_x_pt_v0(
        width_sp_for_prefixed_rendered_line_v0(line_two, [b'^', b' ']).expect("line two width"),
    );

    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let line_one_x = tm_x_for_line_containing_text_v0(&pdf, "(alpha)").expect("line one x");
    let line_two_x = tm_x_for_line_containing_text_v0(&pdf, "(short)").expect("line two x");
    let epsilon_pt = 0.02f32;
    assert!(
        (line_one_x - expected_one).abs() <= epsilon_pt,
        "line one center drift: actual={line_one_x}, expected={expected_one}"
    );
    assert!(
        (line_two_x - expected_two).abs() <= epsilon_pt,
        "line two center drift: actual={line_two_x}, expected={expected_two}"
    );

    let alpha_x = tm_xs_for_segment_text_v0(&pdf, "alpha")[0];
    let mid_x = tm_xs_for_segment_text_v0(&pdf, "mid")[0];
    let gamma_x =
        tm_x_for_segment_substring_v0(&pdf, "(alpha)", "(,gamma)").expect("gamma segment x");
    assert!(
        ((mid_x - alpha_x) - segment_width_pt_v0(b"alpha")).abs() <= epsilon_pt,
        "alpha->mid spacing drift: alpha_x={alpha_x}, mid_x={mid_x}"
    );
    assert!(
        ((gamma_x - mid_x) - segment_width_pt_v0(b"mid")).abs() <= epsilon_pt,
        "mid->gamma spacing drift: mid_x={mid_x}, gamma_x={gamma_x}"
    );
}

#[test]
fn pdf_renderer_right_alignment_handles_styled_segments_without_drift_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"\n| edge, [core] trail\n| alpha{beta}.")
        .expect("writer should accept styled right-aligned lines");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    let line_one = layout.pages[0]
        .lines
        .iter()
        .find(|line| {
            line.glyphs
                .iter()
                .map(|glyph| glyph.byte)
                .collect::<Vec<_>>()
                == b"| edge, [core] trail"
        })
        .expect("right styled line one");
    let line_two = layout.pages[0]
        .lines
        .iter()
        .find(|line| {
            line.glyphs
                .iter()
                .map(|glyph| glyph.byte)
                .collect::<Vec<_>>()
                == b"| alpha{beta}."
        })
        .expect("right styled line two");

    let expected_one = expected_right_x_pt_v0(
        width_sp_for_prefixed_rendered_line_v0(line_one, [b'|', b' ']).expect("line one width"),
    );
    let expected_two = expected_right_x_pt_v0(
        width_sp_for_prefixed_rendered_line_v0(line_two, [b'|', b' ']).expect("line two width"),
    );

    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let line_one_x = tm_x_for_line_containing_text_v0(&pdf, "(edge, )").expect("line one x");
    let line_two_x = tm_x_for_line_containing_text_v0(&pdf, "(alpha)").expect("line two x");
    let epsilon_pt = 0.02f32;
    assert!(
        (line_one_x - expected_one).abs() <= epsilon_pt,
        "line one right drift: actual={line_one_x}, expected={expected_one}"
    );
    assert!(
        (line_two_x - expected_two).abs() <= epsilon_pt,
        "line two right drift: actual={line_two_x}, expected={expected_two}"
    );

    let edge_x =
        tm_x_for_segment_substring_v0(&pdf, "(edge, )", "(edge, )").expect("edge segment x");
    let core_x = tm_xs_for_segment_text_v0(&pdf, "core")[0];
    let trail_x =
        tm_x_for_segment_substring_v0(&pdf, "(edge, )", "( trail)").expect("trail segment x");
    assert!(
        ((core_x - edge_x) - segment_width_pt_v0(b"edge, ")).abs() <= epsilon_pt,
        "edge->core spacing drift: edge_x={edge_x}, core_x={core_x}"
    );
    assert!(
        ((trail_x - core_x) - segment_width_pt_v0(b"core")).abs() <= epsilon_pt,
        "core->trail spacing drift: core_x={core_x}, trail_x={trail_x}"
    );
}

#[test]
fn pdf_renderer_center_alignment_keeps_wrapped_continuation_centered_v1() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTERSTART alpha [mid] gamma words words words words WRAPCENTER tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered text");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    let expected_start_width_pt =
        segment_width_pt_v0(b"CENTERSTART alpha ") + scaled_segment_width_pt_v0(b"mid", 97);
    let expected_start_x =
        (612.0 - expected_start_width_pt) / 2.0;
    let expected_wrap_x = expected_center_x_pt_v0(
        layout_render_width_for_substring_v0(&layout, b"WRAPCENTER").expect("center wrap width"),
    );

    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let (start_x, start_y) = tm_position_for_line_containing_text_v0(&pdf, "CENTERSTART")
        .expect("center start line position");
    let (wrap_x, wrap_y) =
        tm_position_for_line_containing_text_v0(&pdf, "WRAPCENTER").expect("center wrap line position");
    let epsilon_pt = 0.02f32;
    assert!(
        (start_x - expected_start_x).abs() <= epsilon_pt,
        "center wrapped first line drift: actual={start_x}, expected={expected_start_x}"
    );
    assert!(
        (wrap_x - expected_wrap_x).abs() <= epsilon_pt,
        "center wrapped continuation drift: actual={wrap_x}, expected={expected_wrap_x}"
    );
    assert!(start_y > wrap_y, "wrapped continuation should render below first line");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "CENTERSTART")
        .expect("center wrapped styled line should decode");
    assert!(
        rendered == "CENTERSTART alpha mid",
        "center wrapped style boundaries should retain stable spacing: {rendered}"
    );
    let pdf_text = String::from_utf8_lossy(&pdf);
    let centered_line = pdf_text
        .lines()
        .find(|line| line.contains("(mid) Tj"))
        .expect("center wrapped styled segment should render");
    assert!(
        centered_line.contains("97 Tz") && centered_line.contains("(mid) Tj 100 Tz"),
        "wrapped centered styled segment should use v28 seam compensation"
    );
}

#[test]
fn pdf_renderer_right_alignment_keeps_wrapped_continuation_right_v1() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHTSTART edge, [core] trail words words words words WRAPRIGHT tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right-aligned text");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    let expected_start_width_pt =
        segment_width_pt_v0(b"RIGHTSTART edge, ") + scaled_segment_width_pt_v0(b"core", 97);
    let expected_start_x = 540.0 - expected_start_width_pt;
    let expected_wrap_x = expected_right_x_pt_v0(
        layout_render_width_for_substring_v0(&layout, b"WRAPRIGHT").expect("right wrap width"),
    );

    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let (start_x, start_y) = tm_position_for_line_containing_text_v0(&pdf, "RIGHTSTART")
        .expect("right start line position");
    let (wrap_x, wrap_y) =
        tm_position_for_line_containing_text_v0(&pdf, "WRAPRIGHT").expect("right wrap line position");
    let epsilon_pt = 0.02f32;
    assert!(
        (start_x - expected_start_x).abs() <= epsilon_pt,
        "right wrapped first line drift: actual={start_x}, expected={expected_start_x}"
    );
    assert!(
        (wrap_x - expected_wrap_x).abs() <= epsilon_pt,
        "right wrapped continuation drift: actual={wrap_x}, expected={expected_wrap_x}"
    );
    assert!(start_y > wrap_y, "wrapped continuation should render below first line");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "RIGHTSTART")
        .expect("right wrapped styled line should decode");
    assert!(
        rendered == "RIGHTSTART edge, core",
        "right wrapped style boundaries should retain stable spacing: {rendered}"
    );
    let pdf_text = String::from_utf8_lossy(&pdf);
    let right_line = pdf_text
        .lines()
        .find(|line| line.contains("(core) Tj"))
        .expect("right wrapped styled segment should render");
    assert!(
        right_line.contains("97 Tz") && right_line.contains("(core) Tj 100 Tz"),
        "wrapped right styled segment should use v28 seam compensation"
    );
}

#[test]
fn pdf_renderer_wrapped_quote_and_list_styled_seams_use_v29_profile() {
    let xdv = write_dvi_v2_text_page_v0(
        b"\n- LISTSTART alpha alpha alpha alpha alpha alpha alpha [LISTITALICV29] beta beta beta beta beta beta LISTWRAPV29.\n\n> QUOTESTART gamma gamma gamma gamma gamma gamma gamma {QUOTEBOLDV29} delta delta delta delta delta QUOTEWRAPV29.",
    )
    .expect("writer should accept wrapped quote/list text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    let list_line = pdf_text
        .lines()
        .find(|line| line.contains("(LISTITALICV29) Tj"))
        .expect("wrapped list styled line should render");
    let quote_line = pdf_text
        .lines()
        .find(|line| line.contains("(QUOTEBOLDV29) Tj"))
        .expect("wrapped quote styled line should render");

    assert!(
        list_line.contains("97 Tz") && list_line.contains("(LISTITALICV29) Tj 100 Tz"),
        "wrapped list styled segment should use v29 seam compensation"
    );
    assert!(
        quote_line.contains("95 Tz") && quote_line.contains("(QUOTEBOLDV29) Tj 100 Tz"),
        "wrapped quote styled segment should use v29 seam compensation"
    );
    let (_, list_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "LISTSTART").expect("wrapped list start");
    let (_, list_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "LISTWRAPV29").expect("wrapped list wrap");
    let (_, quote_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "QUOTESTART").expect("wrapped quote start");
    let (_, quote_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "QUOTEWRAPV29").expect("wrapped quote wrap");
    assert!(
        list_start_y > list_wrap_y,
        "list fixture should wrap onto a later line: list_start_y={list_start_y}, list_wrap_y={list_wrap_y}"
    );
    assert!(
        quote_start_y > quote_wrap_y,
        "quote fixture should wrap onto a later line: quote_start_y={quote_start_y}, quote_wrap_y={quote_wrap_y}"
    );
}

#[test]
fn pdf_renderer_single_line_quote_and_list_styled_seams_use_v31_profile() {
    let xdv = write_dvi_v2_text_page_v0(
        b"\n- LISTLINEV31 alpha [LISTITALICV31] tail.\n\n> QUOTELINEV31 beta {QUOTEBOLDV31} tail.",
    )
    .expect("writer should accept single-line quote/list text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    let list_line = pdf_text
        .lines()
        .find(|line| line.contains("(LISTITALICV31) Tj"))
        .expect("single-line list styled segment should render");
    let quote_line = pdf_text
        .lines()
        .find(|line| line.contains("(QUOTEBOLDV31) Tj"))
        .expect("single-line quote styled segment should render");

    assert!(
        list_line.contains("97 Tz") && list_line.contains("(LISTITALICV31) Tj 100 Tz"),
        "single-line list styled segment should use indented seam compensation"
    );
    assert!(
        quote_line.contains("95 Tz") && quote_line.contains("(QUOTEBOLDV31) Tj 100 Tz"),
        "single-line quote styled segment should use indented seam compensation"
    );
}
