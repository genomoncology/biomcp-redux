//! Name discovery: exact-name hits in any letter case first, then word-only hits, as one page order.
use super::{Page, candidates};
use crate::openfda_label::literal;
use biomcp_fetch::{Leg, RequestError};

/// openFDA's exact name fields compare letter case as stored, and labels store
/// names upper, title, sentence or lower case, so one selector asks for each form.
/// The forms come in a fixed order, so every casing of a name sends the same request.
pub(super) fn exact(name: &str) -> String {
    let mut forms: Vec<String> = Vec::new();
    for form in [
        name.to_uppercase(),
        title_case(name),
        sentence_case(name),
        name.to_lowercase(),
        name.to_owned(),
    ] {
        if !forms.contains(&form) {
            forms.push(form);
        }
    }
    forms
        .iter()
        .map(|form| {
            let form = literal(form);
            format!("openfda.brand_name.exact:{form} OR openfda.generic_name.exact:{form}")
        })
        .collect::<Vec<_>>()
        .join(" OR ")
}

/// Page the union of exact and word matches with every exact hit ranked first.
/// Positions below the native exact total come from the exact selector; later
/// positions come from word matches that are not exact hits. The total is the
/// sum of the two disjoint native totals.
pub(super) fn union(
    leg: &Leg<'_>,
    exact: &str,
    words: &str,
    limit: u32,
    offset: u32,
) -> Result<Page, RequestError> {
    let first = candidates(leg, exact, limit, offset)?;
    let exact_total = first.window.total().unwrap_or(0);
    if exact_total == 0 {
        return candidates(leg, words, limit, offset);
    }
    let mut taken = first.window.received();
    let rest_selector = format!("({words}) AND NOT ({exact})");
    let mut rows = first.window.rows;
    let mut problems = first.problems;
    let rest_total = if taken < limit {
        let skip = u64::from(offset).saturating_sub(exact_total);
        let skip = u32::try_from(skip)
            .map_err(|_| biomcp_fetch::bad_reply(biomcp_types::Source::OpenfdaLabel, Some(200)))?;
        let rest = candidates(leg, &rest_selector, limit - taken, skip)?;
        taken += rest.window.received();
        rows.extend(rest.window.rows);
        problems.extend(rest.problems);
        rest.window.total()
    } else {
        // A full exact page still needs the word-only total; its one probe row is not shown.
        candidates(leg, &rest_selector, 1, 0)?.window.total()
    };
    let total = exact_total + rest_total.unwrap_or(0);
    Ok(Page {
        window: crate::NativeWindow::new(
            rows,
            offset,
            limit,
            Some(total),
            taken,
            crate::Refusal::None,
        ),
        problems,
        metadata: first.metadata,
    })
}

fn title_case(name: &str) -> String {
    let mut word_start = true;
    let mut out = String::with_capacity(name.len());
    for c in name.chars() {
        if word_start {
            out.extend(c.to_uppercase());
        } else {
            out.extend(c.to_lowercase());
        }
        word_start = c.is_whitespace();
    }
    out
}

fn sentence_case(name: &str) -> String {
    let mut chars = name.chars();
    chars.next().map_or_else(String::new, |first| {
        first
            .to_uppercase()
            .chain(chars.flat_map(char::to_lowercase))
            .collect()
    })
}
