#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenV0 {
    ControlSeq(Vec<u8>),
    Char(u8),
    BeginGroup,
    EndGroup,
    Space,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenizeErrorV0 {
    InvalidInput,
    CaretNotSupported,
    AccentNotSupported,
    ControlSeqNonAscii,
    TooManyTokens,
}

pub const MAX_TOKENS_V0: usize = 1_000_000;

mod caret;
mod comment;
mod control_seq;
mod core;
mod whitespace;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_textword_144;

/// Tokenize TeX input bytes with strict v0 assumptions.
///
/// v0 rules:
/// - Rejects NUL (`0x00`) anywhere (`InvalidInput`).
/// - Decodes `^^hh` where `h` is hex (`[0-9a-fA-F]`) to one byte.
/// - Any other `^^` form is `CaretNotSupported`.
/// - Accent control symbols `\~`, `\^`, and `\"` are explicitly blocked in v0
///   (`AccentNotSupported`), including braced forms like `\~{x}`.
/// - `%` starts a comment that is skipped until `\n`, `\r`, or EOF; the line
///   is not consumed by the comment and is processed normally. Caret decoding
///   is not applied while consuming comment bytes.
/// - Whitespace bytes (`' '`, `\t`, `\r`, `\n`) collapse into one `Space`.
/// - `{` and `}` become `BeginGroup` / `EndGroup`.
/// - `\` starts a control sequence:
///   - If followed by ASCII letters, consume a control word and emit
///     `ControlSeq(name_bytes)`. A following whitespace run is swallowed
///     (no `Space` token emitted).
///   - Control sequence bytes must be ASCII-only (`ControlSeqNonAscii` on
///     violations).
///   - Control word `\verb` is explicitly blocked in v0 (`InvalidInput`).
///   - Otherwise emit control symbol as `ControlSeq(vec![next_byte])`.
///   - A trailing terminal backslash is `InvalidInput`.
/// - All other bytes become `Char(byte)`.
/// - Fails with `TooManyTokens` if output would exceed `MAX_TOKENS_V0`.
pub use core::tokenize_v0;
