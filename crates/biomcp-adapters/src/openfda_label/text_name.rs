//! Name a label that lacks harmonized names only where its own printed text marks the name's end.
use super::raw::Record;
use biodata::Text;
use biomcp_fetch::RequestError;

/// Printed signs that end a product name: a registered or trademark sign, or the opening
/// parenthesis before the generic name, as in `IBRANCE ® (palbociclib)` or `Itovebi™ (inavolisib)`.
const ENDS: [char; 3] = ['®', '™', '('];
/// Punctuation that never belongs to a matched word, such as the apostrophe in `Infants'`.
const LOOSE: [char; 13] = [
    '™', '®', ',', ';', ':', '.', '\'', '’', '"', '(', ')', '[', ']',
];

impl Record {
    /// The longest run of leading product-data words, no word twice, that the display panel or
    /// indications section prints whole, in any letter case, followed by an end sign or the end of
    /// that text. Any other stop gives no name, because a name cut there can name another product.
    pub(super) fn text_name(&self) -> Result<Option<Text>, RequestError> {
        let Some(product) = self.product_name().0 else {
            return Ok(None);
        };
        let printed: Vec<Vec<Token>> = [
            "package_label_principal_display_panel",
            "indications_and_usage",
        ]
        .into_iter()
        .filter_map(|heading| self.parts(heading))
        .flatten()
        .map(|part| tokens(part))
        .collect();
        let raw: Vec<&str> = product.as_str().split_whitespace().collect();
        let mut run: Vec<String> = Vec::new();
        let mut shown = 0;
        for word in &raw {
            let word = clean(word);
            if word.is_empty() || run.contains(&word) {
                break;
            }
            run.push(word);
            let (found, ended) = find(&printed, &run);
            if !found {
                break;
            }
            if ended {
                shown = run.len();
            }
        }
        if shown == 0 {
            return Ok(None);
        }
        let name: Vec<&str> = raw
            .iter()
            .take(shown)
            .map(|word| word.trim_matches([',', ';', ':', '™', '®']))
            .collect();
        // Leading words of admitted text always admit; a failure is a broken reply, not no name.
        crate::native_text::admit(&name.join(" "))
            .map(Some)
            .map_err(|_| biomcp_fetch::bad_reply(biomcp_types::Source::OpenfdaLabel, Some(200)))
    }
}

#[derive(PartialEq)]
enum Token {
    Word(String),
    End,
}

/// Whether any printed text holds the run, and whether one such place ends with an end sign or
/// the end of its text.
fn find(printed: &[Vec<Token>], run: &[String]) -> (bool, bool) {
    let mut found = false;
    for text in printed {
        for (at, window) in text.windows(run.len()).enumerate() {
            let same = window
                .iter()
                .zip(run)
                .all(|(token, word)| matches!(token, Token::Word(w) if w == word));
            if same {
                found = true;
                if text
                    .get(at + run.len())
                    .is_none_or(|next| *next == Token::End)
                {
                    return (true, true);
                }
            }
        }
    }
    (found, false)
}

/// Printed words in lower case without loose punctuation, with each end sign its own token.
fn tokens(text: &str) -> Vec<Token> {
    let mut out = Vec::new();
    for piece in text.split_whitespace() {
        let mut word = String::new();
        for c in piece.chars() {
            if ENDS.contains(&c) {
                push(&mut out, &mut word);
                out.push(Token::End);
            } else {
                word.push(c);
            }
        }
        push(&mut out, &mut word);
    }
    out
}

fn push(out: &mut Vec<Token>, word: &mut String) {
    let cleaned = clean(word);
    if !cleaned.is_empty() {
        out.push(Token::Word(cleaned));
    }
    word.clear();
}

fn clean(word: &str) -> String {
    word.trim_matches(LOOSE).to_lowercase()
}
