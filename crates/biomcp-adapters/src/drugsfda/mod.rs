//! Canonical DRUGSFDA source declaration.
use biomcp_fetch::{SourceSpec, Terms};
use biomcp_types::Source;
use std::time::Duration;
/// Publisher policy shared by health, request admission and response outcomes.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
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
mod admit;
mod map;
mod raw;
mod wire;
use biodata::{DrugId, DrugIdentityLevel, Text};
use biomcp_fetch::{Arg, BadReply, Endpoint, Freshness, Leg, ReplyMetadata, RequestError};
use biomcp_types::{DrugHeader, DrugSource};
pub use map::Application;
use raw::Application as RawApplication;
pub(crate) use raw::date;
const APPLICATION: Endpoint = SPEC.get("").fresh(Freshness::Days(1)).json_absence();
/// One native application candidate page and its original retrieval metadata.
#[derive(Debug)]
pub struct Page {
    /// Actual refused native records, independent of optional fields.
    pub refusals: crate::Refusal,
    /// Rejected optional source metadata.
    pub problems: Vec<biomcp_types::Problem>,
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
/// The sponsor-name prefix an applicant selects: the source stores sponsor names in upper case.
pub fn applicant_prefix(applicant: &Text) -> String {
    applicant.as_str().to_uppercase()
}
/// Discover exact brand or ingredient names, then try their native word fields after a zero total.
/// An applicant narrows either tier by a native sponsor_name prefix wildcard, or selects alone.
pub fn search(
    leg: &Leg<'_>,
    text: Option<&Text>,
    applicant: Option<&Text>,
    limit: u32,
    offset: u32,
) -> Result<Page, RequestError> {
    let names: Vec<Text> = text.cloned().into_iter().collect();
    let exact = page(leg, &selector(&names, applicant, NameStep::Exact), limit, offset, None)?;
    if exact.total > 0 || text.is_none() {
        return Ok(exact);
    }
    page(leg, &selector(&names, applicant, NameStep::Words), limit, offset, None)
}

/// BioMCP's own bound on one native Drugs@FDA request, so a name group stays one request.
const NAME_QUERY_MAX_BYTES: usize = 4096;

/// Which name step a Drugs@FDA search pages, chosen on its first page as the typed name's own
/// search would choose it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NameStep {
    /// Exact brand and ingredient names.
    Exact,
    /// Word matches, after the typed name has no exact match.
    Words,
}

fn sponsor(applicant: &Text) -> String {
    let mut term = String::from("sponsor_name:");
    for c in applicant_prefix(applicant).chars() {
        if !c.is_alphanumeric() {
            term.push('\\');
        }
        term.push(c);
    }
    term.push('*');
    term
}

/// One native selector for a name group at one step, narrowed by the applicant prefix.
fn selector(names: &[Text], applicant: Option<&Text>, step: NameStep) -> String {
    let suffix = if step == NameStep::Exact { ".exact" } else { "" };
    let names = (!names.is_empty()).then(|| {
        names
            .iter()
            .map(|name| {
                let name = literal(name.as_str());
                format!(
                    "products.brand_name{suffix}:{name} OR products.active_ingredients.name{suffix}:{name}"
                )
            })
            .collect::<Vec<_>>()
            .join(" OR ")
    });
    match (names, applicant.map(sponsor)) {
        (Some(names), Some(sponsor)) => format!("({names}) AND {sponsor}"),
        (names, sponsor) => names.or(sponsor).unwrap_or_default(),
    }
}

/// Whether one request can carry this name group.
pub fn names_fit(names: &[Text], applicant: Option<&Text>) -> bool {
    selector(names, applicant, NameStep::Exact).len() <= NAME_QUERY_MAX_BYTES
}

/// Search one bounded name group in one native request; never combine per-name totals.
/// The first page takes the step the typed name alone would reach, so the group finds at least
/// what the typed name finds.
pub fn search_widened(
    leg: &Leg<'_>,
    (names, typed): (&[Text], &Text),
    applicant: Option<&Text>,
    (limit, offset): (u32, u32),
    selected: Option<NameStep>,
) -> (Result<Page, RequestError>, NameStep) {
    let at = |step| page(leg, &selector(names, applicant, step), limit, offset, None);
    if let Some(step) = selected {
        return (at(step), step);
    }
    let exact = at(NameStep::Exact);
    let found = match &exact {
        Err(_) => return (exact, NameStep::Exact),
        Ok(page) => page.total > 0,
    };
    if found && names.len() > 1 {
        let typed = std::slice::from_ref(typed);
        match page(leg, &selector(typed, applicant, NameStep::Exact), 1, 0, None) {
            Ok(page) if page.total > 0 => return (exact, NameStep::Exact),
            Ok(_) => {}
            Err(error) => return (Err(error), NameStep::Exact),
        }
    } else if found {
        return (exact, NameStep::Exact);
    }
    (at(NameStep::Words), NameStep::Words)
}
/// Name resolution uses the existing word selector to retain all exact and approved single-term generic forms.
pub fn search_names(
    leg: &Leg<'_>,
    text: &Text,
    limit: u32,
    offset: u32,
) -> Result<Page, RequestError> {
    let name = literal(text.as_str());
    page(
        leg,
        &format!("products.brand_name:{name} OR products.active_ingredients.name:{name}"),
        limit,
        offset,
        None,
    )
}
/// Read exactly the named native application; reject a wrong identity before caching it.
pub fn get(leg: &Leg<'_>, id: &DrugId) -> Result<Page, RequestError> {
    if !DrugSource::Drugsfda.accepts(id) {
        return Err(biomcp_fetch::bad_reply(Source::Drugsfda, Some(200)));
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
        return Err(biomcp_fetch::bad_reply(Source::Drugsfda, Some(200)));
    }
    let mut args = vec![
        ("search", Arg::Text(selector)),
        ("limit", Arg::Num(limit.into())),
    ];
    if offset > 0 {
        args.push(("skip", Arg::Num(offset.into())));
    }
    let (response, metadata) = crate::openfda::page(
        Source::Drugsfda,
        leg,
        &APPLICATION,
        &args,
        offset,
        limit,
        &|r: &raw::Response| match r {
            raw::Response::Page { meta, results }
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
        },
    )
    .map(|received| (received.value, received.meta))?;
    let mut problems = crate::mapped::Problems {
        source: biomcp_types::Source::Drugsfda,
        record: None,
        list: Vec::new(),
    };
    let (records, total, updated) = match response {
        raw::Response::Page { meta, results } => (results, meta.results.total, meta.last_updated),
        raw::Response::Absent { .. } => (Vec::new(), 0, None),
    };
    problems
        .list
        .extend(records.iter().flat_map(|r| r.problems.iter().cloned()));
    let mut refusals = crate::Refusal::None;
    for record in &records {
        refusals.include(record.refusals);
    }
    Ok(Page {
        refusals,
        problems: problems.list,
        records: records.into_iter().map(map::Application).collect(),
        total,
        metadata,
        updated,
    })
}
impl RawApplication {
    /// An application endpoint establishes application identity, without choosing a brand.
    pub(super) fn header(&self) -> DrugHeader {
        DrugHeader {
            name_resolution: None,
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

/// Complete source policy and command declaration.
pub(crate) mod decl;

/// FDA document downloads retain their API source identity and never send its key.
pub static DOCUMENTS: biomcp_fetch::SourceSpec = biomcp_fetch::SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    source: biomcp_types::Source::Drugsfda,
    base: "https://www.accessdata.fda.gov",
    terms: SPEC.terms,
    key: biomcp_fetch::KeyUse::None,
    quota: &crate::quotas::OPENFDA,
    deadline: std::time::Duration::from_secs(15),
};
