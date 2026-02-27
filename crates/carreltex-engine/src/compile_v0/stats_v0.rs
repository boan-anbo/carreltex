use crate::tex::tokenize_v0::TokenV0;
use carreltex_core::build_tex_stats_json_v0;

pub(crate) fn build_tex_stats_from_tokens_v0(tokens: &[TokenV0]) -> Result<String, ()> {
    let mut depth: u64 = 0;
    let mut max_depth: u64 = 0;
    let mut control_seq_count: u64 = 0;
    let mut char_count: u64 = 0;
    let mut space_count: u64 = 0;
    let mut begin_group_count: u64 = 0;
    let mut end_group_count: u64 = 0;

    for token in tokens {
        match token {
            TokenV0::ControlSeq(_) => {
                control_seq_count = control_seq_count.checked_add(1).ok_or(())?;
            }
            TokenV0::Char(_) => {
                char_count = char_count.checked_add(1).ok_or(())?;
            }
            TokenV0::Space => {
                space_count = space_count.checked_add(1).ok_or(())?;
            }
            TokenV0::BeginGroup => {
                begin_group_count = begin_group_count.checked_add(1).ok_or(())?;
                depth = depth.checked_add(1).ok_or(())?;
                max_depth = max_depth.max(depth);
            }
            TokenV0::EndGroup => {
                if depth == 0 {
                    return Err(());
                }
                end_group_count = end_group_count.checked_add(1).ok_or(())?;
                depth -= 1;
            }
        }
    }

    if depth != 0 {
        return Err(());
    }

    let token_count: u64 = tokens.len().try_into().map_err(|_| ())?;
    build_tex_stats_json_v0(
        token_count,
        control_seq_count,
        char_count,
        space_count,
        begin_group_count,
        end_group_count,
        max_depth,
    )
    .map_err(|_| ())
}
