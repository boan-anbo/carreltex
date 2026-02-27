use super::*;

#[derive(Clone)]
pub(super) struct MacroDefV0 {
    pub(super) param_count: u8,
    pub(super) body_tokens: Vec<TokenV0>,
}

#[derive(Clone)]
pub(super) enum MacroBindingV0 {
    Macro(MacroDefV0),
    ControlSeqLiteral(Vec<u8>),
    LetAlias {
        target_name: Vec<u8>,
        resolved_binding: Box<MacroBindingV0>,
    },
}

pub(super) fn lookup_macro_binding_v0(
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

enum IfxComparableBindingV0 {
    Undefined,
    AliasTarget(Vec<u8>),
    Macro(MacroDefV0),
}

pub(super) fn compare_ifx_control_sequences_v0(
    macro_frames: &[BTreeMap<Vec<u8>, MacroBindingV0>],
    left: &[u8],
    right: &[u8],
) -> bool {
    match (
        classify_ifx_binding_v0(macro_frames, left),
        classify_ifx_binding_v0(macro_frames, right),
    ) {
        (IfxComparableBindingV0::Undefined, IfxComparableBindingV0::Undefined) => true,
        (
            IfxComparableBindingV0::AliasTarget(left_target),
            IfxComparableBindingV0::AliasTarget(right_target),
        ) => left_target == right_target,
        (IfxComparableBindingV0::Macro(left_macro), IfxComparableBindingV0::Macro(right_macro)) => {
            left_macro.param_count == right_macro.param_count
                && left_macro.body_tokens == right_macro.body_tokens
        }
        _ => false,
    }
}

fn classify_ifx_binding_v0(
    macro_frames: &[BTreeMap<Vec<u8>, MacroBindingV0>],
    name: &[u8],
) -> IfxComparableBindingV0 {
    match lookup_macro_binding_v0(macro_frames, name) {
        None => IfxComparableBindingV0::Undefined,
        Some(MacroBindingV0::Macro(definition)) => IfxComparableBindingV0::Macro(definition),
        Some(MacroBindingV0::ControlSeqLiteral(target_name)) => {
            IfxComparableBindingV0::AliasTarget(resolve_alias_target_name_v0(macro_frames, target_name))
        }
        Some(MacroBindingV0::LetAlias {
            target_name: _,
            resolved_binding,
        }) => classify_ifx_from_resolved_binding_v0(*resolved_binding),
    }
}

fn classify_ifx_from_resolved_binding_v0(binding: MacroBindingV0) -> IfxComparableBindingV0 {
    match binding {
        MacroBindingV0::Macro(definition) => IfxComparableBindingV0::Macro(definition),
        MacroBindingV0::ControlSeqLiteral(_name) => IfxComparableBindingV0::Undefined,
        MacroBindingV0::LetAlias {
            target_name: _,
            resolved_binding,
        } => classify_ifx_from_resolved_binding_v0(*resolved_binding),
    }
}

fn resolve_alias_target_name_v0(
    macro_frames: &[BTreeMap<Vec<u8>, MacroBindingV0>],
    start: Vec<u8>,
) -> Vec<u8> {
    let mut current = start;
    let mut seen = Vec::<Vec<u8>>::new();
    loop {
        if seen.iter().any(|entry| entry == &current) {
            return current;
        }
        seen.push(current.clone());
        match lookup_macro_binding_v0(macro_frames, &current) {
            Some(MacroBindingV0::ControlSeqLiteral(next)) => current = next,
            Some(MacroBindingV0::LetAlias {
                target_name,
                resolved_binding: _,
            }) => current = target_name,
            _ => return current,
        }
    }
}

pub(super) fn snapshot_let_binding_v0(
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

pub(super) fn expand_binding_v0(
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
            super::expand_stream_v0(
                &macro_def.body_tokens,
                macro_frames,
                counters,
                out,
                active_macros,
                expansion_count,
                depth + 1,
            )
        }
        MacroBindingV0::ControlSeqLiteral(target) => super::expand_stream_v0(
            &[TokenV0::ControlSeq(target)],
            macro_frames,
            counters,
            out,
            active_macros,
            expansion_count,
            depth + 1,
        ),
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

pub(super) fn total_macro_defs_v0(macro_frames: &[BTreeMap<Vec<u8>, MacroBindingV0>]) -> usize {
    macro_frames.iter().map(|frame| frame.len()).sum()
}
