use super::*;
use super::def_xdef::{parse_def_v0, parse_xdef_v0};
use super::let_futurelet::{parse_futurelet_v0, parse_let_v0};

pub(super) fn parse_global_prefixed_macro_binding_v0(
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
        _ => Err(InvalidInputReasonV0::MacroGlobalPrefixUnsupported),
    }
}
