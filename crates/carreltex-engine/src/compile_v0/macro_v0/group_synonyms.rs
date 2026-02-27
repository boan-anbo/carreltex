use crate::tex::tokenize_v0::TokenV0;

pub(super) fn control_seq_to_group_token_v0(name: &[u8]) -> Option<TokenV0> {
    match name {
        b"begingroup" => Some(TokenV0::BeginGroup),
        b"endgroup" => Some(TokenV0::EndGroup),
        b"bgroup" => Some(TokenV0::BeginGroup),
        b"egroup" => Some(TokenV0::EndGroup),
        _ => None,
    }
}

pub(super) fn is_endgroup_synonym_v0(name: &[u8]) -> bool {
    matches!(name, b"endgroup" | b"egroup")
}
