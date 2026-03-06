fn emit_table_block_v0(
    out: &mut Vec<u8>,
    align_spec: &[u8],
    rows: &[LinePlanV0],
    y: &mut f32,
    min_body_y_pt: f32,
) -> Option<()> {
    if rows.is_empty() {
        return None;
    }
    if align_spec.is_empty() {
        return None;
    }
    let col_count = align_spec.len();
    let mut parsed_rows = Vec::<Vec<Vec<PdfRenderSegmentV0>>>::new();
    let mut col_max_width_pt = vec![0.0f32; col_count];

    for row in rows {
        let cells = parse_table_row_cells_v0(&row.glyphs)?;
        if cells.len() != col_count {
            return None;
        }
        let mut parsed_cells = Vec::<Vec<PdfRenderSegmentV0>>::new();
        for (col_index, cell_glyphs) in cells.iter().enumerate() {
            let segments = parse_styled_segments_v0(cell_glyphs)?;
            let render_segments = split_superscript_segments_v0(&segments);
            if render_segments
                .iter()
                .any(|segment| segment.superscript || segment.is_link)
            {
                return None;
            }
            let width_pt = render_segments
                .iter()
                .map(|segment| segment.advance_pt)
                .sum::<f32>();
            col_max_width_pt[col_index] = col_max_width_pt[col_index].max(width_pt);
            parsed_cells.push(render_segments);
        }
        parsed_rows.push(parsed_cells);
    }

    let table_width_pt = col_max_width_pt
        .iter()
        .copied()
        .fold(0.0f32, |acc, width_pt| {
            acc + width_pt + (TABLE_CELL_PADDING_PT_V10 * 2.0)
        });
    let max_content_width_pt = PAGE_WIDTH_PT_V0 - (2.0 * MARGIN_PT_V0);
    if table_width_pt > max_content_width_pt {
        return None;
    }

    let mut col_left_edges_pt = vec![0.0f32; col_count];
    let mut col_cursor_x_pt = MARGIN_PT_V0;
    for col_index in 0..col_count {
        col_left_edges_pt[col_index] = col_cursor_x_pt;
        col_cursor_x_pt += col_max_width_pt[col_index] + (TABLE_CELL_PADDING_PT_V10 * 2.0);
    }
    let table_left_x_pt = MARGIN_PT_V0;
    let table_right_x_pt = table_left_x_pt + table_width_pt;
    let table_top_y_pt = *y + TABLE_BORDER_TOP_OFFSET_PT_V10;

    for row in &parsed_rows {
        if *y < min_body_y_pt {
            return None;
        }
        let mut col_left_x = MARGIN_PT_V0;
        for col_index in 0..col_count {
            let cell_width_pt = row[col_index]
                .iter()
                .map(|segment| segment.advance_pt)
                .sum::<f32>();
            let col_content_width_pt = col_max_width_pt[col_index];
            let align_offset_pt = match align_spec[col_index] {
                b'l' => 0.0,
                b'c' => (col_content_width_pt - cell_width_pt) * 0.5,
                b'r' => col_content_width_pt - cell_width_pt,
                _ => return None,
            };
            let x_pt = col_left_x + TABLE_CELL_PADDING_PT_V10 + align_offset_pt.max(0.0);
            emit_render_segments_with_superscript_v0(
                out,
                &row[col_index],
                x_pt,
                *y,
                FONT_SIZE_PT_V0,
            );
            col_left_x += col_content_width_pt + (TABLE_CELL_PADDING_PT_V10 * 2.0);
        }
        out.extend_from_slice(b"\n");
        *y -= TABLE_ROW_LEADING_PT_V10;
    }

    let table_bottom_y_pt = *y + TABLE_BORDER_BOTTOM_OFFSET_PT_V10;
    if table_bottom_y_pt < min_body_y_pt {
        return None;
    }
    if table_bottom_y_pt >= table_top_y_pt {
        return None;
    }

    out.extend_from_slice(b"ET\n");
    out.extend_from_slice(b"0 G\n");
    out.extend_from_slice(format!("{TABLE_BORDER_LINE_WIDTH_PT_V0} w\n").as_bytes());
    out.extend_from_slice(
        format!(
            "{:.2} {:.2} {:.2} {:.2} re S\n",
            table_left_x_pt,
            table_bottom_y_pt,
            table_width_pt,
            table_top_y_pt - table_bottom_y_pt,
        )
        .as_bytes(),
    );
    for separator_index in 1..parsed_rows.len() {
        let y_pt = table_top_y_pt - (separator_index as f32 * TABLE_ROW_LEADING_PT_V10);
        out.extend_from_slice(
            format!(
                "{:.2} {:.2} m {:.2} {:.2} l S\n",
                table_left_x_pt, y_pt, table_right_x_pt, y_pt,
            )
            .as_bytes(),
        );
    }
    for col_index in 1..col_count {
        let x_pt = col_left_edges_pt[col_index];
        out.extend_from_slice(
            format!(
                "{:.2} {:.2} m {:.2} {:.2} l S\n",
                x_pt, table_top_y_pt, x_pt, table_bottom_y_pt,
            )
            .as_bytes(),
        );
    }
    out.extend_from_slice(b"BT\n");
    out.extend_from_slice(b"0 g\n");
    Some(())
}

fn emit_figure_block_v0(
    out: &mut Vec<u8>,
    image_metadata: Option<&FigureImageMetadataV0>,
    caption_glyphs: &[GlyphPlanV0],
    y: &mut f32,
    min_body_y_pt: f32,
) -> Option<()> {
    if *y < min_body_y_pt {
        return None;
    }

    let placeholder_segments =
        placeholder_segments_v0(image_metadata.map(|meta| meta.image_path.as_slice()));
    let placeholder_render_segments = split_superscript_segments_v0(&placeholder_segments);
    let placeholder_text_width_pt = placeholder_render_segments
        .iter()
        .map(|segment| segment.advance_pt)
        .sum::<f32>();
    let placeholder_box_width_pt = image_metadata
        .map(|meta| meta.width_pt)
        .unwrap_or(DEFAULT_FIGURE_PLACEHOLDER_WIDTH_PT_V0);
    let placeholder_box_height_pt = image_metadata
        .map(|meta| meta.height_pt)
        .unwrap_or(DEFAULT_FIGURE_PLACEHOLDER_HEIGHT_PT_V0);
    let placeholder_width_pt = placeholder_box_width_pt
        .max(placeholder_text_width_pt + (FIGURE_PLACEHOLDER_LABEL_INSET_PT_V0 * 2.0));
    if placeholder_width_pt > MAX_FIGURE_PLACEHOLDER_WIDTH_PT_V0 {
        return None;
    }
    let required_placeholder_bottom_y = *y - placeholder_box_height_pt;
    if required_placeholder_bottom_y < min_body_y_pt {
        return None;
    }
    let placeholder_x_pt = centered_line_x_v0(placeholder_width_pt);
    let max_placeholder_right = PAGE_WIDTH_PT_V0 - MARGIN_PT_V0;
    if placeholder_x_pt + placeholder_width_pt > max_placeholder_right + 0.01 {
        return None;
    }
    emit_render_segments_with_superscript_v0(
        out,
        &placeholder_render_segments,
        placeholder_x_pt + FIGURE_PLACEHOLDER_LABEL_INSET_PT_V0,
        *y - FIGURE_PLACEHOLDER_LABEL_INSET_PT_V0,
        FONT_SIZE_PT_V0,
    );
    out.extend_from_slice(b"\n");
    out.extend_from_slice(b"ET\n");
    out.extend_from_slice(b"0 G\n");
    out.extend_from_slice(format!("{TABLE_BORDER_LINE_WIDTH_PT_V0} w\n").as_bytes());
    out.extend_from_slice(
        format!(
            "{:.2} {:.2} {:.2} {:.2} re S\n",
            placeholder_x_pt,
            required_placeholder_bottom_y,
            placeholder_width_pt,
            placeholder_box_height_pt,
        )
        .as_bytes(),
    );
    out.extend_from_slice(b"BT\n");
    out.extend_from_slice(b"0 g\n");
    *y = required_placeholder_bottom_y - FIGURE_PLACEHOLDER_TO_CAPTION_GAP_PT_V0;

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
    Some(())
}

fn emit_toc_block_v0(
    out: &mut Vec<u8>,
    toc_entries: &[TocEntryMetadataV0],
    page_numbers_by_anchor_id: &BTreeMap<u32, u32>,
    page_count: usize,
    y: &mut f32,
    min_body_y_pt: f32,
    annotations: &mut Vec<PdfLinkAnnotationV0>,
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
        advance_sp: title_glyphs.iter().map(|glyph| glyph.advance_sp).sum(),
        advance_pt: title_glyphs
            .iter()
            .map(|glyph| (glyph.advance_sp as f32) / 65_536.0)
            .sum(),
        glyphs: title_glyphs,
        is_link: false,
    }];
    emit_styled_segments_v0(
        out,
        &title_segments,
        MARGIN_PT_V0,
        *y,
        TOC_TITLE_FONT_SIZE_PT_V0,
    );
    out.extend_from_slice(b"\n");
    *y -= TOC_TITLE_TO_FIRST_ENTRY_GAP_PT_V5;

    for entry in toc_entries {
        if *y < min_body_y_pt {
            return None;
        }
        let page_no = *page_numbers_by_anchor_id.get(&entry.anchor_id)?;
        if page_no == 0 || usize::try_from(page_no).ok()? > page_count {
            return None;
        }
        let base_segments = parse_styled_segments_v0(&entry.title_glyphs)?;
        let render_segments = split_superscript_segments_v0(&base_segments);
        if render_segments.iter().any(|segment| segment.superscript) {
            return None;
        }
        let title_width_pt = render_segments.iter().map(|segment| segment.advance_pt).sum::<f32>();
        let indent_steps = f32::from(entry.level.saturating_sub(1));
        let x_pt = MARGIN_PT_V0 + (indent_steps * TOC_ENTRY_INDENT_STEP_PT_V0);
        let mut toc_annotations = collect_toc_link_annotations_for_line_v0(
            &render_segments,
            x_pt,
            *y,
            FONT_SIZE_PT_V0,
            PdfLinkTargetV0::Anchor(entry.anchor_id),
        )?;
        annotations.append(&mut toc_annotations);
        emit_render_segments_with_superscript_v0(out, &render_segments, x_pt, *y, FONT_SIZE_PT_V0);

        let page_no_glyphs = page_no
            .to_string()
            .bytes()
            .map(|byte| {
                Some(GlyphPlanV0 {
                    byte,
                    advance_sp: glyph_width_sp_v0(byte, 65_536)?,
                })
            })
            .collect::<Option<Vec<GlyphPlanV0>>>()?;
        if page_no_glyphs.is_empty() {
            return None;
        }
        let page_no_width_pt = glyphs_advance_pt_v0(&page_no_glyphs);
        if page_no_width_pt <= 0.0 || page_no_width_pt > TOC_PAGE_NO_COLUMN_WIDTH_PT_V2 {
            return None;
        }
        let page_no_column_right_pt =
            PAGE_WIDTH_PT_V0 - MARGIN_PT_V0 - TOC_PAGE_NO_COLUMN_RIGHT_INSET_PT_V5;
        let page_no_column_left_pt = page_no_column_right_pt - TOC_PAGE_NO_COLUMN_WIDTH_PT_V2;
        let page_no_x_pt = page_no_column_right_pt - page_no_width_pt;
        if page_no_x_pt < page_no_column_left_pt
            || page_no_x_pt <= x_pt + title_width_pt + TOC_PAGE_NO_COLUMN_GAP_PT_V2
        {
            return None;
        }
        let title_links_enabled = render_segments.iter().any(|segment| segment.is_link);
        let page_no_segments = vec![PdfRenderSegmentV0 {
            style: PdfTextStyleV0::Regular,
            bytes: bytes_from_glyphs_v0(&page_no_glyphs),
            advance_sp: page_no_glyphs.iter().map(|glyph| glyph.advance_sp).sum(),
            advance_pt: page_no_width_pt,
            is_link: title_links_enabled,
            superscript: false,
        }];
        if title_links_enabled {
            let mut page_no_annotations = collect_toc_link_annotations_for_line_v0(
                &page_no_segments,
                page_no_x_pt,
                *y,
                FONT_SIZE_PT_V0,
                PdfLinkTargetV0::AnchorPage(entry.anchor_id),
            )?;
            annotations.append(&mut page_no_annotations);
        }
        emit_render_segments_with_superscript_v0(
            out,
            &page_no_segments,
            page_no_x_pt,
            *y,
            FONT_SIZE_PT_V0,
        );
        out.extend_from_slice(b"\n");
        *y -= TOC_ENTRY_LEADING_PT_V5;
    }
    Some(())
}
