pub(super) fn skip_comment_raw_v0(input: &[u8], mut index: usize) -> usize {
    index += 1;
    while index < input.len() && input[index] != b'\n' && input[index] != b'\r' {
        index += 1;
    }
    index
}
