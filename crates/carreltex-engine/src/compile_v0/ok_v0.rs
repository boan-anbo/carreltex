use crate::tex::tokenize_v0::TokenV0;
#[path = "ok_v0_env_support.rs"]
mod ok_v0_env_support;
#[path = "ok_v0_env_refs.rs"]
mod ok_v0_env_refs;
#[path = "ok_v0_optional_brackets.rs"]
mod ok_v0_optional_brackets;
#[path = "ok_v0_dollar_math.rs"]
mod ok_v0_dollar_math;
#[path = "ok_v0_ensuremath.rs"]
mod ok_v0_ensuremath;
#[path = "ok_v0_lists.rs"]
mod ok_v0_lists;
#[path = "ok_v0_biblabel.rs"]
mod ok_v0_biblabel;
#[path = "ok_v0_noops.rs"]
mod ok_v0_noops;
#[path = "ok_v0_markers.rs"]
mod ok_v0_markers;
use ok_v0_env_refs::{emit_ok_markers_in_env_v0, OkEnvMarkersV0};
use ok_v0_optional_brackets::{
    consume_optional_digits_bracket_span_v0, consume_optional_heading_short_title_v0,
    consume_optional_nested_bracket_span_v0, consume_optional_simple_bracket_span_v0,
};
use ok_v0_dollar_math::{consume_display_math_dollar_span_v0, consume_inline_math_dollar_span_v0};
use ok_v0_ensuremath::{consume_inline_math_group_span_v0, consume_math_control_span_v0};
use ok_v0_lists::{begin_list_v0, emit_list_item_v0, end_list_v0, list_stack_active_v0, ListStateV0};
use ok_v0_biblabel::consume_optional_bibitem_label_fragment_v0;
use ok_v0_markers::{is_cite_marker_command_v0, is_ref_marker_command_v0, ok_marker_command_v0};
use ok_v0_env_support::{
    consume_named_environment_span_v0, is_supported_display_math_env_v0,
    is_supported_ok_block_env_v0, is_supported_ok_table_stub_env_v0, ok_thm_stub_marker_v0,
};
pub(crate) const MAX_OK_TEXT_BYTES_V0: usize = 64 * 1024;
pub(crate) const OK_GLYPH_ADVANCE_SP_V0: i32 = 65_536;
pub(crate) const OK_LINE_ADVANCE_SP_V0: i32 = 786_432;
const MAX_OK_GROUP_DEPTH_V0: usize = 64;
const MAX_OK_BRACKET_BYTES_V0: usize = 256;
const MAX_OK_MATH_SCAN_TOKENS_V0: usize = 4096;
const MAX_OK_MATH_ENV_TOKENS_V0: usize = 4096;
const MAX_OK_DOLLAR_MATH_TOKENS_V0: usize = 4096;
const MAX_OK_ENSUREMATH_TOKENS_V0: usize = 4096;
const MAX_OK_HEADING_SHORT_TOKENS_V0: usize = 2048;
const MAX_OK_CITE_NOTE_TOKENS_V0: usize = 2048;
const MAX_OK_REF_NOTE_TOKENS_V0: usize = 2048;
const MAX_OK_BIBLABEL_TOKENS_V0: usize = 256;
fn skip_spaces(tokens: &[TokenV0], mut index: usize) -> usize {
    while matches!(tokens.get(index), Some(TokenV0::Space)) {
        index += 1;
    }
    index
}
fn skip_spaces_until(tokens: &[TokenV0], mut index: usize, end_limit: usize) -> usize {
    while index < end_limit && matches!(tokens.get(index), Some(TokenV0::Space)) {
        index += 1;
    }
    index
}
fn consume_group_literal(tokens: &[TokenV0], mut index: usize, literal: &[u8]) -> Option<usize> {
    if !matches!(tokens.get(index), Some(TokenV0::BeginGroup)) {
        return None;
    }
    index += 1;
    for expected in literal {
        if !matches!(tokens.get(index), Some(TokenV0::Char(value)) if value == expected) {
            return None;
        }
        index += 1;
    }
    if !matches!(tokens.get(index), Some(TokenV0::EndGroup)) {
        return None;
    }
    Some(index + 1)
}
fn consume_char_space_group_non_empty(tokens: &[TokenV0], mut index: usize) -> Option<usize> {
    if !matches!(tokens.get(index), Some(TokenV0::BeginGroup)) {
        return None;
    }
    index += 1;
    let mut has_non_space_char = false;
    loop {
        match tokens.get(index) {
            Some(TokenV0::EndGroup) if has_non_space_char => return Some(index + 1),
            Some(TokenV0::EndGroup) => return None,
            Some(TokenV0::Space) => {
                index += 1;
            }
            Some(TokenV0::Char(byte)) => {
                if *byte != b' ' {
                    has_non_space_char = true;
                }
                index += 1;
            }
            _ => return None,
        }
    }
}
fn consume_bracket_options_non_empty(tokens: &[TokenV0], mut index: usize) -> Option<usize> {
    if !matches!(tokens.get(index), Some(TokenV0::Char(b'['))) {
        return None;
    }
    index += 1;
    let mut has_non_space_char = false;
    loop {
        match tokens.get(index) {
            Some(TokenV0::Char(b']')) if has_non_space_char => return Some(index + 1),
            Some(TokenV0::Char(b']')) => return None,
            Some(TokenV0::Space) => {
                index += 1;
            }
            Some(TokenV0::Char(byte)) => {
                if *byte != b' ' {
                    has_non_space_char = true;
                }
                index += 1;
            }
            _ => return None,
        }
    }
}
fn consume_usepackage_preamble_command(tokens: &[TokenV0], mut index: usize) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"usepackage"
    ) {
        return None;
    }
    index += 1;
    index = skip_spaces(tokens, index);
    if matches!(tokens.get(index), Some(TokenV0::Char(b'['))) {
        index = consume_bracket_options_non_empty(tokens, index)?;
        index = skip_spaces(tokens, index);
    }
    index = consume_char_space_group_non_empty(tokens, index)?;
    Some(skip_spaces(tokens, index))
}
fn is_supported_meta_preamble_command(name: &[u8]) -> bool {
    matches!(name, b"title" | b"author" | b"date")
}
fn is_supported_bibliography_preamble_command(name: &[u8]) -> bool {
    matches!(name, b"bibliographystyle" | b"bibliography")
}
fn consume_meta_preamble_command(tokens: &[TokenV0], mut index: usize) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if is_supported_meta_preamble_command(name.as_slice())
    ) {
        return None;
    }
    index += 1;
    index = skip_spaces(tokens, index);
    index = consume_char_space_group_non_empty(tokens, index)?;
    Some(skip_spaces(tokens, index))
}
fn consume_char_space_nested_group_non_empty_v0(
    tokens: &[TokenV0],
    index: usize,
    end: usize,
) -> Option<usize> {
    let cursor = skip_spaces_until(tokens, index, end);
    let (_, inner_end, next_index) =
        consume_balanced_group_bounds_v0(tokens, cursor, MAX_OK_GROUP_DEPTH_V0, end)?;
    let mut scan = cursor + 1;
    let mut has_non_space_char = false;
    while scan < inner_end {
        match tokens.get(scan)? {
            TokenV0::Char(byte) => {
                if *byte != b' ' {
                    has_non_space_char = true;
                }
                scan += 1;
            }
            TokenV0::Space | TokenV0::BeginGroup | TokenV0::EndGroup => {
                scan += 1;
            }
            _ => return None,
        }
    }
    if !has_non_space_char {
        return None;
    }
    Some(next_index)
}

fn consume_bibliography_preamble_command(tokens: &[TokenV0], mut index: usize) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if is_supported_bibliography_preamble_command(name.as_slice())
    ) {
        return None;
    }
    index += 1;
    index = consume_char_space_nested_group_non_empty_v0(tokens, index, tokens.len())?;
    Some(skip_spaces(tokens, index))
}

fn is_supported_ok_char_v0(byte: u8) -> bool {
    (0x20..=0x7e).contains(&byte)
}

fn is_supported_ok_wrapper_command_v0(name: &[u8]) -> bool {
    matches!(
        name,
        b"textbf"
            | b"textit"
            | b"emph"
            | b"texttt"
            | b"underline"
            | b"textrm"
            | b"textsf"
            | b"textsc"
            | b"textsl"
            | b"textmd"
            | b"textup"
            | b"textsuperscript"
            | b"textsubscript"
    )
}

fn is_supported_ok_heading_command_v0(name: &[u8]) -> bool {
    matches!(
        name,
        b"section" | b"subsection" | b"subsubsection" | b"paragraph" | b"subparagraph"
    )
}

fn is_supported_ok_style_declaration_v0(name: &[u8]) -> bool {
    matches!(
        name,
        b"bfseries"
            | b"mdseries"
            | b"itshape"
            | b"slshape"
            | b"scshape"
            | b"upshape"
            | b"rmfamily"
            | b"sffamily"
            | b"ttfamily"
            | b"em"
            | b"centering"
    )
}

fn consume_ok_group_fragment_v0(
    tokens: &[TokenV0],
    index: usize,
    end: usize,
    body: &mut Vec<u8>,
    previous_was_space: &mut bool,
) -> Option<usize> {
    let cursor = skip_spaces_until(tokens, index, end);
    let (inner_start, inner_end, next_index) =
        consume_balanced_group_bounds_v0(tokens, cursor, MAX_OK_GROUP_DEPTH_V0, end)?;
    let mut nested_list_env = None;
    consume_ok_body_range_v0(
        tokens,
        inner_start,
        inner_end,
        true,
        &mut nested_list_env,
        body,
        previous_was_space,
    )?;
    Some(next_index)
}

fn consume_ok_group_fragment_discard_v0(tokens: &[TokenV0], index: usize, end: usize) -> Option<usize> {
    let mut scratch_body = Vec::new();
    let mut scratch_previous_was_space = false;
    consume_ok_group_fragment_v0(
        tokens,
        index,
        end,
        &mut scratch_body,
        &mut scratch_previous_was_space,
    )
}

pub(super) fn consume_char_space_nested_group_v0(tokens: &[TokenV0], index: usize, end: usize) -> Option<usize> {
    let cursor = skip_spaces_until(tokens, index, end);
    let (_, inner_end, next_index) =
        consume_balanced_group_bounds_v0(tokens, cursor, MAX_OK_GROUP_DEPTH_V0, end)?;
    let mut scan = cursor + 1;
    while scan < inner_end {
        match tokens.get(scan)? {
            TokenV0::Char(_) | TokenV0::Space | TokenV0::BeginGroup | TokenV0::EndGroup => {
                scan += 1;
            }
            _ => return None,
        }
    }
    Some(next_index)
}

fn consume_balanced_group_bounds_v0(
    tokens: &[TokenV0],
    index: usize,
    depth_cap: usize,
    end_limit: usize,
) -> Option<(usize, usize, usize)> {
    if !matches!(tokens.get(index), Some(TokenV0::BeginGroup)) {
        return None;
    }
    let mut depth = 1usize;
    let mut cursor = index + 1;
    let content_start = cursor;
    while cursor < end_limit {
        let token = tokens.get(cursor)?;
        match token {
            TokenV0::BeginGroup => {
                depth += 1;
                if depth > depth_cap {
                    return None;
                }
            }
            TokenV0::EndGroup => {
                depth -= 1;
                if depth == 0 {
                    return Some((content_start, cursor, cursor + 1));
                }
            }
            _ => {}
        }
        cursor += 1;
    }
    None
}

fn emit_ok_inline_math_marker_v0(body: &mut Vec<u8>, previous_was_space: &mut bool) {
    body.push(b' ');
    body.push(b'[');
    body.push(b'M');
    body.push(b'A');
    body.push(b'T');
    body.push(b'H');
    body.push(b']');
    *previous_was_space = false;
}

fn emit_ok_display_math_marker_v0(body: &mut Vec<u8>, previous_was_space: &mut bool) {
    body.push(0x0a);
    body.push(b'[');
    body.push(b'M');
    body.push(b'A');
    body.push(b'T');
    body.push(b'H');
    body.push(b']');
    body.push(0x0a);
    *previous_was_space = true;
}

fn emit_ok_block_marker_v0(body: &mut Vec<u8>, marker: &[u8], previous_was_space: &mut bool) {
    body.push(0x0a);
    body.push(b'[');
    body.extend_from_slice(marker);
    body.push(b']');
    body.push(0x0a);
    *previous_was_space = true;
}

fn consume_ok_body_range_v0(
    tokens: &[TokenV0],
    start: usize,
    end: usize,
    allow_nested_groups: bool,
    list_env: &mut Option<ListStateV0>,
    body: &mut Vec<u8>,
    previous_was_space: &mut bool,
) -> Option<()> {
    let mut index = start;
    while index < end {
        index = consume_ok_body_token_v0(
            tokens,
            index,
            end,
            allow_nested_groups,
            list_env,
            body,
            previous_was_space,
        )?;
    }
    Some(())
}

fn consume_ok_body_token_v0(
    tokens: &[TokenV0],
    index: usize,
    end: usize,
    allow_nested_groups: bool,
    list_env: &mut Option<ListStateV0>,
    body: &mut Vec<u8>,
    previous_was_space: &mut bool,
) -> Option<usize> {
    if index >= end {
        return None;
    }
    match tokens.get(index) {
        Some(TokenV0::Space) => {
            if !*previous_was_space {
                body.push(b' ');
                *previous_was_space = true;
            }
            Some(index + 1)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"par" => {
            if !*previous_was_space {
                body.push(b' ');
                *previous_was_space = true;
            }
            Some(index + 1)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"newblock" => {
            if !matches!(list_env, Some(ListStateV0::Thebibliography)) {
                return None;
            }
            if !*previous_was_space {
                body.push(b' ');
                *previous_was_space = true;
            }
            Some(index + 1)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"maketitle" => Some(index + 1),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"noindent" => Some(index + 1),
        Some(TokenV0::ControlSeq(name))
            if is_supported_ok_style_declaration_v0(name.as_slice()) =>
        {
            Some(index + 1)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"newline" => {
            body.push(0x0a);
            *previous_was_space = true;
            Some(index + 1)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"\\" => {
            let mut next_index = index + 1;
            if matches!(tokens.get(next_index), Some(TokenV0::Char(b'*')))
                || matches!(
                    tokens.get(next_index),
                    Some(TokenV0::ControlSeq(star)) if star.as_slice() == b"*"
                )
            {
                next_index += 1;
            }
            body.push(0x0a);
            *previous_was_space = true;
            Some(next_index)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"(" => {
            let next_index =
                consume_math_control_span_v0(tokens, index, end, b")", MAX_OK_MATH_SCAN_TOKENS_V0)?;
            emit_ok_inline_math_marker_v0(body, previous_was_space);
            Some(next_index)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"[" => {
            let next_index =
                consume_math_control_span_v0(tokens, index, end, b"]", MAX_OK_MATH_SCAN_TOKENS_V0)?;
            emit_ok_display_math_marker_v0(body, previous_was_space);
            Some(next_index)
        }
        Some(TokenV0::ControlSeq(name))
            if matches!(name.as_slice(), b"ensuremath" | b"text" | b"textnormal" | b"mathrm" | b"mathit" | b"mathbf" | b"mathbb" | b"mathcal" | b"mathsf" | b"mathtt") =>
        {
            let next_index = consume_inline_math_group_span_v0(
                tokens,
                index,
                end,
                MAX_OK_ENSUREMATH_TOKENS_V0,
                MAX_OK_GROUP_DEPTH_V0,
            )?;
            emit_ok_inline_math_marker_v0(body, previous_was_space);
            Some(next_index)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"$" => {
            body.push(b'$');
            *previous_was_space = false;
            Some(index + 1)
        }
        Some(TokenV0::Char(b'$')) if matches!(tokens.get(index + 1), Some(TokenV0::Char(b'$'))) => {
            let next_index =
                consume_display_math_dollar_span_v0(tokens, index, end, MAX_OK_DOLLAR_MATH_TOKENS_V0)?;
            emit_ok_display_math_marker_v0(body, previous_was_space);
            Some(next_index)
        }
        Some(TokenV0::Char(b'$')) => consume_inline_math_dollar_span_v0(
            tokens,
            index,
            end,
            MAX_OK_DOLLAR_MATH_TOKENS_V0,
        )
        .map(|next_index| {
            emit_ok_inline_math_marker_v0(body, previous_was_space);
            next_index
        }),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"footnotemark" => {
            consume_optional_digits_bracket_span_v0(tokens, index + 1, end, 32)
        }
        Some(TokenV0::ControlSeq(name))
            if name.as_slice() == b"footnote" || name.as_slice() == b"footnotetext" =>
        {
            let arg_start = consume_optional_digits_bracket_span_v0(tokens, index + 1, end, 32)?;
            let mut footnote_text = Vec::new();
            let mut footnote_previous_was_space = false;
            let next_index = consume_ok_group_fragment_v0(
                tokens,
                arg_start,
                end,
                &mut footnote_text,
                &mut footnote_previous_was_space,
            )?;
            body.push(b' ');
            body.push(b'[');
            body.extend_from_slice(&footnote_text);
            body.push(b']');
            *previous_was_space = false;
            Some(next_index)
        }
        Some(TokenV0::ControlSeq(name))
            if ok_v0_noops::is_ok_noop_command_v0(name.as_slice()) =>
        {
            ok_v0_noops::consume_ok_noop_command_v0(tokens, index, end, name.as_slice())
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"label" => {
            consume_char_space_nested_group_v0(tokens, index + 1, end)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"includegraphics" => {
            let mut cursor =
                consume_optional_simple_bracket_span_v0(tokens, index + 1, end, MAX_OK_BRACKET_BYTES_V0)?;
            cursor = consume_char_space_nested_group_non_empty_v0(tokens, cursor, end)?;
            body.push(b' ');
            body.push(b'[');
            body.push(b'I');
            body.push(b'M');
            body.push(b'G');
            body.push(b']');
            *previous_was_space = false;
            Some(cursor)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"caption" => {
            let mut cursor = skip_spaces_until(tokens, index + 1, end);
            if matches!(tokens.get(cursor), Some(TokenV0::Char(b'*'))) {
                cursor += 1;
            }
            cursor = consume_optional_simple_bracket_span_v0(tokens, cursor, end, MAX_OK_BRACKET_BYTES_V0)?;
            let mut caption_text = Vec::new();
            let mut caption_previous_was_space = false;
            cursor = consume_ok_group_fragment_v0(
                tokens,
                cursor,
                end,
                &mut caption_text,
                &mut caption_previous_was_space,
            )?;
            body.push(0x0a);
            body.extend_from_slice(&caption_text);
            body.push(0x0a);
            *previous_was_space = true;
            Some(cursor)
        }
        Some(TokenV0::ControlSeq(name))
            if ok_marker_command_v0(name.as_slice()).is_some() =>
        {
            let marker = ok_marker_command_v0(name.as_slice())?;
            let mut arg_start = index + 1;
            if is_cite_marker_command_v0(name.as_slice()) {
                arg_start = skip_spaces_until(tokens, arg_start, end);
                if matches!(tokens.get(arg_start), Some(TokenV0::Char(b'*'))) {
                    arg_start += 1;
                }
            }
            if is_cite_marker_command_v0(name.as_slice()) || is_ref_marker_command_v0(name.as_slice()) {
                let max_note_tokens = if is_cite_marker_command_v0(name.as_slice()) {
                    MAX_OK_CITE_NOTE_TOKENS_V0
                } else {
                    MAX_OK_REF_NOTE_TOKENS_V0
                };
                arg_start = consume_optional_nested_bracket_span_v0(
                    tokens,
                    arg_start,
                    end,
                    max_note_tokens,
                    MAX_OK_GROUP_DEPTH_V0,
                )?;
                arg_start = consume_optional_nested_bracket_span_v0(
                    tokens,
                    arg_start,
                    end,
                    max_note_tokens,
                    MAX_OK_GROUP_DEPTH_V0,
                )?;
            }
            let next_index = consume_char_space_nested_group_v0(tokens, arg_start, end)?;
            body.push(b' ');
            body.push(b'[');
            body.extend_from_slice(marker);
            body.push(b']');
            *previous_was_space = false;
            Some(next_index)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"href" => {
            let mut scratch_body = Vec::new();
            let mut scratch_previous_was_space = *previous_was_space;
            let first_arg_end = consume_ok_group_fragment_v0(
                tokens,
                index + 1,
                end,
                &mut scratch_body,
                &mut scratch_previous_was_space,
            )?;
            consume_ok_group_fragment_v0(tokens, first_arg_end, end, body, previous_was_space)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"url" => {
            consume_ok_group_fragment_v0(tokens, index + 1, end, body, previous_was_space)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"textcolor" => {
            let first_arg_end = consume_ok_group_fragment_discard_v0(tokens, index + 1, end)?;
            consume_ok_group_fragment_v0(tokens, first_arg_end, end, body, previous_was_space)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"color" => {
            consume_ok_group_fragment_discard_v0(tokens, index + 1, end)
        }
        Some(TokenV0::ControlSeq(name))
            if name.as_slice() == b"enquote" || name.as_slice() == b"quote" =>
        {
            let mut quote_text = Vec::new();
            let mut quote_previous_was_space = false;
            let next_index = consume_ok_group_fragment_v0(
                tokens,
                index + 1,
                end,
                &mut quote_text,
                &mut quote_previous_was_space,
            )?;
            body.push(b'"');
            body.extend_from_slice(&quote_text);
            body.push(b'"');
            *previous_was_space = false;
            Some(next_index)
        }
        Some(TokenV0::ControlSeq(name))
            if is_supported_ok_wrapper_command_v0(name.as_slice()) =>
        {
            let mut cursor = skip_spaces_until(tokens, index + 1, end);
            let (inner_start, inner_end, next_index) =
                consume_balanced_group_bounds_v0(tokens, cursor, MAX_OK_GROUP_DEPTH_V0, end)?;
            consume_ok_body_range_v0(
                tokens,
                inner_start,
                inner_end,
                true,
                list_env,
                body,
                previous_was_space,
            )?;
            cursor = next_index;
            Some(cursor)
        }
        Some(TokenV0::ControlSeq(name))
            if is_supported_ok_heading_command_v0(name.as_slice()) =>
        {
            let mut cursor = skip_spaces_until(tokens, index + 1, end);
            if matches!(tokens.get(cursor), Some(TokenV0::Char(b'*'))) {
                cursor += 1;
            }
            cursor = consume_optional_heading_short_title_v0(
                tokens,
                cursor,
                end,
                MAX_OK_HEADING_SHORT_TOKENS_V0,
                MAX_OK_GROUP_DEPTH_V0,
            )?;
            cursor = skip_spaces_until(tokens, cursor, end);
            let (inner_start, inner_end, next_index) =
                consume_balanced_group_bounds_v0(tokens, cursor, MAX_OK_GROUP_DEPTH_V0, end)?;
            body.push(0x0a);
            *previous_was_space = true;
            consume_ok_body_range_v0(
                tokens,
                inner_start,
                inner_end,
                true,
                list_env,
                body,
                previous_was_space,
            )?;
            body.push(0x0a);
            *previous_was_space = true;
            cursor = next_index;
            Some(cursor)
        }
        Some(TokenV0::ControlSeq(name))
            if !allow_nested_groups && name.as_slice() == b"begin" =>
        {
            let cursor = skip_spaces_until(tokens, index + 1, end);
            if list_stack_active_v0(list_env) {
                return begin_list_v0(tokens, cursor, list_env);
            }
            if list_env.is_some() {
                return None;
            }
            if let Some((env_name, inner_start, inner_end, next_index)) =
                consume_named_environment_span_v0(tokens, index, end)
            {
                if is_supported_ok_block_env_v0(&env_name) {
                    if env_name.as_slice() == b"verbatim" {
                        emit_ok_block_marker_v0(body, b"VERBATIM", previous_was_space);
                        return Some(next_index);
                    }
                    let mut inner_list_env = None;
                    body.push(0x0a);
                    *previous_was_space = true;
                    consume_ok_body_range_v0(
                        tokens,
                        inner_start,
                        inner_end,
                        false,
                        &mut inner_list_env,
                        body,
                        previous_was_space,
                    )?;
                    if inner_list_env.is_some() {
                        return None;
                    }
                    body.push(0x0a);
                    *previous_was_space = true;
                    return Some(next_index);
                }
                if is_supported_ok_table_stub_env_v0(&env_name) {
                    emit_ok_block_marker_v0(body, b"TABLE", previous_was_space);
                    return Some(next_index);
                }
                if let Some(marker) = ok_thm_stub_marker_v0(&env_name) {
                    emit_ok_block_marker_v0(body, marker, previous_was_space);
                    emit_ok_markers_in_env_v0(
                        tokens,
                        inner_start,
                        inner_end,
                        OkEnvMarkersV0::TheoremFamily,
                        body,
                        previous_was_space,
                    )?;
                    return Some(next_index);
                }
                if is_supported_display_math_env_v0(&env_name) {
                    if next_index - index > MAX_OK_MATH_ENV_TOKENS_V0 {
                        return None;
                    }
                    emit_ok_display_math_marker_v0(body, previous_was_space);
                    emit_ok_markers_in_env_v0(
                        tokens,
                        inner_start,
                        inner_end,
                        OkEnvMarkersV0::EquationFamily,
                        body,
                        previous_was_space,
                    )?;
                    return Some(next_index);
                }
            }
            if let Some(next_index) = begin_list_v0(tokens, cursor, list_env) {
                return Some(next_index);
            }
            if let Some(begin_env_end) = consume_group_literal(tokens, cursor, b"figure") {
                let next_index = consume_optional_simple_bracket_span_v0(
                    tokens,
                    begin_env_end,
                    end,
                    MAX_OK_BRACKET_BYTES_V0,
                )?;
                *list_env = Some(ListStateV0::Figure);
                return Some(next_index);
            }
            if let Some(begin_env_end) = consume_group_literal(tokens, cursor, b"table") {
                let next_index = consume_optional_simple_bracket_span_v0(
                    tokens,
                    begin_env_end,
                    end,
                    MAX_OK_BRACKET_BYTES_V0,
                )?;
                *list_env = Some(ListStateV0::Table);
                return Some(next_index);
            }
            if let Some(begin_env_end) = consume_group_literal(tokens, cursor, b"thebibliography") {
                let next_index =
                    consume_char_space_nested_group_non_empty_v0(tokens, begin_env_end, end)?;
                *list_env = Some(ListStateV0::Thebibliography);
                return Some(next_index);
            }
            None
        }
        Some(TokenV0::ControlSeq(name))
            if !allow_nested_groups && name.as_slice() == b"end" =>
        {
            let cursor = skip_spaces_until(tokens, index + 1, end);
            if let Some(next_index) = end_list_v0(tokens, cursor, list_env, body, previous_was_space) {
                return Some(next_index);
            }
            match list_env {
                Some(ListStateV0::Thebibliography) => {
                    let next_index = consume_group_literal(tokens, cursor, b"thebibliography")?;
                    body.push(0x0a);
                    *previous_was_space = true;
                    *list_env = None;
                    Some(next_index)
                }
                Some(ListStateV0::Figure) => {
                    let next_index = consume_group_literal(tokens, cursor, b"figure")?;
                    *list_env = None;
                    Some(next_index)
                }
                Some(ListStateV0::Table) => {
                    let next_index = consume_group_literal(tokens, cursor, b"table")?;
                    *list_env = None;
                    Some(next_index)
                }
                None => None,
                Some(ListStateV0::Lists(_)) => None,
            }
        }
        Some(TokenV0::ControlSeq(name))
            if !allow_nested_groups && name.as_slice() == b"item" =>
        {
            emit_list_item_v0(list_env, body, previous_was_space).map(|()| index + 1)
        }
        Some(TokenV0::ControlSeq(name))
            if !allow_nested_groups && name.as_slice() == b"bibitem" =>
        {
            match list_env {
                Some(ListStateV0::Thebibliography) => {
                    let (after_optional, label) = consume_optional_bibitem_label_fragment_v0(
                        tokens,
                        index + 1,
                        end,
                        MAX_OK_BIBLABEL_TOKENS_V0,
                        MAX_OK_GROUP_DEPTH_V0,
                    )?;
                    let next_index =
                        consume_char_space_nested_group_non_empty_v0(tokens, after_optional, end)?;
                    body.push(0x0a);
                    body.push(b'-');
                    body.push(b' ');
                    if let Some(label_bytes) = label {
                        body.push(b'[');
                        body.extend_from_slice(&label_bytes);
                        body.push(b']');
                        body.push(b' ');
                    }
                    *previous_was_space = true;
                    Some(next_index)
                }
                _ => None,
            }
        }
        Some(TokenV0::BeginGroup) if allow_nested_groups => {
            let (inner_start, inner_end, next_index) =
                consume_balanced_group_bounds_v0(tokens, index, MAX_OK_GROUP_DEPTH_V0, end)?;
            consume_ok_body_range_v0(
                tokens,
                inner_start,
                inner_end,
                true,
                list_env,
                body,
                previous_was_space,
            )?;
            Some(next_index)
        }
        Some(TokenV0::Char(0x0c)) => {
            body.push(0x0c);
            *previous_was_space = false;
            Some(index + 1)
        }
        Some(TokenV0::Char(0x0a)) => {
            body.push(0x0a);
            *previous_was_space = true;
            Some(index + 1)
        }
        Some(TokenV0::Char(byte)) if is_supported_ok_char_v0(*byte) => {
            body.push(*byte);
            *previous_was_space = false;
            Some(index + 1)
        }
        _ => None,
    }
}

pub(crate) fn extract_strict_ok_text_body_v0(tokens: &[TokenV0]) -> Option<Vec<u8>> {
    let mut index = 0usize;
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"documentclass"
    ) {
        return None;
    }
    index += 1;
    index = skip_spaces(tokens, index);
    if matches!(tokens.get(index), Some(TokenV0::Char(b'['))) {
        index = consume_bracket_options_non_empty(tokens, index)?;
    }
    index = skip_spaces(tokens, index);
    index = consume_group_literal(tokens, index, b"article")?;
    index = skip_spaces(tokens, index);
    loop {
        match tokens.get(index) {
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"usepackage" => {
                index = consume_usepackage_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if is_supported_meta_preamble_command(name.as_slice()) =>
            {
                index = consume_meta_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if is_supported_bibliography_preamble_command(name.as_slice()) =>
            {
                index = consume_bibliography_preamble_command(tokens, index)?;
                continue;
            }
            _ => {}
        }
        break;
    }

    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"begin"
    ) {
        return None;
    }
    index += 1;
    index = skip_spaces(tokens, index);
    index = consume_group_literal(tokens, index, b"document")?;
    index = skip_spaces(tokens, index);

    let mut body = Vec::<u8>::new();
    let mut previous_was_space = false;
    let mut list_env: Option<ListStateV0> = None;
    loop {
        if list_env.is_none()
            && matches!(
                tokens.get(index),
                Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"end"
            )
            && consume_group_literal(tokens, skip_spaces(tokens, index + 1), b"document").is_some()
        {
            break;
        }
        index = consume_ok_body_token_v0(
            tokens,
            index,
            tokens.len(),
            false,
            &mut list_env,
            &mut body,
            &mut previous_was_space,
        )?;
    }
    if list_env.is_some() {
        return None;
    }

    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"end"
    ) {
        return None;
    }
    index += 1;
    index = skip_spaces(tokens, index);
    index = consume_group_literal(tokens, index, b"document")?;
    index = skip_spaces(tokens, index);
    if index != tokens.len() {
        return None;
    }
    Some(body)
}
