//! Identity-checked openFDA name discovery without clinical text selection.
use biomcp_fetch::{SourceSpec, Terms};
use biomcp_types::Source;
use std::time::Duration;
/// Publisher policy shared by health, request admission and response outcomes.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    source: Source::OpenfdaLabel,
    base: "https://api.fda.gov/drug/label.json",
    terms: Terms {
        name: crate::drug_sources::FDA_TERMS.name,
        url: crate::drug_sources::FDA_TERMS.url,
        usage: crate::drug_sources::FDA_TERMS.usage,
    },
    quota: &crate::quotas::OPENFDA,
    key: crate::openfda_key::OPTIONAL,
    deadline: Duration::from_secs(15),
};
mod map;
mod page;
mod raw;
use biodata::{DrugId, Text};
use biomcp_fetch::{Arg, BadReply, Endpoint, Freshness, Leg, ReplyMetadata, RequestError};
pub use raw::Record;
/// Exact native headings supported by the label surface.
pub const HEADINGS: [&str; 5] = [
    "boxed_warning",
    "dosage_and_administration",
    "drug_interactions",
    "indications_and_usage",
    "warnings_and_cautions",
];
const LABEL: Endpoint = SPEC.get("").fresh(Freshness::Days(1)).json_absence();
/// A source candidate page, preserving native order and source total.
#[derive(Debug)]
pub struct Page {
    /// Source revision records, never merged by ingredient or brand.
    pub records: Vec<Record>,
    /// Number of native rows consumed before any name identity filtering.
    pub scanned: u32,
    /// Native count for this selector, before any name identity filtering.
    pub total: u64,
    /// This reply's actual original retrieval metadata.
    pub metadata: ReplyMetadata,
}
fn literal(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}
/// Exact name discovery, with one tokenized fallback after an empty exact page.
pub fn search(leg: &Leg<'_>, name: &Text, limit: u32, offset: u32) -> Result<Page, RequestError> {
    if !(1..=50).contains(&limit) {
        return Err(bad());
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
    Ok(fallback)
}
fn normalize_name(value: &str) -> String {
    value
        .chars()
        .filter(|c| !c.is_whitespace())
        .flat_map(char::to_lowercase)
        .collect()
}
impl Record {
    /// Compare complete publisher values after the established case and spacing normalization.
    pub fn name_match(&self, input: &Text) -> Option<biomcp_types::DrugNameMatch> {
        let input = normalize_name(input.as_str());
        let matches = |names: &Option<Vec<Text>>| {
            names
                .iter()
                .flatten()
                .any(|name| normalize_name(name.as_str()) == input)
        };
        use biomcp_types::DrugNameMatch;
        match (
            matches(&self.openfda.brand_name),
            matches(&self.openfda.generic_name),
        ) {
            (true, true) => Some(DrugNameMatch::Both),
            (true, false) => Some(DrugNameMatch::Brand),
            (false, true) => Some(DrugNameMatch::Generic),
            (false, false) => None,
        }
    }
}
/// Read only the explicitly selected SPL set, refusing a mismatched returned set before caching.
pub fn get(leg: &Leg<'_>, set: &DrugId) -> Result<Page, RequestError> {
    if !biomcp_types::DrugSource::OpenfdaLabel.accepts(set) {
        return Err(bad());
    }
    page(
        leg,
        &format!("openfda.spl_set_id:{}", literal(&set.to_string())),
        1,
        0,
        Some(set),
    )
}
fn page(
    leg: &Leg<'_>,
    selector: &str,
    limit: u32,
    offset: u32,
    set: Option<&DrugId>,
) -> Result<Page, RequestError> {
    let mut args = vec![
        ("search", Arg::Text(selector)),
        ("limit", Arg::Num(limit.into())),
        ("sort", Arg::Text("effective_time:desc")),
    ];
    if offset > 0 {
        args.push(("skip", Arg::Num(offset.into())));
    }
    let (reply, metadata) =
        leg.json_received_checked::<raw::Response>(&LABEL, &args, |reply| match reply {
            raw::Response::Success { meta, results }
                if meta.results.skip != offset
                    || meta.results.limit != limit
                    || set.is_some_and(|set| results.iter().any(|r| r.set_id != *set)) =>
            {
                Err(BadReply)
            }
            _ => Ok(()),
        })?;
    let (records, total) = match reply {
        raw::Response::Success { meta, results } => (results, meta.results.total),
        raw::Response::Absent { .. } => (Vec::new(), 0),
    };
    Ok(Page {
        scanned: records.len() as u32,
        records,
        total,
        metadata,
    })
}
fn bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::OpenfdaLabel,
        reason: biomcp_types::Reason::BadReply,
        status: Some(200),
        retry_after: None,
    }
    .into()
}
