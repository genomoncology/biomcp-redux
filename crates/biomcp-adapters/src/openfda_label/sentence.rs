//! Quote the label sentence that holds a searched phrase, matched word by word as openFDA does.
use super::raw::Record;
use biodata::Text;
use biomcp_fetch::RequestError;

/// Lowercase letter-and-digit words; openFDA's tokenizer splits label text at everything else.
fn words(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
        .collect()
}

/// Sentences end at a full stop, question or exclamation mark followed by white space, so a
/// section number such as 12.1 stays inside its sentence.
fn sentences(text: &str) -> Vec<&str> {
    let mut found = Vec::new();
    let mut start = 0;
    let mut chars = text.char_indices().peekable();
    while let Some((at, c)) = chars.next() {
        if matches!(c, '.' | '?' | '!')
            && chars.peek().is_some_and(|(_, next)| next.is_whitespace())
        {
            found.push(&text[start..=at]);
            start = at + 1;
        }
    }
    found.push(&text[start..]);
    found
}

impl Record {
    /// The first sentence of a heading whose words hold the phrase's words in order, with its
    /// spacing collapsed; `None` when no single sentence holds them, as when openFDA matched the
    /// phrase across a sentence end.
    pub(super) fn sentence(
        &self,
        heading: &str,
        phrase: &str,
    ) -> Result<Option<Text>, RequestError> {
        let wanted = words(&biomcp_types::openfda_words(phrase));
        let found = self
            .parts(heading)
            .into_iter()
            .flatten()
            .flat_map(|part| sentences(part))
            .find(|sentence| {
                !wanted.is_empty()
                    && words(sentence)
                        .windows(wanted.len())
                        .any(|run| run == wanted.as_slice())
            });
        found
            .map(|sentence| {
                Text::from_source(&sentence.split_whitespace().collect::<Vec<_>>().join(" "))
            })
            .transpose()
            .map_err(|_| biomcp_fetch::bad_reply(biomcp_types::Source::OpenfdaLabel, Some(200)))
    }
}
