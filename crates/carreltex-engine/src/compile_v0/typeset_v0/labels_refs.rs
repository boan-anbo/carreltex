fn is_safe_label_key_byte_v0(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b':' | b'-' | b'_' | b'.' | b'/')
}

fn parse_label_or_ref_key_group_v0(tokens: &[TokenV0], index: usize) -> Option<(Vec<u8>, usize)> {
    let (group_start, group_end, next) = consume_group_bounds(tokens, index + 1)?;
    let mut key = Vec::<u8>::new();
    for token in &tokens[group_start..group_end] {
        match token {
            TokenV0::Char(byte) if is_safe_label_key_byte_v0(*byte) => key.push(*byte),
            TokenV0::Space => return None,
            _ => return None,
        }
    }
    if key.is_empty() {
        return None;
    }
    if key.starts_with(b"/") || key.windows(2).any(|window| window == b"..") {
        return None;
    }
    Some((key, next))
}

fn build_crossref_artifacts_v1(
    labels_by_key: &BTreeMap<Vec<u8>, LabelEntryMetaV0>,
    toc_entries: &[TocEntryMetaV0],
    hyperref_enabled: bool,
) -> Option<CrossRefArtifactsV1> {
    let mut heading_anchor_ids = BTreeMap::<u32, ()>::new();
    for entry in toc_entries {
        if heading_anchor_ids.insert(entry.anchor_id, ()).is_some() {
            return None;
        }
    }
    for entry in labels_by_key.values() {
        if matches!(entry.kind, LabelKindV0::Heading) {
            heading_anchor_ids.entry(entry.anchor_id).or_insert(());
        }
    }
    Some(CrossRefArtifactsV1 {
        labels_by_key: labels_by_key.clone(),
        heading_anchor_ids,
        hyperref_enabled,
    })
}

fn apply_crossref_pass_v1(
    body: &[u8],
    artifacts: &CrossRefArtifactsV1,
    ref_occurrences: &mut Vec<RefOccurrenceMetaV0>,
    ref_link_anchor_ids: &mut Vec<u32>,
    pageref_page_link_anchor_ids: &mut Vec<u32>,
) -> Option<Vec<u8>> {
    let mut out = Vec::<u8>::with_capacity(body.len());
    let mut index = 0usize;
    let mut line_index = 1u32;

    while index < body.len() {
        let (ref_kind, key_start) = if body[index..].starts_with(REF_MARKER_PREFIX_V0) {
            (RefKindV0::Ref, index + REF_MARKER_PREFIX_V0.len())
        } else if body[index..].starts_with(PAGEREF_MARKER_PREFIX_V0) {
            (RefKindV0::Pageref, index + PAGEREF_MARKER_PREFIX_V0.len())
        } else {
            let byte = body[index];
            out.push(byte);
            if byte == NEWLINE_MARKER_V0 {
                line_index = line_index.checked_add(1)?;
            }
            index += 1;
            continue;
        };
        {
            let mut key_end = key_start;
            while key_end < body.len() {
                if body[key_end..].starts_with(REF_MARKER_SUFFIX_V0) {
                    break;
                }
                if !is_safe_label_key_byte_v0(body[key_end]) {
                    return None;
                }
                key_end += 1;
            }
            if key_end == key_start || key_end >= body.len() {
                return None;
            }
            let key = body[key_start..key_end].to_vec();
            let resolved_label = artifacts.labels_by_key.get(&key);
            let resolved_anchor_id = resolved_label.map(|entry| entry.anchor_id);
            let resolved_value = if matches!(ref_kind, RefKindV0::Ref) {
                resolved_label.and_then(|entry| match entry.kind {
                    LabelKindV0::Heading => Some(entry.anchor_id),
                    LabelKindV0::Figure => entry.figure_ordinal,
                    LabelKindV0::Equation => entry.equation_ordinal,
                })
            } else {
                None
            };
            if matches!(ref_kind, RefKindV0::Ref)
                && resolved_anchor_id.is_some() != resolved_value.is_some()
            {
                return None;
            }
            if let Some(anchor_id) = resolved_anchor_id {
                let label = resolved_label?;
                if matches!(label.kind, LabelKindV0::Heading)
                    && !artifacts.heading_anchor_ids.contains_key(&anchor_id)
                {
                    return None;
                }
                if matches!(label.kind, LabelKindV0::Figure) && label.figure_ordinal.is_none() {
                    return None;
                }
                if matches!(label.kind, LabelKindV0::Equation) && label.equation_ordinal.is_none() {
                    return None;
                }
            }
            match ref_kind {
                RefKindV0::Ref => {
                    if let Some(value) = resolved_value {
                        if artifacts.hyperref_enabled {
                            out.push(LINK_START_MARKER_V0);
                            out.extend_from_slice(value.to_string().as_bytes());
                            out.push(LINK_END_MARKER_V0);
                            let anchor_id = resolved_anchor_id?;
                            ref_link_anchor_ids.push(anchor_id);
                        } else {
                            out.extend_from_slice(value.to_string().as_bytes());
                        }
                    } else {
                        out.extend_from_slice(b"??");
                    }
                }
                RefKindV0::Pageref => {
                    if let Some(anchor_id) = resolved_anchor_id {
                        let mut marker = Vec::<u8>::new();
                        marker.extend_from_slice(PAGEREF_RENDER_MARKER_PREFIX_V0);
                        marker.extend_from_slice(anchor_id.to_string().as_bytes());
                        marker.extend_from_slice(PAGEREF_RENDER_MARKER_SUFFIX_V0);
                        if artifacts.hyperref_enabled {
                            out.push(LINK_START_MARKER_V0);
                            out.extend_from_slice(&marker);
                            out.push(LINK_END_MARKER_V0);
                            pageref_page_link_anchor_ids.push(anchor_id);
                        } else {
                            out.extend_from_slice(&marker);
                        }
                    } else {
                        out.extend_from_slice(b"??");
                    }
                }
            }
            ref_occurrences.push(RefOccurrenceMetaV0 {
                kind: ref_kind,
                key,
                line_index,
                resolved_anchor_id,
            });
            index = key_end + REF_MARKER_SUFFIX_V0.len();
            continue;
        }
    }
    Some(out)
}

fn assign_link_metadata_v1(
    body: &[u8],
    href_urls: &[Vec<u8>],
    ref_link_anchor_ids: &[u32],
    pageref_page_link_anchor_ids: &[u32],
) -> Option<(
    Vec<HrefLinkMetaV0>,
    Vec<RefAnchorLinkMetaV0>,
    Vec<PagerefPageLinkMetaV0>,
)> {
    let mut href_links = Vec::<HrefLinkMetaV0>::new();
    let mut ref_links = Vec::<RefAnchorLinkMetaV0>::new();
    let mut pageref_links = Vec::<PagerefPageLinkMetaV0>::new();
    let mut href_cursor = 0usize;
    let mut ref_cursor = 0usize;
    let mut pageref_cursor = 0usize;
    let mut next_link_id = 1u32;
    let mut index = 0usize;

    while index < body.len() {
        if body[index] != LINK_START_MARKER_V0 {
            index += 1;
            continue;
        }
        let segment_start = index + 1;
        let mut segment_end = segment_start;
        while segment_end < body.len() && body[segment_end] != LINK_END_MARKER_V0 {
            segment_end += 1;
        }
        if segment_end >= body.len() || segment_end == segment_start {
            return None;
        }
        let segment = &body[segment_start..segment_end];
        if segment.first().copied() == Some(BOLD_START_MARKER_V0) {
            let href_url = href_urls.get(href_cursor)?.clone();
            href_links.push(HrefLinkMetaV0 {
                link_id: next_link_id,
                url: href_url,
            });
            href_cursor += 1;
        } else if segment.starts_with(PAGEREF_RENDER_MARKER_PREFIX_V0)
            && segment.ends_with(PAGEREF_RENDER_MARKER_SUFFIX_V0)
        {
            let anchor_id = *pageref_page_link_anchor_ids.get(pageref_cursor)?;
            pageref_links.push(PagerefPageLinkMetaV0 {
                link_id: next_link_id,
                anchor_id,
            });
            pageref_cursor += 1;
        } else {
            let anchor_id = *ref_link_anchor_ids.get(ref_cursor)?;
            ref_links.push(RefAnchorLinkMetaV0 {
                link_id: next_link_id,
                anchor_id,
            });
            ref_cursor += 1;
        }
        next_link_id = next_link_id.checked_add(1)?;
        index = segment_end + 1;
    }

    if href_cursor != href_urls.len()
        || ref_cursor != ref_link_anchor_ids.len()
        || pageref_cursor != pageref_page_link_anchor_ids.len()
    {
        return None;
    }
    Some((href_links, ref_links, pageref_links))
}

fn resolve_cite_markers_fixedpoint_v0(
    body: &[u8],
    bibliography_entries_by_key: &BTreeMap<Vec<u8>, BibItemMetaV0>,
    cite_occurrences: &mut Vec<CiteOccurrenceMetaV0>,
) -> Option<(Vec<u8>, Vec<BibItemMetaV0>)> {
    let mut out = Vec::<u8>::with_capacity(body.len());
    let mut index = 0usize;
    let mut line_index = 1u32;
    let mut cite_key_order = Vec::<Vec<u8>>::new();
    let mut cite_seen = BTreeMap::<Vec<u8>, ()>::new();

    while index < body.len() {
        if body[index..].starts_with(CITE_MARKER_PREFIX_V0) {
            let key_start = index + CITE_MARKER_PREFIX_V0.len();
            let mut key_end = key_start;
            while key_end < body.len() {
                if body[key_end..].starts_with(CITE_MARKER_SUFFIX_V0) {
                    break;
                } else {
                    if !is_safe_label_key_byte_v0(body[key_end]) {
                        return None;
                    }
                    key_end += 1;
                }
            }
            if key_end == key_start || key_end >= body.len() {
                return None;
            }
            let key = body[key_start..key_end].to_vec();
            if !cite_seen.contains_key(&key) {
                cite_seen.insert(key.clone(), ());
                cite_key_order.push(key.clone());
            }
            let position = cite_key_order
                .iter()
                .position(|candidate| candidate == &key)?
                .checked_add(1)?;
            let resolved_ordinal = u32::try_from(position).ok()?;
            if !bibliography_entries_by_key.contains_key(&key) {
                return None;
            }
            cite_occurrences.push(CiteOccurrenceMetaV0 {
                key,
                line_index,
                resolved_ordinal: Some(resolved_ordinal),
            });
            out.push(b'[');
            out.extend_from_slice(resolved_ordinal.to_string().as_bytes());
            out.push(b']');
            index = key_end + CITE_MARKER_SUFFIX_V0.len();
            continue;
        }

        let byte = body[index];
        out.push(byte);
        if byte == NEWLINE_MARKER_V0 {
            line_index = line_index.checked_add(1)?;
        }
        index += 1;
    }

    let mut resolved_entries = Vec::<BibItemMetaV0>::new();
    for key in &cite_key_order {
        let entry = bibliography_entries_by_key.get(key)?;
        resolved_entries.push(BibItemMetaV0 {
            key: key.clone(),
            text: entry.text.clone(),
            text_len: entry.text_len,
        });
    }
    Some((out, resolved_entries))
}

fn consume_label_command_v0(
    tokens: &[TokenV0],
    index: usize,
    labels_by_key: &mut BTreeMap<Vec<u8>, LabelEntryMetaV0>,
    pending_label_target: &mut Option<PendingLabelTargetV0>,
) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == LABEL_CONTROL_V0
    ) {
        return None;
    }
    let target = pending_label_target.take()?;
    if matches!(target.kind, LabelKindV0::Figure) && target.figure_ordinal.is_none() {
        return None;
    }
    if matches!(target.kind, LabelKindV0::Equation) && target.equation_ordinal.is_none() {
        return None;
    }
    let (key, next) = parse_label_or_ref_key_group_v0(tokens, index)?;
    if labels_by_key.contains_key(&key) {
        return None;
    }
    labels_by_key.insert(
        key,
        LabelEntryMetaV0 {
            anchor_id: target.anchor_id,
            kind: target.kind,
            level: target.level,
            figure_ordinal: target.figure_ordinal,
            equation_ordinal: target.equation_ordinal,
            title: target.title,
        },
    );
    Some(next)
}

fn consume_ref_command_v0(tokens: &[TokenV0], index: usize, out: &mut Vec<u8>) -> Option<usize> {
    consume_ref_like_command_v0(tokens, index, out, REF_CONTROL_V0, REF_MARKER_PREFIX_V0)
}

fn consume_pageref_command_v0(
    tokens: &[TokenV0],
    index: usize,
    out: &mut Vec<u8>,
) -> Option<usize> {
    consume_ref_like_command_v0(
        tokens,
        index,
        out,
        PAGEREF_CONTROL_V0,
        PAGEREF_MARKER_PREFIX_V0,
    )
}

fn consume_ref_like_command_v0(
    tokens: &[TokenV0],
    index: usize,
    out: &mut Vec<u8>,
    control: &[u8],
    marker_prefix: &[u8],
) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == control
    ) {
        return None;
    }
    let (key, next) = parse_label_or_ref_key_group_v0(tokens, index)?;
    out.extend_from_slice(marker_prefix);
    out.extend_from_slice(&key);
    out.extend_from_slice(REF_MARKER_SUFFIX_V0);
    Some(next)
}

fn consume_cite_command_v0(tokens: &[TokenV0], index: usize, out: &mut Vec<u8>) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == CITE_CONTROL_V0
    ) {
        return None;
    }
    let (key, next) = parse_label_or_ref_key_group_v0(tokens, index)?;
    out.extend_from_slice(CITE_MARKER_PREFIX_V0);
    out.extend_from_slice(&key);
    out.extend_from_slice(CITE_MARKER_SUFFIX_V0);
    Some(next)
}

fn consume_footnote_command_v0(
    tokens: &[TokenV0],
    index: usize,
    out: &mut Vec<u8>,
    footnotes: &mut Vec<Vec<u8>>,
) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == FOOTNOTE_CONTROL_V0
    ) {
        return None;
    }
    let (group_start, group_end, next) = consume_group_bounds(tokens, index + 1)?;
    let mut footnote = Vec::<u8>::new();
    consume_fragment_range_v0(tokens, group_start, group_end, &mut footnote, false, false)?;
    trim_trailing_spaces(&mut footnote);
    if footnote.is_empty() {
        return None;
    }
    footnotes.push(footnote);
    out.push(b'^');
    out.extend_from_slice(footnotes.len().to_string().as_bytes());
    Some(next)
}

fn consume_href_command_v0(
    tokens: &[TokenV0],
    index: usize,
    out: &mut Vec<u8>,
    href_urls: &mut Vec<Vec<u8>>,
) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == HREF_CONTROL_V0
    ) {
        return None;
    }

    let (url_start, url_end, cursor_after_url) = consume_group_bounds(tokens, index + 1)?;
    let mut href_url = Vec::<u8>::new();
    for token in &tokens[url_start..url_end] {
        match token {
            TokenV0::Char(byte) if is_safe_href_url_byte_v0(*byte) => href_url.push(*byte),
            _ => return None,
        }
    }
    if href_url.is_empty() {
        return None;
    }

    let (text_start, text_end, next) = consume_group_bounds(tokens, cursor_after_url)?;
    let mut href_text = Vec::<u8>::new();
    consume_fragment_range_v0(tokens, text_start, text_end, &mut href_text, false, false)?;
    trim_trailing_spaces(&mut href_text);
    if href_text.is_empty() {
        return None;
    }

    href_urls.push(href_url);
    out.push(LINK_START_MARKER_V0);
    out.push(BOLD_START_MARKER_V0);
    out.extend_from_slice(&href_text);
    out.push(BOLD_END_MARKER_V0);
    out.push(LINK_END_MARKER_V0);
    Some(next)
}
