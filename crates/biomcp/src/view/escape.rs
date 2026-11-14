//! Encode data characters without granting them Markdown or link authority.

/// Source text: Markdown and HTML stay literal, and GFM extended autolinks cannot form.
pub(crate) fn escape(text: &str) -> String {
    write(text, true)
}
/// BioMCP's own help text: Markdown and HTML stay literal; its addresses may link.
pub(crate) fn product(text: &str) -> String {
    write(text, false)
}
// GFM extended autolinks start at `www.` or `://` while inline parsing, so a
// backslash escape stops them. Renderers find email autolinks after joining
// adjacent text, so an escape cannot split one; the `@` after a character that
// may end a GFM email local part (or a `mailto:`/`xmpp:` scheme) becomes a code span.
fn write(text: &str, source: bool) -> String {
    let mut out = String::with_capacity(text.len());
    for (at, ch) in text.char_indices() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            ':' if source && text.get(at..).is_some_and(|rest| rest.starts_with("://")) => {
                out.push_str("\\:");
            }
            '.' if source
                && text
                    .get(at.saturating_sub(3)..at)
                    .is_some_and(|w| w.eq_ignore_ascii_case("www")) =>
            {
                out.push_str("\\.");
            }
            '@' if source
                && text
                    .get(..at)
                    .and_then(|before| before.chars().next_back())
                    .is_some_and(|c| c.is_ascii_alphanumeric() || ".+-_:".contains(c)) =>
            {
                out.push_str("`@`");
            }
            '\n' => out.push_str("&#10;"),
            '\r' => out.push_str("&#13;"),
            ch if control(ch) => out.push_str(&visible(ch)),
            '\\' | '*' | '_' | '`' | '|' | '[' | ']' | '#' | '!' | '~' => {
                out.push('\\');
                out.push(ch);
            }
            _ => out.push(ch),
        }
    }
    out
}
/// Only the product registry's source terms URL may use this route.
pub(crate) fn source_link(text: &str) -> String {
    format!("<{text}>")
}

pub(super) fn control(ch: char) -> bool {
    ch.is_control()
        || matches!(ch, '\u{061c}' | '\u{200e}' | '\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}')
}
pub(super) fn visible(ch: char) -> String {
    format!("\\u{{{:04x}}}", u32::from(ch))
}
