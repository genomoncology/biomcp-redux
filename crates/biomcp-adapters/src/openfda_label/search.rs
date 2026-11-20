//! Literal indication and mechanism search and unfielded recovery of labels without harmonized names.
//! Native application numbers remain exact, separate from brand or ingredient identity.
//! Publisher fields: <https://open.fda.gov/apis/drug/label/searchable-fields/>.
use super::{Page, literal, page};
use biodata::{DrugId, Text};
use biomcp_fetch::{Leg, RequestError};
mod expanded;
mod names;
pub use expanded::{NameGroup, NameStep, search_expanded};

/// The native discovery result and the recovery step attempted.
#[derive(Debug)]
pub struct Search {
    /// Candidate page or source failure.
    pub page: Result<Page, RequestError>,
    /// Native label-text selector when discovery passed the name fields.
    pub free_text: Option<Text>,
}
/// Discover applications or names, then indication wording, then other label text.
pub fn search(leg: &Leg<'_>, name: &Text, limit: u32, offset: u32) -> Search {
    let mut free_text = None;
    let page = discover(leg, name, limit, offset, &mut free_text, None);
    Search { page, free_text }
}
/// The selector that narrows every request of one search: the label's own indication and
/// mechanism wording, each a literal phrase, and its application types by number prefix.
pub fn scope(query: &biomcp_types::DrugSearch) -> Option<String> {
    let mut parts = Vec::new();
    if let Some(indication) = query.indication() {
        parts.push(format!(
            "indications_and_usage:{}",
            literal(indication.as_str())
        ));
    }
    if let Some(mechanism) = query.mechanism() {
        parts.push(format!(
            "mechanism_of_action:{}",
            literal(mechanism.as_str())
        ));
    }
    if !query.application_type().is_empty() {
        let types: Vec<String> = query
            .application_type()
            .iter()
            .map(|kind| format!("{}*", kind.prefix()))
            .collect();
        parts.push(format!(
            "openfda.application_number:({})",
            types.join(" OR ")
        ));
    }
    (!parts.is_empty()).then(|| parts.join(" AND "))
}
/// Search within a scope, optionally constrained by a name.
pub fn search_scoped(
    leg: &Leg<'_>,
    scope: &str,
    name: Option<&Text>,
    limit: u32,
    offset: u32,
) -> Search {
    let mut free_text = None;
    let page = match name {
        Some(name) => discover(leg, name, limit, offset, &mut free_text, Some(scope)),
        None => candidates(leg, scope, limit, offset),
    };
    Search { page, free_text }
}
/// Labels per name page; openFDA's largest page.
const NAME_PAGE: u32 = 50;
/// Read every native word-search page for strict complete-name and approved single-term get
/// matching, in source order. The first page gives openFDA's total; the rest overlap within
/// openFDA's pacing, since one name can match hundreds of labels and each page takes about a
/// second to arrive (0871).
pub fn search_names(leg: &Leg<'_>, name: &Text) -> Result<Vec<Page>, RequestError> {
    let selector = {
        let name = literal(name.as_str());
        format!("openfda.brand_name:{name} OR openfda.generic_name:{name}")
    };
    let first = choice_fields(page(leg, &selector, NAME_PAGE, 0, None)?);
    let Some(next) = first.window.next_offset() else {
        return Ok(vec![first]);
    };
    let total = first.window.total().unwrap_or_default();
    let offsets: Vec<u32> = (next..)
        .step_by(NAME_PAGE as usize)
        .take_while(|offset| u64::from(*offset) < total)
        .collect();
    let mut pages = vec![first];
    for found in leg.batch(&offsets, |child, offset| {
        page(child, &selector, NAME_PAGE, *offset, None).map(choice_fields)
    }) {
        pages.push(found?);
    }
    Ok(pages)
}
/// A name lookup holds every page until it chooses a label, and a live page of whole labels may
/// reach the 32 MiB page bound. Choosing reads names and identity, plus the indications it
/// counts, so the other sections go as each page arrives: eight metformin lookups at once over
/// live-sized pages (54 MB per lookup) peaked at 733 MiB while every page stayed whole.
fn choice_fields(mut page: Page) -> Page {
    choice_sections(&mut page.window.rows);
    page
}
fn choice_sections(rows: &mut [super::Record]) {
    for row in rows {
        row.0
            .sections
            .retain(|heading, _| heading == "indications_and_usage");
    }
}
fn discover(
    leg: &Leg<'_>,
    name: &Text,
    limit: u32,
    offset: u32,
    free_text: &mut Option<Text>,
    indication: Option<&str>,
) -> Result<Page, RequestError> {
    let scoped = |selector: String| match indication {
        Some(indication) => format!("({selector}) AND {indication}"),
        None => selector,
    };
    if !(1..=50).contains(&limit) {
        return Err(biomcp_fetch::bad_reply(
            biomcp_types::Source::OpenfdaLabel,
            Some(200),
        ));
    }
    if name
        .as_str()
        .parse::<DrugId>()
        .is_ok_and(|id| biomcp_types::DrugSource::Drugsfda.accepts(&id))
    {
        let found = candidates(
            leg,
            &scoped(format!(
                "openfda.application_number.exact:{}",
                literal(name.as_str())
            )),
            limit,
            offset,
        )?;
        return Ok(found);
    }
    let words = {
        let name = literal(name.as_str());
        format!("openfda.brand_name:{name} OR openfda.generic_name:{name}")
    };
    let found = names::union(
        leg,
        &scoped(names::exact(name.as_str())),
        &scoped(words),
        limit,
        offset,
    )?;
    // Select by native total so exhausted pages never broaden the query.
    if found.window.total().is_some_and(|total| total > 0) {
        return Ok(found);
    }
    let selector = scoped(format!("indications_and_usage:{}", literal(name.as_str())));
    let found = candidates(leg, &selector, limit, offset)?;
    if found.window.total().is_some_and(|total| total > 0) {
        *free_text =
            Some(selector.parse().map_err(|_| {
                biomcp_fetch::bad_reply(biomcp_types::Source::OpenfdaLabel, Some(200))
            })?);
        return Ok(found);
    }
    let selector = literal(name.as_str());
    *free_text = Some(
        selector
            .parse()
            .map_err(|_| biomcp_fetch::bad_reply(biomcp_types::Source::OpenfdaLabel, Some(200)))?,
    );
    // The publisher searched label text, so a matching passage need not name
    // this product. Preserve its candidates and mark exact name matches separately.
    candidates(leg, &scoped(selector), limit, offset)
}

fn candidates(
    leg: &Leg<'_>,
    selector: &str,
    limit: u32,
    offset: u32,
) -> Result<Page, RequestError> {
    page(leg, selector, limit, offset, None)
}

#[cfg(test)]
#[path = "search_tests.rs"]
mod tests;
