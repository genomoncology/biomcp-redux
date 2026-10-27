//! Encode data characters without granting them Markdown or link authority.
pub(crate) fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            ':' => out.push_str("&#58;"),
            '.' => out.push_str("&#46;"),
            '@' => out.push_str("&#64;"),
            '\n' => out.push_str("&#10;"),
            '\r' => out.push_str("&#13;"),
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
