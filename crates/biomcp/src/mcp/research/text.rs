//! A literal readable rendering keeps source markup from becoming instructions or links.
use quick_xml::{Reader, events::Event};
use serde_json::Value;
use std::io;

pub(super) fn render(value: &Value) -> io::Result<String> {
    let mut value = value.clone();
    clean(&mut value);
    let markdown = String::from_utf8(crate::view::markdown(&value)?).map_err(io::Error::other)?;
    let decoded = entities(&markdown);
    let mut text = String::new();
    let mut chars = decoded.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\\' && chars.peek().is_some_and(|c| "\\*_`|[]#!~".contains(*c)) {
            text.push(chars.next().unwrap_or_default());
        } else {
            text.push(ch);
        }
    }
    // A fence longer than any source or generated fence keeps the entire view inert.
    let width = text.split(|ch| ch != '`').map(str::len).max().unwrap_or(0) + 1;
    let fence = "`".repeat(width.max(3));
    Ok(format!("{fence}text\n{}\n{fence}\n", text.trim_end()))
}

fn clean(value: &mut Value) {
    match value {
        Value::String(text) => *text = source(text),
        Value::Array(items) => items.iter_mut().for_each(clean),
        Value::Object(fields) => fields.values_mut().for_each(clean),
        _ => {}
    }
}

pub(super) fn source(text: &str) -> String {
    let decoded = entities(text);
    let mut out = String::new();
    let mut remaining = decoded.as_str();
    while let Some(start) = remaining.find('<') {
        let (prefix, tail) = remaining.split_at(start);
        out.push_str(prefix);
        remaining = tail;
        let name = remaining
            .strip_prefix("</")
            .or_else(|| remaining.strip_prefix('<'))
            .unwrap_or(remaining);
        if name.starts_with(|ch: char| ch.is_ascii_alphabetic() || ch == '!') {
            let mut reader = Reader::from_str(remaining);
            reader.config_mut().check_end_names = false;
            reader.config_mut().allow_unmatched_ends = true;
            let marker = match reader.read_event() {
                Ok(Event::Start(tag)) if tag.name().as_ref().eq_ignore_ascii_case("sup") => {
                    Some("^(")
                }
                Ok(Event::Start(tag)) if tag.name().as_ref().eq_ignore_ascii_case("sub") => {
                    Some("_(")
                }
                Ok(Event::End(tag))
                    if ["sup", "sub"]
                        .iter()
                        .any(|name| tag.name().as_ref().eq_ignore_ascii_case(name)) =>
                {
                    Some(")")
                }
                Ok(Event::Start(tag) | Event::Empty(tag))
                    if tag.name().as_ref().eq_ignore_ascii_case("br") =>
                {
                    Some("\n")
                }
                Ok(Event::Comment(comment)) => {
                    out.push_str(comment.as_ref());
                    Some("")
                }
                Ok(Event::Start(_) | Event::End(_) | Event::Empty(_)) => Some(""),
                _ => None,
            };
            if let Some(marker) = marker {
                out.push_str(marker);
                remaining = remaining.split_at(reader.buffer_position() as usize).1;
                continue;
            }
        }
        out.push('<');
        remaining = remaining.split_at(1).1;
    }
    out.push_str(remaining);
    out
}

fn entities(text: &str) -> String {
    let mut out = String::new();
    let mut remaining = text;
    while let Some(start) = remaining.find('&') {
        let (prefix, tail) = remaining.split_at(start);
        out.push_str(prefix);
        remaining = tail;
        if let Some(end) = remaining
            .as_bytes()
            .iter()
            .take(64)
            .position(|ch| *ch == b';')
        {
            let (reference, tail) = remaining.split_at(end + 1);
            if let Ok(decoded) = quick_xml::escape::unescape(reference) {
                out.push_str(&decoded);
                remaining = tail;
                continue;
            }
        }
        out.push('&');
        remaining = remaining.split_at(1).1;
    }
    out.push_str(remaining);
    out
}
