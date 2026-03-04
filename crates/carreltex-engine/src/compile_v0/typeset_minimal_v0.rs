use crate::tex::tokenize_v0::TokenV0;

const NEWLINE_MARKER_V0: u8 = 0x0a;
const PAGE_BREAK_MARKER_V0: u8 = 0x0c;
const CARRELPAR_MARKER_CONTROL_V0: &[u8] = b"carrelpar";
const CARRELNEWLINE_MARKER_CONTROL_V0: &[u8] = b"carrelnewline";
const HARD_LINE_BREAK_CONTROL_V0: &[u8] = b"\\";
const NEWLINE_ALIAS_CONTROL_V0: &[u8] = b"newline";
const LINEBREAK_ALIAS_CONTROL_V0: &[u8] = b"linebreak";
const PAGEBREAK_ALIAS_CONTROL_V0: &[u8] = b"pagebreak";
const ITALIC_START_MARKER_V0: u8 = b'[';
const ITALIC_END_MARKER_V0: u8 = b']';
const BOLD_START_MARKER_V0: u8 = b'{';
const BOLD_END_MARKER_V0: u8 = b'}';

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

fn consume_document_env_command_v0(tokens: &[TokenV0], index: usize, name: &[u8]) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(control)) if control.as_slice() == name
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
    if env_bytes != b"document" {
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
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"begin" => {
                index = consume_document_env_command_v0(tokens, index, b"begin")?;
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
    loop {
        match tokens.get(index) {
            Some(TokenV0::Space) => {
                push_space(&mut body);
                index += 1;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"maketitle" => {
                emit_maketitle_block_v0(&mut body, &meta);
                index += 1;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"end" => {
                index = consume_document_env_command_v0(tokens, index, b"end")?;
                break;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == CARRELPAR_MARKER_CONTROL_V0 => {
                push_paragraph_break(&mut body);
                index += 1;
            }
            Some(_) => {
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
