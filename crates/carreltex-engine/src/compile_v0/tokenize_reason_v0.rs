use crate::reasons_v0::InvalidInputReasonV0;
use crate::tex::tokenize_v0::TokenizeErrorV0;

pub(super) fn map_tokenize_error_to_reason_v0(error: TokenizeErrorV0) -> InvalidInputReasonV0 {
    match error {
        TokenizeErrorV0::CaretNotSupported => InvalidInputReasonV0::TokenizerCaretNotSupported,
        TokenizeErrorV0::AccentNotSupported => InvalidInputReasonV0::TokenizerAccentNotSupported,
        TokenizeErrorV0::ControlSeqNonAscii => InvalidInputReasonV0::TokenizerControlSeqNonAscii,
        _ => InvalidInputReasonV0::TokenizeFailed,
    }
}
