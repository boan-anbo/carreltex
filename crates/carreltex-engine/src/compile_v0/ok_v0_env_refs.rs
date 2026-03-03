use crate::tex::tokenize_v0::TokenV0;
use super::consume_char_space_nested_group_v0;

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
        b"eqref" if matches!(env_markers, OkEnvMarkersV0::EquationFamily) => Some(b"EQREF"),
        _ => None,
    }
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
                    index = consume_char_space_nested_group_v0(tokens, index + 1, end)?;
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
