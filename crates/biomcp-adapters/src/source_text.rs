//! Undo the two encodings some sources leave in their values. Each caller names a
//! source known to carry the encoding; elsewhere a backslash or `&` is plain text.
use std::borrow::Cow;

/// Decode HTML character references in one pass, so `&amp;lt;` reads `&lt;`.
///
/// `&nbsp;` becomes an ordinary space with the same meaning. An unknown or invalid
/// reference stays as written, because no character is guessed.
pub(crate) fn decode_entities(text: &str) -> Cow<'_, str> {
    if !text.contains('&') {
        return Cow::Borrowed(text);
    }
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some((before, after)) = rest.split_once('&') {
        out.push_str(before);
        let decoded = after
            .split_once(';')
            .and_then(|(name, tail)| Some((entity(name)?, tail)));
        if let Some((decoded, tail)) = decoded {
            out.push(decoded);
            rest = tail;
        } else {
            out.push('&');
            rest = after;
        }
    }
    out.push_str(rest);
    Cow::Owned(out)
}

fn entity(name: &str) -> Option<char> {
    // An unparsable reference returns None, so the caller keeps its text as written.
    let number = |digits: &str, radix| match u32::from_str_radix(digits, radix) {
        Ok(code) => Some(code),
        Err(_) => None,
    };
    match name {
        "nbsp" => Some(' '),
        "lt" => Some('<'),
        "gt" => Some('>'),
        "amp" => Some('&'),
        "quot" => Some('"'),
        "apos" => Some('\''),
        _ => name
            .strip_prefix("#x")
            .or_else(|| name.strip_prefix("#X"))
            .map_or_else(
                || name.strip_prefix('#').and_then(|d| number(d, 10)),
                |d| number(d, 16),
            )
            .filter(|code| *code != 0)
            .and_then(char::from_u32),
    }
}

/// Remove Markdown backslash escapes: a backslash before ASCII punctuation only.
///
/// Any other backslash is literal text, as in Markdown itself.
pub(crate) fn unescape_markdown(text: &str) -> Cow<'_, str> {
    if !text.contains('\\') {
        return Cow::Borrowed(text);
    }
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        match chars.peek() {
            Some(next) if ch == '\\' && next.is_ascii_punctuation() => {
                out.push(*next);
                chars.next();
            }
            _ => out.push(ch),
        }
    }
    Cow::Owned(out)
}
