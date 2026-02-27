use super::*;
use super::bindings::{lookup_macro_binding_v0, MacroBindingV0};
use super::utils::{push_ascii_bytes_v0, skip_space_tokens_v0};

pub(super) fn parse_string_v0(
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

pub(super) fn parse_meaning_v0(
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
