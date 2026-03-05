use crate::tex::tokenize_v0::TokenV0;

const NEWLINE_MARKER_V0: u8 = 0x0a;
const PAGE_BREAK_MARKER_V0: u8 = 0x0c;
const CARRELPAR_MARKER_CONTROL_V0: &[u8] = b"carrelpar";
const CARRELNEWLINE_MARKER_CONTROL_V0: &[u8] = b"carrelnewline";
const HARD_LINE_BREAK_CONTROL_V0: &[u8] = b"\\";
const NEWLINE_ALIAS_CONTROL_V0: &[u8] = b"newline";
const LINEBREAK_ALIAS_CONTROL_V0: &[u8] = b"linebreak";
const PAGEBREAK_ALIAS_CONTROL_V0: &[u8] = b"pagebreak";
const BEGIN_CONTROL_V0: &[u8] = b"begin";
const END_CONTROL_V0: &[u8] = b"end";
const ITEM_CONTROL_V0: &[u8] = b"item";
const DOCUMENT_ENV_V0: &[u8] = b"document";
const ITEMIZE_ENV_V0: &[u8] = b"itemize";
const ENUMERATE_ENV_V0: &[u8] = b"enumerate";
const QUOTE_ENV_V0: &[u8] = b"quote";
const CENTER_ENV_V0: &[u8] = b"center";
const CENTERLINE_CONTROL_V0: &[u8] = b"centerline";
const FLUSHRIGHT_ENV_V0: &[u8] = b"flushright";
const RIGHTLINE_CONTROL_V0: &[u8] = b"rightline";
const NOINDENT_PREFIX_MARKER_V0: &[u8] = b"~ ";
const SECTION_HEADING_PREFIX_MARKER_V0: &[u8] = b"@S ";
const SUBSECTION_HEADING_PREFIX_MARKER_V0: &[u8] = b"@s ";
const ITALIC_START_MARKER_V0: u8 = b'[';
const ITALIC_END_MARKER_V0: u8 = b']';
const BOLD_START_MARKER_V0: u8 = b'{';
const BOLD_END_MARKER_V0: u8 = b'}';
const SECTION_CONTROL_V0: &[u8] = b"section";
const SUBSECTION_CONTROL_V0: &[u8] = b"subsection";
const SUBSUBSECTION_CONTROL_V0: &[u8] = b"subsubsection";
const PARAGRAPH_CONTROL_V0: &[u8] = b"paragraph";
const SUBPARAGRAPH_CONTROL_V0: &[u8] = b"subparagraph";

#[derive(Default)]
struct TitleMetaV0 {
    title: Option<Vec<u8>>,
    author: Option<Vec<u8>>,
    date: Option<Vec<u8>>,
}

fn skip_spaces(tokens: &[TokenV0], mut index: usize) -> usize {
    while matches!(tokens.get(index), Some(TokenV0::Space)) {
        index += 1;
    }
    index
}

fn push_space(out: &mut Vec<u8>) {
    match out.last().copied() {
        None | Some(b' ') | Some(NEWLINE_MARKER_V0) => {}
        _ => out.push(b' '),
    }
}

fn trim_trailing_spaces(out: &mut Vec<u8>) {
    while matches!(out.last(), Some(b' ')) {
        out.pop();
    }
}

fn push_newline(out: &mut Vec<u8>) {
    trim_trailing_spaces(out);
    if !matches!(out.last().copied(), Some(NEWLINE_MARKER_V0)) {
        out.push(NEWLINE_MARKER_V0);
    }
}

fn push_paragraph_break(out: &mut Vec<u8>) {
    trim_trailing_spaces(out);
    if out.is_empty() {
        return;
    }
    if out.ends_with(&[NEWLINE_MARKER_V0, NEWLINE_MARKER_V0]) {
        return;
    }
    if !out.ends_with(&[NEWLINE_MARKER_V0]) {
        out.push(NEWLINE_MARKER_V0);
    }
    out.push(NEWLINE_MARKER_V0);
}

fn push_page_break(out: &mut Vec<u8>) {
    trim_trailing_spaces(out);
    if !matches!(out.last().copied(), Some(PAGE_BREAK_MARKER_V0)) {
        out.push(PAGE_BREAK_MARKER_V0);
    }
}

fn is_horizontal_space_v0(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t')
}

fn trim_horizontal_space_bytes_v0(mut bytes: &[u8]) -> &[u8] {
    while matches!(bytes.first(), Some(byte) if is_horizontal_space_v0(*byte)) {
        bytes = &bytes[1..];
    }
    while matches!(bytes.last(), Some(byte) if is_horizontal_space_v0(*byte)) {
        bytes = &bytes[..bytes.len() - 1];
    }
    bytes
}

fn strip_comment_unescaped_v0(line: &[u8]) -> &[u8] {
    let mut index = 0usize;
    while index < line.len() {
        if line[index] == b'%' {
            let mut backslashes = 0usize;
            let mut cursor = index;
            while cursor > 0 && line[cursor - 1] == b'\\' {
                backslashes += 1;
                cursor -= 1;
            }
            if backslashes % 2 == 0 {
                return &line[..index];
            }
        }
        index += 1;
    }
    line
}

fn is_ascii_letter_v0(byte: u8) -> bool {
    byte.is_ascii_alphabetic()
}

fn rewrite_explicit_par_controls_v0(line: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(line.len());
    let mut index = 0usize;
    while index < line.len() {
        if line[index] != b'\\' {
            out.push(line[index]);
            index += 1;
            continue;
        }
        let control_start = index + 1;
        if control_start >= line.len() || !is_ascii_letter_v0(line[control_start]) {
            out.push(line[index]);
            index += 1;
            continue;
        }
        let mut control_end = control_start;
        while control_end < line.len() && is_ascii_letter_v0(line[control_end]) {
            control_end += 1;
        }
        let control_name = &line[control_start..control_end];
        if control_name == b"par" {
            out.extend_from_slice(b"\\carrelpar");
        } else if control_name == b"newline" {
            out.extend_from_slice(b"\\carrelnewline");
        } else {
            out.extend_from_slice(&line[index..control_end]);
        }
        index = control_end;
    }
    out
}

pub(crate) fn preprocess_typeset_minimal_source_v0(source: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(source.len() + 16);
    let mut cursor = 0usize;
    let mut in_body = false;
    let mut pending_paragraph_break = false;

    while cursor <= source.len() {
        let line_start = cursor;
        while cursor < source.len() && source[cursor] != b'\n' && source[cursor] != b'\r' {
            cursor += 1;
        }
        let line = &source[line_start..cursor];
        let had_newline = cursor < source.len();
        if had_newline {
            if source[cursor] == b'\r' && cursor + 1 < source.len() && source[cursor + 1] == b'\n' {
                cursor += 2;
            } else {
                cursor += 1;
            }
        } else {
            cursor += 1;
        }

        let line_no_comment = strip_comment_unescaped_v0(line);
        let trimmed = trim_horizontal_space_bytes_v0(line_no_comment);

        if !in_body {
            out.extend_from_slice(line);
            if had_newline {
                out.push(b'\n');
            }
            if trimmed == b"\\begin{document}" {
                in_body = true;
            }
            continue;
        }

        if trimmed == b"\\end{document}" {
            pending_paragraph_break = false;
            out.extend_from_slice(line);
            if had_newline {
                out.push(b'\n');
            }
            in_body = false;
            continue;
        }

        if trimmed.is_empty() {
            pending_paragraph_break = true;
            continue;
        }

        if pending_paragraph_break {
            out.extend_from_slice(b"\\carrelpar ");
            pending_paragraph_break = false;
        }
        let rewritten = rewrite_explicit_par_controls_v0(line);
        out.extend_from_slice(&rewritten);
        if had_newline {
            out.push(b'\n');
        }
    }

    out
}

pub(crate) fn normalize_typeset_minimal_tokens_v0(tokens: &[TokenV0]) -> Vec<TokenV0> {
    tokens.to_vec()
}

fn consume_group_bounds(tokens: &[TokenV0], index: usize) -> Option<(usize, usize, usize)> {
    let start = skip_spaces(tokens, index);
    if !matches!(tokens.get(start), Some(TokenV0::BeginGroup)) {
        return None;
    }
    let mut depth = 1usize;
    let mut cursor = start + 1;
    while let Some(token) = tokens.get(cursor) {
        match token {
            TokenV0::BeginGroup => depth += 1,
            TokenV0::EndGroup => {
                depth -= 1;
                if depth == 0 {
                    return Some((start + 1, cursor, cursor + 1));
                }
            }
            _ => {}
        }
        cursor += 1;
    }
    None
}

fn consume_simple_bracket_non_empty(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let mut cursor = skip_spaces(tokens, index);
    if !matches!(tokens.get(cursor), Some(TokenV0::Char(b'['))) {
        return Some(cursor);
    }
    cursor += 1;
    let mut saw_non_space = false;
    while let Some(token) = tokens.get(cursor) {
        match token {
            TokenV0::Char(b']') => return if saw_non_space { Some(cursor + 1) } else { None },
            TokenV0::Char(_) => saw_non_space = true,
            TokenV0::Space => {}
            _ => return None,
        }
        cursor += 1;
    }
    None
}

fn consume_documentclass_v0(tokens: &[TokenV0], index: usize) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"documentclass"
    ) {
        return None;
    }
    let mut cursor = consume_simple_bracket_non_empty(tokens, index + 1)?;
    let (group_start, group_end, next) = consume_group_bounds(tokens, cursor)?;
    let mut class_bytes = Vec::new();
    for token in &tokens[group_start..group_end] {
        match token {
            TokenV0::Char(byte) => class_bytes.push(*byte),
            TokenV0::Space => class_bytes.push(b' '),
            _ => return None,
        }
    }
    while matches!(class_bytes.first(), Some(b' ')) {
        class_bytes.remove(0);
    }
    while matches!(class_bytes.last(), Some(b' ')) {
        class_bytes.pop();
    }
    if class_bytes != b"article" {
        return None;
    }
    cursor = next;
    Some(cursor)
}

fn consume_env_name_command_v0(
    tokens: &[TokenV0],
    index: usize,
    command_name: &[u8],
) -> Option<(Vec<u8>, usize)> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(control)) if control.as_slice() == command_name
    ) {
        return None;
    }
    let (group_start, group_end, next) = consume_group_bounds(tokens, index + 1)?;
    let mut env_bytes = Vec::new();
    for token in &tokens[group_start..group_end] {
        match token {
            TokenV0::Char(byte) => env_bytes.push(*byte),
            TokenV0::Space => env_bytes.push(b' '),
            _ => return None,
        }
    }
    while matches!(env_bytes.first(), Some(b' ')) {
        env_bytes.remove(0);
    }
    while matches!(env_bytes.last(), Some(b' ')) {
        env_bytes.pop();
    }
    if env_bytes.is_empty() {
        return None;
    }
    Some((env_bytes, next))
}

fn consume_document_env_command_v0(tokens: &[TokenV0], index: usize, name: &[u8]) -> Option<usize> {
    let (env_name, next) = consume_env_name_command_v0(tokens, index, name)?;
    if env_name.as_slice() != DOCUMENT_ENV_V0 {
        return None;
    }
    Some(next)
}

fn is_supported_literal_char_v0(byte: u8) -> bool {
    if !(0x20..=0x7e).contains(&byte) {
        return false;
    }
    !matches!(
        byte,
        ITALIC_START_MARKER_V0 | ITALIC_END_MARKER_V0 | BOLD_START_MARKER_V0 | BOLD_END_MARKER_V0
    )
}

fn is_spacing_or_newline_v0(byte: u8) -> bool {
    matches!(byte, b' ' | NEWLINE_MARKER_V0)
}

fn is_punctuation_spacing_target_v0(byte: u8) -> bool {
    matches!(byte, b'.' | b',' | b';' | b':' | b'!' | b'?')
}

fn normalize_punctuation_spacing_v0(body: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(body.len());
    let mut index = 0usize;

    while index < body.len() {
        let byte = body[index];
        out.push(byte);
        index += 1;
        if !is_punctuation_spacing_target_v0(byte) {
            continue;
        }

        if index >= body.len() || !is_spacing_or_newline_v0(body[index]) {
            continue;
        }

        let mut saw_space = false;
        let mut newline_count = 0usize;
        while index < body.len() && is_spacing_or_newline_v0(body[index]) {
            match body[index] {
                b' ' => saw_space = true,
                NEWLINE_MARKER_V0 => newline_count += 1,
                _ => {}
            }
            index += 1;
        }

        if newline_count >= 2 {
            out.push(NEWLINE_MARKER_V0);
            out.push(NEWLINE_MARKER_V0);
        } else if newline_count == 1 {
            out.push(NEWLINE_MARKER_V0);
        } else if saw_space && index < body.len() && body[index] != PAGE_BREAK_MARKER_V0 {
            out.push(b' ');
        }
    }

    out
}

fn normalize_tex_double_quotes_v0(body: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(body.len());
    let mut index = 0usize;

    while index < body.len() {
        if index + 1 < body.len()
            && ((body[index] == b'`' && body[index + 1] == b'`')
                || (body[index] == b'\'' && body[index + 1] == b'\''))
        {
            out.push(b'"');
            index += 2;
            continue;
        }
        out.push(body[index]);
        index += 1;
    }

    out
}

fn normalize_tex_dashes_v0(body: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(body.len());
    let mut index = 0usize;

    while index < body.len() {
        if index + 2 < body.len()
            && body[index] == b'-'
            && body[index + 1] == b'-'
            && body[index + 2] == b'-'
        {
            out.extend_from_slice("—".as_bytes());
            index += 3;
            continue;
        }
        if index + 1 < body.len() && body[index] == b'-' && body[index + 1] == b'-' {
            out.extend_from_slice("–".as_bytes());
            index += 2;
            continue;
        }
        out.push(body[index]);
        index += 1;
    }

    out
}

fn normalize_tex_ellipsis_v0(body: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(body.len());
    let mut index = 0usize;

    while index < body.len() {
        if index + 2 < body.len()
            && body[index] == b'.'
            && body[index + 1] == b'.'
            && body[index + 2] == b'.'
        {
            out.extend_from_slice("…".as_bytes());
            index += 3;
            continue;
        }
        out.push(body[index]);
        index += 1;
    }

    out
}

fn opening_bracket_closer_v0(byte: u8) -> Option<u8> {
    match byte {
        b'(' => Some(b')'),
        b'[' => Some(b']'),
        b'{' => Some(b'}'),
        _ => None,
    }
}

fn is_closing_bracket_v0(byte: u8) -> bool {
    matches!(byte, b')' | b']' | b'}')
}

fn normalize_bracket_spacing_v0(body: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(body.len());
    let mut index = 0usize;

    while index < body.len() {
        let byte = body[index];
        if opening_bracket_closer_v0(byte).is_some() {
            out.push(byte);
            index += 1;

            let spaces_start = index;
            while index < body.len() && body[index] == b' ' {
                index += 1;
            }
            if spaces_start < index {
                if index >= body.len()
                    || body[index] == NEWLINE_MARKER_V0
                    || body[index] == PAGE_BREAK_MARKER_V0
                {
                    out.extend_from_slice(&body[spaces_start..index]);
                }
            }
            continue;
        }

        if byte == b' ' {
            let spaces_start = index;
            while index < body.len() && body[index] == b' ' {
                index += 1;
            }
            if index < body.len() && is_closing_bracket_v0(body[index]) {
                continue;
            }
            out.extend_from_slice(&body[spaces_start..index]);
            continue;
        }

        out.push(byte);
        index += 1;
    }

    out
}

fn consume_fragment_token_v0(
    tokens: &[TokenV0],
    index: usize,
    out: &mut Vec<u8>,
    allow_and: bool,
    allow_hard_break: bool,
) -> Option<usize> {
    match tokens.get(index)? {
        TokenV0::Char(byte) if allow_hard_break && *byte == PAGE_BREAK_MARKER_V0 => {
            push_page_break(out);
            Some(index + 1)
        }
        TokenV0::Char(byte) if *byte == NEWLINE_MARKER_V0 => {
            push_space(out);
            Some(index + 1)
        }
        TokenV0::Char(byte) => {
            if !is_supported_literal_char_v0(*byte) {
                return None;
            }
            out.push(*byte);
            Some(index + 1)
        }
        TokenV0::Space => {
            push_space(out);
            Some(index + 1)
        }
        TokenV0::ControlSeq(name) if allow_and && name.as_slice() == b"and" => {
            push_newline(out);
            Some(index + 1)
        }
        TokenV0::ControlSeq(name)
            if allow_hard_break
                && (name.as_slice().is_empty()
                    || name.as_slice() == HARD_LINE_BREAK_CONTROL_V0
                    || name.as_slice() == NEWLINE_ALIAS_CONTROL_V0
                    || name.as_slice() == LINEBREAK_ALIAS_CONTROL_V0
                    || name.as_slice() == CARRELNEWLINE_MARKER_CONTROL_V0) =>
        {
            push_newline(out);
            Some(index + 1)
        }
        TokenV0::ControlSeq(name)
            if allow_hard_break && name.as_slice() == PAGEBREAK_ALIAS_CONTROL_V0 =>
        {
            push_page_break(out);
            Some(index + 1)
        }
        TokenV0::ControlSeq(name) if name.as_slice() == b"protect" || name.as_slice() == b"relax" => {
            Some(index + 1)
        }
        TokenV0::ControlSeq(name) if name.as_slice() == b"emph" || name.as_slice() == b"textbf" => {
            let style_markers = if name.as_slice() == b"emph" {
                (ITALIC_START_MARKER_V0, ITALIC_END_MARKER_V0)
            } else {
                (BOLD_START_MARKER_V0, BOLD_END_MARKER_V0)
            };
            let (group_start, group_end, next) = consume_group_bounds(tokens, index + 1)?;
            out.push(style_markers.0);
            consume_fragment_range_v0(tokens, group_start, group_end, out, allow_and, allow_hard_break)?;
            out.push(style_markers.1);
            Some(next)
        }
        _ => None,
    }
}

fn consume_fragment_range_v0(
    tokens: &[TokenV0],
    start: usize,
    end: usize,
    out: &mut Vec<u8>,
    allow_and: bool,
    allow_hard_break: bool,
) -> Option<()> {
    let mut cursor = start;
    while cursor < end {
        let next = consume_fragment_token_v0(tokens, cursor, out, allow_and, allow_hard_break)?;
        if next <= cursor || next > end {
            return None;
        }
        cursor = next;
    }
    Some(())
}

fn is_heading_control_v0(name: &[u8]) -> bool {
    name == SECTION_CONTROL_V0
        || name == SUBSECTION_CONTROL_V0
        || name == SUBSUBSECTION_CONTROL_V0
        || name == PARAGRAPH_CONTROL_V0
        || name == SUBPARAGRAPH_CONTROL_V0
}

fn heading_prefix_for_control_v0(name: &[u8]) -> Option<&'static [u8]> {
    if name == SECTION_CONTROL_V0 {
        Some(SECTION_HEADING_PREFIX_MARKER_V0)
    } else if name == SUBSECTION_CONTROL_V0
        || name == SUBSUBSECTION_CONTROL_V0
        || name == PARAGRAPH_CONTROL_V0
        || name == SUBPARAGRAPH_CONTROL_V0
    {
        Some(SUBSECTION_HEADING_PREFIX_MARKER_V0)
    } else {
        None
    }
}

fn consume_heading_command_v0(tokens: &[TokenV0], index: usize, out: &mut Vec<u8>) -> Option<usize> {
    let TokenV0::ControlSeq(name) = tokens.get(index)? else {
        return None;
    };
    if !is_heading_control_v0(name.as_slice()) {
        return None;
    }
    let heading_prefix = heading_prefix_for_control_v0(name.as_slice())?;
    let (group_start, group_end, next) = consume_group_bounds(tokens, index + 1)?;
    let mut heading = Vec::new();
    consume_fragment_range_v0(tokens, group_start, group_end, &mut heading, false, true)?;
    trim_trailing_spaces(&mut heading);
    push_paragraph_break(out);
    out.extend_from_slice(heading_prefix);
    out.push(BOLD_START_MARKER_V0);
    out.extend_from_slice(&heading);
    out.push(BOLD_END_MARKER_V0);
    push_paragraph_break(out);
    Some(next)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ListKindV0 {
    Itemize,
    Enumerate,
}

fn list_item_prefix_v0(kind: ListKindV0, depth: usize, enumerate_counter: usize) -> Vec<u8> {
    let mut prefix = Vec::new();
    for _ in 0..depth {
        prefix.extend_from_slice(b"  ");
    }
    match kind {
        ListKindV0::Itemize => prefix.extend_from_slice(b"- "),
        ListKindV0::Enumerate => {
            prefix.extend_from_slice(enumerate_counter.to_string().as_bytes());
            prefix.extend_from_slice(b". ");
        }
    }
    prefix
}

fn consume_list_environment_with_depth_v0(
    tokens: &[TokenV0],
    index: usize,
    out: &mut Vec<u8>,
    depth: usize,
) -> Option<usize> {
    if depth > 1 {
        return None;
    }
    let (env_name, mut cursor) = consume_env_name_command_v0(tokens, index, BEGIN_CONTROL_V0)?;
    let kind = if env_name.as_slice() == ITEMIZE_ENV_V0 {
        ListKindV0::Itemize
    } else if env_name.as_slice() == ENUMERATE_ENV_V0 {
        ListKindV0::Enumerate
    } else {
        return None;
    };

    push_paragraph_break(out);
    let mut enumerate_counter = 1usize;

    loop {
        cursor = skip_spaces(tokens, cursor);
        match tokens.get(cursor)? {
            TokenV0::ControlSeq(name) if name.as_slice() == ITEM_CONTROL_V0 => {
                push_newline(out);
                out.extend_from_slice(&list_item_prefix_v0(kind, depth, enumerate_counter));
                if kind == ListKindV0::Enumerate {
                    enumerate_counter += 1;
                }
                cursor += 1;

                loop {
                    match tokens.get(cursor) {
                        Some(TokenV0::ControlSeq(name)) if name.as_slice() == ITEM_CONTROL_V0 => break,
                        Some(TokenV0::ControlSeq(name)) if name.as_slice() == BEGIN_CONTROL_V0 => {
                            let (nested_env, _) =
                                consume_env_name_command_v0(tokens, cursor, BEGIN_CONTROL_V0)?;
                            if nested_env.as_slice() != ITEMIZE_ENV_V0
                                && nested_env.as_slice() != ENUMERATE_ENV_V0
                            {
                                return None;
                            }
                            cursor = consume_list_environment_with_depth_v0(tokens, cursor, out, depth + 1)?;
                        }
                        Some(TokenV0::ControlSeq(name)) if name.as_slice() == END_CONTROL_V0 => {
                            let (end_env, next) =
                                consume_env_name_command_v0(tokens, cursor, END_CONTROL_V0)?;
                            if end_env != env_name {
                                return None;
                            }
                            trim_trailing_spaces(out);
                            push_paragraph_break(out);
                            return Some(next);
                        }
                        Some(_) => {
                            cursor = consume_fragment_token_v0(tokens, cursor, out, false, true)?;
                        }
                        None => return None,
                    }
                }
                trim_trailing_spaces(out);
            }
            TokenV0::ControlSeq(name) if name.as_slice() == END_CONTROL_V0 => {
                let (end_env, next) = consume_env_name_command_v0(tokens, cursor, END_CONTROL_V0)?;
                if end_env != env_name {
                    return None;
                }
                push_paragraph_break(out);
                return Some(next);
            }
            _ => return None,
        }
    }
}

fn consume_list_environment_v0(tokens: &[TokenV0], index: usize, out: &mut Vec<u8>) -> Option<usize> {
    consume_list_environment_with_depth_v0(tokens, index, out, 0)
}

fn prefix_quote_lines_v0(content: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(content.len().saturating_add(8));
    let mut at_line_start = true;
    for &byte in content {
        if matches!(byte, NEWLINE_MARKER_V0 | PAGE_BREAK_MARKER_V0) {
            out.push(byte);
            at_line_start = true;
            continue;
        }
        if at_line_start {
            out.extend_from_slice(b"> ");
            at_line_start = false;
        }
        out.push(byte);
    }
    out
}

fn prefix_center_lines_v0(content: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(content.len().saturating_add(8));
    let mut at_line_start = true;
    for &byte in content {
        if matches!(byte, NEWLINE_MARKER_V0 | PAGE_BREAK_MARKER_V0) {
            out.push(byte);
            at_line_start = true;
            continue;
        }
        if at_line_start {
            out.extend_from_slice(b"^ ");
            at_line_start = false;
        }
        out.push(byte);
    }
    out
}

fn prefix_right_lines_v0(content: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(content.len().saturating_add(8));
    let mut at_line_start = true;
    for &byte in content {
        if matches!(byte, NEWLINE_MARKER_V0 | PAGE_BREAK_MARKER_V0) {
            out.push(byte);
            at_line_start = true;
            continue;
        }
        if at_line_start {
            out.extend_from_slice(b"| ");
            at_line_start = false;
        }
        out.push(byte);
    }
    out
}

fn consume_quote_environment_v0(tokens: &[TokenV0], index: usize, out: &mut Vec<u8>) -> Option<usize> {
    let (env_name, mut cursor) = consume_env_name_command_v0(tokens, index, BEGIN_CONTROL_V0)?;
    if env_name.as_slice() != QUOTE_ENV_V0 {
        return None;
    }

    push_paragraph_break(out);
    let mut quoted = Vec::<u8>::new();
    loop {
        match tokens.get(cursor) {
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == END_CONTROL_V0 => {
                let (end_env, next) = consume_env_name_command_v0(tokens, cursor, END_CONTROL_V0)?;
                if end_env.as_slice() != QUOTE_ENV_V0 {
                    return None;
                }
                trim_trailing_spaces(&mut quoted);
                let prefixed = prefix_quote_lines_v0(&quoted);
                out.extend_from_slice(&prefixed);
                push_paragraph_break(out);
                return Some(next);
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == BEGIN_CONTROL_V0 => {
                return None;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == CARRELPAR_MARKER_CONTROL_V0 => {
                push_paragraph_break(&mut quoted);
                cursor += 1;
            }
            Some(_) => {
                cursor = consume_fragment_token_v0(tokens, cursor, &mut quoted, false, true)?;
            }
            None => return None,
        }
    }
}

fn consume_center_environment_v0(tokens: &[TokenV0], index: usize, out: &mut Vec<u8>) -> Option<usize> {
    let (env_name, mut cursor) = consume_env_name_command_v0(tokens, index, BEGIN_CONTROL_V0)?;
    if env_name.as_slice() != CENTER_ENV_V0 {
        return None;
    }

    push_paragraph_break(out);
    let mut centered = Vec::<u8>::new();
    loop {
        match tokens.get(cursor) {
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == END_CONTROL_V0 => {
                let (end_env, next) = consume_env_name_command_v0(tokens, cursor, END_CONTROL_V0)?;
                if end_env.as_slice() != CENTER_ENV_V0 {
                    return None;
                }
                trim_trailing_spaces(&mut centered);
                let prefixed = prefix_center_lines_v0(&centered);
                out.extend_from_slice(&prefixed);
                push_paragraph_break(out);
                return Some(next);
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == BEGIN_CONTROL_V0 => {
                return None;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == CARRELPAR_MARKER_CONTROL_V0 => {
                push_paragraph_break(&mut centered);
                cursor += 1;
            }
            Some(_) => {
                cursor = consume_fragment_token_v0(tokens, cursor, &mut centered, false, true)?;
            }
            None => return None,
        }
    }
}

fn consume_centerline_command_v0(tokens: &[TokenV0], index: usize, out: &mut Vec<u8>) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == CENTERLINE_CONTROL_V0
    ) {
        return None;
    }
    let (group_start, group_end, next) = consume_group_bounds(tokens, index + 1)?;
    let mut centered = Vec::new();
    consume_fragment_range_v0(tokens, group_start, group_end, &mut centered, false, true)?;
    trim_trailing_spaces(&mut centered);
    if centered.is_empty() {
        return None;
    }
    if centered.contains(&NEWLINE_MARKER_V0) || centered.contains(&PAGE_BREAK_MARKER_V0) {
        return None;
    }

    push_paragraph_break(out);
    out.extend_from_slice(b"^ ");
    out.extend_from_slice(&centered);
    push_paragraph_break(out);
    Some(next)
}

fn consume_flushright_environment_v0(
    tokens: &[TokenV0],
    index: usize,
    out: &mut Vec<u8>,
) -> Option<usize> {
    let (env_name, mut cursor) = consume_env_name_command_v0(tokens, index, BEGIN_CONTROL_V0)?;
    if env_name.as_slice() != FLUSHRIGHT_ENV_V0 {
        return None;
    }

    push_paragraph_break(out);
    let mut right_aligned = Vec::<u8>::new();
    loop {
        match tokens.get(cursor) {
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == END_CONTROL_V0 => {
                let (end_env, next) = consume_env_name_command_v0(tokens, cursor, END_CONTROL_V0)?;
                if end_env.as_slice() != FLUSHRIGHT_ENV_V0 {
                    return None;
                }
                trim_trailing_spaces(&mut right_aligned);
                let prefixed = prefix_right_lines_v0(&right_aligned);
                out.extend_from_slice(&prefixed);
                push_paragraph_break(out);
                return Some(next);
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == BEGIN_CONTROL_V0 => {
                return None;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == CARRELPAR_MARKER_CONTROL_V0 => {
                push_paragraph_break(&mut right_aligned);
                cursor += 1;
            }
            Some(_) => {
                cursor = consume_fragment_token_v0(tokens, cursor, &mut right_aligned, false, true)?;
            }
            None => return None,
        }
    }
}

fn consume_rightline_command_v0(tokens: &[TokenV0], index: usize, out: &mut Vec<u8>) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == RIGHTLINE_CONTROL_V0
    ) {
        return None;
    }
    let (group_start, group_end, next) = consume_group_bounds(tokens, index + 1)?;
    let mut right_aligned = Vec::new();
    consume_fragment_range_v0(tokens, group_start, group_end, &mut right_aligned, false, true)?;
    trim_trailing_spaces(&mut right_aligned);
    if right_aligned.is_empty() {
        return None;
    }
    if right_aligned.contains(&NEWLINE_MARKER_V0) || right_aligned.contains(&PAGE_BREAK_MARKER_V0) {
        return None;
    }

    push_paragraph_break(out);
    out.extend_from_slice(b"| ");
    out.extend_from_slice(&right_aligned);
    push_paragraph_break(out);
    Some(next)
}

fn consume_body_environment_v0(tokens: &[TokenV0], index: usize, out: &mut Vec<u8>) -> Option<usize> {
    let (env_name, _) = consume_env_name_command_v0(tokens, index, BEGIN_CONTROL_V0)?;
    if env_name.as_slice() == ITEMIZE_ENV_V0 || env_name.as_slice() == ENUMERATE_ENV_V0 {
        return consume_list_environment_v0(tokens, index, out);
    }
    if env_name.as_slice() == QUOTE_ENV_V0 {
        return consume_quote_environment_v0(tokens, index, out);
    }
    if env_name.as_slice() == CENTER_ENV_V0 {
        return consume_center_environment_v0(tokens, index, out);
    }
    if env_name.as_slice() == FLUSHRIGHT_ENV_V0 {
        return consume_flushright_environment_v0(tokens, index, out);
    }
    None
}

fn consume_meta_declaration_v0(
    tokens: &[TokenV0],
    index: usize,
    name: &[u8],
    allow_and: bool,
) -> Option<(usize, Vec<u8>)> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(control)) if control.as_slice() == name
    ) {
        return None;
    }
    let mut cursor = skip_spaces(tokens, index + 1);
    if matches!(tokens.get(cursor), Some(TokenV0::Char(b'*'))) {
        cursor = skip_spaces(tokens, cursor + 1);
    }
    let (group_start, group_end, next) = consume_group_bounds(tokens, cursor)?;
    let mut value = Vec::new();
    consume_fragment_range_v0(tokens, group_start, group_end, &mut value, allow_and, false)?;
    trim_trailing_spaces(&mut value);
    if value.is_empty() {
        return None;
    }
    Some((next, value))
}

fn emit_maketitle_block_v0(out: &mut Vec<u8>, meta: &TitleMetaV0) {
    let mut emitted = false;
    if let Some(title) = &meta.title {
        out.extend_from_slice(title);
        push_newline(out);
        emitted = true;
    }
    if let Some(author) = &meta.author {
        out.extend_from_slice(author);
        push_newline(out);
        emitted = true;
    }
    if let Some(date) = &meta.date {
        out.extend_from_slice(date);
        push_newline(out);
        emitted = true;
    }
    if emitted {
        push_newline(out);
    }
}

pub(crate) fn extract_typeset_minimal_text_body_v0(tokens: &[TokenV0]) -> Option<Vec<u8>> {
    let mut index = skip_spaces(tokens, 0);
    index = consume_documentclass_v0(tokens, index)?;

    let mut meta = TitleMetaV0::default();
    loop {
        index = skip_spaces(tokens, index);
        match tokens.get(index) {
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"title" => {
                let (next, value) = consume_meta_declaration_v0(tokens, index, b"title", false)?;
                meta.title = Some(value);
                index = next;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"author" => {
                let (next, value) = consume_meta_declaration_v0(tokens, index, b"author", true)?;
                meta.author = Some(value);
                index = next;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"date" => {
                let (next, value) = consume_meta_declaration_v0(tokens, index, b"date", false)?;
                meta.date = Some(value);
                index = next;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == BEGIN_CONTROL_V0 => {
                index = consume_document_env_command_v0(tokens, index, BEGIN_CONTROL_V0)?;
                break;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"protect" || name.as_slice() == b"relax" => {
                index += 1;
            }
            Some(TokenV0::Space) => index += 1,
            _ => return None,
        }
    }

    let mut body = Vec::<u8>::new();
    let mut pending_noindent_after_heading = false;
    loop {
        match tokens.get(index) {
            Some(TokenV0::Space) => {
                push_space(&mut body);
                index += 1;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"maketitle" => {
                emit_maketitle_block_v0(&mut body, &meta);
                pending_noindent_after_heading = false;
                index += 1;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == END_CONTROL_V0 => {
                index = consume_document_env_command_v0(tokens, index, END_CONTROL_V0)?;
                break;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == BEGIN_CONTROL_V0 => {
                pending_noindent_after_heading = false;
                index = consume_body_environment_v0(tokens, index, &mut body)?;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == CARRELPAR_MARKER_CONTROL_V0 => {
                push_paragraph_break(&mut body);
                index += 1;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == CENTERLINE_CONTROL_V0 => {
                pending_noindent_after_heading = false;
                index = consume_centerline_command_v0(tokens, index, &mut body)?;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == RIGHTLINE_CONTROL_V0 => {
                pending_noindent_after_heading = false;
                index = consume_rightline_command_v0(tokens, index, &mut body)?;
            }
            Some(TokenV0::ControlSeq(name)) if is_heading_control_v0(name.as_slice()) => {
                index = consume_heading_command_v0(tokens, index, &mut body)?;
                pending_noindent_after_heading = true;
            }
            Some(_) => {
                if pending_noindent_after_heading
                    && (body.is_empty()
                        || matches!(
                            body.last().copied(),
                            Some(NEWLINE_MARKER_V0 | PAGE_BREAK_MARKER_V0)
                        ))
                {
                    body.extend_from_slice(NOINDENT_PREFIX_MARKER_V0);
                    pending_noindent_after_heading = false;
                }
                index = consume_fragment_token_v0(tokens, index, &mut body, false, true)?;
            }
            None => return None,
        }
    }

    if tokens[index..]
        .iter()
        .any(|token| !matches!(token, TokenV0::Space))
    {
        return None;
    }

    body = normalize_punctuation_spacing_v0(&body);
    body = normalize_tex_double_quotes_v0(&body);
    body = normalize_tex_dashes_v0(&body);
    body = normalize_tex_ellipsis_v0(&body);
    body = normalize_bracket_spacing_v0(&body);
    trim_trailing_spaces(&mut body);
    while matches!(body.last().copied(), Some(NEWLINE_MARKER_V0)) {
        body.pop();
    }
    if body.is_empty() {
        return None;
    }
    Some(body)
}
