#[derive(Clone, Copy)]
pub(crate) enum InvalidInputReasonV0 {
    MountFinalizeFailed,
    RequestInvalid,
    EntrypointMissing,
    TokenizeFailed,
    TokenizerCaretNotSupported,
    TokenizerControlSeqNonAscii,
    StatsBuildFailed,
    InputValidationFailed,
    InputCycleFailed,
    InputDepthExceeded,
    InputExpansionsExceeded,
    MacroValidationFailed,
    MacroParamsUnsupported,
    MacroCycleFailed,
    MacroDepthExceeded,
    MacroExpansionsExceeded,
    MacroGlobalPrefixUnsupported,
    MacroLetUnsupported,
    MacroFutureletUnsupported,
    MacroExpandafterUnsupported,
    MacroCsnameUnsupported,
    MacroStringUnsupported,
    MacroMeaningUnsupported,
    MacroCountAssignmentUnsupported,
    MacroTheUnsupported,
    MacroXdefUnsupported,
    MacroNoexpandUnsupported,
    MacroGroupUnderflow,
    MacroIfnumUnsupported,
    MacroIfDepthExceeded,
    MacroIfElseDuplicate,
    MacroIfElseWithoutIf,
    MacroIfMissingFi,
    MacroIfxUnsupported,
    MacroIfxElseDuplicate,
    MacroIfxElseWithoutIf,
    MacroIfxMissingFi,
    MacroIfxDepthExceeded,
}

pub(crate) fn invalid_log_bytes_v0(reason: InvalidInputReasonV0) -> &'static [u8] {
    match reason {
        InvalidInputReasonV0::MountFinalizeFailed => b"INVALID_INPUT: mount_finalize_failed",
        InvalidInputReasonV0::RequestInvalid => b"INVALID_INPUT: request_invalid",
        InvalidInputReasonV0::EntrypointMissing => b"INVALID_INPUT: entrypoint_missing",
        InvalidInputReasonV0::TokenizeFailed => b"INVALID_INPUT: tokenize_failed",
        InvalidInputReasonV0::TokenizerCaretNotSupported => {
            b"INVALID_INPUT: tokenizer_caret_not_supported"
        }
        InvalidInputReasonV0::TokenizerControlSeqNonAscii => {
            b"INVALID_INPUT: tokenizer_control_seq_non_ascii"
        }
        InvalidInputReasonV0::StatsBuildFailed => b"INVALID_INPUT: stats_build_failed",
        InvalidInputReasonV0::InputValidationFailed => b"INVALID_INPUT: input_validation_failed",
        InvalidInputReasonV0::InputCycleFailed => b"INVALID_INPUT: input_cycle_failed",
        InvalidInputReasonV0::InputDepthExceeded => b"INVALID_INPUT: input_depth_exceeded",
        InvalidInputReasonV0::InputExpansionsExceeded => {
            b"INVALID_INPUT: input_expansions_exceeded"
        }
        InvalidInputReasonV0::MacroValidationFailed => b"INVALID_INPUT: macro_validation_failed",
        InvalidInputReasonV0::MacroParamsUnsupported => b"INVALID_INPUT: macro_params_unsupported",
        InvalidInputReasonV0::MacroCycleFailed => b"INVALID_INPUT: macro_cycle_failed",
        InvalidInputReasonV0::MacroDepthExceeded => b"INVALID_INPUT: macro_depth_exceeded",
        InvalidInputReasonV0::MacroExpansionsExceeded => {
            b"INVALID_INPUT: macro_expansions_exceeded"
        }
        InvalidInputReasonV0::MacroGlobalPrefixUnsupported => {
            b"INVALID_INPUT: macro_global_prefix_unsupported"
        }
        InvalidInputReasonV0::MacroLetUnsupported => b"INVALID_INPUT: macro_let_unsupported",
        InvalidInputReasonV0::MacroFutureletUnsupported => {
            b"INVALID_INPUT: macro_futurelet_unsupported"
        }
        InvalidInputReasonV0::MacroExpandafterUnsupported => {
            b"INVALID_INPUT: macro_expandafter_unsupported"
        }
        InvalidInputReasonV0::MacroCsnameUnsupported => {
            b"INVALID_INPUT: macro_csname_unsupported"
        }
        InvalidInputReasonV0::MacroStringUnsupported => {
            b"INVALID_INPUT: macro_string_unsupported"
        }
        InvalidInputReasonV0::MacroMeaningUnsupported => {
            b"INVALID_INPUT: macro_meaning_unsupported"
        }
        InvalidInputReasonV0::MacroCountAssignmentUnsupported => {
            b"INVALID_INPUT: macro_count_assignment_unsupported"
        }
        InvalidInputReasonV0::MacroTheUnsupported => b"INVALID_INPUT: macro_the_unsupported",
        InvalidInputReasonV0::MacroXdefUnsupported => b"INVALID_INPUT: macro_xdef_unsupported",
        InvalidInputReasonV0::MacroNoexpandUnsupported => {
            b"INVALID_INPUT: macro_noexpand_unsupported"
        }
        InvalidInputReasonV0::MacroGroupUnderflow => b"INVALID_INPUT: macro_group_underflow",
        InvalidInputReasonV0::MacroIfnumUnsupported => b"INVALID_INPUT: macro_ifnum_unsupported",
        InvalidInputReasonV0::MacroIfDepthExceeded => b"INVALID_INPUT: macro_if_depth_exceeded",
        InvalidInputReasonV0::MacroIfElseDuplicate => b"INVALID_INPUT: macro_if_else_duplicate",
        InvalidInputReasonV0::MacroIfElseWithoutIf => {
            b"INVALID_INPUT: macro_if_else_without_if"
        }
        InvalidInputReasonV0::MacroIfMissingFi => b"INVALID_INPUT: macro_if_missing_fi",
        InvalidInputReasonV0::MacroIfxUnsupported => b"INVALID_INPUT: macro_ifx_unsupported",
        InvalidInputReasonV0::MacroIfxElseDuplicate => b"INVALID_INPUT: macro_ifx_else_duplicate",
        InvalidInputReasonV0::MacroIfxElseWithoutIf => {
            b"INVALID_INPUT: macro_ifx_else_without_if"
        }
        InvalidInputReasonV0::MacroIfxMissingFi => b"INVALID_INPUT: macro_ifx_missing_fi",
        InvalidInputReasonV0::MacroIfxDepthExceeded => {
            b"INVALID_INPUT: macro_ifx_depth_exceeded"
        }
    }
}
