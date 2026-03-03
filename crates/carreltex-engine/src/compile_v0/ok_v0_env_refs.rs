use crate::tex::tokenize_v0::TokenV0;
use super::{
    consume_char_space_nested_group_v0, consume_optional_nested_bracket_span_v0,
};

const MAX_OK_CITE_NOTE_TOKENS_V0: usize = 2048;
const MAX_OK_REF_NOTE_TOKENS_V0: usize = 2048;
const MAX_OK_GROUP_DEPTH_V0: usize = 64;

#[derive(Copy, Clone)]
pub(super) enum OkEnvMarkersV0 {
    EquationFamily,
    TheoremFamily,
}

fn marker_for_env_command_v0(name: &[u8], env_markers: OkEnvMarkersV0) -> Option<&'static [u8]> {
    match name {
        b"label" => match env_markers {
            OkEnvMarkersV0::EquationFamily => Some(b"EQ"),
            OkEnvMarkersV0::TheoremFamily => Some(b"LBL"),
        },
        b"cite" | b"citet" | b"citep" => Some(b"CITE"),
        b"ref" | b"autoref" => match env_markers {
            OkEnvMarkersV0::EquationFamily => Some(b"EQREF"),
            OkEnvMarkersV0::TheoremFamily => Some(b"THMREF"),
        },
        b"cref" | b"Cref" => match env_markers {
            OkEnvMarkersV0::EquationFamily => Some(b"EQREF"),
            OkEnvMarkersV0::TheoremFamily => Some(b"THMREF"),
        },
        b"eqref" if matches!(env_markers, OkEnvMarkersV0::EquationFamily) => Some(b"EQREF"),
        b"pageref" => Some(b"PAGEREF"),
        _ => None,
    }
}

fn is_cite_marker_command_v0(name: &[u8]) -> bool {
    matches!(name, b"cite" | b"citet" | b"citep")
}

fn is_ref_marker_command_v0(name: &[u8]) -> bool {
    matches!(name, b"ref" | b"autoref" | b"eqref" | b"pageref" | b"cref" | b"Cref")
}

fn emit_marker_v0(marker: &[u8], body: &mut Vec<u8>, previous_was_space: &mut bool) {
    body.push(b' ');
    body.push(b'[');
    body.extend_from_slice(marker);
    body.push(b']');
    *previous_was_space = false;
}

pub(super) fn emit_ok_markers_in_env_v0(
    tokens: &[TokenV0],
    start: usize,
    end: usize,
    env_markers: OkEnvMarkersV0,
    body: &mut Vec<u8>,
    previous_was_space: &mut bool,
) -> Option<()> {
    let mut index = start;
    while index < end {
        match tokens.get(index)? {
            TokenV0::ControlSeq(name) if name.as_slice() == b"begin" => return None,
            TokenV0::ControlSeq(name) => {
                if let Some(marker) = marker_for_env_command_v0(name.as_slice(), env_markers) {
                    let mut arg_start = index + 1;
                    if is_cite_marker_command_v0(name.as_slice())
                        || is_ref_marker_command_v0(name.as_slice())
                    {
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
                    index = consume_char_space_nested_group_v0(tokens, arg_start, end)?;
                    emit_marker_v0(marker, body, previous_was_space);
                } else {
                    index += 1;
                }
            }
            _ => index += 1,
        }
    }
    Some(())
}
