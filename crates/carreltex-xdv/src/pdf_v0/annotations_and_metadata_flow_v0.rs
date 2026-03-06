fn collect_link_annotations_for_line_v0(
    segments: &[PdfRenderSegmentV0],
    line_x_pt: f32,
    line_y_pt: f32,
    font_size_pt: f32,
    link_targets_by_id: &BTreeMap<u32, PdfLinkTargetV0>,
    next_link_id: &mut u32,
    active_link_target: &mut Option<PdfLinkTargetV0>,
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
                if active_link_target.is_none() {
                    let target = link_targets_by_id.get(next_link_id)?.clone();
                    *next_link_id = next_link_id.checked_add(1)?;
                    *active_link_target = Some(target);
                }
            }
            run_end_x = end_x;
        } else if let Some(run_x) = run_start_x.take() {
            let target = active_link_target.clone()?;
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
            annotations.push(PdfLinkAnnotationV0 { target, rect });
            *active_link_target = None;
        }
        cursor_x = end_x;
    }

    if let Some(run_x) = run_start_x.take() {
        let target = active_link_target.clone()?;
        if run_end_x <= run_x {
            if link_active_at_line_end {
                return Some(annotations);
            }
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
        annotations.push(PdfLinkAnnotationV0 { target, rect });
        if !link_active_at_line_end {
            *active_link_target = None;
        }
    } else if !link_active_at_line_end {
        *active_link_target = None;
    }

    Some(annotations)
}

fn collect_toc_link_annotations_for_line_v0(
    segments: &[PdfRenderSegmentV0],
    line_x_pt: f32,
    line_y_pt: f32,
    font_size_pt: f32,
    target: PdfLinkTargetV0,
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
            annotations.push(PdfLinkAnnotationV0 {
                target: target.clone(),
                rect,
            });
        }
        cursor_x = end_x;
    }

    if let Some(run_x) = run_start_x {
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
        annotations.push(PdfLinkAnnotationV0 { target, rect });
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

fn split_body_and_metadata_lines_v0(
    pages: &[PagePlanV0],
) -> (Vec<Vec<LinePlanV0>>, Vec<LinePlanV0>) {
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
                || has_pageref_line_prefix_v0(&line.glyphs)
                || has_ref_anchor_link_line_prefix_v0(&line.glyphs)
                || has_pageref_page_link_line_prefix_v0(&line.glyphs)
                || has_equation_line_prefix_v0(&line.glyphs)
                || has_bibitem_line_prefix_v0(&line.glyphs)
                || has_cite_line_prefix_v0(&line.glyphs)
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
        let should_pop = body_pages
            .last()
            .map(|lines| lines.is_empty())
            .unwrap_or(false);
        if !should_pop {
            break;
        }
        body_pages.pop();
    }
    (body_pages, metadata_lines)
}

fn collect_nominal_anchor_destinations_v0(
    body_pages: &[Vec<LinePlanV0>],
) -> Option<BTreeMap<u32, AnchorDestinationV0>> {
    let mut anchor_destinations = BTreeMap::<u32, AnchorDestinationV0>::new();
    let mut next_anchor_id = 1u32;
    for (page_index, lines) in body_pages.iter().enumerate() {
        let title_block_len = if page_index == 0 {
            detect_title_block_len_v0(lines)
        } else {
            0
        };
        let mut y = PAGE_HEIGHT_PT_V0 - MARGIN_PT_V0 - TITLE_FONT_SIZE_PT_V0;
        for (line_index, line) in lines.iter().enumerate() {
            if line_index >= title_block_len && has_figure_box_marker_prefix_v0(&line.glyphs) {
                parse_figure_box_line_v0(&line.glyphs)?;
                if anchor_destinations
                    .insert(
                        next_anchor_id,
                        AnchorDestinationV0 {
                            page_index,
                            y_pt: y,
                        },
                    )
                    .is_some()
                {
                    return None;
                }
                next_anchor_id = next_anchor_id.checked_add(1)?;
            } else if line_index >= title_block_len
                && is_display_math_placeholder_line_v0(&line.glyphs)
            {
                if anchor_destinations
                    .insert(
                        next_anchor_id,
                        AnchorDestinationV0 {
                            page_index,
                            y_pt: y,
                        },
                    )
                    .is_some()
                {
                    return None;
                }
                next_anchor_id = next_anchor_id.checked_add(1)?;
            } else if line_index >= title_block_len
                && detect_heading_prefix_v0(&line.glyphs).is_some()
            {
                if anchor_destinations
                    .insert(
                        next_anchor_id,
                        AnchorDestinationV0 {
                            page_index,
                            y_pt: y,
                        },
                    )
                    .is_some()
                {
                    return None;
                }
                next_anchor_id = next_anchor_id.checked_add(1)?;
            }
            y -= LEADING_PT_V0;
            if title_block_len > 0 && line_index + 1 == title_block_len {
                y -= TITLE_EXTRA_GAP_PT_V0;
            }
        }
    }
    Some(anchor_destinations)
}

fn parse_metadata_lines_v0(
    metadata_lines: &[LinePlanV0],
) -> Option<(
    BTreeMap<u32, Vec<Vec<GlyphPlanV0>>>,
    BTreeMap<u32, PdfLinkTargetV0>,
    Vec<TocEntryMetadataV0>,
    BTreeMap<u32, u32>,
)> {
    let mut footnote_defs_by_id = BTreeMap::<u32, Vec<Vec<GlyphPlanV0>>>::new();
    let mut link_targets_by_id = BTreeMap::<u32, PdfLinkTargetV0>::new();
    let mut toc_entries = Vec::<TocEntryMetadataV0>::new();
    let mut equation_ordinals_by_anchor_id = BTreeMap::<u32, u32>::new();
    let mut current_footnote_id = None::<u32>;
    for line in metadata_lines {
        if let Some((footnote_id, footnote_line)) = parse_footnote_definition_line_v0(&line.glyphs)
        {
            if footnote_defs_by_id.contains_key(&footnote_id) {
                return None;
            }
            footnote_defs_by_id.insert(footnote_id, vec![footnote_line]);
            current_footnote_id = Some(footnote_id);
            continue;
        }
        if let Some((href_id, href_url)) = parse_href_url_line_v0(&line.glyphs) {
            if link_targets_by_id
                .insert(href_id, PdfLinkTargetV0::Uri(href_url))
                .is_some()
            {
                return None;
            }
            current_footnote_id = None;
            continue;
        }
        if let Some(ref_link) = parse_ref_anchor_link_line_v0(&line.glyphs) {
            if link_targets_by_id
                .insert(
                    ref_link.link_id,
                    PdfLinkTargetV0::Anchor(ref_link.anchor_id),
                )
                .is_some()
            {
                return None;
            }
            current_footnote_id = None;
            continue;
        }
        if let Some(pageref_link) = parse_pageref_page_link_line_v0(&line.glyphs) {
            if link_targets_by_id
                .insert(
                    pageref_link.link_id,
                    PdfLinkTargetV0::AnchorPage(pageref_link.anchor_id),
                )
                .is_some()
            {
                return None;
            }
            current_footnote_id = None;
            continue;
        }
        if let Some(equation) = parse_equation_line_v0(&line.glyphs) {
            if equation_ordinals_by_anchor_id
                .insert(equation.anchor_id, equation.ordinal)
                .is_some()
            {
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
        if parse_pageref_line_v0(&line.glyphs).is_some() {
            current_footnote_id = None;
            continue;
        }
        if parse_bibitem_line_v0(&line.glyphs).is_some() {
            current_footnote_id = None;
            continue;
        }
        if parse_cite_line_v0(&line.glyphs).is_some() {
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
        footnote_defs_by_id
            .get_mut(&footnote_id)?
            .push(line.glyphs.clone());
    }
    toc_entries.sort_by_key(|entry| entry.anchor_id);
    Some((
        footnote_defs_by_id,
        link_targets_by_id,
        toc_entries,
        equation_ordinals_by_anchor_id,
    ))
}

