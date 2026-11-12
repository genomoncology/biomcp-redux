//! Other spellings of a typed protein change or exon deletion/insertion, written as registry
//! search expressions that later pages repeat unchanged.
use super::TrialSearch;
use crate::Text;
use crate::query::ProteinQuery;

/// Spellings searched for one typed variant, each list in a stable order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VariantSpellings {
    /// The same change written other ways, including the typed spelling's own form.
    pub spellings: Vec<String>,
    /// Codon forms, matching trials written for any change at this position; only with a gene.
    pub codon: Vec<String>,
    /// Codon forms left out because no gene was given.
    pub codon_without_gene: Vec<String>,
}

impl VariantSpellings {
    /// Spellings for a missense protein change or an exon deletion or insertion; others have none.
    pub fn of(typed: &str, gene: bool) -> Option<Self> {
        let typed = typed.trim();
        if let Ok(change) = typed.parse::<ProteinQuery>()
            && change.is_missense()
        {
            let codon = sorted(change.codon_forms().into());
            let (codon, codon_without_gene) = if gene {
                (codon, Vec::new())
            } else {
                (Vec::new(), codon)
            };
            return Some(Self {
                spellings: sorted(change.forms().into()),
                codon,
                codon_without_gene,
            });
        }
        exon(typed).map(|spellings| Self {
            spellings: sorted(spellings),
            codon: Vec::new(),
            codon_without_gene: Vec::new(),
        })
    }

    /// The criteria alternatives: every spelling and codon form, and the typed text.
    pub fn criteria_terms(&self, typed: &str) -> String {
        group(&self.alternatives(typed))
    }

    fn alternatives(&self, typed: &str) -> Vec<String> {
        let mut all: Vec<String> = self.spellings.iter().chain(&self.codon).cloned().collect();
        let typed = typed.trim();
        if !all.iter().any(|value| value.eq_ignore_ascii_case(typed)) {
            all.push(typed.to_owned());
        }
        sorted(all)
    }
}

/// General text or eligibility criteria, keeping the typed words as one alternative so the
/// widened search never finds fewer trials than the typed text alone.
pub fn variant_terms(typed: &str, spellings: Option<&VariantSpellings>) -> String {
    let typed = typed.trim();
    let alternatives = spellings.map_or_else(|| vec![typed.to_owned()], |s| s.alternatives(typed));
    let quoted = group(&alternatives);
    let general = alternatives
        .iter()
        .map(|value| quote(value))
        .collect::<Vec<_>>()
        .join(" OR ");
    format!("(({typed}) OR {general} OR AREA[EligibilityCriteria]{quoted})")
}

/// Whether criteria text is a group this module wrote, `("a" OR "b")`, sent unquoted.
pub fn is_criteria_group(value: &str) -> bool {
    value
        .strip_prefix('(')
        .and_then(|inner| inner.strip_suffix(')'))
        .and_then(|inner| {
            let mut rest = inner;
            let mut count = 0;
            loop {
                let (_, after) = take_quoted(rest)?;
                count += 1;
                if after.is_empty() {
                    return Some(count);
                }
                rest = after.strip_prefix(" OR ")?;
            }
        })
        .is_some()
}

/// The protein-change spellings and codon forms quoted in a variant expression, when it has both.
pub fn codon_split(expression: &str) -> Option<(Vec<String>, Vec<String>)> {
    let mut exact = Vec::new();
    let mut codon = Vec::new();
    let mut rest = expression;
    while let Some(start) = rest.find('"') {
        let (value, after) = take_quoted(&rest[start..])?;
        if value
            .parse::<ProteinQuery>()
            .is_ok_and(|change| change.is_missense())
        {
            exact.push(value);
        } else if is_codon(&value) {
            codon.push(value);
        }
        rest = after;
    }
    for values in [&mut exact, &mut codon] {
        values.sort();
        values.dedup();
    }
    (!exact.is_empty() && !codon.is_empty()).then_some((exact, codon))
}

impl TrialSearch {
    /// Substitute the variant and criteria terms the registry is sent.
    pub fn with_variant_terms(&self, variant: Option<Text>, criteria: Option<Text>) -> Self {
        let mut next = self.clone();
        next.0.variant = variant;
        next.0.criteria = criteria;
        next
    }
}

fn is_codon(value: &str) -> bool {
    let value = value.strip_suffix('X').unwrap_or(value);
    let mut chars = value.chars();
    chars.next().is_some_and(|c| c.is_ascii_uppercase())
        && value.len() > 1
        && value[1..].bytes().all(|b| b.is_ascii_digit())
        && !value[1..].starts_with('0')
}

/// One quoted value and the text after it; quotes and backslashes inside are escaped.
fn take_quoted(text: &str) -> Option<(String, &str)> {
    let mut chars = text.strip_prefix('"')?.char_indices();
    let mut value = String::new();
    while let Some((at, c)) = chars.next() {
        match c {
            '\\' => value.push(chars.next()?.1),
            '"' => return Some((value, &text[at + 2..])),
            other => value.push(other),
        }
    }
    None
}

fn quote(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

fn group(values: &[String]) -> String {
    let quoted: Vec<_> = values.iter().map(|value| quote(value)).collect();
    format!("({})", quoted.join(" OR "))
}

/// A registry reply is cached by its request, so the order must not depend on the typed spelling.
fn sorted(mut values: Vec<String>) -> Vec<String> {
    values.sort_by_key(|value| (value.to_ascii_lowercase(), value.clone()));
    values.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    values
}

/// The fixed spellings of "exon N deletion" or "exon N insertion", from any of them.
fn exon(typed: &str) -> Option<Vec<String>> {
    let words = typed.to_ascii_lowercase();
    let words: Vec<_> = words.split_whitespace().collect();
    let (number, kind) = match words.as_slice() {
        ["exon", number, kind] => (*number, *kind),
        [single] => compact(single)?,
        _ => return None,
    };
    let kind = match kind {
        "deletion" | "deletions" | "del" => ["deletion", "del"],
        "insertion" | "insertions" | "ins" => ["insertion", "ins"],
        _ => return None,
    };
    if number.is_empty()
        || number.len() > 3
        || number.starts_with('0')
        || !number.bytes().all(|b| b.is_ascii_digit())
    {
        return None;
    }
    let [long, short] = kind;
    Some(vec![
        format!("exon {number} {long}"),
        format!("exon {number} {long}s"),
        format!("exon {number} {short}"),
        format!("ex{number}{short}"),
        format!("{number}{short}"),
        format!("{short}{number}"),
    ])
}

/// `ex19del`, `19del` or `del19`, as number and kind.
fn compact(word: &str) -> Option<(&str, &str)> {
    let word = word.strip_prefix("ex").unwrap_or(word);
    for kind in ["del", "ins"] {
        if let Some(number) = word.strip_suffix(kind) {
            return Some((number, kind));
        }
        if let Some(number) = word.strip_prefix(kind) {
            return Some((number, kind));
        }
    }
    None
}
