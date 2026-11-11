//! Literal indication search and unfielded recovery of labels without harmonized names.
//! Native application numbers remain exact, separate from brand or ingredient identity.
//! Publisher fields: <https://open.fda.gov/apis/drug/label/searchable-fields/>.
use super::{Page, literal, page};
use biodata::{DrugId, Text};
use biomcp_fetch::{Leg, RequestError};
mod expanded;
pub use expanded::{NameTier, search_expanded};

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
/// Search the label's own indication wording, optionally constrained by a name.
pub fn search_indication(
    leg: &Leg<'_>,
    indication: &Text,
    name: Option<&Text>,
    limit: u32,
    offset: u32,
) -> Search {
    let selector = format!("indications_and_usage:{}", literal(indication.as_str()));
    let mut free_text = None;
    let page = match name {
        Some(name) => discover(leg, name, limit, offset, &mut free_text, Some(&selector)),
        None => candidates(leg, &selector, limit, offset),
    };
    Search { page, free_text }
}
/// Retain native word-search candidates for strict complete-name and approved single-term get matching.
pub fn search_names(
    leg: &Leg<'_>,
    name: &Text,
    limit: u32,
    offset: u32,
) -> Result<Page, RequestError> {
    let name = literal(name.as_str());
    page(
        leg,
        &format!("openfda.brand_name:{name} OR openfda.generic_name:{name}"),
        limit,
        offset,
        None,
    )
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
    for (selector, label_text) in [
        (scoped(exact_names(name.as_str())), false),
        (scoped(words), false),
        (
            scoped(format!("indications_and_usage:{}", literal(name.as_str()))),
            true,
        ),
    ] {
        let found = candidates(leg, &selector, limit, offset)?;
        // Select by native total so exhausted pages never broaden the query.
        if found.window.total().is_some_and(|total| total > 0) {
            if label_text {
                *free_text = Some(selector.parse().map_err(|_| {
                    biomcp_fetch::bad_reply(biomcp_types::Source::OpenfdaLabel, Some(200))
                })?);
            }
            return Ok(found);
        }
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

/// openFDA's exact name fields compare letter case as stored, and labels store
/// names upper, title, sentence or lower case, so one request asks for each form.
/// The forms come in a fixed order, so every casing of a name sends the same request.
fn exact_names(name: &str) -> String {
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

fn candidates(
    leg: &Leg<'_>,
    selector: &str,
    limit: u32,
    offset: u32,
) -> Result<Page, RequestError> {
    page(leg, selector, limit, offset, None)
}
