//! Count the codes a `/ToUnicode` map covers before lopdf reads it, and rebuild a map lopdf
//! refused, both from one tokenizer that follows the PDF grammar (1068, 1069 review).
//!
//! lopdf builds a table entry for every code a map covers, so the count must never be lower
//! than what lopdf would read. lopdf's map grammar is strict: it reads code ranges and mappings
//! only inside `begincodespacerange`, `beginbfchar` and `beginbfrange` sections, and fails the
//! whole map on anything else there. The tokenizer here reads the PDF lexical grammar exactly:
//! a comment runs from a `%` outside a string to the end of the line; a literal string nests
//! parentheses and honors backslash escapes; a hexadecimal string holds only hex digits and
//! white space; `<<` and `>>` are dictionary marks. Inside a section only hexadecimal strings
//! and, in a bfrange, one level of array of them may appear. Anything the tokenizer cannot read
//! with certainty, or a section that does not close, makes the map unreadable, so it is
//! removed, never skipped past. Where lopdf is more lenient than this, such as a comment inside
//! a hexadecimal target, the map is refused and lopdf never sees it.

/// Section keywords: code space, single codes, code ranges.
const SPACE: (&str, &str) = ("begincodespacerange", "endcodespacerange");
const CHARS: (&str, &str) = ("beginbfchar", "endbfchar");
const RANGES: (&str, &str) = ("beginbfrange", "endbfrange");
const HEADER: &str =
    "/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n/CMapName /Rebuilt def\n";
const FOOTER: &str = "endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n";

/// One token of a map.
#[derive(Debug, PartialEq, Eq)]
enum Token {
    /// A hexadecimal string's digits, white space removed.
    Hex(String),
    Open,
    Close,
    /// Any other token: a name, number, operator, literal string or dictionary mark.
    Word(String),
}

fn white(c: char) -> bool {
    matches!(c, '\0' | '\t' | '\n' | '\x0c' | '\r' | ' ')
}

fn delimiter(c: char) -> bool {
    matches!(c, '(' | ')' | '<' | '>' | '[' | ']' | '{' | '}' | '/' | '%')
}

/// The map's tokens; none when one cannot be read with certainty.
fn tokens(text: &str) -> Option<Vec<Token>> {
    let mut out = Vec::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            c if white(c) => {}
            '%' => while chars.next_if(|c| *c != '\n' && *c != '\r').is_some() {},
            '(' => {
                let mut depth = 1usize;
                while depth > 0 {
                    match chars.next()? {
                        '\\' => {
                            chars.next()?;
                        }
                        '(' => depth += 1,
                        ')' => depth -= 1,
                        _ => {}
                    }
                }
                out.push(Token::Word("()".into()));
            }
            '<' if chars.next_if_eq(&'<').is_some() => out.push(Token::Word("<<".into())),
            '>' if chars.next_if_eq(&'>').is_some() => out.push(Token::Word(">>".into())),
            '<' => {
                let mut digits = String::new();
                loop {
                    match chars.next()? {
                        '>' => break,
                        c if c.is_ascii_hexdigit() => digits.push(c.to_ascii_uppercase()),
                        c if white(c) => {}
                        _ => return None,
                    }
                }
                out.push(Token::Hex(digits));
            }
            '[' => out.push(Token::Open),
            ']' => out.push(Token::Close),
            ')' | '>' => return None,
            '{' | '}' => out.push(Token::Word(c.into())),
            c => {
                let mut word = String::from(c);
                while let Some(next) = chars.next_if(|c| !white(*c) && !delimiter(*c)) {
                    word.push(next);
                }
                out.push(Token::Word(word));
            }
        }
    }
    Some(out)
}

/// One section's entries: each a run of hex strings, an array a nested list.
#[derive(Debug)]
enum Entry {
    Hex(String),
    Array(Vec<String>),
}

/// The map's sections in order, as (keywords, entries); none when one is not regular.
fn sections(text: &str) -> Option<Vec<((&'static str, &'static str), Vec<Entry>)>> {
    let mut found = Vec::new();
    let mut tokens = tokens(text)?.into_iter();
    while let Some(token) = tokens.next() {
        let Token::Word(word) = token else { continue };
        let ends = [SPACE, CHARS, RANGES].iter().any(|(_, end)| *end == word);
        let Some(&keys) = [SPACE, CHARS, RANGES]
            .iter()
            .find(|(begin, _)| *begin == word)
        else {
            // A section's end with no beginning is not regular.
            if ends {
                return None;
            }
            continue;
        };
        let mut entries = Vec::new();
        loop {
            match tokens.next()? {
                Token::Hex(hex) => entries.push(Entry::Hex(hex)),
                Token::Open if keys == RANGES => {
                    let mut items = Vec::new();
                    loop {
                        match tokens.next()? {
                            Token::Hex(hex) => items.push(hex),
                            Token::Close => break,
                            _ => return None,
                        }
                    }
                    entries.push(Entry::Array(items));
                }
                Token::Word(end) if end == keys.1 => break,
                _ => return None,
            }
        }
        found.push((keys, entries));
    }
    Some(found)
}

/// A hexadecimal code's value; codes longer than 8 digits saturate, so they count as too many.
fn value(hex: &str) -> u64 {
    if hex.len() > 8 {
        return u64::MAX;
    }
    hex.chars()
        .filter_map(|c| c.to_digit(16))
        .fold(0, |value, digit| value * 16 + u64::from(digit))
}

/// How many codes the map's sections cover: each bfchar entry once, each bfrange every code
/// it spans; none when the map cannot be read with certainty, so it is never read.
pub(super) fn covered(text: &str) -> Option<u64> {
    let mut codes = 0u64;
    for (keys, entries) in sections(text)? {
        let width = if keys == RANGES { 3 } else { 2 };
        if !entries.len().is_multiple_of(width) {
            return None;
        }
        for entry in entries.chunks(width) {
            let count = match entry {
                [Entry::Hex(low), Entry::Hex(high), _] if keys == RANGES => {
                    value(high).checked_sub(value(low))?.saturating_add(1)
                }
                [Entry::Hex(_), Entry::Hex(_)] if keys == CHARS => 1,
                [Entry::Hex(_), Entry::Hex(_)] => 0,
                _ => return None,
            };
            codes = codes.saturating_add(count);
        }
    }
    Some(codes)
}

/// The map's sections, in their order, rewritten without comments under a header lopdf
/// accepts; none when the map cannot be read with certainty, holds no section, or the copy
/// covers more than `most` codes as counted again here.
pub(super) fn rebuilt(text: &str, most: u64) -> Option<String> {
    let sections = sections(text)?;
    if sections.is_empty() {
        return None;
    }
    let mut out = String::from(HEADER);
    for ((begin, end), entries) in &sections {
        let width = if *begin == RANGES.0 { 3 } else { 2 };
        out.push_str(&format!("{} {begin}\n", entries.len() / width));
        for line in entries.chunks(width) {
            let parts: Vec<String> = line
                .iter()
                .map(|entry| match entry {
                    Entry::Hex(hex) => format!("<{hex}>"),
                    Entry::Array(items) => {
                        let items: Vec<String> = items.iter().map(|h| format!("<{h}>")).collect();
                        format!("[{}]", items.join(" "))
                    }
                })
                .collect();
            out.push_str(&parts.join(" "));
            out.push('\n');
        }
        out.push_str(end);
        out.push('\n');
    }
    out.push_str(FOOTER);
    covered(&out)
        .is_some_and(|codes| codes <= most)
        .then_some(out)
}
