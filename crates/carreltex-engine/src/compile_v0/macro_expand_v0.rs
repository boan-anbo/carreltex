use std::collections::BTreeMap;

use crate::reasons_v0::InvalidInputReasonV0;
use crate::tex::tokenize_v0::{TokenV0, MAX_TOKENS_V0};

pub(crate) const MAX_MACROS_V0: usize = 4096;
pub(crate) const MAX_MACRO_EXPANSIONS_V0: usize = 4096;
pub(crate) const MAX_MACRO_DEPTH_V0: usize = 64;
const MAX_COUNT_VALUE_V0: u32 = 1_000_000;

#[derive(Clone)]
struct MacroDefV0 {
    param_count: u8,
    body_tokens: Vec<TokenV0>,
}

#[derive(Clone)]
enum MacroBindingV0 {
    Macro(MacroDefV0),
    ControlSeqLiteral(Vec<u8>),
    LetAlias {
        target_name: Vec<u8>,
        resolved_binding: Box<MacroBindingV0>,
    },
}

pub(crate) fn expand_macros_v0(tokens: &[TokenV0]) -> Result<Vec<TokenV0>, InvalidInputReasonV0> {
    let mut macro_frames = Vec::<BTreeMap<Vec<u8>, MacroBindingV0>>::new();
    macro_frames.push(BTreeMap::new());
    let mut counters = [0u32; 2];
    let mut output = Vec::<TokenV0>::new();
    let mut active_macros = Vec::<Vec<u8>>::new();
    let mut expansion_count = 0usize;
    expand_stream_v0(
        tokens,
        &mut macro_frames,
        &mut counters,
        &mut output,
        &mut active_macros,
        &mut expansion_count,
        0,
    )?;
    Ok(output)
}

fn expand_stream_v0(
    tokens: &[TokenV0],
    macro_frames: &mut Vec<BTreeMap<Vec<u8>, MacroBindingV0>>,
    counters: &mut [u32; 2],
    out: &mut Vec<TokenV0>,
    active_macros: &mut Vec<Vec<u8>>,
    expansion_count: &mut usize,
    depth: usize,
) -> Result<(), InvalidInputReasonV0> {
    if depth > MAX_MACRO_DEPTH_V0 {
        return Err(InvalidInputReasonV0::MacroDepthExceeded);
    }

    let mut index = 0usize;
    while index < tokens.len() {
        match &tokens[index] {
            TokenV0::BeginGroup => {
                macro_frames.push(BTreeMap::new());
                push_checked_v0(out, TokenV0::BeginGroup)?;
                index += 1;
            }
            TokenV0::EndGroup => {
                if macro_frames.len() > 1 {
                    macro_frames.pop();
                }
                push_checked_v0(out, TokenV0::EndGroup)?;
                index += 1;
            }
            TokenV0::ControlSeq(name)
                if name.as_slice() == b"def"
                    || name.as_slice() == b"gdef"
                    || name.as_slice() == b"edef" =>
            {
                let is_global = name.as_slice() == b"gdef";
                let expand_body = name.as_slice() == b"edef";
                index = parse_def_v0(tokens, index, macro_frames, counters, is_global, expand_body)?;
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"xdef" => {
                index = parse_xdef_v0(tokens, index, macro_frames, counters, true)?;
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"let" => {
                index = parse_let_v0(tokens, index, macro_frames, false)?;
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"futurelet" => {
                index = parse_futurelet_v0(tokens, index, macro_frames, false)?;
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"expandafter" => {
                let (reordered_tokens, next_index) = parse_expandafter_v0(tokens, index)?;
                expand_stream_v0(
                    &reordered_tokens,
                    macro_frames,
                    counters,
                    out,
                    active_macros,
                    expansion_count,
                    depth + 1,
                )?;
                index = next_index;
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"csname" => {
                let (generated_token, next_index) = parse_csname_v0(tokens, index)?;
                expand_stream_v0(
                    &[generated_token],
                    macro_frames,
                    counters,
                    out,
                    active_macros,
                    expansion_count,
                    depth + 1,
                )?;
                index = next_index;
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"string" => {
                let (string_chars, next_index) = parse_string_v0(tokens, index)?;
                for token in string_chars {
                    push_checked_v0(out, token)?;
                }
                index = next_index;
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"meaning" => {
                let (meaning_chars, next_index) = parse_meaning_v0(tokens, index, macro_frames)?;
                for token in meaning_chars {
                    push_checked_v0(out, token)?;
                }
                index = next_index;
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"count" => {
                index = parse_count_assignment_v0(tokens, index, counters)?;
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"the" => {
                let (the_chars, next_index) = parse_the_v0(tokens, index, counters)?;
                for token in the_chars {
                    push_checked_v0(out, token)?;
                }
                index = next_index;
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"global" => {
                index = parse_global_prefixed_macro_binding_v0(tokens, index, macro_frames, counters)?;
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"noexpand" => {
                index = parse_noexpand_v0(tokens, index, out)?;
            }
            TokenV0::ControlSeq(name) => {
                match lookup_macro_binding_v0(macro_frames, name) {
                    Some(MacroBindingV0::Macro(macro_def)) => {
                        *expansion_count = expansion_count
                            .checked_add(1)
                            .ok_or(InvalidInputReasonV0::MacroExpansionsExceeded)?;
                        if *expansion_count > MAX_MACRO_EXPANSIONS_V0 {
                            return Err(InvalidInputReasonV0::MacroExpansionsExceeded);
                        }
                        if active_macros.iter().any(|active| active == name) {
                            return Err(InvalidInputReasonV0::MacroCycleFailed);
                        }
                        let (expanded_body, next_index) = match macro_def.param_count {
                            0 => (macro_def.body_tokens, index + 1),
                            1 => {
                                let (argument_tokens, argument_next_index) =
                                    parse_balanced_group_payload_v0(tokens, index + 1)?;
                                let substituted_body = substitute_single_param_placeholders_v0(
                                    &macro_def.body_tokens,
                                    &argument_tokens,
                                )?;
                                (substituted_body, argument_next_index)
                            }
                            _ => return Err(InvalidInputReasonV0::MacroValidationFailed),
                        };
                        active_macros.push(name.clone());
                        let result = expand_stream_v0(
                            &expanded_body,
                            macro_frames,
                            counters,
                            out,
                            active_macros,
                            expansion_count,
                            depth + 1,
                        );
                        active_macros.pop();
                        result?;
                        index = next_index;
                    }
                    Some(MacroBindingV0::ControlSeqLiteral(target_name)) => {
                        push_checked_v0(out, TokenV0::ControlSeq(target_name))?;
                        index += 1;
                    }
                    Some(MacroBindingV0::LetAlias {
                        target_name: _,
                        resolved_binding,
                    }) => {
                        expand_binding_v0(
                            name,
                            *resolved_binding,
                            macro_frames,
                            counters,
                            out,
                            active_macros,
                            expansion_count,
                            depth,
                        )?;
                        index += 1;
                    }
                    None => {
                        push_checked_v0(out, tokens[index].clone())?;
                        index += 1;
                    }
                }
            }
            token => {
                push_checked_v0(out, token.clone())?;
                index += 1;
            }
        }
    }
    Ok(())
}

fn parse_global_prefixed_macro_binding_v0(
    tokens: &[TokenV0],
    global_index: usize,
    macro_frames: &mut Vec<BTreeMap<Vec<u8>, MacroBindingV0>>,
    counters: &mut [u32; 2],
) -> Result<usize, InvalidInputReasonV0> {
    let mut index = global_index;
    while matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"global"
    ) {
        index += 1;
    }

    match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"def" => {
            parse_def_v0(tokens, index, macro_frames, counters, true, false)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"gdef" => {
            parse_def_v0(tokens, index, macro_frames, counters, true, false)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"edef" => {
            parse_def_v0(tokens, index, macro_frames, counters, true, true)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"xdef" => {
            parse_xdef_v0(tokens, index, macro_frames, counters, true)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"let" => {
            parse_let_v0(tokens, index, macro_frames, true)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"futurelet" => {
            parse_futurelet_v0(tokens, index, macro_frames, true)
        }
        _ => return Err(InvalidInputReasonV0::MacroGlobalPrefixUnsupported),
    }
}

fn parse_def_v0(
    tokens: &[TokenV0],
    def_index: usize,
    macro_frames: &mut Vec<BTreeMap<Vec<u8>, MacroBindingV0>>,
    counters: &mut [u32; 2],
    is_global: bool,
    expand_body: bool,
) -> Result<usize, InvalidInputReasonV0> {
    let name_index = def_index + 1;
    let macro_name = match tokens.get(name_index) {
        Some(TokenV0::ControlSeq(name)) => name.clone(),
        _ => return Err(InvalidInputReasonV0::MacroValidationFailed),
    };

    let mut param_count = 0u8;
    let mut body_start_index = name_index + 1;
    match tokens.get(body_start_index) {
        Some(TokenV0::BeginGroup) => {}
        Some(TokenV0::Char(b'#')) => {
            if expand_body {
                return Err(InvalidInputReasonV0::MacroParamsUnsupported);
            }
            let placeholder_digit = match tokens.get(body_start_index + 1) {
                Some(TokenV0::Char(digit)) => *digit,
                _ => return Err(InvalidInputReasonV0::MacroParamsUnsupported),
            };
            if placeholder_digit != b'1' {
                return Err(InvalidInputReasonV0::MacroParamsUnsupported);
            }
            param_count = 1;
            body_start_index += 2;
            if matches!(tokens.get(body_start_index), Some(TokenV0::Char(b'#'))) {
                return Err(InvalidInputReasonV0::MacroParamsUnsupported);
            }
            if !matches!(tokens.get(body_start_index), Some(TokenV0::BeginGroup)) {
                return Err(InvalidInputReasonV0::MacroValidationFailed);
            }
        }
        _ => return Err(InvalidInputReasonV0::MacroValidationFailed),
    }

    let (body_tokens, next_index) = parse_balanced_group_payload_v0(tokens, body_start_index)?;
    validate_macro_body_tokens_v0(&body_tokens, param_count)?;
    let final_body_tokens = if expand_body {
        if param_count != 0 {
            return Err(InvalidInputReasonV0::MacroParamsUnsupported);
        }
        let mut expanded = Vec::<TokenV0>::new();
        let mut active_macros = Vec::<Vec<u8>>::new();
        let mut expansion_count = 0usize;
        expand_stream_v0(
            &body_tokens,
            macro_frames,
            counters,
            &mut expanded,
            &mut active_macros,
            &mut expansion_count,
            0,
        )?;
        expanded
    } else {
        body_tokens
    };
    let target_frame_index = if is_global {
        0usize
    } else {
        macro_frames
            .len()
            .checked_sub(1)
            .ok_or(InvalidInputReasonV0::MacroValidationFailed)?
    };
    let total_macro_defs = total_macro_defs_v0(macro_frames);
    let target_frame = macro_frames
        .get_mut(target_frame_index)
        .ok_or(InvalidInputReasonV0::MacroValidationFailed)?;
    if !target_frame.contains_key(&macro_name) && total_macro_defs >= MAX_MACROS_V0 {
        return Err(InvalidInputReasonV0::MacroValidationFailed);
    }
    if param_count > 1 {
        return Err(InvalidInputReasonV0::MacroValidationFailed);
    }
    target_frame.insert(
        macro_name,
        MacroBindingV0::Macro(MacroDefV0 {
            param_count: if expand_body { 0 } else { param_count },
            body_tokens: final_body_tokens,
        }),
    );
    Ok(next_index)
}

fn parse_xdef_v0(
    tokens: &[TokenV0],
    xdef_index: usize,
    macro_frames: &mut Vec<BTreeMap<Vec<u8>, MacroBindingV0>>,
    counters: &mut [u32; 2],
    is_global: bool,
) -> Result<usize, InvalidInputReasonV0> {
    let name_index = xdef_index + 1;
    let macro_name = match tokens.get(name_index) {
        Some(TokenV0::ControlSeq(name)) => name.clone(),
        _ => return Err(InvalidInputReasonV0::MacroXdefUnsupported),
    };
    let body_start_index = name_index + 1;
    if !matches!(tokens.get(body_start_index), Some(TokenV0::BeginGroup)) {
        return Err(InvalidInputReasonV0::MacroXdefUnsupported);
    }
    let (body_tokens, next_index) = parse_balanced_group_payload_v0(tokens, body_start_index)
        .map_err(|_| InvalidInputReasonV0::MacroXdefUnsupported)?;
    if body_tokens.iter().any(|token| matches!(token, TokenV0::Char(b'#'))) {
        return Err(InvalidInputReasonV0::MacroXdefUnsupported);
    }

    let mut expanded = Vec::<TokenV0>::new();
    let mut active_macros = Vec::<Vec<u8>>::new();
    let mut expansion_count = 0usize;
    expand_stream_v0(
        &body_tokens,
        macro_frames,
        counters,
        &mut expanded,
        &mut active_macros,
        &mut expansion_count,
        0,
    )?;

    let target_frame_index = if is_global {
        0usize
    } else {
        macro_frames
            .len()
            .checked_sub(1)
            .ok_or(InvalidInputReasonV0::MacroValidationFailed)?
    };
    let total_macro_defs = total_macro_defs_v0(macro_frames);
    let target_frame = macro_frames
        .get_mut(target_frame_index)
        .ok_or(InvalidInputReasonV0::MacroValidationFailed)?;
    if !target_frame.contains_key(&macro_name) && total_macro_defs >= MAX_MACROS_V0 {
        return Err(InvalidInputReasonV0::MacroValidationFailed);
    }
    target_frame.insert(
        macro_name,
        MacroBindingV0::Macro(MacroDefV0 {
            param_count: 0,
            body_tokens: expanded,
        }),
    );
    Ok(next_index)
}

fn parse_noexpand_v0(
    tokens: &[TokenV0],
    noexpand_index: usize,
    out: &mut Vec<TokenV0>,
) -> Result<usize, InvalidInputReasonV0> {
    let next = tokens
        .get(noexpand_index + 1)
        .ok_or(InvalidInputReasonV0::MacroNoexpandUnsupported)?
        .clone();
    push_checked_v0(out, next)?;
    Ok(noexpand_index + 2)
}

fn parse_let_v0(
    tokens: &[TokenV0],
    let_index: usize,
    macro_frames: &mut Vec<BTreeMap<Vec<u8>, MacroBindingV0>>,
    is_global: bool,
) -> Result<usize, InvalidInputReasonV0> {
    let alias_name = match tokens.get(let_index + 1) {
        Some(TokenV0::ControlSeq(name)) => name.clone(),
        _ => return Err(InvalidInputReasonV0::MacroValidationFailed),
    };

    let mut index = skip_space_tokens_v0(tokens, let_index + 2);
    if matches!(tokens.get(index), Some(TokenV0::Char(b'='))) {
        index = skip_space_tokens_v0(tokens, index + 1);
    }

    let target_name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.clone(),
        _ => return Err(InvalidInputReasonV0::MacroLetUnsupported),
    };
    let resolved_binding = snapshot_let_binding_v0(&target_name, macro_frames)?;
    let target_frame_index = if is_global {
        0usize
    } else {
        macro_frames
            .len()
            .checked_sub(1)
            .ok_or(InvalidInputReasonV0::MacroValidationFailed)?
    };
    let total_macro_defs = total_macro_defs_v0(macro_frames);
    let target_frame = macro_frames
        .get_mut(target_frame_index)
        .ok_or(InvalidInputReasonV0::MacroValidationFailed)?;
    if !target_frame.contains_key(&alias_name) && total_macro_defs >= MAX_MACROS_V0 {
        return Err(InvalidInputReasonV0::MacroValidationFailed);
    }
    target_frame.insert(
        alias_name,
        MacroBindingV0::LetAlias {
            target_name,
            resolved_binding: Box::new(resolved_binding),
        },
    );
    Ok(index + 1)
}

fn parse_futurelet_v0(
    tokens: &[TokenV0],
    futurelet_index: usize,
    macro_frames: &mut Vec<BTreeMap<Vec<u8>, MacroBindingV0>>,
    is_global: bool,
) -> Result<usize, InvalidInputReasonV0> {
    let alias_name_index = skip_space_tokens_v0(tokens, futurelet_index + 1);
    let alias_name = match tokens.get(alias_name_index) {
        Some(TokenV0::ControlSeq(name)) => name.clone(),
        _ => return Err(InvalidInputReasonV0::MacroFutureletUnsupported),
    };

    let probe_index = skip_space_tokens_v0(tokens, alias_name_index + 1);
    let _probe_name = match tokens.get(probe_index) {
        Some(TokenV0::ControlSeq(name)) => name.clone(),
        _ => return Err(InvalidInputReasonV0::MacroFutureletUnsupported),
    };

    let target_index = skip_space_tokens_v0(tokens, probe_index + 1);
    let target_name = match tokens.get(target_index) {
        Some(TokenV0::ControlSeq(name)) => name.clone(),
        _ => return Err(InvalidInputReasonV0::MacroFutureletUnsupported),
    };

    let target_frame_index = if is_global {
        0usize
    } else {
        macro_frames
            .len()
            .checked_sub(1)
            .ok_or(InvalidInputReasonV0::MacroValidationFailed)?
    };
    let total_macro_defs = total_macro_defs_v0(macro_frames);
    let target_frame = macro_frames
        .get_mut(target_frame_index)
        .ok_or(InvalidInputReasonV0::MacroValidationFailed)?;
    if !target_frame.contains_key(&alias_name) && total_macro_defs >= MAX_MACROS_V0 {
        return Err(InvalidInputReasonV0::MacroValidationFailed);
    }
    target_frame.insert(alias_name, MacroBindingV0::ControlSeqLiteral(target_name));
    Ok(probe_index)
}

fn parse_expandafter_v0(
    tokens: &[TokenV0],
    expandafter_index: usize,
) -> Result<(Vec<TokenV0>, usize), InvalidInputReasonV0> {
    let first_index = skip_space_tokens_v0(tokens, expandafter_index + 1);
    let first_name = match tokens.get(first_index) {
        Some(TokenV0::ControlSeq(name)) => name.clone(),
        _ => return Err(InvalidInputReasonV0::MacroExpandafterUnsupported),
    };

    let second_index = skip_space_tokens_v0(tokens, first_index + 1);
    let second_name = match tokens.get(second_index) {
        Some(TokenV0::ControlSeq(name)) => name.clone(),
        _ => return Err(InvalidInputReasonV0::MacroExpandafterUnsupported),
    };

    Ok((
        vec![TokenV0::ControlSeq(second_name), TokenV0::ControlSeq(first_name)],
        second_index + 1,
    ))
}

fn parse_csname_v0(
    tokens: &[TokenV0],
    csname_index: usize,
) -> Result<(TokenV0, usize), InvalidInputReasonV0> {
    let mut name_bytes = Vec::<u8>::new();
    let mut index = csname_index + 1;
    while index < tokens.len() {
        match tokens.get(index) {
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"endcsname" => {
                if name_bytes.is_empty() {
                    return Err(InvalidInputReasonV0::MacroCsnameUnsupported);
                }
                return Ok((TokenV0::ControlSeq(name_bytes), index + 1));
            }
            Some(TokenV0::Char(byte)) => name_bytes.push(*byte),
            _ => return Err(InvalidInputReasonV0::MacroCsnameUnsupported),
        }
        index += 1;
    }
    Err(InvalidInputReasonV0::MacroCsnameUnsupported)
}

fn parse_string_v0(
    tokens: &[TokenV0],
    string_index: usize,
) -> Result<(Vec<TokenV0>, usize), InvalidInputReasonV0> {
    let next_index = skip_space_tokens_v0(tokens, string_index + 1);
    let control_name = match tokens.get(next_index) {
        Some(TokenV0::ControlSeq(name)) => name.clone(),
        _ => return Err(InvalidInputReasonV0::MacroStringUnsupported),
    };

    let mut out = Vec::<TokenV0>::new();
    out.push(TokenV0::Char(b'\\'));
    for byte in control_name {
        out.push(TokenV0::Char(byte));
    }
    Ok((out, next_index + 1))
}

fn parse_meaning_v0(
    tokens: &[TokenV0],
    meaning_index: usize,
    macro_frames: &[BTreeMap<Vec<u8>, MacroBindingV0>],
) -> Result<(Vec<TokenV0>, usize), InvalidInputReasonV0> {
    let next_index = skip_space_tokens_v0(tokens, meaning_index + 1);
    let query_name = match tokens.get(next_index) {
        Some(TokenV0::ControlSeq(name)) => name.clone(),
        _ => return Err(InvalidInputReasonV0::MacroMeaningUnsupported),
    };

    let mut out = Vec::<TokenV0>::new();
    match lookup_macro_binding_v0(macro_frames, &query_name) {
        Some(MacroBindingV0::Macro(_)) => {
            push_ascii_bytes_v0(&mut out, b"macro:")?;
            push_ascii_bytes_v0(&mut out, &query_name)?;
        }
        Some(MacroBindingV0::ControlSeqLiteral(target_name)) => {
            push_ascii_bytes_v0(&mut out, b"alias:")?;
            push_ascii_bytes_v0(&mut out, &query_name)?;
            push_ascii_bytes_v0(&mut out, b"->")?;
            push_ascii_bytes_v0(&mut out, &target_name)?;
        }
        Some(MacroBindingV0::LetAlias {
            target_name,
            resolved_binding: _,
        }) => {
            push_ascii_bytes_v0(&mut out, b"alias:")?;
            push_ascii_bytes_v0(&mut out, &query_name)?;
            push_ascii_bytes_v0(&mut out, b"->")?;
            push_ascii_bytes_v0(&mut out, &target_name)?;
        }
        None => {
            push_ascii_bytes_v0(&mut out, b"undefined:")?;
            push_ascii_bytes_v0(&mut out, &query_name)?;
        }
    }
    Ok((out, next_index + 1))
}

fn lookup_macro_binding_v0(
    macro_frames: &[BTreeMap<Vec<u8>, MacroBindingV0>],
    name: &[u8],
) -> Option<MacroBindingV0> {
    for frame in macro_frames.iter().rev() {
        if let Some(binding) = frame.get(name) {
            return Some(binding.clone());
        }
    }
    None
}

fn snapshot_let_binding_v0(
    target_name: &[u8],
    macro_frames: &[BTreeMap<Vec<u8>, MacroBindingV0>],
) -> Result<MacroBindingV0, InvalidInputReasonV0> {
    let mut current = target_name.to_vec();
    let mut seen = Vec::<Vec<u8>>::new();
    loop {
        if seen.iter().any(|entry| entry == &current) {
            return Err(InvalidInputReasonV0::MacroCycleFailed);
        }
        let binding = lookup_macro_binding_v0(macro_frames, &current);
        match binding {
            Some(MacroBindingV0::Macro(definition)) => return Ok(MacroBindingV0::Macro(definition)),
            Some(MacroBindingV0::ControlSeqLiteral(target)) => {
                seen.push(current);
                current = target;
            }
            Some(MacroBindingV0::LetAlias {
                target_name: _,
                resolved_binding,
            }) => return Ok(*resolved_binding),
            None => return Ok(MacroBindingV0::ControlSeqLiteral(current)),
        }
    }
}

fn expand_binding_v0(
    name: &[u8],
    binding: MacroBindingV0,
    macro_frames: &mut Vec<BTreeMap<Vec<u8>, MacroBindingV0>>,
    counters: &mut [u32; 2],
    out: &mut Vec<TokenV0>,
    active_macros: &mut Vec<Vec<u8>>,
    expansion_count: &mut usize,
    depth: usize,
) -> Result<(), InvalidInputReasonV0> {
    *expansion_count = expansion_count
        .checked_add(1)
        .ok_or(InvalidInputReasonV0::MacroExpansionsExceeded)?;
    if *expansion_count > MAX_MACRO_EXPANSIONS_V0 {
        return Err(InvalidInputReasonV0::MacroExpansionsExceeded);
    }
    if active_macros.iter().any(|active| active == name) {
        return Err(InvalidInputReasonV0::MacroCycleFailed);
    }
    active_macros.push(name.to_vec());
    let result = match binding {
        MacroBindingV0::Macro(macro_def) => {
            if macro_def.param_count != 0 {
                return Err(InvalidInputReasonV0::MacroValidationFailed);
            }
            expand_stream_v0(
                &macro_def.body_tokens,
                macro_frames,
                counters,
                out,
                active_macros,
                expansion_count,
                depth + 1,
            )
        }
        MacroBindingV0::ControlSeqLiteral(target) => {
            expand_stream_v0(
                &[TokenV0::ControlSeq(target)],
                macro_frames,
                counters,
                out,
                active_macros,
                expansion_count,
                depth + 1,
            )
        }
        MacroBindingV0::LetAlias {
            target_name: _,
            resolved_binding,
        } => expand_binding_v0(
            name,
            *resolved_binding,
            macro_frames,
            counters,
            out,
            active_macros,
            expansion_count,
            depth + 1,
        ),
    };
    active_macros.pop();
    result
}

fn total_macro_defs_v0(macro_frames: &[BTreeMap<Vec<u8>, MacroBindingV0>]) -> usize {
    macro_frames.iter().map(|frame| frame.len()).sum()
}

fn skip_space_tokens_v0(tokens: &[TokenV0], mut index: usize) -> usize {
    while matches!(tokens.get(index), Some(TokenV0::Space)) {
        index += 1;
    }
    index
}

fn validate_macro_body_tokens_v0(
    body_tokens: &[TokenV0],
    param_count: u8,
) -> Result<(), InvalidInputReasonV0> {
    let mut index = 0usize;
    while index < body_tokens.len() {
        match body_tokens.get(index) {
            Some(TokenV0::Char(b'#')) => match param_count {
                0 => return Err(InvalidInputReasonV0::MacroParamsUnsupported),
                1 => match body_tokens.get(index + 1) {
                    Some(TokenV0::Char(b'1')) => index += 2,
                    _ => return Err(InvalidInputReasonV0::MacroParamsUnsupported),
                },
                _ => return Err(InvalidInputReasonV0::MacroParamsUnsupported),
            },
            Some(_) => index += 1,
            None => break,
        }
    }
    Ok(())
}

fn substitute_single_param_placeholders_v0(
    body_tokens: &[TokenV0],
    argument_tokens: &[TokenV0],
) -> Result<Vec<TokenV0>, InvalidInputReasonV0> {
    let mut out = Vec::<TokenV0>::new();
    let mut index = 0usize;
    while index < body_tokens.len() {
        match body_tokens.get(index) {
            Some(TokenV0::Char(b'#')) => match body_tokens.get(index + 1) {
                Some(TokenV0::Char(b'1')) => {
                    for token in argument_tokens {
                        push_checked_v0(&mut out, token.clone())?;
                    }
                    index += 2;
                }
                _ => return Err(InvalidInputReasonV0::MacroParamsUnsupported),
            },
            Some(token) => {
                push_checked_v0(&mut out, token.clone())?;
                index += 1;
            }
            None => break,
        }
    }
    Ok(out)
}

fn parse_balanced_group_payload_v0(
    tokens: &[TokenV0],
    begin_group_index: usize,
) -> Result<(Vec<TokenV0>, usize), InvalidInputReasonV0> {
    if !matches!(tokens.get(begin_group_index), Some(TokenV0::BeginGroup)) {
        return Err(InvalidInputReasonV0::MacroValidationFailed);
    }

    let mut depth = 1usize;
    let mut payload = Vec::<TokenV0>::new();
    let mut index = begin_group_index + 1;
    while index < tokens.len() {
        match tokens.get(index) {
            Some(TokenV0::BeginGroup) => {
                depth += 1;
                payload.push(TokenV0::BeginGroup);
            }
            Some(TokenV0::EndGroup) => {
                depth -= 1;
                if depth == 0 {
                    return Ok((payload, index + 1));
                }
                payload.push(TokenV0::EndGroup);
            }
            Some(token) => payload.push(token.clone()),
            None => break,
        }
        index += 1;
    }
    Err(InvalidInputReasonV0::MacroValidationFailed)
}

fn push_checked_v0(out: &mut Vec<TokenV0>, token: TokenV0) -> Result<(), InvalidInputReasonV0> {
    if out.len() >= MAX_TOKENS_V0 {
        return Err(InvalidInputReasonV0::MacroValidationFailed);
    }
    out.push(token);
    Ok(())
}

fn push_ascii_bytes_v0(out: &mut Vec<TokenV0>, bytes: &[u8]) -> Result<(), InvalidInputReasonV0> {
    for byte in bytes {
        push_checked_v0(out, TokenV0::Char(*byte))?;
    }
    Ok(())
}

fn parse_count_assignment_v0(
    tokens: &[TokenV0],
    count_index: usize,
    counters: &mut [u32; 2],
) -> Result<usize, InvalidInputReasonV0> {
    let register_index = match tokens.get(count_index + 1) {
        Some(TokenV0::Char(b'0')) => 0usize,
        Some(TokenV0::Char(b'1')) => 1usize,
        _ => return Err(InvalidInputReasonV0::MacroCountAssignmentUnsupported),
    };
    if !matches!(tokens.get(count_index + 2), Some(TokenV0::Char(b'='))) {
        return Err(InvalidInputReasonV0::MacroCountAssignmentUnsupported);
    }

    let mut index = count_index + 3;
    let mut value: u32 = 0;
    let mut saw_digit = false;
    while let Some(TokenV0::Char(byte)) = tokens.get(index) {
        if !byte.is_ascii_digit() {
            break;
        }
        saw_digit = true;
        value = value
            .checked_mul(10)
            .and_then(|current| current.checked_add((byte - b'0') as u32))
            .ok_or(InvalidInputReasonV0::MacroCountAssignmentUnsupported)?;
        if value > MAX_COUNT_VALUE_V0 {
            return Err(InvalidInputReasonV0::MacroCountAssignmentUnsupported);
        }
        index += 1;
    }
    if !saw_digit {
        return Err(InvalidInputReasonV0::MacroCountAssignmentUnsupported);
    }

    counters[register_index] = value;
    Ok(index)
}

fn parse_the_v0(
    tokens: &[TokenV0],
    the_index: usize,
    counters: &[u32; 2],
) -> Result<(Vec<TokenV0>, usize), InvalidInputReasonV0> {
    if !matches!(tokens.get(the_index + 1), Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"count")
    {
        return Err(InvalidInputReasonV0::MacroTheUnsupported);
    }

    let register_index = match tokens.get(the_index + 2) {
        Some(TokenV0::Char(b'0')) => 0usize,
        Some(TokenV0::Char(b'1')) => 1usize,
        _ => return Err(InvalidInputReasonV0::MacroTheUnsupported),
    };

    let mut out = Vec::<TokenV0>::new();
    let digits = counters[register_index].to_string();
    push_ascii_bytes_v0(&mut out, digits.as_bytes())?;
    Ok((out, the_index + 3))
}
