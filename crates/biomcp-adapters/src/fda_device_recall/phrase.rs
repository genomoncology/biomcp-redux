//! Device names as one quoted openFDA phrase, with the characters openFDA refuses spaced.
use biodata::Text;
use biomcp_types::DiagnosticSearch;
/// openFDA answers HTTP 400 to these characters even inside a quoted phrase (live, 2026-11-13).
/// Its tokenizer splits words at them, so a space finds the same words.
const REFUSED: &[char] = &['!', '#', '$', ';', '?', '^', '|', '~', '`'];
/// openFDA also answers HTTP 400 to every character outside ASCII (live, 2026-11-15); the query
/// refuses letters, so what remains here is a symbol such as ® or ™, spaced like punctuation:
/// "Alaris® pump" counts the same 49 recall records as "Alaris pump".
fn spaced(c: char) -> bool {
    REFUSED.contains(&c) || !c.is_ascii()
}
fn words(value: &Text) -> String {
    let words: String = value
        .as_str()
        .chars()
        .map(|c| if spaced(c) { ' ' } else { c })
        .collect();
    words.split_whitespace().collect::<Vec<_>>().join(" ")
}
/// Text as one quoted openFDA phrase, with refused characters spaced: the device recall name
/// (product_description) and the PMA intended-use phrase (ao_statement).
pub(crate) fn phrase(value: &Text) -> String {
    super::literal(&words(value))
}
/// The phrase searched for a device recall name whose refused characters became spaces.
pub fn spaced_name(q: &DiagnosticSearch) -> Option<String> {
    q.name()
        .filter(|name| name.as_str().chars().any(spaced))
        .map(words)
}
