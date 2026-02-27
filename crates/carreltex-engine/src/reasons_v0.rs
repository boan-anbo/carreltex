#[derive(Clone, Copy)]
pub(crate) enum InvalidInputReasonV0 {
    MountFinalizeFailed,
    RequestInvalid,
    EntrypointMissing,
    TokenizeFailed,
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
}

pub(crate) fn invalid_log_bytes_v0(reason: InvalidInputReasonV0) -> &'static [u8] {
    match reason {
        InvalidInputReasonV0::MountFinalizeFailed => b"INVALID_INPUT: mount_finalize_failed",
        InvalidInputReasonV0::RequestInvalid => b"INVALID_INPUT: request_invalid",
        InvalidInputReasonV0::EntrypointMissing => b"INVALID_INPUT: entrypoint_missing",
        InvalidInputReasonV0::TokenizeFailed => b"INVALID_INPUT: tokenize_failed",
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
    }
}
