#[test]
fn pdf_renderer_renders_toc_block_with_level_indentation_v0() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Before paragraph.\n\n!toc\n\nAfter paragraph.\n\n@S {Anchor section one}\n\n@s {Anchor subsection two}\n\n!toc 1 1 Intro toc entry\n!toc 2 2 Detail toc entry",
    )
    .expect("writer should accept toc marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);

    assert!(
        !pdf_text.contains("!toc 1 1"),
        "toc metadata lines must not be visible"
    );
    assert!(
        !pdf_text.contains("!toc 2 2"),
        "toc metadata lines must not be visible"
    );
    assert!(
        pdf_text.contains("(Contents) Tj"),
        "toc title should render"
    );
    assert!(
        pdf_text.contains("(Intro toc entry) Tj"),
        "toc level 1 entry should render"
    );
    assert!(
        pdf_text.contains("(Detail toc entry) Tj"),
        "toc level 2 entry should render"
    );
    assert!(
        pdf_text.contains("(1) Tj"),
        "toc entries should render page number column values"
    );

    let intro_x = tm_x_for_line_containing_text_v0(&pdf, "(Intro toc entry)")
        .expect("toc level 1 position");
    let detail_x = tm_x_for_line_containing_text_v0(&pdf, "(Detail toc entry)")
        .expect("toc level 2 position");
    assert!(
        detail_x > intro_x + 8.0,
        "toc level 2 should be indented from level 1: {intro_x} vs {detail_x}"
    );
}

#[test]
fn pdf_renderer_emits_toc_link_annotations_targeting_heading_anchors_v0() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Prelude.\n\n!toc\n\n@S {Intro section}\x0c@s {Detail section}\n\n!toc 1 1 <Intro section>\n!toc 2 2 <Detail section>",
    )
    .expect("writer should accept toc metadata and heading lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    assert!(
        pdf_text.contains("(Intro section) Tj"),
        "toc section title should render: {pdf_text}"
    );
    assert!(
        pdf_text.contains("(Detail section) Tj"),
        "toc subsection title should render: {pdf_text}"
    );

    let page_one = parse_pdf_object_body_v0(&pdf, 3).expect("page object");
    let annots = parse_pdf_ref_ids_v0(&page_one, "/Annots");
    assert_eq!(
        annots.len(),
        4,
        "toc block should emit title and page-number annotations for each entry"
    );

    let mut xyz_pages = Vec::<u32>::new();
    let mut fit_pages = Vec::<u32>::new();
    for annotation_id in annots {
        let annotation = parse_pdf_object_body_v0(&pdf, annotation_id).expect("annotation body");
        assert!(
            annotation.contains("/Dest ["),
            "toc links should use internal destinations: {annotation}"
        );
        assert!(
            !annotation.contains("/A "),
            "toc links should not use URI actions: {annotation}"
        );
        let Some(page_id) = parse_pdf_annotation_dest_page_id_v0(&annotation) else {
            panic!("toc annotation should include destination page id: {annotation}");
        };
        if annotation.contains("/XYZ") {
            xyz_pages.push(page_id);
        } else if annotation.contains("/Fit]") {
            fit_pages.push(page_id);
            assert!(
                !annotation.contains("/XYZ"),
                "toc page-number links should target /Fit page destinations only: {annotation}"
            );
        } else {
            panic!("unexpected toc annotation destination shape: {annotation}");
        }
        let rect = parse_pdf_annotation_rect_v0(&annotation).expect("annotation rect");
        assert!(
            rect[2] > rect[0],
            "annotation width must be positive: {rect:?}"
        );
        assert!(
            rect[3] > rect[1],
            "annotation height must be positive: {rect:?}"
        );
        assert!(
            rect[0] >= 0.0 && rect[1] >= 0.0 && rect[2] <= 612.0 && rect[3] <= 792.0,
            "annotation rect must stay within page bounds: {rect:?}"
        );
    }
    xyz_pages.sort_unstable();
    fit_pages.sort_unstable();
    assert_eq!(
        xyz_pages,
        vec![3, 4],
        "toc title links should target heading anchor pages in document order"
    );
    assert_eq!(
        fit_pages,
        vec![3, 4],
        "toc page-number links should target page destinations in document order"
    );
}

#[test]
fn pdf_renderer_toc_annotation_destination_order_is_stable_v2() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Prelude.\n\n!toc\n\n@S {Intro section}\x0c@s {Detail section}\n\n!toc 1 1 <Intro section>\n!toc 2 2 <Detail section>",
    ).expect("writer should accept toc metadata and heading lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let page_one = parse_pdf_object_body_v0(&pdf, 3).expect("page object");
    let annots = parse_pdf_ref_ids_v0(&page_one, "/Annots");
    assert_eq!(annots.len(), 4, "expected title+page toc annotations");

    let mut xyz_destinations = Vec::<(u32, f32, f32)>::new();
    let mut fit_page_ids = Vec::<u32>::new();
    for id in annots {
        let body = parse_pdf_object_body_v0(&pdf, id).expect("annotation body");
        if body.contains("/XYZ") {
            xyz_destinations.push(parse_pdf_annotation_dest_xyz_v0(&body).expect("xyz destination"));
        } else if body.contains("/Fit]") {
            fit_page_ids.push(
                parse_pdf_annotation_dest_page_id_v0(&body).expect("fit destination page id"),
            );
        } else {
            panic!("unexpected toc annotation destination shape: {body}");
        }
    }
    assert_eq!(
        xyz_destinations.len(),
        2,
        "expected one heading-anchor link per toc title"
    );
    assert_eq!(
        fit_page_ids.len(),
        2,
        "expected one page-destination link per toc page number"
    );
    assert_eq!(
        xyz_destinations.iter().map(|dest| dest.0).collect::<Vec<_>>(),
        vec![3, 4],
        "toc title link destinations should remain stable and ordered"
    );
    assert_eq!(
        fit_page_ids,
        vec![3, 4],
        "toc page-number link destinations should remain stable and ordered"
    );
}

#[test]
fn pdf_renderer_renders_toc_page_numbers_with_mixed_values_v2() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Prelude.\n\n!toc\n\n@S {First anchor}\x0c@S {Second anchor}\n\n!toc 1 1 First toc entry\n!toc 1 2 Second toc entry",
    )
    .expect("writer should accept toc marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    assert!(
        pdf_text.contains("(First toc entry) Tj") && pdf_text.contains("(Second toc entry) Tj"),
        "toc entry titles should render: {pdf_text}"
    );
    assert!(
        pdf_text.contains("(1) Tj") && pdf_text.contains("(2) Tj"),
        "toc page-number column should render mixed values: {pdf_text}"
    );

    let (one_x, _) = tm_position_for_segment_substring_v0(&pdf, "(1)").expect("page one number x");
    let (two_x, _) = tm_position_for_segment_substring_v0(&pdf, "(2)").expect("page two number x");
    assert!(
        one_x >= 492.0 && two_x >= 492.0,
        "toc page numbers should be right-column aligned: one_x={one_x}, two_x={two_x}"
    );
}

#[test]
fn pdf_renderer_emits_outline_root_for_toc_entries_v0() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Prelude.\n\n!toc\n\n@S {Intro section}\n\n@s {Detail section}\n\n!toc 1 1 Intro section\n!toc 2 2 Detail section",
    )
    .expect("writer should accept toc marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let catalog = parse_pdf_object_body_v0(&pdf, 1).expect("catalog object");
    let outline_root_id =
        parse_pdf_single_ref_id_v0(&catalog, "/Outlines").expect("catalog outlines ref");
    let outline_root = parse_pdf_object_body_v0(&pdf, outline_root_id).expect("outline root");
    assert!(
        outline_root.contains("/Type /Outlines"),
        "outline root type missing: {outline_root}"
    );
    let count = parse_pdf_outline_count_v0(&outline_root).expect("outline count");
    assert_eq!(count, 2, "outline root count should equal toc entry count");
    assert!(
        parse_pdf_single_ref_id_v0(&outline_root, "/First").is_some()
            && parse_pdf_single_ref_id_v0(&outline_root, "/Last").is_some(),
        "outline root should expose first/last refs: {outline_root}"
    );
}

#[test]
fn pdf_renderer_outline_count_matches_toc_entries_v0() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Prelude.\n\n!toc\n\n@S {Alpha}\n\n@s {Beta}\n\x0c@S {Gamma}\n\n!toc 1 1 Alpha\n!toc 2 2 Beta\n!toc 1 3 Gamma",
    )
    .expect("writer should accept toc marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let catalog = parse_pdf_object_body_v0(&pdf, 1).expect("catalog object");
    let outline_root_id =
        parse_pdf_single_ref_id_v0(&catalog, "/Outlines").expect("catalog outlines ref");
    let outline_root = parse_pdf_object_body_v0(&pdf, outline_root_id).expect("outline root");
    assert_eq!(
        parse_pdf_outline_count_v0(&outline_root),
        Some(3),
        "outline root count should match toc entries"
    );
    let outline_item_ids = collect_outline_item_ids_depth_first_v0(&pdf, outline_root_id)
        .expect("outline item traversal");
    assert_eq!(
        outline_item_ids.len(),
        3,
        "outline items should match toc entry count"
    );
}

#[test]
fn pdf_renderer_outline_destinations_resolve_to_anchor_destinations_v0() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Prelude.\n\n!toc\n\n@S {Intro section}\x0c@s {Detail section}\n\n!toc 1 1 <Intro section>\n!toc 2 2 <Detail section>",
    )
    .expect("writer should accept toc metadata and heading lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let pages_obj = parse_pdf_object_body_v0(&pdf, 2).expect("pages object");
    let page_ids = parse_pdf_ref_ids_v0(&pages_obj, "/Kids");
    assert!(!page_ids.is_empty(), "expected page ids");
    let mut known_anchor_destinations = BTreeSet::<(u32, i32, i32)>::new();
    for page_id in page_ids {
        let page_obj = parse_pdf_object_body_v0(&pdf, page_id).expect("page object");
        let annots = parse_pdf_ref_ids_v0(&page_obj, "/Annots");
        for annot_id in annots {
            let annotation = parse_pdf_object_body_v0(&pdf, annot_id).expect("annotation");
            if annotation.contains("/XYZ") {
                let (dest_page, x_pt, y_pt) =
                    parse_pdf_annotation_dest_xyz_v0(&annotation).expect("xyz annotation dest");
                known_anchor_destinations.insert((
                    dest_page,
                    (x_pt * 100.0).round() as i32,
                    (y_pt * 100.0).round() as i32,
                ));
            }
        }
    }
    assert!(
        !known_anchor_destinations.is_empty(),
        "expected at least one known anchor destination from annotations"
    );

    let catalog = parse_pdf_object_body_v0(&pdf, 1).expect("catalog object");
    let outline_root_id =
        parse_pdf_single_ref_id_v0(&catalog, "/Outlines").expect("catalog outlines ref");
    let outline_item_ids = collect_outline_item_ids_depth_first_v0(&pdf, outline_root_id)
        .expect("outline item traversal");
    assert!(
        !outline_item_ids.is_empty(),
        "expected outline items for toc metadata"
    );
    for item_id in outline_item_ids {
        let item_body = parse_pdf_object_body_v0(&pdf, item_id).expect("outline item body");
        assert!(
            item_body.contains("/Dest [") && item_body.contains("/XYZ"),
            "outline items must target /XYZ anchor destinations: {item_body}"
        );
        let (dest_page, x_pt, y_pt) =
            parse_pdf_annotation_dest_xyz_v0(&item_body).expect("outline xyz dest");
        let key = (
            dest_page,
            (x_pt * 100.0).round() as i32,
            (y_pt * 100.0).round() as i32,
        );
        assert!(
            known_anchor_destinations.contains(&key),
            "outline destination should resolve to known anchor destination: {item_body}"
        );
    }
}

#[test]
fn pdf_renderer_rejects_toc_link_annotation_with_missing_anchor_destination_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"Prelude.\n\n!toc\n\n!toc 1 9 <Missing anchor>")
        .expect("writer bytes");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed when toc link anchor target is missing"
    );
}

#[test]
fn pdf_renderer_rejects_toc_entries_with_unsupported_level_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"!toc\n\n!toc 3 1 Too deep")
        .expect("writer should accept bytes");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed for unsupported toc level"
    );
}

#[test]
fn pdf_renderer_rejects_outline_with_unnestable_toc_levels_v0() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Prelude.\n\n!toc\n\n@s {Orphan subsection}\n\n!toc 2 1 Orphan subsection",
    )
    .expect("writer should accept toc marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed when toc level-2 appears without a level-1 parent"
    );
}

#[test]
fn pdf_renderer_toc_block_renders_between_surrounding_paragraphs_v0() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Before block.\n\n!toc\n\nAfter block.\n\n@S {Anchor One}\n\n@s {Anchor Two}\n\n!toc 1 1 Intro toc line\n!toc 2 2 Detail toc line",
    )
    .expect("writer should accept toc marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let (before_x, before_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Before block.)").expect("before");
    let (toc_title_x, toc_title_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Contents)").expect("toc title");
    let (_, toc_intro_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Intro toc line)").expect("toc intro");
    let (_, toc_detail_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Detail toc line)").expect("toc detail");
    let (after_x, after_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(After block.)").expect("after");

    assert!(
        before_y > toc_title_y,
        "toc block should appear below preceding paragraph"
    );
    assert!(
        toc_title_y > toc_intro_y,
        "toc entries should appear below toc title"
    );
    assert!(
        toc_intro_y > toc_detail_y,
        "toc level 2 line should appear below level 1 line"
    );
    assert!(
        toc_detail_y > after_y,
        "toc block should appear above following paragraph"
    );
    assert!(
        toc_title_x >= 72.0,
        "toc title should remain in printable area"
    );
    assert!(
        before_x > 0.0 && after_x > 0.0,
        "paragraph coordinates must be valid"
    );
}

#[test]
fn pdf_renderer_toc_without_entries_renders_title_only_v0() {
    let xdv =
        write_dvi_v2_text_page_v0(b"Intro.\n\n!toc\n\nOutro.").expect("writer should accept bytes");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    assert!(
        pdf_text.contains("(Contents) Tj"),
        "toc title should render"
    );
    assert!(
        !pdf_text.contains("!toc"),
        "toc placeholder should be hidden"
    );
}

#[test]
fn pdf_renderer_rejects_toc_entry_with_non_numeric_anchor_id_v0() {
    let xdv =
        write_dvi_v2_text_page_v0(b"!toc\n\n!toc 1 abc Intro").expect("writer should accept bytes");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed on non-numeric toc anchor ids"
    );
}

#[test]
fn pdf_renderer_rejects_toc_entry_with_empty_title_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"!toc\n\n!toc 1 1 ").expect("writer should accept bytes");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed on empty toc titles"
    );
}

#[test]
fn pdf_renderer_rejects_toc_entry_with_zero_anchor_id_v0() {
    let xdv =
        write_dvi_v2_text_page_v0(b"!toc\n\n!toc 1 0 Intro").expect("writer should accept bytes");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed on zero toc anchors"
    );
}

#[test]
fn pdf_renderer_rejects_toc_entry_with_unterminated_metadata_shape_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"!toc\n\n!toc 1").expect("writer should accept bytes");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed on malformed toc metadata lines"
    );
}

#[test]
fn pdf_renderer_rejects_duplicate_toc_anchor_ids_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"!toc\n\n!toc 1 1 Intro\n!toc 2 1 Duplicate anchor")
        .expect("writer should accept bytes");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed on duplicate toc anchors"
    );
}

