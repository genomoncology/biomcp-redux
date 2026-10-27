//! Canonical DRUGSFDA source declaration.
use biomcp_fetch::{SourceSpec, Terms};
use biomcp_types::Source;
use std::time::Duration;
/// Publisher policy shared by health, request admission and response outcomes.
pub static SPEC: SourceSpec = SourceSpec {
    user_agent: None,
    source: Source::Drugsfda,
    base: "https://api.fda.gov/drug/drugsfda.json",
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
mod raw;
use biodata::{DrugId, DrugIdentityLevel, Text};
use biomcp_fetch::{Arg, BadReply, Endpoint, Freshness, Leg, ReplyMetadata, RequestError};
use biomcp_types::{DrugHeader, DrugSource};
pub use raw::{Application, Document, Ingredient, OpenFda, Product, Property, Submission, date};
const APPLICATION: Endpoint = SPEC.get("").fresh(Freshness::Days(1)).json_absence();
/// One native application candidate page and its original retrieval metadata.
#[derive(Debug)]
pub struct Page {
    /// Native applications, without selecting a first product.
    pub records: Vec<Application>,
    /// Native application total, independent of product count.
    pub total: u64,
    /// Actual transport retrieval metadata.
    pub metadata: ReplyMetadata,
    /// Native dataset last_updated date; distinct from record events and retrieval.
    pub updated: Option<biodata::PartialDate>,
}
fn literal(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}
/// Discover source-native applications using the unchanged literal ingredient phrase.
pub fn search(leg: &Leg<'_>, text: &Text, limit: u32, offset: u32) -> Result<Page, RequestError> {
    page(
        leg,
        &format!(
            "products.active_ingredients.name:{}",
            literal(text.as_str())
        ),
        limit,
        offset,
        None,
    )
}
/// Read exactly the named native application; reject a wrong identity before caching it.
pub fn get(leg: &Leg<'_>, id: &DrugId) -> Result<Page, RequestError> {
    if !DrugSource::Drugsfda.accepts(id) {
        return Err(bad());
    }
    page(
        leg,
        &format!("application_number:{}", literal(&id.to_string())),
        1,
        0,
        Some(id),
    )
}
fn page(
    leg: &Leg<'_>,
    selector: &str,
    limit: u32,
    offset: u32,
    id: Option<&DrugId>,
) -> Result<Page, RequestError> {
    if !(1..=50).contains(&limit) {
        return Err(bad());
    }
    let mut args = vec![
        ("search", Arg::Text(selector)),
        ("limit", Arg::Num(limit.into())),
    ];
    if offset > 0 {
        args.push(("skip", Arg::Num(offset.into())));
    }
    let (response, metadata) =
        leg.json_received_checked(&APPLICATION, &args, |r: &raw::Response| match r {
            raw::Response::Success { meta, results }
                if meta.results.skip != offset
                    || meta.results.limit != limit
                    || id.is_some_and(|id| {
                        meta.results.total != 1
                            || results.iter().any(|r| r.application_number != *id)
                    }) =>
            {
                Err(BadReply)
            }
            _ => Ok(()),
        })?;
    let (records, total, updated) = match response {
        raw::Response::Success { meta, results } => {
            (results, meta.results.total, meta.last_updated)
        }
        raw::Response::Absent { .. } => (Vec::new(), 0, None),
    };
    Ok(Page {
        records,
        total,
        metadata,
        updated,
    })
}
impl Application {
    /// An application endpoint establishes application identity, without choosing a brand.
    pub fn header(&self) -> DrugHeader {
        DrugHeader {
            matched_name: None,
            id: self.application_number.clone(),
            source: DrugSource::Drugsfda,
            name: None,
            identity_level: DrugIdentityLevel::FdaApplication,
            r#type: None,
            version: None,
            url: None,
        }
    }
}
fn bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::Drugsfda,
        reason: biomcp_types::Reason::BadReply,
        status: Some(200),
        retry_after: None,
    }
    .into()
}
