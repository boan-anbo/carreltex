use crate::tex::tokenize_v0::TokenV0;

pub(super) const MAX_OK_LIST_DEPTH_V0: usize = 16;

pub(super) enum ListFrameV0 {
    Itemize,
    Enumerate { next: u32 },
}

pub(super) enum ListStateV0 {
    Lists(Vec<ListFrameV0>),
    Thebibliography,
    Figure,
    Table,
}

fn consume_group_literal_v0(
    tokens: &[TokenV0],
    mut index: usize,
    literal: &[u8],
) -> Option<usize> {
    if !matches!(tokens.get(index), Some(TokenV0::BeginGroup)) {
        return None;
    }
    index += 1;
    for expected in literal {
        if !matches!(tokens.get(index), Some(TokenV0::Char(value)) if value == expected) {
            return None;
        }
        index += 1;
    }
    if !matches!(tokens.get(index), Some(TokenV0::EndGroup)) {
        return None;
    }
    Some(index + 1)
}

pub(super) fn list_stack_active_v0(state: &Option<ListStateV0>) -> bool {
    matches!(state, Some(ListStateV0::Lists(_)))
}

fn push_list_frame_v0(state: &mut Option<ListStateV0>, frame: ListFrameV0) -> Option<()> {
    match state {
        None => {
            *state = Some(ListStateV0::Lists(vec![frame]));
            Some(())
        }
        Some(ListStateV0::Lists(stack)) => {
            if stack.len() >= MAX_OK_LIST_DEPTH_V0 {
                return None;
            }
            stack.push(frame);
            Some(())
        }
        Some(ListStateV0::Thebibliography | ListStateV0::Figure | ListStateV0::Table) => None,
    }
}

pub(super) fn begin_list_v0(
    tokens: &[TokenV0],
    index: usize,
    state: &mut Option<ListStateV0>,
) -> Option<usize> {
    if let Some(next_index) = consume_group_literal_v0(tokens, index, b"itemize") {
        push_list_frame_v0(state, ListFrameV0::Itemize)?;
        return Some(next_index);
    }
    if let Some(next_index) = consume_group_literal_v0(tokens, index, b"enumerate") {
        push_list_frame_v0(state, ListFrameV0::Enumerate { next: 1 })?;
        return Some(next_index);
    }
    None
}

fn emit_list_end_v0(body: &mut Vec<u8>, previous_was_space: &mut bool) {
    body.push(0x0a);
    *previous_was_space = true;
}

pub(super) fn end_list_v0(
    tokens: &[TokenV0],
    index: usize,
    state: &mut Option<ListStateV0>,
    body: &mut Vec<u8>,
    previous_was_space: &mut bool,
) -> Option<usize> {
    match state {
        Some(ListStateV0::Lists(stack)) => {
            if matches!(stack.last(), Some(ListFrameV0::Itemize)) {
                let next_index = consume_group_literal_v0(tokens, index, b"itemize")?;
                stack.pop();
                emit_list_end_v0(body, previous_was_space);
                if stack.is_empty() {
                    *state = None;
                }
                return Some(next_index);
            }
            if matches!(stack.last(), Some(ListFrameV0::Enumerate { .. })) {
                let next_index = consume_group_literal_v0(tokens, index, b"enumerate")?;
                stack.pop();
                emit_list_end_v0(body, previous_was_space);
                if stack.is_empty() {
                    *state = None;
                }
                return Some(next_index);
            }
            None
        }
        _ => None,
    }
}

fn emit_indent_v0(body: &mut Vec<u8>, depth: usize) {
    let indent = depth.saturating_sub(1) * 2;
    for _ in 0..indent {
        body.push(b' ');
    }
}

pub(super) fn emit_list_item_v0(
    state: &mut Option<ListStateV0>,
    body: &mut Vec<u8>,
    previous_was_space: &mut bool,
) -> Option<()> {
    let Some(ListStateV0::Lists(stack)) = state else {
        return None;
    };
    let depth = stack.len();
    let frame = stack.last_mut()?;
    body.push(0x0a);
    emit_indent_v0(body, depth);
    match frame {
        ListFrameV0::Itemize => {
            body.push(b'-');
            body.push(b' ');
        }
        ListFrameV0::Enumerate { next } => {
            for byte in next.to_string().as_bytes() {
                body.push(*byte);
            }
            body.push(b'.');
            body.push(b' ');
            *next += 1;
        }
    }
    *previous_was_space = true;
    Some(())
}
