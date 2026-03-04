use crate::tex::tokenize_v0::TokenV0;

use super::ok_v0_body::{
    consume_bracket_options_non_empty, consume_char_space_group_non_empty,
    consume_char_space_nested_group_non_empty_v0, consume_group_literal, consume_ok_body_token_v0,
    consume_ok_group_fragment_v0, is_supported_ok_style_declaration_v0, skip_spaces,
};
use super::ok_v0_lists::ListStateV0;
use super::ok_v0_noops::{consume_ok_noop_command_v0, is_ok_noop_command_v0};
use super::ok_v0_title_state::OkTitleStateV0;

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

fn consume_meta_preamble_command(
    tokens: &[TokenV0],
    index: usize,
    title_state: &mut OkTitleStateV0,
) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) if is_supported_meta_preamble_command(name.as_slice()) => {
            name.as_slice()
        }
        _ => return None,
    };
    let mut fragment = Vec::new();
    let mut fragment_previous_was_space = false;
    let next_index = consume_ok_group_fragment_v0(
        tokens,
        index + 1,
        tokens.len(),
        title_state,
        &mut fragment,
        &mut fragment_previous_was_space,
    )?;
    title_state.set_field(name, fragment);
    Some(skip_spaces(tokens, next_index))
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

fn consume_theorem_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    match name {
        b"theoremstyle" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"newtheorem" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            let is_star = matches!(tokens.get(cursor), Some(TokenV0::Char(b'*')));
            if is_star {
                cursor += 1;
                cursor = skip_spaces(tokens, cursor);
            }
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            let mut saw_prefix_bracket = false;
            if matches!(tokens.get(cursor), Some(TokenV0::Char(b'['))) {
                if is_star {
                    return None;
                }
                saw_prefix_bracket = true;
                cursor = consume_bracket_options_non_empty(tokens, cursor)?;
                cursor = skip_spaces(tokens, cursor);
            }
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            if matches!(tokens.get(cursor), Some(TokenV0::Char(b'['))) {
                if is_star || saw_prefix_bracket {
                    return None;
                }
                cursor = consume_bracket_options_non_empty(tokens, cursor)?;
            }
            Some(skip_spaces(tokens, cursor))
        }
        _ => None,
    }
}

fn consume_config_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    match name {
        b"hypersetup" | b"geometry" | b"captionsetup" | b"graphicspath" | b"setlist" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            if name == b"setlist" && matches!(tokens.get(cursor), Some(TokenV0::Char(b'['))) {
                cursor = consume_bracket_options_non_empty(tokens, cursor)?;
                cursor = skip_spaces(tokens, cursor);
            }
            cursor = consume_char_space_nested_group_non_empty_v0(tokens, cursor, tokens.len())?;
            Some(skip_spaces(tokens, cursor))
        }
        b"urlstyle" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        _ => None,
    }
}

fn consume_color_graphics_decl_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    match name {
        b"definecolor" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"colorlet" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"DeclareGraphicsExtensions" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        _ => None,
    }
}

fn consume_cite_ref_decl_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    match name {
        b"bibpunct" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            for _ in 0..6 {
                cursor = consume_char_space_group_non_empty(tokens, cursor)?;
                cursor = skip_spaces(tokens, cursor);
            }
            Some(cursor)
        }
        b"bibhang" | b"citestyle" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"setcitestyle" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_nested_group_non_empty_v0(tokens, cursor, tokens.len())?;
            Some(skip_spaces(tokens, cursor))
        }
        b"crefname" | b"Crefname" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            for _ in 0..3 {
                cursor = consume_char_space_group_non_empty(tokens, cursor)?;
                cursor = skip_spaces(tokens, cursor);
            }
            Some(cursor)
        }
        _ => None,
    }
}

fn consume_doc_hook_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    if !matches!(name, b"AtBeginDocument" | b"AtEndDocument") {
        return None;
    }
    let mut cursor = skip_spaces(tokens, index + 1);
    cursor = consume_char_space_nested_group_non_empty_v0(tokens, cursor, tokens.len())?;
    Some(skip_spaces(tokens, cursor))
}

fn consume_single_controlseq_group_v0(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let mut cursor = skip_spaces(tokens, index);
    if !matches!(tokens.get(cursor), Some(TokenV0::BeginGroup)) {
        return None;
    }
    cursor += 1;
    let mut saw_control_seq = false;
    loop {
        match tokens.get(cursor)? {
            TokenV0::EndGroup if saw_control_seq => return Some(cursor + 1),
            TokenV0::EndGroup => return None,
            TokenV0::Space => {
                cursor += 1;
            }
            TokenV0::ControlSeq(_) if !saw_control_seq => {
                saw_control_seq = true;
                cursor += 1;
            }
            _ => return None,
        }
    }
}

fn consume_math_operator_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"DeclareMathOperator"
    ) {
        return None;
    }
    let mut cursor = skip_spaces(tokens, index + 1);
    if matches!(tokens.get(cursor), Some(TokenV0::Char(b'*'))) {
        cursor += 1;
        cursor = skip_spaces(tokens, cursor);
    }
    cursor = consume_single_controlseq_group_v0(tokens, cursor)?;
    cursor = skip_spaces(tokens, cursor);
    cursor = consume_char_space_group_non_empty(tokens, cursor)?;
    Some(skip_spaces(tokens, cursor))
}

fn consume_math_alphabet_decl_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    let trailing_arity = match name {
        b"DeclareMathAlphabet" => 4usize,
        b"SetMathAlphabet" => 5usize,
        _ => return None,
    };
    let mut cursor = skip_spaces(tokens, index + 1);
    cursor = consume_single_controlseq_group_v0(tokens, cursor)?;
    cursor = skip_spaces(tokens, cursor);
    for _ in 0..trailing_arity {
        cursor = consume_char_space_group_non_empty(tokens, cursor)?;
        cursor = skip_spaces(tokens, cursor);
    }
    Some(cursor)
}

fn consume_math_symbol_decl_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    match name {
        b"DeclareSymbolFont" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            for _ in 0..5 {
                cursor = consume_char_space_group_non_empty(tokens, cursor)?;
                cursor = skip_spaces(tokens, cursor);
            }
            Some(cursor)
        }
        b"DeclareMathSymbol" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_single_controlseq_group_v0(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_single_controlseq_group_v0(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"DeclareMathDelimiter" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_single_controlseq_group_v0(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_single_controlseq_group_v0(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            for _ in 0..4 {
                cursor = consume_char_space_group_non_empty(tokens, cursor)?;
                cursor = skip_spaces(tokens, cursor);
            }
            Some(cursor)
        }
        _ => None,
    }
}

fn consume_symbol_font_setter_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    match name {
        b"SetSymbolFont" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            for _ in 0..6 {
                cursor = consume_char_space_group_non_empty(tokens, cursor)?;
                cursor = skip_spaces(tokens, cursor);
            }
            Some(cursor)
        }
        b"DeclareSymbolFontAlphabet" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_single_controlseq_group_v0(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
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
    let mut title_state = OkTitleStateV0::default();
    loop {
        match tokens.get(index) {
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"usepackage" => {
                index = consume_usepackage_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name)) if is_supported_meta_preamble_command(name.as_slice()) => {
                index = consume_meta_preamble_command(tokens, index, &mut title_state)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if is_supported_bibliography_preamble_command(name.as_slice()) =>
            {
                index = consume_bibliography_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if name.as_slice() == b"newtheorem" || name.as_slice() == b"theoremstyle" =>
            {
                index = consume_theorem_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"hypersetup"
                        | b"geometry"
                        | b"captionsetup"
                        | b"graphicspath"
                        | b"urlstyle"
                        | b"setlist"
                ) =>
            {
                index = consume_config_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"definecolor" | b"colorlet" | b"DeclareGraphicsExtensions"
                ) =>
            {
                index = consume_color_graphics_decl_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"bibpunct"
                        | b"bibhang"
                        | b"citestyle"
                        | b"setcitestyle"
                        | b"crefname"
                        | b"Crefname"
                ) =>
            {
                index = consume_cite_ref_decl_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(name.as_slice(), b"AtBeginDocument" | b"AtEndDocument") =>
            {
                index = consume_doc_hook_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if name.as_slice() == b"DeclareMathOperator" =>
            {
                index = consume_math_operator_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(name.as_slice(), b"DeclareMathAlphabet" | b"SetMathAlphabet") =>
            {
                index = consume_math_alphabet_decl_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"DeclareSymbolFont" | b"DeclareMathSymbol" | b"DeclareMathDelimiter"
                ) =>
            {
                index = consume_math_symbol_decl_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(name.as_slice(), b"SetSymbolFont" | b"DeclareSymbolFontAlphabet") =>
            {
                index = consume_symbol_font_setter_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(name.as_slice(), b"protect" | b"relax") =>
            {
                index += 1;
                index = skip_spaces(tokens, index);
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if is_supported_ok_style_declaration_v0(name.as_slice()) =>
            {
                index += 1;
                index = skip_spaces(tokens, index);
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if is_ok_noop_command_v0(name.as_slice()) =>
            {
                index = consume_ok_noop_command_v0(tokens, index, tokens.len(), name.as_slice())?;
                index = skip_spaces(tokens, index);
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
            &mut title_state,
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
