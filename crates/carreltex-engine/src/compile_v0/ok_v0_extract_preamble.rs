use crate::tex::tokenize_v0::TokenV0;

use super::super::ok_v0_body::{
    consume_balanced_group_bounds_v0,
    consume_bracket_options_non_empty, consume_char_space_group_non_empty,
    consume_char_space_nested_group_non_empty_v0, consume_ok_group_fragment_discard_v0,
    consume_ok_group_fragment_v0, is_supported_ok_style_declaration_v0, skip_spaces,
};
use super::super::ok_v0_title_state::OkTitleStateV0;

pub(super) fn consume_usepackage_preamble_command(tokens: &[TokenV0], mut index: usize) -> Option<usize> {
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

pub(super) fn consume_package_option_plumbing_preamble_command(
    tokens: &[TokenV0],
    index: usize,
) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    match name {
        b"RequirePackage" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            if matches!(tokens.get(cursor), Some(TokenV0::Char(b'['))) {
                cursor = consume_bracket_options_non_empty(tokens, cursor)?;
                cursor = skip_spaces(tokens, cursor);
            }
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"PassOptionsToPackage" | b"PassOptionsToClass" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"ExecuteOptions" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        _ => None,
    }
}

pub(super) fn consume_biblatex_resource_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    match name {
        b"addbibresource" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            if matches!(tokens.get(cursor), Some(TokenV0::Char(b'['))) {
                cursor = consume_bracket_options_non_empty(tokens, cursor)?;
                cursor = skip_spaces(tokens, cursor);
            }
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"ExecuteBibliographyOptions" | b"DeclareBibliographyCategory" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"addtocategory" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"DeclareLanguageMapping"
        | b"DeclareBibliographyAlias"
        | b"DeclareNameAlias"
        | b"DeclareListAlias"
        | b"DeclareFieldAlias" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        _ => None,
    }
}

pub(super) fn consume_label_aux_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    match name {
        b"label" | b"ref" | b"pageref" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"addtocontents" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_char_space_nested_group_non_empty_v0(tokens, cursor, tokens.len())?;
            Some(skip_spaces(tokens, cursor))
        }
        b"addcontentsline" => {
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

pub(super) fn consume_length_counter_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    if !matches!(name, b"setcounter" | b"addtolength" | b"setlength") {
        return None;
    }
    let mut cursor = skip_spaces(tokens, index + 1);
    cursor = consume_char_space_group_non_empty(tokens, cursor)?;
    cursor = skip_spaces(tokens, cursor);
    cursor = consume_char_space_group_non_empty(tokens, cursor)?;
    Some(skip_spaces(tokens, cursor))
}

fn consume_balanced_group_discard_non_empty_v0(tokens: &[TokenV0], index: usize, max_tokens: usize) -> Option<usize> {
    let (inner_start, inner_end, next_index) = consume_balanced_group_bounds_v0(
        tokens,
        skip_spaces(tokens, index),
        super::super::MAX_OK_GROUP_DEPTH_V0,
        tokens.len(),
    )?;
    if inner_end <= inner_start || inner_end - inner_start > max_tokens {
        return None;
    }
    let mut has_non_space = false;
    for token in &tokens[inner_start..inner_end] {
        if matches!(token, TokenV0::ControlSeq(name) if matches!(name.as_slice(), b"begin" | b"end")) {
            return None;
        }
        if !matches!(token, TokenV0::Space) {
            has_non_space = true;
        }
    }
    has_non_space.then_some(skip_spaces(tokens, next_index))
}

pub(super) fn consume_fancyhdr_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) { Some(TokenV0::ControlSeq(name)) => name.as_slice(), _ => return None };
    if matches!(name, b"fancyhf" | b"fancyhead" | b"fancyfoot") {
        let mut cursor = skip_spaces(tokens, index + 1);
        if matches!(tokens.get(cursor), Some(TokenV0::Char(b'['))) {
            cursor = consume_bracket_options_non_empty(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
        }
        let (_, _, next_index) =
            consume_balanced_group_bounds_v0(tokens, cursor, super::super::MAX_OK_GROUP_DEPTH_V0, tokens.len())?;
        return Some(skip_spaces(tokens, next_index));
    }
    if name == b"fancypagestyle" {
        let mut cursor = skip_spaces(tokens, index + 1);
        cursor = consume_char_space_group_non_empty(tokens, cursor)?;
        return consume_balanced_group_discard_non_empty_v0(tokens, cursor, 2048);
    }
    None
}

pub(super) fn consume_mark_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    match name {
        b"markright" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_nested_group_non_empty_v0(tokens, cursor, tokens.len())?;
            Some(skip_spaces(tokens, cursor))
        }
        b"markboth" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_nested_group_non_empty_v0(tokens, cursor, tokens.len())?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_char_space_nested_group_non_empty_v0(tokens, cursor, tokens.len())?;
            Some(skip_spaces(tokens, cursor))
        }
        _ => None,
    }
}

pub(super) fn consume_hyperref_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name))
            if matches!(
                name.as_slice(),
                b"pdfstringdefDisableCommands" | b"AtBeginShipout" | b"AtBeginShipoutNext"
            )
    ) {
        return None;
    }
    let mut cursor = skip_spaces(tokens, index + 1);
    cursor = consume_char_space_nested_group_non_empty_v0(tokens, cursor, tokens.len())?;
    Some(skip_spaces(tokens, cursor))
}

pub(super) fn consume_setuptoc_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"setuptoc"
    ) {
        return None;
    }
    let mut cursor = skip_spaces(tokens, index + 1);
    cursor = consume_char_space_nested_group_non_empty_v0(tokens, cursor, tokens.len())?;
    Some(skip_spaces(tokens, cursor))
}

fn consume_koma_style_decls_group_v0(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let mut cursor = skip_spaces(tokens, index);
    if !matches!(tokens.get(cursor), Some(TokenV0::BeginGroup)) {
        return None;
    }
    cursor += 1;
    let mut has_non_space = false;
    loop {
        match tokens.get(cursor)? {
            TokenV0::EndGroup if has_non_space => return Some(cursor + 1),
            TokenV0::EndGroup => return None,
            TokenV0::Space => {
                cursor += 1;
            }
            TokenV0::Char(byte) if (0x20..=0x7e).contains(byte) => {
                has_non_space = true;
                cursor += 1;
            }
            TokenV0::ControlSeq(name) if is_supported_ok_style_declaration_v0(name.as_slice()) => {
                has_non_space = true;
                cursor += 1;
            }
            _ => return None,
        }
    }
}

pub(super) fn consume_koma_config_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    match name {
        b"KOMAoptions" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"KOMAoption" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"setkomafont" | b"addtokomafont" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_koma_style_decls_group_v0(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        _ => None,
    }
}

fn consume_single_named_controlseq_group_v0(
    tokens: &[TokenV0],
    index: usize,
    allowed_names: &[&[u8]],
) -> Option<usize> {
    let mut cursor = skip_spaces(tokens, index);
    if !matches!(tokens.get(cursor), Some(TokenV0::BeginGroup)) {
        return None;
    }
    cursor += 1;
    while matches!(tokens.get(cursor), Some(TokenV0::Space)) {
        cursor += 1;
    }
    let name = match tokens.get(cursor) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    if !allowed_names.iter().any(|allowed| *allowed == name) {
        return None;
    }
    cursor += 1;
    while matches!(tokens.get(cursor), Some(TokenV0::Space)) {
        cursor += 1;
    }
    if !matches!(tokens.get(cursor), Some(TokenV0::EndGroup)) {
        return None;
    }
    Some(cursor + 1)
}

pub(super) fn consume_float_listof_preamble_command(
    tokens: &[TokenV0],
    index: usize,
    title_state: &mut OkTitleStateV0,
) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    match name {
        b"floatplacement" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"renewcommand" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_single_named_controlseq_group_v0(
                tokens,
                cursor,
                &[b"listfigurename", b"listtablename"],
            )?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_ok_group_fragment_discard_v0(tokens, cursor, tokens.len(), title_state)?;
            Some(skip_spaces(tokens, cursor))
        }
        _ => None,
    }
}

pub(super) fn consume_language_decl_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    match name {
        b"selectlanguage" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"setmainlanguage" | b"setdefaultlanguage" | b"setotherlanguage" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            if matches!(tokens.get(cursor), Some(TokenV0::Char(b'['))) {
                cursor = consume_bracket_options_non_empty(tokens, cursor)?;
                cursor = skip_spaces(tokens, cursor);
            }
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        _ => None,
    }
}

pub(super) fn is_supported_meta_preamble_command(name: &[u8]) -> bool {
    matches!(
        name,
        b"title"
            | b"author"
            | b"date"
            | b"subtitle"
            | b"institute"
            | b"affiliation"
            | b"address"
            | b"email"
            | b"homepage"
            | b"keywords"
            | b"subject"
            | b"titlehead"
            | b"authorrunning"
            | b"titlerunning"
            | b"publishers"
            | b"dedication"
            | b"extratitle"
            | b"extrainfo"
            | b"uppertitleback"
            | b"lowertitleback"
    )
}

pub(super) fn is_supported_bibliography_preamble_command(name: &[u8]) -> bool {
    matches!(name, b"bibliographystyle" | b"bibliography")
}

pub(super) fn consume_meta_preamble_command(
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
    if matches!(name, b"title" | b"author" | b"date") {
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
        return Some(skip_spaces(tokens, next_index));
    }
    let next_index =
        consume_ok_group_fragment_discard_v0(tokens, index + 1, tokens.len(), title_state)?;
    Some(skip_spaces(tokens, next_index))
}

pub(super) fn consume_bibliography_preamble_command(tokens: &[TokenV0], mut index: usize) -> Option<usize> {
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

pub(super) fn consume_theorem_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
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

pub(super) fn consume_config_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
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

pub(super) fn consume_color_graphics_decl_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
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

pub(super) fn consume_cite_ref_decl_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
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

pub(super) fn consume_doc_hook_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
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

pub(super) fn consume_mathcode_delcode_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if matches!(name.as_slice(), b"mathcode" | b"delcode")
    ) {
        return None;
    }
    let mut cursor = skip_spaces(tokens, index + 1);
    if !matches!(tokens.get(cursor), Some(TokenV0::Char(_))) {
        return None;
    }
    cursor += 1;
    cursor = skip_spaces(tokens, cursor);
    if !matches!(tokens.get(cursor), Some(TokenV0::Char(b'='))) {
        return None;
    }
    cursor += 1;
    cursor = skip_spaces(tokens, cursor);
    let mut saw_digit = false;
    while matches!(tokens.get(cursor), Some(TokenV0::Char(b'0'..=b'9'))) {
        saw_digit = true;
        cursor += 1;
    }
    if !saw_digit {
        return None;
    }
    Some(skip_spaces(tokens, cursor))
}

pub(super) fn consume_math_version_sizes_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    match name {
        b"DeclareMathVersion" | b"mathversion" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"DeclareMathSizes" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            for _ in 0..4 {
                cursor = consume_char_space_group_non_empty(tokens, cursor)?;
                cursor = skip_spaces(tokens, cursor);
            }
            Some(cursor)
        }
        _ => None,
    }
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

pub(super) fn consume_math_operator_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
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

pub(super) fn consume_math_alphabet_decl_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
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

pub(super) fn consume_math_symbol_decl_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
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

pub(super) fn consume_symbol_font_setter_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
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

pub(super) fn consume_math_accent_radical_decl_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    match name {
        b"DeclareMathAccent" => {
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
        b"DeclareMathRadical" => {
            let mut cursor = skip_spaces(tokens, index + 1);
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

pub(super) fn consume_font_decl_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    let arity = match name {
        b"DeclareFontEncoding" => 3usize,
        b"DeclareFontSubstitution" => 4usize,
        b"DeclareFontFamily" => 3usize,
        b"DeclareFontShape" => 6usize,
        b"DeclareFontEncodingDefaults" => 2usize,
        b"DeclareFontSeriesDefault" => 3usize,
        b"DeclareFontShapeDefault" => 3usize,
        b"DeclareFontFamilyDefault" => 2usize,
        _ => return None,
    };
    let mut cursor = skip_spaces(tokens, index + 1);
    for _ in 0..arity {
        cursor = consume_char_space_group_non_empty(tokens, cursor)?;
        cursor = skip_spaces(tokens, cursor);
    }
    Some(cursor)
}

fn consume_optional_robust_command_one_arity_v0(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let mut cursor = skip_spaces(tokens, index);
    if !matches!(tokens.get(cursor), Some(TokenV0::Char(b'['))) {
        return Some(cursor);
    }
    cursor += 1;
    cursor = skip_spaces(tokens, cursor);
    if !matches!(tokens.get(cursor), Some(TokenV0::Char(b'1'))) {
        return None;
    }
    cursor += 1;
    cursor = skip_spaces(tokens, cursor);
    if !matches!(tokens.get(cursor), Some(TokenV0::Char(b']'))) {
        return None;
    }
    Some(cursor + 1)
}

pub(super) fn consume_declare_robust_command_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"DeclareRobustCommand"
    ) {
        return None;
    }
    let mut cursor = skip_spaces(tokens, index + 1);
    if matches!(tokens.get(cursor), Some(TokenV0::Char(b'*'))) {
        return None;
    }
    cursor = consume_single_controlseq_group_v0(tokens, cursor)?;
    cursor = skip_spaces(tokens, cursor);
    cursor = consume_optional_robust_command_one_arity_v0(tokens, cursor)?;
    cursor = skip_spaces(tokens, cursor);
    cursor = consume_char_space_nested_group_non_empty_v0(tokens, cursor, tokens.len())?;
    Some(skip_spaces(tokens, cursor))
}

pub(super) fn consume_declare_text_font_command_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"DeclareTextFontCommand"
    ) {
        return None;
    }
    let mut cursor = skip_spaces(tokens, index + 1);
    cursor = consume_single_controlseq_group_v0(tokens, cursor)?;
    cursor = skip_spaces(tokens, cursor);
    cursor = consume_char_space_nested_group_non_empty_v0(tokens, cursor, tokens.len())?;
    Some(skip_spaces(tokens, cursor))
}

pub(super) fn consume_declare_text_command_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name))
            if matches!(name.as_slice(), b"DeclareTextCommand" | b"ProvideTextCommand")
    ) {
        return None;
    }
    let mut cursor = skip_spaces(tokens, index + 1);
    cursor = consume_single_controlseq_group_v0(tokens, cursor)?;
    cursor = skip_spaces(tokens, cursor);
    cursor = consume_char_space_group_non_empty(tokens, cursor)?;
    cursor = skip_spaces(tokens, cursor);
    cursor = consume_char_space_nested_group_non_empty_v0(tokens, cursor, tokens.len())?;
    Some(skip_spaces(tokens, cursor))
}

pub(super) fn consume_text_command_default_preamble_command(
    tokens: &[TokenV0],
    index: usize,
) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    if !matches!(
        name,
        b"ProvideTextCommandDefault" | b"DeclareTextCommandDefault" | b"DeclareTextCompositeDefault"
    ) {
        return None;
    }
    let mut cursor = skip_spaces(tokens, index + 1);
    cursor = consume_single_controlseq_group_v0(tokens, cursor)?;
    cursor = skip_spaces(tokens, cursor);
    if name == b"DeclareTextCompositeDefault" {
        if let Some(enc_end) = consume_char_space_group_non_empty(tokens, cursor) {
            let after_enc = skip_spaces(tokens, enc_end);
            if matches!(tokens.get(after_enc), Some(TokenV0::BeginGroup)) {
                cursor =
                    consume_char_space_nested_group_non_empty_v0(tokens, after_enc, tokens.len())?;
                return Some(skip_spaces(tokens, cursor));
            }
        }
    }
    cursor = consume_char_space_nested_group_non_empty_v0(tokens, cursor, tokens.len())?;
    Some(skip_spaces(tokens, cursor))
}

pub(super) fn consume_text_decl_bundle_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    match name {
        b"DeclareTextSymbol" | b"DeclareTextAccent" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_single_controlseq_group_v0(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"DeclareTextAccentDefault" | b"DeclareTextSymbolDefault" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_single_controlseq_group_v0(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_char_space_nested_group_non_empty_v0(tokens, cursor, tokens.len())?;
            Some(skip_spaces(tokens, cursor))
        }
        _ => None,
    }
}

pub(super) fn consume_declare_text_composite_command_preamble_command(
    tokens: &[TokenV0],
    index: usize,
) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"DeclareTextCompositeCommand"
    ) {
        return None;
    }
    let mut cursor = skip_spaces(tokens, index + 1);
    cursor = consume_single_controlseq_group_v0(tokens, cursor)?;
    cursor = skip_spaces(tokens, cursor);
    cursor = consume_char_space_group_non_empty(tokens, cursor)?;
    cursor = skip_spaces(tokens, cursor);
    cursor = consume_char_space_group_non_empty(tokens, cursor)?;
    cursor = skip_spaces(tokens, cursor);
    cursor = consume_optional_robust_command_one_arity_v0(tokens, cursor)?;
    cursor = skip_spaces(tokens, cursor);
    cursor = consume_char_space_nested_group_non_empty_v0(tokens, cursor, tokens.len())?;
    Some(skip_spaces(tokens, cursor))
}

pub(super) fn consume_declare_text_composite_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"DeclareTextComposite"
    ) {
        return None;
    }
    let mut cursor = skip_spaces(tokens, index + 1);
    cursor = consume_single_controlseq_group_v0(tokens, cursor)?;
    cursor = skip_spaces(tokens, cursor);
    cursor = consume_char_space_group_non_empty(tokens, cursor)?;
    cursor = skip_spaces(tokens, cursor);
    cursor = consume_char_space_nested_group_non_empty_v0(tokens, cursor, tokens.len())?;
    Some(skip_spaces(tokens, cursor))
}
