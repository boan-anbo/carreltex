pub(crate) fn extract_typeset_minimal_text_body_v0(tokens: &[TokenV0]) -> Option<Vec<u8>> {
    let empty_external_bib_entries = BTreeMap::<Vec<u8>, Vec<u8>>::new();
    extract_typeset_minimal_text_body_with_external_bib_v0(tokens, &empty_external_bib_entries)
}

pub(crate) fn extract_typeset_minimal_text_body_with_external_bib_v0(
    tokens: &[TokenV0],
    external_bib_entries: &BTreeMap<Vec<u8>, Vec<u8>>,
) -> Option<Vec<u8>> {
    let mut index = skip_spaces(tokens, 0);
    let mut saw_documentclass = false;

    let mut meta = TitleMetaV0::default();
    let mut graphicspath_prefixes = Vec::<Vec<u8>>::new();
    loop {
        index = skip_spaces(tokens, index);
        match tokens.get(index) {
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == DOCUMENTCLASS_CONTROL_V0 => {
                if saw_documentclass {
                    return None;
                }
                index = consume_documentclass_v0(tokens, index)?;
                saw_documentclass = true;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"title" => {
                let (next, value) = consume_meta_declaration_v0(tokens, index, b"title", false)?;
                meta.title = Some(value);
                index = next;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"author" => {
                let (next, value) = consume_meta_declaration_v0(tokens, index, b"author", true)?;
                meta.author = Some(value);
                index = next;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"date" => {
                let (next, value) = consume_meta_declaration_v0(tokens, index, b"date", false)?;
                meta.date = Some(value);
                index = next;
            }
            Some(TokenV0::ControlSeq(name))
                if name.as_slice() == USEPACKAGE_CONTROL_V0
                    || name.as_slice() == REQUIREPACKAGE_CONTROL_V0 =>
            {
                index = consume_package_declaration_noop_v0(tokens, index)?;
            }
            Some(TokenV0::ControlSeq(name))
                if name.as_slice() == REQUIREPACKAGEWITHOPTIONS_CONTROL_V0 =>
            {
                index = consume_requirepackage_with_options_declaration_noop_v0(tokens, index)?;
            }
            Some(TokenV0::ControlSeq(name))
                if name.as_slice() == PASSOPTIONSTOPACKAGE_CONTROL_V0
                    || name.as_slice() == PASSOPTIONSTOCLASS_CONTROL_V0 =>
            {
                index = consume_pass_options_declaration_noop_v0(tokens, index)?;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == INCLUDEONLY_CONTROL_V0 => {
                index = consume_includeonly_declaration_noop_v0(tokens, index)?;
            }
            Some(TokenV0::ControlSeq(name))
                if name.as_slice() == ADDBIBRESOURCE_CONTROL_V0
                    || name.as_slice() == BIBLIOGRAPHY_CONTROL_V0 =>
            {
                let (_, next) = parse_bibliography_resource_command_v0(tokens, index)?;
                index = next;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == GRAPHICSPATH_CONTROL_V0 => {
                let (next, prefixes) = consume_graphicspath_declaration_v0(tokens, index)?;
                graphicspath_prefixes = prefixes;
                index = next;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == BIBLIOGRAPHYSTYLE_CONTROL_V0 => {
                index = consume_bibliographystyle_command_v0(tokens, index)?;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == BEGIN_CONTROL_V0 => {
                if !saw_documentclass {
                    return None;
                }
                index = consume_document_env_command_v0(tokens, index, BEGIN_CONTROL_V0)?;
                break;
            }
            Some(TokenV0::ControlSeq(name))
                if name.as_slice() == b"protect" || name.as_slice() == b"relax" =>
            {
                index += 1;
            }
            Some(TokenV0::Space) => index += 1,
            _ => return None,
        }
    }

    let mut body = Vec::<u8>::new();
    let mut footnotes = Vec::<Vec<u8>>::new();
    let mut href_urls = Vec::<Vec<u8>>::new();
    let mut bibitems = Vec::<BibItemMetaV0>::new();
    let mut bibliography_render_requested = false;
    let mut saw_thebibliography_env = false;
    let mut toc_entries = Vec::<TocEntryMetaV0>::new();
    let mut labels_by_key = BTreeMap::<Vec<u8>, LabelEntryMetaV0>::new();
    let mut ref_occurrences = Vec::<RefOccurrenceMetaV0>::new();
    let mut ref_link_anchor_ids = Vec::<u32>::new();
    let mut pageref_page_link_anchor_ids = Vec::<u32>::new();
    let mut cite_occurrences = Vec::<CiteOccurrenceMetaV0>::new();
    let mut next_anchor_id = 1u32;
    let mut next_figure_ordinal = 1u32;
    let mut next_equation_ordinal = 1u32;
    let mut saw_maketitle = false;
    let mut saw_body_content_after_maketitle = false;
    let mut toc_requested = false;
    let mut pending_noindent_after_heading = false;
    let mut equations = Vec::<EquationMetaV0>::new();
    let mut pending_label_target = None::<PendingLabelTargetV0>;
    loop {
        match tokens.get(index) {
            Some(TokenV0::Space) => {
                push_space(&mut body);
                index += 1;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"maketitle" => {
                emit_maketitle_block_v0(&mut body, &meta);
                saw_maketitle = true;
                pending_noindent_after_heading = false;
                pending_label_target = None;
                index += 1;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == TABLEOFCONTENTS_CONTROL_V0 => {
                if !saw_maketitle || saw_body_content_after_maketitle || toc_requested {
                    return None;
                }
                push_paragraph_break(&mut body);
                body.extend_from_slice(TOC_PLACEHOLDER_MARKER_V0);
                push_newline(&mut body);
                push_paragraph_break(&mut body);
                toc_requested = true;
                pending_label_target = None;
                index += 1;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == END_CONTROL_V0 => {
                index = consume_document_env_command_v0(tokens, index, END_CONTROL_V0)?;
                break;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == BEGIN_CONTROL_V0 => {
                saw_body_content_after_maketitle = true;
                pending_noindent_after_heading = false;
                let (env_name, _) = consume_env_name_command_v0(tokens, index, BEGIN_CONTROL_V0)?;
                if env_name.as_slice() == FIGURE_ENV_V0 {
                    let anchor_id = next_anchor_id;
                    next_anchor_id = next_anchor_id.checked_add(1)?;
                    let figure_ordinal = next_figure_ordinal;
                    next_figure_ordinal = next_figure_ordinal.checked_add(1)?;
                    index = consume_figure_environment_v0(
                        tokens,
                        index,
                        &mut body,
                        &graphicspath_prefixes,
                        anchor_id,
                        figure_ordinal,
                    )?;
                    pending_label_target = Some(PendingLabelTargetV0 {
                        anchor_id,
                        kind: LabelKindV0::Figure,
                        level: None,
                        figure_ordinal: Some(figure_ordinal),
                        equation_ordinal: None,
                        title: None,
                    });
                } else if env_name.as_slice() == THEBIBLIOGRAPHY_ENV_V0 {
                    pending_label_target = None;
                    saw_thebibliography_env = true;
                    index = consume_thebibliography_environment_v0(tokens, index, &mut bibitems)?;
                } else {
                    pending_label_target = None;
                    index = consume_body_environment_v0(tokens, index, &mut body)?;
                }
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == CARRELPAR_MARKER_CONTROL_V0 => {
                push_paragraph_break(&mut body);
                pending_label_target = None;
                index += 1;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == CENTERLINE_CONTROL_V0 => {
                saw_body_content_after_maketitle = true;
                pending_noindent_after_heading = false;
                pending_label_target = None;
                index = consume_centerline_command_v0(tokens, index, &mut body)?;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == RIGHTLINE_CONTROL_V0 => {
                saw_body_content_after_maketitle = true;
                pending_noindent_after_heading = false;
                pending_label_target = None;
                index = consume_rightline_command_v0(tokens, index, &mut body)?;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == FOOTNOTE_CONTROL_V0 => {
                saw_body_content_after_maketitle = true;
                maybe_emit_pending_noindent_prefix_v0(
                    &mut body,
                    &mut pending_noindent_after_heading,
                );
                pending_label_target = None;
                index = consume_footnote_command_v0(tokens, index, &mut body, &mut footnotes)?;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == INCLUDEGRAPHICS_CONTROL_V0 => {
                saw_body_content_after_maketitle = true;
                maybe_emit_pending_noindent_prefix_v0(
                    &mut body,
                    &mut pending_noindent_after_heading,
                );
                let anchor_id = next_anchor_id;
                next_anchor_id = next_anchor_id.checked_add(1)?;
                let figure_ordinal = next_figure_ordinal;
                next_figure_ordinal = next_figure_ordinal.checked_add(1)?;
                let (image, next) =
                    consume_includegraphics_command_v0(tokens, index, &graphicspath_prefixes)?;
                emit_inline_includegraphics_placeholder_v0(
                    &mut body,
                    &image,
                    anchor_id,
                    figure_ordinal,
                );
                index = next;
                pending_label_target = Some(PendingLabelTargetV0 {
                    anchor_id,
                    kind: LabelKindV0::Figure,
                    level: None,
                    figure_ordinal: Some(figure_ordinal),
                    equation_ordinal: None,
                    title: None,
                });
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == HREF_CONTROL_V0 => {
                saw_body_content_after_maketitle = true;
                maybe_emit_pending_noindent_prefix_v0(
                    &mut body,
                    &mut pending_noindent_after_heading,
                );
                pending_label_target = None;
                index = consume_href_command_v0(tokens, index, &mut body, &mut href_urls)?;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"[" => {
                saw_body_content_after_maketitle = true;
                maybe_emit_pending_noindent_prefix_v0(
                    &mut body,
                    &mut pending_noindent_after_heading,
                );
                let anchor_id = next_anchor_id;
                next_anchor_id = next_anchor_id.checked_add(1)?;
                let equation_ordinal = next_equation_ordinal;
                next_equation_ordinal = next_equation_ordinal.checked_add(1)?;
                index = consume_display_math_command_v0(tokens, index, &mut body)?;
                equations.push(EquationMetaV0 {
                    anchor_id,
                    ordinal: equation_ordinal,
                });
                pending_label_target = Some(PendingLabelTargetV0 {
                    anchor_id,
                    kind: LabelKindV0::Equation,
                    level: None,
                    figure_ordinal: None,
                    equation_ordinal: Some(equation_ordinal),
                    title: None,
                });
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == LABEL_CONTROL_V0 => {
                saw_body_content_after_maketitle = true;
                index = consume_label_command_v0(
                    tokens,
                    index,
                    &mut labels_by_key,
                    &mut pending_label_target,
                )?;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == REF_CONTROL_V0 => {
                saw_body_content_after_maketitle = true;
                maybe_emit_pending_noindent_prefix_v0(
                    &mut body,
                    &mut pending_noindent_after_heading,
                );
                pending_label_target = None;
                index = consume_ref_command_v0(tokens, index, &mut body)?;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == PAGEREF_CONTROL_V0 => {
                saw_body_content_after_maketitle = true;
                maybe_emit_pending_noindent_prefix_v0(
                    &mut body,
                    &mut pending_noindent_after_heading,
                );
                pending_label_target = None;
                index = consume_pageref_command_v0(tokens, index, &mut body)?;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == CITE_CONTROL_V0 => {
                saw_body_content_after_maketitle = true;
                maybe_emit_pending_noindent_prefix_v0(
                    &mut body,
                    &mut pending_noindent_after_heading,
                );
                pending_label_target = None;
                index = consume_cite_command_v0(tokens, index, &mut body)?;
            }
            Some(TokenV0::ControlSeq(name))
                if name.as_slice() == ADDBIBRESOURCE_CONTROL_V0
                    || name.as_slice() == BIBLIOGRAPHY_CONTROL_V0 =>
            {
                saw_body_content_after_maketitle = true;
                maybe_emit_pending_noindent_prefix_v0(
                    &mut body,
                    &mut pending_noindent_after_heading,
                );
                pending_label_target = None;
                if name.as_slice() == BIBLIOGRAPHY_CONTROL_V0 {
                    bibliography_render_requested = true;
                }
                let (_, next) = parse_bibliography_resource_command_v0(tokens, index)?;
                index = next;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == PRINTBIBLIOGRAPHY_CONTROL_V0 => {
                saw_body_content_after_maketitle = true;
                maybe_emit_pending_noindent_prefix_v0(
                    &mut body,
                    &mut pending_noindent_after_heading,
                );
                pending_label_target = None;
                bibliography_render_requested = true;
                index += 1;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == BIBLIOGRAPHYSTYLE_CONTROL_V0 => {
                saw_body_content_after_maketitle = true;
                pending_label_target = None;
                index = consume_bibliographystyle_command_v0(tokens, index)?;
            }
            Some(TokenV0::ControlSeq(name)) if is_heading_control_v0(name.as_slice()) => {
                saw_body_content_after_maketitle = true;
                let (next, heading_meta) =
                    consume_heading_command_v0(tokens, index, &mut body, !toc_requested)?;
                index = next;
                if let Some((level, title)) = heading_meta {
                    let anchor_id = next_anchor_id;
                    next_anchor_id = next_anchor_id.checked_add(1)?;
                    if toc_requested {
                        toc_entries.push(TocEntryMetaV0 {
                            level,
                            anchor_id,
                            title: title.clone(),
                        });
                    }
                    pending_label_target = Some(PendingLabelTargetV0 {
                        anchor_id,
                        kind: LabelKindV0::Heading,
                        level: Some(level),
                        figure_ordinal: None,
                        equation_ordinal: None,
                        title: Some(title),
                    });
                } else {
                    pending_label_target = None;
                }
                pending_noindent_after_heading = true;
            }
            Some(_) => {
                saw_body_content_after_maketitle = true;
                maybe_emit_pending_noindent_prefix_v0(
                    &mut body,
                    &mut pending_noindent_after_heading,
                );
                pending_label_target = None;
                index = consume_fragment_token_v0(tokens, index, &mut body, false, true)?;
            }
            None => return None,
        }
    }

    if tokens[index..]
        .iter()
        .any(|token| !matches!(token, TokenV0::Space))
    {
        return None;
    }

    body = normalize_space_before_punctuation_v0(&body);
    body = normalize_punctuation_spacing_v0(&body);
    body = normalize_tex_double_quotes_v0(&body);
    body = normalize_tex_dashes_v0(&body);
    body = normalize_tex_ellipsis_v0(&body);
    body = normalize_bracket_spacing_v0(&body);
    body = normalize_wrapper_marker_spacing_v0(&body);
    let crossref_artifacts =
        build_crossref_artifacts_v1(&labels_by_key, &toc_entries, !href_urls.is_empty())?;
    body = apply_crossref_pass_v1(
        &body,
        &crossref_artifacts,
        &mut ref_occurrences,
        &mut ref_link_anchor_ids,
        &mut pageref_page_link_anchor_ids,
    )?;
    let (href_links, ref_anchor_links, pageref_page_links) = assign_link_metadata_v1(
        &body,
        &href_urls,
        &ref_link_anchor_ids,
        &pageref_page_link_anchor_ids,
    )?;
    let mut bibliography_entries_by_key = BTreeMap::<Vec<u8>, BibItemMetaV0>::new();
    for (key, text) in external_bib_entries {
        if text.is_empty() {
            return None;
        }
        let text_len = u32::try_from(text.len()).ok()?;
        if bibliography_entries_by_key
            .insert(
                key.clone(),
                BibItemMetaV0 {
                    key: key.clone(),
                    text: text.clone(),
                    text_len,
                },
            )
            .is_some()
        {
            return None;
        }
    }
    for item in &bibitems {
        if bibliography_entries_by_key
            .insert(item.key.clone(), item.clone())
            .is_some()
        {
            return None;
        }
    }
    let (resolved_body, resolved_bibliography_entries) = resolve_cite_markers_fixedpoint_v0(
        &body,
        &bibliography_entries_by_key,
        &mut cite_occurrences,
    )?;
    body = resolved_body;

    if !cite_occurrences.is_empty() && resolved_bibliography_entries.is_empty() {
        return None;
    }
    if (saw_thebibliography_env || !cite_occurrences.is_empty())
        && resolved_bibliography_entries.is_empty()
    {
        return None;
    }
    if saw_thebibliography_env || bibliography_render_requested {
        if !resolved_bibliography_entries.is_empty() {
            emit_bibliography_block_v0(&mut body, &resolved_bibliography_entries);
        }
    }

    if !footnotes.is_empty() {
        push_paragraph_break(&mut body);
        for (note_index, footnote) in footnotes.iter().enumerate() {
            body.extend_from_slice(FOOTNOTE_LINE_PREFIX_MARKER_V0);
            body.extend_from_slice((note_index + 1).to_string().as_bytes());
            body.push(b' ');
            body.extend_from_slice(footnote);
            push_newline(&mut body);
        }
    }

    if !href_links.is_empty() {
        push_paragraph_break(&mut body);
        for href in &href_links {
            body.extend_from_slice(HREF_URL_LINE_PREFIX_MARKER_V0);
            body.extend_from_slice(href.link_id.to_string().as_bytes());
            body.push(b' ');
            body.extend_from_slice(&href.url);
            push_newline(&mut body);
        }
    }
    if toc_requested {
        push_paragraph_break(&mut body);
        for entry in &toc_entries {
            body.extend_from_slice(TOC_ENTRY_LINE_PREFIX_MARKER_V0);
            body.extend_from_slice(entry.level.to_string().as_bytes());
            body.push(b' ');
            body.extend_from_slice(entry.anchor_id.to_string().as_bytes());
            body.push(b' ');
            if crossref_artifacts.hyperref_enabled {
                body.push(LINK_START_MARKER_V0);
                body.extend_from_slice(&entry.title);
                body.push(LINK_END_MARKER_V0);
            } else {
                body.extend_from_slice(&entry.title);
            }
            push_newline(&mut body);
        }
    }
    if !equations.is_empty() {
        push_paragraph_break(&mut body);
        for equation in &equations {
            body.extend_from_slice(EQUATION_LINE_PREFIX_MARKER_V0);
            body.extend_from_slice(equation.anchor_id.to_string().as_bytes());
            body.push(b' ');
            body.extend_from_slice(equation.ordinal.to_string().as_bytes());
            push_newline(&mut body);
        }
    }
    if !labels_by_key.is_empty() {
        push_paragraph_break(&mut body);
        for (key, entry) in &labels_by_key {
            body.extend_from_slice(LABEL_LINE_PREFIX_MARKER_V0);
            body.extend_from_slice(key);
            body.push(b' ');
            body.extend_from_slice(entry.anchor_id.to_string().as_bytes());
            body.push(b' ');
            body.extend_from_slice(match entry.kind {
                LabelKindV0::Heading => b"heading",
                LabelKindV0::Figure => b"figure",
                LabelKindV0::Equation => b"equation",
            });
            body.push(b' ');
            let level_or_ordinal = match entry.kind {
                LabelKindV0::Heading => u32::from(entry.level.unwrap_or(0)),
                LabelKindV0::Figure => entry.figure_ordinal.unwrap_or(0),
                LabelKindV0::Equation => entry.equation_ordinal.unwrap_or(0),
            };
            body.extend_from_slice(level_or_ordinal.to_string().as_bytes());
            body.push(b' ');
            if let Some(title) = &entry.title {
                body.extend_from_slice(title);
            } else {
                body.push(b'-');
            }
            push_newline(&mut body);
        }
    }
    if !ref_occurrences.is_empty() {
        push_paragraph_break(&mut body);
        for occurrence in &ref_occurrences {
            if matches!(occurrence.kind, RefKindV0::Pageref) {
                continue;
            }
            body.extend_from_slice(REF_LINE_PREFIX_MARKER_V0);
            body.extend_from_slice(&occurrence.key);
            body.push(b' ');
            body.extend_from_slice(occurrence.line_index.to_string().as_bytes());
            body.push(b' ');
            body.extend_from_slice(
                occurrence
                    .resolved_anchor_id
                    .unwrap_or(0)
                    .to_string()
                    .as_bytes(),
            );
            push_newline(&mut body);
        }
    }
    if ref_occurrences
        .iter()
        .any(|occurrence| matches!(occurrence.kind, RefKindV0::Pageref))
    {
        push_paragraph_break(&mut body);
        for occurrence in &ref_occurrences {
            if !matches!(occurrence.kind, RefKindV0::Pageref) {
                continue;
            }
            body.extend_from_slice(PAGEREF_LINE_PREFIX_MARKER_V0);
            body.extend_from_slice(&occurrence.key);
            body.push(b' ');
            body.extend_from_slice(occurrence.line_index.to_string().as_bytes());
            body.push(b' ');
            body.extend_from_slice(
                occurrence
                    .resolved_anchor_id
                    .unwrap_or(0)
                    .to_string()
                    .as_bytes(),
            );
            push_newline(&mut body);
        }
    }
    if !ref_anchor_links.is_empty() {
        push_paragraph_break(&mut body);
        for link in &ref_anchor_links {
            body.extend_from_slice(REF_ANCHOR_LINK_LINE_PREFIX_MARKER_V0);
            body.extend_from_slice(link.link_id.to_string().as_bytes());
            body.push(b' ');
            body.extend_from_slice(link.anchor_id.to_string().as_bytes());
            push_newline(&mut body);
        }
    }
    if !pageref_page_links.is_empty() {
        push_paragraph_break(&mut body);
        for link in &pageref_page_links {
            body.extend_from_slice(PAGEREF_PAGE_LINK_LINE_PREFIX_MARKER_V0);
            body.extend_from_slice(link.link_id.to_string().as_bytes());
            body.push(b' ');
            body.extend_from_slice(link.anchor_id.to_string().as_bytes());
            push_newline(&mut body);
        }
    }
    if !resolved_bibliography_entries.is_empty() {
        push_paragraph_break(&mut body);
        for (ordinal_index, item) in resolved_bibliography_entries.iter().enumerate() {
            let ordinal = u32::try_from(ordinal_index.checked_add(1)?).ok()?;
            body.extend_from_slice(BIBITEM_LINE_PREFIX_MARKER_V0);
            body.extend_from_slice(&item.key);
            body.push(b' ');
            body.extend_from_slice(ordinal.to_string().as_bytes());
            body.push(b' ');
            body.extend_from_slice(item.text_len.to_string().as_bytes());
            push_newline(&mut body);
        }
    }
    if !cite_occurrences.is_empty() {
        push_paragraph_break(&mut body);
        for occurrence in &cite_occurrences {
            body.extend_from_slice(CITE_LINE_PREFIX_MARKER_V0);
            body.extend_from_slice(&occurrence.key);
            body.push(b' ');
            body.extend_from_slice(occurrence.line_index.to_string().as_bytes());
            body.push(b' ');
            body.extend_from_slice(
                occurrence
                    .resolved_ordinal
                    .unwrap_or(0)
                    .to_string()
                    .as_bytes(),
            );
            push_newline(&mut body);
        }
    }
    trim_trailing_spaces(&mut body);
    while matches!(body.last().copied(), Some(NEWLINE_MARKER_V0)) {
        body.pop();
    }
    if body.is_empty() {
        return None;
    }
    Some(body)
}
