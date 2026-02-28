use std::collections::BTreeMap;

use super::ifnum_v0::parse_ifnum_v0;
use super::ifx_v0::parse_ifx_v0;
use crate::reasons_v0::InvalidInputReasonV0;
use crate::tex::tokenize_v0::{TokenV0, MAX_TOKENS_V0};

#[path = "macro_v0/bindings.rs"]
mod bindings;
#[path = "macro_v0/count_the.rs"]
mod count_the;
#[path = "macro_v0/csname_expandafter.rs"]
mod csname_expandafter;
#[path = "macro_v0/def_xdef.rs"]
mod def_xdef;
#[path = "macro_v0/global_prefix.rs"]
mod global_prefix;
#[path = "macro_v0/group_synonyms.rs"]
mod group_synonyms;
#[path = "macro_v0/let_futurelet.rs"]
mod let_futurelet;
#[path = "macro_v0/noexpand.rs"]
mod noexpand;
#[path = "macro_v0/newcommand_renewcommand.rs"]
mod newcommand_renewcommand;
#[path = "macro_v0/string_meaning.rs"]
mod string_meaning;
#[path = "macro_v0/utils.rs"]
mod utils;

use bindings::{
    compare_ifx_control_sequences_v0, expand_binding_v0, lookup_macro_binding_v0, MacroBindingV0,
};
use count_the::{parse_count_assignment_v0, parse_the_v0};
use csname_expandafter::{parse_csname_v0, parse_expandafter_v0};
use def_xdef::{parse_def_v0, parse_xdef_v0};
use global_prefix::parse_global_prefixed_macro_binding_v0;
use group_synonyms::{control_seq_to_group_token_v0, is_endgroup_synonym_v0};
use let_futurelet::{parse_futurelet_v0, parse_let_v0};
use noexpand::parse_noexpand_v0;
use newcommand_renewcommand::{parse_newcommand_v0, parse_renewcommand_v0};
use string_meaning::{parse_meaning_v0, parse_string_v0};
use utils::{push_checked_v0, substitute_single_param_placeholders_v0};

pub(crate) const MAX_MACROS_V0: usize = 4096;
pub(crate) const MAX_MACRO_EXPANSIONS_V0: usize = 4096;
pub(crate) const MAX_MACRO_DEPTH_V0: usize = 64;
pub(crate) const MAX_GROUP_DEPTH_V0: usize = 1024;

enum ConditionalKindV0 {
    Ifnum,
    Ifx,
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

    let mut last_conditional_kind = None::<ConditionalKindV0>;
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
                index =
                    parse_def_v0(tokens, index, macro_frames, counters, is_global, expand_body)?;
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"newcommand" => {
                index = parse_newcommand_v0(tokens, index, macro_frames)?;
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"renewcommand" => {
                index = parse_renewcommand_v0(tokens, index, macro_frames)?;
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
            TokenV0::ControlSeq(name)
                if control_seq_to_group_token_v0(name.as_slice()).is_some() =>
            {
                let group_token =
                    control_seq_to_group_token_v0(name.as_slice()).expect("checked is_some");
                if matches!(group_token, TokenV0::BeginGroup) {
                    if macro_frames.len() >= MAX_GROUP_DEPTH_V0 {
                        return Err(InvalidInputReasonV0::MacroGroupDepthExceeded);
                    }
                    macro_frames.push(BTreeMap::new());
                } else if macro_frames.len() > 1 {
                    macro_frames.pop();
                } else if is_endgroup_synonym_v0(name.as_slice()) {
                    return Err(InvalidInputReasonV0::MacroGroupUnderflow);
                }
                push_checked_v0(out, group_token)?;
                index += 1;
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"relax" => {
                index += 1;
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"noexpand" => {
                index = parse_noexpand_v0(tokens, index, out)?;
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"ifnum" => {
                let (selected_tokens, next_index) = parse_ifnum_v0(tokens, index, counters, 0)?;
                expand_stream_v0(
                    &selected_tokens,
                    macro_frames,
                    counters,
                    out,
                    active_macros,
                    expansion_count,
                    depth + 1,
                )?;
                index = next_index;
                last_conditional_kind = Some(ConditionalKindV0::Ifnum);
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"ifx" => {
                let mut compare = |left: &[u8], right: &[u8]| {
                    compare_ifx_control_sequences_v0(macro_frames, left, right)
                };
                let (selected_tokens, next_index) =
                    parse_ifx_v0(tokens, index, counters, 0, &mut compare)?;
                expand_stream_v0(
                    &selected_tokens,
                    macro_frames,
                    counters,
                    out,
                    active_macros,
                    expansion_count,
                    depth + 1,
                )?;
                index = next_index;
                last_conditional_kind = Some(ConditionalKindV0::Ifx);
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"else" => {
                return Err(match last_conditional_kind {
                    Some(ConditionalKindV0::Ifx) => {
                        InvalidInputReasonV0::MacroIfxElseWithoutIf
                    }
                    _ => InvalidInputReasonV0::MacroIfElseWithoutIf,
                });
            }
            TokenV0::ControlSeq(name) => match lookup_macro_binding_v0(macro_frames, name) {
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
                                utils::parse_balanced_group_payload_v0(tokens, index + 1)?;
                            let substituted_body =
                                substitute_single_param_placeholders_v0(
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
            },
            token => {
                push_checked_v0(out, token.clone())?;
                index += 1;
            }
        }
    }
    if out.len() > MAX_TOKENS_V0 {
        return Err(InvalidInputReasonV0::MacroValidationFailed);
    }
    Ok(())
}
