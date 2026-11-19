//! Name a label that lacks harmonized names only where its own printed text marks the name's end.
use super::raw::Record;
use biodata::Text;
use biomcp_fetch::RequestError;

/// Registered and trademark signs, allowed directly before a name's end.
const MARKS: [char; 2] = ['®', '™'];
/// Punctuation that never belongs to a matched word, such as the apostrophe in `Infants'`.
const LOOSE: [char; 13] = [
    '™', '®', ',', ';', ':', '.', '\'', '’', '"', '(', ')', '[', ']',
];

impl Record {
    /// The longest run of leading product-data words, no word twice, that the display panel or
    /// indications section prints whole, in any letter case, ended by an opening parenthesis or the
    /// end of that text, with at most one ® or ™ directly before that end, as in
    /// `IBRANCE ® (palbociclib)` or `Itovebi™ (inavolisib)`. Any other stop gives no name, because a
    /// name cut there can be a brand alone or another product, as in `Vicks® VapoRub`.
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
    Mark,
    Open,
}

/// Whether the text ends the name at this position: an opening parenthesis or the text's end,
/// after at most one mark.
fn ends(text: &[Token], at: usize) -> bool {
    let at = if text.get(at) == Some(&Token::Mark) {
        at + 1
    } else {
        at
    };
    text.get(at).is_none_or(|next| *next == Token::Open)
}

/// Whether any printed text holds the run, and whether one such place ends the name.
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
                if ends(text, at + run.len()) {
                    return (true, true);
                }
            }
        }
    }
    (found, false)
}

/// Printed words in lower case without loose punctuation, with each mark and opening parenthesis
/// its own token.
fn tokens(text: &str) -> Vec<Token> {
    let mut out = Vec::new();
    for piece in text.split_whitespace() {
        let mut word = String::new();
        for c in piece.chars() {
            if MARKS.contains(&c) || c == '(' {
                push(&mut out, &mut word);
                out.push(if c == '(' { Token::Open } else { Token::Mark });
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
