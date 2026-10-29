use super::{Page, bad, literal, page};
use biodata::{DrugId, Text};
use biomcp_fetch::{Leg, RequestError};

/// The native discovery result and the recovery step attempted.
#[derive(Debug)]
pub struct Search {
    /// Candidate page or source failure.
    pub page: Result<Page, RequestError>,
    /// Exact unfielded selector when discovery reached the free-text step.
    pub free_text: Option<Text>,
}
/// Discover exact applications or names, then words, then free text.
pub fn search(leg: &Leg<'_>, name: &Text, limit: u32, offset: u32) -> Search {
    let mut free_text = None;
    let page = discover(leg, name, limit, offset, &mut free_text);
    Search { page, free_text }
}
fn discover(
    leg: &Leg<'_>,
    name: &Text,
    limit: u32,
    offset: u32,
    free_text: &mut Option<Text>,
) -> Result<Page, RequestError> {
    if !(1..=50).contains(&limit) {
        return Err(bad());
    }
    if name
        .as_str()
        .parse::<DrugId>()
        .is_ok_and(|id| biomcp_types::DrugSource::Drugsfda.accepts(&id))
    {
        let mut found = page(
            leg,
            &format!("openfda.application_number:{}", literal(name.as_str())),
            limit,
            offset,
            None,
        )?;
        found.records.retain(|record| {
            record
                .openfda
                .application_number
                .iter()
                .flatten()
                .any(|number| number.as_str() == name.as_str())
        });
        return Ok(found);
    }
    let selector = |suffix: &str| {
        let name = literal(name.as_str());
        format!("openfda.brand_name{suffix}:{name} OR openfda.generic_name{suffix}:{name}")
    };
    let mut exact = page(leg, &selector(".exact"), limit, offset, None)?;
    exact
        .records
        .retain(|record| record.name_match(name).is_some());
    if !exact.records.is_empty() {
        return Ok(exact);
    }
    let mut fallback = page(leg, &selector(""), limit, offset, None)?;
    fallback
        .records
        .retain(|record| record.name_match(name).is_some());
    if !fallback.records.is_empty() {
        return Ok(fallback);
    }
    let selector = literal(name.as_str());
    *free_text = Some(selector.parse().map_err(|_| bad())?);
    let mut fallback = page(leg, &selector, limit, offset, None)?;
    fallback
        .records
        .retain(|record| record.name_match(name).is_some());
    Ok(fallback)
}
