//! Which stated names one provider searches, and why each other name was left out.
use biodata::Text;

/// Most official names one search keeps past the name limit, so one OR group stays bounded.
const OFFICIAL_NAME_CEILING: usize = 10;

/// Why the searching provider cannot take a name, or `None` when it can.
pub(crate) type Fit = fn(&str) -> Option<&'static str>;

/// Every name fits a provider that takes any literal text.
pub(crate) fn any(_: &str) -> Option<&'static str> {
    None
}

/// openFDA refuses a whole query holding non-ASCII text or an entity such as `Eliquis&reg;`,
/// and ChEMBL's "X component of Y" and bracketed product names are not names of the drug alone.
pub(crate) fn openfda(name: &str) -> Option<&'static str> {
    let lowered = name.to_ascii_lowercase();
    if !name.is_ascii() {
        Some("non-ascii")
    } else if name.contains(['&', '<', '>']) {
        Some("markup")
    } else if lowered.contains(" component of ") {
        Some("component of another product")
    } else if name.contains(['(', ')', '[', ']']) {
        Some("bracketed")
    } else if lowered.split_whitespace().any(|word| word == "or") {
        Some("boolean")
    } else {
        None
    }
}

/// Labels a source states only in the context of one paper, such as "COMPOUND 20" or
/// "compound 20 [PMID: 32250617]", name no drug on their own and match unrelated records in
/// every provider, so no widened search takes them. Development codes such as "MRTX849" or
/// "AMG 510" stay.
fn context_only(name: &str) -> Option<&'static str> {
    let lowered = name.to_ascii_lowercase();
    if lowered.contains("pmid") || name.contains(['[', ']']) {
        return Some("citation label");
    }
    let mut words = lowered.split([' ', '-', '#']).filter(|word| !word.is_empty());
    match (words.next(), words.next(), words.next()) {
        (Some(word), Some(number), None)
            if PLACEHOLDER_WORDS.contains(&word)
                && number.starts_with(|c: char| c.is_ascii_digit()) =>
        {
            Some("placeholder label")
        }
        _ => None,
    }
}

/// Words a paper uses to number its own molecules, as in "compound 20" or "example 3".
const PLACEHOLDER_WORDS: &[&str] = &["compound", "cpd", "example", "analog", "analogue"];

fn lowered(name: &Text) -> String {
    name.as_str().to_lowercase()
}

/// The names one provider will search, and how many the record stated beyond them.
pub(crate) struct Capped {
    pub(crate) names: Vec<Text>,
    left_out: usize,
    /// Names left out because they are context-only labels or the provider cannot take them,
    /// each with its reason.
    unfit: Vec<(Text, &'static str)>,
    /// Official names past [`OFFICIAL_NAME_CEILING`], named in the notice.
    official_left_out: Vec<Text>,
}

impl Capped {
    /// The names left out, grouped by reason in first-seen order, then the counts past each
    /// limit, for a compact notice.
    pub(crate) fn left_out(&self) -> Vec<String> {
        let mut groups: Vec<(&str, Vec<&str>)> = Vec::new();
        for (name, reason) in &self.unfit {
            match groups.iter_mut().find(|(seen, _)| seen == reason) {
                Some((_, names)) => names.push(name.as_str()),
                None => groups.push((reason, vec![name.as_str()])),
            }
        }
        let mut out: Vec<String> = groups
            .into_iter()
            .map(|(reason, names)| format!("skipped {reason}: {}", names.join(", ")))
            .collect();
        if self.left_out > 0 {
            out.push(format!("{} past drug_search_name_limit", self.left_out));
        }
        if !self.official_left_out.is_empty() {
            out.push(format!(
                "{} official past {OFFICIAL_NAME_CEILING}",
                self.official_left_out.len()
            ));
        }
        out
    }
    pub(crate) fn message(&self, source: biomcp_types::Source, record: &str) -> String {
        let total = self.names.len() + self.left_out + self.unfit.len();
        let mut message = format!("Searched the names stated by {source} record {record}.");
        if !self.unfit.is_empty() {
            let names: Vec<_> = self
                .unfit
                .iter()
                .map(|(name, reason)| format!("{:?} ({reason})", name.as_str()))
                .collect();
            message.push_str(&format!(
                " Left out {} of {total} names: {}.",
                self.unfit.len(),
                names.join(", ")
            ));
        }
        if self.left_out > 0 {
            message.push_str(&format!(
                " Left out {} of {total} names, past the drug_search_name_limit setting.",
                self.left_out
            ));
        }
        if !self.official_left_out.is_empty() {
            let names: Vec<_> = self.official_left_out.iter().map(Text::as_str).collect();
            message.push_str(&format!(
                " Left out official names past the ceiling of {OFFICIAL_NAME_CEILING}: {}.",
                names.join(", ")
            ));
        }
        message
    }
}

/// One spelling per name, the typed one kept when the provider can take it. Official names
/// pass the limit up to [`OFFICIAL_NAME_CEILING`]; the record's preferred names rank next and then the shortest, so the same
/// drug keeps the same names whichever of them was typed.
pub(crate) fn capped(
    names: Vec<Text>,
    (preferred, official): (&[String], &[String]),
    input: &Text,
    limit: usize,
    fit: Fit,
) -> Capped {
    let mut seen = std::collections::BTreeSet::new();
    let mut unfit = Vec::new();
    let mut names: Vec<_> = std::iter::once(input.clone())
        .chain(names)
        .filter(|name| seen.insert(lowered(name)))
        .filter(|name| match (name != input)
            .then(|| context_only(name.as_str()))
            .flatten()
            .or_else(|| fit(name.as_str()))
        {
            Some(reason) => {
                unfit.push((name.clone(), reason));
                false
            }
            None => true,
        })
        .collect();
    let lowered_set = |names: &[String]| -> std::collections::BTreeSet<_> {
        names.iter().map(|name| name.to_lowercase()).collect()
    };
    let preferred = lowered_set(preferred);
    let official = lowered_set(official);
    names.sort_by_key(|name| {
        (
            !official.contains(&lowered(name)),
            !preferred.contains(&lowered(name)),
            name.as_str().chars().count(),
            lowered(name),
        )
    });
    let kept_official = names
        .iter()
        .filter(|name| official.contains(&lowered(name)))
        .count();
    let limit = limit.max(kept_official.min(OFFICIAL_NAME_CEILING));
    let left_out = names.len().saturating_sub(limit);
    let mut cut = names.split_off(limit.min(names.len()));
    // The typed name stays, in place of the last name that is not official, else the last one.
    if let Some(at) = cut.iter().position(|name| name == input) {
        let typed = cut.remove(at);
        let slot = names
            .iter()
            .rposition(|name| !official.contains(&lowered(name)))
            .or(names.len().checked_sub(1));
        if let Some(kept) = slot.and_then(|slot| names.get_mut(slot)) {
            cut.push(std::mem::replace(kept, typed));
        }
    }
    let official_left_out = cut
        .into_iter()
        .filter(|name| official.contains(&lowered(name)))
        .collect();
    Capped {
        names,
        left_out,
        unfit,
        official_left_out,
    }
}
