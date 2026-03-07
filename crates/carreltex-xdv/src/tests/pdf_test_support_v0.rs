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

