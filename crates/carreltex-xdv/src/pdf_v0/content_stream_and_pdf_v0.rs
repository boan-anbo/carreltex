fn build_page_content_stream_v0(
    lines: &[LinePlanV0],
    footnote_defs_by_id: &BTreeMap<u32, Vec<Vec<GlyphPlanV0>>>,
    link_targets_by_id: &BTreeMap<u32, PdfLinkTargetV0>,
    page_numbers_by_anchor_id: &BTreeMap<u32, u32>,
    page_count: usize,
    toc_entries: &[TocEntryMetadataV0],
    equation_ordinals_by_anchor_id: &BTreeMap<u32, u32>,
    next_link_id: &mut u32,
    page_index: usize,
    next_anchor_id: &mut u32,
    anchor_destinations: &mut BTreeMap<u32, AnchorDestinationV0>,
    allow_title_block: bool,
) -> Option<PageRenderV0> {
    let mut out = Vec::new();
    out.extend_from_slice(b"BT\n");
    out.extend_from_slice(b"0 g\n");

    let page_footnote_ids = collect_page_footnote_marker_ids_v0(lines)?;
    let mut footnote_line_count = 0usize;
    for footnote_id in &page_footnote_ids {
        let Some(footnote_lines) = footnote_defs_by_id.get(footnote_id) else {
            return None;
        };
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
    let mut active_link_target = None::<PdfLinkTargetV0>;
    let mut line_index = 0usize;
    while line_index < lines.len() {
        let line = &lines[line_index];
        let resolved_line_glyphs =
            replace_pageref_render_markers_v0(&line.glyphs, page_numbers_by_anchor_id)?;
        if y < MARGIN_PT_V0 {
            break;
        }
        if y < min_body_y_pt {
            return None;
        }
        if line_index >= title_block_len
            && (has_table_spec_prefix_v0(&resolved_line_glyphs)
                || has_table_row_prefix_v0(&resolved_line_glyphs))
        {
            let mut cursor = line_index;
            if !has_table_spec_prefix_v0(&resolved_line_glyphs) {
                return None;
            }
            let align_spec = parse_table_align_spec_line_v0(&resolved_line_glyphs)?;
            cursor += 1;
            let mut table_end = cursor;
            while table_end < lines.len() && has_table_row_prefix_v0(&lines[table_end].glyphs) {
                table_end += 1;
            }
            if cursor == table_end {
                return None;
            }
            emit_table_block_v0(
                &mut out,
                &align_spec,
                &lines[cursor..table_end],
                &mut y,
                min_body_y_pt,
            )?;
            previous_rendered_line_was_empty = false;
            skip_indent_after_title_block = false;
            active_hang_indent_pt = 0.0;
            active_quote_indent_pt = 0.0;
            line_index = table_end;
            continue;
        }
        if line_index >= title_block_len && has_figure_box_marker_prefix_v0(&resolved_line_glyphs)
        {
            let _figure_placement = parse_figure_box_line_v0(&resolved_line_glyphs)?;
            let figure_anchor_id = *next_anchor_id;
            *next_anchor_id = next_anchor_id.checked_add(1)?;
            if anchor_destinations
                .insert(
                    figure_anchor_id,
                    AnchorDestinationV0 {
                        page_index,
                        y_pt: y,
                    },
                )
                .is_some()
            {
                return None;
            }
            let mut cursor = line_index + 1;
            let mut image_metadata: Option<FigureImageMetadataV0> = None;
            if let Some(image_line) = lines.get(cursor) {
                if has_figure_image_prefix_v0(&image_line.glyphs) {
                    let parsed_image = parse_figure_image_line_v0(&image_line.glyphs)?;
                    image_metadata = Some(parsed_image);
                    cursor += 1;
                }
            }
            let caption_line = lines.get(cursor)?;
            let caption_glyphs = parse_figure_caption_line_v0(&caption_line.glyphs)?;
            emit_figure_block_v0(
                &mut out,
                image_metadata.as_ref(),
                &caption_glyphs,
                &mut y,
                min_body_y_pt,
            )?;
            previous_rendered_line_was_empty = false;
            skip_indent_after_title_block = false;
            active_hang_indent_pt = 0.0;
            active_quote_indent_pt = 0.0;
            line_index = cursor + 1;
            continue;
        }
        if line_index >= title_block_len && has_figure_image_prefix_v0(&resolved_line_glyphs) {
            return None;
        }
        if line_index >= title_block_len && has_figure_caption_prefix_v0(&resolved_line_glyphs) {
            return None;
        }
        if line_index >= title_block_len && has_toc_placeholder_line_v0(&resolved_line_glyphs) {
            emit_toc_block_v0(
                &mut out,
                toc_entries,
                page_numbers_by_anchor_id,
                page_count,
                &mut y,
                min_body_y_pt,
                &mut annotations,
            )?;
            previous_rendered_line_was_empty = false;
            skip_indent_after_title_block = false;
            active_hang_indent_pt = 0.0;
            active_quote_indent_pt = 0.0;
            line_index += 1;
            continue;
        }
        if line_index >= title_block_len && has_toc_entry_line_prefix_v0(&resolved_line_glyphs) {
            return None;
        }
        let quote_prefix_advance_pt = if line_index >= title_block_len {
            detect_quote_prefix_advance_pt_v0(&resolved_line_glyphs)
        } else {
            None
        };
        let center_prefixed = line_index >= title_block_len
            && quote_prefix_advance_pt.is_none()
            && has_center_prefix_v0(&resolved_line_glyphs);
        let right_prefixed = line_index >= title_block_len
            && quote_prefix_advance_pt.is_none()
            && !center_prefixed
            && has_right_prefix_v0(&resolved_line_glyphs);
        let noindent_prefixed = line_index >= title_block_len
            && quote_prefix_advance_pt.is_none()
            && !center_prefixed
            && !right_prefixed
            && has_noindent_prefix_v0(&resolved_line_glyphs);
        let heading_kind = if line_index >= title_block_len
            && quote_prefix_advance_pt.is_none()
            && !center_prefixed
            && !right_prefixed
            && !noindent_prefixed
        {
            detect_heading_prefix_v0(&resolved_line_glyphs)
        } else {
            None
        };
        let display_math_line = line_index >= title_block_len
            && quote_prefix_advance_pt.is_none()
            && center_prefixed
            && is_display_math_placeholder_line_v0(&resolved_line_glyphs);
        let render_glyphs_base: &[GlyphPlanV0] = if quote_prefix_advance_pt.is_some() {
            &resolved_line_glyphs[2..]
        } else if center_prefixed {
            &resolved_line_glyphs[2..]
        } else if right_prefixed {
            &resolved_line_glyphs[2..]
        } else if noindent_prefixed {
            &resolved_line_glyphs[2..]
        } else if let Some(kind) = heading_kind {
            &resolved_line_glyphs[heading_prefix_len_v0(kind)..]
        } else {
            &resolved_line_glyphs
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
        ) else {
            return None;
        };
        let render_segments = split_superscript_segments_v0(&segments);
        let line_is_empty = render_segments.is_empty();
        // Collapse consecutive blank lines to a single rhythm gap so vertical spacing stays stable.
        if line_is_empty && previous_rendered_line_was_empty {
            line_index += 1;
            continue;
        }
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
            let line_width_pt: f32 = render_segments
                .iter()
                .map(|segment| segment.advance_pt)
                .sum();
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
            let mut equation_ordinal = None::<u32>;
            if heading_kind.is_some() || display_math_line {
                let anchor_id = *next_anchor_id;
                *next_anchor_id = next_anchor_id.checked_add(1)?;
                if anchor_destinations
                    .insert(
                        anchor_id,
                        AnchorDestinationV0 {
                            page_index,
                            y_pt: y,
                        },
                    )
                    .is_some()
                {
                    return None;
                }
                if display_math_line {
                    equation_ordinal = equation_ordinals_by_anchor_id.get(&anchor_id).copied();
                }
            }
            if let Some(prefix) = list_prefix {
                let display_prefix_glyphs = &render_glyphs_base
                    [prefix.display_start..prefix.display_start + prefix.display_len];
                let Some(display_prefix_segments) = parse_styled_segments_v0(display_prefix_glyphs)
                else {
                    return None;
                };
                let prefix_width_pt = glyphs_advance_pt_v0(display_prefix_glyphs);
                let prefix_x = match prefix.kind {
                    ListPrefixKindV0::Itemize => MARGIN_PT_V0 + prefix.leading_advance_pt,
                    ListPrefixKindV0::Enumerate => {
                        ENUM_NUMBER_COLUMN_RIGHT_PT_V0 + prefix.leading_advance_pt - prefix_width_pt
                    }
                };
                emit_styled_segments_v0(
                    &mut out,
                    &display_prefix_segments,
                    prefix_x,
                    y,
                    font_size_pt,
                );
            }
            let mut line_annotations = collect_link_annotations_for_line_v0(
                &render_segments,
                line_x,
                y,
                font_size_pt,
                link_targets_by_id,
                next_link_id,
                &mut active_link_target,
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
            if let Some(ordinal) = equation_ordinal {
                let equation_number = format!("({ordinal})").into_bytes();
                let equation_number_width_pt =
                    (equation_number.len() as f32) * (FONT_SIZE_PT_V0 * 0.6);
                let equation_number_x =
                    (PAGE_WIDTH_PT_V0 - MARGIN_PT_V0 - equation_number_width_pt).max(MARGIN_PT_V0);
                let equation_segments = [PdfRenderSegmentV0 {
                    style: PdfTextStyleV0::Regular,
                    bytes: equation_number,
                    advance_pt: equation_number_width_pt,
                    is_link: false,
                    superscript: false,
                }];
                emit_render_segments_with_superscript_v0(
                    &mut out,
                    &equation_segments,
                    equation_number_x,
                    y,
                    FONT_SIZE_PT_V0,
                );
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
    if active_link_target.is_some() {
        return None;
    }

    if footnote_line_count > 0 {
        let mut footnote_y = MARGIN_PT_V0 + footnote_reserved_height_pt - FOOTNOTE_LEADING_PT_V0;
        for footnote_id in &page_footnote_ids {
            for footnote_line in footnote_defs_by_id.get(footnote_id)? {
                if footnote_y < MARGIN_PT_V0 {
                    return None;
                }
                let Some(segments) = parse_styled_segments_v0(footnote_line) else {
                    return None;
                };
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
    let Some((
        footnote_defs_by_id,
        link_targets_by_id,
        toc_entries,
        equation_ordinals_by_anchor_id,
    )) = parse_metadata_lines_v0(&metadata_lines)
    else {
        return Vec::new();
    };
    let Some(nominal_anchor_destinations) = collect_nominal_anchor_destinations_v0(&body_pages)
    else {
        return Vec::new();
    };
    let Some(page_numbers_by_anchor_id) =
        build_page_numbers_by_anchor_id_v0(&nominal_anchor_destinations)
    else {
        return Vec::new();
    };

    let mut next_link_id = 1u32;
    let mut next_anchor_id = 1u32;
    let mut anchor_destinations = BTreeMap::<u32, AnchorDestinationV0>::new();
    let mut page_renders = Vec::<PageRenderV0>::with_capacity(body_pages.len());
    for (page_index, body_lines) in body_pages.iter().enumerate() {
        let Some(rendered) = build_page_content_stream_v0(
            body_lines,
            &footnote_defs_by_id,
            &link_targets_by_id,
            &page_numbers_by_anchor_id,
            body_pages.len(),
            &toc_entries,
            &equation_ordinals_by_anchor_id,
            &mut next_link_id,
            page_index,
            &mut next_anchor_id,
            &mut anchor_destinations,
            page_index == 0,
        ) else {
            return Vec::new();
        };
        page_renders.push(rendered);
    }
    for (anchor_id, destination) in nominal_anchor_destinations {
        anchor_destinations.entry(anchor_id).or_insert(destination);
    }
    if usize::try_from(next_link_id.saturating_sub(1)).ok() != Some(link_targets_by_id.len()) {
        return Vec::new();
    }
    for target in link_targets_by_id.values() {
        match target {
            PdfLinkTargetV0::Anchor(anchor_id) | PdfLinkTargetV0::AnchorPage(anchor_id) => {
                if !anchor_destinations.contains_key(anchor_id) {
                    return Vec::new();
                }
            }
            PdfLinkTargetV0::Uri(_) => {}
        }
    }
    let Some(outline_items) = build_outline_items_plan_v0(&toc_entries) else {
        return Vec::new();
    };
    for item in &outline_items {
        if !anchor_destinations.contains_key(&item.anchor_id) {
            return Vec::new();
        }
    }

    // Object numbering:
    // 1: Catalog
    // 2: Pages
    // 3..(3+page_count-1): Page objects
    // (3+page_count)..(3+2*page_count-1): Content stream objects
    // next: annotation objects (if any)
    // next: annotation action objects (if any)
    // next: outline root + outline item objects
    // last: Font objects (regular/italic/bold)
    let page_count = page_renders.len() as u32;
    let total_annotations = page_renders
        .iter()
        .map(|page| u32::try_from(page.annotations.len()).unwrap_or(0))
        .sum::<u32>();
    let total_uri_annotations = page_renders
        .iter()
        .flat_map(|page| page.annotations.iter())
        .filter(|annotation| matches!(annotation.target, PdfLinkTargetV0::Uri(_)))
        .count() as u32;
    let first_page_id = 3u32;
    let first_stream_id = first_page_id + page_count;
    let first_annotation_id = first_stream_id + page_count;
    let first_action_id = first_annotation_id + total_annotations;
    let outline_root_id = first_action_id + total_uri_annotations;
    let outline_item_count = match u32::try_from(outline_items.len()) {
        Ok(value) => value,
        Err(_) => return Vec::new(),
    };
    let first_outline_item_id = outline_root_id + 1;
    let font_regular_id = first_outline_item_id + outline_item_count;
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
        format!("<< /Type /Catalog /Pages {pages_id} 0 R /Outlines {outline_root_id} 0 R >>")
            .as_bytes(),
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
    let mut action_counter = 0u32;
    for page in &page_renders {
        for annotation in &page.annotations {
            let annotation_id = first_annotation_id + annotation_counter;
            match &annotation.target {
                PdfLinkTargetV0::Uri(uri) => {
                    let action_id = first_action_id + action_counter;
                    action_counter += 1;
                    let annot_body = format!(
                        "<< /Type /Annot /Subtype /Link /Rect [{:.2} {:.2} {:.2} {:.2}] /Border [0 0 0] /A {action_id} 0 R >>",
                        annotation.rect[0],
                        annotation.rect[1],
                        annotation.rect[2],
                        annotation.rect[3],
                    );
                    offsets.push(write_pdf_obj(
                        &mut out,
                        annotation_id,
                        annot_body.as_bytes(),
                    ));
                    let mut action_body = Vec::<u8>::new();
                    action_body.extend_from_slice(b"<< /S /URI /URI (");
                    action_body.extend_from_slice(&escape_pdf_string_bytes(uri));
                    action_body.extend_from_slice(b") >>");
                    offsets.push(write_pdf_obj(&mut out, action_id, &action_body));
                }
                PdfLinkTargetV0::Anchor(anchor_id) => {
                    let Some(destination) = anchor_destinations.get(anchor_id).copied() else {
                        return Vec::new();
                    };
                    let destination_page_id =
                        first_page_id + u32::try_from(destination.page_index).unwrap_or(0);
                    if destination_page_id >= first_stream_id {
                        return Vec::new();
                    }
                    let annot_body = format!(
                        "<< /Type /Annot /Subtype /Link /Rect [{:.2} {:.2} {:.2} {:.2}] /Border [0 0 0] /Dest [{destination_page_id} 0 R /XYZ {:.2} {:.2} null] >>",
                        annotation.rect[0],
                        annotation.rect[1],
                        annotation.rect[2],
                        annotation.rect[3],
                        MARGIN_PT_V0,
                        destination.y_pt,
                    );
                    offsets.push(write_pdf_obj(
                        &mut out,
                        annotation_id,
                        annot_body.as_bytes(),
                    ));
                }
                PdfLinkTargetV0::AnchorPage(anchor_id) => {
                    let Some(destination) = anchor_destinations.get(anchor_id).copied() else {
                        return Vec::new();
                    };
                    let destination_page_id =
                        first_page_id + u32::try_from(destination.page_index).unwrap_or(0);
                    if destination_page_id >= first_stream_id {
                        return Vec::new();
                    }
                    let annot_body = format!(
                        "<< /Type /Annot /Subtype /Link /Rect [{:.2} {:.2} {:.2} {:.2}] /Border [0 0 0] /Dest [{destination_page_id} 0 R /Fit] >>",
                        annotation.rect[0],
                        annotation.rect[1],
                        annotation.rect[2],
                        annotation.rect[3],
                    );
                    offsets.push(write_pdf_obj(
                        &mut out,
                        annotation_id,
                        annot_body.as_bytes(),
                    ));
                }
            }
            annotation_counter += 1;
        }
    }
    if action_counter != total_uri_annotations {
        return Vec::new();
    }

    let outline_root_body = if outline_items.is_empty() {
        b"<< /Type /Outlines /Count 0 >>".to_vec()
    } else {
        let Some(root_first_index) = outline_items
            .iter()
            .enumerate()
            .find_map(|(index, item)| {
                (item.parent_index.is_none() && item.prev_sibling_index.is_none()).then_some(index)
            })
        else {
            return Vec::new();
        };
        let Some(root_last_index) = outline_items
            .iter()
            .enumerate()
            .find_map(|(index, item)| {
                (item.parent_index.is_none() && item.next_sibling_index.is_none()).then_some(index)
            })
        else {
            return Vec::new();
        };
        let root_first_id = match u32::try_from(root_first_index) {
            Ok(value) => first_outline_item_id + value,
            Err(_) => return Vec::new(),
        };
        let root_last_id = match u32::try_from(root_last_index) {
            Ok(value) => first_outline_item_id + value,
            Err(_) => return Vec::new(),
        };
        format!(
            "<< /Type /Outlines /First {root_first_id} 0 R /Last {root_last_id} 0 R /Count {outline_item_count} >>"
        )
        .into_bytes()
    };
    offsets.push(write_pdf_obj(&mut out, outline_root_id, &outline_root_body));

    for (index, item) in outline_items.iter().enumerate() {
        let item_id = match u32::try_from(index) {
            Ok(value) => first_outline_item_id + value,
            Err(_) => return Vec::new(),
        };
        let parent_id = if let Some(parent_index) = item.parent_index {
            match u32::try_from(parent_index) {
                Ok(value) => first_outline_item_id + value,
                Err(_) => return Vec::new(),
            }
        } else {
            outline_root_id
        };
        let Some(destination) = anchor_destinations.get(&item.anchor_id).copied() else {
            return Vec::new();
        };
        let destination_page_id = match u32::try_from(destination.page_index) {
            Ok(value) => first_page_id + value,
            Err(_) => return Vec::new(),
        };
        if destination_page_id >= first_stream_id {
            return Vec::new();
        }

        let mut body = Vec::<u8>::new();
        body.extend_from_slice(b"<< /Title (");
        body.extend_from_slice(&escape_pdf_string_bytes(&item.title_bytes));
        body.extend_from_slice(b") /Parent ");
        body.extend_from_slice(parent_id.to_string().as_bytes());
        body.extend_from_slice(b" 0 R");
        if let Some(prev_index) = item.prev_sibling_index {
            let prev_id = match u32::try_from(prev_index) {
                Ok(value) => first_outline_item_id + value,
                Err(_) => return Vec::new(),
            };
            body.extend_from_slice(b" /Prev ");
            body.extend_from_slice(prev_id.to_string().as_bytes());
            body.extend_from_slice(b" 0 R");
        }
        if let Some(next_index) = item.next_sibling_index {
            let next_id = match u32::try_from(next_index) {
                Ok(value) => first_outline_item_id + value,
                Err(_) => return Vec::new(),
            };
            body.extend_from_slice(b" /Next ");
            body.extend_from_slice(next_id.to_string().as_bytes());
            body.extend_from_slice(b" 0 R");
        }
        if let (Some(first_child_index), Some(last_child_index)) =
            (item.first_child_index, item.last_child_index)
        {
            let first_child_id = match u32::try_from(first_child_index) {
                Ok(value) => first_outline_item_id + value,
                Err(_) => return Vec::new(),
            };
            let last_child_id = match u32::try_from(last_child_index) {
                Ok(value) => first_outline_item_id + value,
                Err(_) => return Vec::new(),
            };
            body.extend_from_slice(b" /First ");
            body.extend_from_slice(first_child_id.to_string().as_bytes());
            body.extend_from_slice(b" 0 R /Last ");
            body.extend_from_slice(last_child_id.to_string().as_bytes());
            body.extend_from_slice(b" 0 R /Count ");
            body.extend_from_slice(item.child_count.to_string().as_bytes());
        } else if item.first_child_index.is_some() || item.last_child_index.is_some() {
            return Vec::new();
        }
        body.extend_from_slice(
            format!(
                " /Dest [{destination_page_id} 0 R /XYZ {:.2} {:.2} null] >>",
                MARGIN_PT_V0, destination.y_pt
            )
            .as_bytes(),
        );
        offsets.push(write_pdf_obj(&mut out, item_id, &body));
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

fn build_page_numbers_by_anchor_id_v0(
    anchor_destinations: &BTreeMap<u32, AnchorDestinationV0>,
) -> Option<BTreeMap<u32, u32>> {
    let mut out = BTreeMap::<u32, u32>::new();
    for (anchor_id, destination) in anchor_destinations {
        let page_no = u32::try_from(destination.page_index).ok()?.checked_add(1)?;
        if out.insert(*anchor_id, page_no).is_some() {
            return None;
        }
    }
    Some(out)
}

fn toc_entry_title_bytes_for_outline_v0(entry: &TocEntryMetadataV0) -> Option<Vec<u8>> {
    let segments = parse_styled_segments_v0(&entry.title_glyphs)?;
    let mut title_bytes = Vec::<u8>::new();
    for segment in segments {
        for glyph in segment.glyphs {
            if glyph.byte == NEWLINE_MARKER_V0 || glyph.byte == PAGE_BREAK_MARKER_V0 {
                return None;
            }
            if !(0x20..=0x7e).contains(&glyph.byte) {
                return None;
            }
            title_bytes.push(glyph.byte);
        }
    }
    while matches!(title_bytes.first(), Some(b' ')) {
        title_bytes.remove(0);
    }
    while matches!(title_bytes.last(), Some(b' ')) {
        title_bytes.pop();
    }
    if title_bytes.is_empty() {
        return None;
    }
    Some(title_bytes)
}

fn assign_outline_siblings_v0(indices: &[usize], items: &mut [OutlineItemPlanV0]) -> Option<()> {
    for (position, index) in indices.iter().copied().enumerate() {
        let prev = if position > 0 {
            Some(indices[position - 1])
        } else {
            None
        };
        let next = if position + 1 < indices.len() {
            Some(indices[position + 1])
        } else {
            None
        };
        let item = items.get_mut(index)?;
        item.prev_sibling_index = prev;
        item.next_sibling_index = next;
    }
    Some(())
}

fn build_outline_items_plan_v0(toc_entries: &[TocEntryMetadataV0]) -> Option<Vec<OutlineItemPlanV0>> {
    if toc_entries.is_empty() {
        return Some(Vec::new());
    }
    let mut items = Vec::<OutlineItemPlanV0>::with_capacity(toc_entries.len());
    let mut root_indices = Vec::<usize>::new();
    let mut current_level1_index = None::<usize>;

    for entry in toc_entries {
        if !(1..=2).contains(&entry.level) {
            return None;
        }
        let title_bytes = toc_entry_title_bytes_for_outline_v0(entry)?;
        let item_index = items.len();
        items.push(OutlineItemPlanV0 {
            anchor_id: entry.anchor_id,
            title_bytes,
            parent_index: None,
            first_child_index: None,
            last_child_index: None,
            prev_sibling_index: None,
            next_sibling_index: None,
            child_count: 0,
        });
        if entry.level == 1 {
            root_indices.push(item_index);
            current_level1_index = Some(item_index);
        } else {
            let parent_index = current_level1_index?;
            items[item_index].parent_index = Some(parent_index);
        }
    }

    if root_indices.is_empty() {
        return None;
    }

    assign_outline_siblings_v0(&root_indices, &mut items)?;
    for parent_index in root_indices {
        let child_indices = items
            .iter()
            .enumerate()
            .filter_map(|(index, item)| (item.parent_index == Some(parent_index)).then_some(index))
            .collect::<Vec<_>>();
        if child_indices.is_empty() {
            continue;
        }
        assign_outline_siblings_v0(&child_indices, &mut items)?;
        let parent = items.get_mut(parent_index)?;
        parent.first_child_index = child_indices.first().copied();
        parent.last_child_index = child_indices.last().copied();
        parent.child_count = u32::try_from(child_indices.len()).ok()?;
    }

    Some(items)
}
