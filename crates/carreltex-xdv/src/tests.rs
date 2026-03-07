use super::{
    count_dvi_v2_text_movements_v0, count_dvi_v2_text_pages_v0,
    count_dvi_v2_text_pages_with_advance_v0, parse_dvi_v2_text_page_to_layout_v0, plan_layout_v0,
    plan_layout_width_v0, recompute_line_width_sp_v0, render_dvi_v2_text_page_to_pdf_v0,
    sum_dvi_v2_positive_right3_amounts_with_layout_v0, validate_dvi_v2_empty_page_v0,
    validate_dvi_v2_text_page_matches_layout_v0, validate_dvi_v2_text_page_v0,
    validate_dvi_v2_text_page_with_layout_v0, write_dvi_v2_empty_page_v0,
    write_dvi_v2_text_page_from_layout_v0, write_dvi_v2_text_page_v0,
    write_dvi_v2_text_page_with_advance_v0, write_dvi_v2_text_page_with_layout_and_wrap_v0,
    write_dvi_v2_text_page_with_layout_v0, write_dvi_v2_text_page_with_layout_wrap_and_paging_v0,
    LinePlanV0, DVI_DOWN3, DVI_EOP, DVI_FNT_DEF1, DVI_PRE, DVI_RIGHT3, DVI_TRAILER_BYTE,
};
use std::collections::BTreeSet;

// DVI writer/planner and baseline layout behavior.
include!("tests/writer_planner_v0.rs");
// Shared PDF/layout test helpers used across renderer test files.
include!("tests/pdf_test_support_v0.rs");
// PDF layout helpers and alignment/rhythm invariants, split into concern modules.
#[path = "tests/pdf_layout_and_alignment_v0/mod.rs"]
mod pdf_layout_and_alignment_v0;
// Parse roundtrip and core renderer/link/annotation behavior.
include!("tests/roundtrip_and_core_renderer_v0.rs");
// Table and float rendering seams.
include!("tests/table_and_float_v0.rs");
// TOC and outline/bookmark seams.
include!("tests/toc_outline_v0.rs");
// Low-level validator and wrap/paging guards.
include!("tests/validator_and_wrap_v0.rs");
