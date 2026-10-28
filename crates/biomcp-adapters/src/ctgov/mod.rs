//! Native registry discovery; no local clinical interpretation or reordering.
pub mod detail;
pub mod document;
mod raw;
use biodata::Text;
use biomcp_fetch::{Arg, Endpoint, Freshness, KeyUse, Leg, RequestError, SourceSpec};
use biomcp_types::TrialHeader;
use biomcp_types::TrialSearch;
use raw::{FirstPage, RawPage, Version};
use std::{collections::BTreeMap, time::Duration};
/// Reviewed document download origin.
pub(crate) const DOCUMENT_ORIGIN: &str = "https://cdn.clinicaltrials.gov";
/// Publisher terms apply to this registry; individual records are not a blanket CC0 grant.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Additional(&[DOCUMENT_ORIGIN]),
    user_agent: None,
    source: biomcp_types::Source::ClinicalTrialsGov,
    base: "https://clinicaltrials.gov/api/v2",
    key: KeyUse::None,
    quota: &crate::quotas::CTGOV,
    deadline: Duration::from_secs(15),
    terms: biomcp_fetch::Terms {
        name: "ClinicalTrials.gov attribution and source processing date; publisher terms",
        url: "https://clinicaltrials.gov/about-site/terms-conditions",
        usage: biomcp_types::Use::Restricted,
    },
};
const SEARCH: Endpoint = SPEC.get("/studies").fresh(Freshness::Days(1));
/// Small source processing version endpoint, also the bounded health probe.
pub const VERSION: Endpoint = SPEC.get("/version").fresh(Freshness::Days(1));
/// A native search page with opaque source cursor.
#[derive(Debug)]
pub struct SearchPage {
    /// Source order, untouched.
    pub items: Vec<TrialHeader>,
    /// Exact native total.
    pub total: Option<u64>,
    /// Native count lookup failure while page rows remain usable.
    pub count_error: Option<RequestError>,
    /// Exact fallback count parameters, only when the source omitted continuation totals.
    pub count_request: Option<BTreeMap<String, String>>,
    /// Opaque next page.
    pub next: Option<Text>,
}
/// Exact parameters; resolved gene text is explicit and attributed to HGNC by the caller.
pub fn parameters(query: &TrialSearch, gene: Option<&str>) -> BTreeMap<String, String> {
    let mut p = BTreeMap::from([
        ("countTotal".into(), "true".into()),
        ("pageSize".into(), if query.count_only() { "1".into() } else { query.limit().to_string() }),
        (
            "fields".into(),
            "NCTId,BriefTitle,OverallStatus,Phase,Condition,InterventionName,LastUpdatePostDate,HasResults".into(),
        ),
    ]);
    for (name, value) in [
        ("query.cond", query.condition()),
        ("query.intr", query.intervention()),
        ("pageToken", query.next_page()),
    ] {
        if let Some(value) = value {
            p.insert(name.into(), value.as_str().into());
        }
    }
    let mut text = Vec::new();
    if let Some(value) = query.query() {
        text.push(value.as_str().to_owned());
    }
    if let Some(value) = query.variant() {
        text.push(value.as_str().to_owned());
    }
    if let Some(value) = gene {
        text.push(value.to_owned());
    }
    if !text.is_empty() {
        p.insert("query.term".into(), text.join(" AND "));
    }
    if let Some(status) = query.status() {
        p.insert("filter.overallStatus".into(), status.as_str().to_owned());
    }
    let advanced = advanced(query);
    if !advanced.is_empty() {
        p.insert("filter.advanced".into(), advanced);
    }
    let mut facets = Vec::new();
    if let Some(sex) = query.sex() {
        facets.push(format!(
            "sex:{}",
            match sex {
                biodata::TrialSex::Female => "f",
                biodata::TrialSex::Male => "m",
                biodata::TrialSex::All => "a",
            }
        ));
    }
    if let Some(sponsor) = query.sponsor_type() {
        facets.push(format!("funderType:{}", sponsor.as_str()));
    }
    if !facets.is_empty() {
        p.insert("aggFilters".into(), facets.join(","));
    }
    if let (Some(lat), Some(lon), Some(distance)) = (query.lat(), query.lon(), query.distance()) {
        p.insert(
            "filter.geo".into(),
            format!(
                "distance({},{},{})",
                lat.as_str(),
                lon.as_str(),
                distance.as_str()
            ),
        );
    }
    p
}
fn advanced(query: &TrialSearch) -> String {
    let mut advanced = Vec::new();
    if let Some(id) = query.id() {
        advanced.push(format!("AREA[NCTId]{id}"));
    }
    if let Some(phase) = query.phase() {
        advanced.extend(
            phase
                .phases()
                .iter()
                .map(|phase| format!("AREA[Phase]{}", phase.as_str())),
        );
    }
    for (field, value) in [
        ("EligibilityCriteria", query.criteria()),
        ("LocationFacility", query.facility()),
        ("LeadSponsorName", query.sponsor()),
    ] {
        if let Some(value) = value {
            advanced.push(format!("AREA[{field}]{}", quote(value.as_str())));
        }
    }
    if let Some(study_type) = query.study_type() {
        advanced.push(format!("AREA[StudyType]{}", study_type.as_str()));
    }
    if let Some(age) = query.age() {
        advanced.push(format!(
            "(AREA[MinimumAge]RANGE[MIN,{age} years] OR AREA[MinimumAge]MISSING) AND \
            (AREA[MaximumAge]RANGE[{age} years,MAX] OR AREA[MaximumAge]MISSING)"
        ));
    }
    if query.updated_since().is_some() || query.updated_until().is_some() {
        advanced.push(format!(
            "AREA[LastUpdatePostDate]RANGE[{},{}]",
            query
                .updated_since()
                .map_or("MIN".into(), ToString::to_string),
            query
                .updated_until()
                .map_or("MAX".into(), ToString::to_string)
        ));
    }
    if query.has_results() {
        advanced.push("AREA[ResultsFirstPostDate]RANGE[MIN,MAX]".into());
    }
    advanced.join(" AND ")
}
fn quote(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}
/// Read and validate native discovery results before shared cache admission.
pub fn search(
    leg: &Leg<'_>,
    params: &BTreeMap<String, String>,
) -> Result<SearchPage, RequestError> {
    let raw = fetch_page(leg, params)?;
    let mut total = raw.total_count;
    let mut count_error = None;
    let mut count_request = None;
    if total.is_none() {
        let mut count = params.clone();
        count.remove("pageToken");
        count.insert("pageSize".into(), "1".into());
        match fetch_page(leg, &count) {
            Ok(page) => total = page.total_count,
            Err(error) => count_error = Some(error),
        }
        count_request = Some(count);
    }
    Ok(SearchPage {
        items: raw
            .studies
            .unwrap_or_default()
            .into_iter()
            .map(|row| row.protocol_section.header())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| bad())?,
        total,
        count_error,
        count_request,
        next: raw.next_page_token,
    })
}
fn fetch_page(leg: &Leg<'_>, params: &BTreeMap<String, String>) -> Result<RawPage, RequestError> {
    let args: Vec<_> = params
        .iter()
        .map(|(k, v)| (k.as_str(), Arg::Text(v)))
        .collect();
    if params.contains_key("pageToken") {
        leg.json::<RawPage>(&SEARCH, &args)
    } else {
        leg.json::<FirstPage>(&SEARCH, &args).map(|page| page.0)
    }
}
/// Source processing timestamp, distinct from request time and record update dates.
pub fn version(leg: &Leg<'_>) -> Result<Text, RequestError> {
    Ok(leg.json::<Version>(&VERSION, &[])?.data_timestamp)
}

fn bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: biomcp_types::Source::ClinicalTrialsGov,
        reason: biomcp_types::Reason::BadReply,
        status: Some(200),
        retry_after: None,
    }
    .into()
}
